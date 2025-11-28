use crate::{
    goblin_error::GoblinError,
    markets::{CommonMarket, MarketVariant},
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

    fn commit_delta<M>(
        common_market: &CommonMarket<M, Self>,
        global_delta: &mut GlobalDelta,
        local_delta: &LocalDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        M: MarketVariant + Clone + Copy,
        Self: Sized,
    {
        let sender_delta = &mut global_delta.global_sender_delta;

        sender_delta
            .eth_delta
            .common_delta
            .apply_local_update::<Base>(
                &local_delta.local_sender_delta,
                &common_market.lot_size_pair,
            );

        let quote_token_index = common_market.token_index_pair;

        let lazy_delta = quote_token_index.token_delta_mut(&mut sender_delta.token_deltas);
        lazy_delta.apply_local_update::<Quote>(
            &common_market.lot_size_pair,
            &local_delta.local_sender_delta,
            local_delta.deposit_pair,
        );

        // TODO maker deltas

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
        // let base_delta = common_market
        //     .token_index_pair
        //     .token_delta_mut(&mut global_delta.global_sender_delta);
        // base_delta
        //     .deposit(local_delta.deposit_pair)
        //     .ok_or(GoblinError::DepositOverflow)?;

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
        // let base_delta = common_market
        //     .token_index_pair
        //     .base
        //     .token_delta_mut(&mut global_delta.global_sender_delta);
        // base_delta
        //     .deposit(local_delta.deposit_pair.base)
        //     .ok_or(GoblinError::DepositOverflow)?;

        // let quote_delta = common_market
        //     .token_index_pair
        //     .quote
        //     .token_delta_mut(&mut global_delta.global_sender_delta);
        // quote_delta
        //     .deposit(local_delta.deposit_pair.quote)
        //     .ok_or(GoblinError::DepositOverflow)?;

        Ok(())
    }
}
