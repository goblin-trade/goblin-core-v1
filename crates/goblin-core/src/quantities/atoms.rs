/// The number of raw units ETH (i.e. wei) or the `balanceOf` value of an ERC20 token
/// in **big endian**
///
/// * This type is used for hostio calls, e.g. when reading wei from `msg_value()` or
/// when making ERC20 transfers.
///
/// * It holds numbers in big endian which is EVM's wire format.
///
/// * Using [u64; 4] instead of [u8; 32] produces smaller bytecode.
///
/// * Refer to lots.rs for conversion functions.
pub struct Atoms(pub [u64; 4]);
