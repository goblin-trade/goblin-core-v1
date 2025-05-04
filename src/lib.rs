#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use clear_cache_and_call::*;
use core::mem::MaybeUninit;
use getter::*;
use handler::*;
use hostio::*;

pub mod call;
pub mod clear_cache_and_call;
pub mod erc20;
pub mod eth;
pub mod events;
pub mod getter;
pub mod handler;
pub mod hostio;
pub mod market_params;
pub mod quantities;
pub mod state;
pub mod types;

#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub mod indexer_hostio;

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

        let payload = &input[offset..len];
        let result = match selector {
            HANDLE_0_CREDIT_ETH => handle_0_credit_eth(payload),
            HANDLE_1_CREDIT_ERC20 => handle_1_credit_erc20(payload),
            HANDLE_2_WITHDRAW_ETH => handle_2_withdraw_eth(payload),
            HANDLE_3_WITHDRAW_ERC20 => handle_3_withdraw_erc20(payload),
            HANDLE_4_PLACE_MULTIPLE_ORDERS => handle_4_place_multiple_orders(payload),

            // Getters
            GET_10_TRADER_TOKEN_STATE => get_10_trader_token_state(payload),
            _ => Err(()),
        };

        if let Ok(bytes_used) = result {
            offset += bytes_used;
        } else {
            // If any handler fails then exit
            return 1;
        }
    }

    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[cfg(all(not(test), target_arch = "wasm32"))]
#[no_mangle]
pub unsafe extern "C" fn mark_used() {
    pay_for_memory_grow(0);
    panic!();
}
