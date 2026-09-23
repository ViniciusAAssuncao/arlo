use arlo_domain::Player;
use arlo_engine::attributes::{AttributeKeyIndex, PlayerAttributeTable};
use arlo_engine::current_ability::calculate_player_ca;

pub fn calculate_player_ability(player: &Player, key_index: &AttributeKeyIndex) -> Option<i32> {
    let table = PlayerAttributeTable::from_player_with_index(player, key_index);
    calculate_player_ca(player, &table)
}

pub fn calculate_player_ca_with_index(
    player: &Player,
    key_index: &AttributeKeyIndex,
) -> Option<i32> {
    calculate_player_ability(player, key_index)
}
