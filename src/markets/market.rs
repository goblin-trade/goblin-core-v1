use crate::{
    goblin_error::GoblinError,
    hostio::{hostio_native_keccak256, HostioBuffer},
    markets::HARDCODED_MARKETS,
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    require,
    state::SlotKey,
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

impl IndexedMarket {
    pub(crate) const fn new_unchecked(
        base_token_index: u8,
        quote_token_index: u8,
        base_lot_size: BaseLotsPerBaseUnit,
        quote_lot_size: QuoteLotsPerQuoteUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
    ) -> Self {
        IndexedMarket {
            base_token_index,
            quote_token_index,
            base_lot_size,
            quote_lot_size,
            tick_size,
        }
    }

    pub fn from_index(
        index: usize,
        dangerous_custom_market_list: &[IndexedMarket],
    ) -> Result<Self, GoblinError> {
        if index < dangerous_custom_market_list.len() {
            let dangerous_custom_market = dangerous_custom_market_list[index];
            require!(
                dangerous_custom_market.is_valid(),
                GoblinError::InvalidMarket
            );

            Ok(dangerous_custom_market)
        } else if index > 127 && index < (127 + HARDCODED_MARKETS.len()) {
            Ok(HARDCODED_MARKETS[index - 127])
        } else {
            Err(GoblinError::NoMarketAtIndex)
        }
    }

    fn is_valid(&self) -> bool {
        let base_lot_size = self.base_lot_size;
        let quote_lot_size = self.quote_lot_size;
        self.base_token_index != self.quote_token_index
            && base_lot_size.valid()
            && quote_lot_size.valid()
            && self.tick_size % self.base_lot_size == 0
    }

    pub fn to_market(&self, custom_erc20_list: &[Address]) -> Result<Market, GoblinError> {
        let base_token =
            Token::get_token_by_index(custom_erc20_list, self.base_token_index as usize)?;
        let quote_token =
            Token::get_token_by_index(custom_erc20_list, self.base_token_index as usize)?;

        Ok(Market {
            base_token: *base_token.address(),
            quote_token: *quote_token.address(),
            base_lot_size: self.base_lot_size,
            quote_lot_size: self.quote_lot_size,
            tick_size: self.tick_size,
        })
    }
}

pub struct MarketKey {
    hash: HostioBuffer<[u8; 32]>,
}

impl SlotKey for MarketKey {
    const DISCRIMINATOR: u8 = 3;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}

impl MarketKey {
    pub fn new(
        base_token: &Address,
        quote_token: &Address,
        base_lot_size: BaseLotsPerBaseUnit,
        quote_lot_size: QuoteLotsPerQuoteUnit,
        tick_size: QuoteLotsPerBaseUnitPerTick,
    ) -> Self {
        let mut bytes = [0u8; (1 + 2 * 20 + 3 * 8)];
        bytes[0] = Self::DISCRIMINATOR;
        bytes[1..21].copy_from_slice(base_token);
        bytes[21..41].copy_from_slice(quote_token);
        bytes[41..49].copy_from_slice(&base_lot_size.0.to_le_bytes());
        bytes[49..57].copy_from_slice(&quote_lot_size.0.to_le_bytes());
        bytes[57..65].copy_from_slice(&tick_size.0.to_le_bytes());

        let hash = unsafe { hostio_native_keccak256(bytes.as_slice()) };

        Self { hash }
    }
}

#[derive(Clone, Copy)]
pub struct Market {
    pub base_token: Address,
    pub quote_token: Address,
    pub base_lot_size: BaseLotsPerBaseUnit,
    pub quote_lot_size: QuoteLotsPerQuoteUnit,
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

// impl Market {
//     pub(crate) const fn new_unchecked(
//         base_token: Address,
//         quote_token: Address,
//         base_lot_size: BaseLotsPerBaseUnit,
//         quote_lot_size: QuoteLotsPerQuoteUnit,
//         tick_size: QuoteLotsPerBaseUnitPerTick,
//     ) -> Self {
//         Self {
//             base_token,
//             quote_token,
//             base_lot_size,
//             quote_lot_size,
//             tick_size,
//         }
//     }

//     // pub fn from_index(
//     //     index: usize,
//     //     custom_market_list: &[IndexedMarket],
//     //     custom_token_list: &[Address],
//     // ) -> Result<Self, GoblinError> {
//     //     if index < custom_market_list.len() {
//     //         let indexed_market = custom_market_list[index];

//     //         let base_token = Token::get_token_by_index(
//     //             custom_token_list,
//     //             indexed_market.base_token_index as usize,
//     //         )?;
//     //         let quote_token = Token::get_token_by_index(
//     //             custom_token_list,
//     //             indexed_market.base_token_index as usize,
//     //         )?;

//     //         Market::new(
//     //             *base_token.address(),
//     //             *quote_token.address(),
//     //             indexed_market.base_lot_size,
//     //             indexed_market.quote_lot_size,
//     //             indexed_market.tick_size,
//     //         )
//     //     } else if index > 127 && index < (127 + HARDCODED_MARKETS.len()) {
//     //         Ok(HARDCODED_MARKETS[index - 127])
//     //     } else {
//     //         Err(GoblinError::NoMarketAtIndex)
//     //     }
//     // }

//     fn new(
//         base_token: Address,
//         quote_token: Address,
//         base_lot_size: BaseLotsPerBaseUnit,
//         quote_lot_size: QuoteLotsPerQuoteUnit,
//         tick_size: QuoteLotsPerBaseUnitPerTick,
//     ) -> Result<Self, GoblinError> {
//         require!(base_token != quote_token, GoblinError::InvalidMarket);
//         require!(base_lot_size.valid(), GoblinError::InvalidMarket);
//         require!(quote_lot_size.valid(), GoblinError::InvalidMarket);
//         require!(tick_size % base_lot_size == 0, GoblinError::InvalidMarket);

//         Ok(Self {
//             base_token,
//             quote_token,
//             base_lot_size,
//             quote_lot_size,
//             tick_size,
//         })
//     }

//     // Getters
//     #[inline(always)]
//     pub const fn base_token(&self) -> &Address {
//         &self.base_token
//     }

//     #[inline(always)]
//     pub const fn quote_token(&self) -> &Address {
//         &self.quote_token
//     }

//     #[inline(always)]
//     pub const fn base_lot_size(&self) -> BaseLotsPerBaseUnit {
//         self.base_lot_size
//     }

//     #[inline(always)]
//     pub const fn quote_lot_size(&self) -> QuoteLotsPerQuoteUnit {
//         self.quote_lot_size
//     }

//     #[inline(always)]
//     pub const fn tick_size(&self) -> QuoteLotsPerBaseUnitPerTick {
//         self.tick_size
//     }
// }
