use crate::{
    axis::token::token_marker::ETHDelta,
    settlement::{global_delta::TokenDelta, ConstZero},
};

impl ConstZero for ETHDelta {
    const ZEROED: Self = Self {
        inner: TokenDelta::ZEROED,
    };
}
