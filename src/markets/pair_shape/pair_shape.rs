use crate::{
    goblin_error::GoblinError,
    markets::{CommonMarket, MarketVariant},
    settlement::{
        global_delta::{GlobalMakerUpdatePair, GlobalSenderUpdatePair, TokenMakerDeltaKey},
        local_delta::LocalDeposits,
        Delta,
    },
    types::{Base, Pair, Quote},
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
        let sender_update_pair = GlobalSenderUpdatePair::new(
            &delta.local.local_sender_delta.taker_delta_pair,
            &common_market.lot_size_pair,
        );

        // Update base
        let sender_delta = &mut delta.global.global_sender_delta;
        sender_delta
            .eth_delta
            .unsided_sender_delta
            .add_global_update::<Base>(&sender_update_pair.base)
            .ok_or(GoblinError::DeltaOverflow)?;

        // Update quote
        let quote_token_index = common_market.token_index_pair;
        let quote_delta = quote_token_index.token_sender_delta_mut(&mut sender_delta.token_deltas);

        let deposit_pair = LocalDeposits::<Self>::deposit_mut(&mut delta.local.deposits);
        quote_delta.apply_global_update::<Quote>(*deposit_pair, &sender_update_pair.quote)?;

        for (maker, maker_delta_pair) in delta.local.local_maker_deltas.iter() {
            let maker_update_pair =
                GlobalMakerUpdatePair::new(maker_delta_pair, &common_market.lot_size_pair);

            // Update base (ETH)
            let global_maker_delta_eth_base = delta
                .global
                .maker_deltas
                .eth_deltas
                .get_or_insert_mut(*maker)
                .ok_or(GoblinError::MakerListFull)?;

            global_maker_delta_eth_base
                .add_global_update::<Base>(&maker_update_pair.base)
                .ok_or(GoblinError::DeltaOverflow)?;

            // Update quote (ERC20)
            let global_maker_delta_erc20_quote = quote_token_index
                .token_maker_delta_mut(*maker, &mut delta.global.maker_deltas.token_deltas)
                .ok_or(GoblinError::MakerListFull)?;

            global_maker_delta_erc20_quote
                .add_global_update(&maker_update_pair.quote)
                .ok_or(GoblinError::DeltaOverflow)?;
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
