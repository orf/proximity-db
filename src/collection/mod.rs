use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, DimName, Point, Scalar, VectorN};

// pub struct Collection<D>
//     where D: DimName,
//           DefaultAllocator: Allocator<f32, D>
// {
//     name: String,
//     points: Vec<VectorN<f32, D>>,
// }
//
// impl<D: DimName> Collection<D> {
//     pub fn new<S: Into<String>>(name: S) -> Collection<D> {
//         Collection {
//             name: name.into(),
//             points: Vec::new()
//         }
//     }
// }

pub struct Collection<F, DimX>
where
    F: Scalar,
    DimX: DimName,
    DefaultAllocator: Allocator<F, DimX>,
{
    name: String,
    pub points: Vec<Point<F, DimX>>,
}

impl<F, DimX> Collection<F, DimX>
where
    F: Scalar,
    DimX: DimName,
    DefaultAllocator: Allocator<F, DimX>,
{
    pub fn new<S: Into<String>>(name: S) -> Collection<F, DimX> {
        Collection {
            name: name.into(),
            points: Vec::new(),
        }
    }

    pub fn size() -> usize {
        return DimX::dim();
    }
}
