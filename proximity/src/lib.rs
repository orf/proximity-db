mod simple;
pub use simple::SimpleConstellation;

#[cfg(feature = "simd")]
mod vec;
#[cfg(feature = "simd")]
pub use vec::VecSIMDConstellation;

use typenum::Unsigned;

pub trait Constellation<T: Unsigned>: Sync + Send {
    fn add_points(&self, points: Vec<Vec<f32>>);
    fn find(&self, point: Vec<f32>, within: f32) -> Box<dyn Iterator<Item = (f32, Vec<f32>)>>;

    fn count(&self) -> usize;
    fn dimensions(&self) -> usize;
    fn memory_size(&self) -> usize;
}
