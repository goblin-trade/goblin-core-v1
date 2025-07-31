#[repr(u8)]
#[derive(Clone, Copy)]
pub enum SelfTradeBehavior {
    Abort,
    CancelProvide,
    DecrementTake,
}
