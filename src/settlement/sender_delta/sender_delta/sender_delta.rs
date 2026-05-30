use crate::settlement::{
    sender_delta::{MakeDelta, TakeDelta},
    CheckedAdd, ConstZero,
};

#[derive(Default, Clone, Copy)]
pub struct SenderDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedAdd,
    O: Clone + Copy + ConstZero + CheckedAdd,
{
    pub take: TakeDelta<I, O>,

    // TODO fix, LocalSenderDelta uses Lots instead of MatchLots
    //
    // Options
    // - Try to simplify SenderDelta trait. But we want to avoid more generic fields
    // - MakerDelta to store MatchLots. Convert it when moving to global delta.
    // Use decode_matching_lots(matching_lots, base_lot_size)
    pub make: MakeDelta<O>,
}

impl<I, O> ConstZero for SenderDelta<I, O>
where
    I: Clone + Copy + ConstZero + CheckedAdd,
    O: Clone + Copy + ConstZero + CheckedAdd,
{
    const ZEROED: Self = Self {
        take: TakeDelta::ZEROED,
        make: MakeDelta::ZEROED,
    };
}
