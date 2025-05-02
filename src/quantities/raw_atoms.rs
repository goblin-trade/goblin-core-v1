/// The number of raw atoms as `U256` in **big endian**. It represents the amount of wei or
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
pub struct RawAtoms(pub [u64; 4]);

impl RawAtoms {
    /// Casts the `Raw Atoms` struct to a `[u8; 32]` array in big-endian format.
    pub fn to_be_bytes(&self) -> &[u8; 32] {
        unsafe { &*(self.0.as_ptr() as *const [u8; 32]) }
    }

    pub fn from_be_bytes(bytes: &[u8; 32]) -> Self {
        let limbs = unsafe { &*(bytes.as_ptr() as *const [u64; 4]) };
        RawAtoms(*limbs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion_to_bytes() {
        let atoms = RawAtoms([0, 0, 0, 1u64.swap_bytes()]);
        let bytes: &[u8; 32] = unsafe { &*(atoms.0.as_ptr() as *const [u8; 32]) };

        let mut expected_bytes = [0u8; 32];
        expected_bytes[31] = 1;
        assert_eq!(*bytes, expected_bytes);
    }
}
