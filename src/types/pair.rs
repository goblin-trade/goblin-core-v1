use crate::types::{Base, Quote};

/// A generic container for a pair of items for the base and quote sides of a market
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Pair<B, Q> {
    pub base: B,
    pub quote: Q,
}

/// Trait to generically access one leg of the pair given the generic <L: LegMarker>
pub trait PairAccessor<B, Q> {
    type Result;

    fn get_leg(pair: &Pair<B, Q>) -> &Self::Result;
    fn get_leg_mut(pair: &mut Pair<B, Q>) -> &mut Self::Result;
}

impl<B, Q> PairAccessor<B, Q> for Base {
    type Result = B;

    fn get_leg(pair: &Pair<B, Q>) -> &Self::Result {
        &pair.base
    }

    fn get_leg_mut(pair: &mut Pair<B, Q>) -> &mut Self::Result {
        &mut pair.base
    }
}

impl<B, Q> PairAccessor<B, Q> for Quote {
    type Result = Q;

    fn get_leg(pair: &Pair<B, Q>) -> &Self::Result {
        &pair.quote
    }

    fn get_leg_mut(pair: &mut Pair<B, Q>) -> &mut Self::Result {
        &mut pair.quote
    }
}

// pub type LotSizePairV2 = Pair<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit>;
// pub type MarketLegs = Pair<MarketLeg<Base>, MarketLeg<Quote>>;
// pub type TokenIndexPair = Pair<TokenIndex, TokenIndex>;

// fn use_token_index<In>(legs: &TokenIndexPair)
// where
//     In: LegMarker + PairAccessor<TokenIndex, TokenIndex, Result = TokenIndex>,
// {
//     let index = In::get_leg(legs);
// }

// fn use_lot_size<In>(legs: &LotSizePairV2)
// where
//     In: LegMarker + PairAccessor<BaseLotsPerBaseUnit, QuoteLotsPerQuoteUnit, Result = In::LotsPerUnit>,
// {
//     let lot_size = In::get_leg(legs);
//     assert!(In::lots_per_unit_valid(*lot_size));
// }

// fn get_market_leg<In>(legs: &MarketLegs)
// where
//     In: LegMarker + PairAccessor<MarketLeg<Base>, MarketLeg<Quote>, Result = MarketLeg<In>>,
// {
//     let gg = In::get_leg(legs);
// }
