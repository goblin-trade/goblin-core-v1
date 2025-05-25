use crate::{define_custom_types, goblin_error::GoblinError};

use super::Atoms;

define_custom_types!(Delta<i64>);

impl Delta {
    pub fn abs(&self) -> Atoms {
        Atoms(self.0.abs() as u64)
    }

    pub fn rev(&self) -> Self {
        Delta(-self.0)
    }

    pub fn checked_add(self, rhs: Delta) -> Result<Delta, GoblinError> {
        self.0
            .checked_add(rhs.0)
            .map(Delta)
            .ok_or(GoblinError::DeltaOverflow)
    }

    pub fn checked_sub(self, rhs: Delta) -> Result<Delta, GoblinError> {
        self.0
            .checked_sub(rhs.0)
            .map(Delta)
            .ok_or(GoblinError::DeltaUnderflow)
    }
}

impl core::ops::Add<Atoms> for Delta {
    type Output = Result<Delta, GoblinError>;

    fn add(self, atoms: Atoms) -> Self::Output {
        // Ensure the atoms value fits in i64
        if let Ok(val) = i64::try_from(atoms.0) {
            self.0
                .checked_add(val)
                .map(Delta)
                .ok_or(GoblinError::DeltaOverflow)
        } else {
            Err(GoblinError::DeltaOverflow)
        }
    }
}

impl core::ops::Sub<Atoms> for Delta {
    type Output = Result<Delta, GoblinError>;

    fn sub(self, atoms: Atoms) -> Self::Output {
        if let Ok(val) = i64::try_from(atoms.0) {
            self.0
                .checked_sub(val)
                .map(Delta)
                .ok_or(GoblinError::DeltaUnderflow)
        } else {
            Err(GoblinError::DeltaUnderflow)
        }
    }
}

impl core::ops::Add<Delta> for Atoms {
    type Output = Result<Atoms, GoblinError>;

    fn add(self, delta: Delta) -> Self::Output {
        if delta.0 >= 0 {
            self.0
                .checked_add(delta.0 as u64)
                .map(Atoms)
                .ok_or(GoblinError::AtomOverflow)
        } else {
            self.0
                .checked_sub(delta.0.unsigned_abs())
                .map(Atoms)
                .ok_or(GoblinError::AtomUndeflow)
        }
    }
}

impl core::ops::Sub<Delta> for Atoms {
    type Output = Result<Atoms, GoblinError>;

    fn sub(self, delta: Delta) -> Self::Output {
        if delta.0 >= 0 {
            self.0
                .checked_sub(delta.0 as u64)
                .map(Atoms)
                .ok_or(GoblinError::AtomUndeflow)
        } else {
            self.0
                .checked_add(delta.0.unsigned_abs())
                .map(Atoms)
                .ok_or(GoblinError::AtomOverflow)
        }
    }
}
