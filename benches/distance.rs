use criterion::measurement::WallTime;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use nalgebra::allocator::Allocator;
use nalgebra::{ComplexField, DefaultAllocator, DimName, RealField, VectorN};
use nalgebra::{U32, U64};

use crossbeam_channel::unbounded;
use embedding_db::constellations::{Constellation, VecConstellation};
use rand::distributions::Standard;
use rand::prelude::Distribution;
use std::time::Duration;
use typenum::{U128, U256, U512};

fn criterion_benchmark<DimX>(group: &mut BenchmarkGroup<WallTime>)
where
    DimX: DimName,
    DefaultAllocator: Allocator<f32, DimX>,
    <DefaultAllocator as Allocator<f32, DimX>>::Buffer: Send + Sync,
    Standard: Distribution<f32>,
{
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build()
        .unwrap();

    for size in (100_000..500_000).step_by(100_000) {
        let mut collection = VecConstellation::default();

        let random_points: Vec<_> = (0..size).map(|_| VectorN::new_random().into()).collect();

        collection.add_points(&random_points);

        let our_point = VectorN::new_random().into();
        group.throughput(Throughput::Elements(collection.len() as u64));
        group.bench_function(
            BenchmarkId::new(format!("{}", DimX::dim()), collection.len()),
            |b| {
                return pool.install(|| {
                    b.iter(|| {
                        let (send, recv) = unbounded();
                        black_box(collection.find_stream(&our_point, 0.1, send));
                    });
                });
            },
        );
        std::mem::drop(collection)
    }
}

fn run_bench(c: &mut Criterion) {
    let mut g = c.benchmark_group("Test");
    g.warm_up_time(Duration::from_secs(10));
    g.measurement_time(Duration::from_secs(30));
    criterion_benchmark::<U64>(&mut g);
    criterion_benchmark::<U128>(&mut g);
    criterion_benchmark::<U256>(&mut g);
    criterion_benchmark::<U512>(&mut g);
    g.finish()
}

criterion_group!(benches, run_bench);
criterion_main!(benches);
