use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::fs;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::{fs::File, io::Read};
use language::language::{c::{ self, C}, Language};
use std::io::Write;
use std::path::Path;

pub mod node_parser;
pub mod proto_parser;
pub mod nodes_replacer;

pub mod proto {
    tonic::include_proto!("c.lenga");
}

// Service Interface
use proto::{
    EditRequest, InitRequest, OpenRequest, SessionId, SourceFile,
    c_lenga_server::{CLenga, CLengaServer}, Void
};

use crate::lenga_service::clenga::node_parser::{ source_file_to_proto, c_language_object_to_proto};
use crate::lenga_service::clenga::proto::{EditResponse, SaveRequest};
use crate::lenga_service::clenga::proto_parser::proto_to_c_language_object;
use crate::lenga_service::clenga::nodes_replacer::replace_source_file;

#[derive(Debug)]
pub struct CLengaService {
    files: Arc<Mutex<HashMap<String, c::language_object::source_file::SourceFile>>>,
}

impl Default for CLengaService {
    fn default() -> Self {
        Self {
            files: Default::default(),
        }
    }
}

#[tonic::async_trait]
impl CLenga for CLengaService {
    async fn initialize(
        &self,
        _request: Request<InitRequest>,
    ) -> Result<Response<SessionId>, Status> {
        Ok(Response::new(SessionId {
            id: Uuid::new_v4().to_string(),
        }))
    }

    async fn open_file(
        &self,
        request: Request<OpenRequest>,
    ) -> Result<Response<SourceFile>, Status> {
        let req = request.into_inner();

        let mut files = self.files.lock().unwrap(); //TODO: Define how to de-poison lock
        let file = match files.get(&req.path) {
            Some(file_ast) => {
                file_ast.clone()
            },
            None => {
                let file = File::open(&req.path).map_err(|err| Status::from_error(Box::new(err)))?;
                let content: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect(); //TODO: recover or abort
                let c = C::new();
                let src_file = c.parse_nodes(content).map_err(|err| Status::internal(err))?;

                files.insert(req.path, src_file.clone());

                src_file
            }
        };

        let ast = source_file_to_proto(file);

        Ok(Response::new(ast))
    }

    async fn edit(
        &self,
        request: Request<EditRequest>,
    ) -> Result<Response<EditResponse>, Status> {
        let req = request.into_inner();
        let edited_object = req.edited_object.ok_or(Status::invalid_argument("Inexistent edited_object field"))?;
        let edited_node = proto_to_c_language_object(edited_object).map_err(|err| Status::invalid_argument(err))?;

        let mut files = self.files.lock().unwrap(); //TODO: Define how to de-poison lock
        match files.get_mut(&req.path) {
            Some(file_ast) => {
                let node_id = edited_node.id();
                let replaced = replace_source_file(file_ast, edited_node)
                    .ok_or_else(|| Status::failed_precondition(format!("Matching objects with id {} not found", node_id)))?;

                let ast = source_file_to_proto(file_ast.clone());
                let rep = c_language_object_to_proto(replaced);
                let res = proto::EditResponse {
                    new_object: Some(proto::LanguageObject {
                        language_object: Some(proto::language_object::LanguageObject::SourceFile(ast)),
                    }),
                    old_object: Some(rep),
                };                    
                Ok(Response::new(res))
                
            },
            None => Err(Status::not_found(format!("File not found: {}", req.path)))
        }
    }

    async fn save(
        &self,
        request: Request<SaveRequest>,
    ) -> Result<Response<Void>, Status> {
        let req = request.into_inner();
        
        let file_ast = {
            let files = self.files.lock().unwrap(); //TODO: Define how to de-poison lock
            files.get(&req.path).cloned()
        }.ok_or_else(|| Status::not_found(format!("File not found: {}", req.path)))?;

        let c = C::new();
        let output = c.write_to_nodes(file_ast).map_err(|err| Status::data_loss(err))?;

        let path = Path::new(&req.write_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| Status::internal(format!("Failed to create directories: {}", e)))?;
        }

        let mut output_file = File::create(req.write_path).map_err(|err| Status::from_error(Box::new(err)))?; 
        output_file.write_all(&output).map_err(|err| Status::data_loss(err.to_string()))?;

        Ok(Response::new(proto::Void {}))
    }
}

pub fn clenga_service() -> CLengaServer<CLengaService> {
    let c_lenga_service = CLengaService::default();
    CLengaServer::new(c_lenga_service)
}
