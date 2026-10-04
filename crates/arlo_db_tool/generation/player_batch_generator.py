import random
from typing import Dict, List, Optional, Sequence, Tuple
import uuid
from arlo_db_tool.generation.attribute_draft_builder import build_player_attributes
from arlo_db_tool.generation.birthdate_sampler import sample_birthdate
from arlo_db_tool.generation.height_sampler import sample_height
from arlo_db_tool.generation.name_generator import generate_name
from arlo_db_tool.generation.nationality_sampler import sample_nationality
from arlo_db_tool.generation.player_draft import PlayerDraft, PlayerPositionDraft
from arlo_db_tool.generation.position_line_groups import sample_secondary_positions
from arlo_db_tool.schema.entities.position_codes import SHORT_POSITION_CODES


def generate_player_batch(
    count: int,
    position_quotas: Optional[Dict[str, int]] = None,
    team_id: Optional[str] = None,
    ca_range: Tuple[int, int] = (80, 130),
    age_range: Tuple[int, int] = (18, 35),
    reference_year: int = 3627,
    available_country_ids: Sequence[str] = (),
    preferred_country_id: Optional[str] = None,
    preferred_country_bias: float = 0.70,
    secondary_positions_chance: float = 0.40,
    max_secondary_positions: int = 2,
    used_squad_numbers: Optional[Sequence[int]] = None,
    db_path: Optional[str] = None,
    rng: Optional[random.Random] = None,
) -> List[PlayerDraft]:
    r = rng if rng is not None else random.Random()

    positions_to_generate: List[str] = []
    if position_quotas:
        for pos_code, q_count in position_quotas.items():
            for _ in range(max(0, q_count)):
                positions_to_generate.append(pos_code)

    while len(positions_to_generate) < count:
        positions_to_generate.append(r.choice(SHORT_POSITION_CODES))

    if len(positions_to_generate) > count:
        positions_to_generate = positions_to_generate[:count]

    r.shuffle(positions_to_generate)

    existing_numbers = set(used_squad_numbers) if used_squad_numbers else set()
    available_squad_numbers = [
        num for num in range(1, 100) if num not in existing_numbers
    ]
    r.shuffle(available_squad_numbers)

    players: List[PlayerDraft] = []
    min_age, max_age = min(age_range), max(age_range)
    min_ca, max_ca = min(ca_range), max(ca_range)

    for i, primary_pos in enumerate(positions_to_generate):
        player_id = str(uuid.uuid4())
        name = generate_name(rng=r)
        nat_id = sample_nationality(
            available_country_ids=available_country_ids,
            preferred_country_id=preferred_country_id,
            preferred_bias=preferred_country_bias,
            rng=r,
        )
        height = sample_height(position_code=primary_pos, rng=r)
        birthdate = sample_birthdate(
            reference_year=reference_year,
            min_age=min_age,
            max_age=max_age,
            rng=r,
        )

        squad_num = (
            available_squad_numbers[i]
            if i < len(available_squad_numbers)
            else None
        )

        positions = [PlayerPositionDraft(position=primary_pos, proficiency=10)]
        secondary_pos_list = sample_secondary_positions(
            primary_position=primary_pos,
            max_count=max_secondary_positions,
            secondary_chance=secondary_positions_chance,
            rng=r,
        )
        for sec_pos, sec_prof in secondary_pos_list:
            positions.append(
                PlayerPositionDraft(position=sec_pos, proficiency=sec_prof)
            )

        attrs = build_player_attributes(
            position_code=primary_pos,
            target_ca_min=min_ca,
            target_ca_max=max_ca,
            db_path=db_path,
            rng=r,
        )

        draft = PlayerDraft(
            id=player_id,
            name=name,
            height_m=height,
            birthdate_unix_seconds=birthdate,
            nationality_id=nat_id,
            team_id=team_id,
            squad_number=squad_num,
            captaincy_role=None,
            positions=positions,
            attributes=attrs,
        )
        players.append(draft)

    return players
