#[derive(Clone, Copy)]
pub struct Base;

#[derive(Clone, Copy)]
pub struct Quote;

pub trait LegMarker {
    type Opposite: LegMarker;
}

impl LegMarker for Base {
    type Opposite = Quote;
}

impl LegMarker for Quote {
    type Opposite = Base;
}
