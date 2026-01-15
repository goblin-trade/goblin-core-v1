use crate::{goblin_error::GoblinError, token::CustomERC20Store, types::Address};

/// Convenience trait to map token index to address. Used as a trait bound on MarketVariant::TokenIndex.
///
/// We need a separate trait in addition to ERC20Data because
///
/// * We must cover DynamicIndex, an enum type.
/// * DynamicIndex does not use ERC20Data directly
///
pub trait AddressGetter {
    fn address(self, custom_erc20_list: &[CustomERC20Store]) -> Result<Address, GoblinError>;
}
