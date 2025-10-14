use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder},
    markets::LotSizePair,
    quantities::QuoteLotsPerBaseUnitPerTick,
    tokens::{
        DynamicIndex, HardcodedDecoder, HardcodedToken, MarketVariant, PairShape, TokenIndex,
        TokenPair, TokenPairKind,
    },
};

pub struct CommonMarket<M: MarketVariant, P: PairShape>
where
    TokenPair<M, P>: TokenPairKind,
{
    /// The token pair, parameterized by shape and variant.
    pub token_pair: <TokenPair<M, P> as TokenPairKind>::IndexPair,

    /// Lot sizes (one per side)
    pub lot_size_pair: LotSizePair,

    /// Tick size (quote lots per base unit per tick)
    pub tick_size: QuoteLotsPerBaseUnitPerTick,
}

/// A market hardcoded within the smart contract. It keccak hash is also hardcoded,
/// allowing slot reads without having to compute hash at runtime.
///
/// * All token indices in hardcoded markets are hardcoded.
/// * It has 3 variants corresponding to the 3 pair shapes
pub struct HardcodedMarket<P: PairShape>
where
    TokenPair<TokenIndex<HardcodedToken>, P>: TokenPairKind,
{
    /// The common market configuration (lot sizes, tick size, token indices).
    pub common: CommonMarket<TokenIndex<HardcodedToken>, P>,

    /// The keccak256 hash of this market’s identifier.
    pub keccak_hash: [u8; 32],
}

impl<P> HardcodedMarket<P>
where
    P: PairShape + 'static,
    TokenPair<TokenIndex<HardcodedToken>, P>: TokenPairKind,
    Self: HardcodedDecoder<P>,
{
    pub fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<&'static Self, GoblinError> {
        let market_index_raw = payload.decode::<u8>(offset, len)? as usize;

        let markets = <Self as HardcodedDecoder<P>>::HARDCODED_MARKET_LIST;

        let market = markets
            .get(market_index_raw)
            .ok_or(GoblinError::InvalidHardcodedMarket)?;

        Ok(market)
    }
}

// impl<P: PairShape> HardcodedMarket<P>
// where
//     TokenPair<TokenIndex<HardcodedToken>, P>: TokenPairKind,
// {
//     pub fn decode(payload: &ArgsBuffer, offset: &mut usize, len: usize) -> Result<(), GoblinError> {
//         let market_index_raw = payload.decode::<u8>(offset, len)? as usize;

//         let markets = Self::HARDCODED_MARKET_LIST;

//         Ok(())
//     }
// }

/// A market whose token indices are dynamically specified at runtime.
/// Works with any token pair shape (ETH–ERC20, ERC20–ETH, ERC20–ERC20).
pub type DynamicMarket<P: PairShape> = CommonMarket<DynamicIndex, P>;
