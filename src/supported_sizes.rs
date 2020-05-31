use crate::constellation::{Constellation, VecConstellation};
use enum_iterator::IntoEnumIterator;
use nalgebra::{U6, U64};
use num_enum::{IntoPrimitive, TryFromPrimitive};

use typenum::{U128, U256, U512};

#[derive(TryFromPrimitive, IntoPrimitive, IntoEnumIterator)]
#[repr(usize)]
pub enum SupportedSize {
    // For debugging
    U6 = 6,
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
            SupportedSize::U6 => Box::from(VecConstellation::<U6>::default()),
            SupportedSize::U64 => Box::from(VecConstellation::<U64>::default()),
            SupportedSize::U128 => Box::from(VecConstellation::<U128>::default()),
            SupportedSize::U256 => Box::from(VecConstellation::<U256>::default()),
            SupportedSize::U512 => Box::from(VecConstellation::<U512>::default()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_choices() {
        assert_eq!(SupportedSize::possible_choices(), "6, ");
    }
}
