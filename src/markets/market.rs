use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    quantities::QuoteLotsPerBaseUnitPerTick,
    require,
    tokens::{
        DynamicIndex, HardcodedDecoder, HardcodedToken, MarketVariant, PairShape, TokenIndex,
        TokenPair, TokenPairKind,
    },
    types::{Base, LegMarker, Pair, Quote},
};

pub type LotSizePair = Pair<<Base as LegMarker>::LotsPerUnit, <Quote as LegMarker>::LotsPerUnit>;

pub struct CommonMarket<M: MarketVariant, P: PairShape>
where
    TokenPair<M, P>: TokenPairKind,
{
    /// The token pair, parameterized by shape and variant.
    pub token_index_pair: <TokenPair<M, P> as TokenPairKind>::IndexPair,

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

/// A market whose token indices are dynamically specified at runtime.
/// Works with any token pair shape (ETH–ERC20, ERC20–ETH, ERC20–ERC20).
pub type DynamicMarket<P: PairShape> = CommonMarket<DynamicIndex, P>;

/// `Decodable` impl for HardcodedMarket<P>
impl<P> Decodable<&'static HardcodedMarket<P>> for HardcodedMarket<P>
where
    P: PairShape + 'static,
    TokenPair<TokenIndex<HardcodedToken>, P>: TokenPairKind,
    Self: HardcodedDecoder<P>,
{
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<&'static HardcodedMarket<P>, GoblinError> {
        let market_index_raw = payload.decode::<u8>(offset, len)? as usize;

        Self::HARDCODED_MARKET_LIST
            .get(market_index_raw)
            .ok_or(GoblinError::InvalidHardcodedMarket)
    }
}

/// Decodable implementation for dynamic markets
impl<P> Decodable<DynamicMarket<P>> for DynamicMarket<P>
where
    P: PairShape,
    TokenPair<DynamicIndex, P>:
        TokenPairKind + Decodable<<TokenPair<DynamicIndex, P> as TokenPairKind>::IndexPair>,
{
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<DynamicMarket<P>, GoblinError> {
        let token_index_pair = TokenPair::<DynamicIndex, P>::decode(payload, offset, len)?;

        require!(len >= *offset + 3, GoblinError::InvalidPayload);
        let lot_size_pair = *payload.decode_ref_unchecked::<LotSizePair>(offset);
        let tick_size = *payload.decode_ref_unchecked::<QuoteLotsPerBaseUnitPerTick>(offset);

        Ok(DynamicMarket::<P> {
            token_index_pair,
            lot_size_pair,
            tick_size,
        })
    }
}
