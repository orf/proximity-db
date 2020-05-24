use crate::constellations::{Constellation, VecConstellation};
use crate::SupportedSizes;
use crossbeam_channel::Sender;
use nalgebra::{ComplexField, DimName, Point, RealField, Vector1, VectorN, U1, U3, U64};
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use rayon::prelude::*;
use std::collections::HashMap;

use std::sync::{Arc, RwLock};
use std::thread::spawn;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SkyError {
    #[error("A vector with length {} is not valid. Valid sizes: {}", .0.number, SupportedSizes::possible_choices())]
    InvalidSize(#[from] TryFromPrimitiveError<SupportedSizes>),
    #[error("A constellation with the name {0} and size {1} does not exist.")]
    NotFound(String, usize),
}

// A sky contains lots of constellations?
// <S: Into<String>>
#[derive(Default)]
pub struct Sky {
    // For debugging!
    u3: HashMap<String, Arc<RwLock<VecConstellation<U3>>>>,
    // u64: HashMap<String, VecConstellation<U64>>,
    // u128: HashMap<String, VecConstellation<U128>>,
    // u256: HashMap<String, VecConstellation<U256>>,
    // u512: HashMap<String, VecConstellation<U512>>,
}

impl<'a> Sky {
    pub fn add(&mut self, name: String, values: Vec<f32>) -> Result<(), SkyError> {
        let supported_size = SupportedSizes::try_from_primitive(values.len())?;

        match supported_size {
            SupportedSizes::U3 => {
                let point = Point::<f32, U3>::from_slice(&values);
                let mut thing = self.u3.entry(name).or_default().write().unwrap();
                thing.add_point(point);
            }
        }
        return Ok(());
    }

    pub fn query(
        &'a self,
        name: String,
        within_distance: f32,
        values: Vec<f32>,
        sender: Sender<(f32, Vec<f32>)>,
    ) -> Result<(), SkyError> {
        let supported_size = SupportedSizes::try_from_primitive(values.len())?;
        match supported_size {
            SupportedSizes::U3 => {
                let constellation = self
                    .u3
                    .get(&name)
                    .ok_or_else(|| SkyError::NotFound(name.clone(), values.len()))?
                    .clone();
                let point = Point::<f32, U3>::from_slice(&values);
                spawn(move || {
                    let reader = constellation.read().unwrap();
                    reader.find_stream(&point, within_distance, sender);
                });
            }
        }

        Ok(())
    }
}
