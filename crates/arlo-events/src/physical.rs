use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhysicalStrainRecorded {
    player_id: Uuid,
    energy_remaining: f64,
    w_prime_balance: f64,
    distance_delta_mirim: f64,
}

impl PhysicalStrainRecorded {
    pub fn new(
        player_id: Uuid,
        energy_remaining: f64,
        w_prime_balance: f64,
        distance_delta_mirim: f64,
    ) -> Self {
        Self {
            player_id,
            energy_remaining,
            w_prime_balance,
            distance_delta_mirim,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn energy_remaining(&self) -> f64 {
        self.energy_remaining
    }

    pub fn w_prime_balance(&self) -> f64 {
        self.w_prime_balance
    }

    pub fn distance_delta_mirim(&self) -> f64 {
        self.distance_delta_mirim
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecoveryIntervalProcessed {
    player_id: Uuid,
    recovery_amount: f64,
    duration_seconds: f64,
    new_w_prime_balance: f64,
}

impl RecoveryIntervalProcessed {
    pub fn new(
        player_id: Uuid,
        recovery_amount: f64,
        duration_seconds: f64,
        new_w_prime_balance: f64,
    ) -> Self {
        Self {
            player_id,
            recovery_amount,
            duration_seconds,
            new_w_prime_balance,
        }
    }

    pub fn player_id(&self) -> Uuid {
        self.player_id
    }

    pub fn recovery_amount(&self) -> f64 {
        self.recovery_amount
    }

    pub fn duration_seconds(&self) -> f64 {
        self.duration_seconds
    }

    pub fn new_w_prime_balance(&self) -> f64 {
        self.new_w_prime_balance
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PhysicalEvent {
    PhysicalStrainRecorded(PhysicalStrainRecorded),
    RecoveryIntervalProcessed(RecoveryIntervalProcessed),
}

impl From<PhysicalStrainRecorded> for PhysicalEvent {
    fn from(ev: PhysicalStrainRecorded) -> Self {
        Self::PhysicalStrainRecorded(ev)
    }
}

impl From<RecoveryIntervalProcessed> for PhysicalEvent {
    fn from(ev: RecoveryIntervalProcessed) -> Self {
        Self::RecoveryIntervalProcessed(ev)
    }
}