// Allow `cargo stylus export-abi` to generate a main function.
// main function is needed for tests, else it gives a linker error
#![cfg_attr(all(not(feature = "export-abi"), not(test)), no_main)]

extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use stylus_sdk::{
    alloy_primitives::{keccak256, Address, B256, U256},
    console, contract,
    deploy::RawDeploy,
    prelude::*,
};

sol_storage! {
    #[entrypoint]
    pub struct GoblinFactory {}
}

#[external]
impl GoblinFactory {
    pub fn initialize_market(&mut self) -> Result<(), Vec<u8>> {
        // random salt
        let salt = B256::default();
        let contract_bytes = include_bytes!("./deployment_tx_data");

        // console!("this address {:?}", contract::address());

        // expected address is correct
        let expected_address = get_create2_address(contract::address(), salt, contract_bytes);
        console!("expected address {:?}", expected_address);

        // ETH sent to contract
        let endowment = U256::from(0);

        // failing here
        let res = unsafe {
            RawDeploy::new()
                .salt(salt)
                .deploy(contract_bytes, endowment)?
        };

        // important- actual address is correct
        // console!("actual address {:?}", res);
        Ok(())
    }
}

fn get_create2_address(from: Address, salt: B256, init_code: &[u8]) -> Address {
    let init_code_hash = keccak256(init_code);

    let mut bytes = Vec::with_capacity(1 + 20 + salt.len() + init_code_hash.len());
    bytes.push(0xff);
    bytes.extend_from_slice(from.as_slice());
    bytes.extend_from_slice(salt.as_slice());
    bytes.extend_from_slice(init_code_hash.as_slice());

    let hash = keccak256(bytes.as_slice());

    let mut address_bytes = [0u8; 20];
    address_bytes.copy_from_slice(&hash[12..]);

    let address = Address::from_slice(&address_bytes);

    address
}
