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
    fn try_decode(ctx: &'a DecodeCtx<'a>) -> Result<Self, GoblinError> {
        u8::try_decode(ctx).map(DynamicIndex::new)
    }
}

impl<'a> Decodable<'a> for <ERC20 as TokenMarker>::Deposit {
    fn try_decode(ctx: &'a DecodeCtx<'a>) -> Result<Self, GoblinError> {
        i64::try_decode(ctx).map(DeltaAtoms::new)
    }
}
