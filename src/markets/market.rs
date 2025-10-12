use crate::{
    markets::LotSizePair,
    quantities::QuoteLotsPerBaseUnitPerTick,
    tokens::{
        DynamicIndex, HardcodedToken, MarketVariant, PairShape, TokenIndex, TokenPair,
        TokenPairKind,
    },
};

pub struct CommonMarket<K: TokenPairKind> {
    /// The token pair, parameterized by shape and variant.
    pub token_pair: K::IndexPair,

    /// Lot sizes (one per side)
    pub lot_size_pair: LotSizePair,

    /// Tick size (quote lots per base unit per tick)
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

pub type DynamicMarket<P: PairShape> = CommonMarket<TokenPair<DynamicIndex, P>>;
pub type HcMarket<P: PairShape> = CommonMarket<TokenPair<TokenIndex<HardcodedToken>, P>>;

pub struct HardcodedMarketV2<P>
where
    P: PairShape,
{
    pub common: HcMarket<P>,
}

// /// A market whose token indices are hardcoded.
// /// Works with any token pair shape (ETH–ERC20, ERC20–ETH, ERC20–ERC20).
// pub struct HardcodedMarket<P: PairShape> {
//     /// The common market configuration (lot sizes, tick size, token indices).
//     pub common: CommonMarket<MarketPair<TokenIndex<HardcodedToken>, P>>,

//     /// The keccak256 hash of this market’s identifier.
//     pub keccak_hash: [u8; 32],
// }

// pub type DynamicMarket<P: PairShape> = CommonMarket<P, DynamicIndex>;

// pub struct HardcodedMarket<P: PairShape> {
//     /// The common market configuration (lot sizes, tick size, token indices).
//     pub common: CommonMarket<MarketPair<M, P>>,

//     /// The keccak256 hash of this market’s identifier.
//     pub keccak_hash: [u8; 32],
// }

// /// Common fields in each market
// pub struct CommonMarket<P, V>
// where
//     P: TokenPairShape<V>,
//     V: MarketVariant,
// {
//     /// The token pair, parameterized by shape and variant.
//     pub token_pair: <P as TokenPairShape<V>>::IndexPair,

//     /// Lot sizes (one per side)
//     pub lot_size_pair: LotSizePair,

//     /// Tick size (quote lots per base unit per tick)
//     pub tick_size: QuoteLotsPerBaseUnitPerTick,
// }

// /// A market whose token indices are hardcoded.
// /// Works with any token pair shape (ETH–ERC20, ERC20–ETH, ERC20–ERC20).
// pub struct HardcodedMarket<P>
// where
//     P: TokenPairShape<TokenIndex<HardcodedToken>>,
// {
//     /// The common market configuration (lot sizes, tick size, token indices).
//     pub common: CommonMarket<P, TokenIndex<HardcodedToken>>,

//     /// The keccak256 hash of this market’s identifier.
//     pub keccak_hash: [u8; 32],
// }

// /// A market whose token indices are dynamically specified at runtime.
// /// Works with any token pair shape (ETH–ERC20, ERC20–ETH, ERC20–ERC20).
// pub type DynamicMarket<P> = CommonMarket<P, DynamicIndex>;
