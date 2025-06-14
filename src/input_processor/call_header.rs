use crate::{quantities::Atoms, settlement::ERC20_WITHDRAWAL_ITEM_SIZE, types::Address};

pub struct CallHeader {
    /// Number of custom token addresses provided, maximum 2^4 - 1 = 15
    pub custom_token_count: usize,

    /// Number of token deltas to update, i.e. perform deposit or withdraw
    /// operations for these many tokens. Maximum 2^4 - 1 = 15
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

    // We have 3 free bits on input[1] because ix_collect_fee_count is
    // removed
    /// The number of post-only order instructions. Occupies 4 bits, max 2^4 - 1 = 15
    pub ix_post_only_count: u8,

    /// The number of cancel order instructions. Occupies 4 bits, max 2^4 - 1 = 15
    pub ix_cancel_count: u8,

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

            // We have 3 free bits on input[1] because ix_collect_fee_count is
            // removed
            ix_post_only_count: input[2] & 0b0000_1111,
            ix_cancel_count: input[2] >> 4,

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
            + self.token_delta_count * ERC20_WITHDRAWAL_ITEM_SIZE;

        // TODO add instruction sizes once finalized
        // PlaceMultiplePostOnly() and CancelMultipleOrders() have variable size-
        // variable number of orders can be posted or canceled

        size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_with_zero_input() {
        let input = [0u8; 512];
        let header = CallHeader::init(&input);

        assert_eq!(header.custom_token_count, 0);
        assert_eq!(header.token_delta_count, 0);
        assert!(!header.recipient_provided);
        assert!(!header.track_msg_value);
        assert!(!header.track_eth_withdrawal_due);
        assert!(!header.deposit_shortfall);
        assert!(!header.withdraw_internally);
        assert_eq!(header.ix_post_only_count, 0);
        assert_eq!(header.ix_cancel_count, 0);
        assert_eq!(header.ix_take_only_count, 0);
        assert_eq!(header.ix_limit_order_count, 0);
    }

    #[test]
    fn test_init_byte_0_parsing() {
        let mut input = [0u8; 512];
        // Set byte 0: custom_token_count = 5 (lower 4 bits), token_delta_count = 10 (upper 4 bits)
        input[0] = 0b1010_0101; // 10 << 4 | 5 = 165

        let header = CallHeader::init(&input);
        assert_eq!(header.custom_token_count, 5);
        assert_eq!(header.token_delta_count, 10);
    }

    #[test]
    fn test_init_byte_0_max_values() {
        let mut input = [0u8; 512];
        // Set byte 0: both fields to maximum (15)
        input[0] = 0b1111_1111; // 255

        let header = CallHeader::init(&input);
        assert_eq!(header.custom_token_count, 15);
        assert_eq!(header.token_delta_count, 15);
    }

    #[test]
    fn test_init_byte_1_boolean_flags() {
        let mut input = [0u8; 512];
        // Set all boolean flags in byte 1 (bits 0-4)
        input[1] = 0b0001_1111;

        let header = CallHeader::init(&input);
        assert!(header.recipient_provided);
        assert!(header.track_msg_value);
        assert!(header.track_eth_withdrawal_due);
        assert!(header.deposit_shortfall);
        assert!(header.withdraw_internally);
    }

    #[test]
    fn test_init_byte_1_upper_bits_unused() {
        let mut input = [0u8; 512];
        // Set upper 3 bits of byte 1 (these are now unused/reserved)
        input[1] = 0b1110_0000; // 7 << 5 = 224

        let header = CallHeader::init(&input);
        assert!(!header.recipient_provided);
        assert!(!header.track_msg_value);
        assert!(!header.track_eth_withdrawal_due);
        assert!(!header.deposit_shortfall);
        assert!(!header.withdraw_internally);
    }

    #[test]
    fn test_init_byte_2_parsing() {
        let mut input = [0u8; 512];
        // Set byte 2: ix_post_only_count = 3 (lower 4 bits), ix_cancel_count = 12 (upper 4 bits)
        input[2] = 0b1100_0011; // 12 << 4 | 3 = 195

        let header = CallHeader::init(&input);
        assert_eq!(header.ix_post_only_count, 3);
        assert_eq!(header.ix_cancel_count, 12);
    }

    #[test]
    fn test_init_byte_3_parsing() {
        let mut input = [0u8; 512];
        // Set byte 3: ix_take_only_count = 8 (lower 4 bits), ix_limit_order_count = 6 (upper 4 bits)
        input[3] = 0b0110_1000; // 6 << 4 | 8 = 104

        let header = CallHeader::init(&input);
        assert_eq!(header.ix_take_only_count, 8);
        assert_eq!(header.ix_limit_order_count, 6);
    }

    #[test]
    fn test_init_all_fields_set() {
        let mut input = [0u8; 512];
        input[0] = 0b1010_0101; // token_delta_count=10, custom_token_count=5
        input[1] = 0b0001_1111; // all flags set (upper 3 bits unused)
        input[2] = 0b1100_0011; // ix_cancel_count=12, ix_post_only_count=3
        input[3] = 0b0110_1000; // ix_limit_order_count=6, ix_take_only_count=8

        let header = CallHeader::init(&input);

        // Verify all fields
        assert_eq!(header.custom_token_count, 5);
        assert_eq!(header.token_delta_count, 10);
        assert!(header.recipient_provided);
        assert!(header.track_msg_value);
        assert!(header.track_eth_withdrawal_due);
        assert!(header.deposit_shortfall);
        assert!(header.withdraw_internally);
        assert_eq!(header.ix_post_only_count, 3);
        assert_eq!(header.ix_cancel_count, 12);
        assert_eq!(header.ix_take_only_count, 8);
        assert_eq!(header.ix_limit_order_count, 6);
    }

    #[test]
    fn test_payload_size_minimal() {
        let input = [0u8; 512];
        let header = CallHeader::init(&input);

        // Should only include the header size
        assert_eq!(header.payload_size(), CallHeader::HEADER_BYTE_SIZE);
    }

    #[test]
    fn test_payload_size_with_recipient() {
        let mut input = [0u8; 512];
        input[1] = 0b0000_0001; // recipient_provided = true

        let header = CallHeader::init(&input);
        let expected_size = CallHeader::HEADER_BYTE_SIZE + core::mem::size_of::<Address>();

        assert_eq!(header.payload_size(), expected_size);
    }

    #[test]
    fn test_payload_size_with_msg_value() {
        let mut input = [0u8; 512];
        input[1] = 0b0000_0010; // track_msg_value = true

        let header = CallHeader::init(&input);
        let expected_size = CallHeader::HEADER_BYTE_SIZE + core::mem::size_of::<Atoms>();

        assert_eq!(header.payload_size(), expected_size);
    }

    #[test]
    fn test_payload_size_with_custom_tokens() {
        let mut input = [0u8; 512];
        input[0] = 0b0000_0011; // custom_token_count = 3

        let header = CallHeader::init(&input);
        let expected_size = CallHeader::HEADER_BYTE_SIZE + 3 * core::mem::size_of::<Address>();

        assert_eq!(header.payload_size(), expected_size);
    }

    #[test]
    fn test_payload_size_with_token_deltas() {
        let mut input = [0u8; 512];
        input[0] = 0b0010_0000; // token_delta_count = 2

        let header = CallHeader::init(&input);
        let expected_size = CallHeader::HEADER_BYTE_SIZE + 2 * ERC20_WITHDRAWAL_ITEM_SIZE;

        assert_eq!(header.payload_size(), expected_size);
    }

    #[test]
    fn test_payload_size_comprehensive() {
        let mut input = [0u8; 512];
        input[0] = 0b0011_0101; // token_delta_count=3, custom_token_count=5
        input[1] = 0b0000_0011; // recipient_provided=true, track_msg_value=true

        let header = CallHeader::init(&input);
        let expected_size = CallHeader::HEADER_BYTE_SIZE
            + core::mem::size_of::<Address>() // recipient
            + core::mem::size_of::<Atoms>() // msg_value
            + 5 * core::mem::size_of::<Address>() // custom tokens
            + 3 * ERC20_WITHDRAWAL_ITEM_SIZE; // token deltas

        assert_eq!(header.payload_size(), expected_size);
    }

    #[test]
    fn test_header_byte_size_constant() {
        assert_eq!(CallHeader::HEADER_BYTE_SIZE, 4);
    }

    #[test]
    fn test_max_values_edge_cases() {
        let mut input = [0u8; 512];
        // Test maximum values for each field
        input[0] = 0xFF; // Both counts at max (15)
        input[1] = 0xFF; // All flags true (upper 3 bits unused)
        input[2] = 0xFF; // Both counts at max (15)
        input[3] = 0xFF; // Both counts at max (15)

        let header = CallHeader::init(&input);

        assert_eq!(header.custom_token_count, 15);
        assert_eq!(header.token_delta_count, 15);
        assert!(header.recipient_provided);
        assert!(header.track_msg_value);
        assert!(header.track_eth_withdrawal_due);
        assert!(header.deposit_shortfall);
        assert!(header.withdraw_internally);
        assert_eq!(header.ix_post_only_count, 15);
        assert_eq!(header.ix_cancel_count, 15);
        assert_eq!(header.ix_take_only_count, 15);
        assert_eq!(header.ix_limit_order_count, 15);
    }

    #[test]
    fn test_bit_manipulation_isolation() {
        let mut input = [0u8; 512];

        // Test that setting one field doesn't affect others
        input[0] = 0b0000_0001; // Only custom_token_count = 1
        let header = CallHeader::init(&input);
        assert_eq!(header.custom_token_count, 1);
        assert_eq!(header.token_delta_count, 0);

        input[0] = 0b0001_0000; // Only token_delta_count = 1
        let header = CallHeader::init(&input);
        assert_eq!(header.custom_token_count, 0);
        assert_eq!(header.token_delta_count, 1);

        // Test individual boolean flags
        for i in 0..5 {
            input[1] = 1 << i;
            let header = CallHeader::init(&input);

            assert_eq!(header.recipient_provided, i == 0);
            assert_eq!(header.track_msg_value, i == 1);
            assert_eq!(header.track_eth_withdrawal_due, i == 2);
            assert_eq!(header.deposit_shortfall, i == 3);
            assert_eq!(header.withdraw_internally, i == 4);
        }
    }
}
