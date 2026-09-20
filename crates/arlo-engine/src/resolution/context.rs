use crate::resolution::duel_kind::DuelKind;
use crate::resolution::orientation::ContestOrientation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, Default)]
pub struct DuelContext {
    orientation: ContestOrientation,
    is_home_offense: bool,
    home_advantage_duel_logit: f64,
    aggression_logit_offset: f64,
    misdirection_logit_offset: f64,
    physicality_logit_offset: f64,
}

impl DuelContext {
    pub fn new(orientation: ContestOrientation, is_home_offense: bool, home_advantage_duel_logit: f64) -> Self {
        Self {
            orientation,
            is_home_offense,
            home_advantage_duel_logit,
            aggression_logit_offset: 0.0,
            misdirection_logit_offset: 0.0,
            physicality_logit_offset: 0.0,
        }
    }

    pub fn with_offsets(
        orientation: ContestOrientation,
        is_home_offense: bool,
        home_advantage_duel_logit: f64,
        aggression_logit_offset: f64,
        misdirection_logit_offset: f64,
        physicality_logit_offset: f64,
    ) -> Self {
        Self {
            orientation,
            is_home_offense,
            home_advantage_duel_logit,
            aggression_logit_offset,
            misdirection_logit_offset,
            physicality_logit_offset,
        }
    }

    pub fn neutral() -> Self {
        Self {
            orientation: ContestOrientation::Neutral,
            is_home_offense: false,
            home_advantage_duel_logit: 0.0,
            aggression_logit_offset: 0.0,
            misdirection_logit_offset: 0.0,
            physicality_logit_offset: 0.0,
        }
    }

    pub fn orientation(&self) -> ContestOrientation {
        self.orientation
    }

    pub fn with_orientation(&self, orientation: ContestOrientation) -> Self {
        if self.orientation == orientation {
            return *self;
        }
        let flip = (self.orientation == ContestOrientation::AttackerIsOffense && orientation == ContestOrientation::AttackerIsDefense) ||
                   (self.orientation == ContestOrientation::AttackerIsDefense && orientation == ContestOrientation::AttackerIsOffense);
        
        Self {
            orientation,
            aggression_logit_offset: if flip { -self.aggression_logit_offset } else { self.aggression_logit_offset },
            misdirection_logit_offset: if flip { -self.misdirection_logit_offset } else { self.misdirection_logit_offset },
            physicality_logit_offset: if flip { -self.physicality_logit_offset } else { self.physicality_logit_offset },
            ..*self
        }
    }

    pub fn is_home_offense(&self) -> bool {
        self.is_home_offense
    }

    pub fn attacker_is_home(&self) -> bool {
        match self.orientation {
            ContestOrientation::AttackerIsOffense => self.is_home_offense,
            ContestOrientation::AttackerIsDefense => !self.is_home_offense,
            ContestOrientation::Neutral => false,
        }
    }

    pub fn defender_is_home(&self) -> bool {
        match self.orientation {
            ContestOrientation::AttackerIsOffense => !self.is_home_offense,
            ContestOrientation::AttackerIsDefense => self.is_home_offense,
            ContestOrientation::Neutral => false,
        }
    }

    pub fn home_advantage_duel_logit(&self) -> f64 {
        self.home_advantage_duel_logit
    }

    pub fn aggression_logit_offset(&self) -> f64 {
        self.aggression_logit_offset
    }

    pub fn misdirection_logit_offset(&self) -> f64 {
        self.misdirection_logit_offset
    }

    pub fn physicality_logit_offset(&self) -> f64 {
        self.physicality_logit_offset
    }

    pub fn for_duel_kind(&self, kind: DuelKind) -> Self {
        if kind.is_contact_duel() {
            *self
        } else {
            Self::with_offsets(
                self.orientation,
                self.is_home_offense,
                self.home_advantage_duel_logit,
                0.0,
                self.misdirection_logit_offset,
                0.0,
            )
        }
    }
}
