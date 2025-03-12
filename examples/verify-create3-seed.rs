use alloy_primitives::{address, keccak256, Address, B256};
use hex_literal::hex;

const DEPLOYER: Address = address!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
const FACTORY_ADDRESS: Address = address!("A6E41fFD769491a42A6e5Ce453259b93983a22EF");
const PROXY_BYTECODE: [u8; 16] = hex!("67363d3d37363d34f03d5260086018f3");

fn main() {
    let salt = B256::new(hex!(
        "0000000000000000000000000000000000000000000000006000000000001aca"
    ));
    let proxy_bytecode_hash = keccak256(PROXY_BYTECODE);

    let address = get_create3_address(FACTORY_ADDRESS, DEPLOYER, salt, proxy_bytecode_hash);
    println!("address {:?}", address);
}

pub fn get_create3_address(
    factory: Address,
    deployer: Address,
    salt: B256,
    proxy_bytecode_hash: B256,
) -> Address {
    let namespaced_salt = namespace_salt(deployer, salt);

    let proxy_address = Address::from_slice(
        &keccak256(
            [
                &[0xff],
                factory.as_slice(),
                namespaced_salt.as_slice(),
                proxy_bytecode_hash.as_slice(),
            ]
            .concat(),
        )[12..32],
    );

    Address::from_slice(
        &keccak256([&[0xd6, 0x94], proxy_address.as_slice(), &[0x01]].concat())[12..32],
    )
}

/// Namespace the salt by hashing the deployer address with the provided salt.
fn namespace_salt(deployer: Address, salt: B256) -> B256 {
    keccak256([deployer.as_slice(), salt.as_slice()].concat()).into()
}
