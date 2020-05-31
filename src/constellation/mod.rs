mod vec;

use crossbeam_channel::Receiver;
pub use vec::VecConstellation;

pub trait Constellation: Sync + Send {
    fn add_points(&self, points: Vec<Vec<f32>>);
    fn find(&self, point: Vec<f32>, within: f32) -> QueryIterator;

    fn count(&self) -> usize;
    fn dimensions(&self) -> usize;
    fn memory_size(&self) -> usize;
}

pub struct QueryIterator {
    receiver: Receiver<(f32, Vec<f32>)>,
}

impl Iterator for QueryIterator {
    type Item = (f32, Vec<f32>);

    fn next(&mut self) -> Option<Self::Item> {
        return self.receiver.recv().ok();
    }
}
