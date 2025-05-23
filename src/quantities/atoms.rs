use core::u64;

///! The number of atoms of a token, obtained by normalizing raw atoms.
///!
///! Every token is normalized from K decimal places to 6 decimal places.
///! * USDC (6 decimal places): no change
///! * ETH (18 decimal places): Normalized from 18 -> 6 decimal places
///!
///! This allows us to use u64 instead of U256 to represent atoms.
///! These atoms are grouped into lots.
///!
///! Important- Decimal places must be in the range [6, 19]
/// * Raw atoms will be undefined if decimal places are less than 6
/// * Since raw atoms are internally capped to 128 bits, decimal places more than
/// 19 can overflow this value.
///!
///! # Math
///!
///! atoms = |raw atoms / 10^(K - 6)|
///! - For USDC = raw atoms / 10^0 = raw atoms
///! - For eth = raw atoms / 10^(18 - 6) = raw atoms / 10^12
use crate::{define_custom_types, goblin_error::GoblinError, require};

use super::{Delta, RawAtoms};

define_custom_types!(Atoms<u64>);

const MIN_DECIMALS: u8 = 6;
const MAX_DECIMALS: u8 = 19;

impl Atoms {
    fn check_decimals(decimals: u8) -> Result<(), GoblinError> {
        // log base 10 ((2^(128) - 1) / ((2^64) - 1)) = 19.26
        // That is if decimal places exceed 19 then u64 atoms can overflow
        // 128 bits of raw atoms. Then our optimization of skipping upper 16 bytes won't work.
        require!(
            decimals >= MIN_DECIMALS && decimals <= MAX_DECIMALS,
            GoblinError::UnsupportedDecimals
        );

        Ok(())
    }

    pub fn from_raw_atoms(raw: &RawAtoms, decimals: u8) -> Result<Self, GoblinError> {
        Self::check_decimals(decimals)?;

        // If high bits are active, the value will overflow despite of division.
        // Cap to u64::MAX
        if raw.0[0] > 0 || raw.0[1] > 0 {
            return Ok(Atoms(u64::MAX));
        }

        let lower_16_bytes = &raw.to_be_bytes()[16..32];
        let raw_atoms_u128: u128 = u128::from_be_bytes(lower_16_bytes.try_into().unwrap());

        let divisor = 10u64.pow(decimals as u32 - 6);
        let atoms_u128 = raw_atoms_u128 / divisor as u128;

        let atoms = if atoms_u128 > u64::MAX as u128 {
            Atoms(u64::MAX)
        } else {
            Atoms(atoms_u128 as u64)
        };
        Ok(atoms)
    }

    pub fn to_raw_atoms(&self, decimals: u8) -> Result<RawAtoms, GoblinError> {
        Self::check_decimals(decimals)?;

        let divisor = 10u64.pow((decimals - MIN_DECIMALS) as u32) as u128;
        let raw_atoms_u128 = self.0 as u128 * divisor;

        // Create a 32-byte array initialized with zeros
        let mut bytes = [0u8; 32];

        // Fill the lower 16 bytes with the u128 value in big-endian format
        let raw_atoms_be_bytes = raw_atoms_u128.to_be_bytes();
        bytes[16..32].copy_from_slice(&raw_atoms_be_bytes);

        // Convert the byte array to RawAtoms
        Ok(RawAtoms::from_be_bytes(&bytes))
    }

    pub fn to_delta(self) -> Result<Delta, GoblinError> {
        if self.0 <= i64::MAX as u64 {
            Ok(Delta(self.0 as i64))
        } else {
            Err(GoblinError::AtomOverflow)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod raw_atoms_to_atoms {
        use super::*;

        #[test]
        fn test_raw_atoms_to_atoms() {
            // Less than 6 decimals
            let raw = RawAtoms([0, 0, 0, 1000000u64.swap_bytes()]);
            let atoms_result = Atoms::from_raw_atoms(&raw, 5);
            assert!(atoms_result.is_err());

            // USDC (6 decimals)
            let raw = RawAtoms([0, 0, 0, 1000000u64.swap_bytes()]);
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, 1000000);

            // ETH (18 decimals)
            let raw = RawAtoms([0, 0, 0, 10u64.pow(18).swap_bytes()]);
            let atoms = Atoms::from_raw_atoms(&raw, 18).unwrap();
            assert_eq!(atoms.0, 1000000);
        }

        #[test]
        fn test_dust() {
            // ETH (18 decimals)
            let raw = RawAtoms([0, 0, 0, 1_000_000u64.swap_bytes()]);
            let atoms = Atoms::from_raw_atoms(&raw, 18).unwrap();
            assert_eq!(atoms.0, 0);
        }

        #[test]
        fn test_max_value() {
            let raw = RawAtoms([u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, u64::MAX);

            let raw = RawAtoms([1u64.swap_bytes(), 0, 0, 0]);
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, u64::MAX);

            let raw = RawAtoms([0, 1u64.swap_bytes(), 0, 0]);
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, u64::MAX);

            let raw = RawAtoms([0, 0, 1u64.swap_bytes(), 0]);
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, u64::MAX);

            // Just below max value
            let raw = RawAtoms([0, 0, 0, (u64::MAX - 1).swap_bytes()]);
            let atoms = Atoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.0, u64::MAX - 1);

            // Just below max value for ETH (18 decimals)
            // Calculate raw_atoms = (u64::MAX - 1) * 10^12
            let eth_raw_atoms = (u64::MAX as u128 - 1) * 10u128.pow(12);

            // Create a properly formatted raw atoms array
            let mut raw_atoms_arr = [0u64; 4];

            // The u128 value will occupy the lower 16 bytes (bytes 16-31)
            // Split the u128 into two u64 values (for the last two positions in the array)
            raw_atoms_arr[2] = (((eth_raw_atoms >> 64) & u64::MAX as u128) as u64).swap_bytes();
            raw_atoms_arr[3] = ((eth_raw_atoms & u64::MAX as u128) as u64).swap_bytes();

            let raw = RawAtoms(raw_atoms_arr);
            let atoms = Atoms::from_raw_atoms(&raw, 18).unwrap();
            assert_eq!(atoms.0, u64::MAX - 1);
        }
    }

    mod atoms_to_raw_atoms {
        use super::*;

        fn get_raw_atom_limbs(raw_atoms_u128: u128) -> [u64; 4] {
            let high = (raw_atoms_u128 >> 64) as u64;
            let low = (raw_atoms_u128 & u64::MAX as u128) as u64;

            [0, 0, high.swap_bytes(), low.swap_bytes()]
        }

        #[test]
        fn test_atoms_to_raw_atoms() {
            // atoms will convert to raw atoms without any loss of data
            // We just need to ensure that decimal places are valid

            let atoms = Atoms(1);

            // Less than 6 decimals
            let decimals = 5;
            assert!(atoms.to_raw_atoms(decimals).is_err());

            // More than 19 decimals
            let decimals = 20;
            assert!(atoms.to_raw_atoms(decimals).is_err());

            // 6 decimals
            let decimals = 6;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            assert_eq!(raw_atoms.0, [0, 0, 0, 1u64.swap_bytes()]);

            // 7 decimals
            let decimals = 7;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            assert_eq!(raw_atoms.0, [0, 0, 0, 10u64.swap_bytes()]);

            // 19 decimals- value takes 2 limbs
            let decimals = 19;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();

            // 10^13
            let raw_atoms_u128 = 10u128.pow(13);
            assert_eq!(raw_atoms.0, get_raw_atom_limbs(raw_atoms_u128));
        }

        #[test]
        fn test_max_atoms() {
            let atoms = Atoms(u64::MAX);

            // 6 decimals
            let decimals = 6;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            assert_eq!(raw_atoms.0, [0, 0, 0, u64::MAX.swap_bytes()]);

            // 7 decimals
            let decimals = 7;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            let raw_atoms_u128 = u64::MAX as u128 * 10;
            assert_eq!(raw_atoms.0, get_raw_atom_limbs(raw_atoms_u128));

            // 19 decimals
            let decimals = 19;
            let raw_atoms = atoms.to_raw_atoms(decimals).unwrap();
            let raw_atoms_u128 = u64::MAX as u128 * 10u128.pow(19 - 6);
            assert_eq!(raw_atoms.0, get_raw_atom_limbs(raw_atoms_u128));
        }
    }
}
