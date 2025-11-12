use crate::markets::{HardcodedMarket, PairShape};

/// Map each PairShape to a hardcoded market list
pub trait HardcodedMarketList<P: PairShape + 'static> {
    const HARDCODED_MARKET_LIST: &'static [HardcodedMarket<P>];
}
