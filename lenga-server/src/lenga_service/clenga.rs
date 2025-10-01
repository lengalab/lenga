use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
use uuid::Uuid;

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

// Objects
use proto::{UnknownNode, language_object::LanguageObject as CLanguageObject};

use crate::lenga_service::clenga::node_parser::parse_file;

#[derive(Debug)]
pub struct CLengaService {
    files: Arc<Mutex<HashMap<String, SourceFile>>>,
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
        let ast = match files.get(&req.path) {
            Some(file_ast) => file_ast.clone(),
            None => {
                let file = parse_file(&req.path);
                files.insert(req.path.clone(), file.clone());
                file
            }
        };

        Ok(Response::new(ast))
    }

    async fn edit(
        &self,
        request: Request<EditRequest>,
    ) -> Result<Response<LanguageObject>, Status> {
        let req = request.into_inner();

        let res = LanguageObject {
            language_object: Some(CLanguageObject::UnknownNode(UnknownNode {
                id: "edit".to_string(),
                contents: req.edit_data,
            })),
        };

        Ok(Response::new(res))
    }
}

pub fn clenga_service() -> CLengaServer<CLengaService> {
    let c_lenga_service = CLengaService::default();
    CLengaServer::new(c_lenga_service)
}
