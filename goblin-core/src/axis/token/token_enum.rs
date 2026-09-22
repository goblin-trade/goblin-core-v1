use goblin_macros::define_axis;

#[define_axis]
pub enum TokenEnum {
    ETH = 0,
    HardcodedERC20 = 1,
    CustomERC20 = 2,
}
