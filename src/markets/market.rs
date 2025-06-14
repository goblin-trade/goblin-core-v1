use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick},
    tokens::get_token_by_index,
    types::Address,
};

// Max number of custom markets
pub const MAX_CUSTOM_MARKETS: usize = 7;

// Bytes per market item in market item list
// u8 + u8 + u64 + u64 + u64
pub const MARKET_ITEM_SIZE: usize = 1 + 1 + 8 + 8 + 8;

#[derive(Clone, Copy)]
pub struct Market {
    pub base_token: Address,
    pub quote_token: Address,
    pub base_lot_size: BaseLotsPerBaseUnit,
    pub quote_lot_size: QuoteLotsPerBaseUnit,
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

pub struct CustomMarketList {
    inner: [MaybeUninit<Market>; MAX_CUSTOM_MARKETS],
    pub len: usize,
}

impl CustomMarketList {
    fn default() -> Self {
        CustomMarketList {
            inner: [MaybeUninit::<Market>::uninit(); MAX_CUSTOM_MARKETS],
            len: 0,
        }
    }

    pub fn init(bytes: &[u8], custom_token_list: &[Address]) -> Result<Self, GoblinError> {
        debug_assert!(bytes.len() <= MAX_CUSTOM_MARKETS * MARKET_ITEM_SIZE);

        let mut list = CustomMarketList::default();
        list.len = bytes.len() / MARKET_ITEM_SIZE;

        for (i, chunk) in bytes.chunks_exact(MARKET_ITEM_SIZE).enumerate() {
            let base_token_index = chunk[0] as usize;
            let quote_token_index = chunk[1] as usize;
            let base_lot_size =
                BaseLotsPerBaseUnit(unsafe { *(chunk.as_ptr().add(2) as *const u64) });
            let quote_lot_size =
                QuoteLotsPerBaseUnit(unsafe { *(chunk.as_ptr().add(10) as *const u64) });
            let tick_size =
                QuoteLotsPerBaseUnitPerTick(unsafe { *(chunk.as_ptr().add(18) as *const u64) });

            let base_token = get_token_by_index(custom_token_list, base_token_index)?;
            let quote_token = get_token_by_index(custom_token_list, quote_token_index)?;

            list.inner[i].write(Market {
                base_token,
                quote_token,
                base_lot_size,
                quote_lot_size,
                tick_size,
            });
        }

        Ok(list)
    }
}
