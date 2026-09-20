pub mod back_line;
pub mod defense_line;
pub mod goalguard;
pub mod offensive_line;

pub use back_line::*;
pub use defense_line::*;
pub use goalguard::*;
pub use offensive_line::*;

use crate::attributes::profiles::get_position_attribute_profile;
use crate::current_ability::weights::PositionWeightProfile;
use arlo_domain::Position;

pub fn get_profile_for_position(position: Position) -> PositionWeightProfile {
    let profile = get_position_attribute_profile(position);
    PositionWeightProfile::from_profile(position, &profile)
}
