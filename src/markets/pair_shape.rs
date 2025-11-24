use crate::{
    goblin_error::GoblinError,
    markets::MarketVariant,
    matching::MatchResult,
    settlement::{global_delta::GlobalDelta, local_delta::LocalDelta},
    types::{Base, Pair},
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
        // TODO use common_market instead of index_pair?
        // If we need lot size pair and tick size then yes
        index_pair: &Self::ResolvedPair<M>,
        global_delta: &mut GlobalDelta,
        local_delta: &LocalDelta<Self>,
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
        local_delta: &LocalDelta<Self>,
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
            .deposit(local_delta.deposit_pair)
            .ok_or(GoblinError::DepositOverflow)?;

        // Sender delta
        // It has base and quote sides
        // We need to apply it into ETH or ERC20 as per the pair shape

        let base_take_result = &local_delta.sender_delta.take_result_pair.base;
        let quote_take_result = &local_delta.sender_delta.take_result_pair.quote;

        if *base_take_result != MatchResult::<Base>::default() {
            // ETH goes in, ERC20 comes out
            // This result applies on both ETH and ERC20
        }
        Ok(())
    }
}

impl PairShape for Pair<ERC20, ETH> {
    const DISCRIMINATOR: u8 = 1;

    type ResolvedPair<K> = K;

    fn commit_delta<M: MarketVariant + Clone + Copy>(
        index_pair: &Self::ResolvedPair<M>,
        global_delta: &mut GlobalDelta,
        local_delta: &LocalDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        Self: Sized,
    {
        let base_delta = index_pair.token_delta_mut(global_delta);
        base_delta
            .deposit(local_delta.deposit_pair)
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
        local_delta: &LocalDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        Self: Sized,
    {
        let base_delta = index_pair.base.token_delta_mut(global_delta);
        base_delta
            .deposit(local_delta.deposit_pair.base)
            .ok_or(GoblinError::DepositOverflow)?;

        let quote_delta = index_pair.quote.token_delta_mut(global_delta);
        quote_delta
            .deposit(local_delta.deposit_pair.quote)
            .ok_or(GoblinError::DepositOverflow)?;

        Ok(())
    }
}
