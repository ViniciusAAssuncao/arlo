from dataclasses import dataclass
from typing import Dict, List


@dataclass(frozen=True)
class CascadeRelation:
    child_table: str
    child_fk_column: str
    parent_table: str
    parent_pk_column: str = "id"


TABLE_DELETION_ORDER: List[str] = [
    "schedule_block_pool_groups",
    "league_calendar_stage_schedule_blocks",
    "league_calendar_stage_entry_rule_pools",
    "league_calendar_stage_definitions",
    "competition_group_teams",
    "competition_groups",
    "league_calendar_tie_break_criteria",
    "league_calendar_matchday_weekdays",
    "league_calendar_configs",
    "rules",
    "titles",
    "player_attributes",
    "player_positions",
    "players",
    "manager_preferred_formations",
    "manager_tactical_profiles",
    "manager_attributes",
    "managers",
    "referee_attributes",
    "referees",
    "persons",
    "leagues",
    "teams",
    "venues",
    "competitions",
    "countries",
    "federations",
    "continents",
]

TABLE_PRIMARY_KEYS: Dict[str, str] = {
    "leagues": "competition_id",
    "player_positions": "player_id",
    "player_attributes": "player_id",
    "manager_attributes": "manager_id",
    "referee_attributes": "referee_id",
}


def get_table_primary_key(table_name: str) -> str:
    return TABLE_PRIMARY_KEYS.get(table_name, "id")