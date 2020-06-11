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
use rand::{thread_rng, Rng};
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

    for number_of_points in (10_000..50_000).step_by(10_000) { //(250_000..1_000_000).step_by(250_000) {
        let sky = Sky::default();

        // Add 100,000 random vectors
        sky.add("test".into(), random_points(number_of_points, dimension))
            .expect("Error adding vectors");

        let random_point = random_points(1, dimension).first().unwrap().clone();

        // let iterator = sky.query("test".into(), 1., random_point).unwrap();
        // black_box(iterator.collect::<Vec<(f32, Vec<f32>)>>());
        group.throughput(Throughput::Elements(number_of_points as u64));
        group.bench_function(
            BenchmarkId::new(
                format!("{}", dimension),
                number_of_points,
            ),
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
    // g.warm_up_time(Duration::from_secs(10));
    // g.measurement_time(Duration::from_secs(30));
    for size in SupportedSize::into_enum_iter() {
        bench_search(&mut g, size.into());
    }
    g.finish()
}

criterion_group!(benches, run_bench);
criterion_main!(benches);
