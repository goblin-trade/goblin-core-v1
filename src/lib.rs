#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

use core::mem::MaybeUninit;
use getter::*;
use goblin_error::*;
use hostio::*;
use instructions::*;
use settlement::{EthDelta, TokenDeltaList};
use types::Address;

pub mod erc20;
pub mod eth;
pub mod events;
pub mod getter;
pub mod goblin_error;
pub mod hostio;
pub mod instructions;
pub mod market_params;
pub mod quantities;
pub mod settlement;
pub mod state;
pub mod token_addresses;
pub mod types;

#[cfg(all(not(test), not(target_arch = "wasm32")))]
pub mod indexer_hostio;

pub const ADDRESS: [u8; 20] = [
    0x88, 0x88, 0x41, 0x5d, 0xb8, 0x0e, 0xab, 0xcf, 0x58, 0x02, 0x83, 0xa3, 0xd6, 0x52, 0x49, 0x88,
    0x7d, 0x31, 0x61, 0xb0,
];

fn user_entrypoint_inner(len: usize) -> Result<(), GoblinError> {
    let msg_reentrant = unsafe { hostio::msg_reentrant() };
    require!(!msg_reentrant, GoblinError::Reentrant);

    require!(len > 0, GoblinError::InvalidPayload);

    let mut input_maybe = MaybeUninit::<[u8; 512]>::uninit();
    let input = unsafe {
        read_args(input_maybe.as_mut_ptr() as *mut u8);
        input_maybe.assume_init_ref()
    };

    let eth_delta = &mut EthDelta::default();
    let erc20_deltas = &mut TokenDeltaList::default();

    // input[0] is the header byte
    //
    // * Pos 0 bit tells whether to deposit ETH
    // * Pos 1 tells whether a recipient is provided, otherwise the recipient is msg.sender
    // * Pos 2 tells whether to transfer to recipient internally
    //   - Value is ignored if recipient is not provided
    //   - If true, then `withdrawal_due` is credited internally to recipient's TraderTokenState
    //   - If false, the amount is withdrawn to the recipient
    //
    // * Remaining MSB 5 bits give the number of calls. The max value
    // is 2^5 - 1 = 31
    //
    let header_byte = input[0];
    let deposit_native_token = (header_byte & 0b0000_0001) != 0;
    let recipient_provided = (header_byte & 0b0000_0010) != 0;
    let transfer_to_recipient_internally = (header_byte & 0b0000_0100) != 0;

    let num_calls = (header_byte >> 3) as usize;

    if deposit_native_token {
        ix_deposit_eth(eth_delta)?;
    }

    let mut msg_sender_maybe = MaybeUninit::<Address>::uninit();
    let msg_sender = unsafe {
        hostio::msg_sender(msg_sender_maybe.as_mut_ptr() as *mut u8);
        msg_sender_maybe.assume_init_ref()
    };

    let mut offset = 1;

    let recipient = if recipient_provided {
        offset += 20;
        require!(len > offset, GoblinError::InvalidPayload);
        unsafe { &*(input[1..offset].as_ptr() as *const Address) }
    } else {
        msg_sender
    };

    for _ in 0..num_calls {
        // Invalid input: not enough bytes for selector
        require!(len > offset, GoblinError::InvalidPayload);

        let selector = input[offset];
        offset += 1;

        let payload = &input[offset..len];
        let bytes_used = match selector {
            IX_0_WITHDRAW_ETH => ix_0_withdraw_eth(payload, eth_delta),
            // IX_1_DEPOSIT_ERC20 => ix_1_credit_erc20(payload),
            // IX_2_WITHDRAW_ERC20 => ix_2_withdraw_erc20(payload),
            // IX_3_PLACE_MULTIPLE_ORDERS => ix_3_place_multiple_orders(payload),
            // Getters
            // GET_10_TRADER_TOKEN_STATE => get_10_trader_token_state(payload),
            _ => Err(GoblinError::InvalidSelector),
        }?;
        offset += bytes_used;
    }

    eth_delta.settle(&msg_sender, recipient, transfer_to_recipient_internally)?;
    // TODO settle token_delta_list

    // Write cache to trie
    // https://github.com/OffchainLabs/stylus-sdk-rs/blob/2c709a5a1a620ed7585c7d8af64fefabe3a0fc9a/stylus-sdk/src/storage/mod.rs#L81
    unsafe {
        hostio::storage_flush_cache(false);
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
