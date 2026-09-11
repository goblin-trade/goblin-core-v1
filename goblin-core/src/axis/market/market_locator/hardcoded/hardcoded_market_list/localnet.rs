use super::HardcodedMarketList;
use crate::{
    axis::{
        leg::Pair,
        token::{ETH, ETHStub, HardcodedERC20, token_marker::HardcodedERC20Index},
    },
    market::{CommonMarket, MarketReadables},
    quantities::{BaseLotsPerBaseUnit, QuoteLotsPerBaseUnitPerTick, QuoteLotsPerQuoteUnit},
    state::SlotKey,
    types::Tuple,
};

impl HardcodedMarketList for Pair<ETH, HardcodedERC20> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<Pair<ETH, HardcodedERC20>>] =
        &[MarketReadables::get_hardcoded(CommonMarket::new(
            Pair::new(ETHStub, HardcodedERC20Index::new(0)),
            Tuple::new(
                BaseLotsPerBaseUnit::new(100),
                QuoteLotsPerQuoteUnit::new(100),
            ),
            QuoteLotsPerBaseUnitPerTick::new(1),
        ))];

    // const HARDCODED_MARKET_LIST: &'static [MarketReadables<Pair<ETH, HardcodedERC20>>] =
    //     &[MarketReadables {
    //         market: CommonMarket::new(
    //             Pair::new(ETHStub, HardcodedERC20Index::new(0)),
    //             Tuple::new(
    //                 BaseLotsPerBaseUnit::new(100),
    //                 QuoteLotsPerQuoteUnit::new(100),
    //             ),
    //             QuoteLotsPerBaseUnitPerTick::new(1),
    //         ),
    //         // TODO generate market_key based on params
    //         market_key: SlotKey::new([0u8; 32]),
    //     }];
}

impl HardcodedMarketList for Pair<HardcodedERC20, ETH> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<Pair<HardcodedERC20, ETH>>] =
        &[MarketReadables {
            market: CommonMarket::new(
                Pair::new(HardcodedERC20Index::new(1), ETHStub),
                Tuple::new(
                    BaseLotsPerBaseUnit::new(100),
                    QuoteLotsPerQuoteUnit::new(100),
                ),
                QuoteLotsPerBaseUnitPerTick::new(1),
            ),
            market_key: SlotKey::new([0u8; 32]),
        }];
}

impl HardcodedMarketList for Pair<HardcodedERC20, HardcodedERC20> {
    const HARDCODED_MARKET_LIST: &'static [MarketReadables<
        Pair<HardcodedERC20, HardcodedERC20>,
    >] = &[MarketReadables {
        market: CommonMarket::new(
            Pair::new(HardcodedERC20Index::new(0), HardcodedERC20Index::new(1)),
            Tuple::new(
                BaseLotsPerBaseUnit::new(100),
                QuoteLotsPerQuoteUnit::new(100),
            ),
            QuoteLotsPerBaseUnitPerTick::new(1),
        ),
        market_key: SlotKey::new([0u8; 32]),
    }];
}
