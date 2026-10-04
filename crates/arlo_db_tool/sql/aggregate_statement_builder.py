from typing import List, Optional
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.sql.config_statement_builder import build_config_insert_statement
from arlo_db_tool.sql.group_statement_builder import build_group_insert_statements
from arlo_db_tool.sql.stage_statement_builder import build_stage_statements
from arlo_db_tool.sql.tie_break_statement_builder import build_tie_break_insert_statements
from arlo_db_tool.sql.weekday_statement_builder import build_weekday_insert_statements

def gerar_statements_create(
    draft: LeagueCalendarConfigDraft,
    created_at_unix_seconds: Optional[int] = None,
) -> List[str]:
    statements = []
    statements.append(build_config_insert_statement(draft, created_at_unix_seconds=created_at_unix_seconds))
    statements.extend(build_weekday_insert_statements(draft.id, draft.weekdays))
    statements.extend(build_tie_break_insert_statements(draft.id, draft.tie_break_criteria))
    statements.extend(build_group_insert_statements(draft.id, draft.groups))

    sorted_stages = sorted(draft.stages, key=lambda s: s.stage_order_index)
    for stage in sorted_stages:
        statements.extend(build_stage_statements(draft.id, stage))

    return statements

def generate_create_statements(
    draft: LeagueCalendarConfigDraft,
    created_at_unix_seconds: Optional[int] = None,
) -> List[str]:
    return gerar_statements_create(draft, created_at_unix_seconds=created_at_unix_seconds)