use criterion::measurement::WallTime;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use embedding_db::constellation::Constellation;
use nalgebra::allocator::Allocator;
use nalgebra::{ComplexField, DefaultAllocator, DimName, RealField, VectorN};
use nalgebra::{U32, U64};

use rand::distributions::Standard;
use rand::prelude::Distribution;
use std::time::Duration;
use typenum::{U128, U256, U512};

fn criterion_benchmark<DimX, DType>(group: &mut BenchmarkGroup<WallTime>)
where
    DimX: DimName,
    DType: ComplexField + RealField,
    DefaultAllocator: Allocator<DType, DimX>,
    <DefaultAllocator as Allocator<DType, DimX>>::Buffer: Send + Sync,
    Standard: Distribution<DType>,
{
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build()
        .unwrap();

    for size in (100_000..500_000).step_by(100_000) {
        let mut collection = Constellation::new();

        let random_points: Vec<_> = (0..size).map(|_| VectorN::new_random().into()).collect();

        collection.extend(&random_points);

        let our_point = VectorN::new_random().into();
        group.throughput(Throughput::Elements(collection.len() as u64));
        group.bench_function(
            BenchmarkId::new(
                format!("{}-{}", DimX::dim(), std::any::type_name::<DType>()),
                collection.len(),
            ),
            |b| {
                return pool.install(|| {
                    b.iter(|| return black_box(collection.find(&our_point, 0.5f32).len()));
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
    criterion_benchmark::<U64, f32>(&mut g);
    criterion_benchmark::<U128, f32>(&mut g);
    criterion_benchmark::<U256, f32>(&mut g);
    criterion_benchmark::<U512, f32>(&mut g);

    criterion_benchmark::<U64, f64>(&mut g);
    criterion_benchmark::<U128, f64>(&mut g);
    criterion_benchmark::<U256, f64>(&mut g);
    criterion_benchmark::<U512, f64>(&mut g);
    g.finish()
}

criterion_group!(benches, run_bench);
criterion_main!(benches);
