use super::{Lots, HIGH_LOTS_SCALE};

/// The number of atoms as `U256` in **big endian**. It represents the amount of wei or
/// the amount of ERC20 tokens.
///
/// * This type is used for hostio calls, e.g. when reading wei from `msg_value()` or
/// when making ERC20 transfers.
///
/// * It holds numbers in big endian which is EVM's wire format.
///
/// * Using [u64; 4] instead of [u8; 32] produces smaller bytecode.
///
/// * Call `unsafe { &*(amount.0.as_ptr() as *const [u8; 32]) }` to convert it to `[u8; 32]`.
/// We don't provide a getter function for bytes because it can produce a dangling reference.
///
#[derive(Default)]
pub struct Atoms(pub [u64; 4]);

impl Atoms {
    /// Converts the `Atoms` struct to a `[u8; 32]` array in big-endian format.
    pub fn to_be_bytes(&self) -> &[u8; 32] {
        unsafe { &*(self.0.as_ptr() as *const [u8; 32]) }
    }
}

impl From<&Lots> for Atoms {
    /// Convert lots to atoms
    ///
    /// * Lots are stored in little endian format while Atoms are in big endian
    /// * 1 lot = 10^6 atoms
    /// * The conversion preserves the relationship: from_lots(to_lots(atoms)) == atoms
    ///   for values within the supported range
    ///
    /// # Formula
    /// * Input: lots in little endian
    /// * For the high word: lots / HIGH_LOTS_SCALE (where HIGH_LOTS_SCALE = 2^64 / 10^6)
    /// * For the low word: (lots % HIGH_LOTS_SCALE) * 10^6
    /// * Convert both to big endian by swapping bytes
    /// * Store in [u64; 4] array
    fn from(lots: &Lots) -> Self {
        let lots_value = lots.0;

        // Split into high and low components using HIGH_LOTS_SCALE
        // lots = high * HIGH_LOTS_SCALE + low / 10^6
        // - high = lots / HIGH_LOTS_SCALE
        // - lot = remainder of above division * 10^6
        let high = lots_value / HIGH_LOTS_SCALE;
        let low = (lots_value % HIGH_LOTS_SCALE) * 1_000_000;

        // Convert to big endian format
        Atoms([
            0,                 // Most significant word is always 0
            0,                 // Second word is always 0
            high.swap_bytes(), // Third word contains high bits
            low.swap_bytes(),  // Least significant word contains low bits
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_to_bytes() {
        let atoms = Atoms([0, 0, 0, 1u64.swap_bytes()]);
        let bytes: &[u8; 32] = unsafe { &*(atoms.0.as_ptr() as *const [u8; 32]) };

        let mut expected_bytes = [0u8; 32];
        expected_bytes[31] = 1;
        assert_eq!(*bytes, expected_bytes);
    }

    mod test_atom_to_lot_conversions {
        use super::*;

        #[test]
        fn test_basic_conversion() {
            let zero_lots = Lots(0);
            let atoms = Atoms::from(&zero_lots);
            assert_eq!(atoms.0, [0, 0, 0, 0]);

            let one_lot = Lots(1);
            let atoms = Atoms::from(&one_lot);
            assert_eq!(atoms.0[3].swap_bytes(), 1_000_000);
            assert_eq!(atoms.0[0], 0);
            assert_eq!(atoms.0[1], 0);
            assert_eq!(atoms.0[2], 0);

            // 2 lots = 2_500_000 atoms
            let two_lots = Lots(2);
            let atoms = Atoms::from(&two_lots);
            assert_eq!(atoms.0[3].swap_bytes(), 2_000_000);
        }

        #[test]
        fn test_large_values() {
            // Test with SCALE value
            let scale_lots = Lots(HIGH_LOTS_SCALE);
            let atoms = Atoms::from(&scale_lots);
            assert_eq!(atoms.0[2].swap_bytes(), 1);
            assert_eq!(atoms.0[3], 0);

            // Test with SCALE + 1
            let scale_plus_one = Lots(HIGH_LOTS_SCALE + 1);
            let atoms = Atoms::from(&scale_plus_one);
            assert_eq!(atoms.0[2].swap_bytes(), 1);
            assert_eq!(atoms.0[3].swap_bytes(), 1_000_000);
        }

        #[test]
        fn test_roundtrip() {
            // Test that converting from lots to atoms and back preserves the value
            let original_lots = Lots(123456);
            let atoms = Atoms::from(&original_lots);
            let roundtrip_lots = Lots::from(&atoms);
            assert_eq!(original_lots.0, roundtrip_lots.0);
        }
    }
}
