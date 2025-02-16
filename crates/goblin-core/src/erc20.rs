use crate::types::Address;

// keccak256('transferFrom(address,address,uint256)') = 0x23b872dd
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];

///
pub fn transfer_from(contract: &Address, sender: &Address, recipient: &Address, amount: &[u8; 32]) {
    let mut payload = [0u8; 4 + 32 * 3];

    payload[0..4].copy_from_slice(&TRANSFER_FROM_SELECTOR);

    // Addresses are in big endian so we can copy them directly
    payload[16..36].copy_from_slice(sender);
    payload[48..68].copy_from_slice(recipient);

    // Amount must be converted to big endian.
    payload[68..100].copy_from_slice(amount);
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    #[test]
    fn test_amount_encoding() {
        let amount = hex!("00000001");
        println!("amount {:?}", amount);
    }
}
