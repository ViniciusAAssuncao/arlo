use crate::attributes::PlayerAttributeTable;
use crate::physical::systems::degradation::{
    calculate_physical_exhaustion, extract_effective_attribute_value,
};
use crate::physical::PhysicalState;
use crate::psychology::state::ImpulseState;
use crate::psychology::systems::baseline::{
    calculate_captaincy_influence, calculate_player_contextual_baseline_from_table,
};
use crate::psychology::systems::dynamics::fatigue_depression;
use arlo_domain::sport_constants::{
    impulse_floor_for_baseline, CAPTAINCY_LOSS_AVERSION_BUFFER, HOME_MOMENTUM_RESILIENCE_BOOST,
    IMPULSE_SCALE_MAX,
};
use arlo_domain::{AttributeKey, Player};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

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
}

impl ImpulseEventKind {
    pub fn is_positive(self) -> bool {
        match self {
            Self::DuelWon
            | Self::ScoreFor
            | Self::TurnoverWon
            | Self::SeriesSuccess
            | Self::MilestoneStreak
            | Self::BigPlayCompleted => true,
            Self::DuelLost
            | Self::ScoreAgainst
            | Self::TurnoverCommitted
            | Self::SeriesFailure
            | Self::BigPlayAllowed => false,
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

pub fn calculate_reaction_scale(
    determination: f64,
    bravery: f64,
    composure: f64,
    consistency: f64,
    involved: bool,
) -> f64 {
    let norm_det = determination.clamp(0.0, 20.0) / 10.0;
    let norm_brav = bravery.clamp(0.0, 20.0) / 10.0;
    let norm_comp = composure.clamp(0.0, 20.0) / 10.0;
    let norm_cons = consistency.clamp(0.0, 20.0) / 10.0;

    let involvement_factor = if involved { 1.0 } else { 0.45 };
    let mental_drive = (0.4 * norm_det + 0.35 * norm_brav + 0.25 * norm_comp).clamp(0.5, 2.0);
    let stability_dampener = (1.0 / (0.6 + 0.25 * norm_comp + 0.15 * norm_cons)).clamp(0.5, 1.5);

    12.0 * involvement_factor * mental_drive * stability_dampener
}

pub fn calculate_loss_aversion_lambda(
    composure: f64,
    determination: f64,
    bravery: f64,
    exhaustion: f64,
) -> f64 {
    let norm_comp = composure.clamp(0.0, 20.0) / 10.0;
    let norm_det = determination.clamp(0.0, 20.0) / 10.0;
    let norm_brav = bravery.clamp(0.0, 20.0) / 10.0;

    let base_lambda =
        2.25 - 0.4 * (norm_comp - 1.0) - 0.35 * (norm_det - 1.0) - 0.25 * (norm_brav - 1.0)
            + 0.5 * exhaustion.clamp(0.0, 1.0);

    base_lambda.clamp(1.1, 3.8)
}

pub fn calculate_contextual_loss_aversion_lambda_from_table(
    table: &PlayerAttributeTable,
    exhaustion: f64,
    captain_influence: f64,
) -> f64 {
    let composure = extract_effective_attribute_value(
        table,
        AttributeKey::Composure,
        &PhysicalState::initial(),
    );
    let determination = extract_effective_attribute_value(
        table,
        AttributeKey::Determination,
        &PhysicalState::initial(),
    );
    let bravery =
        extract_effective_attribute_value(table, AttributeKey::Bravery, &PhysicalState::initial());
    let base_lambda = calculate_loss_aversion_lambda(composure, determination, bravery, exhaustion);
    let captain_modifier = captain_influence * CAPTAINCY_LOSS_AVERSION_BUFFER;
    (base_lambda - captain_modifier).clamp(1.1, 3.8)
}

pub fn calculate_contextual_loss_aversion_lambda(
    composure: f64,
    determination: f64,
    bravery: f64,
    exhaustion: f64,
    captain: Option<&Player>,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
) -> f64 {
    let base_lambda = calculate_loss_aversion_lambda(composure, determination, bravery, exhaustion);
    let captain_modifier = match captain {
        Some(cap) => {
            let influence = calculate_captaincy_influence(cap, attribute_keys);
            influence * CAPTAINCY_LOSS_AVERSION_BUFFER
        }
        None => 0.0,
    };
    (base_lambda - captain_modifier).clamp(1.1, 3.8)
}

pub fn apply_impulse_event_contextual_from_table_at(
    state: &mut ImpulseState,
    player: &Player,
    table: &PlayerAttributeTable,
    physical_state: &PhysicalState,
    event: &ImpulseEvent,
    timestamp_seconds: f64,
    captain_influence: f64,
    is_captain: bool,
    is_home: bool,
) -> ImpulseShift {
    let is_positive = event.kind().is_positive();
    let sign = if is_positive { 1.0 } else { -1.0 };

    let determination =
        extract_effective_attribute_value(table, AttributeKey::Determination, physical_state);
    let bravery = extract_effective_attribute_value(table, AttributeKey::Bravery, physical_state);
    let composure =
        extract_effective_attribute_value(table, AttributeKey::Composure, physical_state);
    let consistency =
        extract_effective_attribute_value(table, AttributeKey::Consistency, physical_state);

    let exhaustion = calculate_physical_exhaustion(physical_state);

    let reaction_scale = calculate_reaction_scale(
        determination,
        bravery,
        composure,
        consistency,
        event.involved(),
    );

    let surprisal_norm = (event.surprisal().max(0.0) / 4.605).clamp(0.0, 2.0);
    let epv_norm = (event.epv_delta().abs() / 5.0).clamp(0.0, 2.0);
    let raw_stimulus = ALPHA_SURPRISAL_WEIGHT * surprisal_norm + BETA_EPV_WEIGHT * epv_norm;
    let base_magnitude = reaction_scale * raw_stimulus;

    let base_lambda = calculate_loss_aversion_lambda(composure, determination, bravery, exhaustion);
    let captain_modifier = captain_influence * CAPTAINCY_LOSS_AVERSION_BUFFER;
    let effective_lambda = (base_lambda - captain_modifier).clamp(1.1, 3.8);

    let baseline = calculate_player_contextual_baseline_from_table(
        player,
        table,
        captain_influence,
        is_captain,
        is_home,
    );
    let base_floor = impulse_floor_for_baseline(baseline);
    let fatigue_dep = fatigue_depression(physical_state);
    let effective_floor = (base_floor * fatigue_dep).clamp(0.0, baseline);
    let norm_det = determination.clamp(0.0, 20.0) / 10.0;
    let effective_ceiling = (baseline
        + ((IMPULSE_SCALE_MAX as f64) - baseline) * (0.35 + 0.3 * (norm_det / 2.0)))
        .clamp(baseline, IMPULSE_SCALE_MAX as f64);

    let momentum = state.momentum_index(timestamp_seconds);
    let raw_momentum_multiplier = state.momentum_multiplier_for(is_positive, momentum);
    let momentum_multiplier = if !is_positive && is_home {
        (raw_momentum_multiplier * (1.0 - HOME_MOMENTUM_RESILIENCE_BOOST)).max(0.4)
    } else if is_positive && is_home {
        raw_momentum_multiplier * 1.05
    } else {
        raw_momentum_multiplier
    };

    let magnitude = if is_positive {
        base_magnitude * momentum_multiplier
    } else {
        base_magnitude * effective_lambda * momentum_multiplier
    };

    let delta = sign * magnitude;
    let previous_accumulator = state.accumulator();
    let previous_value = state.value();

    let new_accumulator = (previous_accumulator + delta).clamp(effective_floor, effective_ceiling);
    state.set_accumulator(new_accumulator);
    state.record_event(is_positive, raw_stimulus.max(0.1), timestamp_seconds);

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

pub fn apply_impulse_event_contextual_at(
    state: &mut ImpulseState,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    event: &ImpulseEvent,
    timestamp_seconds: f64,
    captain: Option<&Player>,
    is_home: bool,
) -> ImpulseShift {
    let table = PlayerAttributeTable::from_player(player, attribute_keys);
    let (captain_influence, is_captain) = match captain {
        Some(cap) => (
            calculate_captaincy_influence(cap, attribute_keys),
            cap.id() == player.id(),
        ),
        None => (0.0, false),
    };
    apply_impulse_event_contextual_from_table_at(
        state,
        player,
        &table,
        physical_state,
        event,
        timestamp_seconds,
        captain_influence,
        is_captain,
        is_home,
    )
}

pub fn apply_impulse_event_at(
    state: &mut ImpulseState,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    event: &ImpulseEvent,
    timestamp_seconds: f64,
) -> ImpulseShift {
    apply_impulse_event_contextual_at(
        state,
        player,
        attribute_keys,
        physical_state,
        event,
        timestamp_seconds,
        None,
        false,
    )
}

pub fn apply_impulse_event(
    state: &mut ImpulseState,
    player: &Player,
    attribute_keys: &HashMap<Uuid, AttributeKey>,
    physical_state: &PhysicalState,
    event: &ImpulseEvent,
) -> ImpulseShift {
    apply_impulse_event_at(state, player, attribute_keys, physical_state, event, 0.0)
}
