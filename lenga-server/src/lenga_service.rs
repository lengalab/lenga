// declaring services
pub mod clenga;

// importing services
use clenga::clenga_service;

use std::net::SocketAddr;
use tonic::{Request, Response, Status, transport::Server};

mod proto {
    include!("../rpc/generated/lenga.rs");
}

use proto::{
    LanguageRequest, LanguageSupport, SuportedLanguages, Void,
    lenga_server::{Lenga, LengaServer},
};

#[derive(Debug)]
pub struct LengaService {
    supported_languages: Vec<String>,
}

impl LengaService {
    pub fn new(supported_languages: Vec<String>) -> LengaService {
        LengaService {
            supported_languages,
        }
    }
}

#[tonic::async_trait]
impl Lenga for LengaService {
    async fn list_languages(
        &self,
        _request: Request<Void>,
    ) -> Result<Response<SuportedLanguages>, Status> {
        Ok(Response::new(SuportedLanguages {
            languages: self.supported_languages.clone(),
        }))
    }

    async fn check_language(
        &self,
        request: Request<LanguageRequest>,
    ) -> Result<Response<LanguageSupport>, Status> {
        let inner = request.into_inner();
        Ok(Response::new(LanguageSupport {
            supported: self.supported_languages.contains(&inner.language),
        }))
    }
}

pub async fn start_lenga_service(
    lenga_service_addr: SocketAddr,
) -> Result<(), Box<dyn std::error::Error>> {
    let lenga_service = LengaService::new(vec!["c".to_string()]);
    Server::builder()
        .add_service(LengaServer::new(lenga_service))
        .add_service(clenga_service())
        // .add_service(other_service())
        .serve(lenga_service_addr)
        .await?;
    Ok(())
}
