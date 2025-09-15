use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::{IndexedMarketV2, MarketInstructions},
    quantities::Atoms,
    require,
    settlement::{ERC20Deposit, ERC20Input, ERC20Withdraw},
    types::Address,
};

pub struct Header {
    /// Number of custom erc20 token addresses provided, maximum 2^4 - 1 = 15
    pub custom_erc20_count: usize,

    /// Number of ERC20 token deltas to deposit. Maximum 2^4 - 1 = 15
    pub erc20_deposit_count: usize,

    /// Number of ERC20 token deltas to withdraw. Maximum 2^4 - 1 = 15
    pub erc20_withdraw_count: usize,

    /// The number of markets to process. Max 2^4 - 1 = 15
    pub market_instructions_count: usize,

    /// Number of custom market addresses provided, maximum 2^3 - 1 = 7
    pub custom_market_count: usize,

    /// Whether to read recipient address from payload. If false, use msg.sender as recipient.
    pub recipient_provided: bool,

    /// Whether to read msg.value from hostio
    pub track_msg_value: bool,

    /// Whether to read eth_withdrawal_due from payload
    pub track_eth_withdrawal_due: bool,

    /// Whether to credit tokens to ERC20Store or EthStore, or to actually transfer out tokens
    pub withdraw_internally: bool,
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

        Header {
            // Lists
            custom_erc20_count: (byte_0 & 0b0000_1111) as usize,
            erc20_deposit_count: (byte_0 >> 4) as usize,

            erc20_withdraw_count: (byte_1 & 0b0000_1111) as usize,
            market_instructions_count: (byte_1 >> 4) as usize,

            custom_market_count: (byte_2 & 0b0000_0111) as usize,

            // Optional variables
            recipient_provided: (byte_2 & 0b0000_1000) != 0,
            track_msg_value: (byte_2 & 0b0001_0000) != 0,
            track_eth_withdrawal_due: (byte_2 & 0b0010_0000) != 0,

            // Settlement flags
            withdraw_internally: (byte_2 & 0b0100_0000) != 0,
        }
    }

    pub fn payload_size(&self) -> usize {
        let size = Self::HEADER_BYTE_SIZE
            + self.recipient_provided as usize * core::mem::size_of::<Address>()
            + self.track_msg_value as usize * core::mem::size_of::<Atoms>()
            // Lists
            + self.custom_erc20_count * core::mem::size_of::<Address>()
            + self.erc20_deposit_count * core::mem::size_of::<ERC20Input<ERC20Deposit>>()
            + self.erc20_withdraw_count * core::mem::size_of::<ERC20Input<ERC20Withdraw>>()
            + self.custom_market_count * core::mem::size_of::<IndexedMarketV2>()
            * self.market_instructions_count * core::mem::size_of::<MarketInstructions>();

        // TODO add instruction sizes once finalized
        // PlaceMultiplePostOnly() and CancelMultipleOrders() have variable size-
        // variable number of orders can be posted or canceled

        size
    }
}
