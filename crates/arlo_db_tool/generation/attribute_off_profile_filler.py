import random
from typing import Dict, Iterable, Optional, Set
from arlo_db_tool.ca.constants import ATTRIBUTE_MAX, ATTRIBUTE_MIN


def fill_off_profile_attributes(
    all_keys: Iterable[str],
    profile_keys: Set[str],
    rng: Optional[random.Random] = None,
    mean: float = 10.0,
    std_dev: float = 2.0,
) -> Dict[str, int]:
    r = rng if rng is not None else random.Random()
    result: Dict[str, int] = {}
    for key in all_keys:
        if key not in profile_keys:
            raw = r.gauss(mean, std_dev)
            clamped = int(round(max(ATTRIBUTE_MIN, min(ATTRIBUTE_MAX, raw))))
            result[key] = clamped
    return result
