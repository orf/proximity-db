use enum_iterator::IntoEnumIterator;
use num_enum::{IntoPrimitive, TryFromPrimitive, TryFromPrimitiveError};

use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

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
        assert_eq!(SupportedSize::possible_choices(), "4, 64, 128, 256, 512, ");
    }
}
