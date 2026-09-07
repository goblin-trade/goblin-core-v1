use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        token::{token_marker::TokenMarker, token_quantity::TokenQuantity},
    },
    axis_helpers::{LegToToken, TokenPair},
    market::TokenIndexPair,
    settlement::local_delta::LocalDeposits,
    types::StoreReader,
};

pub trait PairLeg: Clone + Copy {
    type Pair: TokenPair;
    type Selected: TokenMarker;
    type Leg: LegMatcher
        + LegToToken<Self::Pair, Selected = Self::Selected>
        + StoreReader<
            TokenIndexPair<Self::Pair>,
            Result = <Self::Selected as TokenQuantity>::TokenIndex,
        > + StoreReader<
            LocalDeposits<Self::Pair>,
            Result = <Self::Selected as TokenQuantity>::LocalDeposit,
        >;
}

impl<TP, In> PairLeg for (TP, In)
where
    TP: TokenPair,
    In: LegMatcher
        + LegToToken<TP>
        + StoreReader<TokenIndexPair<TP>, Result = <In::Selected as TokenQuantity>::TokenIndex>
        + StoreReader<LocalDeposits<TP>, Result = <In::Selected as TokenQuantity>::LocalDeposit>,
{
    type Pair = TP;
    type Selected = In::Selected;
    type Leg = In;
}
