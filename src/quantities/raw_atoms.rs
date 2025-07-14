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
/// # Behavior
///
/// * The underlying bytes `[u8; 32]` are stored in big-endian format.
/// * However we are interpreting them as `[u64; 4]` using an unsafe cast. Therefore each u64 bin
/// interprets its `[u8; 8]` slice in little-endian format.
///
#[derive(Default)]
pub struct RawAtoms(pub [u8; 32]);

// impl RawAtoms {
//     /// Casts the `Raw Atoms` struct to a `[u8; 32]` array in big-endian format.
//     pub fn to_be_bytes(&self) -> &[u8; 32] {
//         unsafe { &*(self.0.as_ptr() as *const [u8; 32]) }
//     }

//     pub fn from_be_bytes(bytes: &[u8; 32]) -> Self {
//         let limbs = unsafe { &*(bytes.as_ptr() as *const [u64; 4]) };
//         RawAtoms(*limbs)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_conversion_to_bytes() {
//         // We wish to store the value 1. In big endian, the MSB goes at index 0 and LSB at index 7.
//         // We are posting the smallest value 1 at the highest index.
//         let bin_0: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 1];
//         let bin_0_u64 = u64::from_be_bytes(bin_0);
//         assert_eq!(bin_0_u64, 1);

//         // Unsafe casts act like lower endian conversions
//         // This is equivalent to u64::from_le_bytes(bin_0)
//         let bin_0_casted: &u64 = unsafe { &*(bin_0.as_ptr() as *const u64) };
//         assert_eq!(*bin_0_casted, 1u64.swap_bytes());
//         assert_eq!(*bin_0_casted, u64::from_le_bytes(bin_0));

//         let atoms = RawAtoms([0, 0, 0, *bin_0_casted]);
//         let bytes: &[u8; 32] = unsafe { &*(atoms.0.as_ptr() as *const [u8; 32]) };

//         let mut expected_bytes = [0u8; 32];
//         expected_bytes[31] = 1;
//         assert_eq!(*bytes, expected_bytes);
//     }
// }
