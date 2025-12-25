use crate::{
    goblin_error::GoblinError,
    hostio,
    market::{CommonMarket, Dynamic},
    state::{DynamicMarketHasher, DynamicMarketKey},
    token::{CustomToken, ERC20, ETH},
    types::{Base, TupleReader},
};

impl DynamicMarketHasher<ERC20, ETH> for DynamicMarketKey<ERC20, ETH> {
    fn hash(
        market: &CommonMarket<Dynamic, ERC20, ETH>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; Self::BUFFER_SIZE];
        Self::set_common_fields(&mut bytes, market);

        let base_token_index = Base::get(&market.token_index_pair);
        let quote_address = base_token_index.address(custom_erc20_list)?;
        bytes[25..45].copy_from_slice(&quote_address);

        let hash = hostio::native_keccak256(bytes.as_slice());
        Ok(Self::new(hash))
    }
}
