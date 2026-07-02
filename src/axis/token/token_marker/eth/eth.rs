use crate::axis::token::{
    token_marker::{eth::ETHStub, TokenMarker},
    ETH,
};

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type StoredDecimals = ETHStub;
    type StoredPadding = [u8; 16 - size_of::<Self::StoredDecimals>()];
}
