/// Marker type for ETH within a token pair
pub struct ETH;

/// Marker type for ERC20 within a token pair
pub struct ERC20;

pub trait TokenMarker {}

impl TokenMarker for ETH {}
impl TokenMarker for ERC20 {}
