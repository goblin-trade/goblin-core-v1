use crate::axis::token::{token_marker::TokenMarker, ETH};

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;
}
