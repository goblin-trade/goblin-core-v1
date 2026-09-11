use crate::{
    axis::token::{
        TokenDataTriple,
        token_list::{CustomERC20List, HARDCODED_ERC20_LIST},
    },
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    market::CommonMarket,
    require,
    state::{MarketPreimage, SlotKey},
};

pub struct MarketReadables<TP: TokenPair> {
    pub market: CommonMarket<TP>,
    pub market_key: SlotKey<MarketPreimage<TP>>,
}

impl<TP: TokenPair> MarketReadables<TP> {
    pub const fn get_hardcoded(market: CommonMarket<TP>) -> Result<Self, GoblinError> {
        require!(market.lot_size_pair.valid(), GoblinError::InvalidLotSize);
        // problem- LegMarker::get() doesn't work with const
        //

        // let custom_erc20_list = CustomERC20List { inner: &[] };
        // let token_data_triple = TokenDataTriple::const_from(custom_erc20_list);

        // let preimage = market.get_preimage(&token_data_triple);

        // problem- this has cases for ETH which has no index.
        //
        // Only 2 options
        //
        // * Externally calculate and embed
        // * Enable nightly
        // let base_address = HARDCODED_ERC20_LIST.inner[market.token_index_pair.0];
        // let preimage = MarketPreimage {
        //     lot_size_pair: market.lot_size_pair,
        //     tick_size: market.tick_size,
        //     token_address_pair: Pair::new(HARDCODED_ERC20_LIST),
        // };

        Ok(Self {
            market,
            market_key: SlotKey::new([0u8; 32]),
        })
    }
}
