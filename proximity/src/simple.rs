use crate::Constellation;
use generic_array::{ArrayLength, GenericArray};
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
            .iter()
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
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use typenum::{U4, U8};

    #[test]
    fn test_len() {
        let constellation = SimpleConstellation::<U4>::default();
        assert_eq!(constellation.count(), 0);
    }

    #[test]
    fn test_mem_size() {
        let constellation1 = SimpleConstellation::<U8>::default();
        constellation1.add_points(vec![vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0]]);
        // Should be exactly 32 bytes
        assert_eq!(constellation1.memory_size(), 32);
    }

    #[test]
    fn test_add_multiple() {
        let constellation = SimpleConstellation::<U4>::default();
        let points: Vec<_> = vec![vec![1.0, 1.0, 1.0, 1.0], vec![1.0, 1.0, 1.0, 1.0]];
        constellation.add_points(points);
        assert_eq!(constellation.count(), 2);
    }

    #[test]
    fn test_query() {
        let constellation = SimpleConstellation::<U4>::default();
        constellation.add_points(vec![vec![2.0, 2.0, 2.0, 2.0]]);
        let iterator = constellation.find(vec![1.0, 1.0, 1.0, 1.0], 10.);
        let items: Vec<(f32, Vec<f32>)> = iterator.collect();
        assert_eq!(items, vec![(2.0, vec![2.0, 2.0, 2.0, 2.0])]);
    }

    #[test]
    fn test_query_missing() {
        let constellation = SimpleConstellation::<U4>::default();
        constellation.add_points(vec![vec![2., 2., 2., 2.]]);
        let iterator = constellation.find(vec![1., 1., 1., 1.], 0.99);
        let items: Vec<(f32, Vec<f32>)> = iterator.collect();
        assert_eq!(items, vec![]);
    }
}
