use crate::define_axis;

define_axis! {
    pub struct Token;
    enum TokenEnum {
        ETH = 0,
        HardcodedERC20 = 1,
        CustomERC20 = 2,
    }
}
