#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::mem::MaybeUninit;
use getter::{get_10_trader_token_state, GET_10_PAYLOAD_LEN, GET_10_TRADER_TOKEN_STATE};
use handler::{
    handle_0_credit_eth, handle_1_credit_erc20, HANDLE_0_CREDIT_ETH, HANDLE_0_PAYLOAD_LEN,
    HANDLE_1_CREDIT_ERC20, HANDLE_1_PAYLOAD_LEN,
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

// Address 0x8888415db80eabcf580283a3d65249887d3161b0
pub const ADDRESS: [u8; 20] = [
    0x88, 0x88, 0x41, 0x5d, 0xb8, 0x0e, 0xab, 0xcf, 0x58, 0x02, 0x83, 0xa3, 0xd6, 0x52, 0x49, 0x88,
    0x7d, 0x31, 0x61, 0xb0,
];

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
        // Invalid input: not enough bytes for selector
        if offset >= len {
            return 1;
        }

        let selector = input[offset];
        offset += 1;

        let payload_len = match selector {
            HANDLE_0_CREDIT_ETH => HANDLE_0_PAYLOAD_LEN,
            HANDLE_1_CREDIT_ERC20 => HANDLE_1_PAYLOAD_LEN,
            GET_10_TRADER_TOKEN_STATE => GET_10_PAYLOAD_LEN,
            _ => return 1, // Unknown selector
        };

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
            _ => return 1,
        };

        // If any handler fails (returns nonzero), propagate the error
        if result != 0 {
            return result;
        }
    }

    0
}

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
