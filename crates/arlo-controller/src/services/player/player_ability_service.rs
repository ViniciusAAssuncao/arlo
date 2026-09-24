use arlo_domain::Player;
use crate::repositories::attribute::attribute_definition_cache::AttributeKeyIndex;

pub fn calculate_player_ability(player: &Player, key_index: &AttributeKeyIndex) -> Option<i32> {
    let (sum, count) = player
        .attributes()
        .iter()
        .filter(|attribute| key_index.contains_key(&attribute.attribute_definition_id()))
        .fold((0_i32, 0_i32), |(sum, count), attribute| {
            (sum + attribute.value(), count + 1)
        });
    (count > 0).then(|| (sum * 5 + count / 2) / count)
}

pub fn calculate_player_ca_with_index(
    player: &Player,
    key_index: &AttributeKeyIndex,
) -> Option<i32> {
    calculate_player_ability(player, key_index)
}
