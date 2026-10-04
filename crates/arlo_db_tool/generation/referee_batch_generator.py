import random
from typing import List, Optional, Sequence, Tuple
import uuid
from arlo_db_tool.db.reference_lookup import get_record_by_id
from arlo_db_tool.generation.attribute_draft_builder import build_referee_attributes
from arlo_db_tool.generation.birthdate_sampler import sample_birthdate
from arlo_db_tool.generation.height_sampler import sample_height
from arlo_db_tool.generation.name_generator import generate_name
from arlo_db_tool.generation.nationality_sampler import sample_nationality
from arlo_db_tool.generation.referee_draft import RefereeDraft


def generate_referee_batch(
    count: int,
    primary_league_id: Optional[str] = None,
    tier: Optional[str] = None,
    age_range: Tuple[int, int] = (25, 55),
    reference_year: int = 3627,
    available_country_ids: Sequence[str] = (),
    preferred_country_id: Optional[str] = None,
    preferred_country_bias: float = 0.70,
    mean_attribute: float = 12.0,
    std_dev_attribute: float = 3.0,
    db_path: Optional[str] = None,
    rng: Optional[random.Random] = None,
) -> List[RefereeDraft]:
    r = rng if rng is not None else random.Random()

    resolved_tier = tier
    resolved_preferred_country_id = preferred_country_id

    if (resolved_tier is None or resolved_preferred_country_id is None) and primary_league_id and db_path:
        comp_record = get_record_by_id(db_path, "competitions", "id", primary_league_id)
        if comp_record:
            if resolved_tier is None and "scope" in comp_record and comp_record["scope"]:
                resolved_tier = str(comp_record["scope"])
            if resolved_preferred_country_id is None and "country_id" in comp_record and comp_record["country_id"]:
                resolved_preferred_country_id = str(comp_record["country_id"])

    if resolved_tier is None:
        resolved_tier = "National"

    referees: List[RefereeDraft] = []
    min_age, max_age = min(age_range), max(age_range)

    for _ in range(count):
        referee_id = str(uuid.uuid4())
        name = generate_name(rng=r)
        nat_id = sample_nationality(
            available_country_ids=available_country_ids,
            preferred_country_id=resolved_preferred_country_id,
            preferred_bias=preferred_country_bias,
            rng=r,
        )
        height = sample_height(position_line="General", rng=r)
        birthdate = sample_birthdate(
            reference_year=reference_year,
            min_age=min_age,
            max_age=max_age,
            rng=r,
        )
        attrs = build_referee_attributes(
            db_path=db_path,
            rng=r,
            mean=mean_attribute,
            std_dev=std_dev_attribute,
        )

        draft = RefereeDraft(
            id=referee_id,
            name=name,
            height_m=height,
            birthdate_unix_seconds=birthdate,
            nationality_id=nat_id,
            primary_league_id=primary_league_id,
            tier=resolved_tier,
            attributes=attrs,
        )
        referees.append(draft)

    return referees
