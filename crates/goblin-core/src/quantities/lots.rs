///! Token lot size conversion (1 lot = 10^6 atoms)
///!
///! # Limitations
///! * Max value: u64::MAX * 10^6 atoms (capped to u64::MAX lots)
///! * Min value: Dust < 10^6 atoms is truncated
///! * Only supports fungible tokens
///!
///! Input atoms are big-endian.
use crate::define_custom_types;

const SCALE: u64 = 18446744073709; // (2^64 / 10^6)

define_custom_types!(Lots<u64>);

impl Lots {
    /// Converts big-endian atoms to lots, handling endianness conversion
    /// and potential overflow
    pub fn from_atoms(atoms: &[u64; 4]) -> Self {
        let high = atoms[2].swap_bytes();
        let low = atoms[3].swap_bytes();

        let high_lots = high.wrapping_mul(SCALE);
        let low_lots = low / 1_000_000;

        Lots(high_lots.wrapping_add(low_lots))
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_with_hex_literals() {
        let msg_value_bytes =
            hex!("0000000000000000000000000000000000000000000000000000000000000000");
        let msg_value: &[u64; 4] = unsafe { &*(msg_value_bytes.as_ptr() as *const [u64; 4]) };
        assert_eq!(Lots::from_atoms(&msg_value).0, 0);

        // 10^6 = 0xF4240
        let msg_value_bytes =
            hex!("00000000000000000000000000000000000000000000000000000000000F4240");
        let msg_value: &[u64; 4] = unsafe { &*(msg_value_bytes.as_ptr() as *const [u64; 4]) };
        assert_eq!(Lots::from_atoms(&msg_value).0, 1);
    }

    #[test]
    fn test_basic_conversion() {
        // 1_000_000 in big-endian
        assert_eq!(Lots::from_atoms(&[0, 0, 0, 1_000_000u64.swap_bytes()]).0, 1);
        // 2_500_000 in big-endian
        assert_eq!(Lots::from_atoms(&[0, 0, 0, 2_500_000u64.swap_bytes()]).0, 2);
    }

    #[test]
    fn test_dust_handling() {
        // 999_999 in big-endian
        assert_eq!(Lots::from_atoms(&[0, 0, 0, 999_999u64.swap_bytes()]).0, 0);
    }

    #[test]
    fn test_large_values() {
        // 1 in position 2 (big-endian)
        assert_eq!(Lots::from_atoms(&[0, 0, 1u64.swap_bytes(), 0]).0, SCALE);

        assert_eq!(
            Lots::from_atoms(&[0, 0, 1u64.swap_bytes(), 1_000_000u64.swap_bytes()]).0,
            SCALE + 1
        );
    }

    #[test]
    fn test_overflow() {
        assert_eq!(
            Lots::from_atoms(&[0, 0, u64::MAX.swap_bytes(), u64::MAX.swap_bytes()]).0,
            0
        );
    }
}
