#[derive(Clone, Copy)]
pub struct CustomRange<C>
where
    C: Clone + Copy + PartialEq,
{
    pub start: C,
    pub end: C,
}
