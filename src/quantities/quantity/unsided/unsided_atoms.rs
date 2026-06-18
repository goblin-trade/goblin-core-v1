///! The number of atoms of a token, obtained by normalizing raw atoms.
///!
///! Every token is normalized from K decimal places to 6 decimal places.
///! * USDC (6 decimal places): no change
///! * ETH (18 decimal places): Normalized from 18 -> 6 decimal places
///!
///! This allows us to use u64 instead of U256 to represent atoms.
///! These atoms are grouped into lots.
///!
///! Important- Decimal places must be in the range [6, 18]
///!
///! * Raw atoms will be undefined if decimal places are less than 6
///!
///! * Since raw atoms are internally capped to 128 bits, decimal places more than
///! 19 can overflow this value.
///!
///! * We remove 12 places for 18 decimals. To convert into Raw atoms again,
///! we need floor (log2 (u64::MAX * 10^12)) + 1 = 104 bits to represent u64::MAX atoms.
///! 104 bits fit within u128.
///!
///! # Math
///!
///! atoms = |raw atoms / 10^(K - 6)|
///! - For USDC = raw atoms / 10^0 = raw atoms
///! - For eth = raw atoms / 10^(18 - 6) = raw atoms / 10^12
///
use crate::{
    goblin_error::GoblinError,
    quantities::{Quantity, RawAtoms, SidedDim, P1, Z0},
};

pub type Unsided<L, U, A> = Quantity<SidedDim<L, U, A>>;
pub type UnsidedAtoms = Unsided<Z0, Z0, P1>;

impl UnsidedAtoms {
    pub fn from_raw_atoms(raw: &RawAtoms, decimals: u8) -> Result<Self, GoblinError> {
        let raw_atoms_clamped = raw.to_clamped_u128();

        match decimals {
            6 => {
                // Decimal places are already 6
                let atoms = raw_atoms_clamped.min(u64::MAX as u128) as u64;
                Ok(UnsidedAtoms::new(atoms))
            }
            7..19 => {
                let divisor = 10u128.pow(decimals as u32 - 6);
                let atoms = (raw_atoms_clamped / divisor).min(u64::MAX as u128) as u64;

                Ok(UnsidedAtoms::new(atoms))
            }
            _ => Err(GoblinError::UnsupportedDecimals),
        }
    }

    pub fn to_raw_atoms(&self, decimals: u8) -> Result<RawAtoms, GoblinError> {
        match decimals {
            6 => {
                // No conversion needed. Optimized implementation avoids creation of multiplier.
                //  Simply copy the 64 bit number from index 24 onwards
                let mut raw_atom_bytes = [0u8; 32];
                raw_atom_bytes[24..].copy_from_slice(&self.inner.to_be_bytes());
                Ok(RawAtoms(raw_atom_bytes))
            }
            7..19 => {
                // floor (log2 (u64::MAX * 10^12)) + 1 = 104
                // This fits in u128
                let multiplier = 10u128.pow(decimals as u32 - 6);
                let raw_atoms = self.inner as u128 * multiplier;

                let mut raw_atom_bytes = [0u8; 32];
                raw_atom_bytes[16..].copy_from_slice(&raw_atoms.to_be_bytes());
                Ok(RawAtoms(raw_atom_bytes))
            }

            _ => Err(GoblinError::UnsupportedDecimals),
        }
    }
}
