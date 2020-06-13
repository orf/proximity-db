use criterion::measurement::WallTime;
use criterion::{
    criterion_group, criterion_main, BatchSize, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use proximity::{Constellation, SIMDConstellation, SimpleConstellation};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::Rng;
use std::time::Duration;
use typenum::{U128, U16, U32, U512, U64};

fn random_points(count: usize, dimension: usize) -> Vec<Vec<f32>> {
    let rng = rand::thread_rng();
    (0..count)
        .map(|_| rng.sample_iter(Standard).take(dimension).collect())
        .collect()
}

fn bench_search(group: &mut BenchmarkGroup<WallTime>, factory: &dyn Fn() -> Box<dyn Constellation>)
where
    Standard: Distribution<f32>,
{
    // let mut rng = thread_rng();
    for number_of_points in (250_000..=1_000_000).step_by(250_000) {
        let constellation: Box<dyn Constellation> = factory();
        let dimension = constellation.dimensions();

        constellation.add_points(random_points(number_of_points, dimension));
        let random_point = random_points(1, dimension).first().unwrap().clone();

        group.throughput(Throughput::Elements(number_of_points as u64));
        group.bench_function(
            BenchmarkId::new(format!("dim: {}", dimension), number_of_points),
            |b| {
                b.iter_batched(
                    || random_point.clone(),
                    |p| constellation.find(p, 0.).collect::<Vec<(f32, Vec<f32>)>>(),
                    BatchSize::PerIteration,
                );
            },
        );
    }
}

fn run_bench(c: &mut Criterion) {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get() - 1)
        .thread_name(|idx| format!("rayon-iter-{}", idx))
        .build_global()
        .unwrap();
    let mut simple = c.benchmark_group("simple");
    simple.measurement_time(Duration::from_secs(30));
    bench_search(&mut simple, &|| {
        Box::new(SimpleConstellation::<U64>::default())
    });
    // Skip the middle benchmarks, to save time.
    // bench_search(&mut simple, &|| {
    //     Box::new(SimpleConstellation::<U128>::default())
    // });
    // bench_search(&mut simple, &|| {
    //     Box::new(SimpleConstellation::<U256>::default())
    // });
    bench_search(&mut simple, &|| {
        Box::new(SimpleConstellation::<U512>::default())
    });
    simple.finish();

    let mut simd = c.benchmark_group("simd");
    simd.measurement_time(Duration::from_secs(30));
    bench_search(&mut simd, &|| Box::new(SIMDConstellation::<U16>::default()));
    bench_search(&mut simd, &|| Box::new(SIMDConstellation::<U32>::default()));
    bench_search(&mut simd, &|| Box::new(SIMDConstellation::<U64>::default()));
    bench_search(
        &mut simd,
        &|| Box::new(SIMDConstellation::<U128>::default()),
    );
    simd.finish();
}

criterion_group!(benches, run_bench);
criterion_main!(benches);
