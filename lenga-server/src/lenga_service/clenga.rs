use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
use uuid::Uuid;
use std::{fs::File, io::Read};
use language::language::{c::{ self, C}, Language};

pub mod node_parser;
pub mod proto_parser;
pub mod nodes_replacer;

pub mod proto {
    tonic::include_proto!("c.lenga");
}

// Service Interface
use proto::{
    EditRequest, InitRequest, LanguageObject, OpenRequest, SessionId, SourceFile,
    c_lenga_server::{CLenga, CLengaServer},
};

use crate::lenga_service::clenga::node_parser::source_file_to_proto;
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

        let mut files = self.files.lock().unwrap();
        let file = match files.get(&req.path) {
            Some(file_ast) => {
                file_ast.clone()
            },
            None => {
                let file = File::open(&req.path).unwrap();
                let content: Vec<u8> = file.bytes().map(|b| b.unwrap()).collect();
                let c = C::new();
                let src_file = c.parse_nodes(content).unwrap();

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
    ) -> Result<Response<LanguageObject>, Status> {
        let req = request.into_inner();
        let edited_node = proto_to_c_language_object(req.edited_object.unwrap()).unwrap();

        let mut files = self.files.lock().unwrap();
        match files.get_mut(&req.path) {
            Some(file_ast) => {
                if let Some(_) = replace_source_file(file_ast, edited_node){
                    let ast = source_file_to_proto(file_ast.clone());
                    let res = proto::LanguageObject {
                        language_object: Some(proto::language_object::LanguageObject::SourceFile(ast)),
                    };
                    Ok(Response::new(res))
                } else {
                    panic!() //TODO: Return a error status code if the node could not be replaced
                }
            },
            None => panic!(), //TODO: Send a error status code if the file is not found
        }
    }
}

pub fn clenga_service() -> CLengaServer<CLengaService> {
    let c_lenga_service = CLengaService::default();
    CLengaServer::new(c_lenga_service)
}
