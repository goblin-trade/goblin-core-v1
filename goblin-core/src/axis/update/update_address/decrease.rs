use crate::{
    axis::update::{Decrease, UpdateAddress},
    input_processor::CallerAddresses,
    types::Address,
};

impl UpdateAddress for Decrease {
    fn get_update_address<'a>(call_addresses: CallerAddresses<'a>) -> &'a Address {
        // Credit to custom recipient if present
        call_addresses
            .custom_recipient
            .unwrap_or(call_addresses.caller)
    }
}
