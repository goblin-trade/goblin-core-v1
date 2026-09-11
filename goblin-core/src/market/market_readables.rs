use crate::{
    axis::token::{TokenDataTriple, token_list::CustomERC20List},
    axis_helpers::TokenPair,
    market::CommonMarket,
    state::{ConstPreimage, MarketPreimage, SlotKey},
};

pub struct MarketReadables<TP: TokenPair> {
    pub market: CommonMarket<TP>,
    pub market_key: SlotKey<MarketPreimage<TP>>,
}

impl<TP: TokenPair> MarketReadables<TP> {
    pub const fn get_const(market: CommonMarket<TP>) -> Self {
        if !market.lot_size_pair.valid() {
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
}
