use crate::{
    goblin_error::GoblinError,
    token::{CustomERC20Data, ERC20Data, ERC20Index},
};

/// Marker trait for ERC20 tokens
///
/// It has 2 variants
/// 1. Hardcoded- Token data is hardcoded
/// 2. Custom- Token data is read at runtime
///
pub trait ERC20Marker: Sized {
    type Data: ERC20Data;

    fn get_data(
        erc20_index: ERC20Index<Self>,
        custom_erc20_list: &[CustomERC20Data],
    ) -> Result<&Self::Data, GoblinError>;

    // No need, use markers directly
    // HardcodedERC20::get_leg_mut(token_sender_deltas).get_delta_mut(market_erc20_index)
    // fn sender_delta_mut(
    //     erc20_index: ERC20Index<Self>,
    //     erc20_sender_deltas: &mut ERC20SenderDeltas,
    // ) -> &mut ERC20Delta;

    // fn maker_delta_mut(
    //     erc20_index: ERC20Index<Self>,
    //     maker: Address,
    //     token_maker_deltas: &mut ERC20MakerDeltas,
    // ) -> Option<&mut UnsidedMakerDelta>;
}
