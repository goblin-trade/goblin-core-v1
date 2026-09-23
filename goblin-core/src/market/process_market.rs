use deku::no_std_io::{Cursor, Read};

use crate::{
    axis::token::TokenDataTriple, axis_helpers::MarketSpec, goblin_error::GoblinError,
    input_processor::ArgsReader, market::process_market_inner, settlement::StaticDelta,
    types::Address,
};

pub fn process_market<'a, MS: MarketSpec>(
    msg_sender: &Address,
    reader: &mut ArgsReader<'_>,
    token_data_triple: &TokenDataTriple<'a>,
    market_counts: &mut Cursor<&[u8]>,
    static_delta: &mut StaticDelta,
) -> Result<(), GoblinError> {
    if MS::illegal() {
        return Ok(());
    }

    let mut count = [0u8; 1];
    market_counts
        .read_exact(&mut count)
        .map_err(|_| GoblinError::InvalidPayload)?;

    for _ in 0..count[0] {
        process_market_inner::<MS>(msg_sender, reader, token_data_triple, static_delta)?;
    }

    Ok(())
}
