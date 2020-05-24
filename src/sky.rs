use crate::constellation::Constellation;
use nalgebra::U64;
use num_enum::{TryFromPrimitive, TryFromPrimitiveError};
use std::collections::HashMap;
use thiserror::Error;
use typenum::{U128, U256, U512};

#[derive(Error, Debug)]
pub enum SkyError {
    #[error("A vector with length {} is not valid.", .0.number)]
    InvalidSize(#[from] TryFromPrimitiveError<SupportedSizes>),
}

#[derive(TryFromPrimitive)]
#[repr(usize)]
pub enum SupportedSizes {
    U64 = 64,
    U128 = 128,
    U256 = 256,
    U512 = 512,
}

// A sky contains lots of constellations?
// <S: Into<String>>
#[derive(Default)]
pub struct Sky {
    u64: HashMap<String, Constellation<f32, U64>>,
    u128: HashMap<String, Constellation<f32, U128>>,
    u256: HashMap<String, Constellation<f32, U256>>,
    u512: HashMap<String, Constellation<f32, U512>>,
}

impl Sky {
    pub fn add(&mut self, values: Vec<f32>) -> Result<(), SkyError> {
        let supported_size = SupportedSizes::try_from_primitive(values.len())?;
        match supported_size {
            SupportedSizes::U64 => self.u64.push(),
            SupportedSizes::U128 => self.u128.push(),
            SupportedSizes::U256 => self.u256.push(),
            SupportedSizes::U512 => self.u512.push()
        }
        return Ok(());
    }
}
