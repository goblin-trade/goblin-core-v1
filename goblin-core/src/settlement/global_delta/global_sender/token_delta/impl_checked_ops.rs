use crate::{
    axis::token::token_marker::TokenMarker,
    settlement::{CheckedOps, TokenDelta},
};

impl<TM: TokenMarker> CheckedOps for TokenDelta<TM> {
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

    fn checked_mul(self, rhs: Self) -> Option<Self> {
        Some(Self {
            deposit: CheckedOps::checked_mul(self.deposit, rhs.deposit)?,
            take: CheckedOps::checked_mul(self.take, rhs.take)?,
            make: CheckedOps::checked_mul(self.make, rhs.make)?,
        })
    }
}
