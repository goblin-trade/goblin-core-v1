use crate::input_processor::{DecodeCtx, FixedDecode, HeaderFlags};

impl<'a> FixedDecode<'a> for HeaderFlags {
    const ENCODED_SIZE: usize = 1;

    fn raw_fixed_decode(ctx: &'a DecodeCtx) -> Self {
        let byte_0 = u8::raw_fixed_decode(ctx);

        Self {
            read_custom_recipient: (byte_0 & 0b0000_0001) != 0,
            read_msg_value: (byte_0 & 0b0000_0010) != 0,
            process_dynamic_markets: (byte_0 & 0b0000_0100) != 0,
            withdraw_eth: (byte_0 & 0b0000_1000) != 0,
            withdraw_internally: (byte_0 & 0b0001_0000) != 0,
            custom_erc20_count: (byte_0 >> 5) as usize,
        }
    }
}
