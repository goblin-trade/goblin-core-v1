use crate::{axis::market::Readables, axis_helpers::MarketSpec, instructions::PosHeader};

pub struct MakeReadables<'a, MS: MarketSpec> {
    pub readables: &'a Readables<'a, MS>,
    pub pos_header: PosHeader,
}
