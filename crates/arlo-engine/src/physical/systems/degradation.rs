use crate::attributes::PlayerAttributeTable;
use crate::physical::state::PhysicalState;
use crate::psychology::state::ImpulseState;
use arlo_domain::AttributeKey;

#[derive(Debug, Clone, Copy)]
pub struct DegradationContext<'a> {
    physical_state: &'a PhysicalState,
    impulse_state: Option<&'a ImpulseState>,
    baseline: f64,
    is_cerebral_role: bool,
}

impl<'a> DegradationContext<'a> {
    pub fn new(physical_state: &'a PhysicalState) -> Self {
        Self {
            physical_state,
            impulse_state: None,
            baseline: 50.0,
            is_cerebral_role: false,
        }
    }

    pub fn with_impulse(
        physical_state: &'a PhysicalState,
        impulse_state: &'a ImpulseState,
        baseline: f64,
    ) -> Self {
        Self {
            physical_state,
            impulse_state: Some(impulse_state),
            baseline,
            is_cerebral_role: false,
        }
    }

    pub fn with_cerebral_role(mut self, is_cerebral_role: bool) -> Self {
        self.is_cerebral_role = is_cerebral_role;
        self
    }

    pub fn physical_state(&self) -> &'a PhysicalState {
        self.physical_state
    }

    pub fn impulse_state(&self) -> Option<&'a ImpulseState> {
        self.impulse_state
    }

    pub fn baseline(&self) -> f64 {
        self.baseline
    }

    pub fn is_cerebral_role(&self) -> bool {
        self.is_cerebral_role
    }
}

impl<'a> From<&'a PhysicalState> for DegradationContext<'a> {
    fn from(physical_state: &'a PhysicalState) -> Self {
        Self::new(physical_state)
    }
}

pub fn is_physical_attribute(key: AttributeKey) -> bool {
    matches!(
        key,
        AttributeKey::Acceleration
            | AttributeKey::Pace
            | AttributeKey::Agility
            | AttributeKey::Balance
            | AttributeKey::Strength
            | AttributeKey::Stamina
            | AttributeKey::JumpingReach
            | AttributeKey::NaturalFitness
    )
}

pub fn is_cerebral_attribute(key: AttributeKey) -> bool {
    matches!(
        key,
        AttributeKey::Decisions
            | AttributeKey::Vision
            | AttributeKey::Passing
            | AttributeKey::Technique
            | AttributeKey::Composure
            | AttributeKey::TacticalKnowledge
            | AttributeKey::Anticipation
    )
}

pub fn is_cognitive_or_technical_attribute(key: AttributeKey) -> bool {
    !is_physical_attribute(key)
}

pub fn calculate_physical_exhaustion(state: &PhysicalState) -> f64 {
    (1.0 - state.w_prime_balance()).max(0.0) * 0.70 + (1.0 - state.energy()).max(0.0) * 0.30
}

pub fn physical_attribute_modifier(state: &PhysicalState) -> f64 {
    let energy = state.energy().clamp(0.0, 1.0);
    let w_bal = state.w_prime_balance().clamp(0.0, 1.0);
    let combined = energy * (0.80 + 0.20 * w_bal);
    let mod_val = 0.65 + 0.35 * combined.powf(0.85);
    mod_val.clamp(0.35, 1.0)
}

pub fn cerebral_attribute_modifier(
    context: &DegradationContext<'_>,
    concentration: f64,
) -> f64 {
    let norm_conc = concentration.clamp(0.0, 20.0) / 20.0;
    let role_protection = if context.is_cerebral_role { 0.10 } else { 0.04 };
    let critical_threshold = (0.28 - 0.15 * norm_conc - role_protection).clamp(0.08, 0.35);
    let current_energy = context.physical_state.energy()
        * (0.88 + 0.12 * context.physical_state.w_prime_balance());

    let base_floor = if context.is_cerebral_role { 0.88 } else { 0.82 };

    let base_mod = if current_energy >= critical_threshold {
        let buffer =
            (current_energy - critical_threshold) / (1.0 - critical_threshold).max(1e-5);
        (0.96 + 0.04 * buffer).clamp(0.96, 1.0)
    } else {
        let deficit = (critical_threshold - current_energy) / critical_threshold.max(1e-5);
        let k = 1.0 + (1.0 - norm_conc) * 0.8;
        let decay = (-k * deficit).exp();
        (base_floor + (0.96 - base_floor) * decay).clamp(base_floor, 0.96)
    };

    let impulse_modifier = if let Some(impulse_state) = context.impulse_state {
        let impulse_delta = impulse_state.accumulator() - context.baseline;
        if impulse_delta >= 0.0 {
            let norm_excess = impulse_delta / 50.0;
            0.03 * (2.0 / (1.0 + (-2.0 * norm_excess).exp()) - 1.0)
        } else {
            let norm_deficit = -impulse_delta / 50.0;
            let max_penalty = if context.is_cerebral_role { -0.03 } else { -0.05 };
            max_penalty * (2.0 / (1.0 + (-2.0 * norm_deficit).exp()) - 1.0)
        }
    } else {
        0.0
    };

    let floor = if context.is_cerebral_role { 0.85 } else { 0.78 };
    (base_mod + impulse_modifier).clamp(floor, 1.05)
}

pub fn cognitive_technical_modifier(
    context: &DegradationContext<'_>,
    concentration: f64,
) -> f64 {
    let norm_conc = concentration.clamp(0.0, 20.0) / 20.0;
    let critical_threshold = (0.35 - 0.15 * norm_conc).clamp(0.12, 0.45);
    let current_energy = context.physical_state.energy()
        * (0.85 + 0.15 * context.physical_state.w_prime_balance());

    let base_mod = if current_energy >= critical_threshold {
        let buffer =
            (current_energy - critical_threshold) / (1.0 - critical_threshold).max(1e-5);
        (0.95 + 0.05 * buffer).clamp(0.95, 1.0)
    } else {
        let deficit = (critical_threshold - current_energy) / critical_threshold.max(1e-5);
        let k = 1.2 + (1.0 - norm_conc) * 1.0;
        let decay = (-k * deficit).exp();
        (0.78 + 0.17 * decay).clamp(0.75, 0.95)
    };

    let impulse_modifier = if let Some(impulse_state) = context.impulse_state {
        let impulse_delta = impulse_state.accumulator() - context.baseline;
        if impulse_delta >= 0.0 {
            let norm_excess = impulse_delta / 50.0;
            0.04 * (2.0 / (1.0 + (-2.5 * norm_excess).exp()) - 1.0)
        } else {
            let norm_deficit = -impulse_delta / 50.0;
            -0.06 * (2.0 / (1.0 + (-2.5 * norm_deficit).exp()) - 1.0)
        }
    } else {
        0.0
    };

    (base_mod + impulse_modifier).clamp(0.70, 1.05)
}

pub fn attribute_degradation_modifier(
    key: AttributeKey,
    context: &DegradationContext<'_>,
    concentration: f64,
) -> f64 {
    if is_physical_attribute(key) {
        physical_attribute_modifier(context.physical_state)
    } else if is_cerebral_attribute(key) {
        cerebral_attribute_modifier(context, concentration)
    } else {
        cognitive_technical_modifier(context, concentration)
    }
}

pub fn extract_effective_attribute_value(
    table: &PlayerAttributeTable,
    key: AttributeKey,
    context: &DegradationContext<'_>,
) -> f64 {
    let base_val = table.get(key);
    let concentration = table.get(AttributeKey::Concentration);
    let modifier = attribute_degradation_modifier(key, context, concentration);
    (base_val * modifier).clamp(0.0, 20.0)
}