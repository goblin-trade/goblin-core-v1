use crate::{
    axis::{
        market::{Market, MarketEnum},
        token::{token_reader::TokenDataTriple, TokenEnum},
    },
    axis_helpers::{AxisMarker, MarketSpec, TokenPair},
    goblin_error::GoblinError,
    input_processor::ArgsReader,
    market::process_market,
    settlement::StaticDelta,
    types::Address,
};

pub fn process_market_outer<'a, MS: MarketSpec>(
    msg_sender: &Address,
    reader: &ArgsReader,
    token_data_triple: &TokenDataTriple<'a>,
    static_delta: &mut StaticDelta,
) -> Result<(), GoblinError> {
    if (MS::Market::VARIANT == MarketEnum::Hardcoded
        && (<MS::Pair as TokenPair>::Base::VARIANT == TokenEnum::CustomERC20
            || <MS::Pair as TokenPair>::Quote::VARIANT == TokenEnum::CustomERC20))
        || (<MS::Pair as TokenPair>::Base::VARIANT == TokenEnum::ETH
            && <MS::Pair as TokenPair>::Quote::VARIANT == TokenEnum::ETH)
    {
        return Ok(());
    }

    process_market::<MS>(msg_sender, reader, token_data_triple, static_delta)
}
