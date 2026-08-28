use crate::{
    axis::party::{Counterparties, Party, Sender},
    axis_helpers::TokenPair,
    settlement::local_delta::{
        LocalCounterparties, LocalDelta, LocalDeposits, LocalSenderUpdateV2,
    },
    types::{StoreReader, Tuple},
};

pub type LocalUpdateV2<'a, TP> = Tuple<LocalSenderUpdateV2<TP>, &'a LocalCounterparties, Party>;

impl<'a, TP: TokenPair> From<(&'a LocalDelta<'a>, LocalDeposits<TP>)> for LocalUpdateV2<'a, TP> {
    fn from(value: (&'a LocalDelta<'a>, LocalDeposits<TP>)) -> Self {
        let sender_update = LocalSenderUpdateV2 {
            sender: Sender::get(value.0),
            deposits: value.1,
        };

        let counterparty_update = &**Counterparties::get_leg(value.0);

        Self::new(sender_update, counterparty_update)
    }
}
