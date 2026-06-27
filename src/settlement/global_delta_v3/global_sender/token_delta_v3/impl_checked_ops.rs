use crate::{
    axis::token::token_marker::TokenMarker,
    settlement::{global_delta_v3::TokenDeltaV3, CheckedOps},
};

impl<T: TokenMarker> CheckedOps for TokenDeltaV3<T> {
    fn checked_add(self, rhs: Self) -> Option<Self> {
        Some(Self {
            deposit: self.deposit.checked_add(rhs.deposit)?,
            take: self.take.checked_add(rhs.take)?,
            make: self.make.checked_add(rhs.make)?,
        })
    }

    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            deposit: self.deposit.checked_sub(rhs.deposit)?,
            take: self.take.checked_sub(rhs.take)?,
            make: self.make.checked_sub(rhs.make)?,
        })
    }
}
