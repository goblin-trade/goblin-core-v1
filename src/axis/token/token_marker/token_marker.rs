use crate::axis::token::{token_deltas::TokenDeltas, CustomERC20, HardcodedERC20, ETH};

/// Marker trait for the 3 token variants
pub trait TokenMarker: 'static + TokenDeltas {}

impl TokenMarker for ETH {}
impl TokenMarker for HardcodedERC20 {}
impl TokenMarker for CustomERC20 {}
