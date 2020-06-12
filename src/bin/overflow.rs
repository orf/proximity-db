use nalgebra::{distance_squared, Point, VectorN, U8};
use rand::distributions::Standard;
use rand::Rng;
use rayon::prelude::*;
use std::sync::{Arc, RwLock};

pub type Point32<DimX> = Point<f32, DimX>;

fn random_points(count: usize, dimension: usize) -> Vec<Point32<U8>> {
    let rng = rand::thread_rng();
    let vecs: Vec<Vec<f32>> = (0..count)
        .map(|_| rng.sample_iter(Standard).take(dimension).collect())
        .collect();

    vecs.into_iter()
        .map(|v| VectorN::<f32, U8>::from_vec(v).into())
        .collect()
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(40)
        .thread_name(|idx| format!("rayon-iter-{}", idx))
        .build_global()
        .unwrap();

    let points = Arc::new(RwLock::new(random_points(15_000, 8)));
    let point: Point32<U8> = random_points(1, 8).first().unwrap().clone();

    for _ in 0..100_000 {
        let loop_points = points.clone();
        loop_points.read().unwrap().par_iter().for_each(|p| {
            distance_squared(&point, &p);
        });
    }
}
