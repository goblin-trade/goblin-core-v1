use crate::{axis_helpers::MarketSpec, instructions::PosHeader, market::Readables};

pub struct MakeReadables<'a, MS: MarketSpec> {
    pub readables: &'a Readables<'a, MS>,
    pub pos_header: PosHeader,
}
