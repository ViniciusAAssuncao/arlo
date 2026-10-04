from typing import List, Optional
from arlo_db_tool.diff.aggregate_diff import diff_aggregate
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.sql.config_statement_builder import build_config_update_statement
from arlo_db_tool.sql.entry_rule_pool_statement_builder import (
    build_entry_rule_pool_delete_statement,
    build_entry_rule_pool_insert_statement,
    build_entry_rule_pool_update_statement,
)
from arlo_db_tool.sql.group_statement_builder import (
    build_group_delete_cascade_statements,
    build_group_insert_statements,
    build_group_team_delete_statement,
    build_group_team_insert_statement,
    build_group_update_statement,
)
from arlo_db_tool.sql.schedule_block_statement_builder import (
    build_schedule_block_delete_cascade_statements,
    build_schedule_block_pool_group_delete_statement,
    build_schedule_block_pool_group_insert_statement,
    build_schedule_block_statements,
    build_schedule_block_update_statement,
)
from arlo_db_tool.sql.stage_statement_builder import (
    build_stage_delete_cascade_statements,
    build_stage_statements,
    build_stage_update_statement,
)
from arlo_db_tool.sql.tie_break_statement_builder import (
    build_tie_break_delete_all_statement,
    build_tie_break_insert_statements,
)
from arlo_db_tool.sql.weekday_statement_builder import (
    build_weekday_delete_all_statement,
    build_weekday_insert_statements,
)

def gerar_statements_update(
    original_draft: Optional[LeagueCalendarConfigDraft],
    current_draft: LeagueCalendarConfigDraft,
) -> List[str]:
    diff_res = diff_aggregate(original_draft, current_draft)
    if not diff_res.has_changes:
        return []

    statements: List[str] = []

    for stage_diff in diff_res.stages_diff.updated:
        for block_diff in stage_diff.schedule_blocks_diff.updated:
            for group_id in block_diff.pool_group_ids_diff.removed:
                statements.append(
                    build_schedule_block_pool_group_delete_statement(
                        block_diff.block_id, group_id
                    )
                )

    for stage_diff in diff_res.stages_diff.updated:
        for block in stage_diff.schedule_blocks_diff.removed:
            statements.extend(build_schedule_block_delete_cascade_statements(block.id))

    for stage_diff in diff_res.stages_diff.updated:
        for pool in stage_diff.entry_rule_pools_diff.removed:
            statements.append(build_entry_rule_pool_delete_statement(pool.id))

    for stage in diff_res.stages_diff.removed:
        statements.extend(build_stage_delete_cascade_statements(stage.id))

    for group_diff in diff_res.groups_diff.updated:
        for team_id in group_diff.team_ids_diff.removed:
            statements.append(
                build_group_team_delete_statement(group_diff.group_id, team_id)
            )

    for group in diff_res.groups_diff.removed:
        statements.extend(build_group_delete_cascade_statements(group.id))

    if diff_res.weekdays_changed:
        statements.append(build_weekday_delete_all_statement(diff_res.config_id))

    if diff_res.tie_break_criteria_changed:
        statements.append(build_tie_break_delete_all_statement(diff_res.config_id))

    if diff_res.core_diff.has_changes:
        stmt = build_config_update_statement(
            diff_res.config_id, diff_res.core_diff.changed_fields
        )
        if stmt:
            statements.append(stmt)

    for group_diff in diff_res.groups_diff.updated:
        if group_diff.scalar_changes:
            stmt = build_group_update_statement(
                group_diff.group_id, group_diff.scalar_changes
            )
            if stmt:
                statements.append(stmt)

    for stage_diff in diff_res.stages_diff.updated:
        if stage_diff.scalar_changes:
            stmt = build_stage_update_statement(
                stage_diff.stage_id, stage_diff.scalar_changes
            )
            if stmt:
                statements.append(stmt)

        for pool_diff in stage_diff.entry_rule_pools_diff.updated:
            if pool_diff.scalar_changes:
                stmt = build_entry_rule_pool_update_statement(
                    pool_diff.pool_id, pool_diff.scalar_changes
                )
                if stmt:
                    statements.append(stmt)

        for block_diff in stage_diff.schedule_blocks_diff.updated:
            if block_diff.scalar_changes:
                stmt = build_schedule_block_update_statement(
                    block_diff.block_id, block_diff.scalar_changes
                )
                if stmt:
                    statements.append(stmt)

    if diff_res.weekdays_changed and diff_res.current_weekdays:
        statements.extend(
            build_weekday_insert_statements(
                diff_res.config_id, diff_res.current_weekdays
            )
        )

    if diff_res.tie_break_criteria_changed and diff_res.current_tie_break_criteria:
        statements.extend(
            build_tie_break_insert_statements(
                diff_res.config_id, diff_res.current_tie_break_criteria
            )
        )

    if diff_res.groups_diff.added:
        statements.extend(
            build_group_insert_statements(
                diff_res.config_id, diff_res.groups_diff.added
            )
        )

    for group_diff in diff_res.groups_diff.updated:
        for team_id in group_diff.team_ids_diff.added:
            statements.append(
                build_group_team_insert_statement(group_diff.group_id, team_id)
            )

    for stage in diff_res.stages_diff.added:
        statements.extend(build_stage_statements(diff_res.config_id, stage))

    for stage_diff in diff_res.stages_diff.updated:
        for pool in stage_diff.entry_rule_pools_diff.added:
            statements.append(
                build_entry_rule_pool_insert_statement(stage_diff.stage_id, pool)
            )

        for block in stage_diff.schedule_blocks_diff.added:
            statements.extend(
                build_schedule_block_statements(stage_diff.stage_id, [block])
            )

        for block_diff in stage_diff.schedule_blocks_diff.updated:
            for group_id in block_diff.pool_group_ids_diff.added:
                statements.append(
                    build_schedule_block_pool_group_insert_statement(
                        block_diff.block_id, group_id
                    )
                )

    if not statements:
        return []

    return ["BEGIN TRANSACTION;"] + statements + ["COMMIT;"]

def generate_update_statements(
    original_draft: Optional[LeagueCalendarConfigDraft],
    current_draft: LeagueCalendarConfigDraft,
) -> List[str]:
    return gerar_statements_update(original_draft, current_draft)