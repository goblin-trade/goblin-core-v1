use crate::{
    goblin_error::GoblinError,
    market::{CommonMarket, Dynamic},
    state::{SlotKey, SlotState},
    token::{CustomToken, TokenMarker},
    types::{Base, Quote, TupleReader},
};

/// Trait to generate slot key for dynamic markets
///
/// # Stable rust limitation
///
/// * Generic const expressions are unstable. We cannot set size of a fixed size array
/// equal to a const from a generic. `[u8; B::Size]` is illegal.
///
/// * This trait handles the 3 concrete cases for (ETH, ERC20), (ERC20, ETH) and (ERC20, ERC20)
///
pub trait DynamicMarketHasher<B, Q>
where
    B: TokenMarker,
    Q: TokenMarker,
    Self: SlotState,
{
    /// Size of the hash buffer
    const BUFFER_SIZE: usize =
        1 + 8 * 3 + core::mem::size_of::<B::Address>() + core::mem::size_of::<Q::Address>();

    fn compute_slot_key(
        market: &CommonMarket<Dynamic, B, Q>,
        custom_erc20_list: &[CustomToken],
    ) -> Result<SlotKey<Self>, GoblinError>;

    /// Set the common fields- discriminator, lot sizes and tick size
    fn set_common_fields<const N: usize>(
        bytes: &mut [u8; N],
        market: &CommonMarket<Dynamic, B, Q>,
    ) {
        bytes[0] = Self::SLOT_DISCRIMINATOR;

        bytes[1..9].copy_from_slice(&Base::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[9..17].copy_from_slice(&Quote::get(&market.lot_size_pair).inner.to_le_bytes());
        bytes[17..25].copy_from_slice(&market.tick_size.inner.to_le_bytes());
    }
}
