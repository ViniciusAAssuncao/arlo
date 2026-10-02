use crate::performance::observation::ObservationCategory;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct CategorySignalPolicy {
    pub importance: f64,
    pub saturation: f64,
    pub rate_scale: f64,
}

pub(crate) const fn policy_for(category: ObservationCategory) -> CategorySignalPolicy {
    match category {
        ObservationCategory::Duel => CategorySignalPolicy {
            importance: 1.00,
            saturation: 18.0,
            rate_scale: 0.08,
        },
        ObservationCategory::Pass => CategorySignalPolicy {
            importance: 0.65,
            saturation: 12.0,
            rate_scale: 0.06,
        },
        ObservationCategory::Reception => CategorySignalPolicy {
            importance: 0.75,
            saturation: 12.0,
            rate_scale: 0.08,
        },
        ObservationCategory::Carry => CategorySignalPolicy {
            importance: 0.85,
            saturation: 14.0,
            rate_scale: 0.08,
        },
        ObservationCategory::Drive => CategorySignalPolicy {
            importance: 1.00,
            saturation: 6.0,
            rate_scale: 0.45,
        },
        ObservationCategory::ArtrineDecision => CategorySignalPolicy {
            importance: 0.35,
            saturation: 10.0,
            rate_scale: 0.10,
        },
        ObservationCategory::Recovery => CategorySignalPolicy {
            importance: 0.50,
            saturation: 5.0,
            rate_scale: 0.40,
        },
        ObservationCategory::Turnover => CategorySignalPolicy {
            importance: 1.15,
            saturation: 6.0,
            rate_scale: 1.20,
        },
        ObservationCategory::Scoring => CategorySignalPolicy {
            importance: 1.45,
            saturation: 6.0,
            rate_scale: 0.55,
        },
        ObservationCategory::Assist => CategorySignalPolicy {
            importance: 1.20,
            saturation: 3.0,
            rate_scale: 0.80,
        },
        ObservationCategory::Foul => CategorySignalPolicy {
            importance: 0.75,
            saturation: 4.0,
            rate_scale: 0.35,
        },
        ObservationCategory::Punishment => CategorySignalPolicy {
            importance: 0.90,
            saturation: 4.0,
            rate_scale: 0.45,
        },
        ObservationCategory::PasserContact => CategorySignalPolicy {
            importance: 0.65,
            saturation: 4.0,
            rate_scale: 0.45,
        },
        ObservationCategory::KickFoul => CategorySignalPolicy {
            importance: 0.85,
            saturation: 3.0,
            rate_scale: 0.30,
        },
    }
}

pub(crate) fn reliability(opportunity_weight: f64, saturation: f64) -> f64 {
    if opportunity_weight <= 0.0 || saturation <= 0.0 {
        return 0.0;
    }
    1.0 - (-opportunity_weight / saturation).exp()
}
