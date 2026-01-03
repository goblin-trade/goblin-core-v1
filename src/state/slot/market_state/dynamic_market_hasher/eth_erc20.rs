use crate::{
    goblin_error::GoblinError,
    market::{CommonMarket, Dynamic},
    state::{DynamicMarketHasher, MarketState, SlotKey},
    token::{CustomToken, ERC20, ETH},
    types::{Quote, TupleReader},
};

impl DynamicMarketHasher<ETH, ERC20> for MarketState<Dynamic, ETH, ERC20> {
    fn compute_slot_key(
        market: &CommonMarket<Dynamic, ETH, ERC20>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<SlotKey<Self>, GoblinError> {
        let mut bytes = [0u8; Self::BUFFER_SIZE];
        Self::set_common_fields(&mut bytes, market);

        let quote_token_index = Quote::get(&market.token_index_pair);
        let quote_address = quote_token_index.address(custom_erc20_list)?;
        bytes[25..45].copy_from_slice(&quote_address);

        Ok(SlotKey::generate(bytes.as_slice()))
    }
}
