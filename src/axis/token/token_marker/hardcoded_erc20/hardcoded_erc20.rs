use crate::axis::token::{token_marker::TokenMarker, HardcodedERC20};

impl TokenMarker for HardcodedERC20 {
    const DISCRIMINATOR: u8 = 1;
}
