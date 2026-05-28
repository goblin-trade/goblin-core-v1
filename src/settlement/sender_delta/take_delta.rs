pub struct TakeDelta<I, O>
where
    I: Clone + Copy,
    O: Clone + Copy,
{
    take_in: I,
    take_out: O,
}
