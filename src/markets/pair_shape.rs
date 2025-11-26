use crate::{
    goblin_error::GoblinError,
    markets::{CommonMarket, MarketVariant},
    matching::MatchResult,
    settlement::{
        global_delta::{CommonDelta, ERC20Delta, EthDelta, GlobalDelta, GlobalSenderDelta},
        local_delta::LocalDelta,
    },
    types::{Base, LegMarker, Pair, Quote},
};

/// Marker type for ETH within a token pair
pub struct ETH;

/// Marker type for ERC20 within a token pair
pub struct ERC20;

// pub trait TokenMarker {
//     type Delta;

//     fn get_delta_mut(global_sender_delta: &mut GlobalSenderDelta) -> &mut Self::Delta;
// }

// impl TokenMarker for ETH {
//     type Delta = EthDelta;

//     fn get_delta_mut(global_sender_delta: &mut GlobalSenderDelta) -> &mut Self::Delta {
//         &mut global_sender_delta.eth_delta
//     }
// }

// impl TokenMarker for ERC20 {
//     type Delta = ERC20Delta;
//     // This won't work. We need the token index.
// }

// pub type TokenPair<B: TokenMarker, Q: TokenMarker> = Pair<B, Q>;

// impl<B: TokenMarker, Q: TokenMarker> Pair<B, Q> {
//     fn delta_pair(&self) -> Pair<B::Delta, Q::Delta> {
//         todo!()
//     }
// }

// every PairShape is a Pair. Can we impose a requirement of Pair?
pub trait PairShape {
    const DISCRIMINATOR: u8;

    type ResolvedPair<K>;

    // fn common_delta<In: LegMarker>(global_delta: &mut GlobalDelta) -> &mut CommonDelta;

    /// Commit market namespaced delta into the global delta
    fn commit_delta<M>(
        common_market: &CommonMarket<M, Self>,
        global_delta: &mut GlobalDelta,
        local_delta: &LocalDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        M: MarketVariant + Clone + Copy,
        Self: Sized;
}

impl PairShape for Pair<ETH, ERC20> {
    const DISCRIMINATOR: u8 = 0;

    type ResolvedPair<K> = K;

    // 3 steps
    // 1. Update deposit amounts
    // 2. Update sender delta
    // 3. Update maker deltas
    fn commit_delta<M>(
        common_market: &CommonMarket<M, Self>,
        global_delta: &mut GlobalDelta,
        local_delta: &LocalDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        M: MarketVariant + Clone + Copy,
        Self: Sized,
    {
        let eth_base_delta = &mut global_delta.global_sender_delta.eth_delta;
        eth_base_delta.common_delta.apply_local_update::<Base>(
            &local_delta.local_sender_delta,
            &common_market.lot_size_pair,
        );

        let erc20_quote_delta = common_market
            .token_index_pair
            .token_delta_mut(&mut global_delta.global_sender_delta);

        erc20_quote_delta.apply_local_update::<Quote>(
            &local_delta.local_sender_delta,
            &common_market.lot_size_pair,
            local_delta.deposit_pair,
        );

        Ok(())
    }
}

impl PairShape for Pair<ERC20, ETH> {
    const DISCRIMINATOR: u8 = 1;

    type ResolvedPair<K> = K;

    fn commit_delta<M>(
        common_market: &CommonMarket<M, Self>,
        global_delta: &mut GlobalDelta,
        local_delta: &LocalDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        M: MarketVariant + Clone + Copy,
        Self: Sized,
    {
        let base_delta = common_market
            .token_index_pair
            .token_delta_mut(&mut global_delta.global_sender_delta);
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

    fn commit_delta<M>(
        common_market: &CommonMarket<M, Self>,
        global_delta: &mut GlobalDelta,
        local_delta: &LocalDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        M: MarketVariant + Clone + Copy,
        Self: Sized,
    {
        let base_delta = common_market
            .token_index_pair
            .base
            .token_delta_mut(&mut global_delta.global_sender_delta);
        base_delta
            .deposit(local_delta.deposit_pair.base)
            .ok_or(GoblinError::DepositOverflow)?;

        let quote_delta = common_market
            .token_index_pair
            .quote
            .token_delta_mut(&mut global_delta.global_sender_delta);
        quote_delta
            .deposit(local_delta.deposit_pair.quote)
            .ok_or(GoblinError::DepositOverflow)?;

        Ok(())
    }
}
