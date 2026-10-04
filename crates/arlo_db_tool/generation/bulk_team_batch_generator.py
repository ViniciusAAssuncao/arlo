from dataclasses import dataclass, field
import random
from typing import List, Optional, Sequence, Set, Tuple
import uuid
from arlo_db_tool.generation.nationality_sampler import sample_nationality
from arlo_db_tool.schema.entities.team import TEAM_SPEC
from arlo_db_tool.sql.statement_builder import build_insert_statement
from arlo_db_tool.utils.date_helpers import date_to_unix_seconds

CITY_NAMES: List[str] = [
    "Aethelgard",
    "Alverton",
    "Astra",
    "Belmonte",
    "Bravos",
    "Caelum",
    "Corinth",
    "Drakensberg",
    "Dunharrow",
    "Eldoria",
    "Elysium",
    "Falkirk",
    "Frostfall",
    "Galandor",
    "Gideon",
    "Helios",
    "Highland",
    "Ilyria",
    "Ironhold",
    "Jareth",
    "Kallan",
    "Kestrel",
    "Lorn",
    "Lunaria",
    "Merrick",
    "Morvath",
    "Nadir",
    "Navarra",
    "Oakhaven",
    "Orion",
    "Palas",
    "Porto",
    "Quorra",
    "Ravenna",
    "Rohan",
    "Silverstone",
    "Solaria",
    "Talon",
    "Thessala",
    "Ursa",
    "Valder",
    "Valora",
    "Westfall",
    "Windermere",
    "Xanadu",
    "Yorvik",
    "Zephyr",
    "Zion",
]

TEAM_NICKNAMES: List[str] = [
    "Athletic",
    "United",
    "Rovers",
    "City",
    "Town",
    "Dynamo",
    "Strikers",
    "Vanguard",
    "Warriors",
    "Titans",
    "Lions",
    "Wolves",
    "Eagles",
    "Falcons",
    "Knights",
    "Guardians",
    "Harriers",
    "Phoenix",
    "Storm",
    "Dragons",
    "Stars",
    "Blazers",
    "Rangers",
    "Miners",
    "Ironclads",
    "Spartans",
    "Hawks",
    "Bears",
    "Cobras",
    "Tempest",
]

TEAM_HEX_COLORS: List[str] = [
    "#E63946",
    "#1D3557",
    "#457B9D",
    "#A8DADC",
    "#2A9D8F",
    "#E76F51",
    "#F4A261",
    "#264653",
    "#6A4C93",
    "#1982C4",
    "#8AC926",
    "#FFCA3A",
    "#FF595E",
    "#003049",
    "#D62828",
    "#F77F00",
    "#FCBF49",
    "#0077B6",
    "#023E8A",
    "#03045E",
    "#2D6A4F",
    "#1B4332",
    "#081C15",
    "#52B788",
    "#74C69D",
    "#B7094C",
    "#892B64",
    "#5C4D7D",
    "#455E89",
    "#2E6F95",
]


@dataclass
class TeamDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    name: str = ""
    country_id: str = ""
    league_id: Optional[str] = None
    founded_at_unix_seconds: int = 0
    prestige: int = 500
    primary_color_hex: Optional[str] = None
    secondary_color_hex: Optional[str] = None
    home_venue_id: Optional[str] = None


def generate_team_name(
    used_names: Set[str],
    rng: Optional[random.Random] = None,
) -> str:
    r = rng if rng is not None else random.Random()
    for _ in range(100):
        city = r.choice(CITY_NAMES)
        nick = r.choice(TEAM_NICKNAMES)
        candidate = f"{city} {nick}"
        if candidate not in used_names:
            used_names.add(candidate)
            return candidate

    suffix = 1
    while True:
        candidate = f"{r.choice(CITY_NAMES)} {r.choice(TEAM_NICKNAMES)} {suffix}"
        if candidate not in used_names:
            used_names.add(candidate)
            return candidate
        suffix += 1


def generate_team_batch(
    count: int,
    league_id: Optional[str] = None,
    country_id: Optional[str] = None,
    available_country_ids: Sequence[str] = (),
    prestige_range: Tuple[int, int] = (200, 750),
    reference_year: int = 3627,
    existing_team_names: Optional[Sequence[str]] = None,
    rng: Optional[random.Random] = None,
) -> List[TeamDraft]:
    r = rng if rng is not None else random.Random()
    used_names: Set[str] = set(existing_team_names or [])
    min_p = max(0, min(1000, min(prestige_range)))
    max_p = max(0, min(1000, max(prestige_range)))

    teams: List[TeamDraft] = []
    shuffled_colors = list(TEAM_HEX_COLORS)
    r.shuffle(shuffled_colors)

    for i in range(count):
        team_id = str(uuid.uuid4())
        name = generate_team_name(used_names, rng=r)

        if country_id and str(country_id).strip():
            nat_id = str(country_id).strip()
        else:
            nat_id = sample_nationality(
                available_country_ids=available_country_ids,
                rng=r,
            )

        if count > 1:
            fraction = i / (count - 1)
            raw_prestige = min_p + fraction * (max_p - min_p)
            jitter = r.randint(-20, 20)
            prestige = int(max(min_p, min(max_p, round(raw_prestige + jitter))))
        else:
            prestige = int(round((min_p + max_p) / 2.0))

        founded_year = r.randint(max(1, reference_year - 120), max(1, reference_year - 5))
        founded_month = r.randint(1, 12)
        founded_day = r.randint(1, 28)
        founded_unix = date_to_unix_seconds(founded_year, founded_month, founded_day)

        c1 = shuffled_colors[(i * 2) % len(shuffled_colors)]
        c2 = shuffled_colors[(i * 2 + 1) % len(shuffled_colors)]
        if c1 == c2:
            c2 = shuffled_colors[(i * 2 + 2) % len(shuffled_colors)]

        team = TeamDraft(
            id=team_id,
            name=name,
            country_id=nat_id,
            league_id=league_id,
            founded_at_unix_seconds=founded_unix,
            prestige=prestige,
            primary_color_hex=c1,
            secondary_color_hex=c2,
            home_venue_id=None,
        )
        teams.append(team)

    return teams


def build_team_insert_statements(teams: List[TeamDraft]) -> List[str]:
    statements: List[str] = []
    for team in teams:
        values = {
            "id": team.id,
            "name": team.name,
            "country_id": team.country_id,
            "league_id": team.league_id,
            "founded_at_unix_seconds": team.founded_at_unix_seconds,
            "prestige": team.prestige,
            "primary_color_hex": team.primary_color_hex,
            "secondary_color_hex": team.secondary_color_hex,
            "home_venue_id": team.home_venue_id,
        }
        statements.append(build_insert_statement(TEAM_SPEC, values))
    return statements
