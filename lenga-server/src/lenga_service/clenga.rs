use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};
use uuid::Uuid;

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

#[derive(Debug, Default)]
pub struct CLengaService {
    asts: Arc<Mutex<HashMap<String, SourceFile>>>,
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

        let mut asts = self.asts.lock().unwrap();
        let ast = match asts.get(&req.path) {
            Some(file_ast) => file_ast.clone(),
            None => {
                let file_ast: SourceFile = todo!();
                asts.insert(req.path.clone(), file_ast.clone());
                file_ast
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
