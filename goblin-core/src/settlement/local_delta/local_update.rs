use crate::{
    axis::party::{Counterparties, Party, Sender},
    axis_helpers::TokenPair,
    settlement::local_delta::{LocalCounterparties, LocalDelta, LocalDeposits, LocalSenderUpdate},
    types::{StoreReader, Tuple},
};

/// Intermediate type between local delta and global delta
///
/// The sender side additionally includes `LocalDeposits` and the counterparties
/// limb access is reduced to read only.
pub type LocalUpdate<'a, TP> = Tuple<LocalSenderUpdate<TP>, &'a LocalCounterparties, Party>;

impl<'a, TP: TokenPair> From<(&'a LocalDelta<'a>, LocalDeposits<TP>)> for LocalUpdate<'a, TP> {
    fn from(value: (&'a LocalDelta<'a>, LocalDeposits<TP>)) -> Self {
        let sender_update = LocalSenderUpdate {
            sender: Sender::get(value.0),
            deposits: value.1,
        };

        let counterparty_update = &**Counterparties::get_leg(value.0);

        Self::new(sender_update, counterparty_update)
    }
}
