from typing import Dict, List, Optional, Union
from arlo_db_tool.schema.entities.attribute_definition import ATTRIBUTE_DEFINITION_SPEC
from arlo_db_tool.schema.entities.competition import COMPETITION_SPEC
from arlo_db_tool.schema.entities.continent import CONTINENT_SPEC
from arlo_db_tool.schema.entities.country import COUNTRY_SPEC
from arlo_db_tool.schema.entities.fault_definition import FAULT_DEFINITION_SPEC
from arlo_db_tool.schema.entities.federation import FEDERATION_SPEC
from arlo_db_tool.schema.entities.formation import FORMATION_SPEC
from arlo_db_tool.schema.entities.formation_slot import FORMATION_SLOT_SPEC
from arlo_db_tool.schema.entities.injury_definition import INJURY_DEFINITION_SPEC
from arlo_db_tool.schema.entities.league import LEAGUE_SPEC
from arlo_db_tool.schema.entities.manager import MANAGER_SPEC
from arlo_db_tool.schema.entities.manager_attribute import MANAGER_ATTRIBUTE_SPEC
from arlo_db_tool.schema.entities.player import PLAYER_SPEC
from arlo_db_tool.schema.entities.player_attribute import PLAYER_ATTRIBUTE_SPEC
from arlo_db_tool.schema.entities.player_position import PLAYER_POSITION_SPEC
from arlo_db_tool.schema.entities.referee import REFEREE_SPEC
from arlo_db_tool.schema.entities.referee_attribute import REFEREE_ATTRIBUTE_SPEC
from arlo_db_tool.schema.entities.rule import RULE_SPEC
from arlo_db_tool.schema.entities.team import TEAM_SPEC
from arlo_db_tool.schema.entities.title import TITLE_SPEC
from arlo_db_tool.schema.entities.venue import VENUE_SPEC
from arlo_db_tool.schema.entity_spec import CompositeEntitySpec, EntitySpec

REGISTRY: Dict[str, Union[EntitySpec, CompositeEntitySpec]] = {
    "Continent": CONTINENT_SPEC,
    "Federation": FEDERATION_SPEC,
    "Country": COUNTRY_SPEC,
    "Venue": VENUE_SPEC,
    "Competition": COMPETITION_SPEC,
    "League": LEAGUE_SPEC,
    "Team": TEAM_SPEC,
    "Manager": MANAGER_SPEC,
    "Player": PLAYER_SPEC,
    "Referee": REFEREE_SPEC,
    "Title": TITLE_SPEC,
    "Rule": RULE_SPEC,
    "Formation": FORMATION_SPEC,
    "FormationSlot": FORMATION_SLOT_SPEC,
    "AttributeDefinition": ATTRIBUTE_DEFINITION_SPEC,
    "PlayerPosition": PLAYER_POSITION_SPEC,
    "PlayerAttribute": PLAYER_ATTRIBUTE_SPEC,
    "ManagerAttribute": MANAGER_ATTRIBUTE_SPEC,
    "RefereeAttribute": REFEREE_ATTRIBUTE_SPEC,
    "FaultDefinition": FAULT_DEFINITION_SPEC,
    "InjuryDefinition": INJURY_DEFINITION_SPEC,
}

def get_entity_spec(name: str) -> Optional[Union[EntitySpec, CompositeEntitySpec]]:
    return REGISTRY.get(name)

def get_all_entity_names() -> List[str]:
    return list(REGISTRY.keys())
