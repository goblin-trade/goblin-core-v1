use crate::{
    goblin_error::GoblinError,
    input_processor::{Decodable, DecodeCtx},
    token::{ERC20Index, HardcodedERC20, TokenMarker},
};

impl<'a> Decodable<'a> for <HardcodedERC20 as TokenMarker>::TokenIndex {
    fn try_decode(ctx: &'a DecodeCtx<'a>) -> Result<Self, GoblinError> {
        u8::try_decode(ctx).map(ERC20Index::new)
    }
}
