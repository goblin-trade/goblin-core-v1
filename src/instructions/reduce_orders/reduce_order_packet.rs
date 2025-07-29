use crate::{
    goblin_error::GoblinError, input_processor::ArgsBuffer, quantities::BaseLots, require,
};

#[repr(C, packed)]
pub struct ReduceOrderPacket {
    // TODO replace with a single MatrixPosition: u8
    pub row_index: u8,    // 5 bits
    pub column_index: u8, // 3 bits
    pub size: BaseLots,
}

impl ReduceOrderPacket {
    const BYTE_SIZE: usize = 1 + core::mem::size_of::<BaseLots>();

    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        require!(
            len >= *offset + Self::BYTE_SIZE,
            GoblinError::InvalidPayload
        );
        let packet = Self {
            row_index: (payload[*offset] & 0b1111_1000) >> 5,
            column_index: payload[*offset] & 0b0000_0111,
            size: BaseLots(u64::from_le_bytes([
                payload[*offset + 1],
                payload[*offset + 2],
                payload[*offset + 3],
                payload[*offset + 4],
                payload[*offset + 5],
                payload[*offset + 6],
                payload[*offset + 7],
                payload[*offset + 8],
            ])),
        };
        *offset += Self::BYTE_SIZE;

        Ok(packet)
    }
}
