use crate::{
    input_processor::DecodePrimitive,
    quantities::{Exp, Quantity},
};

impl<D: Exp> DecodePrimitive for Quantity<D> {
    fn from_le_bytes_at(buffer: &[u8], offset: usize) -> Self {
        Self::new(u64::from_le_bytes_at(buffer, offset))
    }
}
