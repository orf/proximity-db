use criterion::measurement::WallTime;
use criterion::{
    criterion_group, criterion_main, BatchSize, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use embedding_db::sky::Sky;
use embedding_db::SupportedSize;
use enum_iterator::IntoEnumIterator;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Duration;

fn random_points(count: usize, dimension: usize) -> Vec<Vec<f32>> {
    let rng = rand::thread_rng();
    (0..count)
        .map(|_| rng.sample_iter(Standard).take(dimension).collect())
        .collect()
}

fn bench_search(group: &mut BenchmarkGroup<WallTime>, dimension: usize)
where
    Standard: Distribution<f32>,
{
    // let mut rng = thread_rng();

    for number_of_points in (100_000..=1_000_000).step_by(100_000) {
        //(250_000..1_000_000).step_by(250_000) {
        let sky = Sky::default();

        // Add 100,000 random vectors
        sky.add("test".into(), random_points(number_of_points, dimension))
            .expect("Error adding vectors");

        let random_point = random_points(1, dimension).first().unwrap().clone();

        // let iterator = sky.query("test".into(), 1., random_point).unwrap();
        // black_box(iterator.collect::<Vec<(f32, Vec<f32>)>>());
        group.throughput(Throughput::Elements(number_of_points as u64));
        group.bench_function(
            BenchmarkId::new(format!("{}", dimension), number_of_points),
            |b| {
                b.iter_batched(
                    || random_point.clone(),
                    |p| {
                        sky.query("test".to_string(), 0., p)
                            .unwrap()
                            .collect::<Vec<(f32, Vec<f32>)>>()
                    },
                    BatchSize::PerIteration,
                );
            },
        );
    }
}

fn run_bench(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .thread_name(|idx| format!("rayon-iter-{}", idx))
        .build_global()
        .unwrap();
    let mut g = c.benchmark_group("search_no_match");
    // g.warm_up_time(Duration::from_secs(0));
    g.measurement_time(Duration::from_secs(30));
    for size in SupportedSize::into_enum_iter() {
        bench_search(&mut g, size.into());
    }
    g.finish()
}

criterion_group!(benches, run_bench);
criterion_main!(benches);

//     for match_percent in (0..25).step_by(5) {
//         let match_percent = match_percent as f32;
//
//         let sky = Sky::default();
//         let search_point = &random_points(1, DimX::dim())[0];
//         let number_of_matches = if match_percent == 0.0 {
//             0
//         } else {
//             // This is fucking horrible, but I'm too fatigued to care.
//             let percent = match_percent / 100.0;
//             println!("Percent: {}%", percent);
//             (percent * (size as f32)) as usize
//         };
//         let number_of_non_matches = size - number_of_matches;
//
//         println!(
//             "Adding {} points, with {} matches and {} non matches",
//             size, number_of_matches, number_of_non_matches
//         );
//         println!(
//             "{} / 100 * {} ({}, {})",
//             match_percent, size, number_of_non_matches, number_of_matches
//         );
//
//         let matching_points: Vec<_> = vec![search_point.clone()]
//             .into_iter()
//             .cycle()
//             .take(number_of_matches)
//             .collect();
//         let non_matching_points: Vec<_> = random_points(number_of_non_matches, DimX::dim());
//         let mut combined_points: Vec<Vec<f32>> = matching_points
//             .into_iter()
//             .chain(non_matching_points)
//             .collect();
//         combined_points.shuffle(&mut rng);
//
//         sky.add("test_sky".to_string(), combined_points).unwrap();
//
//         group.throughput(Throughput::Elements(size as u64));
//         group.bench_function(
//             BenchmarkId::new(format!("{} - {}%", DimX::dim(), match_percent), size),
//             |b| {
//                 b.iter_batched(
//                     || search_point.clone(),
//                     |p| {
//                         sky.query("test_sky".to_string(), 0.0, p)
//                             .unwrap()
//                             .collect::<Vec<(f32, Vec<f32>)>>()
//                     },
//                     BatchSize::PerIteration,
//                 );
//             },
//         );
//     }
// }