use crate::{
    axis::token::{
        token_list::custom_erc20::CustomERC20List, token_marker::TokenData, CustomERC20,
    },
    input_processor::{DecodeCtx, HeaderFlags, VariableDecode},
    types::Address,
};

impl<'a> VariableDecode<'a> for CustomERC20List<'a> {
    type Flags = HeaderFlags;

    fn size(flags: &Self::Flags) -> usize {
        flags.custom_erc20_count * core::mem::size_of::<Address>()
    }

    fn raw_variable_decode(ctx: &'a DecodeCtx, flags: &Self::Flags) -> Self {
        let inner =
            ctx.zero_copy_slice_unchecked::<TokenData<CustomERC20>>(flags.custom_erc20_count);
        CustomERC20List { inner }
    }
}
