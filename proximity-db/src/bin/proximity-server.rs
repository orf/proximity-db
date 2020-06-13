use proximity_db::handler::ProximityDBHandler;
use proximity_db::sky::Sky;
use proximity_grpc::proximity_db_server::ProximityDbServer;
use tonic::transport::Server;
use structopt::StructOpt;
use num_cpus;

#[derive(Debug, StructOpt)]
#[structopt(about = "Run a proximity database instance.")]
struct Opt {
    #[structopt(short, long, default_value = "[::1]:4321", env = "PROXIMITY_ADDRESS")]
    /// The interface and port that Proximity will listen on
    address: String,
    #[structopt(short, long, env = "PROXIMITY_THREADS")]
    /// Specifies how many threads Proximity DB will use. Defaults to CPU count - 1
    threads: Option<usize>
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let addr = opt.address.parse()?;
    let threads = opt.threads.unwrap_or_else(|| num_cpus::get() - 1);

    rayon::ThreadPoolBuilder::new()
        .num_threads(threads)
        .thread_name(|idx| format!("rayon-iter-{}", idx))
        .build_global()
        .unwrap();

    let sky = Sky::default();
    let embedding_handler = ProximityDBHandler::new(sky);

    Server::builder()
        .add_service(ProximityDbServer::new(embedding_handler))
        .serve(addr)
        .await?;

    Ok(())
}
