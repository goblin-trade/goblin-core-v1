use crate::{
    axis::party::Party,
    settlement::{
        local_delta::{LocalCounterparties, LocalSender},
        ConstDefault,
    },
    types::Tuple,
};

pub type LocalDelta<'a> = Tuple<LocalSender, &'a mut LocalCounterparties, Party>;

impl<'a> LocalDelta<'a> {
    pub const fn create_new(counterparties: &'a mut LocalCounterparties) -> Self {
        Self::new(LocalSender::DEFAULT, counterparties)
    }
}
