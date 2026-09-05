use crate::define_axis;

define_axis! {
    pub struct Caller;
    enum CallerEnum {
        HardcodedCaller = 0,
        CustomCaller = 1,
    }
}
