use crate::{
    axis::token::token_quantity::TokenQuantity,
    quantities::UnsidedDeltaAtoms,
    settlement::{global_delta::TokenDelta, ConstZero},
};

impl<T: TokenQuantity> ConstZero for TokenDelta<T> {
    const ZEROED: Self = Self {
        deposit: <T as TokenQuantity>::GlobalDeposit::ZEROED,
        take: UnsidedDeltaAtoms::ZEROED,
        make: UnsidedDeltaAtoms::ZEROED,
    };
}
