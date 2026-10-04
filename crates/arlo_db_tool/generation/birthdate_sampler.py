import random
from typing import Optional
from arlo_db_tool.utils.date_helpers import date_to_unix_seconds


def sample_birthdate(
    reference_year: int = 3627,
    min_age: int = 18,
    max_age: int = 35,
    rng: Optional[random.Random] = None,
) -> int:
    r = rng if rng is not None else random.Random()

    lower_age = min(min_age, max_age)
    upper_age = max(min_age, max_age)

    age = r.randint(lower_age, upper_age)
    birth_year = reference_year - age
    month = r.randint(1, 13)
    day = r.randint(1, 28)

    return date_to_unix_seconds(birth_year, month, day)
