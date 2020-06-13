use proximity_db::handler::ProximityDBHandler;
use proximity_db::sky::Sky;
use proximity_grpc::proximity_db_server::ProximityDbServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let sky = Sky::default();
    let embedding_handler = ProximityDBHandler::new(sky);

    Server::builder()
        .add_service(ProximityDbServer::new(embedding_handler))
        .serve(addr)
        .await?;

    Ok(())
}
