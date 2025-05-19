use core::mem::MaybeUninit;

use crate::{
    goblin_error::GoblinError,
    msg_value,
    quantities::{Atoms, RawAtoms},
    require,
    state::{SlotState, TraderTokenKey, TraderTokenState},
    token_delta::TokenDeltaList,
    types::{Address, NATIVE_TOKEN},
};

pub const HANDLE_0_CREDIT_ETH: u8 = 0;
pub const HANDLE_0_PAYLOAD_LEN: usize = core::mem::size_of::<Address>();
pub const NATIVE_TOKEN_DECIMALS: u8 = 18;

#[cfg(all(not(test), not(target_arch = "wasm32")))]
use crate::indexer_hostio;

/// Credit ETH to a recipient
///
/// * Wei is passed using `--value` and read with `msg_value`. It is big endian encoded.
///
/// * The address is encoded in `payload`. The client call encodes the data such that we obtain
/// the big endian result in a slice without need of any processing.
///
/// # Returns
///
/// The number of bytes consumed. Add these bytes to the offset.
///
/// # Example
///
/// ```
/// cast send 0xa6e41ffd769491a42a6e5ce453259b93983a22ef \
///   0x003f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E \
///   --value 1000000wei \
///   --rpc-url http://127.0.0.1:8547 \
///   --private-key 0xb6b15c8cb491557369f3c7d2c287b053eb229daa9c22138887752191c9520
/// ```
///
/// * After removing selector `00` we're left with payload `3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E`
/// * This payload is decoded as [0x3f, 0x1E, ..., 0E]
/// * The address is already in big endian
///
pub fn handle_0_credit_eth(
    payload: &[u8],
    delta_list: &mut TokenDeltaList,
) -> Result<usize, GoblinError> {
    require!(
        payload.len() >= HANDLE_0_PAYLOAD_LEN,
        GoblinError::InvalidPayload
    );

    // Extra bytes in `payload` are ignored. They remain usable outside this function
    let recipient: &Address = unsafe { &*(payload.as_ptr() as *const Address) };

    // Amount of ETH in, in 64-bit chunks, in big endian encoding
    let mut amount_in_maybe = MaybeUninit::<RawAtoms>::uninit();
    let amount_in = unsafe {
        msg_value(amount_in_maybe.as_mut_ptr() as *mut u8);
        amount_in_maybe.assume_init_ref()
    };

    // Convert raw atoms to atoms
    let atoms = Atoms::from_raw_atoms(amount_in, NATIVE_TOKEN_DECIMALS)?;

    let token_delta = delta_list.get(NATIVE_TOKEN)?;

    // ETH transfer is a special case. It gets sent with the call in the beginning
    // itself instead of being settled in the end.
    // Calling this handler twice should be illegal because msg.value will hold
    // total ETH for both cases.
    // When depositing- just subtract from inner_delta. Don't change outer_delta
    // because ETH has already been supplied, there are no pending transfers
    // in the settlement phase.
    // However when we withdraw ETH, the withdrawal does happen during settlement.
    // We should have a separate struct native_delta, independent of TokenDeltaList

    let trader_token_key = &TraderTokenKey {
        trader: *recipient,
        token: NATIVE_TOKEN,
    };
    let mut trader_token_state_maybe = MaybeUninit::<TraderTokenState>::uninit();
    let trader_token_state =
        unsafe { TraderTokenState::load(trader_token_key, &mut trader_token_state_maybe) };

    trader_token_state.decimals = NATIVE_TOKEN_DECIMALS;
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

    Ok(HANDLE_0_PAYLOAD_LEN)
}

#[cfg(test)]
mod tests {
    use super::*;
    use hex_literal::hex;

    use crate::{
        getter::read_trader_token_state,
        set_msg_value, set_test_args,
        state::{SlotState, TraderTokenState},
        user_entrypoint,
    };

    use super::HANDLE_0_CREDIT_ETH;

    #[test]
    pub fn test_deposit() {
        // Set msg.value to 10^12 in big endian

        // TODO obtain 256 bit
        let raw_atoms = 10u128.pow(18 - 6);
        let raw_atoms_u256 = [0u128, raw_atoms.swap_bytes()];
        let msg_value = unsafe { &*(raw_atoms_u256.as_ptr() as *const [u8; 32]) };

        set_msg_value(*msg_value);

        // Set args
        let mut test_args: Vec<u8> = vec![];
        let num_calls: u8 = 1;
        test_args.push(num_calls);
        test_args.push(HANDLE_0_CREDIT_ETH);

        let recipient = hex!("3f1Eae7D46d88F08fc2F8ed27FCb2AB183EB2d0E");
        test_args.extend_from_slice(&recipient);
        set_test_args(test_args.clone());

        let result = user_entrypoint(test_args.len());
        assert_eq!(result, 0);

        // Check lot balance
        let key = &TraderTokenKey {
            trader: recipient,
            token: NATIVE_TOKEN,
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
