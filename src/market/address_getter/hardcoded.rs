use crate::{
    market::{AddressGetter, Hardcoded, MarketVariant},
    token::TokenIndex,
};

impl AddressGetter for <Hardcoded as MarketVariant>::TokenIndex {}
