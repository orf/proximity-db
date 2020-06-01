use embedding_db::grpc::embedding_db_client::EmbeddingDbClient;
use embedding_db::grpc::{AddRequest, ListRequest, Point as GrpcPoint, SearchRequest};
use futures::StreamExt;
use human_format::{Formatter, Scales};
use rand::distributions::Standard;
use rand::Rng;
use stats::MinMax;
use std::time::Instant;
use structopt::StructOpt;
use tonic::transport::Channel;
use tonic::Request;

#[derive(Debug, StructOpt)]
#[structopt(name = "example", about = "An example of StructOpt usage.")]
enum Opt {
    Fill {
        name: String,
        #[structopt(short, long)]
        dimensions: usize,
        #[structopt(short, long)]
        number: usize,
        #[structopt(short, long, default_value = "30")]
        parallel: usize,
        #[structopt(short, long, default_value = "1")]
        batch_size: usize,
    },

    List {
        #[structopt(default_value = "")]
        prefix: String,
    },

    Search {
        name: String,
        #[structopt(short, long)]
        dimensions: usize,
        #[structopt(short, long, default_value = "0.1")]
        within: f32,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let client = EmbeddingDbClient::connect("http://[::1]:50051").await?;
    match opt {
        Opt::Fill {
            name,
            dimensions,
            number,
            parallel,
            batch_size,
        } => fill(client, name, dimensions, number, parallel, batch_size).await,
        Opt::List { prefix } => list(client, prefix).await,
        Opt::Search {
            name,
            dimensions,
            within,
        } => search(client, name, dimensions, within).await,
    }
}

async fn search(
    mut client: EmbeddingDbClient<Channel>,
    name: String,
    dimensions: usize,
    within: f32,
) -> anyhow::Result<()> {
    let rng = rand::thread_rng();
    let random_point = GrpcPoint {
        coords: rng.sample_iter(Standard).take(dimensions).collect(),
    };

    let result_stream = client
        .search(Request::new(SearchRequest {
            distance: within,
            name,
            point: Some(random_point),
        }))
        .await?;

    let mut inbound = result_stream.into_inner();

    let mut stats = MinMax::new();
    println!("Searching...");
    let now = Instant::now();
    while let Some(feature) = inbound.message().await? {
        stats.add(feature.distance);
    }
    println!("Elapsed: {:?}", now.elapsed());
    println!("Total results: {}", stats.len());
    println!("Min distance: {:#?}", stats.min());
    println!("Max distance: {:#?}", stats.max());

    Ok(())
}

async fn fill(
    client: EmbeddingDbClient<Channel>,
    name: String,
    dimensions: usize,
    number: usize,
    parallel: usize,
    batch_size: usize,
) -> anyhow::Result<()> {
    let rng = rand::thread_rng();

    // Create our random points
    let mut items: Vec<Vec<GrpcPoint>> = vec![];
    for _ in (0..number).step_by(batch_size) {
        items.push(
            (0..batch_size)
                .map(|_| GrpcPoint {
                    coords: rng.sample_iter(Standard).take(dimensions).collect(),
                })
                .collect(),
        );
    }

    let mut request_futures = futures::stream::iter(items.into_iter().map(|batch| {
        async {
            // This is super cheap, see https://github.com/hyperium/tonic/issues/285
            let mut cloned_client = client.clone();
            cloned_client
                .add(Request::new(futures::stream::iter(vec![AddRequest {
                    name: name.clone(),
                    points: batch,
                }])))
                .await?;
            // See https://github.com/rust-lang/rust/issues/63502#issuecomment-520647948
            Ok::<(), anyhow::Error>(())
        }
    }))
    .buffer_unordered(parallel);

    println!(
        "Sending {} batches of {} points, with {} parallel requests",
        number, batch_size, parallel
    );
    let started = Instant::now();
    while let Some(feature) = request_futures.next().await {
        feature?;
    }
    let elapsed = started.elapsed();
    println!("Completed in {}ms", elapsed.as_millis());
    println!(
        "{} adds per second",
        (number as u128 / elapsed.as_millis()) * 1000
    );
    Ok(())
}

async fn list(mut client: EmbeddingDbClient<Channel>, prefix: String) -> anyhow::Result<()> {
    let mut bytes_formatter = Formatter::new();
    bytes_formatter.with_scales(Scales::Binary());

    let mut count_formatter = Formatter::new();
    count_formatter.with_decimals(1);

    let result = client.list(Request::new(ListRequest { prefix })).await?;
    let mut result_stream = result.into_inner();
    while let Some(feature) = result_stream.message().await? {
        println!(" - name : {}", feature.name);
        println!("   dims : {}", feature.dimensions);
        println!("   count: {}", count_formatter.format(feature.count as f64));
        println!(
            "   size : {}",
            bytes_formatter.format(feature.memory_size as f64)
        );
    }
    Ok(())
}

//
// let items : Vec<Vec<Vec<f32>>> = (0..count).step_by(batch_size).map(|_| ).collect();
//
// let start = Instant::now();
//
// let futures = items.chunks(batch_size).map(|coords| {
//     client.add(Request::new(AddRequest { name: name.clone(), points: vec![GrpcPoint { coords }] }))
// });

// for _ in 0..count {
//     client.add(Request::new(
//         AddRequest {
//             name,
//             points: vec![GrpcPoint { coords }]
//         }
//     ))
// }

// for _ in 0..100 {
//     let coords: Vec<f32> = (0..6).map(|_| rng.gen()).collect();
//     client
//         .add(Request::new(AddRequest {
//             name: "test".to_string(),
//             points: vec![GrpcPoint { coords }],
//         }))
//         .await?;
// }
//
// let random_items: Vec<f32> = (0..6).map(|_| rng.gen()).collect();
