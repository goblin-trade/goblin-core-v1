use crate::{
    goblin_error::GoblinError,
    input_processor::{DecodablePrimitive, DecodeCtx},
    market::{process_market, Dynamic},
    require,
    settlement::Delta,
    token::{CustomERC20, CustomERC20Data, HardcodedERC20, ETH},
    types::Address,
};

const BYTE_COUNT: usize = 5;

pub struct DynamicMarketHeader<'a> {
    pub market_counts: [u8; 8],

    /// Addresses of custom erc20 tokens to use
    pub custom_erc20_list: &'a [CustomERC20Data],
}

impl<'a> DynamicMarketHeader<'a> {
    pub fn new(ctx: &'a DecodeCtx) -> Result<Self, GoblinError> {
        require!(
            ctx.len() >= ctx.offset.get() + BYTE_COUNT,
            GoblinError::InvalidPayload
        );

        let byte_0 = u8::decode_unchecked_no_advance(ctx);
        let byte_1 = u8::decode_unchecked_no_advance(ctx);
        let byte_2 = u8::decode_unchecked_no_advance(ctx);
        let byte_3 = u8::decode_unchecked_no_advance(ctx);
        let byte_4 = u8::decode_unchecked_no_advance(ctx);

        let market_counts = [
            byte_0 & 0b0000_1111,
            byte_0 >> 4,
            byte_1 & 0b0000_1111,
            byte_1 >> 4,
            byte_2 & 0b0000_1111,
            byte_2 >> 4,
            byte_3 & 0b0000_1111,
            byte_3 >> 4,
        ];

        let custom_erc20_count = byte_4 as usize;

        ctx.advance_offset(BYTE_COUNT);

        let custom_erc20_list =
            ctx.zero_copy_slice_unchecked::<CustomERC20Data>(custom_erc20_count);

        Ok(Self {
            market_counts,
            custom_erc20_list,
        })
    }

    pub fn process_markets(
        &self,
        ctx: &DecodeCtx,
        msg_sender: &Address,
        delta: &mut Delta,
    ) -> Result<(), GoblinError> {
        // Dynamic with hardcoded ERC20 (3)
        for _ in 0..self.market_counts[0] {
            process_market::<Dynamic, ETH, HardcodedERC20>(
                ctx,
                msg_sender,
                self.custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.market_counts[1] {
            process_market::<Dynamic, HardcodedERC20, ETH>(
                ctx,
                msg_sender,
                self.custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.market_counts[2] {
            process_market::<Dynamic, HardcodedERC20, HardcodedERC20>(
                ctx,
                msg_sender,
                self.custom_erc20_list,
                delta,
            )?;
        }

        // Dynamic with custom ERC20 (3)
        for _ in 0..self.market_counts[3] {
            process_market::<Dynamic, ETH, CustomERC20>(
                ctx,
                msg_sender,
                self.custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.market_counts[4] {
            process_market::<Dynamic, CustomERC20, ETH>(
                ctx,
                msg_sender,
                self.custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.market_counts[5] {
            process_market::<Dynamic, CustomERC20, CustomERC20>(
                ctx,
                msg_sender,
                self.custom_erc20_list,
                delta,
            )?;
        }

        // Dynamic with mixture of hardcoded and custom ERC20 (2)
        for _ in 0..self.market_counts[6] {
            process_market::<Dynamic, HardcodedERC20, CustomERC20>(
                ctx,
                msg_sender,
                self.custom_erc20_list,
                delta,
            )?;
        }

        for _ in 0..self.market_counts[7] {
            process_market::<Dynamic, CustomERC20, HardcodedERC20>(
                ctx,
                msg_sender,
                self.custom_erc20_list,
                delta,
            )?;
        }

        Ok(())
    }
}
