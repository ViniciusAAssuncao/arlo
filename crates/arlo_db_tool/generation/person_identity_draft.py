from dataclasses import dataclass, field
import random
from typing import List, Optional, Sequence
import uuid
from arlo_db_tool.generation.birthdate_sampler import sample_birthdate
from arlo_db_tool.generation.height_sampler import sample_height
from arlo_db_tool.generation.name_generator import generate_name
from arlo_db_tool.generation.nationality_sampler import sample_nationality


@dataclass
class PersonIdentityDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    name: str = ""
    height_m: float = 1.85
    birthdate_unix_seconds: int = 0
    nationality_id: str = ""


def create_person_identity_draft(
    available_country_ids: Sequence[str],
    preferred_country_id: Optional[str] = None,
    preferred_bias: float = 0.70,
    reference_year: int = 2025,
    min_age: int = 18,
    max_age: int = 35,
    position_line: Optional[str] = None,
    position_code: Optional[str] = None,
    custom_id: Optional[str] = None,
    rng: Optional[random.Random] = None,
) -> PersonIdentityDraft:
    r = rng if rng is not None else random.Random()

    person_id = custom_id if custom_id is not None else str(uuid.uuid4())
    name = generate_name(rng=r)
    nationality_id = sample_nationality(
        available_country_ids=available_country_ids,
        preferred_country_id=preferred_country_id,
        preferred_bias=preferred_bias,
        rng=r,
    )
    height_m = sample_height(
        position_line=position_line,
        position_code=position_code,
        rng=r,
    )
    birthdate_unix_seconds = sample_birthdate(
        reference_year=reference_year,
        min_age=min_age,
        max_age=max_age,
        rng=r,
    )

    return PersonIdentityDraft(
        id=person_id,
        name=name,
        height_m=height_m,
        birthdate_unix_seconds=birthdate_unix_seconds,
        nationality_id=nationality_id,
    )


def generate_person_identities(
    count: int,
    available_country_ids: Sequence[str],
    preferred_country_id: Optional[str] = None,
    preferred_bias: float = 0.70,
    reference_year: int = 2025,
    min_age: int = 18,
    max_age: int = 35,
    position_line: Optional[str] = None,
    position_code: Optional[str] = None,
    rng: Optional[random.Random] = None,
) -> List[PersonIdentityDraft]:
    r = rng if rng is not None else random.Random()
    identities: List[PersonIdentityDraft] = []

    for _ in range(max(0, count)):
        identities.append(
            create_person_identity_draft(
                available_country_ids=available_country_ids,
                preferred_country_id=preferred_country_id,
                preferred_bias=preferred_bias,
                reference_year=reference_year,
                min_age=min_age,
                max_age=max_age,
                position_line=position_line,
                position_code=position_code,
                rng=r,
            )
        )

    return identities
