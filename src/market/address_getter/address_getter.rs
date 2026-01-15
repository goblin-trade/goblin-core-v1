use crate::{token::CustomERC20Store, types::Address};

pub trait AddressGetter {
    fn address(&self, custom_erc20_list: &[CustomERC20Store]) -> Address;
}
