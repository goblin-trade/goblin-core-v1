use crate::{
    axis::token::{
        token_list::custom_erc20::CustomERC20List, token_marker::TokenData, CustomERC20,
    },
    input_processor::{DecodeCtx, FixedDecode, HeaderFlags, VariableDecode},
};

impl<'a> VariableDecode<'a> for CustomERC20List<'a> {
    // problem-
    // * whether to read u8 depends on flag
    // * then array size depends on this u8
    //
    // we have 2 variables
    type Flags = HeaderFlags;

    fn size(flags: &Self::Flags) -> usize {
        if flags.read_custom_erc20 {
            1
        } else {
            0
        }
    }

    fn raw_variable_decode(ctx: &'a DecodeCtx, flags: &Self::Flags) -> Self {
        let inner = if flags.read_custom_erc20 {
            let custom_erc20_count = u8::raw_fixed_decode(ctx) as usize;

            ctx.zero_copy_slice_unchecked::<TokenData<CustomERC20>>(custom_erc20_count)
        } else {
            ctx.zero_copy_slice_unchecked::<TokenData<CustomERC20>>(0)
        };

        CustomERC20List { inner }
    }
}
