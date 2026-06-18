use crate::{
    axis::{
        leg::leg_matcher::LegMatcher,
        market::{market_marker::MarketMarker, Readables, Writables},
        token::token_marker::TokenMarker,
    },
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    instructions::take::take_header::TakeHeader,
    matching::match_order,
};

pub fn ix_take<M, B, Q, In>(
    ctx: &DecodeCtx,
    readables: &Readables<M, B, Q>,
    writables: &mut Writables,
) -> Result<(), GoblinError>
where
    M: MarketMarker,
    B: TokenMarker,
    Q: TokenMarker,
    In: LegMatcher,
{
    let header = TakeHeader::<In>::try_decode(ctx)?;
    match_order::<M, B, Q, In>(header, readables, writables)
}
