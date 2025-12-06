use crate::{
    markets::MarketVariant,
    tokens::{DynamicIndex, HardcodedIndex},
};

/// Stores one value of `T` for each supported market variant
pub struct MarketVariantMap<T> {
    pub hardcoded: T,
    pub dynamic: T,
}

pub trait MarketVariantGetter<T, M: MarketVariant> {
    fn get_ref(&self) -> &T;
    fn get_mut(&mut self) -> &mut T;
}

impl<T> MarketVariantGetter<T, HardcodedIndex> for MarketVariantMap<T> {
    fn get_ref(&self) -> &T {
        &self.hardcoded
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.hardcoded
    }
}

impl<T> MarketVariantGetter<T, DynamicIndex> for MarketVariantMap<T> {
    fn get_ref(&self) -> &T {
        &self.dynamic
    }

    fn get_mut(&mut self) -> &mut T {
        &mut self.dynamic
    }
}
