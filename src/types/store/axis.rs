///! Axes or namespaces used by the exchange
///! Use them with `Marker` to generate sub types. The sub types will
///! then implement its respective trait.

/// Leg axis- Base or Quote leg
pub struct Leg;

/// Market axis- Hardcoded, Custom
pub struct Market;

/// Token axis- ETH, HardcodedERC20, CustomERC20
pub struct Token;
