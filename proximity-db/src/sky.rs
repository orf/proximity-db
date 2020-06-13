use crate::constellation_builder::ConstellationBuilder;
use crate::SupportedSize;
use dashmap::DashMap;
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use proximity::{Constellation, QueryIterator};

use thiserror::Error;
use tonic::{Code, Status};

#[derive(Error, Debug)]
pub enum SkyError {
    #[error("A vector with length {} is not valid. Valid sizes: {}", .0.number, SupportedSize::possible_choices())]
    InvalidSize(#[from] TryFromPrimitiveError<SupportedSize>),
    #[error(
        "Constellation {name:?} requires vectors with length {expected:?}, but you gave {given:?}"
    )]
    IncorrectSize {
        name: String,
        expected: usize,
        given: usize,
    },
    #[error("A constellation with the name {0} does not exist.")]
    NotFound(String),
}

impl From<SkyError> for Status {
    fn from(other: SkyError) -> Self {
        let msg = format!("{}", other);
        match other {
            SkyError::InvalidSize(..) => Status::new(Code::InvalidArgument, msg),
            SkyError::NotFound(..) => Status::new(Code::NotFound, msg),
            SkyError::IncorrectSize { .. } => Status::new(Code::InvalidArgument, msg),
        }
    }
}

// A sky contains lots of constellations?
// <S: Into<String>>
#[derive(Default)]
pub struct Sky {
    constellations: DashMap<String, Box<dyn Constellation>>,
}

impl<'a> Sky {
    pub fn add(&self, name: String, values: Vec<Vec<f32>>) -> Result<usize, SkyError> {
        if !values.len() == 0 {
            return Ok(0);
        }

        let supported_size = SupportedSize::try_from_primitive(values.first().unwrap().len())?;

        let constellation_rw = self
            .constellations
            .entry(name.clone())
            .or_insert_with(|| ConstellationBuilder::from(supported_size).build());

        let expected = constellation_rw.dimensions();
        for value in &values {
            if value.len() != expected {
                return Err(SkyError::IncorrectSize {
                    name: name.clone(),
                    expected,
                    given: value.len(),
                });
            }
        }
        let total_points = values.len();
        constellation_rw.add_points(values);
        return Ok(total_points);
    }

    pub fn query(
        &self,
        name: String,
        within_distance: f32,
        values: Vec<f32>,
    ) -> Result<QueryIterator, SkyError> {
        let constellation = self
            .constellations
            .get(&name)
            .ok_or_else(|| SkyError::NotFound(name.clone()))?;

        if constellation.dimensions() != values.len() {
            return Err(SkyError::IncorrectSize {
                name,
                expected: constellation.dimensions(),
                given: values.len(),
            });
        }

        Ok(constellation.find(values, within_distance))
    }

    pub fn list(&self, prefix: &String) -> Vec<Metrics> {
        self.constellations
            .iter()
            .filter_map(|kv| {
                if kv.key().starts_with(prefix) {
                    let value = kv.value();
                    Some(Metrics::from_constellation(kv.key().clone(), value))
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn describe(&self, name: &String) -> Result<Metrics, SkyError> {
        let constellation = self
            .constellations
            .get(name)
            .ok_or_else(|| SkyError::NotFound(name.clone()))?;

        return Ok(Metrics::from_constellation(name.clone(), &constellation));
    }
}

pub struct Metrics {
    pub name: String,
    pub count: usize,
    pub dimensions: usize,
    pub memory_size: usize,
}

impl Metrics {
    pub fn from_constellation(name: String, constellation: &Box<dyn Constellation>) -> Self {
        Self {
            name,
            count: constellation.count(),
            dimensions: constellation.dimensions(),
            memory_size: constellation.memory_size(),
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let sky = Sky::default();
        sky.add("hello".into(), vec![vec![1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0]])
            .unwrap();
    }

    #[test]
    fn test_query() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 1.0, 2.0, 3.0, 4.0];
        let sky = Sky::default();
        sky.add("hello".into(), vec![values.clone()]).unwrap();
        let receiver = sky.query("hello".into(), 0.0, values.clone()).unwrap();

        let items: Vec<(f32, Vec<f32>)> = receiver.collect();
        assert_eq!(items, vec![(0.0, values)]);
    }
}
