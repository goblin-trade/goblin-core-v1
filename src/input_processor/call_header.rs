use crate::goblin_error::GoblinError;

use super::CallPayload;

pub struct CallHeader {
    /// Number of custom token addresses provided, maximum 15
    pub custom_token_count: usize,

    /// Number of token deltas to update, i.e. perform deposit or withdraw
    /// operations for these many tokens
    pub token_delta_count: usize,

    /// Whether to deposit or withdraw ETH
    pub track_eth_delta: bool,

    /// Whether to deposit shortfall amount during settlement
    pub deposit_shortfall: bool,

    /// Whether to read recipient address from payload. If false, use msg.sender
    pub recipient_provided: bool,

    /// Whether to update TraderTokenState for recipient, or to actually transfer out tokens
    pub withdraw_internally: bool,

    /// The number of collect fee instructions. Occupies 4 bits, max 2^4 - 1 = 15
    pub ix_collect_fee_count: u8,

    /// The number of post-only order instructions. Occupies entire byte. Max 2^8 - 1 = 255
    pub ix_post_only_count: u8,

    /// The number of take-only order instructions. Occupies 4 bits, max 2^4 - 1 = 15
    pub ix_take_only_count: u8,

    /// The number of limit order instructions. Occupies 4 bits, max 2^4 - 1 = 15
    pub ix_limit_order_count: u8,
}

impl CallHeader {
    pub fn init(payload: &mut CallPayload) -> Result<Self, GoblinError> {
        payload.advance_offset::<[u8; 4]>()?;

        let input = unsafe { payload.input_ref() };

        Ok(CallHeader {
            custom_token_count: (input[0] & 0b0000_1111) as usize,
            token_delta_count: (input[0] >> 4) as usize,

            track_eth_delta: (input[1] & 0b0000_0001) != 0,
            deposit_shortfall: (input[1] & 0b0000_0010) != 0,
            recipient_provided: (input[1] & 0b0000_0100) != 0,
            withdraw_internally: (input[1] & 0b0000_1000) != 0,
            ix_collect_fee_count: input[1] >> 4,

            ix_post_only_count: input[2],

            ix_take_only_count: input[3] & 0b0000_1111,
            ix_limit_order_count: input[3] >> 4,
        })
    }
}
