use crate::{
    axis::token::token_marker::TokenMarker,
    quantities::DeltaAtoms,
    settlement::{global_delta_v3::TokenDeltaV3, ConstZero},
};

impl<T: TokenMarker> ConstZero for TokenDeltaV3<T> {
    const ZEROED: Self = Self {
        deposit: <T as TokenMarker>::GlobalDeposit::ZEROED,
        take: DeltaAtoms::ZEROED,
        make: DeltaAtoms::ZEROED,
    };
}
