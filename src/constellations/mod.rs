use nalgebra::{DefaultAllocator, DimName};

mod vec;

use crossbeam_channel::Sender;
use nalgebra::allocator::Allocator;
use std::mem;

use crate::Point32;
pub use vec::VecConstellation;

pub trait Constellation<'a, DimX>: 'static + Default + Sync + Send
where
    DimX: DimName + Sync,
    DefaultAllocator: Allocator<f32, DimX>,
    <DefaultAllocator as Allocator<f32, DimX>>::Buffer: Send + Sync,
{
    fn add_point(&mut self, point: Point32<DimX>);
    fn add_points(&mut self, points: &[Point32<DimX>]);
    fn len(&self) -> usize;

    fn find_stream(&'a self, point: &Point32<DimX>, within: f32, sender: Sender<(f32, Vec<f32>)>);

    fn dimensions(&self) -> usize {
        DimX::dim()
    }
    fn memory_size(&self) -> usize {
        mem::size_of::<Point32<DimX>>() * self.len()
    }
}
