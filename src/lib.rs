pub mod constellations;
pub mod grpc;
pub mod handler;
pub mod help;
pub mod sky;

use enum_iterator::IntoEnumIterator;
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(TryFromPrimitive, IntoPrimitive, IntoEnumIterator)]
#[repr(usize)]
pub enum SupportedSizes {
    // For debugging
    U3 = 3,
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
