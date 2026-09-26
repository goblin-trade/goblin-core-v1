use core::borrow::Borrow;

use crate::{
    axis::{
        market::{
            MarketEnum, MarketLocator, market_locator::hardcoded::HardcodedMarketList,
            market_marker::MarketMarker,
        },
        token::TokenEnum,
    },
    axis_helpers::{AxisMarker, TokenPair},
    input_processor::CodecBounds,
    market::MarketReadables,
};

pub trait MarketSpec: Clone + Copy + PartialEq + PartialOrd {
    type Market: MarketMarker
        + MarketLocator<Self::Pair, Locator = Self::Locator, Readables = Self::Readables>;
    type Pair: TokenPair + HardcodedMarketList;
    type Locator: CodecBounds;
    type Readables: Borrow<MarketReadables<Self::Pair>>;

    /// Whether the generic combination is illegal
    ///
    /// # Illegal
    ///
    /// 1. Hardcoded market with custom tokens
    /// 2. Both token variants are ETH
    ///
    /// Total combinations = 2 * 3 * 3 = 18
    ///
    /// Total legal combinations = 11
    ///
    fn illegal() -> bool {
        (Self::Market::VARIANT == MarketEnum::Hardcoded
            && (<Self::Pair as TokenPair>::Base::VARIANT == TokenEnum::CustomERC20
                || <Self::Pair as TokenPair>::Quote::VARIANT == TokenEnum::CustomERC20))
            || (<Self::Pair as TokenPair>::Base::VARIANT == TokenEnum::ETH
                && <Self::Pair as TokenPair>::Quote::VARIANT == TokenEnum::ETH)
    }
}

impl<M, TP> MarketSpec for (M, TP)
where
    M: MarketMarker + MarketLocator<TP>,
    TP: TokenPair + HardcodedMarketList,
{
    type Market = M;
    type Pair = TP;
    type Locator = <M as MarketLocator<TP>>::Locator;
    type Readables = <M as MarketLocator<TP>>::Readables;
}
