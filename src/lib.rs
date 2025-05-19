#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::mem::MaybeUninit;
use getter::*;
use goblin_error::*;
use handler::*;
use hostio::*;
use token_delta::TokenDeltaList;

pub mod call;
pub mod erc20;
pub mod eth;
pub mod events;
pub mod getter;
pub mod goblin_error;
pub mod handler;
pub mod hostio;
pub mod market_params;
pub mod quantities;
pub mod state;
pub mod token_delta;
pub mod types;

#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub mod indexer_hostio;

pub const ADDRESS: [u8; 20] = [
    0x88, 0x88, 0x41, 0x5d, 0xb8, 0x0e, 0xab, 0xcf, 0x58, 0x02, 0x83, 0xa3, 0xd6, 0x52, 0x49, 0x88,
    0x7d, 0x31, 0x61, 0xb0,
];

fn user_entrypoint_inner(len: usize) -> Result<(), GoblinError> {
    require!(len > 0, GoblinError::InvalidPayload);

    // TODO exit if re-entrant

    let mut input = MaybeUninit::<[u8; 512]>::uninit();
    let input = unsafe {
        read_args(input.as_mut_ptr() as *mut u8);
        input.assume_init_ref()
    };

    let delta_list = &mut TokenDeltaList::default();

    let num_calls = input[0] as usize;
    let mut offset = 1;

    for _ in 0..num_calls {
        // Invalid input: not enough bytes for selector
        require!(offset < len, GoblinError::InvalidPayload);

        let selector = input[offset];
        offset += 1;

        // This is the entire payload from offset to end length. We need
        // to shorten the payload
        let payload = &input[offset..len];
        let bytes_used = match selector {
            HANDLE_0_CREDIT_ETH => handle_0_credit_eth(payload, delta_list),
            // HANDLE_1_CREDIT_ERC20 => handle_1_credit_erc20(payload),
            // HANDLE_2_WITHDRAW_ETH => handle_2_withdraw_eth(payload),
            // HANDLE_3_WITHDRAW_ERC20 => handle_3_withdraw_erc20(payload),
            // HANDLE_4_PLACE_MULTIPLE_ORDERS => handle_4_place_multiple_orders(payload),
            // // Getters
            // GET_10_TRADER_TOKEN_STATE => get_10_trader_token_state(payload),
            _ => Err(GoblinError::InvalidSelector),
        }?;
        offset += bytes_used;
    }

    // TODO study re-entrancy. The SDK flushes before cross contract calls only
    // in re-entrant mode. If we disable re-entrancy, we could reduce the number of calls.
    unsafe {
        hostio::storage_flush_cache(true);
    }

    Ok(())
}

#[no_mangle]
pub extern "C" fn user_entrypoint(len: usize) -> i32 {
    match user_entrypoint_inner(len) {
        Ok(_) => 0,
        Err(err) => err.code(),
    }
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
