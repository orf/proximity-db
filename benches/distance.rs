use approx::RelativeEq;
use criterion::measurement::WallTime;
use criterion::{
    black_box, criterion_group, criterion_main, BenchmarkGroup, BenchmarkId, Criterion, Throughput,
};
use embeddingdb;
use embeddingdb::collection::Collection;
use nalgebra::allocator::Allocator;
use nalgebra::{
    distance, DefaultAllocator, DimName, Matrix2x3, MatrixMN, MatrixN, Point, Point3, Scalar,
    VectorN, U1, U3,
};
use nalgebra::{U32, U64};
use num_traits::Float;
use num_traits::FromPrimitive;
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use simba;
use simba::scalar::{ComplexField, RealField};
use std::time::Duration;
use typenum::{U128, U256, U384, U512};

fn criterion_benchmark<DimX, DType>(group: &mut BenchmarkGroup<WallTime>)
where
    DimX: DimName,
    DType: ComplexField + RealField,
    // <DType as ComplexField>::RealField: ,
    DefaultAllocator: Allocator<DType, DimX>,
    <DefaultAllocator as Allocator<DType, DimX>>::Buffer: Send + Sync,
{
    let mut rng = thread_rng();
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(6)
        .build()
        .unwrap();

    for size in [10_000, 100_000, 250_000, 500_000, 1_000_000, 2_000_000].iter() {
        let mut collection: Collection<DType, DimX> =
            embeddingdb::collection::Collection::new("My Collection", &pool);
        for _ in 0..*size {
            // I HAVE NO FUCKING CLUE WHAT I AM DOING
            // WHAT THE FUCK IS ANY OF THIS CODE
            let mut random_vec: Vec<DType> = Vec::with_capacity(DimX::dim());
            for _ in 0..(DimX::dim()) {
                random_vec.push(DType::from_f64(rng.gen::<f64>()).unwrap());
            }
            collection.push(VectorN::<DType, DimX>::from_vec(random_vec).into());
        }
        let mut random_vec: Vec<DType> = Vec::with_capacity(DimX::dim());
        for _ in 0..(DimX::dim()) {
            random_vec.push(DType::from_f64(rng.gen::<f64>()).unwrap());
        }
        let our_point: Point<DType, DimX> = VectorN::<DType, DimX>::from_vec(random_vec).into();
        let comparison: <DType as ComplexField>::RealField = DType::from_f32(10.).unwrap();
        group.throughput(Throughput::Elements(collection.points.len() as u64));
        group.bench_with_input(
            BenchmarkId::new(
                format!("{}/{}", std::any::type_name::<DType>(), DimX::dim()),
                collection.len(),
            ),
            &collection.points,
            |b, points| {
                return pool.install(|| {
                    b.iter(|| {
                        return points
                            .into_par_iter()
                            .filter_map(|p| {
                                // simulate comparing
                                distance(&our_point, p).partial_cmp(&comparison)
                            })
                            // if we don't do this, nothing happens
                            .count();
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
    criterion_benchmark::<U32, f32>(&mut g);
    criterion_benchmark::<U64, f32>(&mut g);
    criterion_benchmark::<U128, f32>(&mut g);
    criterion_benchmark::<U256, f32>(&mut g);
    criterion_benchmark::<U384, f32>(&mut g);
    criterion_benchmark::<U512, f32>(&mut g);

    criterion_benchmark::<U32, f64>(&mut g);
    criterion_benchmark::<U64, f64>(&mut g);
    criterion_benchmark::<U128, f64>(&mut g);
    criterion_benchmark::<U256, f64>(&mut g);
    criterion_benchmark::<U384, f64>(&mut g);
    criterion_benchmark::<U512, f64>(&mut g);
    g.finish()
}

criterion_group!(benches, run_bench);
criterion_main!(benches);
