mod simple;
pub use simple::SimpleConstellation;

#[cfg(feature = "simd")]
mod vec;
#[cfg(feature = "simd")]
pub use vec::VecSIMDConstellation;

pub type QueryIterator = Box<dyn Iterator<Item = (f32, Vec<f32>)>>;

pub trait Constellation: Sync + Send {
    fn add_points(&self, points: Vec<Vec<f32>>);
    fn find(&self, point: Vec<f32>, within: f32) -> QueryIterator;

    fn count(&self) -> usize;
    fn dimensions(&self) -> usize;
    fn memory_size(&self) -> usize;
}
