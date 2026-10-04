from arlo_db_tool.schema.entities.attribute_definition import (
    ATTRIBUTE_CATEGORIES,
    ATTRIBUTE_DEFINITION_SPEC,
    ATTRIBUTE_KEYS,
    ATTRIBUTE_TARGETS,
)
from arlo_db_tool.schema.entities.competition import (
    COMPETITION_KINDS,
    COMPETITION_SCOPES,
    COMPETITION_SPEC,
)
from arlo_db_tool.schema.entities.continent import CONTINENT_SPEC
from arlo_db_tool.schema.entities.country import COUNTRY_SPEC
from arlo_db_tool.schema.entities.fault_definition import (
    FAULT_DEFINITION_SPEC,
    FAULT_SEVERITIES,
)
from arlo_db_tool.schema.entities.federation import FEDERATION_SCOPES, FEDERATION_SPEC
from arlo_db_tool.schema.entities.formation import FORMATION_SPEC
from arlo_db_tool.schema.entities.formation_slot import FORMATION_SLOT_SPEC
from arlo_db_tool.schema.entities.injury_definition import (
    BODY_REGIONS,
    INJURY_DEFINITION_SPEC,
    INJURY_MECHANISMS,
)
from arlo_db_tool.schema.entities.league import LEAGUE_SPEC
from arlo_db_tool.schema.entities.manager import (
    ARTRINE_DEPENDENCIES,
    DEFENSIVE_APPROACHES,
    MANAGER_CONTROL_MODES,
    MANAGER_SPEC,
    OFFENSIVE_APPROACHES,
    ROTATION_POLICIES,
)
from arlo_db_tool.schema.entities.manager_attribute import MANAGER_ATTRIBUTE_SPEC
from arlo_db_tool.schema.entities.person_fields import get_person_fields
from arlo_db_tool.schema.entities.player import CAPTAINCY_ROLES, PLAYER_SPEC
from arlo_db_tool.schema.entities.player_attribute import PLAYER_ATTRIBUTE_SPEC
from arlo_db_tool.schema.entities.player_position import PLAYER_POSITION_SPEC
from arlo_db_tool.schema.entities.position_codes import SHORT_POSITION_CODES
from arlo_db_tool.schema.entities.referee import (
    REFEREE_SPEC,
    REFEREE_TIERS,
)
from arlo_db_tool.schema.entities.referee_attribute import REFEREE_ATTRIBUTE_SPEC
from arlo_db_tool.schema.entities.rule import RULE_CATEGORIES, RULE_SPEC
from arlo_db_tool.schema.entities.team import TEAM_SPEC
from arlo_db_tool.schema.entities.title import TITLE_SPEC
from arlo_db_tool.schema.entities.venue import VENUE_KINDS, VENUE_SPEC

__all__ = [
    "CONTINENT_SPEC",
    "FEDERATION_SPEC",
    "FEDERATION_SCOPES",
    "COUNTRY_SPEC",
    "VENUE_SPEC",
    "VENUE_KINDS",
    "COMPETITION_SPEC",
    "COMPETITION_SCOPES",
    "COMPETITION_KINDS",
    "LEAGUE_SPEC",
    "TEAM_SPEC",
    "get_person_fields",
    "MANAGER_SPEC",
    "MANAGER_CONTROL_MODES",
    "OFFENSIVE_APPROACHES",
    "DEFENSIVE_APPROACHES",
    "ROTATION_POLICIES",
    "ARTRINE_DEPENDENCIES",
    "PLAYER_SPEC",
    "CAPTAINCY_ROLES",
    "REFEREE_SPEC",
    "REFEREE_TIERS",
    "TITLE_SPEC",
    "RULE_SPEC",
    "RULE_CATEGORIES",
    "FORMATION_SPEC",
    "FORMATION_SLOT_SPEC",
    "SHORT_POSITION_CODES",
    "ATTRIBUTE_DEFINITION_SPEC",
    "ATTRIBUTE_KEYS",
    "ATTRIBUTE_CATEGORIES",
    "ATTRIBUTE_TARGETS",
    "PLAYER_POSITION_SPEC",
    "PLAYER_ATTRIBUTE_SPEC",
    "MANAGER_ATTRIBUTE_SPEC",
    "REFEREE_ATTRIBUTE_SPEC",
    "FAULT_DEFINITION_SPEC",
    "FAULT_SEVERITIES",
    "INJURY_DEFINITION_SPEC",
    "INJURY_MECHANISMS",
    "BODY_REGIONS",
]
