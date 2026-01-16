use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    token::{CustomERC20, ERC20Index, TokenMarker},
};

impl<'a> Decodable<'a> for <CustomERC20 as TokenMarker>::TokenIndex {
    fn try_decode(ctx: &'a DecodeCtx<'a>) -> Result<Self, GoblinError> {
        u8::try_decode(ctx).map(ERC20Index::new)
    }
}
