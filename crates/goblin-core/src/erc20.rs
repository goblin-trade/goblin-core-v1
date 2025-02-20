use crate::{call_contract, log_i64, log_txt, quantities::Atoms, types::Address};

// keccak256('transferFrom(address,address,uint256)') = 0x23b872dd
const TRANSFER_FROM_SELECTOR: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];
// const TRANSFER_FROM_SELECTOR: [u8; 4] = [0, 0, 0, 0];

pub fn transfer_from(
    contract: &Address,
    // sender: &Address,
    // recipient: &Address,
    // amount: &Atoms,
) -> u8 {
    unsafe {
        let msg = "Looping in erc20.rs";
        log_txt(msg.as_ptr(), msg.len());

        for i in 0..20 {
            let byte = contract[i];
            log_i64(byte as i64);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    #[test]
    fn test_amount_encoding() {
        let amount = hex!("00000001");
        println!("amount {:?}", amount);
    }

    #[test]
    fn test_encode_as_arr() {
        // cast calldata "transferFrom(address,address,uint256)" 0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E 0x84401cd7abbebb22acb7af2becfd9be56c30bcf1 1
        let calldata = hex!("23b872dd0000000000000000000000003f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e00000000000000000000000084401cd7abbebb22acb7af2becfd9be56c30bcf10000000000000000000000000000000000000000000000000000000000000001");

        println!("calldata {:?}", calldata);
    }

    #[test]
    fn test_get_token_as_arr() {
        let token = hex!("A6E41fFD769491a42A6e5Ce453259b93983a22EF");
        println!("token {:?}", token);
    }

    #[test]
    fn test_get_sender_as_arr() {
        let token = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
        println!("token {:?}", token);
    }
}
