use crate::injury::outcome::InjuryIncidentResolution;
use crate::officiating::foul::FoulResolution;
use crate::resolution::AttributedDuelOutcome;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OpenPlayLoopState {
    pub current_carrier_id: Uuid,
    pub accumulated_mirins: f64,
    pub accumulated_drives: u32,
    pub duels: Vec<AttributedDuelOutcome>,
    pub fouls: Vec<FoulResolution>,
    pub injuries: Vec<InjuryIncidentResolution>,
}

impl OpenPlayLoopState {
    pub fn new(current_carrier_id: Uuid) -> Self {
        Self {
            current_carrier_id,
            accumulated_mirins: 0.0,
            accumulated_drives: 0,
            duels: Vec::new(),
            fouls: Vec::new(),
            injuries: Vec::new(),
        }
    }

    pub fn current_carrier_id(&self) -> Uuid {
        self.current_carrier_id
    }

    pub fn accumulated_mirins(&self) -> f64 {
        self.accumulated_mirins
    }

    pub fn accumulated_drives(&self) -> u32 {
        self.accumulated_drives
    }

    pub fn duels(&self) -> &[AttributedDuelOutcome] {
        &self.duels
    }

    pub fn fouls(&self) -> &[FoulResolution] {
        &self.fouls
    }

    pub fn injuries(&self) -> &[InjuryIncidentResolution] {
        &self.injuries
    }

    pub fn set_carrier(&mut self, carrier_id: Uuid) {
        self.current_carrier_id = carrier_id;
    }

    pub fn record_advance(&mut self, mirins: f64) {
        self.accumulated_mirins += mirins.max(0.0);
    }

    pub fn record_drives(&mut self, drives: u32) {
        self.accumulated_drives += drives;
    }

    pub fn add_duel(&mut self, duel: AttributedDuelOutcome) {
        self.duels.push(duel);
    }

    pub fn add_foul(&mut self, foul: FoulResolution) {
        self.fouls.push(foul);
    }

    pub fn add_injury(&mut self, injury: InjuryIncidentResolution) {
        self.injuries.push(injury);
    }
}
