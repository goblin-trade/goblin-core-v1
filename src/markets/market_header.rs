use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::MarketIndex,
    types::Pair,
};

// #[repr(u8)]
// pub enum MarketSource {
//     Hardcoded = 0,
//     Custom = 1,
// }

// #[repr(u8)]
// pub enum TokenPairType {
//     ETHBaseERC20Quote = 0,
//     ERC20BaseETHQuote = 1,
//     ERC20BaseERC20Quote = 2,
// }

// enum TransferAction {
//     None,
//     Deposit,
//     Withdraw,
// }

// impl TryFrom<u8> for TransferAction {
//     type Error = GoblinError;

//     fn try_from(value: u8) -> Result<Self, Self::Error> {
//         match value {
//             0 => Ok(TransferAction::None),
//             1 => Ok(TransferAction::Deposit),
//             2 => Ok(TransferAction::Withdraw),
//             _ => Err(GoblinError::InvalidTransferAction),
//         }
//     }
// }

pub struct MarketHeader {
    /// Whether a hardcoded or custom market
    pub market_source_raw: u8,

    /// The type of token pair
    pub pair_type_raw: u8,

    // /// Whether to deposit or withdraw one of the tokens
    // pub transfers: Pair<TransferAction, TransferAction>,
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

        let market_source_raw = byte_0 & 0b0000_0001;
        let pair_type_raw = (byte_0 & 0b0000_0110) >> 1;

        let execute_takes = Pair {
            base: (byte_0 & 0b0000_1000) != 0,
            quote: (byte_0 & 0b0001_0000) != 0,
        };

        // 3 bits- max value 7
        // This field is currently unused. Increase the amount if needed by reading a new byte.
        let outer_bitmap_indices = (byte_0 & 0b1110_0000) >> 5;

        // let market_source = unsafe { core::mem::transmute::<u8, MarketSource>(market_source_byte) };

        // let token_pair_type = match token_pair_type_raw {
        //     0 => TokenPairType::ETHBaseERC20Quote,
        //     1 => TokenPairType::ERC20BaseETHQuote,
        //     2 => TokenPairType::ERC20BaseERC20Quote,
        //     _ => return Err(GoblinError::InvalidTokenPairType),
        // };

        // Move to dynamic decoding
        // We cannot perform deposit for ETH
        // let base_transfer_raw = (byte_0 & 0b0001_1000) >> 3;
        // let quote_transfer_raw = (byte_0 & 0b0110_0000) >> 3;

        // 2 bits needed for execute_takes, but we have just 1

        // let transfers = Pair {
        //     base: TransferAction::try_from(base_transfer_raw)?,
        //     quote: TransferAction::try_from(quote_transfer_raw)?,
        // };

        Ok(Self {
            market_source_raw,
            pair_type_raw,
            execute_takes,
            outer_bitmap_indices,
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
