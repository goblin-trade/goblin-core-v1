use crate::{
    goblin_error::GoblinError,
    hostio::{self},
    markets::{CommonMarket, PairShape},
    state::{DynamicMarketKey, SlotKey},
    token::{CustomToken, DynamicIndex, ERC20, ETH},
    types::{Base, Quote, TupleReader},
};

pub trait DynamicMarketHasher<P>
where
    P: PairShape,
    Self: Sized,
{
    const BYTE_SIZE: usize;

    fn hash(
        market: &CommonMarket<DynamicIndex, P>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError>;
}

impl DynamicMarketHasher<(ETH, ERC20)> for DynamicMarketKey<(ETH, ERC20)> {
    const BYTE_SIZE: usize = 1 + 20 * 1 + 8 * 3;

    fn hash(
        market: &CommonMarket<DynamicIndex, (ETH, ERC20)>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        bytes[0] = Self::DISCRIMINATOR;

        let quote_address = market.token_index_pair.address(custom_erc20_list)?;

        bytes[1..21].copy_from_slice(&quote_address);

        bytes[21..29].copy_from_slice(&Base::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[29..37].copy_from_slice(&Quote::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[37..45].copy_from_slice(&market.tick_size.inner.to_le_bytes());

        let hash = hostio::native_keccak256(bytes.as_slice());

        Ok(Self::new(hash))
    }
}

impl DynamicMarketHasher<(ERC20, ETH)> for DynamicMarketKey<(ERC20, ETH)> {
    const BYTE_SIZE: usize = 1 + 20 * 1 + 8 * 3;

    fn hash(
        market: &CommonMarket<DynamicIndex, (ERC20, ETH)>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        bytes[0] = Self::DISCRIMINATOR;

        let quote_address = market.token_index_pair.address(custom_erc20_list)?;

        bytes[1..21].copy_from_slice(&quote_address);

        bytes[21..29].copy_from_slice(&Base::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[29..37].copy_from_slice(&Quote::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[37..45].copy_from_slice(&market.tick_size.inner.to_le_bytes());

        let hash = hostio::native_keccak256(bytes.as_slice());

        Ok(Self::new(hash))
    }
}

impl DynamicMarketHasher<(ERC20, ERC20)> for DynamicMarketKey<(ERC20, ERC20)> {
    const BYTE_SIZE: usize = 1 + 20 * 2 + 8 * 3;

    fn hash(
        market: &CommonMarket<DynamicIndex, (ERC20, ERC20)>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        bytes[0] = Self::DISCRIMINATOR;

        let base_address = Base::get(&market.token_index_pair).address(custom_erc20_list)?;
        let quote_address = Quote::get(&market.token_index_pair).address(custom_erc20_list)?;

        bytes[1..21].copy_from_slice(&base_address);
        bytes[21..41].copy_from_slice(&quote_address);

        bytes[41..49].copy_from_slice(&Base::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[49..57].copy_from_slice(&Quote::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[57..65].copy_from_slice(&market.tick_size.inner.to_le_bytes());

        let hash = hostio::native_keccak256(bytes.as_slice());

        Ok(Self::new(hash))
    }
}
