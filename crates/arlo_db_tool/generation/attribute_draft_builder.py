import random
from typing import Any, Dict, List, Optional
from arlo_db_tool.ca.profiles import get_manager_profile, get_position_profile
from arlo_db_tool.db.reference_lookup import get_attribute_definitions
from arlo_db_tool.generation.attribute_off_profile_filler import fill_off_profile_attributes
from arlo_db_tool.generation.attribute_target_solver import solve_profile_attributes

FALLBACK_PLAYER_ATTRIBUTE_KEYS = [
    "arlo_control",
    "passing",
    "dribbling",
    "finishing",
    "crossing",
    "offensive_blocking",
    "defensive_containment",
    "passer_pressure",
    "hands_reception",
    "drive_technique",
    "false_artrine_bluff",
    "technique",
    "anticipation",
    "decisions",
    "composure",
    "concentration",
    "vision",
    "positioning",
    "teamwork",
    "determination",
    "leadership",
    "controlled_aggression",
    "bravery",
    "flair",
    "work_rate",
    "consistency",
    "acceleration",
    "pace",
    "agility",
    "balance",
    "strength",
    "stamina",
    "jumping_reach",
    "natural_fitness",
    "reflexes",
    "handling",
    "area_command",
    "communication",
    "one_on_one",
    "distribution",
    "goal_kicking",
    "rushing_out",
]

FALLBACK_MANAGER_ATTRIBUTE_KEYS = [
    "tactical_knowledge",
    "offense_planning",
    "defense_organization",
    "artro_strategy",
    "adaptability",
    "artrine_communication",
    "time_call_management",
    "challenge_judgment",
    "in_game_adjustments",
    "man_management",
    "load_management",
    "player_development",
    "judging_ability",
    "judging_potential",
    "discipline",
]

FALLBACK_REFEREE_ATTRIBUTE_KEYS = [
    "rigor",
    "authority",
    "discipline",
    "anticipation",
    "decisions",
    "composure",
    "concentration",
    "positioning",
    "communication",
    "stamina",
    "natural_fitness",
    "consistency",
]


def get_definitions_for_target(
    applies_to: str,
    db_path: Optional[str] = None,
) -> List[Dict[str, Any]]:
    defs = get_attribute_definitions(db_path, applies_to) if db_path else []
    if defs:
        return defs
    if applies_to == "Player":
        fallback_keys = FALLBACK_PLAYER_ATTRIBUTE_KEYS
    elif applies_to == "Manager":
        fallback_keys = FALLBACK_MANAGER_ATTRIBUTE_KEYS
    else:
        fallback_keys = FALLBACK_REFEREE_ATTRIBUTE_KEYS
    return [
        {"id": k, "key": k, "display_name": k, "applies_to": applies_to}
        for k in fallback_keys
    ]


def build_player_attributes(
    position_code: str,
    target_ca_min: int = 80,
    target_ca_max: int = 120,
    db_path: Optional[str] = None,
    rng: Optional[random.Random] = None,
    noise_std: float = 1.2,
) -> Dict[str, int]:
    r = rng if rng is not None else random.Random()
    profile = get_position_profile(position_code)
    profile_attrs = solve_profile_attributes(
        profile=profile,
        target_ca_min=target_ca_min,
        target_ca_max=target_ca_max,
        rng=r,
        noise_std=noise_std,
    )
    profile_keys = set(profile_attrs.keys())
    defs = get_definitions_for_target("Player", db_path)
    all_keys = [d["key"] for d in defs if "key" in d]
    off_profile_attrs = fill_off_profile_attributes(
        all_keys=all_keys,
        profile_keys=profile_keys,
        rng=r,
    )
    combined_by_key = {**off_profile_attrs, **profile_attrs}
    result: Dict[str, int] = {}
    for d in defs:
        key = d.get("key")
        def_id = str(d.get("id", key))
        if key in combined_by_key:
            result[def_id] = combined_by_key[key]
    return result


def build_manager_attributes(
    target_ca_min: int = 80,
    target_ca_max: int = 120,
    db_path: Optional[str] = None,
    rng: Optional[random.Random] = None,
    noise_std: float = 1.2,
) -> Dict[str, int]:
    r = rng if rng is not None else random.Random()
    profile = get_manager_profile()
    profile_attrs = solve_profile_attributes(
        profile=profile,
        target_ca_min=target_ca_min,
        target_ca_max=target_ca_max,
        rng=r,
        noise_std=noise_std,
    )
    profile_keys = set(profile_attrs.keys())
    defs = get_definitions_for_target("Manager", db_path)
    all_keys = [d["key"] for d in defs if "key" in d]
    off_profile_attrs = fill_off_profile_attributes(
        all_keys=all_keys,
        profile_keys=profile_keys,
        rng=r,
    )
    combined_by_key = {**off_profile_attrs, **profile_attrs}
    result: Dict[str, int] = {}
    for d in defs:
        key = d.get("key")
        def_id = str(d.get("id", key))
        if key in combined_by_key:
            result[def_id] = combined_by_key[key]
    return result


def build_referee_attributes(
    db_path: Optional[str] = None,
    rng: Optional[random.Random] = None,
    mean: float = 12.0,
    std_dev: float = 3.0,
) -> Dict[str, int]:
    r = rng if rng is not None else random.Random()
    defs = get_definitions_for_target("Referee", db_path)
    all_keys = [d["key"] for d in defs if "key" in d]
    attrs_by_key = fill_off_profile_attributes(
        all_keys=all_keys,
        profile_keys=set(),
        rng=r,
        mean=mean,
        std_dev=std_dev,
    )
    result: Dict[str, int] = {}
    for d in defs:
        key = d.get("key")
        def_id = str(d.get("id", key))
        if key in attrs_by_key:
            result[def_id] = attrs_by_key[key]
    return result
