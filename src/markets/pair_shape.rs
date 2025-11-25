use crate::{
    goblin_error::GoblinError,
    markets::{CommonMarket, MarketVariant},
    matching::MatchResult,
    settlement::{
        global_delta::{CommonDelta, GlobalDelta},
        local_delta::{LocalDelta, SenderGlobalUpdate},
    },
    types::{Base, LegMarker, Pair},
};

/// Marker type for ETH within a token pair
pub struct ETH;

/// Marker type for ERC20 within a token pair
pub struct ERC20;

pub trait PairShape {
    const DISCRIMINATOR: u8;

    type ResolvedPair<K>;

    fn common_delta<In: LegMarker>(global_delta: &mut GlobalDelta) -> &mut CommonDelta;

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

    fn common_delta<In: LegMarker>(global_delta: &mut GlobalDelta) -> &mut CommonDelta {
        // ETH if base, ERC20 if quote
        // This has 2 branches. Move it into LegMarker
    }

    fn commit_delta<M>(
        common_market: &CommonMarket<M, Self>,
        global_delta: &mut GlobalDelta,
        local_delta: &LocalDelta<Self>,
    ) -> Result<(), GoblinError>
    where
        M: MarketVariant + Clone + Copy,
        Self: Sized,
    {
        // 3 steps
        // 1. Update deposit amounts
        // 2. Update sender delta
        // 3. Update maker deltas

        // Deposit amounts
        let quote_delta = common_market
            .token_index_pair
            .token_delta_mut(&mut global_delta.global_sender_delta);
        quote_delta
            .deposit(local_delta.deposit_pair)
            .ok_or(GoblinError::DepositOverflow)?;

        // Sender delta
        // It has base and quote sides
        // We need to apply it into ETH or ERC20 as per the pair shape

        // MatchingLots -> Lots -> Atoms -> UnsidedAtoms
        let sender_global_update_base = SenderGlobalUpdate::<Base>::new(
            &local_delta.local_sender_delta,
            common_market.lot_size_pair,
        );

        // Apply to base side ETH
        // PairShape should have function on GobalDelta to return CommonDelta for the given side

        let common_delta = Self::common_delta::<Base>(global_delta);

        // We have a pending update ready to be applied.
        // It can go in 3 places- ETH, hardcoded list or custom list
        //
        // 1. Branch 1 checks In: LegMarker and PairShape: ETH or ERC20
        // TODO
        //
        // 2. Branch 2 if it was ERC20. This is a dynamic branch. Match on DynamicIndex for hardcoded and custom cases
        // MarketVariant::token_delta_mut() already does this
        common_delta.taker_self_trade_unlocked +=
            sender_global_update_base.atoms_released_by_self_trade;

        // let base_take_result = &local_delta.sender_delta.take_result_pair.base;
        // let quote_take_result = &local_delta.sender_delta.take_result_pair.quote;

        // if *base_take_result != MatchResult::<Base>::default() {
        //     // ETH goes in, ERC20 comes out
        //     // This result applies on both ETH and ERC20

        //     // The value is in MatchingLots. We need to convert it.
        //     // MatchingLots -> Lots -> Atoms -> UnsidedAtoms
        //     // Markets have different lot sizes. Therefore we cannot accumulate Lots at the top level, we
        //     // must convert to atoms.
        //     //
        //     // Reference function- global_delta.apply_side_update<In>
        //     let free_matching_lots_in = base_take_result.free_matching_lots_in;
        // }

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
