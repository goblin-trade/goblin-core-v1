#[cfg(test)]
mod test;

use crate::{
    goblin_error::GoblinError,
    quantities::{RawAtoms, UnsidedAtoms},
};

impl<const D: u8> TryFrom<RawAtoms<D>> for UnsidedAtoms {
    type Error = GoblinError;

    fn try_from(value: RawAtoms<D>) -> Result<Self, Self::Error> {
        let raw_atoms_clamped = value.to_clamped_u128();

        match D {
            6 => {
                // Decimal places are already 6
                let atoms = raw_atoms_clamped.min(u64::MAX as u128) as u64;
                Ok(UnsidedAtoms::new(atoms))
            }
            7..19 => {
                let divisor = 10u128.pow(D as u32 - 6);
                let atoms = (raw_atoms_clamped / divisor).min(u64::MAX as u128) as u64;

                Ok(UnsidedAtoms::new(atoms))
            }
            _ => Err(GoblinError::UnsupportedDecimals),
        }
    }
}
