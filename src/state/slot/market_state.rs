use core::marker::PhantomData;

use crate::{
    goblin_error::GoblinError,
    hostio::{self, HostioBuffer},
    markets::{DynamicMarket, MarketVariant, PairShape, ERC20, ETH},
    quantities::Ticks,
    state::{SlotKey, SlotState},
    tokens::{CustomToken, DynamicIndex, HardcodedToken, TokenIndex},
    types::Pair,
};

/// The hash is hardcoded for hardcoded markets
pub struct HardcodedMarketKey<P: PairShape> {
    hash: [u8; 32],
    _marker: PhantomData<P>,
}

impl<P: PairShape> HardcodedMarketKey<P> {
    pub const fn new(hash: [u8; 32]) -> Self {
        Self {
            hash,
            _marker: PhantomData,
        }
    }
}

impl<P: PairShape> SlotKey for HardcodedMarketKey<P> {
    // The PairShape discriminator not used at runtime. But it is used for
    // pre-computing the hash for hardcoding.
    const DISCRIMINATOR: u8 = P::DISCRIMINATOR;

    fn hash(&self) -> &[u8; 32] {
        &self.hash
    }
}

/// The key for a custom market
pub struct DynamicMarketKey<P: PairShape> {
    hash: HostioBuffer<[u8; 32]>,
    _marker: PhantomData<P>,
}

pub trait DynamicMarketHasher<P>
where
    P: PairShape,
    Self: Sized,
{
    const BYTE_SIZE: usize;

    fn hash(
        market: &DynamicMarket<P>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError>;
}

impl DynamicMarketHasher<Pair<ETH, ERC20>> for DynamicMarketKey<Pair<ETH, ERC20>> {
    const BYTE_SIZE: usize = 1 + 20 * 1 + 8 * 3;

    fn hash(
        market: &DynamicMarket<Pair<ETH, ERC20>>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        bytes[0] = Self::DISCRIMINATOR;

        let quote_address = market.token_index_pair.address_bytes(custom_erc20_list)?;

        bytes[1..21].copy_from_slice(&quote_address);

        bytes[21..29].copy_from_slice(&market.lot_size_pair.base.inner.to_le_bytes());
        bytes[29..37].copy_from_slice(&market.lot_size_pair.quote.inner.to_le_bytes());
        bytes[37..45].copy_from_slice(&market.tick_size.inner.to_le_bytes());

        let hash = hostio::native_keccak256(bytes.as_slice());

        Ok(Self {
            hash,
            _marker: PhantomData,
        })
    }
}

impl DynamicMarketHasher<Pair<ERC20, ETH>> for DynamicMarketKey<Pair<ERC20, ETH>> {
    const BYTE_SIZE: usize = 1 + 20 * 1 + 8 * 3;

    fn hash(
        market: &DynamicMarket<Pair<ERC20, ETH>>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        bytes[0] = Self::DISCRIMINATOR;

        let quote_address = market.token_index_pair.address_bytes(custom_erc20_list)?;

        bytes[1..21].copy_from_slice(&quote_address);

        bytes[21..29].copy_from_slice(&market.lot_size_pair.base.inner.to_le_bytes());
        bytes[29..37].copy_from_slice(&market.lot_size_pair.quote.inner.to_le_bytes());
        bytes[37..45].copy_from_slice(&market.tick_size.inner.to_le_bytes());

        let hash = hostio::native_keccak256(bytes.as_slice());

        Ok(Self {
            hash,
            _marker: PhantomData,
        })
    }
}

impl DynamicMarketHasher<Pair<ERC20, ERC20>> for DynamicMarketKey<Pair<ERC20, ERC20>> {
    const BYTE_SIZE: usize = 1 + 20 * 2 + 8 * 3;

    fn hash(
        market: &DynamicMarket<Pair<ERC20, ERC20>>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        bytes[0] = Self::DISCRIMINATOR;

        let base_address = market
            .token_index_pair
            .base
            .address_bytes(custom_erc20_list)?;

        let quote_address = market
            .token_index_pair
            .base
            .address_bytes(custom_erc20_list)?;

        bytes[1..21].copy_from_slice(&base_address);
        bytes[21..41].copy_from_slice(&quote_address);

        bytes[41..49].copy_from_slice(&market.lot_size_pair.base.inner.to_le_bytes());
        bytes[49..57].copy_from_slice(&market.lot_size_pair.quote.inner.to_le_bytes());
        bytes[57..65].copy_from_slice(&market.tick_size.inner.to_le_bytes());

        let hash = hostio::native_keccak256(bytes.as_slice());

        Ok(Self {
            hash,
            _marker: PhantomData,
        })
    }
}

impl<P: PairShape> SlotKey for DynamicMarketKey<P> {
    const DISCRIMINATOR: u8 = P::DISCRIMINATOR;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}

/// The market state slot
/// We have 6 possible sub-types based on MarketVariant and PairShape
#[repr(C)]
pub struct MarketState<M: MarketVariant, P: PairShape> {
    pub best_prices: Pair<Ticks, Ticks>,
    /// Padding to match 32 bits
    _padding: [u8; 16],
    _marker: PhantomData<(M, P)>,
}

impl<P: PairShape> SlotState<HardcodedMarketKey<P>> for MarketState<TokenIndex<HardcodedToken>, P> {}
impl<P: PairShape> SlotState<DynamicMarketKey<P>> for MarketState<DynamicIndex, P> {}
