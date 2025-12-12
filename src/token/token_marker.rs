#[derive(Clone, Copy)]
pub struct ETH;

#[derive(Clone, Copy)]
pub struct ERC20;

pub trait TokenMarker {}

impl TokenMarker for ETH {}
impl TokenMarker for ERC20 {}
