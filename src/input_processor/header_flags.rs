use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodablePrimitive, DecodeCtx},
    quantities::UnsidedAtoms,
    require,
    types::Address,
};

const BYTE_COUNT: usize = 1;

pub struct HeaderFlags {
    /// Whether to read recipient address from payload. If false, use msg.sender as recipient.
    pub recipient_provided: bool,

    /// Whether to read msg.value from hostio
    pub track_msg_value: bool,

    /// Whether to process dynamic markets
    pub process_dynamic_markets: bool,

    /// Whether to read ETH withdraw amount from args and withdraw ETH
    pub withdraw_eth: bool,

    /// Whether to credit tokens to ERC20Store or EthStore, or to actually transfer out tokens
    pub withdraw_internally: bool,
}

impl Decodable for HeaderFlags {
    fn try_decode(ctx: &DecodeCtx) -> Result<Self, GoblinError> {
        require!(ctx.len() >= BYTE_COUNT, GoblinError::InvalidPayload);

        let byte_0 = u8::decode_unchecked_no_advance(ctx);
        let header = HeaderFlags {
            // Optional variables
            recipient_provided: (byte_0 & 0b0000_0001) != 0,
            track_msg_value: (byte_0 & 0b0000_0010) != 0,
            process_dynamic_markets: (byte_0 & 0b0000_0100) != 0,

            withdraw_eth: (byte_0 & 0b0000_1000) != 0,

            // Settlement flags
            withdraw_internally: (byte_0 & 0b0001_0000) != 0,
        };
        ctx.advance_offset(BYTE_COUNT);

        require!(
            ctx.len() >= header.payload_size(),
            GoblinError::InvalidPayload
        );

        Ok(header)
    }
}

impl HeaderFlags {
    pub fn payload_size(&self) -> usize {
        let size = BYTE_COUNT
            + self.recipient_provided as usize * core::mem::size_of::<Address>()
            + self.withdraw_eth as usize * core::mem::size_of::<UnsidedAtoms>();
        size
    }
}
