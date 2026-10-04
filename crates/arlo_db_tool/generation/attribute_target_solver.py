import random
from typing import Dict, List, Optional, Tuple
from arlo_db_tool.ca.calculator import calculate_current_ability
from arlo_db_tool.ca.constants import (
    ATTRIBUTE_MAX,
    ATTRIBUTE_MIN,
    MAX_CURRENT_ABILITY,
    MIN_CURRENT_ABILITY,
)


def find_base_level(profile: List[Tuple[str, float]], target_ca: int) -> float:
    clamped_target = max(MIN_CURRENT_ABILITY, min(MAX_CURRENT_ABILITY, target_ca))
    active_profile = [(k, w) for k, w in profile if w > 0.0]
    if not active_profile:
        return 10.0

    low = ATTRIBUTE_MIN
    high = ATTRIBUTE_MAX
    for _ in range(30):
        mid = (low + high) / 2.0
        pairs = [(mid, weight) for _, weight in active_profile]
        ca = calculate_current_ability(pairs)
        if ca < clamped_target:
            low = mid
        else:
            high = mid
    return (low + high) / 2.0


def solve_profile_attributes(
    profile: List[Tuple[str, float]],
    target_ca_min: int = 80,
    target_ca_max: int = 120,
    rng: Optional[random.Random] = None,
    noise_std: float = 1.2,
    tolerance: int = 5,
    max_attempts: int = 15,
) -> Dict[str, int]:
    r = rng if rng is not None else random.Random()
    min_ca = max(MIN_CURRENT_ABILITY, min(target_ca_min, target_ca_max))
    max_ca = min(MAX_CURRENT_ABILITY, max(target_ca_min, target_ca_max))
    target_ca = r.randint(min_ca, max_ca)

    active_profile = [(key, weight) for key, weight in profile if weight > 0.0]
    if not active_profile:
        return {}

    base_level = find_base_level(active_profile, target_ca)

    best_attrs: Dict[str, int] = {}
    best_diff = float("inf")

    for _ in range(max_attempts):
        candidate: Dict[str, int] = {}
        for key, _ in active_profile:
            val = r.gauss(base_level, noise_std)
            clamped = int(round(max(ATTRIBUTE_MIN, min(ATTRIBUTE_MAX, val))))
            candidate[key] = clamped

        pairs = [(float(candidate[key]), weight) for key, weight in active_profile]
        actual_ca = calculate_current_ability(pairs)

        diff = abs(actual_ca - target_ca)
        if diff < best_diff:
            best_diff = diff
            best_attrs = candidate

        if min_ca <= actual_ca <= max_ca or diff <= tolerance:
            return candidate

    return best_attrs
