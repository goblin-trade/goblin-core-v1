use crate::{
    axis::token::{token_list::custom_erc20::CustomERC20List, token_reader::TokenDataTriple},
    input_processor::{
        ArgsReader, HeaderFlags, VariableDecode, ZeroCopyReader,
        global_args::header_refs::HeaderRefs,
    },
    types::Address,
};

impl<'a> VariableDecode<'a> for HeaderRefs<'a> {
    type Flags = HeaderFlags;

    fn size(flags: &Self::Flags) -> usize {
        (flags.read_custom_recipient as usize * core::mem::size_of::<Address>())
            + CustomERC20List::size(flags)
    }

    fn raw_variable_decode(reader: &'a ArgsReader, flags: &Self::Flags) -> Self {
        let custom_recipient = if flags.read_custom_recipient {
            Some(reader.zero_copy_unchecked::<Address>())
        } else {
            None
        };

        let custom_erc20_list = CustomERC20List::raw_variable_decode(reader, flags);
        let token_data_triple = TokenDataTriple::const_from(custom_erc20_list);

        Self {
            custom_recipient,
            token_data_triple,
        }
    }
}
