pub struct ETH;
pub struct ERC20;

pub trait TokenMarker {}

impl TokenMarker for ETH {}
impl TokenMarker for ERC20 {}
