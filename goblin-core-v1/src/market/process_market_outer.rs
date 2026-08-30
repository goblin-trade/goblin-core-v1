use crate::{
    axis::token::token_reader::TokenDataTriple,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, MarketCountsV2},
    market::process_market,
    settlement::StaticDelta,
    types::Address,
};

pub fn process_market_outer<'a, MS: MarketSpec>(
    msg_sender: &Address,
    reader: &ArgsReader,
    token_data_triple: &TokenDataTriple<'a>,
    static_delta: &mut StaticDelta,
    market_counts: &mut MarketCountsV2,
) -> Result<(), GoblinError> {
    if MS::illegal() {
        return Ok(());
    }

    let count = market_counts.get_count_and_advance();

    for _ in 0..count {
        process_market::<MS>(msg_sender, reader, token_data_triple, static_delta)?;
    }

    Ok(())
}
