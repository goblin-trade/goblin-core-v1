// Allow `cargo stylus export-abi` to generate a main function.
// main function is needed for tests, else it gives a linker error
#![cfg_attr(all(not(feature = "export-abi"), not(test)), no_main)]

extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use stylus_sdk::{
    alloy_primitives::{B256, U256},
    console,
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
        // let salt_val: [u8; 32] = [
        //     33, 195, 93, 190, 27, 52, 74, 36, 136, 207, 51, 33, 214, 206, 84, 47, 142, 159, 48, 85, 68,
        //     255, 9, 228, 153, 58, 98, 49, 154, 73, 124, 31,
        // ];
        // let salt = B256::new(salt_val);

        let salt = B256::default();

        let contract_bytes = include_bytes!("./deployment_tx_data");

        // Read from .wasm file
        // let creation_code: [u8; 1] = [1];

        // ETH sent to contract
        let endowment = U256::from(0);

        let res = unsafe {
            RawDeploy::new()
                .salt(salt)
                .deploy(contract_bytes, endowment)?
        };

        console!("deployed at {:?}", res);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use ethers::{
        types::Address,
        utils::{get_create2_address, keccak256},
    };
    use std::str::FromStr;
    use stylus_sdk::alloy_primitives::{address, B256};

    #[test]
    fn read_wasm_bytes() {
        // println!("ok");
        let bytes = include_bytes!("./deployment_tx_data");

        let hex_bytes = hex::encode(bytes);
        println!("bytes {:?}", hex_bytes);
    }

    #[test]
    fn get_deployed_address() {
        // 0xff16c0c231f5d3fd55d4b8e1373885218d1ccd4d
        let bytes = include_bytes!("./deployment_tx_data");

        // 0xe0830031474b01dc77eade766d15e5af018bf941
        // let bytes = include_bytes!("./goblin_market.wasm");

        // let salt = B256::default();
        let salt = B256::new([0u8; 32]);

        let this_address = Address::from_str("0x525c2aBA45F66987217323E8a05EA400C65D06DC").unwrap();

        let create2_address = get_create2_address(this_address, salt, bytes);

        println!("address {:?}", create2_address);
    }


}

// #[cfg(test)]
// mod test_native {
//     use std::str::FromStr;
//     use stylus_sdk::alloy_primitives::{address, keccak256, Address, B256};

//     fn get_create2(from: Address, salt: B256, init_code: &[u8]) -> Address {
//         let init_code_hash = keccak256(init_code);

//         let mut bytes = Vec::with_capacity(1 + 20 + salt.len() + init_code_hash.len());
//         bytes.push(0xff);
//         bytes.extend_from_slice(from.as_slice());
//         bytes.extend_from_slice(salt.as_slice());
//         bytes.extend_from_slice(init_code_hash.as_slice());

//         let hash = keccak256(bytes);

//         let mut address_bytes = [0u8; 20];
//         address_bytes.copy_from_slice(&hash[12..]);

//         let address = Address::from_slice(&address_bytes);

//         address
//     }

//     #[test]
//     fn get_deployed_address_native() {
//         let bytes = include_bytes!("./deployment_tx_data");
//         let salt = B256::new([0u8; 32]);

//         let this_address = address!("525c2aBA45F66987217323E8a05EA400C65D06DC");

//         let create2_address = get_create2(this_address, salt, bytes);

//         println!("address {:?}", create2_address);
//     }
// }


