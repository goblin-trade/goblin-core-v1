use crate::{
    axis::token::token_list::custom_erc20::CustomERC20Count,
    input_processor::{FixedDecode, HeaderFlags, VariableDecode},
};

impl<'a> VariableDecode<'a> for CustomERC20Count {
    type Flags = HeaderFlags;

    fn size(flags: &Self::Flags) -> usize {
        flags.read_custom_erc20 as usize
    }

    fn raw_variable_decode(
        ctx: &'a crate::input_processor::DecodeCtx,
        flags: &Self::Flags,
    ) -> Self {
        Self(if flags.read_custom_erc20 {
            u8::raw_fixed_decode(ctx) as usize
        } else {
            0
        })
    }
}
