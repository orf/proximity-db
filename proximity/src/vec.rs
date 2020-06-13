use crate::{Constellation, QueryIterator};
use bytemuck::cast;
use crossbeam_channel::bounded;
use nalgebra::allocator::Allocator;
use nalgebra::Point;
use nalgebra::{distance_squared, DefaultAllocator, DimName, VectorN};
use rayon::prelude::*;
use simba::simd::{SimdValue, WideF32x4};
use std::mem;
use std::sync::{Arc, RwLock};

pub type Point32<DimX> = Point<WideF32x4, DimX>;

fn make_point<DimX>(point: Vec<f32>) -> Point32<DimX>
    where
        DimX: DimName,
        DefaultAllocator: Allocator<WideF32x4, DimX>,
{
    let wide_vec: Vec<WideF32x4> = point
        .chunks(4)
        .map(|c| WideF32x4::from(<[f32; 4]>::from([c[0], c[1], c[2], c[3]])))
        .collect();
    VectorN::<WideF32x4, DimX>::from_vec(wide_vec).into()
}


/// A constellation contains lots of points.
pub struct VecSIMDConstellation<DimX>
    where
        DimX: DimName,
        DefaultAllocator: Allocator<WideF32x4, DimX>,
{
    points: Arc<RwLock<Vec<(u128, Point32<DimX>)>>>,
}

impl<DimX> Default for VecSIMDConstellation<DimX>
    where
        DimX: DimName,
        DefaultAllocator: Allocator<WideF32x4, DimX>,
{
    fn default() -> Self {
        VecSIMDConstellation {
            points: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl<DimX> Constellation for VecSIMDConstellation<DimX>
    where
        DimX: DimName + Sync,
        DefaultAllocator: Allocator<WideF32x4, DimX>,
        <DefaultAllocator as Allocator<WideF32x4, DimX>>::Buffer: Send + Sync,
{
    fn add_points(&self, points: Vec<(u128, Vec<f32>)>) {
        self.points
            .clone()
            .write()
            .unwrap()
            .extend(points.into_iter().map(|(id, p)| (id, make_point(p))))
    }

    fn find(&self, point: Vec<f32>, within: f32) -> QueryIterator {
        let point: Point32<DimX> = make_point(point);
        let (tx, rx) = bounded(100);
        let points = self.points.clone();

        std::thread::Builder::new()
            .name("find_iterate".to_string())
            .spawn(move || {
                points
                    .read()
                    .unwrap()
                    .par_iter()
                    .try_for_each_with(tx.clone(), |tx, (id, p)| {
                        let result = distance_squared(&point, &p);
                        let dist: f32 = cast::<_, [f32; 4]>(result.0).iter().sum();
                        if dist <= within {
                            // This seems absolutely horrible. Is there really not a better way?
                            let flat_coords: Vec<f32> = p
                                .coords
                                .iter()
                                .map(|p| cast::<_, [f32; 4]>(p.0).to_vec())
                                .flatten()
                                .collect();
                            return tx.send((dist, id.clone(), flat_coords));
                        }
                        Ok(())
                    })
                    .ok();
                // This is really important. Without this line there are sporadic stack overflows
                // with the benchmark - this thread doesn't terminate fast enough after `par_iter()`
                // finishes, and threads pile up.
                mem::drop(tx);
            })
            .unwrap();
        return QueryIterator { receiver: rx };
    }

    fn count(&self) -> usize {
        self.points.read().unwrap().len()
    }

    fn dimensions(&self) -> usize {
        DimX::dim() * WideF32x4::lanes()
    }

    fn memory_size(&self) -> usize {
        mem::size_of::<Point32<DimX>>() * self.count()
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use nalgebra::{U1, U2};

    #[test]
    fn test_len() {
        let constellation = VecSIMDConstellation::<U1>::default();
        assert_eq!(constellation.count(), 0);
    }

    #[test]
    fn test_size() {
        let constellation1 = VecSIMDConstellation::<U2>::default();
        constellation1.add_points(vec![vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]]);
        // Should be exactly 32 bytes
        assert_eq!(constellation1.memory_size(), 32);
    }

    #[test]
    fn test_add_multiple() {
        let constellation = VecSIMDConstellation::<U1>::default();
        let points: Vec<_> = vec![vec![1.0, 1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0, 1.0]];
        constellation.add_points(points);
        assert_eq!(constellation.count(), 2);
    }

    #[test]
    fn test_query() {
        let constellation = VecSIMDConstellation::<U1>::default();
        constellation.add_points(vec![vec![2.0, 2.0, 2.0, 2.0]]);
        let iterator = constellation.find(vec![1.0, 1.0, 1.0, 1.0], 10.);
        let items: Vec<(f32, Vec<f32>)> = iterator.collect();
        assert_eq!(items, vec![(4.0, vec![2.0, 2.0, 2.0, 2.0])]);
    }

    #[test]
    fn test_query_missing() {
        let constellation = VecSIMDConstellation::<U1>::default();
        constellation.add_points(vec![vec![2., 2., 2., 2.]]);
        let iterator = constellation.find(vec![1., 1., 1., 1.], 0.99);
        let items: Vec<(f32, Vec<f32>)> = iterator.collect();
        assert_eq!(items, vec![]);
    }
}
