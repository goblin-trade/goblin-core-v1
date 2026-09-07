use hex_literal::hex;

#[test]
fn test_amount_encoding() {
    let amount = hex!("00000001");
    std::println!("amount {:?}", amount);
}

#[test]
fn test_encode_as_arr() {
    // cast calldata "transferFrom(address,address,uint256)" 0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E 0x84401cd7abbebb22acb7af2becfd9be56c30bcf1 1
    let calldata = hex!("23b872dd0000000000000000000000003f1eae7d46d88f08fc2f8ed27fcb2ab183eb2d0e00000000000000000000000084401cd7abbebb22acb7af2becfd9be56c30bcf10000000000000000000000000000000000000000000000000000000000000001");

    std::println!("calldata {:?}", calldata);
}

#[test]
fn test_get_token_as_arr() {
    let token = hex!("F5FfD11A55AFD39377411Ab9856474D2a7Cb697e");
    std::println!("token {:?}", token);
}

#[test]
fn test_get_contract_as_arr() {
    let token = hex!("a6e41ffd769491a42a6e5ce453259b93983a22ef");
    std::println!("token {:?}", token);
}
