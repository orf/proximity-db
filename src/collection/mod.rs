use nalgebra::allocator::Allocator;
use nalgebra::{DefaultAllocator, DimName, Point, Scalar};
use rayon::ThreadPool;

pub struct Collection<'a, F, DimX>
where
    F: Scalar,
    DimX: DimName,
    DefaultAllocator: Allocator<F, DimX>,
{
    name: String,
    points: Vec<Point<F, DimX>>,
    pool: &'a ThreadPool,
}

impl<F, DimX> Collection<'_, F, DimX>
where
    F: Scalar,
    DimX: DimName,
    DefaultAllocator: Allocator<F, DimX>,
{
    pub fn new<S: Into<String>>(name: S, pool: &ThreadPool) -> Collection<F, DimX> {
        Collection {
            name: name.into(),
            points: Vec::new(),
            pool,
        }
    }

    pub fn size() -> usize {
        return DimX::dim();
    }

    pub fn push(mut self, point: Point<F, DimX>) {
        self.points.push(point)
    }

    pub fn len(self) -> usize {
        self.points.len()
    }
}
