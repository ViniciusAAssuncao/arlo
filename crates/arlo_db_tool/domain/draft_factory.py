from typing import Optional
import uuid
from arlo_db_tool.domain.entry_rule_pool_draft import EntryRulePoolDraft
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.domain.stage_draft import StageDraft
from arlo_db_tool.domain.tie_break_criterion_draft import TieBreakCriterionDraft
from arlo_db_tool.domain.weekday_draft import WeekdayDraft

def create_league_calendar_config_draft(
    competition_id: Optional[str] = None,
) -> LeagueCalendarConfigDraft:
    config_id = str(uuid.uuid4())
    comp_id = competition_id if competition_id is not None else str(uuid.uuid4())

    default_weekdays = [
        WeekdayDraft(id=str(uuid.uuid4()), weekday_order_index=5),
        WeekdayDraft(id=str(uuid.uuid4()), weekday_order_index=6),
    ]

    default_criteria = [
        TieBreakCriterionDraft(
            id=str(uuid.uuid4()),
            order_index=0,
            criterion_kind="IspaTotal",
        ),
        TieBreakCriterionDraft(
            id=str(uuid.uuid4()),
            order_index=1,
            criterion_kind="QtaScore",
        ),
        TieBreakCriterionDraft(
            id=str(uuid.uuid4()),
            order_index=2,
            criterion_kind="GoalDifference",
        ),
        TieBreakCriterionDraft(
            id=str(uuid.uuid4()),
            order_index=3,
            criterion_kind="GoalPointsTotal",
        ),
        TieBreakCriterionDraft(
            id=str(uuid.uuid4()),
            order_index=4,
            criterion_kind="HeadToHead",
        ),
        TieBreakCriterionDraft(
            id=str(uuid.uuid4()),
            order_index=5,
            criterion_kind="Random",
        ),
    ]

    default_stage = StageDraft(
        id=str(uuid.uuid4()),
        stage_order_index=0,
        stage_type="RoundRobinTable",
        leg_format=None,
        entry_rule_pools=[
            EntryRulePoolDraft(
                id=str(uuid.uuid4()),
                pool_order_index=0,
                pool_kind="AllTeams",
            )
        ],
        schedule_blocks=[],
    )

    return LeagueCalendarConfigDraft(
        id=config_id,
        competition_id=comp_id,
        algorithm="RoundRobinDoubleLeg",
        season_start_month_order_index=0,
        season_start_day_of_month=1,
        season_length_weeks=38,
        max_games_per_team_per_week=1,
        games_per_week_conflict_scope="AcrossAllCompetitions",
        postponement_strategy_kind="NextAvailableByeWeek",
        neutral_opener_enabled=False,
        neutral_opener_selection_strategy=None,
        spa_win_weight=3.0,
        spa_draw_weight=1.0,
        spa_loss_weight=0.25,
        spa_feo_k_factor=5.0,
        qta_home_win_weight=1.0,
        qta_away_win_weight=0.9,
        qta_home_draw_weight=0.6,
        qta_away_draw_weight=0.55,
        qta_home_loss_weight=-0.2,
        qta_away_loss_weight=-0.15,
        standings_stage_order_index=0,
        promotion_rule_kind="None",
        promotion_count=None,
        promotion_playoff_stage_order_index=None,
        promotion_target_league_id=None,
        relegation_rule_kind="None",
        relegation_count=None,
        relegation_playoff_stage_order_index=None,
        relegation_target_league_id=None,
        weekdays=default_weekdays,
        tie_break_criteria=default_criteria,
        groups=[],
        stages=[default_stage],
    )

def create_default_league_calendar_config_draft(
    competition_id: Optional[str] = None,
) -> LeagueCalendarConfigDraft:
    return create_league_calendar_config_draft(competition_id)
