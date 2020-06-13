use crate::SupportedSize;
use proximity::{Constellation, VecSIMDConstellation};

use nalgebra::{U16, U2, U32, U64};
use typenum::U128;

pub struct ConstellationBuilder {
    size: SupportedSize,
}

impl ConstellationBuilder {
    pub fn new(size: SupportedSize) -> Self {
        ConstellationBuilder { size }
    }

    pub fn build(&self) -> Box<dyn Constellation> {
        match self.size {
            SupportedSize::U8 => Box::from(VecSIMDConstellation::<U2>::default()),
            SupportedSize::U64 => Box::from(VecSIMDConstellation::<U16>::default()),
            SupportedSize::U128 => Box::from(VecSIMDConstellation::<U32>::default()),
            SupportedSize::U256 => Box::from(VecSIMDConstellation::<U64>::default()),
            SupportedSize::U512 => Box::from(VecSIMDConstellation::<U128>::default()),
        }
    }
}

impl From<SupportedSize> for ConstellationBuilder {
    fn from(size: SupportedSize) -> Self {
        ConstellationBuilder::new(size)
    }
}
