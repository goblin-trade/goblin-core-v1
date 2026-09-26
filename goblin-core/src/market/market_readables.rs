use crate::{
    axis::token::{TokenDataTriple, token_list::CustomERC20List},
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    market::CommonMarket,
    state::{ConstPreimage, MarketPreimage, Preimage, SlotKey},
};

/// A resolved market: the decoded [`CommonMarket`] together with its derived
/// slot key.
///
/// The slot key is never taken from the wire; it is always computed in-contract
/// from the market's token addresses (see [`MarketReadables::from_market`]).
#[derive(Clone, Copy)]
pub struct MarketReadables<TP: TokenPair> {
    pub market: CommonMarket<TP>,
    pub market_key: SlotKey<MarketPreimage<TP>>,
}

impl<TP: TokenPair> MarketReadables<TP> {
    pub const fn get_const(market: CommonMarket<TP>) -> Self {
        if !market.lot_size_pair_u32.valid() {
            panic!("InvalidLotSize");
        }

        let custom_erc20_list = CustomERC20List { inner: &[] };
        let token_data_triple = TokenDataTriple::const_from(custom_erc20_list);

        match market.get_preimage(&token_data_triple) {
            Ok(preimage) => {
                let market_key = preimage.const_hash();

                Self { market, market_key }
            }
            Err(_) => panic!("market tokens not found"),
        }
    }

    /// Build a readables value from a decoded market, deriving the slot key from
    /// the token list referenced by `token_data_triple`.
    ///
    /// This is the runtime counterpart of [`Self::get_const`]. The key is
    /// computed here rather than accepted from the caller, so a forged key can
    /// never reach storage.
    pub fn from_market(
        market: CommonMarket<TP>,
        token_data_triple: &TokenDataTriple,
    ) -> Result<Self, GoblinError> {
        let market_key = market.get_preimage(token_data_triple)?.hash();
        Ok(Self { market, market_key })
    }
}
