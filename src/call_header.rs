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
    pub deposit_native_token: bool,
    pub recipient_provided: bool,
    pub transfer_to_recipient_internally: bool,
    pub num_calls: u8,
}

impl CallHeader {
    pub fn decode(header_byte: u8) -> Self {
        CallHeader {
            deposit_native_token: (header_byte & 0b0000_0001) != 0,
            recipient_provided: (header_byte & 0b0000_0010) != 0,
            transfer_to_recipient_internally: (header_byte & 0b0000_0100) != 0,
            num_calls: header_byte >> 3,
        }
    }
}
