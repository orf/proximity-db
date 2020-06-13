use crate::Constellation;
use generic_array::{ArrayLength, GenericArray};
use rayon::prelude::*;
use std::sync::RwLock;

/// A slow, reference constellation.
#[derive(Default)]
pub struct SimpleConstellation<N: ArrayLength<f32>> {
    points: RwLock<Vec<GenericArray<f32, N>>>,
}

impl<N: ArrayLength<f32>> Constellation for SimpleConstellation<N> {
    fn add_points(&self, points: Vec<Vec<f32>>) {
        self.points
            .write()
            .expect("Error getting write lock")
            .extend(
                points
                    .into_iter()
                    .map(|p| GenericArray::<f32, N>::from_exact_iter(p).expect("Incorrect length")),
            );
    }

    fn find(&self, point: Vec<f32>, within: f32) -> Box<dyn Iterator<Item = (f32, Vec<f32>)>> {
        let arr = GenericArray::<f32, N>::from_exact_iter(point).expect("Incorrect length");
        // let results = arr![];
        let things: Vec<(f32, Vec<f32>)> = self
            .points
            .read()
            .expect("Error unwrapping points")
            .par_iter()
            .filter_map(|p| {
                let distance = p
                    .iter()
                    .zip(&arr)
                    .map(|(a, b)| (a - b).powf(2.))
                    .sum::<f32>()
                    .sqrt();
                if distance <= within {
                    return Some((distance, p.clone().into_iter().collect()));
                }
                None
            })
            .collect();

        Box::new(things.into_iter())
    }

    fn count(&self) -> usize {
        self.points.read().expect("Error getting read lock").len()
    }

    fn dimensions(&self) -> usize {
        N::to_usize()
    }

    fn memory_size(&self) -> usize {
        std::mem::size_of::<GenericArray<f32, N>>() * self.count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sizes::{U16, U4};

    #[test]
    fn test_len() {
        crate::tests::test_length(&SimpleConstellation::<U4>::default());
        crate::tests::test_length(&SimpleConstellation::<U16>::default());
    }

    #[test]
    fn test_mem() {
        crate::tests::test_mem_size(&SimpleConstellation::<U4>::default());
        crate::tests::test_mem_size(&SimpleConstellation::<U16>::default());
    }

    #[test]
    fn test_add_multiple() {
        crate::tests::test_add_multiple(&SimpleConstellation::<U4>::default());
        crate::tests::test_add_multiple(&SimpleConstellation::<U16>::default());
    }

    #[test]
    fn test_query() {
        crate::tests::test_query(&SimpleConstellation::<U16>::default());
    }
}
