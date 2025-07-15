/// The number of raw atoms as `U256` in **big endian**. It represents the amount of wei or
/// the amount of ERC20 tokens.
///
/// * This type is used for hostio calls, e.g. when reading wei from `msg_value()` or
/// when making ERC20 transfers.
///
/// * It holds numbers in big endian which is EVM's wire format.
#[derive(Default)]
pub struct RawAtoms(pub [u8; 32]);

impl RawAtoms {
    /// Convert RawAtoms to a clamped u128.
    /// Values are clamped to u128::MAX if they exceed the maximum representable value.
    ///
    /// Since bytes are in big endian, we cannot unsafe cast into [u128; 2].
    pub fn to_clamped_u128(&self) -> u128 {
        let upper_bytes = unsafe { &*(self.0.as_ptr() as *const [u8; 16]) };
        let lower_bytes = unsafe { &*(self.0.as_ptr().add(16) as *const [u8; 16]) };

        if *upper_bytes != [0u8; 16] {
            u128::MAX
        } else {
            u128::from_be_bytes(*lower_bytes)
        }
    }

    #[cfg(test)]
    pub const MAX: RawAtoms = RawAtoms([0xff; 32]);

    #[cfg(test)]
    pub fn from_u128(value: u128) -> Self {
        let mut raw_atoms = RawAtoms([0u8; 32]);
        let raw_atom_bytes = value.to_be_bytes();
        raw_atoms.0[16..].copy_from_slice(&raw_atom_bytes);
        raw_atoms
    }
}

#[cfg(test)]
mod tests {
    use core::u128;

    use super::*;

    #[test]
    fn test_zero_raw_atoms() {
        let raw_atoms = RawAtoms([0u8; 32]);
        assert_eq!(raw_atoms.to_clamped_u128(), 0);
    }

    #[test]
    fn test_small_value() {
        let mut raw_atoms = RawAtoms([0u8; 32]);
        raw_atoms.0[31] = 1;
        assert_eq!(raw_atoms.to_clamped_u128(), 1);

        let mut raw_atoms = RawAtoms([0u8; 32]);
        let expected_value = 100u128;
        let expected_bytes = expected_value.to_be_bytes();
        raw_atoms.0[16..].copy_from_slice(&expected_bytes);
        assert_eq!(raw_atoms.to_clamped_u128(), expected_value);
    }

    #[test]
    fn test_clamping() {
        let mut raw_atoms = RawAtoms([0u8; 32]);
        raw_atoms.0[15] = 1;
        assert_eq!(raw_atoms.to_clamped_u128(), u128::MAX);
    }
}
