use crate::constellations::Constellation;
use crate::Point32;
use crossbeam_channel::Sender;
use nalgebra::allocator::Allocator;
use nalgebra::{distance, DefaultAllocator, DimName};
use rayon::prelude::*;

/// A constellation contains lots of points.
pub struct VecConstellation<DimX>
where
    DimX: DimName,
    DefaultAllocator: Allocator<f32, DimX>,
{
    points: Vec<Point32<DimX>>,
}

impl<DimX> Default for VecConstellation<DimX>
where
    DimX: DimName,
    DefaultAllocator: Allocator<f32, DimX>,
{
    fn default() -> Self {
        VecConstellation { points: Vec::new() }
    }
}

impl<'a, DimX> Constellation<'a, DimX> for VecConstellation<DimX>
where
    DimX: DimName + Sync,
    DefaultAllocator: Allocator<f32, DimX>,
    <DefaultAllocator as Allocator<f32, DimX>>::Buffer: Send + Sync,
{
    fn add_point(&mut self, point: Point32<DimX>) {
        self.points.push(point)
    }

    fn add_points(&mut self, points: &[Point32<DimX>]) {
        self.points.extend_from_slice(points)
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn find_stream(&'a self, point: &Point32<DimX>, within: f32, sender: Sender<(f32, Vec<f32>)>) {
        self.points
            .par_iter()
            .try_for_each_with(sender, |s, p| {
                let dist = distance(&point, &p);
                if dist <= within {
                    return s.send((dist, p.coords.as_slice().into()));
                }
                Ok(())
            })
            .ok();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crossbeam_channel::bounded;
    use nalgebra::{U1, U8};

    #[test]
    fn test_len() {
        let constellation = VecConstellation::<U1>::default();
        assert_eq!(constellation.len(), 0);
    }

    #[test]
    fn test_size() {
        let mut constellation1 = VecConstellation::<U8>::default();
        constellation1.add_point(Point32::<U8>::from_slice(&[
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ]));
        // Should be exactly 32 bytes
        assert_eq!(constellation1.memory_size(), 32);
    }

    #[test]
    fn test_add() {
        let mut constellation = VecConstellation::<U1>::default();
        constellation.add_point(Point32::<U1>::new(1.0));
        assert_eq!(constellation.len(), 1);
    }

    #[test]
    fn test_add_multiple() {
        let mut constellation = VecConstellation::<U1>::default();
        let points: Vec<_> = vec![Point32::<U1>::new(1.0), Point32::<U1>::new(1.0)];
        constellation.add_points(&points);
        assert_eq!(constellation.len(), 2);
    }

    #[test]
    fn test_query() {
        let mut constellation = VecConstellation::<U1>::default();
        constellation.add_point(Point32::<U1>::new(1.0));
        let (sender, receiver) = bounded(1);
        let target_point = Point32::<U1>::new(1.0);
        constellation.find_stream(&target_point, 1.0, sender);
        let items: Vec<(f32, Vec<f32>)> = receiver.iter().collect();
        assert_eq!(items, vec![(0.0, vec![1.0])]);
    }
}
