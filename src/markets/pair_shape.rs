use crate::{
    goblin_error::GoblinError,
    markets::{CommonMarket, MarketVariant},
    settlement::{global_delta::GlobalUpdatePair, local_delta::LocalDeposits, Delta},
    types::Pair,
};

/// Marker type for ETH within a token pair
pub struct ETH;

/// Marker type for ERC20 within a token pair
pub struct ERC20;

// every PairShape is a Pair. Can we impose a requirement of Pair?
pub trait PairShape {
    const DISCRIMINATOR: u8;

    type ResolvedPair<K>;

    /// Commit local delta into the global delta
    fn commit_local_delta<M>(
        common_market: &CommonMarket<M, Self>,
        delta: &mut Delta,
    ) -> Result<(), GoblinError>
    where
        M: MarketVariant + Clone + Copy,
        Self: Sized;
}

impl PairShape for Pair<ETH, ERC20> {
    const DISCRIMINATOR: u8 = 0;

    type ResolvedPair<K> = K;

    fn commit_local_delta<M>(
        common_market: &CommonMarket<M, Self>,
        delta: &mut Delta,
    ) -> Result<(), GoblinError>
    where
        M: MarketVariant + Clone + Copy,
        Self: Sized,
    {
        let global_update_pair = GlobalUpdatePair::new(
            &delta.local.local_sender_delta.taker_delta_pair,
            &common_market.lot_size_pair,
        );

        // Update base
        let sender_delta = &mut delta.global.global_sender_delta;
        sender_delta
            .eth_delta
            .common_delta
            .add_global_update(&global_update_pair.base)
            .ok_or(GoblinError::DeltaOverflow)?;

        // Update quote
        let quote_token_index = common_market.token_index_pair;
        let quote_delta = quote_token_index.token_delta_mut(&mut sender_delta.token_deltas);

        let deposit_pair = LocalDeposits::<Self>::deposit_mut(&mut delta.local.deposits);
        quote_delta.apply_global_update(*deposit_pair, &global_update_pair.quote)?;

        // TODO maker deltas
        // Base is ETH. The makers of In: Quote will populate ETH delta
        let global_maker_deltas_eth = &mut delta.global.maker_deltas.eth_deltas;

        for local_maker_delta in delta.local.local_maker_deltas.iter() {
            // Each element has a base and quote branch
            // One of these can be empty- we need to add if-else now
            //
            // Better to have separate lists for base and quote?
            // if local_maker_delta.1.base.
        }

        Ok(())
    }
}

impl PairShape for Pair<ERC20, ETH> {
    const DISCRIMINATOR: u8 = 1;

    type ResolvedPair<K> = K;

    fn commit_local_delta<M>(
        common_market: &CommonMarket<M, Self>,
        delta: &mut Delta,
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

    fn commit_local_delta<M>(
        common_market: &CommonMarket<M, Self>,
        delta: &mut Delta,
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
