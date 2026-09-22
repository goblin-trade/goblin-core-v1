use goblin_macros::define_axis;

#[define_axis]
pub enum MarketEnum {
    Hardcoded = 0,
    Dynamic = 1,
}
