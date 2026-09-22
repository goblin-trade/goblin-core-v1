use deku::DekuReader;

use crate::{
    Ctx,
    axis::{
        party::{Counterparties, PartyCommit},
        token::token_reader::TokenDataTriple,
    },
    axis_helpers::MarketSpec,
    for_axes,
    goblin_error::GoblinError,
    input_processor::ArgsReader,
    instructions::{process_makes, process_takes},
    market::MarketHeader,
    settlement::{
        StaticDelta,
        local_delta::{LocalDeposits, LocalUpdate},
    },
    types::{Address, StoreReader},
};

#[inline(never)]
pub fn process_market_inner<'a, MS: MarketSpec>(
    msg_sender: &Address,
    reader: &mut ArgsReader<'_>,
    token_data_triple: &TokenDataTriple<'a>,
    static_delta: &mut StaticDelta,
) -> Result<(), GoblinError> {
    let header =
        MarketHeader::from_reader_with_ctx(reader, ()).map_err(|_| GoblinError::InvalidPayload)?;

    let local_deposits = if header.decode_deposit_amounts {
        LocalDeposits::<MS::Pair>::from_reader_with_ctx(reader, ())
            .map_err(|_| GoblinError::InvalidPayload)?
    } else {
        LocalDeposits::<MS::Pair>::default()
    };

    let ctx = &mut Ctx::<MS>::try_new(
        msg_sender,
        reader,
        token_data_triple,
        &mut static_delta.local_counterparties,
    )?;

    process_takes(&header, reader, ctx)?;
    process_makes(&header, reader, ctx)?;

    let market = &ctx.readables.market_readables().market;
    let local_update = LocalUpdate::<MS::Pair>::from((&ctx.writables.local_delta, local_deposits));

    // Commit sender and counterparty
    for_axes!(PT => PT::commit::<MS::Pair>(
        market,
        &local_update,
        &mut static_delta.global
    )?);

    // Reset counter of global mut counterparty buffer
    Counterparties::get_leg_mut(&mut ctx.writables.local_delta).reset();

    Ok(())
}
