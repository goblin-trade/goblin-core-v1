use crate::{
    goblin_error::GoblinError, quantities::Atoms, settlement::TokenWithdrawalDue, types::Address,
};

use super::CallPayload;

pub struct CallHeader {
    /// Number of custom token addresses provided, maximum 15
    pub custom_token_count: usize,

    /// Number of token deltas to update, i.e. perform deposit or withdraw
    /// operations for these many tokens
    pub token_delta_count: usize,

    /// Whether to read recipient address from payload. If false, use msg.sender as recipient.
    pub recipient_provided: bool,

    /// Whether to read msg.value from hostio
    pub track_msg_value: bool,

    /// Whether to read eth_withdrawal_due from payload
    pub track_eth_withdrawal_due: bool,

    /// Whether to deposit shortfall amount during settlement
    pub deposit_shortfall: bool,

    /// Whether to update TraderTokenState for recipient, or to actually transfer out tokens
    pub withdraw_internally: bool,

    /// The number of collect fee instructions. Occupies 4 bits, max 2^3 - 1 = 7
    pub ix_collect_fee_count: u8,

    /// The number of post-only order instructions. Occupies entire byte. Max 2^8 - 1 = 255
    pub ix_post_only_count: u8,

    /// The number of take-only order instructions. Occupies 4 bits, max 2^4 - 1 = 15
    pub ix_take_only_count: u8,

    /// The number of limit order instructions. Occupies 4 bits, max 2^4 - 1 = 15
    pub ix_limit_order_count: u8,
}

impl CallHeader {
    pub const HEADER_BYTE_SIZE: usize = 4;

    pub fn init(input: &[u8; 512]) -> Self {
        CallHeader {
            // Lists
            custom_token_count: (input[0] & 0b0000_1111) as usize,
            token_delta_count: (input[0] >> 4) as usize,

            // Optional variables
            recipient_provided: (input[1] & 0b0000_0001) != 0,
            track_msg_value: (input[1] & 0b0000_0010) != 0,
            track_eth_withdrawal_due: (input[1] & 0b0000_0100) != 0,

            // Settlement flags
            deposit_shortfall: (input[1] & 0b0000_1000) != 0,
            withdraw_internally: (input[1] & 0b0001_0000) != 0,

            // Instructions
            ix_collect_fee_count: input[1] >> 5,

            ix_post_only_count: input[2],

            ix_take_only_count: input[3] & 0b0000_1111,
            ix_limit_order_count: input[3] >> 4,
        }
    }

    pub fn payload_size(&self) -> usize {
        let size = Self::HEADER_BYTE_SIZE
            + self.recipient_provided as usize * core::mem::size_of::<Address>()
            + self.track_msg_value as usize * core::mem::size_of::<Atoms>()
            // Lists
            + self.custom_token_count * core::mem::size_of::<Address>()
            + self.token_delta_count * core::mem::size_of::<TokenWithdrawalDue>();

        // TODO add instruction sizes once finalized
        // PlaceMultiplePostOnly() has variable size- variable number of orders can be posted

        size
    }
}
