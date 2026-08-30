use crate::{
    axis::{
        market::{
            market_locator::hardcoded::HardcodedMarketList, market_marker::MarketMarker,
            MarketEnum, MarketLocator,
        },
        token::TokenEnum,
    },
    axis_helpers::{AxisMarker, TokenPair},
};

pub trait MarketSpec: Clone + Copy + PartialEq + PartialOrd {
    type Market: MarketMarker + MarketLocator<Self::Pair, Locator = Self::Locator>;
    type Pair: TokenPair + HardcodedMarketList;
    type Locator;

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
}
