#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::mem::MaybeUninit;
use getter::{get_10_trader_token_state, GET_10_TRADER_TOKEN_STATE};
use handler::{
    handle_0_credit_eth, handle_1_credit_erc20, HANDLE_0_CREDIT_ETH, HANDLE_1_CREDIT_ERC20,
};
use hostio::*;

pub mod erc20;
pub mod getter;
pub mod handler;
pub mod hostio;
pub mod market_params;
pub mod quantities;
pub mod state;
pub mod types;

// Address 0xa6e41ffd769491a42a6e5ce453259b93983a22ef
// Deployer 0x3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E, nonce 0
// The contract should be deployed in the first transaction on testnet to get nonce 0
pub const ADDRESS: [u8; 20] = [
    166, 228, 31, 253, 118, 148, 145, 164, 42, 110, 92, 228, 83, 37, 155, 147, 152, 58, 34, 239,
];

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn mark_used() {
    pay_for_memory_grow(0);
    panic!();
}

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    if len == 0 {
        return 1;
    }

    let mut input = MaybeUninit::<[u8; 512]>::uninit();
    let input = unsafe {
        read_args(input.as_mut_ptr() as *mut u8);
        input.assume_init_ref()
    };

    let num_calls = input[0] as usize;
    let mut offset = 1;

    for _ in 0..num_calls {
        // Invalid input: not enough bytes for selector + length
        if offset + 2 > len {
            return 1;
        }

        let selector = input[offset];
        let payload_len = input[offset + 1] as usize;
        offset += 2;

        if offset + payload_len > len {
            // Invalid input: payload out of bounds
            return 1;
        }

        let payload = &input[offset..offset + payload_len];
        offset += payload_len;

        let result = match selector {
            HANDLE_0_CREDIT_ETH => handle_0_credit_eth(payload),
            HANDLE_1_CREDIT_ERC20 => handle_1_credit_erc20(payload),
            GET_10_TRADER_TOKEN_STATE => get_10_trader_token_state(payload),
            _ => return 1, // Unknown selector
        };

        // If any handler fails (returns nonzero), propagate the error
        if result != 0 {
            return result;
        }
    }

    0
}
