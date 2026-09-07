use crate::{
    axis::{Decrease, UpdateAddress},
    input_processor::CallerAddresses,
    types::Address,
};

impl UpdateAddress for Decrease {
    fn update_address(call_addresses: CallerAddresses) -> &Address {
        // Credit to custom recipient if present
        call_addresses
            .custom_recipient
            .unwrap_or(call_addresses.caller)
    }
}
