use crate::lenga_service::start_lenga_service;

pub mod lenga_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lenga_service_addr = "127.0.0.1:49100".parse()?;
    start_lenga_service(lenga_service_addr).await?;

    Ok(())
}
