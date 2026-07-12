use crate::{
    axis::token::{CustomERC20Stub, HardcodedERC20Stub, Token},
    goblin_error::GoblinError,
    input_processor::{DecodeCtx, ETHTransfers, HeaderFlags},
    types::Triple,
};

pub type MsgTransfers = Triple<ETHTransfers, HardcodedERC20Stub, CustomERC20Stub, Token>;

impl MsgTransfers {
    pub fn try_new(ctx: &DecodeCtx, flags: &HeaderFlags) -> Result<Self, GoblinError> {
        let eth_transfers = ETHTransfers::try_new(ctx, flags)?;

        Ok(Triple::new(
            eth_transfers,
            HardcodedERC20Stub,
            CustomERC20Stub,
        ))
    }
}
