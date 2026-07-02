use crate::axis::token::{token_marker::TokenMarker, CustomERC20};

impl TokenMarker for CustomERC20 {
    const DISCRIMINATOR: u8 = 2;

    type StoredDecimals = u8;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];
}
