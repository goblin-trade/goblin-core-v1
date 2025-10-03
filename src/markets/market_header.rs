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
    ETHBaseERC20Quote = 0,
    ERC20BaseETHQuote = 1,
    ERC20BaseERC20Quote = 2,
}

enum TransferAction {
    None,
    Deposit,
    Withdraw,
}

impl TryFrom<u8> for TransferAction {
    type Error = GoblinError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(TransferAction::None),
            1 => Ok(TransferAction::Deposit),
            2 => Ok(TransferAction::Withdraw),
            _ => Err(GoblinError::InvalidTransferAction),
        }
    }
}

pub struct MarketHeader {
    /// Whether a hardcoded or custom market
    pub market_source: MarketSource,

    /// The type of token pair
    pub token_pair_type: TokenPairType,

    /// Whether to deposit or withdraw one of the tokens
    pub transfers: Pair<TransferAction, TransferAction>,

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
        let byte_0 = payload.decode::<u8>(offset, len)?;
        let byte_1 = payload.decode::<u8>(offset, len)?;

        let market_source_raw = byte_0 & 0b0000_0001;
        let token_pair_type_raw = (byte_0 & 0b0000_0110) >> 1;

        let market_source = unsafe { core::mem::transmute::<u8, MarketSource>(market_source_raw) };

        let token_pair_type = match token_pair_type_raw {
            0 => TokenPairType::ETHBaseERC20Quote,
            1 => TokenPairType::ERC20BaseETHQuote,
            2 => TokenPairType::ERC20BaseERC20Quote,
            _ => return Err(GoblinError::InvalidTokenPairType),
        };

        let base_transfer_raw = (byte_0 & 0b0001_1000) >> 3;
        let quote_transfer_raw = (byte_0 & 0b0110_0000) >> 3;

        // 2 bits needed for execute_takes, but we have just 1

        let transfers = Pair {
            base: TransferAction::try_from(base_transfer_raw)?,
            quote: TransferAction::try_from(quote_transfer_raw)?,
        };

        // Dynamic fields read based on the header
        match market_source {
            MarketSource::Hardcoded => {
                let hardcoded_index_raw = payload.decode::<u8>(offset, len)?;

                // As there are 3 token pair types, there are 3 hardcoded token indices corresponding to each.

                // Now use token_pair_type to read from the appropriate hardcoded list
                // Next, read deposit / withdraw amounts. There is no deposit field for ETH.
                // Deposit and withdraw amounts are u64, i.e. 8 bytes. We use a header to decide
                // whether to read these or not. Passing 0u64 is wasteful.
            }
            MarketSource::Custom => todo!(),
        }

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
            transfers,

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
