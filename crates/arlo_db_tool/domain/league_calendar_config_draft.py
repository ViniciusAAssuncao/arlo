from dataclasses import dataclass, field
from typing import List, Optional
import uuid
from arlo_db_tool.domain.competition_group_draft import CompetitionGroupDraft
from arlo_db_tool.domain.stage_draft import StageDraft
from arlo_db_tool.domain.tie_break_criterion_draft import TieBreakCriterionDraft
from arlo_db_tool.domain.weekday_draft import WeekdayDraft

@dataclass
class LeagueCalendarConfigDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    competition_id: str = ""
    algorithm: str = "RoundRobinDoubleLeg"
    season_start_month_order_index: int = 0
    season_start_day_of_month: int = 1
    season_length_weeks: int = 38
    max_games_per_team_per_week: int = 1
    games_per_week_conflict_scope: str = "AcrossAllCompetitions"
    postponement_strategy_kind: str = "NextAvailableByeWeek"
    neutral_opener_enabled: bool = False
    neutral_opener_selection_strategy: Optional[str] = None
    spa_win_weight: float = 3.0
    spa_draw_weight: float = 1.0
    spa_loss_weight: float = 0.25
    spa_feo_k_factor: float = 5.0
    qta_home_win_weight: float = 1.0
    qta_away_win_weight: float = 0.9
    qta_home_draw_weight: float = 0.6
    qta_away_draw_weight: float = 0.55
    qta_home_loss_weight: float = -0.2
    qta_away_loss_weight: float = -0.15
    standings_stage_order_index: int = 0
    promotion_rule_kind: str = "None"
    promotion_count: Optional[int] = None
    promotion_playoff_stage_order_index: Optional[int] = None
    promotion_target_league_id: Optional[str] = None
    relegation_rule_kind: str = "None"
    relegation_count: Optional[int] = None
    relegation_playoff_stage_order_index: Optional[int] = None
    relegation_target_league_id: Optional[str] = None
    weekdays: List[WeekdayDraft] = field(default_factory=list)
    tie_break_criteria: List[TieBreakCriterionDraft] = field(default_factory=list)
    groups: List[CompetitionGroupDraft] = field(default_factory=list)
    stages: List[StageDraft] = field(default_factory=list)

    @property
    def schedule_algorithm_kind(self) -> str:
        return self.algorithm

    @schedule_algorithm_kind.setter
    def schedule_algorithm_kind(self, value: str) -> None:
        self.algorithm = value

    def renumerar(self) -> None:
        for idx, criterion in enumerate(self.tie_break_criteria):
            criterion.order_index = idx
        for idx, group in enumerate(self.groups):
            group.order_index = idx
        for idx, stage in enumerate(self.stages):
            stage.stage_order_index = idx
            stage.renumerar()

    def renumber(self) -> None:
        self.renumerar()
