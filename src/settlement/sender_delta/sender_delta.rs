use crate::settlement::sender_delta::{MakeDelta, TakeDelta};

pub struct SenderDelta<I, O>
where
    I: Clone + Copy,
    O: Clone + Copy,
{
    pub take: TakeDelta<I, O>,
    pub make: MakeDelta<O>,
}
