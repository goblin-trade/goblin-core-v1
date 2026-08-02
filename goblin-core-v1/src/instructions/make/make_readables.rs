use crate::{
    axis::market::{market_spec::MarketSpec, Readables},
    instructions::PosHeader,
};

pub struct MakeReadables<'a, MS: MarketSpec> {
    pub readables: &'a Readables<'a, MS>,
    pub pos_header: PosHeader,
}
