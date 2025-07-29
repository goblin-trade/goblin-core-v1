use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::MarketIndex,
    require,
};

pub struct ReduceOrdersHeader {
    pub market_index: MarketIndex,

    pub bid_outer_indices: u8,

    pub ask_outer_indices: u8,
}

impl ReduceOrdersHeader {
    const BYTE_SIZE: usize = 2;

    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        require!(
            len >= *offset + Self::BYTE_SIZE,
            GoblinError::InvalidPayload
        );
        let header = Self {
            market_index: MarketIndex(payload[*offset]),
            bid_outer_indices: payload[*offset + 1] & 0b0000_1111,
            ask_outer_indices: (payload[*offset + 1] & 0b1111_0000) >> 4,
        };
        *offset += Self::BYTE_SIZE;

        Ok(header)
    }
}

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MatrixHeader {
    pub outer_index: u16,
    pub order_count: u8,
}

impl MatrixHeader {
    const BYTE_SIZE: usize = core::mem::size_of::<Self>();

    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        require!(
            len >= *offset + Self::BYTE_SIZE,
            GoblinError::InvalidPayload
        );
        let header = payload.decode_ref::<MatrixHeader>(*offset);
        *offset += Self::BYTE_SIZE;

        Ok(*header)
    }
}
