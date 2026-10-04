import random
from typing import List, Optional, Sequence, Set, Tuple
import uuid
from arlo_db_tool.generation.bulk_team_batch_generator import CITY_NAMES
from arlo_db_tool.generation.nationality_sampler import sample_nationality
from arlo_db_tool.generation.venue_draft import VenueDraft
from arlo_db_tool.schema.entities.venue import VENUE_SPEC
from arlo_db_tool.sql.statement_builder import build_insert_statement

VENUE_NAME_SUFFIXES: List[str] = [
    "Arena",
    "Stadium",
    "Coliseum",
    "Park",
    "Dome",
    "Grounds",
    "Field",
    "Center",
    "Complex",
    "Memorial Stadium",
    "Municipal Stadium",
    "Athletic Grounds",
    "Grand Arena",
    "Crown Park",
    "Victory Field",
    "Olympus Coliseum",
]

TRAINING_CENTER_SUFFIXES: List[str] = [
    "Training Ground",
    "Performance Center",
    "Training Complex",
    "Academy Fields",
    "Base Camp",
    "Development Center",
    "Athletic Facility",
]


def generate_venue_name(
    used_names: Set[str],
    team_name: Optional[str] = None,
    kind: str = "MatchStadium",
    rng: Optional[random.Random] = None,
) -> str:
    r = rng if rng is not None else random.Random()
    suffixes = VENUE_NAME_SUFFIXES if kind == "MatchStadium" else TRAINING_CENTER_SUFFIXES

    if team_name and str(team_name).strip():
        clean_team = str(team_name).strip()
        for _ in range(30):
            suffix = r.choice(suffixes)
            candidate = f"{clean_team} {suffix}"
            if candidate not in used_names:
                used_names.add(candidate)
                return candidate

        team_words = clean_team.split()
        if team_words:
            first_word = team_words[0]
            for _ in range(30):
                suffix = r.choice(suffixes)
                candidate = f"{first_word} {suffix}"
                if candidate not in used_names:
                    used_names.add(candidate)
                    return candidate

    for _ in range(100):
        city = r.choice(CITY_NAMES)
        suffix = r.choice(suffixes)
        candidate = f"{city} {suffix}"
        if candidate not in used_names:
            used_names.add(candidate)
            return candidate

    index = 1
    while True:
        city = r.choice(CITY_NAMES)
        suffix = r.choice(suffixes)
        candidate = f"{city} {suffix} {index}"
        if candidate not in used_names:
            used_names.add(candidate)
            return candidate
        index += 1


def generate_venue_batch(
    count: int,
    kind: str = "MatchStadium",
    owner_team_id: Optional[str] = None,
    owner_team_ids: Optional[Sequence[Optional[str]]] = None,
    owner_team_names: Optional[Sequence[Optional[str]]] = None,
    owner_team_countries: Optional[Sequence[Optional[str]]] = None,
    owner_team_prestiges: Optional[Sequence[Optional[int]]] = None,
    country_id: Optional[str] = None,
    available_country_ids: Sequence[str] = (),
    preferred_country_id: Optional[str] = None,
    preferred_country_bias: float = 0.70,
    capacity_range: Tuple[int, int] = (15000, 65000),
    pitch_length_range: Tuple[float, float] = (140.0, 150.0),
    pitch_width_range: Tuple[float, float] = (80.0, 90.0),
    existing_venue_names: Optional[Sequence[str]] = None,
    rng: Optional[random.Random] = None,
) -> List[VenueDraft]:
    r = rng if rng is not None else random.Random()
    used_names: Set[str] = set(existing_venue_names or [])

    cap_min = max(0, min(capacity_range))
    cap_max = max(0, max(capacity_range))

    len_min = max(140.0, min(150.0, min(pitch_length_range)))
    len_max = max(140.0, min(150.0, max(pitch_length_range)))

    wid_min = max(80.0, min(90.0, min(pitch_width_range)))
    wid_max = max(80.0, min(90.0, max(pitch_width_range)))

    assigned_teams: List[Optional[str]] = []
    if owner_team_ids is not None:
        assigned_teams = list(owner_team_ids)
    while len(assigned_teams) < count:
        assigned_teams.append(owner_team_id)
    if len(assigned_teams) > count:
        assigned_teams = assigned_teams[:count]

    team_names: List[Optional[str]] = []
    if owner_team_names is not None:
        team_names = list(owner_team_names)
    while len(team_names) < count:
        team_names.append(None)
    if len(team_names) > count:
        team_names = team_names[:count]

    team_countries: List[Optional[str]] = []
    if owner_team_countries is not None:
        team_countries = list(owner_team_countries)
    while len(team_countries) < count:
        team_countries.append(None)
    if len(team_countries) > count:
        team_countries = team_countries[:count]

    team_prestiges: List[Optional[int]] = []
    if owner_team_prestiges is not None:
        team_prestiges = list(owner_team_prestiges)
    while len(team_prestiges) < count:
        team_prestiges.append(None)
    if len(team_prestiges) > count:
        team_prestiges = team_prestiges[:count]

    venues: List[VenueDraft] = []

    for i in range(count):
        venue_id = str(uuid.uuid4())
        cur_team_id = assigned_teams[i]
        cur_team_name = team_names[i]
        cur_team_country = team_countries[i]
        cur_team_prestige = team_prestiges[i]

        venue_name = generate_venue_name(
            used_names=used_names,
            team_name=cur_team_name,
            kind=kind,
            rng=r,
        )

        if country_id and str(country_id).strip():
            nat_id = str(country_id).strip()
        elif cur_team_country and str(cur_team_country).strip():
            nat_id = str(cur_team_country).strip()
        else:
            nat_id = sample_nationality(
                available_country_ids=available_country_ids,
                preferred_country_id=preferred_country_id,
                preferred_bias=preferred_country_bias,
                rng=r,
            )

        if cur_team_prestige is not None and cur_team_prestige >= 0:
            norm_p = max(0.0, min(1.0, float(cur_team_prestige) / 1000.0))
            scaled_cap = cap_min + norm_p * (cap_max - cap_min)
            jitter = r.randint(-2000, 2000)
            capacity_val = int(max(cap_min, min(cap_max, round(scaled_cap + jitter))))
        else:
            raw_cap = r.randint(cap_min, cap_max)
            capacity_val = int(round(raw_cap / 500.0) * 500)
            capacity_val = max(cap_min, min(cap_max, capacity_val))

        if kind == "MatchStadium":
            raw_len = r.uniform(len_min, len_max)
            pitch_len = round(raw_len * 2.0) / 2.0
            pitch_len = max(140.0, min(150.0, pitch_len))

            raw_wid = r.uniform(wid_min, wid_max)
            pitch_wid = round(raw_wid * 2.0) / 2.0
            pitch_wid = max(80.0, min(90.0, pitch_wid))
        else:
            pitch_len = None
            pitch_wid = None
            capacity_val = None

        draft = VenueDraft(
            id=venue_id,
            name=venue_name,
            kind=kind,
            owner_team_id=cur_team_id,
            country_id=nat_id,
            capacity=capacity_val,
            pitch_length_mirim=pitch_len,
            pitch_width_mirim=pitch_wid,
        )
        venues.append(draft)

    return venues


def build_venue_insert_statements(
    venues: List[VenueDraft],
    update_teams_home_venue: bool = False,
) -> List[str]:
    statements: List[str] = []
    for venue in venues:
        values = {
            "id": venue.id,
            "name": venue.name,
            "kind": venue.kind,
            "owner_team_id": venue.owner_team_id,
            "country_id": venue.country_id,
            "capacity": venue.capacity,
            "pitch_length_mirim": venue.pitch_length_mirim,
            "pitch_width_mirim": venue.pitch_width_mirim,
        }
        statements.append(build_insert_statement(VENUE_SPEC, values))
        if update_teams_home_venue and venue.owner_team_id:
            team_id_escaped = str(venue.owner_team_id).replace("'", "''")
            venue_id_escaped = str(venue.id).replace("'", "''")
            statements.append(
                f"UPDATE teams SET home_venue_id = '{venue_id_escaped}' WHERE id = '{team_id_escaped}';"
            )
    return statements
