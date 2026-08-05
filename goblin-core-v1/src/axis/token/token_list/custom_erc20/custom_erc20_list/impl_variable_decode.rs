use crate::{
    axis::token::{
        token_list::custom_erc20::{CustomERC20Count, CustomERC20List},
        token_marker::TokenData,
        CustomERC20,
    },
    input_processor::{DecodeCtx, VariableDecode},
};

impl<'a> VariableDecode<'a> for CustomERC20List<'a> {
    type Flags = CustomERC20Count;

    // problem- we need total size which includes this byte too
    fn size(flags: &Self::Flags) -> usize {
        flags.0
    }

    fn raw_variable_decode(ctx: &'a DecodeCtx, flags: &Self::Flags) -> Self {
        let inner = ctx.zero_copy_slice_unchecked::<TokenData<CustomERC20>>(flags.0);
        CustomERC20List { inner }
    }
}
