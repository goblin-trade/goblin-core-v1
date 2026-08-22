use crate::define_axis;

define_axis! {
    pub struct Party;
    enum PartyEnum {
        Sender = 0,
        Counterparties = 1,
    }
}
