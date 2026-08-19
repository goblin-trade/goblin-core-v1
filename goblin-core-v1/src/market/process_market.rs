use crate::{
    axis::token::token_reader::TokenDataTriple,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    market::{MarketHeader, Readables, Writables},
    settlement::{local_delta::LocalDelta, StaticDelta},
    types::Address,
};

#[inline(never)]
pub fn process_market<'a, MS>(
    msg_sender: &Address,
    ctx: &DecodeCtx,
    token_data_triple: &TokenDataTriple<'a>,
    static_delta: &mut StaticDelta,
) -> Result<(), GoblinError>
where
    MS: MarketSpec,
{
    let local_delta = &mut LocalDelta::new(&mut static_delta.take_counterparties);
    let market_header = MarketHeader::<MS>::try_fixed_decode(ctx)?;

    if market_header.decode_deposit_amounts {
        local_delta.deposits.decode_and_set::<MS::Pair>(ctx)?;
    }
    let readables = &Readables::new(msg_sender, ctx, token_data_triple)?;

    let market_state = &mut readables.market_readables().market_key.load();
    let writables = &mut Writables {
        local_delta,
        market_state,
    };

    // TODO convert to axis- make and take?
    market_header.execute_takes(ctx, readables, writables)?;
    market_header.execute_makes(ctx, readables, writables)?;

    static_delta
        .global
        .commit_local_delta::<MS>(readables, writables)
}
