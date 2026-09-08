use core::marker::PhantomData;

use crate::{
    axis::{token::token_marker::TokenMarker, update::UpdateMarker},
    input_processor::CallerAddresses,
    quantities::UnsidedAtoms,
};

pub struct UpdateParams<'a, TM: TokenMarker, UM: UpdateMarker> {
    pub token_address: &'a TM::TokenAddress,
    pub caller_addresses: CallerAddresses<'a>,
    pub deposit: UnsidedAtoms,
    pub decimals: TM::StoredDecimals,
    pub _marker: PhantomData<UM>,
}
