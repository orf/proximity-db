pub mod constellations;
pub mod grpc;
pub mod handler;
pub mod sky;

use enum_iterator::IntoEnumIterator;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(TryFromPrimitive, IntoPrimitive, IntoEnumIterator)]
#[repr(usize)]
pub enum SupportedSizes {
    // For debugging
    U6 = 6,
    //
    // U64 = 64,
    // U128 = 128,
    // U256 = 256,
    // U512 = 512,
}

impl SupportedSizes {
    fn possible_choices() -> String {
        return SupportedSizes::into_enum_iter().fold(String::new(), |a, b| {
            let value: usize = b.into();
            format!("{}{}, ", a, value)
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_choices() {
        assert_eq!(SupportedSizes::possible_choices(), "6, ");
    }
}
