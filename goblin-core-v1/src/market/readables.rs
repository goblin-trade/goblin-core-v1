use crate::{
    axis::token::token_reader::TokenDataTriple,
    axis_helpers::MarketSpec,
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, FixedDecode},
    market::MarketReadables,
    settlement::local_delta::LocalDeposits,
    types::Address,
};

pub struct Readables<'a, MS: MarketSpec> {
    pub msg_sender: &'a Address,
    pub deposits: LocalDeposits<MS::Pair>,
    locator: MS::Locator,
}

impl<'a, MS: MarketSpec> Readables<'a, MS> {
    pub fn try_new(
        msg_sender: &'a Address,
        ctx: &DecodeCtx,
        token_data_triple: &TokenDataTriple<'a>,
        decode_deposit_amounts: bool,
    ) -> Result<Self, GoblinError> {
        let locator = MS::decode_locator(ctx, token_data_triple)?;

        let deposits = if decode_deposit_amounts {
            LocalDeposits::<MS::Pair>::try_fixed_decode(ctx)?
        } else {
            LocalDeposits::<MS::Pair>::default()
        };

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
