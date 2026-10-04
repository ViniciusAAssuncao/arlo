from typing import List, Optional, Tuple
from arlo_db_tool.ca.constants import (
    ATTRIBUTE_SATURATION_MULTIPLIER,
    ATTRIBUTE_SATURATION_THRESHOLD,
    CA_FORMULA_MULTIPLIER,
    CA_FORMULA_OFFSET,
    MAX_CURRENT_ABILITY,
    MIN_CURRENT_ABILITY,
)

def calculate_weighted_saturated_average(
    attributes_and_weights: List[Tuple[float, float]],
    threshold: float = ATTRIBUTE_SATURATION_THRESHOLD,
    multiplier: float = ATTRIBUTE_SATURATION_MULTIPLIER,
) -> Optional[float]:
    total_weight = 0.0
    total_score = 0.0

    for val, weight in attributes_and_weights:
        if weight <= 0.0:
            continue
        if val > threshold:
            effective = threshold + (val - threshold) * multiplier
        else:
            effective = val
        total_score += effective * weight
        total_weight += weight

    if total_weight <= 0.0:
        return None

    return total_score / total_weight

def calculate_current_ability(attributes_and_weights: List[Tuple[float, float]]) -> int:
    waa = calculate_weighted_saturated_average(attributes_and_weights)
    if waa is None:
        return MIN_CURRENT_ABILITY

    raw_ca = (waa * CA_FORMULA_MULTIPLIER) - CA_FORMULA_OFFSET
    rounded_ca = int(round(raw_ca))

    if rounded_ca < MIN_CURRENT_ABILITY:
        return MIN_CURRENT_ABILITY
    if rounded_ca > MAX_CURRENT_ABILITY:
        return MAX_CURRENT_ABILITY
    return rounded_ca