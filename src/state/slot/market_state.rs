use core::marker::PhantomData;

use crate::{
    hostio::{self, HostioBuffer},
    markets::DynamicMarket,
    quantities::{QuoteLotsPerBaseUnitPerTick, Ticks},
    state::{SlotKey, SlotState},
    tokens::{HardcodedToken, MarketVariant, PairShape, TokenIndex, TokenPair, ERC20, ETH},
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
pub struct CustomMarketKey<P: PairShape> {
    hash: HostioBuffer<[u8; 32]>,
    _marker: PhantomData<P>,
}

/// Each PairShape has a custom implementation
impl CustomMarketKey<Pair<ETH, ERC20>> {
    const BYTE_SIZE: usize = 20 * 2 + 8 * 3;

    pub fn new(market: DynamicMarket<Pair<ERC20, ERC20>>) {
        let mut bytes = [0u8; Self::BYTE_SIZE];
        bytes[0] = Self::DISCRIMINATOR;

        // We need address not index. We can't pass DynamicMarket, instead we need to pass addresses
        bytes[1..21].copy_from_slice(market.token_index_pair);
    }
}

impl<P: PairShape> SlotKey for CustomMarketKey<P> {
    const DISCRIMINATOR: u8 = P::DISCRIMINATOR;

    fn hash(&self) -> &[u8; 32] {
        self.hash.as_ref()
    }
}

// impl<P: PairShape> CustomMarketKey<P> {
//     // TODO read custom market from args
//     pub fn new() {}
// }

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

// pub struct MarketKey {
//     hash: HostioBuffer<[u8; 32]>,
// }

// pub struct MarketKey {
//     hash: HostioBuffer<[u8; 32]>,
// }

// impl SlotKey for MarketKey {
//     const DISCRIMINATOR: u8 = 3;

//     fn hash(&self) -> &[u8; 32] {
//         self.hash.as_ref()
//     }
// }

// impl MarketKey {
//     pub fn new(
//         token_pair: &ValidatedTokenPair,
//         lot_size_pair: LotSizePair,
//         tick_size: QuoteLotsPerBaseUnitPerTick,
//     ) -> Self {
//         match token_pair {
//             ValidatedTokenPair::ERC20ERC20(ERC20TokenPair {
//                 base: base_token,
//                 quote: quote_token,
//             }) => {
//                 let mut bytes = [0u8; (1 + 2 * 20 + 3 * 8)];
//                 bytes[0] = token_pair.discriminator();
//                 bytes[1..21].copy_from_slice(base_token.address());
//                 bytes[21..41].copy_from_slice(quote_token.address());
//                 bytes[41..49].copy_from_slice(&lot_size_pair.base.inner.to_le_bytes());
//                 bytes[49..57].copy_from_slice(&lot_size_pair.quote.inner.to_le_bytes());
//                 bytes[57..65].copy_from_slice(&tick_size.inner.to_le_bytes());

//                 let hash = hostio::native_keccak256(bytes.as_slice());

//                 Self { hash }
//             }
//             ValidatedTokenPair::ETHERC20(erc20_token)
//             | ValidatedTokenPair::ERC20ETH(erc20_token) => {
//                 let mut bytes = [0u8; (1 + 20 + 3 * 8)];
//                 bytes[0] = token_pair.discriminator();
//                 bytes[1..21].copy_from_slice(erc20_token.address());
//                 bytes[21..29].copy_from_slice(&lot_size_pair.base.inner.to_le_bytes());
//                 bytes[29..37].copy_from_slice(&lot_size_pair.quote.inner.to_le_bytes());
//                 bytes[37..45].copy_from_slice(&tick_size.inner.to_le_bytes());

//                 let hash = hostio::native_keccak256(bytes.as_slice());

//                 Self { hash }
//             }
//         }
//     }
// }

// /// The market state stored in a slot
// #[repr(C)]
// pub struct MarketState {
//     pub best_prices: Pair<Ticks, Ticks>,
//     /// Padding to match 32 bits
//     _padding: [u8; 16],
// }

// impl SlotState<MarketKey> for MarketState {}

// // Better to move side related functions here
