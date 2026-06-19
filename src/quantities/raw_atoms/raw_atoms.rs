/// The number of raw atoms as `U256` in **big endian**. It represents the amount of wei or
/// the amount of ERC20 tokens.
///
/// * This type is used for hostio calls, e.g. when reading wei from `msg_value()` or
/// when making ERC20 transfers.
///
/// * It holds numbers in big endian which is EVM's wire format.
#[derive(Default)]
pub struct RawAtoms<const D: u8>(pub [u8; 32]);

impl<const D: u8> RawAtoms<D> {
    pub const ZERO: Self = RawAtoms([0; 32]);

    #[cfg(test)]
    pub const MAX: Self = RawAtoms([0xff; 32]);

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
    pub fn from_u128(value: u128) -> Self {
        let mut raw_atoms = RawAtoms([0u8; 32]);
        let raw_atom_bytes = value.to_be_bytes();
        raw_atoms.0[16..].copy_from_slice(&raw_atom_bytes);
        raw_atoms
    }
}
