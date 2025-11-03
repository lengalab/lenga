use language::language::{
    Language,
    c::{self, C},
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::fs;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub mod node_parser;
pub mod node_searcher;
pub mod nodes_replacer;
pub mod proto_parser;

pub mod proto {
    tonic::include_proto!("c.lenga");
}

// Service Interface
use proto::{
    CloseRequest, EditRequest, InitRequest, OpenRequest, SessionId, SourceFile, Void,
    c_lenga_server::{CLenga, CLengaServer},
};

use crate::lenga_service::clenga::{
    node_parser::{c_language_object_to_proto, source_file_to_proto},
    node_searcher::find_node,
    nodes_replacer::replace_source_file,
    proto::{AvailableInsertsRequest, EditResponse, InsertOptions, SaveRequest},
    proto_parser::proto_to_c_language_object,
};

#[derive(Debug, Default)]
pub struct CLengaService {
    files: Arc<Mutex<HashMap<Uuid, c::language_object::special_object::source_file::SourceFile>>>,
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
        let file_id =
            Uuid::parse_str(&req.id).map_err(|_err| Status::invalid_argument("Invalid id"))?;

        let mut files = self.files.lock().unwrap(); //TODO: Define how to de-poison lock
        let file = if let Some(file_ast) = files.get(&file_id) {
            file_ast.clone()
        } else {
            let mut file =
                File::open(&req.path).map_err(|err| Status::from_error(Box::new(err)))?;
            let mut content = Vec::new();
            BufReader::new(&mut file).read_to_end(&mut content).unwrap(); //TODO: recover or abort
            let c = C::new();
            let src_file = c.parse_nodes(content).map_err(Status::internal)?;

            files.insert(file_id, src_file.clone());

            src_file
        };

        let ast = source_file_to_proto(file);

        Ok(Response::new(ast))
    }

    async fn edit(&self, request: Request<EditRequest>) -> Result<Response<EditResponse>, Status> {
        let req = request.into_inner();

        let file_id =
            Uuid::parse_str(&req.id).map_err(|_err| Status::invalid_argument("Invalid id"))?;
        let edited_object = req
            .edited_object
            .ok_or(Status::invalid_argument("Inexistent edited_object field"))?;
        let edited_node =
            proto_to_c_language_object(edited_object).map_err(Status::invalid_argument)?;

        let mut files = self.files.lock().unwrap(); //TODO: Define how to de-poison lock
        match files.get_mut(&file_id) {
            Some(file_ast) => {
                let node_id = edited_node.id();
                let replaced = replace_source_file(file_ast, edited_node).ok_or_else(|| {
                    Status::failed_precondition(format!(
                        "Matching objects with id {node_id} not found"
                    ))
                })?;

                let ast = source_file_to_proto(file_ast.clone());
                let rep = c_language_object_to_proto(replaced);
                let res = proto::EditResponse {
                    new_object: Some(proto::LanguageObject {
                        language_object: Some(proto::language_object::LanguageObject::SourceFile(
                            ast,
                        )),
                    }),
                    old_object: Some(rep),
                };
                Ok(Response::new(res))
            }
            None => Err(Status::not_found(format!("File not found: {}", req.id))),
        }
    }

    async fn available_inserts(
        &self,
        request: Request<AvailableInsertsRequest>,
    ) -> Result<Response<InsertOptions>, Status> {
        let req = request.into_inner();
        let file_id =
            Uuid::parse_str(&req.id).map_err(|_err| Status::invalid_argument("Invalid id"))?;

        let mut files = self.files.lock().unwrap();
        match files.get_mut(&file_id) {
            Some(file_ast) => {
                let node_id = Uuid::parse_str(&req.node_id).unwrap();
                let parent = find_node(file_ast, node_id).ok_or_else(|| {
                    Status::failed_precondition(format!(
                        "Matching objects with id {node_id} not found"
                    ))
                })?;
                let options = parent.get_options(&req.node_key);
                let mapped_option = options
                    .into_iter()
                    .map(c_language_object_to_proto)
                    .collect();
                Ok(Response::new(InsertOptions {
                    options: mapped_option,
                }))
            }
            None => Err(Status::not_found(format!("File not found: {}", req.id))),
        }
    }

    async fn save(&self, request: Request<SaveRequest>) -> Result<Response<Void>, Status> {
        let req = request.into_inner();

        let file_id =
            Uuid::parse_str(&req.id).map_err(|_err| Status::invalid_argument("Invalid id"))?;
        let file_ast = {
            let files = self.files.lock().unwrap(); //TODO: Define how to de-poison lock
            files.get(&file_id).cloned()
        }
        .ok_or_else(|| Status::not_found(format!("File not found: {}", req.id)))?;

        let c = C::new();
        let output = c.write_to_nodes(file_ast).map_err(Status::data_loss)?;

        let path = Path::new(&req.write_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .await
                .map_err(|e| Status::internal(format!("Failed to create directories: {e}")))?;
        }

        let mut output_file =
            File::create(req.write_path).map_err(|err| Status::from_error(Box::new(err)))?;
        output_file
            .write_all(&output)
            .map_err(|err| Status::data_loss(err.to_string()))?;

        Ok(Response::new(proto::Void {}))
    }

    async fn close_file(&self, request: Request<CloseRequest>) -> Result<Response<Void>, Status> {
        let req = request.into_inner();

        let file_id =
            Uuid::parse_str(&req.id).map_err(|_err| Status::invalid_argument("Invalid id"))?;

        let mut files = self.files.lock().unwrap(); //TODO: Define how to de-poison lock
        files.remove(&file_id);

        Ok(Response::new(proto::Void {}))
    }
}

pub fn clenga_service() -> CLengaServer<CLengaService> {
    let c_lenga_service = CLengaService::default();
    CLengaServer::new(c_lenga_service)
}
