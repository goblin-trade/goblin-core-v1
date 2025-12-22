use crate::{
    goblin_error::GoblinError,
    hostio::{self},
    markets::CommonMarket,
    state::DynamicMarketKey,
    token::{CustomToken, DynamicIndex, TokenMarker, ERC20, ETH},
    types::{Base, Quote, TupleReader},
};

pub trait DynamicMarketHasher<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
    Self: Sized,
{
    const BYTE_SIZE: usize =
        1 + 8 * 3 + core::mem::size_of::<B::Address>() + core::mem::size_of::<Q::Address>();

    fn hash(
        market: &CommonMarket<DynamicIndex, B, Q>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError>;
}

impl DynamicMarketHasher<ETH, ERC20> for DynamicMarketKey<ETH, ERC20> {
    fn hash(
        market: &CommonMarket<DynamicIndex, ETH, ERC20>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        Self::set_common_fields(&mut bytes, market);

        let quote_token_index = Quote::get(&market.token_index_pair);
        let quote_address = quote_token_index.address(custom_erc20_list)?;
        bytes[25..45].copy_from_slice(&quote_address);

        let hash = hostio::native_keccak256(bytes.as_slice());
        Ok(Self::new(hash))
    }
}

impl DynamicMarketHasher<ERC20, ETH> for DynamicMarketKey<ERC20, ETH> {
    fn hash(
        market: &CommonMarket<DynamicIndex, ERC20, ETH>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        Self::set_common_fields(&mut bytes, market);

        let base_token_index = Base::get(&market.token_index_pair);
        let quote_address = base_token_index.address(custom_erc20_list)?;
        bytes[25..45].copy_from_slice(&quote_address);

        let hash = hostio::native_keccak256(bytes.as_slice());
        Ok(Self::new(hash))
    }
}

impl DynamicMarketHasher<ERC20, ERC20> for DynamicMarketKey<ERC20, ERC20> {
    fn hash(
        market: &CommonMarket<DynamicIndex, ERC20, ERC20>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        Self::set_common_fields(&mut bytes, market);

        let base_token_index = Base::get(&market.token_index_pair);
        let quote_address = base_token_index.address(custom_erc20_list)?;
        bytes[25..45].copy_from_slice(&quote_address);

        let quote_token_index = Quote::get(&market.token_index_pair);
        let quote_address = quote_token_index.address(custom_erc20_list)?;
        bytes[45..65].copy_from_slice(&quote_address);

        let hash = hostio::native_keccak256(bytes.as_slice());
        Ok(Self::new(hash))
    }
}
