use crate::{
    axis::token::token_reader::TokenDataTriple, axis_helpers::MarketSpec,
    goblin_error::GoblinError, input_processor::ArgsReader, market::MarketReadables,
    types::Address,
};

pub struct Readables<'a, MS: MarketSpec> {
    pub msg_sender: &'a Address,
    locator: MS::Locator,
}

impl<'a, MS: MarketSpec> Readables<'a, MS> {
    pub fn try_new(
        msg_sender: &'a Address,
        reader: &ArgsReader,
        token_data_triple: &TokenDataTriple<'a>,
    ) -> Result<Self, GoblinError> {
        let locator = MS::decode_locator(reader, token_data_triple)?;

        Ok(Self {
            msg_sender,
            locator,
        })
    }

    pub fn market_readables(&self) -> &MarketReadables<MS::Pair> {
        MS::locate_market(&self.locator)
    }
}
