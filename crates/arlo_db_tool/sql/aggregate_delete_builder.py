from typing import List
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.sql.config_statement_builder import build_config_delete_statement
from arlo_db_tool.sql.group_statement_builder import build_group_delete_cascade_statements
from arlo_db_tool.sql.stage_statement_builder import build_stage_delete_cascade_statements
from arlo_db_tool.sql.tie_break_statement_builder import build_tie_break_delete_all_statement
from arlo_db_tool.sql.weekday_statement_builder import build_weekday_delete_all_statement

def gerar_statements_delete(draft: LeagueCalendarConfigDraft) -> List[str]:
    statements: List[str] = []

    for stage in reversed(draft.stages):
        statements.extend(build_stage_delete_cascade_statements(stage.id))

    for group in reversed(draft.groups):
        statements.extend(build_group_delete_cascade_statements(group.id))

    statements.append(build_tie_break_delete_all_statement(draft.id))
    statements.append(build_weekday_delete_all_statement(draft.id))
    statements.append(build_config_delete_statement(draft.id))

    return ["BEGIN TRANSACTION;"] + statements + ["COMMIT;"]

def generate_delete_statements(draft: LeagueCalendarConfigDraft) -> List[str]:
    return gerar_statements_delete(draft)

def generate_full_delete_statements(draft: LeagueCalendarConfigDraft) -> List[str]:
    return gerar_statements_delete(draft)
