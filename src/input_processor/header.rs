use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::{IndexedMarketV2, MarketInstructions},
    quantities::Atoms,
    require,
    settlement::ERC20DeltaInput,
    types::Address,
};

pub struct Header {
    /// Number of custom erc20 token addresses provided, maximum 2^4 - 1 = 15
    pub custom_erc20_count: usize,

    /// Number of ERC20 token deltas to update, i.e. perform deposit or withdraw
    /// operations for these many tokens. Maximum 2^4 - 1 = 15
    pub erc20_delta_count: usize,

    /// Number of custom market addresses provided, maximum 2^3 - 1 = 7
    pub custom_market_count: usize,

    /// Whether to read recipient address from payload. If false, use msg.sender as recipient.
    pub recipient_provided: bool,

    /// Whether to read msg.value from hostio
    pub track_msg_value: bool,

    /// Whether to read eth_withdrawal_due from payload
    pub track_eth_withdrawal_due: bool,

    /// Whether to deposit shortfall amount during settlement
    pub deposit_shortfall: bool,

    /// Whether to credit tokens to ERC20Store or EthStore, or to actually transfer out tokens
    pub withdraw_internally: bool,

    /// The number of markets to process
    pub market_instructions_count: usize,
}

impl Header {
    pub const HEADER_BYTE_SIZE: usize = 3;

    pub fn init(input: &[u8; 512], len: usize) -> Result<Self, GoblinError> {
        require!(len >= Header::HEADER_BYTE_SIZE, GoblinError::InvalidPayload);
        let header = Header::init_unchecked(input);
        require!(len >= header.payload_size(), GoblinError::InvalidPayload);

        Ok(header)
    }

    fn init_unchecked(input: &ArgsBuffer) -> Self {
        let byte_0 = input.decode_unchecked::<u8>(0);
        let byte_1 = input.decode_unchecked::<u8>(1);
        let byte_2 = input.decode_unchecked::<u8>(2);
        // let byte_3 = input.decode_unchecked::<u8>(3);

        Header {
            // Lists
            custom_erc20_count: (byte_0 & 0b0000_1111) as usize,
            erc20_delta_count: (byte_0 >> 4) as usize,
            custom_market_count: (byte_1 & 0b0000_0111) as usize,

            // Optional variables
            recipient_provided: (byte_1 & 0b0000_1000) != 0,
            track_msg_value: (byte_1 & 0b0001_0000) != 0,
            track_eth_withdrawal_due: (byte_1 & 0b0010_0000) != 0,

            // Settlement flags
            deposit_shortfall: (byte_1 & 0b0100_0000) != 0,
            withdraw_internally: (byte_1 & 0b1000_0000) != 0,

            market_instructions_count: byte_2 as usize,
        }
    }

    pub fn payload_size(&self) -> usize {
        let size = Self::HEADER_BYTE_SIZE
            + self.recipient_provided as usize * core::mem::size_of::<Address>()
            + self.track_msg_value as usize * core::mem::size_of::<Atoms>()
            // Lists
            + self.custom_erc20_count * core::mem::size_of::<Address>()
            + self.erc20_delta_count * core::mem::size_of::<ERC20DeltaInput>()
            + self.custom_market_count * core::mem::size_of::<IndexedMarketV2>()
            * self.market_instructions_count * core::mem::size_of::<MarketInstructions>();

        // TODO add instruction sizes once finalized
        // PlaceMultiplePostOnly() and CancelMultipleOrders() have variable size-
        // variable number of orders can be posted or canceled

        size
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_init_with_zero_input() {
//         let input = [0u8; 512];
//         let header = Header::init_unchecked(&input);

//         assert_eq!(header.custom_erc20_count, 0);
//         assert_eq!(header.erc20_delta_count, 0);
//         assert_eq!(header.custom_market_count, 0);
//         assert!(!header.recipient_provided);
//         assert!(!header.track_msg_value);
//         assert!(!header.track_eth_withdrawal_due);
//         assert!(!header.deposit_shortfall);
//         assert!(!header.withdraw_internally);

//         assert_eq!(header.ix_post_only_count, 0);
//         assert_eq!(header.ix_reduce_count, 0);
//         assert_eq!(header.ix_take_only_count, 0);
//         assert_eq!(header.ix_limit_order_count, 0);
//     }

//     #[test]
//     fn test_init_byte_0_parsing() {
//         let mut input = [0u8; 512];
//         // Set byte 0: custom_token_count = 5 (lower 4 bits), erc20_delta_count = 10 (upper 4 bits)
//         input[0] = 0b1010_0101; // 10 << 4 | 5 = 165

//         let header = Header::init_unchecked(&input);
//         assert_eq!(header.custom_erc20_count, 5);
//         assert_eq!(header.erc20_delta_count, 10);
//     }

//     #[test]
//     fn test_init_byte_0_max_values() {
//         let mut input = [0u8; 512];
//         // Set byte 0: both fields to maximum (15)
//         input[0] = 0b1111_1111; // 255

//         let header = Header::init_unchecked(&input);
//         assert_eq!(header.custom_erc20_count, 15);
//         assert_eq!(header.erc20_delta_count, 15);
//     }

//     #[test]
//     fn test_init_byte_1_custom_market_count() {
//         let mut input = [0u8; 512];
//         // Set custom_market_count = 5 (bits 0-2)
//         input[1] = 0b0000_0101;

//         let header = Header::init_unchecked(&input);
//         assert_eq!(header.custom_market_count, 5);
//         assert!(!header.recipient_provided);
//         assert!(!header.track_msg_value);
//         assert!(!header.track_eth_withdrawal_due);
//         assert!(!header.deposit_shortfall);
//         assert!(!header.withdraw_internally);
//     }

//     #[test]
//     fn test_init_byte_1_max_custom_market_count() {
//         let mut input = [0u8; 512];
//         // Set custom_market_count = 7 (maximum value for 3 bits)
//         input[1] = 0b0000_0111;

//         let header = Header::init_unchecked(&input);
//         assert_eq!(header.custom_market_count, 7);
//     }

//     #[test]
//     fn test_init_byte_1_boolean_flags() {
//         let mut input = [0u8; 512];
//         // Set all boolean flags in byte 1 (bits 3-7)
//         input[1] = 0b1111_1000;

//         let header = Header::init_unchecked(&input);
//         assert_eq!(header.custom_market_count, 0); // bits 0-2 are 0
//         assert!(header.recipient_provided);
//         assert!(header.track_msg_value);
//         assert!(header.track_eth_withdrawal_due);
//         assert!(header.deposit_shortfall);
//         assert!(header.withdraw_internally);
//     }

//     #[test]
//     fn test_init_byte_1_combined_market_count_and_flags() {
//         let mut input = [0u8; 512];
//         // Set custom_market_count = 3 (bits 0-2) and some flags
//         input[1] = 0b0001_1011; // withdraw_internally=0, deposit_shortfall=0, track_eth_withdrawal_due=0, track_msg_value=1, recipient_provided=1, custom_market_count=3

//         let header = Header::init_unchecked(&input);
//         assert_eq!(header.custom_market_count, 3);
//         assert!(header.recipient_provided);
//         assert!(header.track_msg_value);
//         assert!(!header.track_eth_withdrawal_due);
//         assert!(!header.deposit_shortfall);
//         assert!(!header.withdraw_internally);
//     }

//     #[test]
//     fn test_init_byte_2_parsing() {
//         let mut input = [0u8; 512];
//         // Set byte 2: ix_post_only_count = 3 (lower 4 bits), ix_cancel_count = 12 (upper 4 bits)
//         input[2] = 0b1100_0011; // 12 << 4 | 3 = 195

//         let header = Header::init_unchecked(&input);
//         assert_eq!(header.ix_post_only_count, 3);
//         assert_eq!(header.ix_reduce_count, 12);
//     }

//     #[test]
//     fn test_init_byte_3_parsing() {
//         let mut input = [0u8; 512];
//         // Set byte 3: ix_take_only_count = 8 (lower 4 bits), ix_limit_order_count = 6 (upper 4 bits)
//         input[3] = 0b0110_1000; // 6 << 4 | 8 = 104

//         let header = Header::init_unchecked(&input);
//         assert_eq!(header.ix_take_only_count, 8);
//         assert_eq!(header.ix_limit_order_count, 6);
//     }

//     #[test]
//     fn test_init_all_fields_set() {
//         let mut input = [0u8; 512];
//         input[0] = 0b1010_0101; // erc20_delta_count=10, custom_token_count=5
//         input[1] = 0b1111_1111; // all flags set, custom_market_count=7
//         input[2] = 0b1100_0011; // ix_cancel_count=12, ix_post_only_count=3
//         input[3] = 0b0110_1000; // ix_limit_order_count=6, ix_take_only_count=8

//         let header = Header::init_unchecked(&input);

//         // Verify all fields
//         assert_eq!(header.custom_erc20_count, 5);
//         assert_eq!(header.erc20_delta_count, 10);
//         assert_eq!(header.custom_market_count, 7);
//         assert!(header.recipient_provided);
//         assert!(header.track_msg_value);
//         assert!(header.track_eth_withdrawal_due);
//         assert!(header.deposit_shortfall);
//         assert!(header.withdraw_internally);
//         assert_eq!(header.ix_post_only_count, 3);
//         assert_eq!(header.ix_reduce_count, 12);
//         assert_eq!(header.ix_take_only_count, 8);
//         assert_eq!(header.ix_limit_order_count, 6);
//     }

//     #[test]
//     fn test_payload_size_minimal() {
//         let input = [0u8; 512];
//         let header = Header::init_unchecked(&input);

//         // Should only include the header size
//         assert_eq!(header.payload_size(), Header::HEADER_BYTE_SIZE);
//     }

//     #[test]
//     fn test_payload_size_with_recipient() {
//         let mut input = [0u8; 512];
//         input[1] = 0b0000_1000; // recipient_provided = true (bit 3)

//         let header = Header::init_unchecked(&input);
//         let expected_size = Header::HEADER_BYTE_SIZE + core::mem::size_of::<Address>();

//         assert_eq!(header.payload_size(), expected_size);
//     }

//     #[test]
//     fn test_payload_size_with_msg_value() {
//         let mut input = [0u8; 512];
//         input[1] = 0b0001_0000; // track_msg_value = true (bit 4)

//         let header = Header::init_unchecked(&input);
//         let expected_size = Header::HEADER_BYTE_SIZE + core::mem::size_of::<Atoms>();

//         assert_eq!(header.payload_size(), expected_size);
//     }

//     #[test]
//     fn test_payload_size_with_custom_tokens() {
//         let mut input = [0u8; 512];
//         input[0] = 0b0000_0011; // custom_token_count = 3

//         let header = Header::init_unchecked(&input);
//         let expected_size = Header::HEADER_BYTE_SIZE + 3 * core::mem::size_of::<Address>();

//         assert_eq!(header.payload_size(), expected_size);
//     }

//     #[test]
//     fn test_payload_size_with_erc20_deltas() {
//         let mut input = [0u8; 512];
//         input[0] = 0b0010_0000; // erc20_delta_count = 2

//         let header = Header::init_unchecked(&input);
//         let expected_size = Header::HEADER_BYTE_SIZE + 2 * core::mem::size_of::<ERC20DeltaInput>();

//         assert_eq!(header.payload_size(), expected_size);
//     }

//     #[test]
//     fn test_payload_size_with_custom_markets() {
//         let mut input = [0u8; 512];
//         input[1] = 0b0000_0100; // custom_market_count = 4

//         let header = Header::init_unchecked(&input);
//         let expected_size = Header::HEADER_BYTE_SIZE + 4 * core::mem::size_of::<IndexedMarket>();

//         assert_eq!(header.payload_size(), expected_size);
//     }

//     #[test]
//     fn test_payload_size_comprehensive() {
//         let mut input = [0u8; 512];
//         input[0] = 0b0011_0101; // erc20_delta_count=3, custom_token_count=5
//         input[1] = 0b0001_1010; // custom_market_count=2, recipient_provided=true, track_msg_value=true

//         let header = Header::init_unchecked(&input);
//         let expected_size = Header::HEADER_BYTE_SIZE
//             + core::mem::size_of::<Address>() // recipient
//             + core::mem::size_of::<Atoms>() // msg_value
//             + 5 * core::mem::size_of::<Address>() // custom tokens
//             + 3 * core::mem::size_of::<ERC20DeltaInput>() // token deltas
//             + 2 * core::mem::size_of::<IndexedMarket>(); // custom markets

//         assert_eq!(header.payload_size(), expected_size);
//     }

//     #[test]
//     fn test_header_byte_size_constant() {
//         assert_eq!(Header::HEADER_BYTE_SIZE, 4);
//     }

//     #[test]
//     fn test_max_values_edge_cases() {
//         let mut input = [0u8; 512];
//         // Test maximum values for each field
//         input[0] = 0xFF; // Both counts at max (15)
//         input[1] = 0xFF; // All flags true, custom_market_count at max (7)
//         input[2] = 0xFF; // Both counts at max (15)
//         input[3] = 0xFF; // Both counts at max (15)

//         let header = Header::init_unchecked(&input);

//         assert_eq!(header.custom_erc20_count, 15);
//         assert_eq!(header.erc20_delta_count, 15);
//         assert_eq!(header.custom_market_count, 7);
//         assert!(header.recipient_provided);
//         assert!(header.track_msg_value);
//         assert!(header.track_eth_withdrawal_due);
//         assert!(header.deposit_shortfall);
//         assert!(header.withdraw_internally);
//         assert_eq!(header.ix_post_only_count, 15);
//         assert_eq!(header.ix_reduce_count, 15);
//         assert_eq!(header.ix_take_only_count, 15);
//         assert_eq!(header.ix_limit_order_count, 15);
//     }

//     #[test]
//     fn test_bit_manipulation_isolation() {
//         let mut input = [0u8; 512];

//         // Test that setting one field doesn't affect others
//         input[0] = 0b0000_0001; // Only custom_token_count = 1
//         let header = Header::init_unchecked(&input);
//         assert_eq!(header.custom_erc20_count, 1);
//         assert_eq!(header.erc20_delta_count, 0);

//         input[0] = 0b0001_0000; // Only erc20_delta_count = 1
//         let header = Header::init_unchecked(&input);
//         assert_eq!(header.custom_erc20_count, 0);
//         assert_eq!(header.erc20_delta_count, 1);

//         // Test individual boolean flags (now starting from bit 3)
//         for i in 3..8 {
//             input[1] = 1 << i;
//             let header = Header::init_unchecked(&input);

//             assert_eq!(header.custom_market_count, 0); // bits 0-2 are 0
//             assert_eq!(header.recipient_provided, i == 3);
//             assert_eq!(header.track_msg_value, i == 4);
//             assert_eq!(header.track_eth_withdrawal_due, i == 5);
//             assert_eq!(header.deposit_shortfall, i == 6);
//             assert_eq!(header.withdraw_internally, i == 7);
//         }
//     }
// }
