use crate::{
    goblin_error::GoblinError,
    markets::MarketVariant,
    quantities::DeltaAtoms,
    token::{CustomToken, DynamicIndex},
    types::Address,
};

#[derive(Clone, Copy, Default)]
pub struct ETH;

#[derive(Clone, Copy, Default)]
pub struct ERC20;

pub trait TokenMarker: Clone + Copy {
    const DISCRIMINATOR: u8;
    const ADDRESS_SIZE: usize = 1;
    // const ADDRESS_SIZE: usize = core::mem::size_of::<Self::Address>();

    type TokenIndex<M: MarketVariant>: Clone + Copy;
    type Address: Clone + Copy + Sized + Default;
    type Deposit: Clone + Copy;

    fn set_token_address<const N: usize>(
        buffer: &mut [u8; N],
        offset: &mut usize,
        index: Self::TokenIndex<DynamicIndex>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<(), GoblinError>;

    fn update_offset(offset: &mut usize) {
        *offset += core::mem::size_of::<Self::Address>();
    }
}

impl TokenMarker for ETH {
    const DISCRIMINATOR: u8 = 0;

    type TokenIndex<M: MarketVariant> = ();
    type Address = ();
    type Deposit = ();

    fn set_token_address<const N: usize>(
        buffer: &mut [u8; N],
        offset: &mut usize,
        index: Self::TokenIndex<DynamicIndex>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<(), GoblinError> {
        Self::update_offset(offset);

        Ok(())
    }
}

impl TokenMarker for ERC20 {
    const DISCRIMINATOR: u8 = 1;

    type TokenIndex<M: MarketVariant> = M;
    type Address = Address;
    type Deposit = DeltaAtoms;

    fn set_token_address<const N: usize>(
        buffer: &mut [u8; N],
        offset: &mut usize,
        index: Self::TokenIndex<DynamicIndex>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<(), GoblinError> {
        let address = index.address(custom_erc20_list)?;
        buffer[*offset..(*offset + 20)].copy_from_slice(&address);

        Self::update_offset(offset);
        Ok(())
    }

    // fn get_token_address(
    //     index: Self::TokenIndex<DynamicIndex>,
    //     custom_erc20_list: &[CustomToken],
    // ) -> Result<Self::Address, GoblinError> {
    //     // problem- this copies the value
    //     index.address(custom_erc20_list)
    // }
}
