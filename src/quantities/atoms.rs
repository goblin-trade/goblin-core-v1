use super::RawAtoms;
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
use crate::{
    define_custom_type, define_delta_operations,
    goblin_error::GoblinError,
    quantities::{BaseAtoms, BaseAtomsDelta, QuoteAtoms, QuoteAtomsDelta},
};
use core::u64;

define_custom_type!(Atoms<u64>);
define_custom_type!(AtomsDelta<i64>);

impl Atoms {
    pub fn from_raw_atoms(raw: &RawAtoms, decimals: u8) -> Result<Self, GoblinError> {
        let raw_atoms_clamped = raw.to_clamped_u128();

        match decimals {
            6 => {
                // Decimal places are already 6
                let atoms = raw_atoms_clamped.min(u64::MAX as u128) as u64;
                Ok(Atoms(atoms))
            }
            7..19 => {
                let divisor = 10u128.pow(decimals as u32 - 6);
                let atoms = (raw_atoms_clamped / divisor).min(u64::MAX as u128) as u64;

                Ok(Atoms(atoms))
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
                raw_atom_bytes[24..].copy_from_slice(&self.0.to_be_bytes());
                Ok(RawAtoms(raw_atom_bytes))
            }
            7..19 => {
                // floor (log2 (u64::MAX * 10^12)) + 1 = 104
                // This fits in u128
                let multiplier = 10u128.pow(decimals as u32 - 6);
                let raw_atoms = self.0 as u128 * multiplier;

                let mut raw_atom_bytes = [0u8; 32];
                raw_atom_bytes[16..].copy_from_slice(&raw_atoms.to_be_bytes());
                Ok(RawAtoms(raw_atom_bytes))
            }

            _ => Err(GoblinError::UnsupportedDecimals),
        }
    }

    pub fn to_delta(self) -> Result<AtomsDelta, GoblinError> {
        if self.0 <= i64::MAX as u64 {
            Ok(AtomsDelta(self.0 as i64))
        } else {
            Err(GoblinError::Overflow)
        }
    }
}

impl From<BaseAtoms> for Atoms {
    fn from(value: BaseAtoms) -> Self {
        Self(value.inner)
    }
}

impl From<QuoteAtoms> for Atoms {
    fn from(value: QuoteAtoms) -> Self {
        Self(value.inner)
    }
}

impl AtomsDelta {
    pub fn abs(&self) -> Atoms {
        Atoms(self.0.abs() as u64)
    }
}

define_delta_operations!(AtomsDelta<i64>, Atoms<u64>);

impl From<BaseAtomsDelta> for AtomsDelta {
    fn from(value: BaseAtomsDelta) -> Self {
        Self(value.inner)
    }
}

impl From<QuoteAtomsDelta> for AtomsDelta {
    fn from(value: QuoteAtomsDelta) -> Self {
        Self(value.inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod raw_atoms_to_atoms {
        use core::u128;

        use super::*;

        #[test]
        fn test_raw_atoms_to_atoms() {
            // Less than 6 decimals
            let raw = RawAtoms::from_u128(1_000_000);
            let atoms_result = Atoms::from_raw_atoms(&raw, 5);
            assert!(atoms_result.is_err());

            // USDC (6 decimals)
            let raw = RawAtoms::from_u128(1_000_000);
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, 1000000);

            // ETH (18 decimals)
            let raw = RawAtoms::from_u128(10u128.pow(18));
            let atoms = Atoms::from_raw_atoms(&raw, 18).unwrap();
            assert_eq!(atoms.0, 1000000);
        }

        #[test]
        fn test_dust() {
            // ETH (18 decimals)
            let raw = RawAtoms::from_u128(1_000_000);
            let atoms = Atoms::from_raw_atoms(&raw, 18).unwrap();
            assert_eq!(atoms.0, 0);
        }

        #[test]
        fn test_max_value() {
            // MAX possible value
            let raw = RawAtoms::MAX;
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, u64::MAX);

            // Set MSB (0 index in big endian) to 1
            let mut raw = RawAtoms::default();
            raw.0[0] = 1;
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, u64::MAX);

            // Smallest value greater than u128::MAX. This will get clamped to u128::MAX
            let mut raw = RawAtoms::default();
            raw.0[15] = 1;
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, u64::MAX);

            // Just below max value of u64
            let raw = RawAtoms::from_u128(u128::MAX - 1);
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, u64::MAX);

            // u64::MAX - 1
            let raw = RawAtoms::from_u128(u64::MAX as u128 - 1);
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, u64::MAX - 1);

            // Just below max value for ETH (18 decimals)
            let raw = RawAtoms::from_u128((u64::MAX as u128 - 1) * 10u128.pow(12));
            let atoms = Atoms::from_raw_atoms(&raw, 18).unwrap();
            assert_eq!(atoms.0, u64::MAX - 1);
        }
    }

    mod atoms_to_raw_atoms {
        use super::*;

        #[test]
        fn test_atoms_to_raw_atoms() {
            // atoms will convert to raw atoms without any loss of data
            // We just need to ensure that decimal places are valid

            let atoms = Atoms(1);

            // Less than 6 decimals
            let decimals = 5;
            assert!(atoms.to_raw_atoms(decimals).is_err());

            // More than 18 decimals
            let decimals = 19;
            assert!(atoms.to_raw_atoms(decimals).is_err());

            // 6 decimals
            let decimals = 6;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            assert_eq!(raw_atoms.to_clamped_u128(), 1);

            // 7 decimals
            let decimals = 7;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            assert_eq!(raw_atoms.to_clamped_u128(), 10);

            // 18 decimals
            let decimals = 18;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            assert_eq!(raw_atoms.to_clamped_u128(), 10u128.pow(12));
        }

        #[test]
        fn test_max_atoms() {
            let atoms = Atoms(u64::MAX);

            // 6 decimals
            let decimals = 6;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            assert_eq!(raw_atoms.to_clamped_u128(), u64::MAX as u128);

            // 7 decimals
            let decimals = 7;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            assert_eq!(raw_atoms.to_clamped_u128(), u64::MAX as u128 * 10);

            // 18 decimals
            let decimals = 18;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            assert_eq!(
                raw_atoms.to_clamped_u128(),
                u64::MAX as u128 * 10u128.pow(12)
            );
        }
    }
}
