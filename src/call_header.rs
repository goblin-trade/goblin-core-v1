/// input[0] is the header byte
///
/// * Pos 0 bit tells whether to deposit ETH
/// * Pos 1 tells whether a recipient is provided, otherwise the recipient is msg.sender
/// * Pos 2 tells whether to transfer to recipient internally
///   - Value is ignored if recipient is not provided
///   - If true, then `withdrawal_due` is credited internally to recipient's TraderTokenState
///   - If false, the amount is withdrawn to the recipient
///
/// * Remaining MSB 5 bits give the number of calls. The max value
/// is 2^5 - 1 = 31
pub struct CallHeader {
    /// Whether to deposit ETH
    pub deposit_eth: bool,

    /// Whether to read recipient address from payload. If false, use msg.sender
    pub recipient_provided: bool,

    /// Whether to update TraderTokenState for recipient, or to actually transfer out tokens
    pub transfer_to_recipient_internally: bool,

    /// The number of instructions
    pub num_calls: u8,

    /// Number of custom token addresses provided, maximum 15
    pub custom_token_count: u8,

    /// Number of token deltas to update, i.e. perform deposit or withdraw
    /// operations for these many tokens
    pub token_delta_count: u8,
}

impl CallHeader {
    pub fn decode(header_bytes: [u8; 2]) -> Self {
        CallHeader {
            deposit_eth: (header_bytes[0] & 0b0000_0001) != 0,
            recipient_provided: (header_bytes[0] & 0b0000_0010) != 0,
            transfer_to_recipient_internally: (header_bytes[0] & 0b0000_0100) != 0,
            num_calls: header_bytes[0] >> 3,
            custom_token_count: header_bytes[1] & 0b0000_1111,
            token_delta_count: header_bytes[1] >> 4,
        }
    }
}
