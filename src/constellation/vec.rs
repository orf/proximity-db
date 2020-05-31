use crate::constellation::{Constellation, QueryIterator};
use crossbeam_channel::bounded;
use nalgebra::allocator::Allocator;
use nalgebra::Point;
use nalgebra::{distance, DefaultAllocator, DimName, VectorN};
use rayon::prelude::*;
use std::mem;
use std::sync::{Arc, RwLock};

pub type Point32<DimX> = Point<f32, DimX>;

/// A constellation contains lots of points.
pub struct VecConstellation<DimX>
where
    DimX: DimName,
    DefaultAllocator: Allocator<f32, DimX>,
{
    points: Arc<RwLock<Vec<Point32<DimX>>>>,
}

impl<DimX> Default for VecConstellation<DimX>
where
    DimX: DimName,
    DefaultAllocator: Allocator<f32, DimX>,
{
    fn default() -> Self {
        VecConstellation {
            points: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl<DimX> Constellation for VecConstellation<DimX>
where
    DimX: DimName + Sync,
    DefaultAllocator: Allocator<f32, DimX>,
    <DefaultAllocator as Allocator<f32, DimX>>::Buffer: Send + Sync,
{
    fn add_points(&self, points: Vec<Vec<f32>>) {
        self.points.clone().write().unwrap().extend(
            points
                .into_iter()
                .map(|p| VectorN::<f32, DimX>::from_vec(p).into()),
        )
    }

    fn find(&self, point: Vec<f32>, within: f32) -> QueryIterator {
        let point: Point32<DimX> = VectorN::<f32, DimX>::from_vec(point).into();
        // let thing = Point32::<DimX>::from_slice(point);
        let (tx, rx) = bounded(100);
        let points = self.points.clone();
        std::thread::spawn(move || {
            points
                .read()
                .unwrap()
                .par_iter()
                .try_for_each_with(tx, |tx, p| {
                    let dist = distance(&point, &p);
                    if dist <= within {
                        return tx.send((dist, p.coords.as_slice().to_vec()));
                    }
                    Ok(())
                })
                .ok();
        });

        return QueryIterator { receiver: rx };
    }

    fn len(&self) -> usize {
        self.points.read().unwrap().len()
    }

    fn dimensions(&self) -> usize {
        DimX::dim()
    }

    fn memory_size(&self) -> usize {
        mem::size_of::<Point32<DimX>>() * self.len()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use nalgebra::{U1, U8};

    #[test]
    fn test_len() {
        let constellation = VecConstellation::<U1>::default();
        assert_eq!(constellation.len(), 0);
    }

    #[test]
    fn test_size() {
        let mut constellation1 = VecConstellation::<U8>::default();
        constellation1.add_points(&vec![Point32::<U8>::from_slice(&[
            1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
        ])]);
        // Should be exactly 32 bytes
        assert_eq!(constellation1.memory_size(), 32);
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
        constellation.add_points(&vec![Point32::<U1>::new(1.0)]);
        let target_point = Point32::<U1>::new(1.0);
        let iterator = constellation.find(target_point, 1.0);
        let items: Vec<(f32, Vec<f32>)> = iterator.collect();
        assert_eq!(items, vec![(0.0, vec![1.0])]);
    }
}
