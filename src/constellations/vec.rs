use crate::constellations::Constellation;
use crossbeam_channel::Sender;
use nalgebra::allocator::Allocator;
use nalgebra::{distance, ComplexField, DefaultAllocator, DimName, Point, RealField};
use rayon::prelude::*;

/// A constellation contains lots of points.
pub struct VecConstellation<DimX>
where
    DimX: DimName,
    DefaultAllocator: Allocator<f32, DimX>,
{
    points: Vec<Point<f32, DimX>>,
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
    fn add_point(&mut self, point: Point<f32, DimX>) {
        self.points.push(point)
    }

    fn add_points(&mut self, points: &[Point<f32, DimX>]) {
        self.points.extend_from_slice(points)
    }

    fn len(&self) -> usize {
        self.points.len()
    }

    fn find_stream(
        &'a self,
        point: &Point<f32, DimX>,
        within: f32,
        sender: Sender<(f32, Vec<f32>)>,
    ) {
        // let the_limit = F::from_f32(within).unwrap();
        // let (sender2, receiver) = channel();
        self.points
            .par_iter()
            .try_for_each_with(sender, |s, p| {
                let dist = distance(&point, &p);
                if dist < within {
                    return s.send((dist, p.coords.as_slice().into()));
                }
                Ok(())
            })
            .ok();
    }
}

// https://github.com/hyperium/tonic/blob/6f378e2bd0cdf3a1a3df87e1feff842a8a599142/tonic-health/src/server.rs#L156
