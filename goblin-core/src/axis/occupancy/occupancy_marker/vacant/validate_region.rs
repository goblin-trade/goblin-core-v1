use crate::{
    axis::leg::LegMatcher,
    goblin_error::GoblinError,
    matching::MakeRegion,
    quantities::{InnerPos, Position},
    require,
    state::InnerBitmap,
};

pub fn validate_region<In>(
    region: MakeRegion,
    position: Position,
    inner_bitmap_state: &InnerBitmap,
) -> Result<(), GoblinError>
where
    In: LegMatcher,
{
    match region {
        MakeRegion::In(leg_enum) => {
            require!(In::VARIANT == leg_enum, GoblinError::InvalidOpenPrice);

            let inner_pos = InnerPos::from(position);
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
