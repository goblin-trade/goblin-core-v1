use super::HardcodedMarketList;
use crate::{
    axis::{
        leg::Pair,
        token::{ETH, ETHStub, HardcodedERC20, token_marker::HardcodedERC20Index},
    },
    market::{CommonMarket, MarketReadables},
    quantities::{
        BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit, U32Variant,
    },
    types::Tuple,
};

impl HardcodedMarketList for Pair<ETH, HardcodedERC20> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<Pair<ETH, HardcodedERC20>>] =
        &[MarketReadables::get_const(CommonMarket::new(
            Pair::new(ETHStub, HardcodedERC20Index::new(0)),
            Tuple::new(
                U32Variant::<BaseLotsPerBaseUnit>::new(100),
                U32Variant::<QuoteLotsPerQuoteUnit>::new(100),
            ),
            QuoteLotsPerBaseUnitPerTick::<u32>::new(1),
        ))];
}

impl HardcodedMarketList for Pair<HardcodedERC20, ETH> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<Pair<HardcodedERC20, ETH>>] =
        &[MarketReadables::get_const(CommonMarket::new(
            Pair::new(HardcodedERC20Index::new(1), ETHStub),
            Tuple::new(
                U32Variant::<BaseLotsPerBaseUnit>::new(100),
                U32Variant::<QuoteLotsPerQuoteUnit>::new(100),
            ),
            QuoteLotsPerBaseUnitPerTick::<u32>::new(1),
        ))];
}

impl HardcodedMarketList for Pair<HardcodedERC20, HardcodedERC20> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<
        Pair<HardcodedERC20, HardcodedERC20>,
    >] = &[MarketReadables::get_const(CommonMarket::new(
        Pair::new(HardcodedERC20Index::new(0), HardcodedERC20Index::new(1)),
        Tuple::new(
            U32Variant::<BaseLotsPerBaseUnit>::new(100),
            U32Variant::<QuoteLotsPerQuoteUnit>::new(100),
        ),
        QuoteLotsPerBaseUnitPerTick::<u32>::new(1),
    ))];
}
