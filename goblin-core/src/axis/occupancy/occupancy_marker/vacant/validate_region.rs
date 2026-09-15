use crate::{
    axis::leg::LegMatcher,
    goblin_error::GoblinError,
    matching::MakeRegion,
    quantities::{FullPosition, PositionV2},
    require,
    state::InnerBitmap,
};

pub fn validate_region<In>(
    region: MakeRegion,
    position: PositionV2,
    inner_bitmap_state: &InnerBitmap,
) -> Result<(), GoblinError>
where
    In: LegMatcher,
{
    match region {
        MakeRegion::In(leg_enum) => {
            require!(In::VARIANT == leg_enum, GoblinError::InvalidOpenPrice);

            let inner_pos = position.extract_and_convert();
            require!(
                !inner_bitmap_state.index_active(inner_pos),
                GoblinError::PositionOccupied
            );
        }
        MakeRegion::OnLastPrice(leg_enum) => {
            require!(In::VARIANT == leg_enum, GoblinError::InvalidOpenPrice);
        }
        MakeRegion::Spread => {}
    }

    Ok(())
}
