use arlo_domain::{BodyRegion, InjuryMechanism, InjurySeverityGrade};
use arlo_events::{InjuryIncidentRecorded, MatchClockInstant};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchInjuryRow {
    pub id: String,
    pub match_id: String,
    pub sequence_number: i64,
    pub period: i32,
    pub seconds_in_period: f64,
    pub player_id: String,
    pub team_id: String,
    pub mechanism: String,
    pub body_region: String,
    pub severity_grade: String,
    pub injury_definition_id: String,
    pub trigger_probability: f64,
}

impl MatchInjuryRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        sequence_number: u64,
        period: u32,
        seconds_in_period: f64,
        player_id: Uuid,
        team_id: Uuid,
        mechanism: impl Into<String>,
        body_region: impl Into<String>,
        severity_grade: impl Into<String>,
        injury_definition_id: Uuid,
        trigger_probability: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            sequence_number: sequence_number as i64,
            period: period as i32,
            seconds_in_period,
            player_id: player_id.to_string(),
            team_id: team_id.to_string(),
            mechanism: mechanism.into(),
            body_region: body_region.into(),
            severity_grade: severity_grade.into(),
            injury_definition_id: injury_definition_id.to_string(),
            trigger_probability,
        }
    }

    pub fn from_event(
        id: Uuid,
        match_id: Uuid,
        seq: u64,
        clock: MatchClockInstant,
        event: &InjuryIncidentRecorded,
    ) -> Self {
        let mech_str = match event.mechanism() {
            InjuryMechanism::Contact => "Contact",
            InjuryMechanism::NonContact => "NonContact",
        };

        let region_str = match event.body_region() {
            BodyRegion::Head => "Head",
            BodyRegion::Neck => "Neck",
            BodyRegion::Shoulder => "Shoulder",
            BodyRegion::Arm => "Arm",
            BodyRegion::Hand => "Hand",
            BodyRegion::Trunk => "Trunk",
            BodyRegion::Hip => "Hip",
            BodyRegion::Groin => "Groin",
            BodyRegion::Thigh => "Thigh",
            BodyRegion::Knee => "Knee",
            BodyRegion::Calf => "Calf",
            BodyRegion::Ankle => "Ankle",
            BodyRegion::Foot => "Foot",
        };

        let grade_str = match event.severity_grade() {
            InjurySeverityGrade::Grade1 => "Grade1",
            InjurySeverityGrade::Grade2 => "Grade2",
            InjurySeverityGrade::Grade3 => "Grade3",
        };

        Self::new(
            id,
            match_id,
            seq,
            clock.period(),
            clock.seconds_in_period(),
            event.player_id(),
            event.team_id(),
            mech_str,
            region_str,
            grade_str,
            event.injury_definition_id(),
            event.trigger_probability().value(),
        )
    }
}
