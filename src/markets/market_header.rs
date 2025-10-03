use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::MarketIndex,
    types::Pair,
};

#[repr(u8)]
enum MarketSource {
    Hardcoded = 0,
    Custom = 1,
}

#[repr(u8)]
enum TokenPairType {
    EthERC20 = 0,
    ERC20Eth = 1,
    ERC20ERC20 = 2,
}

pub struct MarketHeader {
    /// Whether a hardcoded or custom market
    pub market_source: MarketSource,

    /// The type of token pair
    pub token_pair_type: TokenPairType,

    /// Whether to execute base-in and quote-in take orders
    pub execute_takes: Pair<bool, bool>,

    /// Number of outer bitmap indices
    pub outer_bitmap_indices: u8,
}

impl MarketHeader {
    pub fn decode(
        payload: &ArgsBuffer,
        len: usize,
        offset: &mut usize,
    ) -> Result<Self, GoblinError> {
        let byte = payload.decode::<u8>(offset, len)?;

        let market_source_raw = byte & 0b0000_0001;
        let token_pair_type_raw = (byte & 0b0000_0110) >> 1;

        let market_source = unsafe { core::mem::transmute::<u8, MarketSource>(market_source_raw) };

        let token_pair_type = match token_pair_type_raw {
            0 => TokenPairType::EthERC20,
            1 => TokenPairType::ERC20Eth,
            2 => TokenPairType::ERC20ERC20,
            _ => return Err(GoblinError::InvalidTokenPairType),
        };

        // These fields are fixed for all the 2*3 types
        //
        // We need variable decoding for
        // - Hardcoded market- just read market index
        // - custom market- read index-pair (either 1 or 2 addresses, variable), lot size pair and tick size
        //
        // Deposit / withdraw amount fields are also variable
        // - ETH cannot be deposited
        Ok(Self {
            market_source,
            token_pair_type,

            // TODO decode
            execute_takes: Pair {
                base: false,
                quote: false,
            },
            outer_bitmap_indices: 0,
        })
    }
}

#[repr(C)]
pub struct MarketInstructions {
    pub market_index: MarketIndex,
    pub instructions_byte: u8,
}

impl MarketInstructions {
    pub fn take_bid(&self) -> bool {
        self.instructions_byte & 0b0000_0001 == 1
    }

    pub fn take_ask(&self) -> bool {
        self.instructions_byte & 0b0000_0010 == 1
    }

    pub fn outer_bitmap_indices(&self) -> u8 {
        self.instructions_byte >> 2
    }
}
