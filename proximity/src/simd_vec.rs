use crate::Constellation;
use bytemuck::cast;
use crossbeam_channel::bounded;
use nalgebra::{
    allocator::Allocator, distance, DefaultAllocator, DimName, NamedDim, Point, VectorN,
};
use rayon::prelude::*;
use simba::simd::{SimdValue, WideF32x4};
use std::sync::{Arc, RwLock};

pub type Point32<DimX> = Point<WideF32x4, DimX>;

fn make_point<DimX>(point: Vec<f32>) -> Point32<DimX::Name>
where
    DimX: NamedDim,
    DefaultAllocator: Allocator<WideF32x4, DimX::Name>,
{
    let wide_vec: Vec<WideF32x4> = point
        .chunks(4)
        .map(|c| WideF32x4::from([c[0], c[1], c[2], c[3]]))
        .collect();
    VectorN::<WideF32x4, DimX::Name>::from_vec(wide_vec).into()
}

/// A constellation contains lots of points.
pub struct SIMDConstellation<DimX>
where
    DimX: NamedDim,
    DefaultAllocator: Allocator<WideF32x4, DimX::Name>,
{
    points: Arc<RwLock<Vec<Point32<DimX::Name>>>>,
}

impl<DimX> Default for SIMDConstellation<DimX>
where
    DimX: NamedDim,
    DefaultAllocator: Allocator<WideF32x4, DimX::Name>,
{
    fn default() -> Self {
        SIMDConstellation {
            points: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl<DimX> Constellation for SIMDConstellation<DimX>
where
    DimX: NamedDim + Sync,
    DefaultAllocator: Allocator<WideF32x4, DimX::Name>,
    <DefaultAllocator as Allocator<WideF32x4, DimX::Name>>::Buffer: Send + Sync,
{
    fn add_points(&self, points: Vec<Vec<f32>>) {
        self.points
            .clone()
            .write()
            .unwrap()
            .extend(points.into_iter().map(make_point::<DimX>))
    }

    fn find(&self, point: Vec<f32>, within: f32) -> Box<dyn Iterator<Item = (f32, Vec<f32>)>> {
        let point = make_point::<DimX>(point);
        let (tx, rx) = bounded(100);
        let points = self.points.clone();

        std::thread::Builder::new()
            .name("find_iterate".to_string())
            .spawn(move || {
                points
                    .read()
                    .unwrap()
                    .par_iter()
                    .try_for_each_with(tx.clone(), |tx, p| {
                        let result = distance(&point, &p);
                        let dist: f32 = cast::<_, [f32; 4]>(result.0)
                            .iter()
                            .map(|i| i.powf(2.))
                            .sum::<f32>()
                            .sqrt();
                        if dist <= within {
                            // This seems absolutely horrible. Is there really not a better way?
                            let flat_coords: Vec<f32> = p
                                .coords
                                .iter()
                                .map(|p| cast::<_, [f32; 4]>(p.0).to_vec())
                                .flatten()
                                .collect();
                            return tx.send((dist, flat_coords));
                        }
                        Ok(())
                    })
                    .ok();
                // This is really important. Without this line there are sporadic stack overflows
                // with the benchmark - this thread doesn't terminate fast enough after `par_iter()`
                // finishes, and threads pile up.
                std::mem::drop(tx);
            })
            .expect("Error spawning iterator thread");
        Box::new(rx.into_iter())
    }

    fn count(&self) -> usize {
        self.points.read().unwrap().len()
    }

    fn dimensions(&self) -> usize {
        DimX::Name::dim() * WideF32x4::lanes()
    }

    fn memory_size(&self) -> usize {
        std::mem::size_of::<Point32<DimX::Name>>() * self.count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sizes::{U1, U16, U4};

    #[test]
    fn test_len() {
        crate::tests::test_length(&SIMDConstellation::<U1>::default());
        crate::tests::test_length(&SIMDConstellation::<U16>::default());
    }

    #[test]
    fn test_mem() {
        crate::tests::test_mem_size(&SIMDConstellation::<U1>::default());
        crate::tests::test_mem_size(&SIMDConstellation::<U16>::default());
    }

    #[test]
    fn test_add_multiple() {
        crate::tests::test_add_multiple(&SIMDConstellation::<U1>::default());
        crate::tests::test_add_multiple(&SIMDConstellation::<U16>::default());
    }

    #[test]
    fn test_query() {
        crate::tests::test_query(&SIMDConstellation::<U4>::default());
    }
}
