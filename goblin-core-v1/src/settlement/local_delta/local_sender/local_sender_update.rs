use crate::{
    axis::party::Sender,
    axis_helpers::TokenPair,
    settlement::local_delta::{LocalDeposits, LocalSender, LocalUpdate},
    types::StoreReader,
};

#[derive(Clone, Copy)]
pub struct LocalSenderUpdate<'a, TP: TokenPair> {
    pub sender: &'a LocalSender,
    pub deposits: LocalDeposits<TP>,
}

impl<'a, TP: TokenPair> From<&LocalUpdate<'a, TP>> for LocalSenderUpdate<'a, TP> {
    fn from(value: &LocalUpdate<'a, TP>) -> Self {
        Self {
            sender: Sender::get_leg(value.delta),
            deposits: value.deposits,
        }
    }
}
