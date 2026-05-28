pub struct MakeDelta<O>
where
    O: Clone + Copy,
{
    increase: O,
    reduce: O,
}
