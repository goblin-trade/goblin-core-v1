use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    market::Dynamic,
    quantities::DeltaAtoms,
    token::{DynamicIndex, TokenMarker, ERC20},
};

impl<'a> Decodable<'a> for <ERC20 as TokenMarker>::TokenIndex<Dynamic>
where
    ERC20: TokenMarker,
{
    fn decode(ctx: &'a DecodeCtx<'a>) -> Result<Self, GoblinError> {
        ctx.decode::<u8>().map(DynamicIndex::new)?
    }
}

impl<'a> Decodable<'a> for <ERC20 as TokenMarker>::Deposit {
    fn decode(ctx: &'a DecodeCtx<'a>) -> Result<Self, GoblinError> {
        ctx.decode::<i64>().map(DeltaAtoms::new)
    }
}
