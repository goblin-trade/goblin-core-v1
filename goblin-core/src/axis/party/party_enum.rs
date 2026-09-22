use goblin_macros::define_axis;

#[define_axis]
pub enum PartyEnum {
    Sender = 0,
    Counterparties = 1,
}
