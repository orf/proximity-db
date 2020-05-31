use embedding_db::grpc::embedding_db_client::EmbeddingDbClient;
use embedding_db::grpc::{AddRequest, Point as GrpcPoint, SearchRequest};
use rand::Rng;
use structopt::StructOpt;
use tonic::Request;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long)]
    debug: bool,

    /// Set speed
    // we don't want to name it "speed", need to look smart
    #[structopt(short = "v", long = "velocity", default_value = "42")]
    speed: f64,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(short)]
    out_type: String,

    /// File name: only required when `out` is set to `file`
    #[structopt(name = "FILE", required_if("out_type", "file"))]
    file_name: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();

    // let opt = Opt::from_args();
    // println!("{:?}", opt);
    let mut client = EmbeddingDbClient::connect("http://[::1]:50051").await?;
    for _ in 0..100 {
        let coords: Vec<f32> = (0..6).map(|_| rng.gen()).collect();
        client
            .add(Request::new(AddRequest {
                name: "test".to_string(),
                points: vec![GrpcPoint { coords }],
            }))
            .await?;
    }

    let random_items: Vec<f32> = (0..6).map(|_| rng.gen()).collect();
    let stream = client
        .search(Request::new(SearchRequest {
            distance: 1.0,
            name: "test".to_string(),
            point: Some(GrpcPoint {
                coords: random_items,
            }),
        }))
        .await?;

    let mut inbound = stream.into_inner();

    println!("Reading...");
    while let Some(feature) = inbound.message().await? {
        println!("NOTE = {:?}", feature);
    }

    Ok(())
}
