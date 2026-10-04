import random
from typing import Optional, Sequence
from arlo_db_tool.generation.name_pools import FIRST_NAMES, LAST_NAMES


def generate_name(
    first_names: Optional[Sequence[str]] = None,
    last_names: Optional[Sequence[str]] = None,
    rng: Optional[random.Random] = None,
) -> str:
    r = rng if rng is not None else random.Random()
    f_pool = first_names if first_names is not None else FIRST_NAMES
    l_pool = last_names if last_names is not None else LAST_NAMES

    first = r.choice(f_pool)
    last = r.choice(l_pool)

    return f"{first} {last}"
