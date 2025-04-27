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
///! Important- tokens with less than 6 decimal places are unsupported.
///!
///! # Math
///!
///! atoms = |raw atoms / 10^(K - 6)|
///! - For USDC = raw atoms / 10^0 = raw atoms
///! - For eth = raw atoms / 10^(18 - 6) = raw atoms / 10^12
use crate::define_custom_types;

use super::RawAtoms;

define_custom_types!(Atoms<u64>);

const MIN_DECIMALS: u8 = 6;
const MAX_DECIMALS: u8 = 19;

impl Atoms {
    pub fn from_raw_atoms(raw: &RawAtoms, decimals: u8) -> Result<Self, ()> {
        // log base 10 ((2^(128) - 1) / ((2^64) - 1)) = 19.26
        // That is if decimal places exceed 19 then u64 atoms can overflow
        // 128 bits of raw atoms. Then our optimization of skipping upper 16 bytes won't work.
        if decimals < MIN_DECIMALS || decimals > MAX_DECIMALS {
            return Err(());
        }

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
}

#[cfg(test)]
mod tests {
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
    fn test_max_value() {
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
