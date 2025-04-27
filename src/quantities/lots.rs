///! A lot is the smallest unit that the matching engine can process
///!
///! * 1 lot equals 10^6 globally for all tokens. 1 lot = 10^6 raw atoms.
///!
///! * Lots are u64 numbers using **little endian** encoding. This allows zero copy
///! serialization and deserialization when reading to or writing from args and slots.
///!
///! * On the other hand `RawAtoms` uses big endian. We use `RawAtoms` to read wei from `msg_value()`
///! and for making ERC20 calls. The big endian format is forced upon us by EVM.
///!
///! # Limitations
///! * Max value: u64::MAX * 10^6 raw atoms (capped to u64::MAX lots)
///! * Min value: Dust < 10^6 raw atoms is truncated
///! * Only supports fungible tokens
///! * But since USDC has 6 decimals, 1 lot = 1 unit of USDC
///! This is too big. We can't express 0.1, 0.001 ticks
///!
use crate::define_custom_types;

use super::RawAtoms;

pub const HIGH_LOTS_SCALE: u64 = 18446744073709; // (2^64 / 10^6)

define_custom_types!(Lots<u64>);

impl From<&RawAtoms> for Lots {
    /// Convert raw atoms to lots
    ///
    /// * Since RawAtoms have a size of 32 bytes while Lots have a 8 byte size,
    /// we cannot deal with large values of raw atoms. The max value of raw atoms is
    /// u64::MAX * 10^6 raw atoms (capped to u64::MAX lots).
    ///
    /// * Lots are steps of 10^6 raw atoms. Dust values lower than 10^6 raw atoms are lost.
    ///
    /// # Formula
    ///
    /// * Suppose RawAtoms = 0x000...0001
    /// * We group the bytes in 4 groups to fit in [u64; 4]
    /// * [(0x00, 0x00, ...), (0x00, 0x00, ...), (0x00, 0x00, ...), (0x00, 0x00, ..., 0x01)]
    /// * Group 2 and 3 are sufficient to max out `lots: u64`. Discard group 0 and 1.
    ///
    /// * Swap bytes to convert to little endian
    /// swap_bytes([0x00, 0x00, ..., 0x01]) = [0x01, 0x00, ...] = 1
    ///
    /// * We must divide by 10^6 to convert raw atoms to lots
    /// lots = (word_2 * 2^64 + word_3) / 10^6
    ///
    fn from(rawAtoms: &RawAtoms) -> Self {
        let high = rawAtoms.0[2].swap_bytes();
        let low = rawAtoms.0[3].swap_bytes();

        let high_lots = high.wrapping_mul(HIGH_LOTS_SCALE);
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
        assert_eq!(Lots::from(&RawAtoms(*msg_value)).0, 0);

        // 10^6 = 0xF4240
        let msg_value_bytes =
            hex!("00000000000000000000000000000000000000000000000000000000000F4240");
        let msg_value: &[u64; 4] = unsafe { &*(msg_value_bytes.as_ptr() as *const [u64; 4]) };
        assert_eq!(Lots::from(&RawAtoms(*msg_value)).0, 1);
    }

    #[test]
    fn test_basic_conversion() {
        // 1_000_000 in big-endian
        assert_eq!(
            Lots::from(&RawAtoms([0, 0, 0, 1_000_000u64.swap_bytes()])).0,
            1
        );
        // 2_500_000 in big-endian
        assert_eq!(
            Lots::from(&RawAtoms([0, 0, 0, 2_500_000u64.swap_bytes()])).0,
            2
        );
    }

    #[test]
    fn test_dust_handling() {
        // 999_999 in big-endian
        assert_eq!(
            Lots::from(&RawAtoms([0, 0, 0, 999_999u64.swap_bytes()])).0,
            0
        );
    }

    #[test]
    fn test_large_values() {
        // 1 in position 2 (big-endian)
        assert_eq!(
            Lots::from(&RawAtoms([0, 0, 1u64.swap_bytes(), 0])).0,
            HIGH_LOTS_SCALE
        );

        assert_eq!(
            Lots::from(&RawAtoms([
                0,
                0,
                1u64.swap_bytes(),
                1_000_000u64.swap_bytes()
            ]))
            .0,
            HIGH_LOTS_SCALE + 1
        );
    }

    #[test]
    fn test_overflow() {
        assert_eq!(
            Lots::from(&RawAtoms([
                0,
                0,
                u64::MAX.swap_bytes(),
                u64::MAX.swap_bytes()
            ]))
            .0,
            0
        );
    }
}
