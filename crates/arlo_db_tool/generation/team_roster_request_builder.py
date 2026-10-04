import random
from typing import List, Optional, Tuple
from arlo_db_tool.db.connection import get_db_connection
from arlo_db_tool.db.reference_lookup import get_options, get_record_by_id
from arlo_db_tool.generation.player_batch_generator import generate_player_batch
from arlo_db_tool.generation.player_draft import PlayerDraft
from arlo_db_tool.generation.squad_role_distribution import (
    build_squad_quotas_for_prestige,
)
from arlo_db_tool.generation.team_prestige_ca_mapper import (
    map_prestige_to_ca_range,
)
from arlo_db_tool.sql.bulk_player_statement_builder import (
    generate_bulk_player_sql,
)


def get_team_existing_squad_numbers(db_path: str, team_id: str) -> List[int]:
    try:
        with get_db_connection(db_path) as conn:
            cursor = conn.execute(
                "SELECT squad_number FROM players WHERE team_id = ? AND squad_number IS NOT NULL;",
                (str(team_id),),
            )
            return [
                int(row["squad_number"])
                for row in cursor.fetchall()
                if row["squad_number"] is not None
            ]
    except Exception:
        return []


def build_team_roster_batch(
    team_id: str,
    db_path: str,
    age_range: Tuple[int, int] = (18, 35),
    reference_year: int = 3627,
    preferred_country_bias: float = 0.70,
    secondary_positions_chance: float = 0.40,
    rng: Optional[random.Random] = None,
) -> List[PlayerDraft]:
    team_record = get_record_by_id(db_path, "teams", "id", team_id)
    if team_record is None:
        raise ValueError(f"Team '{team_id}' not found in database")

    raw_prestige = team_record.get("prestige", 0)
    try:
        prestige = int(raw_prestige)
    except (ValueError, TypeError):
        prestige = 0

    country_id = team_record.get("country_id")
    preferred_country_id = str(country_id) if country_id else None

    ca_range = map_prestige_to_ca_range(prestige)
    quotas = build_squad_quotas_for_prestige(prestige)
    total_count = sum(quotas.values())

    countries = get_options(db_path, "countries", id_col="id", label_expr="name")
    available_country_ids = [cid for cid, _ in countries]

    used_squad_numbers = get_team_existing_squad_numbers(db_path, team_id)

    return generate_player_batch(
        count=total_count,
        position_quotas=quotas,
        team_id=team_id,
        ca_range=ca_range,
        age_range=age_range,
        reference_year=reference_year,
        available_country_ids=available_country_ids,
        preferred_country_id=preferred_country_id,
        preferred_country_bias=preferred_country_bias,
        secondary_positions_chance=secondary_positions_chance,
        used_squad_numbers=used_squad_numbers,
        db_path=db_path,
        rng=rng,
    )


def build_team_roster_sql(
    team_id: str,
    db_path: str,
    age_range: Tuple[int, int] = (18, 35),
    reference_year: int = 3627,
    preferred_country_bias: float = 0.70,
    secondary_positions_chance: float = 0.40,
    rng: Optional[random.Random] = None,
) -> str:
    players = build_team_roster_batch(
        team_id=team_id,
        db_path=db_path,
        age_range=age_range,
        reference_year=reference_year,
        preferred_country_bias=preferred_country_bias,
        secondary_positions_chance=secondary_positions_chance,
        rng=rng,
    )
    return generate_bulk_player_sql(players)
