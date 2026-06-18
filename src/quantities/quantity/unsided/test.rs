use crate::quantities::{RawAtoms, UnsidedAtoms};
use core::u128;

#[cfg(test)]
mod tests {
    use super::*;

    mod raw_atoms_to_atoms {

        use super::*;

        #[test]
        fn test_raw_atoms_to_atoms() {
            // Less than 6 decimals
            let raw = RawAtoms::from_u128(1_000_000);
            let atoms_result = UnsidedAtoms::from_raw_atoms(&raw, 5);
            assert!(atoms_result.is_err());

            // USDC (6 decimals)
            let raw = RawAtoms::from_u128(1_000_000);
            let atoms = UnsidedAtoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.inner, 1000000);

            // ETH (18 decimals)
            let raw = RawAtoms::from_u128(10u128.pow(18));
            let atoms = UnsidedAtoms::from_raw_atoms(&raw, 18).unwrap();
            assert_eq!(atoms.inner, 1000000);
        }

        #[test]
        fn test_dust() {
            // ETH (18 decimals)
            let raw = RawAtoms::from_u128(1_000_000);
            let atoms = UnsidedAtoms::from_raw_atoms(&raw, 18).unwrap();
            assert_eq!(atoms.inner, 0);
        }

        #[test]
        fn test_max_value() {
            // MAX possible value
            let raw = RawAtoms::MAX;
            let atoms = UnsidedAtoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.inner, u64::MAX);

            // Set MSB (0 index in big endian) to 1
            let mut raw = RawAtoms::default();
            raw.0[0] = 1;
            let atoms = UnsidedAtoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.inner, u64::MAX);

            // Smallest value greater than u128::MAX. This will get clamped to u128::MAX
            let mut raw = RawAtoms::default();
            raw.0[15] = 1;
            let atoms = UnsidedAtoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.inner, u64::MAX);

            // Just below max value of u64
            let raw = RawAtoms::from_u128(u128::MAX - 1);
            let atoms = UnsidedAtoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.inner, u64::MAX);

            // u64::MAX - 1
            let raw = RawAtoms::from_u128(u64::MAX as u128 - 1);
            let atoms = UnsidedAtoms::from_raw_atoms(&raw, 6).unwrap();
            assert_eq!(atoms.inner, u64::MAX - 1);

            // Just below max value for ETH (18 decimals)
            let raw = RawAtoms::from_u128((u64::MAX as u128 - 1) * 10u128.pow(12));
            let atoms = UnsidedAtoms::from_raw_atoms(&raw, 18).unwrap();
            assert_eq!(atoms.inner, u64::MAX - 1);
        }
    }

    mod atoms_to_raw_atoms {
        use super::*;

        #[test]
        fn test_atoms_to_raw_atoms() {
            // atoms will convert to raw atoms without any loss of data
            // We just need to ensure that decimal places are valid

            let atoms = UnsidedAtoms::new(1);

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
            let atoms = UnsidedAtoms::new(u64::MAX);

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
