use crate::{
    axis::token::TokenDataTriple,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{ArgsReader, MarketCounts},
    market::process_market_inner,
    settlement::StaticDelta,
    types::Address,
};

pub fn process_market<'a, MS: MarketSpec>(
    msg_sender: &Address,
    reader: &mut ArgsReader<'_>,
    token_data_triple: &TokenDataTriple<'a>,
    market_counts: &MarketCounts,
    static_delta: &mut StaticDelta,
) -> Result<(), GoblinError> {
    let count = market_counts.get_count::<MS>();

    for _ in 0..count {
        process_market_inner::<MS>(msg_sender, reader, token_data_triple, static_delta)?;
    }

    Ok(())
}
