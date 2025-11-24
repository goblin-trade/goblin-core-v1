use crate::{
    goblin_error::GoblinError,
    markets::MarketVariant,
    settlement::{global_delta::GlobalDelta, market_delta::MarketDelta},
    types::Pair,
};

/// Marker type for ETH within a token pair
pub struct ETH;

/// Marker type for ERC20 within a token pair
pub struct ERC20;

pub trait PairShape {
    const DISCRIMINATOR: u8;

    type ResolvedPair<K>;

    /// Commit market namespaced delta into the global delta
    fn commit_delta<M: MarketVariant + Clone + Copy>(
        index_pair: &Self::ResolvedPair<M>,
        global_delta: &mut GlobalDelta,
        market_delta: &MarketDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        Self: Sized;
}

impl PairShape for Pair<ETH, ERC20> {
    const DISCRIMINATOR: u8 = 0;

    type ResolvedPair<K> = K;

    fn commit_delta<M: MarketVariant + Clone + Copy>(
        index_pair: &Self::ResolvedPair<M>,
        global_delta: &mut GlobalDelta,
        market_delta: &MarketDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        Self: Sized,
    {
        // 3 steps
        // 1. Update deposit amounts
        // 2. Update sender delta
        // 3. Update maker deltas

        // Deposit amounts
        let quote_delta = index_pair.token_delta_mut(global_delta);
        quote_delta
            .deposit(market_delta.deposit_pair)
            .ok_or(GoblinError::DepositOverflow)?;

        // Sender delta
        // It has base and quote sides
        // We need to apply it into ETH or ERC20 as per the pair shape

        let base_sender_delta = &market_delta.sender_delta.base; // TODO apply on ETH
        let quote_sender_delta = &market_delta.sender_delta.quote; // TODO apply on ERC20

        Ok(())
    }
}

impl PairShape for Pair<ERC20, ETH> {
    const DISCRIMINATOR: u8 = 1;

    type ResolvedPair<K> = K;

    fn commit_delta<M: MarketVariant + Clone + Copy>(
        index_pair: &Self::ResolvedPair<M>,
        global_delta: &mut GlobalDelta,
        market_delta: &MarketDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        Self: Sized,
    {
        let base_delta = index_pair.token_delta_mut(global_delta);
        base_delta
            .deposit(market_delta.deposit_pair)
            .ok_or(GoblinError::DepositOverflow)?;

        Ok(())
    }
}

/// A pair of two ERC20 tokens.
/// The decoder guarantees that the two tokens are different.
impl PairShape for Pair<ERC20, ERC20> {
    const DISCRIMINATOR: u8 = 2;

    type ResolvedPair<K> = Pair<K, K>;

    fn commit_delta<M: MarketVariant + Clone + Copy>(
        index_pair: &Self::ResolvedPair<M>,
        global_delta: &mut GlobalDelta,
        market_delta: &MarketDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        Self: Sized,
    {
        let base_delta = index_pair.base.token_delta_mut(global_delta);
        base_delta
            .deposit(market_delta.deposit_pair.base)
            .ok_or(GoblinError::DepositOverflow)?;

        let quote_delta = index_pair.quote.token_delta_mut(global_delta);
        quote_delta
            .deposit(market_delta.deposit_pair.quote)
            .ok_or(GoblinError::DepositOverflow)?;

        Ok(())
    }
}
