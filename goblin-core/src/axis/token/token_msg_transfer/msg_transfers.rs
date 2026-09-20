use crate::{
    axis::token::{CustomERC20Stub, ETHTransfers, HardcodedERC20Stub, Token},
    types::Triple,
};

pub type MsgTransfers = Triple<ETHTransfers, HardcodedERC20Stub, CustomERC20Stub, Token>;

impl From<ETHTransfers> for MsgTransfers {
    fn from(value: ETHTransfers) -> Self {
        Triple::new(value, HardcodedERC20Stub, CustomERC20Stub)
    }
}
