use crate::{
    axis::token::token_reader::TokenDataTriple, axis_helpers::MarketSpec,
    goblin_error::GoblinError, input_processor::ArgsReader,
    settlement::local_delta::local_take::TakeCounterparties, types::Address, Readables, Writables,
};

pub struct Ctx<'a, MS: MarketSpec> {
    pub readables: Readables<'a, MS>,
    pub writables: Writables<'a>,
}

impl<'a, MS: MarketSpec> Ctx<'a, MS> {
    pub fn try_new(
        msg_sender: &'a Address,
        reader: &ArgsReader,
        token_data_triple: &TokenDataTriple<'a>,
        take_counterparties: &'a mut TakeCounterparties,
    ) -> Result<Self, GoblinError> {
        let readables = Readables::try_new(msg_sender, reader, token_data_triple)?;
        let writables = Writables::try_new(
            &readables.market_readables().market_key,
            take_counterparties,
        )?;

        Ok(Self {
            readables,
            writables,
        })
    }
}
