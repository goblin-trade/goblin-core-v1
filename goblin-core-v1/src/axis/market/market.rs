use crate::define_axis;

define_axis! {
    pub struct Market;
    enum MarketEnum {
        Hardcoded = 0,
        Dynamic = 1,
    }
}
