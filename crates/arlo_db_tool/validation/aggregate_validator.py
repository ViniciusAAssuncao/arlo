from typing import List
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.validation.entry_rule_validator import validate_entry_rules
from arlo_db_tool.validation.group_order_validator import validate_group_order
from arlo_db_tool.validation.promotion_relegation_validator import validate_promotion_relegation
from arlo_db_tool.validation.schedule_block_reference_validator import validate_schedule_block_references
from arlo_db_tool.validation.stage_order_validator import validate_stage_order


def validar(draft: LeagueCalendarConfigDraft) -> List[str]:
    errors: List[str] = []
    errors.extend(validate_stage_order(draft))
    errors.extend(validate_group_order(draft))
    errors.extend(validate_schedule_block_references(draft))
    errors.extend(validate_entry_rules(draft))
    errors.extend(validate_promotion_relegation(draft))
    return errors


def validate(draft: LeagueCalendarConfigDraft) -> List[str]:
    return validar(draft)
