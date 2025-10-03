use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::MarketInstructions,
    quantities::UnsidedAtoms,
    require,
    types::Address,
};

pub struct Header {
    /// Number of custom erc20 token addresses provided, maximum 2^4 - 1 = 15
    pub custom_erc20_count: usize,

    /// The number of markets to process. Max 2^4 - 1 = 15
    pub market_count: usize,

    /// Whether to read recipient address from payload. If false, use msg.sender as recipient.
    pub recipient_provided: bool,

    /// Whether to read msg.value from hostio
    pub track_msg_value: bool,

    /// Whether to credit tokens to ERC20Store or EthStore, or to actually transfer out tokens
    pub withdraw_internally: bool,
}

impl Header {
    pub const HEADER_BYTE_SIZE: usize = 2;

    pub fn init(input: &ArgsBuffer, len: usize) -> Result<Self, GoblinError> {
        require!(len >= Header::HEADER_BYTE_SIZE, GoblinError::InvalidPayload);
        let header = Header::init_unchecked(input);
        require!(len >= header.payload_size(), GoblinError::InvalidPayload);

        Ok(header)
    }

    fn init_unchecked(input: &ArgsBuffer) -> Self {
        let byte_0 = input.decode_unchecked::<u8>(0);
        let byte_1 = input.decode_unchecked::<u8>(1);
        Header {
            // Lists
            custom_erc20_count: (byte_0 & 0b0000_1111) as usize,
            market_count: (byte_0 >> 4) as usize,

            // Optional variables
            recipient_provided: (byte_1 & 0b0000_0001) != 0,
            track_msg_value: (byte_1 & 0b0000_0010) != 0,

            // Settlement flags
            withdraw_internally: (byte_1 & 0b0000_0100) != 0,
        }
    }

    pub fn payload_size(&self) -> usize {
        let size = Self::HEADER_BYTE_SIZE
            + self.recipient_provided as usize * core::mem::size_of::<Address>()
            + self.track_msg_value as usize * core::mem::size_of::<UnsidedAtoms>()
            // Lists
            + self.custom_erc20_count * core::mem::size_of::<Address>()
            * self.market_count * core::mem::size_of::<MarketInstructions>();

        size
    }
}
