use crate::quantities::DeltaAtoms;
use crate::types::TokenPair;

/// Track deposit amounts for a given side
///
/// * The first limb (ETH) is `()` because ETH does not support deposits
/// * The second limb (ERC20) is `DeltaAtoms`. It is shared by HardcodedERC20 and
/// CustomERC20
pub type DepositForSide = TokenPair<(), DeltaAtoms>;

impl DepositForSide {
    pub const fn zero() -> Self {
        Self::new((), DeltaAtoms::ZERO)
    }
}
