use crate::{
    axis_helpers::TokenPair,
    market::{LotSizePair, TokenAddressPair},
    quantities::QuoteLotsPerBaseUnitPerTick,
    state::{ConstPreimage, MarketState, Preimage},
};

/// Key preimage to read MarketState from slot
///
/// This is similar to MarketState, but instead of token index pair we have
/// a pair of token addresses
///
/// # `packed` representation
///
/// The byte layout of this type is hashed by [crate::state::ConstPreimage],
/// which reads the raw bytes of `self`. Without `packed`, the trailing
/// alignment padding would be uninitialized and reading it during const
/// evaluation is a hard error. `packed` keeps the struct padding-free.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct MarketPreimage<TP: TokenPair> {
    pub lot_size_pair: LotSizePair,
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
    pub token_address_pair: TokenAddressPair<TP>,
}

impl<TP: TokenPair> Preimage for MarketPreimage<TP> {
    const SLOT_DISCRIMINATOR: u8 = TP::DISCRIMINATOR;

    type SlotState = MarketState;
}

const impl<TP: TokenPair> ConstPreimage for MarketPreimage<TP> {}
