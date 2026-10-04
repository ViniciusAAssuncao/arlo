from dataclasses import dataclass
from typing import List, Optional
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.domain.tie_break_criterion_draft import TieBreakCriterionDraft
from arlo_db_tool.domain.weekday_draft import WeekdayDraft
from arlo_db_tool.diff.config_core_diff import ConfigCoreDiffResult, diff_config_core
from arlo_db_tool.diff.group_diff import GroupCollectionDiffResult, diff_groups
from arlo_db_tool.diff.stage_diff import StageCollectionDiffResult, diff_stages

@dataclass
class AggregateDiffResult:
    config_id: str
    core_diff: ConfigCoreDiffResult
    weekdays_changed: bool
    current_weekdays: List[WeekdayDraft]
    tie_break_criteria_changed: bool
    current_tie_break_criteria: List[TieBreakCriterionDraft]
    groups_diff: GroupCollectionDiffResult
    stages_diff: StageCollectionDiffResult

    @property
    def has_changes(self) -> bool:
        return bool(
            self.core_diff.has_changes
            or self.weekdays_changed
            or self.tie_break_criteria_changed
            or self.groups_diff.has_changes
            or self.stages_diff.has_changes
        )

def diff_aggregate(
    original: Optional[LeagueCalendarConfigDraft],
    current: LeagueCalendarConfigDraft,
) -> AggregateDiffResult:
    core_diff = diff_config_core(original, current)

    orig_weekdays = [w.weekday_order_index for w in (original.weekdays if original else [])]
    curr_weekdays = [w.weekday_order_index for w in current.weekdays]
    weekdays_changed = (original is None) or (sorted(orig_weekdays) != sorted(curr_weekdays))

    orig_criteria = [
        c.criterion_kind
        for c in sorted(original.tie_break_criteria if original else [], key=lambda x: x.order_index)
    ]
    curr_criteria = [
        c.criterion_kind
        for c in sorted(current.tie_break_criteria, key=lambda x: x.order_index)
    ]
    tie_break_changed = (original is None) or (orig_criteria != curr_criteria)

    groups_diff = diff_groups(original.groups if original else None, current.groups)
    stages_diff = diff_stages(original.stages if original else None, current.stages)

    return AggregateDiffResult(
        config_id=current.id,
        core_diff=core_diff,
        weekdays_changed=weekdays_changed,
        current_weekdays=current.weekdays,
        tie_break_criteria_changed=tie_break_changed,
        current_tie_break_criteria=current.tie_break_criteria,
        groups_diff=groups_diff,
        stages_diff=stages_diff,
    )