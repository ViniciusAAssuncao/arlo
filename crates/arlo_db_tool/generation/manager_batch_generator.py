import random
from typing import List, Optional, Sequence, Tuple
import uuid
from arlo_db_tool.generation.attribute_draft_builder import build_manager_attributes
from arlo_db_tool.generation.birthdate_sampler import sample_birthdate
from arlo_db_tool.generation.height_sampler import sample_height
from arlo_db_tool.generation.manager_draft import ManagerDraft
from arlo_db_tool.generation.name_generator import generate_name
from arlo_db_tool.generation.nationality_sampler import sample_nationality


def generate_manager_batch(
    count: int,
    team_ids: Optional[Sequence[Optional[str]]] = None,
    team_id: Optional[str] = None,
    ca_range: Tuple[int, int] = (80, 140),
    age_range: Tuple[int, int] = (35, 65),
    reference_year: int = 3627,
    available_country_ids: Sequence[str] = (),
    preferred_country_id: Optional[str] = None,
    preferred_country_bias: float = 0.70,
    control_mode: str = "Ai",
    db_path: Optional[str] = None,
    rng: Optional[random.Random] = None,
) -> List[ManagerDraft]:
    r = rng if rng is not None else random.Random()

    assigned_teams: List[Optional[str]] = []
    if team_ids is not None:
        assigned_teams = list(team_ids)
    while len(assigned_teams) < count:
        assigned_teams.append(team_id)
    if len(assigned_teams) > count:
        assigned_teams = assigned_teams[:count]

    managers: List[ManagerDraft] = []
    min_age, max_age = min(age_range), max(age_range)
    min_ca, max_ca = min(ca_range), max(ca_range)

    for i in range(count):
        manager_id = str(uuid.uuid4())
        name = generate_name(rng=r)
        nat_id = sample_nationality(
            available_country_ids=available_country_ids,
            preferred_country_id=preferred_country_id,
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
        attrs = build_manager_attributes(
            target_ca_min=min_ca,
            target_ca_max=max_ca,
            db_path=db_path,
            rng=r,
        )
        current_team_id = assigned_teams[i] if i < len(assigned_teams) else team_id

        draft = ManagerDraft(
            id=manager_id,
            name=name,
            height_m=height,
            birthdate_unix_seconds=birthdate,
            nationality_id=nat_id,
            team_id=current_team_id,
            control_mode=control_mode,
            attributes=attrs,
        )
        managers.append(draft)

    return managers
