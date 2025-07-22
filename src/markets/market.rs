use crate::{
    goblin_error::GoblinError,
    markets::HARDCODED_MARKETS,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    require,
    tokens::Token,
    types::Address,
};

// Max number of custom markets
pub const MAX_CUSTOM_MARKETS: usize = 7;

/// Input payload receives an array of MarketItems. They hold token indices instead of token addresses.
/// Token addresses are mapped to token indices to obtain the Market struct.
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct IndexedMarket {
    pub base_token_index: u8,
    pub quote_token_index: u8,
    pub base_lot_size: BaseLotsPerBaseUnit,
    pub quote_lot_size: QuoteLotsPerQuoteUnit,
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

// impl IndexedMarket {
//     pub fn from_index(
//         index: usize,
//         custom_market_list: &[IndexedMarket],
//         custom_token_list: &[Address],
//     ) -> Result<Self, GoblinError> {
//         if index < custom_market_list.len() {
//             custom_market_list[index]
//         } else if index > 127 && index < (127 + HARDCODED_MARKETS.len()) {
//             Ok(HARDCODED_MARKETS[index - 127])
//         } else {
//             Err(GoblinError::NoMarketAtIndex)
//         }
//     }
// }

#[derive(Clone, Copy)]
pub struct Market {
    base_token: Address,
    quote_token: Address,
    base_lot_size: BaseLotsPerBaseUnit,
    quote_lot_size: QuoteLotsPerQuoteUnit,
    tick_size: QuoteLotsPerBaseUnitPerTick,
}

impl Market {
    pub(crate) const fn new_unchecked(
        base_token: Address,
        quote_token: Address,
        base_lot_size: BaseLotsPerBaseUnit,
        quote_lot_size: QuoteLotsPerQuoteUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
    ) -> Self {
        Self {
            base_token,
            quote_token,
            base_lot_size,
            quote_lot_size,
            tick_size,
        }
    }

    pub fn from_index(
        index: usize,
        custom_market_list: &[IndexedMarket],
        custom_token_list: &[Address],
    ) -> Result<Self, GoblinError> {
        if index < custom_market_list.len() {
            let indexed_market = custom_market_list[index];

            let base_token = Token::get_token_by_index(
                custom_token_list,
                indexed_market.base_token_index as usize,
            )?;
            let quote_token = Token::get_token_by_index(
                custom_token_list,
                indexed_market.base_token_index as usize,
            )?;

            Market::new(
                *base_token.address(),
                *quote_token.address(),
                indexed_market.base_lot_size,
                indexed_market.quote_lot_size,
                indexed_market.tick_size,
            )
        } else if index > 127 && index < (127 + HARDCODED_MARKETS.len()) {
            Ok(HARDCODED_MARKETS[index - 127])
        } else {
            Err(GoblinError::NoMarketAtIndex)
        }
    }

    fn new(
        base_token: Address,
        quote_token: Address,
        base_lot_size: BaseLotsPerBaseUnit,
        quote_lot_size: QuoteLotsPerQuoteUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
    ) -> Result<Self, GoblinError> {
        require!(base_token != quote_token, GoblinError::InvalidMarket);
        require!(base_lot_size.valid(), GoblinError::InvalidMarket);
        require!(quote_lot_size.valid(), GoblinError::InvalidMarket);
        require!(tick_size % base_lot_size == 0, GoblinError::InvalidMarket);

        Ok(Self {
            base_token,
            quote_token,
            base_lot_size,
            quote_lot_size,
            tick_size,
        })
    }

    // Getters
    #[inline(always)]
    pub const fn base_token(&self) -> &Address {
        &self.base_token
    }

    #[inline(always)]
    pub const fn quote_token(&self) -> &Address {
        &self.quote_token
    }

    #[inline(always)]
    pub const fn base_lot_size(&self) -> BaseLotsPerBaseUnit {
        self.base_lot_size
    }

    #[inline(always)]
    pub const fn quote_lot_size(&self) -> QuoteLotsPerQuoteUnit {
        self.quote_lot_size
    }

    #[inline(always)]
    pub const fn tick_size(&self) -> QuoteLotsPerBaseUnitPerTick {
        self.tick_size
    }
}
