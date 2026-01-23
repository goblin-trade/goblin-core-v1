use crate::{
    impl_triple_reader,
    token::{CustomERC20, HardcodedERC20, ETH},
};

impl_triple_reader!(ETH, HardcodedERC20, CustomERC20);
