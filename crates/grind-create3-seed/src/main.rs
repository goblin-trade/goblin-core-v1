use alloy_primitives::{address, keccak256, Address, B256, U256};
use hex_literal::hex;
use rayon::prelude::*;
use std::io::Read;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

const DEPLOYER: Address = address!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
const FACTORY_ADDRESS: Address = address!("A6E41fFD769491a42A6e5Ce453259b93983a22EF");
const PROXY_BYTECODE: [u8; 16] = hex!("67363d3d37363d34f03d5260086018f3");

/// Namespace the salt by hashing the deployer address with the provided salt.
/// This is correct
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
        )[..20],
    );

    Address::from_slice(
        &keccak256([&[0xd6, 0x94], proxy_address.as_slice(), &[0x01]].concat())[..20],
    )
}

/// Search for a salt that produces an address with the desired prefix, namespaced by the deployer.
fn find_salt(
    factory: Address,
    deployer: Address,
    proxy_bytecode_hash: B256,
    prefix: &str,
) -> Option<B256> {
    let found = Arc::new(AtomicBool::new(false));

    (0u64..u64::MAX).into_par_iter().find_map_any(|i| {
        if found.load(Ordering::Relaxed) {
            return None;
        }

        let salt = B256::from(U256::try_from(i).unwrap());
        let address = get_create3_address(factory, deployer, salt, proxy_bytecode_hash);
        let address_str = format!("{:?}", address);

        if address_str.starts_with(prefix) {
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
    let desired_prefix = "0x8888";

    println!("Starting search for CREATE3 salt...");

    match find_salt(
        FACTORY_ADDRESS,
        DEPLOYER,
        proxy_bytecode_hash.into(),
        desired_prefix,
    ) {
        Some(salt) => println!("Found matching salt: {:?}", salt),
        None => println!("No matching salt found."),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_proxy_bytecode_hash() {
        let proxy_bytecode_hash = keccak256(PROXY_BYTECODE);

        // This is valid
        // smart contract logs 15271506168544636618683946165347184908672584999956201311530805028234774281247
        // too
        let hash_num = U256::try_from(proxy_bytecode_hash).unwrap();
        println!("hash num {:?}", hash_num);
    }

    // This is also correct
    #[test]
    fn test_salt_with_deployer() {
        let salt = B256::new(hex!(
            "0000000000000000000000000000000000000000000000004000000000005443"
        ));
        let salt_with_deployer = namespace_salt(DEPLOYER, salt);
        println!(
            "salt_with_deployer {:?}",
            U256::try_from(salt_with_deployer).unwrap()
        );
    }

    #[test]
    fn test_address_for_salt() {
        let salt = B256::new(hex!(
            "0000000000000000000000000000000000000000000000004000000000005443"
        ));
        let proxy_bytecode_hash = keccak256(PROXY_BYTECODE);

        let address = get_create3_address(FACTORY_ADDRESS, DEPLOYER, salt, proxy_bytecode_hash);
        println!("address {:?}", address);
    }
}
