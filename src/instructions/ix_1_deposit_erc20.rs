use core::mem::MaybeUninit;

use crate::{
    erc20,
    goblin_error::GoblinError,
    msg_sender,
    quantities::Atoms,
    require,
    state::{SlotState, TraderTokenKey, TraderTokenState},
    types::Address,
    ADDRESS,
};

#[cfg(all(not(test), not(target_arch = "wasm32")))]
use crate::indexer_hostio;

pub const HANDLE_1_CREDIT_ERC20: u8 = 1;
pub const HANDLE_1_PAYLOAD_LEN: usize = core::mem::size_of::<CreditERC20Params>();

#[repr(C, packed)]
struct CreditERC20Params {
    /// The token to credit
    pub token: Address,

    /// Credit input atoms to `recipient`. This allows a wallet to fund another wallet
    pub recipient: Address,

    /// The atoms to credit. Raw atom to atom conversions should happen on client side.
    ///
    /// The atom bytes should be encoded in **little endian** for zero copy deserialization.
    ///
    /// For 1 atom
    /// - Correct (little endian, non ABI): 0x0100000000000000 = [0x01, 0x00, ...]
    /// - Wrong (big endian, ABI style): 0x0000000000000001 = [0x00, 0x00, ..., 0x01]
    pub atoms: Atoms,
}

/// Credit an ERC20 token to a recipient
///
/// If ERC20 balance is insufficient then the call frame reverts.
/// Since the indexer filters for successful call frames, the requested amount
///
pub fn ix_1_credit_erc20(payload: &[u8]) -> Result<usize, GoblinError> {
    require!(
        payload.len() >= HANDLE_1_PAYLOAD_LEN,
        GoblinError::InvalidPayload
    );

    let params = unsafe { &*(payload.as_ptr() as *const CreditERC20Params) };
    let atoms = params.atoms;

    let mut sender_maybe = MaybeUninit::<Address>::uninit();
    let sender = unsafe {
        msg_sender(sender_maybe.as_mut_ptr() as *mut u8);
        sender_maybe.assume_init_ref()
    };

    // Read trader token state, see if decimals are stored
    // If not stored then read from token contract

    let trader_token_key = &TraderTokenKey {
        trader: params.recipient,
        token: params.token,
    };
    let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
    let trader_token_state =
        unsafe { TraderTokenState::load(trader_token_key, &mut trader_token_state_maybe) };

    // This is not a simple success / failure transaction. We need to know the decimal value returned
    // - The slot write is determined using inputs. We can't use it.
    // - The value requested in erc20::transfer_from() is also determinted using inputs.
    // Therefore decimal places have to be found.
    if trader_token_state.is_empty() {
        trader_token_state.decimals = erc20::decimals(&params.token)?;
    }

    trader_token_state.atoms_free += atoms;

    unsafe {
        trader_token_state.store(trader_token_key);

        #[cfg(all(not(test), not(target_arch = "wasm32")))]
        indexer_hostio::index_deposit(
            trader_token_key.trader.as_ptr(),
            trader_token_key.token.as_ptr(),
            atoms.0,
        );
    }

    // Cross contract call should be performed last to remove the need to flush cache twice
    // Transfer tokens to smart contract ADDRESS, not params.recipient
    let raw_atoms = atoms.to_raw_atoms(trader_token_state.decimals)?;
    erc20::transfer_from(&params.token, sender, &ADDRESS, &raw_atoms)?;

    Ok(HANDLE_1_PAYLOAD_LEN)
}

#[cfg(test)]
mod test {
    use super::*;

    use hex_literal::hex;

    use crate::{
        getter::read_trader_token_state,
        hostio::*,
        state::{SlotState, TraderTokenKey, TraderTokenState},
        user_entrypoint,
    };

    use super::{CreditERC20Params, HANDLE_1_CREDIT_ERC20};

    #[test]
    pub fn test_deposit_erc20() {
        // Set hostios
        let msg_sender = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
        set_msg_sender(msg_sender);

        let mut return_data = vec![0u8; 32];
        return_data[31] = 1;
        set_return_data(vec![return_data]);

        // Set args
        let mut test_args: Vec<u8> = vec![];
        let num_calls: u8 = 1;
        test_args.push(num_calls);
        test_args.push(HANDLE_1_CREDIT_ERC20);

        let payload = CreditERC20Params {
            token: hex!("7E32b54800705876d3b5cFbc7d9c226a211F7C1a"),
            recipient: hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E"),
            atoms: Atoms(1),
        };

        // Serialize into bytes array
        let payload_bytes: &[u8] = unsafe {
            core::slice::from_raw_parts(
                &payload as *const CreditERC20Params as *const u8,
                core::mem::size_of::<CreditERC20Params>(),
            )
        };
        test_args.extend_from_slice(payload_bytes);
        set_test_args(test_args.clone());

        let decimals: u8 = 6;
        let mut return_data = [0u8; 32];
        return_data[31] = decimals;
        set_return_data(vec![return_data.to_vec()]);

        let result = user_entrypoint(test_args.len());
        assert_eq!(result, 0);

        let key = &TraderTokenKey {
            trader: payload.recipient,
            token: payload.token,
        };

        let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
        let trader_token_state =
            unsafe { TraderTokenState::load(key, &mut trader_token_state_maybe) };

        assert_eq!(trader_token_state.atoms_free.0, 1);
        assert_eq!(trader_token_state.atoms_locked.0, 0);

        // Validate result from getter
        let trader_token_state_bytes = read_trader_token_state(key);
        let trader_token_state: &TraderTokenState =
            unsafe { &*(trader_token_state_bytes.as_ptr() as *const TraderTokenState) };

        assert_eq!(trader_token_state.atoms_free.0, 1);
        assert_eq!(trader_token_state.atoms_locked.0, 0);
    }
}
