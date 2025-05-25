pub struct CallHeader {
    /// Number of custom token addresses provided, maximum 15
    pub custom_token_count: u8,

    /// Number of token deltas to update, i.e. perform deposit or withdraw
    /// operations for these many tokens
    pub token_delta_count: u8,

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
    pub fn decode(header_bytes: [u8; 4]) -> Self {
        CallHeader {
            custom_token_count: header_bytes[0] & 0b0000_1111,
            token_delta_count: header_bytes[0] >> 4,

            track_eth_delta: (header_bytes[1] & 0b0000_0001) != 0,
            deposit_shortfall: (header_bytes[1] & 0b0000_0010) != 0,
            recipient_provided: (header_bytes[1] & 0b0000_0100) != 0,
            withdraw_internally: (header_bytes[1] & 0b0000_1000) != 0,
            ix_collect_fee_count: header_bytes[1] >> 4,

            ix_post_only_count: header_bytes[2],

            ix_take_only_count: header_bytes[3] & 0b0000_1111,
            ix_limit_order_count: header_bytes[3] >> 4,
        }
    }
}
