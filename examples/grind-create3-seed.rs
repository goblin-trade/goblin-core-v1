use alloy_primitives::{address, keccak256, Address, B256, U256};
use hex_literal::hex;
use rayon::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const DEPLOYER: Address = address!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
const FACTORY_ADDRESS: Address = address!("A6E41fFD769491a42A6e5Ce453259b93983a22EF");
const PROXY_BYTECODE: [u8; 16] = hex!("67363d3d37363d34f03d5260086018f3");
const DESIRED_PREFIX: [u8; 2] = hex!("8888"); // Define desired prefix as bytes

/// Namespace the salt by hashing the deployer address with the provided salt.
fn namespace_salt(deployer: Address, salt: B256) -> B256 {
    keccak256([deployer.as_slice(), salt.as_slice()].concat()).into()
}

/// Generate a CREATE3 address given the factory, deployer, salt, and proxy bytecode hash.
fn get_create3_address(
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

/// Search for a salt that produces an address with the desired prefix.
fn find_salt(
    factory: Address,
    deployer: Address,
    proxy_bytecode_hash: B256,
    desired_prefix: &[u8],
) -> Option<B256> {
    let found = Arc::new(AtomicBool::new(false));

    (0u64..u64::MAX).into_par_iter().find_map_any(|i| {
        if found.load(Ordering::Relaxed) {
            return None;
        }

        let salt = B256::from(U256::try_from(i).unwrap());
        let address = get_create3_address(factory, deployer, salt, proxy_bytecode_hash);

        if address.as_slice().starts_with(desired_prefix) {
            println!("Found address {:?} for salt {:?}", address, salt);
            found.store(true, Ordering::Relaxed);
            Some(salt)
        } else {
            None
        }
    })
}

fn main() {
    let proxy_bytecode_hash = keccak256(PROXY_BYTECODE);

    println!("Starting search for CREATE3 salt...");

    match find_salt(
        FACTORY_ADDRESS,
        DEPLOYER,
        proxy_bytecode_hash.into(),
        &DESIRED_PREFIX,
    ) {
        Some(salt) => println!("Found matching salt: {:?}", salt),
        None => println!("No matching salt found."),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_address_for_salt() {
        let salt = B256::new(hex!(
            "0000000000000000000000000000000000000000000000000000000000000001"
        ));
        let proxy_bytecode_hash = keccak256(PROXY_BYTECODE);

        let address = get_create3_address(FACTORY_ADDRESS, DEPLOYER, salt, proxy_bytecode_hash);
        println!("address {:?}", address);
        // assert_eq!(
        //     address,
        //     address!("8888415db80eabcf580283a3d65249887d3161b0")
        // );
    }
}
