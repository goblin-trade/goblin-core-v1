use crate::{
    goblin_error::GoblinError,
    quantities::{RawAtoms, UnsidedAtoms},
};

impl<const D: u8> TryFrom<UnsidedAtoms> for RawAtoms<D> {
    type Error = GoblinError;

    fn try_from(value: UnsidedAtoms) -> Result<Self, Self::Error> {
        match D {
            6 => {
                // No conversion needed. Optimized implementation avoids creation of multiplier.
                //  Simply copy the 64 bit number from index 24 onwards
                let mut raw_atom_bytes = [0u8; 32];
                raw_atom_bytes[24..].copy_from_slice(&value.inner.to_be_bytes());
                Ok(RawAtoms(raw_atom_bytes))
            }
            7..19 => {
                // floor (log2 (u64::MAX * 10^12)) + 1 = 104
                // This fits in u128
                let multiplier = 10u128.pow(D as u32 - 6);
                let raw_atoms = value.inner as u128 * multiplier;

                let mut raw_atom_bytes = [0u8; 32];
                raw_atom_bytes[16..].copy_from_slice(&raw_atoms.to_be_bytes());
                Ok(RawAtoms(raw_atom_bytes))
            }

            _ => Err(GoblinError::UnsupportedDecimals),
        }
    }
}
