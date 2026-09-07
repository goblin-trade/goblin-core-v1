use crate::{input_processor::CallerAddresses, types::Address};

pub trait UpdateAddress {
    fn get_update_address<'a>(call_addresses: CallerAddresses<'a>) -> &'a Address;
}
