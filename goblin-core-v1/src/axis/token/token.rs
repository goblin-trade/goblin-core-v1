use crate::types::Marker;

/// Token axis- ETH, hardcoded ERC20 or custom ERC20
#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Token;

pub type ETH = Marker<Token, 0>;
pub type HardcodedERC20 = Marker<Token, 1>;
pub type CustomERC20 = Marker<Token, 2>;
