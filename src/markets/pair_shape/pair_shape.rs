use crate::{
    goblin_error::GoblinError,
    markets::{CommonMarket, MarketVariant},
    settlement::{
        global_delta::{
            ERC20SenderDeltas, EthDelta, GlobalMakerUpdatePair, GlobalSenderUpdatePair,
        },
        Delta,
    },
    token::{TokenMarker, ERC20, ETH},
    types::{Base, Pair, Quote, TripleReader, TupleReader},
};

pub struct PairShapeV2<T0: TokenMarker, T1: TokenMarker>(Pair<T0, T1>);

impl<T0: TokenMarker, T1: TokenMarker> PairShapeV2<T0, T1> {
    const DISCRIMINATOR: u8 = T0::DISCRIMINATOR + T1::DISCRIMINATOR << 1;
    // Improvement- we get a common shape for all 3 forms
    // Individual complexity goes into TokenMarker
    //
    // Revisit old design- should we have an intermediary MarketForm<M: MarketVariant, P: PairShape>?
    // Currently we treat hardcoded and dynamic market separately
}

// every PairShape is a Pair. Can we impose a requirement of Pair?
pub trait PairShape {
    const DISCRIMINATOR: u8;

    type ResolvedPair<K: Clone + Copy>;

    // /// Commit local delta into the global delta
    // fn commit_local_delta<M>(
    //     common_market: &CommonMarket<M, Self>,
    //     delta: &mut Delta,
    // ) -> Result<(), GoblinError>
    // where
    //     M: MarketVariant + Clone + Copy,
    //     Self: Sized;

    // fn commit_local_delta_v2<M>(
    //     common_market: &CommonMarket<M, Self>,
    //     delta: &mut Delta,
    // ) -> Result<(), GoblinError>
    // where
    //     M: MarketVariant + Clone + Copy,
    //     Self: Sized + TupleReader<EthDelta, ERC20SenderDeltas, Self, Result = M>,
    // {
    //     let sender_update_pair = GlobalSenderUpdatePair::new_pair(
    //         &delta.local.local_sender_delta.taker_delta_pair,
    //         &common_market.lot_size_pair,
    //     );
    //     let base_update = Base::get_leg(&sender_update_pair);
    //     let quote_update = Quote::get_leg(&sender_update_pair);

    //     // Apply the base update
    //     // It can be appled on ETH or ERC20, depending on token shape
    //     //
    //     // - ETH: simple
    //     // - ERC20: use common_market.token_index_pair to lookup delta from ERC20 list
    //     //
    //     //
    //     // MarketVariant::token_sender_delta_mut() will handle sub cases for hardcoded and dynamic market variants

    //     // TODO we need Self.0 = ETH and Self.1 = ERC20
    //     // Then map individual units to ETHDelta and ERC20Delta
    //     //
    //     // Options
    //     // 1. Reintroduce Pair<ETH, ERC20> and introduce a bound PairShape: Pair<T0: Token, T1: Token>
    //     // This allows us to access individual element as .base and .quote
    //     //
    //     // 2. Implement a tuple of delta types for each variant
    //     //
    //     // let sender_base_delta = Self::get_leg_mut(&mut delta.global.global_sender_delta);

    //     Ok(())
    // }
}

impl PairShape for (ETH, ERC20) {
    const DISCRIMINATOR: u8 = 0;

    type ResolvedPair<K: Clone + Copy> = K;

    // // TODO common function for all 3 shapes
    // fn commit_local_delta<M>(
    //     common_market: &CommonMarket<M, Self>,
    //     delta: &mut Delta,
    // ) -> Result<(), GoblinError>
    // where
    //     M: MarketVariant + Clone + Copy,
    //     Self: Sized,
    // {
    //     let sender_update_pair = GlobalSenderUpdatePair::new_pair(
    //         &delta.local.local_sender_delta.taker_delta_pair,
    //         &common_market.lot_size_pair,
    //     );

    //     // Update base
    //     let sender_delta = &mut delta.global.global_sender_delta;
    //     let base_update = Base::get_leg(&sender_update_pair);

    //     ETH::get_leg_mut(sender_delta)
    //         .unsided_sender_delta
    //         .add_global_update::<Base>(base_update)
    //         .ok_or(GoblinError::DeltaOverflow)?;

    //     // Update quote
    //     let quote_token_index = common_market.token_index_pair;
    //     let quote_delta =
    //         quote_token_index.token_sender_delta_mut(ERC20::get_leg_mut(sender_delta));

    //     let deposit_pair = Self::get_leg_mut(&mut delta.local.deposits);
    //     let quote_update = Quote::get_leg(&sender_update_pair);
    //     quote_delta.apply_global_update::<Quote>(*deposit_pair, quote_update)?;

    //     for (maker, maker_delta_pair) in delta.local.local_maker_deltas.iter() {
    //         let maker_update_pair =
    //             GlobalMakerUpdatePair::new_pair(maker_delta_pair, &common_market.lot_size_pair);

    //         // Update base (ETH)
    //         let global_maker_delta_eth_base = ETH::get_leg_mut(&mut delta.global.maker_deltas)
    //             .get_or_insert_mut(*maker)
    //             .ok_or(GoblinError::MakerListFull)?;

    //         global_maker_delta_eth_base
    //             .add_global_update::<Base>(&maker_update_pair.0)
    //             .ok_or(GoblinError::DeltaOverflow)?;

    //         // Update quote (ERC20)
    //         let global_maker_delta_erc20_quote = quote_token_index
    //             .token_maker_delta_mut(*maker, ERC20::get_leg_mut(&mut delta.global.maker_deltas))
    //             .ok_or(GoblinError::MakerListFull)?;

    //         global_maker_delta_erc20_quote
    //             .add_global_update(&maker_update_pair.1)
    //             .ok_or(GoblinError::DeltaOverflow)?;
    //     }

    //     Ok(())
    // }
}

impl PairShape for (ERC20, ETH) {
    const DISCRIMINATOR: u8 = 1;

    type ResolvedPair<K: Clone + Copy> = K;

    // fn commit_local_delta<M>(
    //     common_market: &CommonMarket<M, Self>,
    //     delta: &mut Delta,
    // ) -> Result<(), GoblinError>
    // where
    //     M: MarketVariant + Clone + Copy,
    //     Self: Sized,
    // {
    //     // let base_delta = common_market
    //     //     .token_index_pair
    //     //     .token_delta_mut(&mut global_delta.global_sender_delta);
    //     // base_delta
    //     //     .deposit(local_delta.deposit_pair)
    //     //     .ok_or(GoblinError::DepositOverflow)?;

    //     Ok(())
    // }
}

/// A pair of two ERC20 tokens.
/// The decoder guarantees that the two tokens are different.
impl PairShape for (ERC20, ERC20) {
    const DISCRIMINATOR: u8 = 2;

    type ResolvedPair<K: Clone + Copy> = Pair<K, K>;

    // fn commit_local_delta<M>(
    //     common_market: &CommonMarket<M, Self>,
    //     delta: &mut Delta,
    // ) -> Result<(), GoblinError>
    // where
    //     M: MarketVariant + Clone + Copy,
    //     Self: Sized,
    // {
    //     // let base_delta = common_market
    //     //     .token_index_pair
    //     //     .base
    //     //     .token_delta_mut(&mut global_delta.global_sender_delta);
    //     // base_delta
    //     //     .deposit(local_delta.deposit_pair.base)
    //     //     .ok_or(GoblinError::DepositOverflow)?;

    //     // let quote_delta = common_market
    //     //     .token_index_pair
    //     //     .quote
    //     //     .token_delta_mut(&mut global_delta.global_sender_delta);
    //     // quote_delta
    //     //     .deposit(local_delta.deposit_pair.quote)
    //     //     .ok_or(GoblinError::DepositOverflow)?;

    //     Ok(())
    // }
}
