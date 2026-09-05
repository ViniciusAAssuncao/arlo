use arlo_domain::pitch::ArtroPlacement;
use arlo_math::Probability;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use arlo_domain::pitch::ArtroPlacement as EventArtroPlacement;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DuelKind {
    PassProtection,
    RouteContest,
    RunBreakthrough,
    CentralBlock,
    LateralBlock,
    ArtroBreakthrough,
    AerialDuel,
    FinishingAttempt,
}

impl DuelKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::PassProtection => "PassProtection",
            Self::RouteContest => "RouteContest",
            Self::RunBreakthrough => "RunBreakthrough",
            Self::CentralBlock => "CentralBlock",
            Self::LateralBlock => "LateralBlock",
            Self::ArtroBreakthrough => "ArtroBreakthrough",
            Self::AerialDuel => "AerialDuel",
            Self::FinishingAttempt => "FinishingAttempt",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallToActionStarted {
    offense_team_id: Uuid,
    defense_team_id: Uuid,
    passer_id: Uuid,
    artrine_id: Uuid,
    down_number: u32,
    scrimmage_x_mirim: f64,
    target_advance_mirim: f64,
}

impl CallToActionStarted {
    pub fn new(
        offense_team_id: Uuid,
        defense_team_id: Uuid,
        passer_id: Uuid,
        artrine_id: Uuid,
        down_number: u32,
        scrimmage_x_mirim: f64,
        target_advance_mirim: f64,
    ) -> Self {
        Self {
            offense_team_id,
            defense_team_id,
            passer_id,
            artrine_id,
            down_number,
            scrimmage_x_mirim,
            target_advance_mirim,
        }
    }

    pub fn offense_team_id(&self) -> Uuid {
        self.offense_team_id
    }

    pub fn defense_team_id(&self) -> Uuid {
        self.defense_team_id
    }

    pub fn passer_id(&self) -> Uuid {
        self.passer_id
    }

    pub fn artrine_id(&self) -> Uuid {
        self.artrine_id
    }

    pub fn down_number(&self) -> u32 {
        self.down_number
    }

    pub fn scrimmage_x_mirim(&self) -> f64 {
        self.scrimmage_x_mirim
    }

    pub fn target_advance_mirim(&self) -> f64 {
        self.target_advance_mirim
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PassCompleted {
    passer_id: Uuid,
    receiver_id: Uuid,
    is_aerial: bool,
    reception_x_mirim: f64,
    reception_y_mirim: f64,
    distance_mirim: f64,
}

impl PassCompleted {
    pub fn new(
        passer_id: Uuid,
        receiver_id: Uuid,
        is_aerial: bool,
        reception_x_mirim: f64,
        reception_y_mirim: f64,
        distance_mirim: f64,
    ) -> Self {
        Self {
            passer_id,
            receiver_id,
            is_aerial,
            reception_x_mirim,
            reception_y_mirim,
            distance_mirim,
        }
    }

    pub fn passer_id(&self) -> Uuid {
        self.passer_id
    }

    pub fn receiver_id(&self) -> Uuid {
        self.receiver_id
    }

    pub fn is_aerial(&self) -> bool {
        self.is_aerial
    }

    pub fn reception_x_mirim(&self) -> f64 {
        self.reception_x_mirim
    }

    pub fn reception_y_mirim(&self) -> f64 {
        self.reception_y_mirim
    }

    pub fn distance_mirim(&self) -> f64 {
        self.distance_mirim
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DriveRecorded {
    artrine_id: Uuid,
    artro_row_index: usize,
    placement: ArtroPlacement,
    drives_in_series: u32,
    x_mirim: f64,
}

impl DriveRecorded {
    pub fn new(
        artrine_id: Uuid,
        artro_row_index: usize,
        placement: ArtroPlacement,
        drives_in_series: u32,
        x_mirim: f64,
    ) -> Self {
        Self {
            artrine_id,
            artro_row_index,
            placement,
            drives_in_series,
            x_mirim,
        }
    }

    pub fn artrine_id(&self) -> Uuid {
        self.artrine_id
    }

    pub fn artro_row_index(&self) -> usize {
        self.artro_row_index
    }

    pub fn placement(&self) -> ArtroPlacement {
        self.placement
    }

    pub fn drives_in_series(&self) -> u32 {
        self.drives_in_series
    }

    pub fn x_mirim(&self) -> f64 {
        self.x_mirim
    }
}

pub type DriveRegistered = DriveRecorded;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DuelResolved {
    kind: DuelKind,
    attacker_ids: Vec<Uuid>,
    defender_ids: Vec<Uuid>,
    attacker_won: bool,
    win_probability: Probability,
    net_advantage: f64,
}

impl DuelResolved {
    pub fn new(
        kind: DuelKind,
        attacker_ids: Vec<Uuid>,
        defender_ids: Vec<Uuid>,
        attacker_won: bool,
        win_probability: Probability,
        net_advantage: f64,
    ) -> Self {
        Self {
            kind,
            attacker_ids,
            defender_ids,
            attacker_won,
            win_probability,
            net_advantage,
        }
    }

    pub fn single(
        kind: DuelKind,
        attacker_id: Uuid,
        defender_id: Uuid,
        attacker_won: bool,
        win_probability: Probability,
        net_advantage: f64,
    ) -> Self {
        Self {
            kind,
            attacker_ids: vec![attacker_id],
            defender_ids: vec![defender_id],
            attacker_won,
            win_probability,
            net_advantage,
        }
    }

    pub fn kind(&self) -> DuelKind {
        self.kind
    }

    pub fn attacker_ids(&self) -> &[Uuid] {
        &self.attacker_ids
    }

    pub fn defender_ids(&self) -> &[Uuid] {
        &self.defender_ids
    }

    pub fn primary_attacker(&self) -> Option<Uuid> {
        self.attacker_ids.first().copied()
    }

    pub fn primary_defender(&self) -> Option<Uuid> {
        self.defender_ids.first().copied()
    }

    pub fn attacker_won(&self) -> bool {
        self.attacker_won
    }

    pub fn defender_won(&self) -> bool {
        !self.attacker_won
    }

    pub fn win_probability(&self) -> Probability {
        self.win_probability
    }

    pub fn net_advantage(&self) -> f64 {
        self.net_advantage
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ActionEvent {
    CallToActionStarted(CallToActionStarted),
    PassCompleted(PassCompleted),
    DriveRecorded(DriveRecorded),
    DuelResolved(DuelResolved),
}

impl From<CallToActionStarted> for ActionEvent {
    fn from(ev: CallToActionStarted) -> Self {
        Self::CallToActionStarted(ev)
    }
}

impl From<PassCompleted> for ActionEvent {
    fn from(ev: PassCompleted) -> Self {
        Self::PassCompleted(ev)
    }
}

impl From<DriveRecorded> for ActionEvent {
    fn from(ev: DriveRecorded) -> Self {
        Self::DriveRecorded(ev)
    }
}

impl From<DuelResolved> for ActionEvent {
    fn from(ev: DuelResolved) -> Self {
        Self::DuelResolved(ev)
    }
}