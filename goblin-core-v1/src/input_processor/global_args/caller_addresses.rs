use crate::types::Address;

#[derive(Clone, Copy)]
pub struct CallerAddresses<'a> {
    pub caller: &'a Address,
    pub custom_recipient: Option<&'a Address>,
}
