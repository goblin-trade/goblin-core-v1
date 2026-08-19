use crate::{
    axis::token::token_reader::TokenDataTriple, axis_helpers::MarketSpec,
    goblin_error::GoblinError, input_processor::DecodeCtx, market::MarketReadables,
    settlement::local_delta::LocalDeposits, types::Address,
};

pub struct Readables<'a, MS: MarketSpec> {
    pub msg_sender: &'a Address,
    pub deposits: LocalDeposits,
    locator: MS::Locator,
}

impl<'a, MS: MarketSpec> Readables<'a, MS> {
    pub fn try_new(
        msg_sender: &'a Address,
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple<'a>,
        decode_deposit_amounts: bool,
    ) -> Result<Self, GoblinError> {
        // TODO read which one first?
        let locator = MS::decode_locator(ctx, token_data_triple)?;
        let deposits = LocalDeposits::try_new::<MS::Pair>(decode_deposit_amounts, ctx)?;

        Ok(Self {
            msg_sender,
            deposits,
            locator,
        })
    }

    pub fn market_readables(&self) -> &MarketReadables<MS> {
        MS::locate_market(&self.locator)
    }
}
