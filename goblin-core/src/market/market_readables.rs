use crate::{
    axis::token::{TokenDataTriple, token_list::CustomERC20List},
    axis_helpers::TokenPair,
    goblin_error::GoblinError,
    market::CommonMarket,
    require,
    state::{ConstPreimage, MarketPreimage, SlotKey},
};

pub struct MarketReadables<TP: TokenPair> {
    pub market: CommonMarket<TP>,
    pub market_key: SlotKey<MarketPreimage<TP>>,
}

impl<TP: TokenPair> MarketReadables<TP> {
    pub const fn get_hardcoded(market: CommonMarket<TP>) -> Self {
        // require!(market.lot_size_pair.valid(), GoblinError::InvalidLotSize);

        let custom_erc20_list = CustomERC20List { inner: &[] };
        let token_data_triple = TokenDataTriple::const_from(custom_erc20_list);

        match market.get_preimage(&token_data_triple) {
            Ok(preimage) => {
                let market_key = preimage.const_hash();

                Self { market, market_key }
            }
            Err(_) => panic!("market tokens not found"),
        }

        // let market_key = SlotKey::new([0u8; 32]);
        // Ok(Self { market, market_key })
    }
}
