use crate::{
    axis::market::{market_spec::MarketSpec, MarketReadables},
    types::Address,
};

pub struct Readables<'a, MS: MarketSpec> {
    pub msg_sender: &'a Address,
    pub market_readables: &'a MarketReadables<MS>,
}
