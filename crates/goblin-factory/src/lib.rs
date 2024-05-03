// Allow `cargo stylus export-abi` to generate a main function.
// main function is needed for tests, else it gives a linker error
#![cfg_attr(all(not(feature = "export-abi"), not(test)), no_main)]

extern crate alloc;

/// Use an efficient WASM allocator.
#[global_allocator]
static ALLOC: mini_alloc::MiniAlloc = mini_alloc::MiniAlloc::INIT;

use stylus_sdk::{alloy_primitives::{B256, U256}, console, deploy::RawDeploy, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct GoblinFactory {}
}

#[external]
impl GoblinFactory {
    pub fn initialize_market(&mut self) -> Result<(), Vec<u8>> {
        // random salt
        let salt_val: [u8; 32] = [
            33, 195, 93, 190, 27, 52, 74, 36, 136, 207, 51, 33, 214, 206, 84, 47, 142, 159, 48, 85, 68,
            255, 9, 228, 153, 58, 98, 49, 154, 73, 124, 31,
        ];
        let salt = B256::new(salt_val);

        let contract_bytes = include_bytes!("./goblin_market.wasm");

        // Read from .wasm file
        // let creation_code: [u8; 1] = [1];

        // ETH sent to contract
        let endowment = U256::from(0);

        let res = unsafe { RawDeploy::new().salt(salt).deploy(contract_bytes, endowment)? };

        console!("deployed at {:?}", res);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_wasm_bytes() {
        // println!("ok");
        let bytes = include_bytes!("./goblin_market.wasm");
        println!("bytes {:?}", bytes);
    }
}
