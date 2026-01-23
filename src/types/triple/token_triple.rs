use crate::{
    impl_triple_reader,
    token::{CustomERC20, HardcodedERC20, ETH},
    types::Triple,
};

impl_triple_reader!(ETH, HardcodedERC20, CustomERC20);

pub type TokenTriple<T0, T1, T2> = Triple<T0, T1, T2, (ETH, HardcodedERC20, CustomERC20)>;
