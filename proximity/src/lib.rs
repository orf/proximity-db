mod simple;

pub use simple::SimpleConstellation;
pub use typenum::consts as sizes;

#[cfg(feature = "simd")]
mod simd_vec;

#[cfg(feature = "simd")]
pub use simd_vec::SIMDConstellation;

pub type QueryIterator = Box<dyn Iterator<Item = (f32, Vec<f32>)>>;

pub trait Constellation: Sync + Send {
    fn add_points(&self, points: Vec<Vec<f32>>);
    fn find(&self, point: Vec<f32>, within: f32) -> QueryIterator;

    fn count(&self) -> usize;
    fn dimensions(&self) -> usize;
    fn memory_size(&self) -> usize;
}

#[cfg(test)]
mod tests {
    use crate::Constellation;
    use std::iter;

    fn make_vec(dims: usize, value: f32) -> Vec<f32> {
        iter::repeat(value).take(dims).collect()
    }

    pub fn test_length(constellation: &dyn Constellation) {
        assert_eq!(constellation.count(), 0);
        let dims = constellation.dimensions();
        constellation.add_points(vec![make_vec(dims, 1.)]);
        assert_eq!(constellation.count(), 1);
    }

    pub fn test_mem_size(constellation: &dyn Constellation) {
        let dims = constellation.dimensions();
        constellation.add_points(vec![make_vec(dims, 1.)]);
        // Memory size should be exactly 4 bytes per dimension, i.e no overhead.
        assert_eq!(constellation.memory_size(), dims * 4);
    }

    pub fn test_add_multiple(constellation: &dyn Constellation) {
        let dims = constellation.dimensions();
        constellation.add_points(vec![make_vec(dims, 1.), make_vec(dims, 1.)]);
        assert_eq!(constellation.count(), 2);
    }

    pub fn test_query(constellation: &dyn Constellation) {
        assert_eq!(constellation.dimensions(), 16);
        let dims = constellation.dimensions();

        // Insert two vectors with repeated elements (1 and 10)
        constellation.add_points(vec![make_vec(dims, 1.), make_vec(dims, 10.)]);
        // Match against the vector full of 1's
        let items: Vec<(f32, Vec<f32>)> = constellation.find(make_vec(dims, 1.), 0.).collect();
        assert_eq!(items, vec![(0., make_vec(dims, 1.))]);

        let inner = vec![
            1., 2., 3., 4., 1., 2., 3., 4., 1., 2., 3., 4., 1., 2., 3., 4.,
        ];

        // The threaded version of this has a race condition where these are not always ordered.
        let mut items2: Vec<(f32, Vec<f32>)> = constellation.find(inner, 36.).collect();
        items2.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        assert_eq!(
            items2,
            vec![
                (7.483315, make_vec(dims, 1.)),
                (30.331501, make_vec(dims, 10.))
            ]
        );

        let items3: Vec<(f32, Vec<f32>)> = constellation.find(make_vec(dims, 0.), 0.).collect();
        assert_eq!(items3, vec![]);
    }
}
