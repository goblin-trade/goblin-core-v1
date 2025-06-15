use crate::{
    goblin_error::GoblinError,
    markets::HARDCODED_MARKETS,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick},
    tokens::get_token_by_index,
    types::Address,
};

// Max number of custom markets
pub const MAX_CUSTOM_MARKETS: usize = 7;

/// Input payload receives an array of MarketItems.
/// These hold token indices instead of token addresses. The tokens
/// can be mapped to obtain `Market` struct
#[repr(C, packed)]
pub struct MarketItem {
    pub base_token_index: u8,
    pub quote_token: u8,
    pub base_lot_size: BaseLotsPerBaseUnit,
    pub quote_lot_size: QuoteLotsPerBaseUnit,
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

#[derive(Clone, Copy)]
pub struct Market {
    base_token: Address,
    pub quote_token: Address,
    pub base_lot_size: BaseLotsPerBaseUnit,
    pub quote_lot_size: QuoteLotsPerBaseUnit,
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

impl Market {
    pub(crate) const fn new_unchecked(
        base_token: Address,
        quote_token: Address,
        base_lot_size: BaseLotsPerBaseUnit,
        quote_lot_size: QuoteLotsPerBaseUnit,
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

    pub const fn new(
        base_token: Address,
        quote_token: Address,
        base_lot_size: BaseLotsPerBaseUnit,
        quote_lot_size: QuoteLotsPerBaseUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
    ) -> Result<Self, GoblinError> {
        // TODO validate params
        Ok(Self {
            base_token,
            quote_token,
            base_lot_size,
            quote_lot_size,
            tick_size,
        })
    }

    pub fn from_index(
        index: usize,
        custom_market_list: &[MarketItem],
        custom_token_list: &[Address],
    ) -> Result<Self, GoblinError> {
        match custom_market_list.get(index) {
            Some(market_item) => {
                let base_token =
                    get_token_by_index(custom_token_list, market_item.base_token_index as usize)?;
                let quote_token =
                    get_token_by_index(custom_token_list, market_item.base_token_index as usize)?;

                Market::new(
                    base_token,
                    quote_token,
                    market_item.base_lot_size,
                    market_item.quote_lot_size,
                    market_item.tick_size,
                )
            }
            None => {
                if index > 127 && index < (127 + HARDCODED_MARKETS.len()) {
                    Ok(HARDCODED_MARKETS[index - 127])
                } else {
                    Err(GoblinError::NoMarketAtIndex)
                }
            }
        }
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
    pub const fn quote_lot_size(&self) -> QuoteLotsPerBaseUnit {
        self.quote_lot_size
    }

    #[inline(always)]
    pub const fn tick_size(&self) -> QuoteLotsPerBaseUnitPerTick {
        self.tick_size
    }
}
