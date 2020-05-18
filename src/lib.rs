use crate::collection::Collection;
use nalgebra::{distance, Matrix2x3, MatrixMN, MatrixN, Point, Point3, VectorN, U1, U3};
use typenum::U256;

pub mod collection;
mod point;

// pub type Point256<f32> = Point<f32, U256>;

// pub type Collection256 = Collection<U256>;

// pub type MatrixThing = MatrixMN<f32, U1, U3>;

fn main() {
    let mut collection: Collection<f32, U3> = Collection::new("My Collection");
    // let vector : VectorN<f32, U3> = );
    for _ in 0..1_000_000 {
        collection
            .points
            .push(VectorN::<f32, U3>::from_vec(vec![1., 2., 3.]).into());
    }

    // collection.points.push(VectorN::<f32, U3>::from_vec(vec![2., 3., 4.,]).into());
    // collection.points.push(VectorN::<f32, U3>::from_vec(vec![3., 4., 5.,]).into());
    // collection.points.push(VectorN::<f32, U3>::from_vec(vec![0., 1., 1.,]).into());
    println!("Running... {}", collection.points.len());
    let our_point: Point<f32, U3> = VectorN::<f32, U3>::from_vec(vec![0., 0., 0.]).into();

    let mut total = 0;

    for point in collection.points {
        let result = distance(&point, &our_point);
        if result > 10.0 {
            total += 1;
        }
        // println!("{} = {}", point, result)
    }
    println!("point {}", total);
}
