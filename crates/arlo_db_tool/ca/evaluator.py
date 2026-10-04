from typing import Dict, List, Tuple
from arlo_db_tool.ca.calculator import calculate_current_ability
from arlo_db_tool.ca.profiles import (
    POSITION_CODES,
    get_manager_profile,
    get_position_profile,
)

def calculate_player_ca(
    attributes: Dict[str, float],
    position_code: str = "P",
) -> int:
    profile = get_position_profile(position_code)
    attributes_and_weights: List[Tuple[float, float]] = []

    for attr_key, weight in profile:
        if weight > 0.0:
            val = float(attributes.get(attr_key, 10.0))
            attributes_and_weights.append((val, weight))

    return calculate_current_ability(attributes_and_weights)

def calculate_player_all_positions_ca(
    attributes: Dict[str, float],
) -> Dict[str, int]:
    result: Dict[str, int] = {}
    for code in POSITION_CODES:
        result[code] = calculate_player_ca(attributes, code)
    return result

def calculate_manager_ca(
    attributes: Dict[str, float],
) -> int:
    profile = get_manager_profile()
    attributes_and_weights: List[Tuple[float, float]] = []

    for attr_key, weight in profile:
        if weight > 0.0:
            val = float(attributes.get(attr_key, 10.0))
            attributes_and_weights.append((val, weight))

    return calculate_current_ability(attributes_and_weights)
