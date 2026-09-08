use keccak_const::Keccak256;

/// Computes an Ethereum ABI function selector, i.e. the first 4 bytes of
/// `keccak256(signature)`.
pub const fn abi_selector(signature: &[u8]) -> [u8; 4] {
    let hash = Keccak256::new().update(signature).finalize();
    let mut result = [0u8; 4];
    let mut i = 0;
    while i < 4 {
        result[i] = hash[i];
        i += 1;
    }
    result
}
