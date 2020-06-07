use criterion::measurement::WallTime;
use criterion::{
    criterion_group, criterion_main, BatchSize, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use nalgebra::allocator::Allocator;
use nalgebra::U64;
use nalgebra::{DefaultAllocator, DimName};

use embedding_db::sky::Sky;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::time::Duration;
use typenum::{U128, U256, U512};

fn random_points(count: usize, number: usize) -> Vec<Vec<f32>> {
    let rng = rand::thread_rng();
    (0..count)
        .map(|_| rng.sample_iter(Standard).take(number).collect())
        .collect()
}

fn bench_search<DimX>(group: &mut BenchmarkGroup<WallTime>)
where
    DimX: DimName,
    DefaultAllocator: Allocator<f32, DimX>,
    <DefaultAllocator as Allocator<f32, DimX>>::Buffer: Send + Sync,
    Standard: Distribution<f32>,
{
    let mut rng = thread_rng();
    for size in (250_000..1_000_000).step_by(250_000) {
        for match_percent in (0..100).step_by(25) {
            let sky = Sky::default();
            let search_point = &random_points(1, DimX::dim())[0];
            let number_of_matches = if match_percent == 0 {
                0
            } else {
                // This is fucking horrible, but I'm too fatigued to care.
                let percent : f32 = (match_percent as f32 / 100) as f32;
                println!("Percent: {}", percent);
                (percent * (size as f32)) as usize
            };
            let number_of_non_matches = size - number_of_matches;

            println!(
                "Adding {} points, with {} matches and {} non matches",
                size, number_of_matches, number_of_non_matches
            );
            println!("{} / 100 * {} ({}, {})", match_percent, size, number_of_non_matches, number_of_matches);

            let matching_points: Vec<_> = vec![search_point.clone()]
                .into_iter()
                .cycle()
                .take(number_of_matches)
                .collect();
            let non_matching_points: Vec<_> = random_points(number_of_non_matches, DimX::dim());
            let mut combined_points: Vec<Vec<f32>> = matching_points
                .into_iter()
                .chain(non_matching_points)
                .collect();
            combined_points.shuffle(&mut rng);

            sky.add("test_sky".to_string(), combined_points).unwrap();

            group.throughput(Throughput::Elements(size as u64));
            group.bench_function(
                BenchmarkId::new(format!("{} - {}%", DimX::dim(), match_percent), size),
                |b| {
                    b.iter_batched(
                        || search_point.clone(),
                        |p| {
                            sky.query("test_sky".to_string(), 0.0, p)
                                .unwrap()
                                .collect::<Vec<(f32, Vec<f32>)>>()
                        },
                        BatchSize::PerIteration,
                    );
                },
            );
        }
    }
}

fn run_bench(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build_global()
        .unwrap();
    let mut g = c.benchmark_group("bench_raw_search_single_match");
    g.warm_up_time(Duration::from_secs(10));
    g.measurement_time(Duration::from_secs(30));
    bench_search::<U64>(&mut g);
    bench_search::<U128>(&mut g);
    bench_search::<U256>(&mut g);
    bench_search::<U512>(&mut g);
    g.finish()
}

criterion_group!(benches, run_bench);
criterion_main!(benches);
