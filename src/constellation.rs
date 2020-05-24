use nalgebra::allocator::Allocator;
use nalgebra::{distance, ComplexField, DefaultAllocator, DimName, Point, RealField};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

/// A constellation contains lots of points.
pub struct Constellation<F, DimX>
where
    F: ComplexField + RealField,
    DimX: DimName,
    DefaultAllocator: Allocator<F, DimX>,
{
    points: Vec<Point<F, DimX>>,
}

impl<F, DimX> Constellation<F, DimX>
where
    F: ComplexField + RealField,
    DimX: DimName + Sync,
    DefaultAllocator: Allocator<F, DimX>,
    <DefaultAllocator as Allocator<F, DimX>>::Buffer: Send + Sync,
{
    pub fn new() -> Constellation<F, DimX> {
        Constellation { points: Vec::new() }
    }

    pub fn size() -> usize {
        return DimX::dim();
    }

    pub fn push(&mut self, point: Point<F, DimX>) {
        self.points.push(point)
    }

    pub fn extend(&mut self, points: &[Point<F, DimX>]) {
        self.points.extend_from_slice(points)
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn find(&self, point: &Point<F, DimX>, within: f32) -> Vec<&Point<F, DimX>> {
        let the_limit = F::from_f32(within).unwrap();
        return self
            .points
            .par_iter()
            .filter_map(|p| {
                let distance = distance(&point, p);
                if distance < the_limit {
                    Some(p)
                } else {
                    None
                }
            })
            .collect();
    }
}
