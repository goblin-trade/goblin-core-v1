use crate::{
    goblin_error::GoblinError,
    input_processor::{ArgsBuffer, ArgsDecoder, Decodable},
    markets::{HardcodedMarketList, MarketHeader, MarketVariant, PairShape},
    quantities::{DeltaAtoms, QuoteLotsPerBaseUnitPerTick},
    require,
    settlement::{global::GlobalDelta, market::MarketDelta},
    state::{DynamicMarketHasher, DynamicMarketKey, HardcodedMarketKey, MarketState, SlotState},
    tokens::{CustomToken, DynamicIndex, HardcodedIndex, HardcodedToken, TokenIndex},
    types::{Base, LegMarker, Pair, Quote},
};

pub type LotSizePair = Pair<<Base as LegMarker>::LotsPerUnit, <Quote as LegMarker>::LotsPerUnit>;

pub struct CommonMarket<M: MarketVariant, P: PairShape> {
    /// The token pair, parameterized by shape and variant.
    pub token_index_pair: P::ResolvedPair<M>,

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
pub struct HardcodedMarket<P>
where
    P: PairShape,
{
    /// The common market configuration (lot sizes, tick size, token indices).
    pub common: CommonMarket<TokenIndex<HardcodedToken>, P>,

    /// The hardcoded keccak256 hash.
    pub keccak_hash: HardcodedMarketKey<P>,
}

/// A market whose token indices are dynamically specified at runtime.
/// Works with any token pair shape (ETH–ERC20, ERC20–ETH, ERC20–ERC20).
pub type DynamicMarket<P: PairShape> = CommonMarket<DynamicIndex, P>;

impl<P> HardcodedMarket<P>
where
    P: PairShape + 'static + Decodable<P::ResolvedPair<DeltaAtoms>>,
    P::ResolvedPair<DeltaAtoms>: Default,
    Self: HardcodedMarketList<P>,
{
    pub const DISCRIMINATOR: u8 = HardcodedIndex::DISCRIMINATOR | (P::DISCRIMINATOR << 1);

    pub fn process(
        market_header: &MarketHeader,
        global_delta: &mut GlobalDelta,
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<(), GoblinError> {
        let market = Self::decode(payload, offset, len)?;
        let market_state = MarketState::load(&market.keccak_hash);

        let market_delta =
            MarketDelta::<P>::new(market_header.decode_deposit_amounts, payload, offset, len)?;

        // Take bid and take quote
        if market_header.execute_takes.base {}

        // Apply market delta updates on global delta
        P::commit_delta(&market.common.token_index_pair, global_delta, &market_delta)?;

        Ok(())
    }
}

impl<P> DynamicMarket<P>
where
    P: PairShape
        + Decodable<P::ResolvedPair<DynamicIndex>>
        + Decodable<P::ResolvedPair<DeltaAtoms>>,
    P::ResolvedPair<DeltaAtoms>: Default,
    DynamicMarketKey<P>: DynamicMarketHasher<P>,
{
    pub const DISCRIMINATOR: u8 = DynamicIndex::DISCRIMINATOR | (P::DISCRIMINATOR << 1);

    pub fn process(
        market_header: &MarketHeader,
        global_delta: &mut GlobalDelta,
        custom_erc20_list: &[CustomToken],
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<(), GoblinError> {
        let market = Self::decode(payload, offset, len)?;
        let market_key = DynamicMarketKey::hash(&market, custom_erc20_list)?;
        let market_state = MarketState::load(&market_key);

        let market_delta =
            MarketDelta::<P>::new(market_header.decode_deposit_amounts, payload, offset, len)?;

        P::commit_delta(&market.token_index_pair, global_delta, &market_delta)?;

        Ok(())
    }
}

// Decoding traits

/// `Decodable` impl for HardcodedMarket<P>
impl<P> Decodable<&'static HardcodedMarket<P>> for HardcodedMarket<P>
where
    P: PairShape + 'static,
    Self: HardcodedMarketList<P>,
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

impl<P> Decodable<DynamicMarket<P>> for DynamicMarket<P>
where
    P: PairShape + Decodable<P::ResolvedPair<DynamicIndex>>,
{
    fn decode(
        payload: &ArgsBuffer,
        offset: &mut usize,
        len: usize,
    ) -> Result<DynamicMarket<P>, GoblinError> {
        let token_index_pair = P::decode(payload, offset, len)?;

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
