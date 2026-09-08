use crate::{
    Readables, Writables, axis::token::TokenDataTriple, axis_helpers::MarketSpec,
    goblin_error::GoblinError, input_processor::ArgsReader, settlement::LocalCounterparties,
    types::Address,
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
        local_counterparties: &'a mut LocalCounterparties,
    ) -> Result<Self, GoblinError> {
        let readables = Readables::try_new(msg_sender, reader, token_data_triple)?;
        let writables = Writables::try_new(
            &readables.market_readables().market_key,
            local_counterparties,
        )?;

        Ok(Self {
            readables,
            writables,
        })
    }
}
