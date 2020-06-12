use crate::constellation::{Constellation, VecConstellation};
use enum_iterator::IntoEnumIterator;
use nalgebra::{U64, U8};
use num_enum::{IntoPrimitive, TryFromPrimitive, TryFromPrimitiveError};

use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;
use typenum::{U128, U256, U512};

#[derive(TryFromPrimitive, IntoPrimitive, IntoEnumIterator, Debug)]
#[repr(usize)]
pub enum SupportedSize {
    // For debugging
    U8 = 8,
    // Usual sizes
    U64 = 64,
    U128 = 128,
    U256 = 256,
    U512 = 512,
}

impl SupportedSize {
    pub fn possible_choices() -> String {
        return SupportedSize::into_enum_iter().fold(String::new(), |a, b| {
            let value: usize = b.into();
            format!("{}{}, ", a, value)
        });
    }
}

impl Into<Box<dyn Constellation>> for SupportedSize {
    fn into(self) -> Box<dyn Constellation> {
        // I don't know how to move this into the VecConstellation struct :'(
        match self {
            SupportedSize::U8 => Box::from(VecConstellation::<U8>::default()),
            SupportedSize::U64 => Box::from(VecConstellation::<U64>::default()),
            SupportedSize::U128 => Box::from(VecConstellation::<U128>::default()),
            SupportedSize::U256 => Box::from(VecConstellation::<U256>::default()),
            SupportedSize::U512 => Box::from(VecConstellation::<U512>::default()),
        }
    }
}

#[derive(Error, Debug)]
pub enum SupportedSizeConversionError {
    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),
    #[error("Value {} is not valid. Valid values: {}", .0.number, SupportedSize::possible_choices())]
    InvalidSize(#[from] TryFromPrimitiveError<SupportedSize>),
}

impl FromStr for SupportedSize {
    type Err = SupportedSizeConversionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let target: usize = s.parse()?;
        return Ok(SupportedSize::try_from_primitive(target)?);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_choices() {
        assert_eq!(SupportedSize::possible_choices(), "6, 64, 128, 256, 512, ");
    }
}
