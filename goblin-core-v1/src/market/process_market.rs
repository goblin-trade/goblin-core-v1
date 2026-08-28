use crate::{
    axis::{party::Counterparties, token::token_reader::TokenDataTriple},
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, FixedDecode},
    market::MarketHeader,
    settlement::{
        local_delta::{LocalDeposits, LocalUpdateV2},
        StaticDelta,
    },
    types::{Address, StoreReader},
    Ctx,
};

#[inline(never)]
pub fn process_market<'a, MS>(
    msg_sender: &Address,
    reader: &ArgsReader,
    token_data_triple: &TokenDataTriple<'a>,
    static_delta: &mut StaticDelta,
) -> Result<(), GoblinError>
where
    MS: MarketSpec,
{
    let market_header = MarketHeader::<MS>::try_fixed_decode(reader)?;

    let local_deposits = if market_header.decode_deposit_amounts {
        LocalDeposits::<MS::Pair>::try_fixed_decode(reader)?
    } else {
        LocalDeposits::<MS::Pair>::default()
    };

    let ctx = &mut Ctx::try_new(
        msg_sender,
        reader,
        token_data_triple,
        &mut static_delta.local_counterparties,
    )?;

    // TODO convert to axis- make and take?
    market_header.execute_takes(reader, ctx)?;
    market_header.execute_makes(reader, ctx)?;

    let local_update_v2 =
        LocalUpdateV2::<MS::Pair>::from((&ctx.writables.local_delta, local_deposits));

    static_delta
        .global
        .commit::<MS>(&ctx.readables.market_readables().market, &local_update_v2)?;

    // Reset counter of global mut counterparty buffer
    Counterparties::get_leg_mut(&mut ctx.writables.local_delta).reset();

    Ok(())
}
