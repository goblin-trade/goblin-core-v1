use core::marker::PhantomData;

use crate::{
    goblin_error::GoblinError,
    hostio::{self, HostioBuffer},
    markets::CommonMarket,
    state::{DynamicMarketHasher, SlotKey},
    token::{CustomToken, DynamicIndex, TokenMarker, ERC20, ETH},
    types::{Base, Quote, TupleReader},
};

/// The key for a custom market
pub struct DynamicMarketKey<B: TokenMarker, Q: TokenMarker> {
    hash: HostioBuffer<[u8; 32]>,
    _marker: PhantomData<(B, Q)>,
}

impl<B: TokenMarker, Q: TokenMarker> DynamicMarketKey<B, Q> {
    pub const BYTE_SIZE_INNER: usize =
        1 + 8 * 3 + core::mem::size_of::<B::Address>() + core::mem::size_of::<Q::Address>();

    pub fn new(hash: HostioBuffer<[u8; 32]>) -> Self {
        Self {
            hash,
            _marker: PhantomData,
        }
    }

    pub fn set_common_fields<const N: usize>(
        bytes: &mut [u8; N],
        market: &CommonMarket<DynamicIndex, B, Q>,
    ) {
        bytes[0] = Self::DISCRIMINATOR;

        bytes[1..9].copy_from_slice(&Base::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[9..17].copy_from_slice(&Quote::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[17..25].copy_from_slice(&market.tick_size.inner.to_le_bytes());
    }

    pub fn hash_v2_inner<const N: usize>(
        market: &CommonMarket<DynamicIndex, B, Q>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<Self, GoblinError> {
        let mut bytes = [0u8; N];

        bytes[0] = Self::DISCRIMINATOR;

        bytes[1..9].copy_from_slice(&Base::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[9..17].copy_from_slice(&Quote::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[17..25].copy_from_slice(&market.tick_size.inner.to_le_bytes());

        let offset = &mut 25;

        let base_token_index = Base::get(&market.token_index_pair);
        let quote_token_index = Quote::get(&market.token_index_pair);

        B::set_token_address(&mut bytes, offset, base_token_index, custom_erc20_list)?;
        Q::set_token_address(&mut bytes, offset, quote_token_index, custom_erc20_list)?;

        let hash = hostio::native_keccak256(bytes.as_slice());
        Ok(Self::new(hash))
    }

    // pub fn hash_v3(market: &CommonMarket<DynamicIndex, B, Q>)
    // where
    //     Self: DynamicMarketHasher<B, Q>,
    // {
    //     let mut buffer = <Self as DynamicMarketHasher<B, Q>>::new_buffer();

    //     buffer[0] = Self::DISCRIMINATOR;
    //     // Self::set_common_fields(&mut buffer, market);
    // }
}

impl<B: TokenMarker, Q: TokenMarker> SlotKey for DynamicMarketKey<B, Q> {
    const DISCRIMINATOR: u8 = B::DISCRIMINATOR + Q::DISCRIMINATOR << 1;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}
