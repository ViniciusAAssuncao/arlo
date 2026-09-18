use crate::psychology::math::{
    apply_desperation_buff, calculate_effective_ceiling, calculate_effective_floor,
    calculate_impulse_delta, calculate_loss_aversion_lambda_with_deficit, calculate_reaction_scale,
    calculate_shift_magnitude, desperation_dampener, desperation_floor_offset,
};
pub use crate::psychology::math::{calculate_loss_aversion_lambda, calculate_reaction_scale as calculate_reaction_scale_fn};
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::dynamics::fatigue_depression;
use arlo_domain::sport_constants::{
    CAPTAINCY_LOSS_AVERSION_BUFFER, HOME_MOMENTUM_RESILIENCE_BOOST,
};
use serde::{Deserialize, Serialize};

pub const ALPHA_SURPRISAL_WEIGHT: f64 = 0.6;
pub const BETA_EPV_WEIGHT: f64 = 0.4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImpulseEventKind {
    DuelWon,
    DuelLost,
    ScoreFor,
    ScoreAgainst,
    TurnoverCommitted,
    TurnoverWon,
    SeriesSuccess,
    SeriesFailure,
    MilestoneStreak,
    BigPlayCompleted,
    BigPlayAllowed,
    FoulCommitted,
    FoulDrawn,
}

impl ImpulseEventKind {
    pub fn is_positive(self) -> bool {
        match self {
            Self::DuelWon
            | Self::ScoreFor
            | Self::TurnoverWon
            | Self::SeriesSuccess
            | Self::MilestoneStreak
            | Self::BigPlayCompleted
            | Self::FoulDrawn => true,
            Self::DuelLost
            | Self::ScoreAgainst
            | Self::TurnoverCommitted
            | Self::SeriesFailure
            | Self::BigPlayAllowed
            | Self::FoulCommitted => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpulseEvent {
    kind: ImpulseEventKind,
    surprisal: f64,
    epv_delta: f64,
    involved: bool,
}

impl ImpulseEvent {
    pub fn new(kind: ImpulseEventKind, surprisal: f64, epv_delta: f64, involved: bool) -> Self {
        Self {
            kind,
            surprisal: surprisal.max(0.0),
            epv_delta,
            involved,
        }
    }

    pub fn from_probability(
        kind: ImpulseEventKind,
        probability: f64,
        epv_delta: f64,
        involved: bool,
    ) -> Self {
        let p = probability.clamp(0.0001, 0.9999);
        let surprisal = -p.ln();
        Self::new(kind, surprisal, epv_delta, involved)
    }

    pub fn kind(&self) -> ImpulseEventKind {
        self.kind
    }

    pub fn surprisal(&self) -> f64 {
        self.surprisal
    }

    pub fn epv_delta(&self) -> f64 {
        self.epv_delta
    }

    pub fn involved(&self) -> bool {
        self.involved
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ImpulseShift {
    delta: f64,
    previous_value: u8,
    new_value: u8,
    previous_accumulator: f64,
    new_accumulator: f64,
    momentum_multiplier: f64,
    effective_lambda: f64,
}

impl ImpulseShift {
    pub fn new(
        delta: f64,
        previous_value: u8,
        new_value: u8,
        previous_accumulator: f64,
        new_accumulator: f64,
        momentum_multiplier: f64,
        effective_lambda: f64,
    ) -> Self {
        Self {
            delta,
            previous_value,
            new_value,
            previous_accumulator,
            new_accumulator,
            momentum_multiplier,
            effective_lambda,
        }
    }

    pub fn delta(&self) -> f64 {
        self.delta
    }

    pub fn previous_value(&self) -> u8 {
        self.previous_value
    }

    pub fn new_value(&self) -> u8 {
        self.new_value
    }

    pub fn previous_accumulator(&self) -> f64 {
        self.previous_accumulator
    }

    pub fn new_accumulator(&self) -> f64 {
        self.new_accumulator
    }

    pub fn momentum_multiplier(&self) -> f64 {
        self.momentum_multiplier
    }

    pub fn effective_lambda(&self) -> f64 {
        self.effective_lambda
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PlayerImpulseContext {
    pub determination: f64,
    pub bravery: f64,
    pub composure: f64,
    pub consistency: f64,
    pub exhaustion: f64,
    pub captain_influence: f64,
    pub is_captain: bool,
    pub is_home: bool,
    pub score_deficit: i32,
}

impl PlayerImpulseContext {
    pub fn new(
        determination: f64,
        bravery: f64,
        composure: f64,
        consistency: f64,
        exhaustion: f64,
        captain_influence: f64,
        is_captain: bool,
        is_home: bool,
    ) -> Self {
        Self {
            determination,
            bravery,
            composure,
            consistency,
            exhaustion,
            captain_influence,
            is_captain,
            is_home,
            score_deficit: 0,
        }
    }

    pub fn with_deficit(
        determination: f64,
        bravery: f64,
        composure: f64,
        consistency: f64,
        exhaustion: f64,
        captain_influence: f64,
        is_captain: bool,
        is_home: bool,
        score_deficit: i32,
    ) -> Self {
        Self {
            determination,
            bravery,
            composure,
            consistency,
            exhaustion,
            captain_influence,
            is_captain,
            is_home,
            score_deficit,
        }
    }
}

pub fn apply_impulse_event(
    state: &mut ImpulseState,
    context: &PlayerImpulseContext,
    event: &ImpulseEvent,
) -> ImpulseShift {
    let is_positive = event.kind().is_positive();

    let (effective_det, effective_brav) = apply_desperation_buff(
        context.determination,
        context.bravery,
        context.score_deficit,
    );

    let reaction_scale = calculate_reaction_scale(
        context.determination,
        context.bravery,
        context.composure,
        context.consistency,
        event.involved(),
    );

    let surprisal_norm = (event.surprisal().max(0.0) / 4.605).clamp(0.0, 2.0);
    let epv_norm = (event.epv_delta().abs() / 5.0).clamp(0.0, 2.0);
    let raw_stimulus = ALPHA_SURPRISAL_WEIGHT * surprisal_norm + BETA_EPV_WEIGHT * epv_norm;
    let base_magnitude = reaction_scale * raw_stimulus;

    let base_lambda = calculate_loss_aversion_lambda_with_deficit(
        context.composure,
        context.determination,
        context.bravery,
        context.exhaustion,
        context.score_deficit,
    );
    let captain_modifier = context.captain_influence * CAPTAINCY_LOSS_AVERSION_BUFFER;
    let effective_lambda = (base_lambda - captain_modifier).clamp(1.1, 3.8);

    let baseline = state.baseline();
    let floor_offset = desperation_floor_offset(context.score_deficit, baseline);
    let fatigue_dep = fatigue_depression(context.exhaustion);
    let effective_floor = calculate_effective_floor(baseline, floor_offset, fatigue_dep);
    let effective_ceiling = calculate_effective_ceiling(baseline, effective_det);

    let raw_momentum_multiplier = state.momentum_multiplier_for(is_positive);
    let momentum_multiplier = if !is_positive && context.is_home {
        (raw_momentum_multiplier * (1.0 - HOME_MOMENTUM_RESILIENCE_BOOST)).max(0.4)
    } else if is_positive && context.is_home {
        raw_momentum_multiplier * 1.10
    } else {
        raw_momentum_multiplier
    };

    let dampener = desperation_dampener(
        context.score_deficit,
        effective_det,
        effective_brav,
    );

    let is_turnover_or_series_success = matches!(
        event.kind(),
        ImpulseEventKind::TurnoverWon | ImpulseEventKind::SeriesSuccess
    );

    let magnitude = calculate_shift_magnitude(
        base_magnitude,
        effective_lambda,
        momentum_multiplier,
        dampener,
        is_positive,
        context.is_home,
        is_turnover_or_series_success,
    );

    let delta = calculate_impulse_delta(is_positive, magnitude);
    let previous_accumulator = state.accumulator();
    let previous_value = state.value();

    let new_accumulator = (previous_accumulator + delta).clamp(effective_floor, effective_ceiling);
    state.set_accumulator(new_accumulator);

    let event_signal = if is_positive {
        raw_stimulus.clamp(0.1, 1.0)
    } else {
        -raw_stimulus.clamp(0.1, 1.0)
    };
    state.update_momentum(event_signal, 0.35);

    ImpulseShift::new(
        delta,
        previous_value,
        state.value(),
        previous_accumulator,
        new_accumulator,
        momentum_multiplier,
        effective_lambda,
    )
}