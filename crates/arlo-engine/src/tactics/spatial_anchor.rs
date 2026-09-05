use crate::error::{EngineError, EngineResult};
use crate::tactics::lineup::Lineup;
use arlo_domain::pitch::{project_slot, project_slot_mirrored, Pitch};
use arlo_domain::{Venue, VenueKind};
use arlo_math::units::Position as VectorPosition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpatialAnchorMap {
    home_anchors: HashMap<Uuid, VectorPosition>,
    away_anchors: HashMap<Uuid, VectorPosition>,
    all_anchors: HashMap<Uuid, VectorPosition>,
}

impl SpatialAnchorMap {
    pub fn new(
        pitch: &Pitch,
        home_lineup: &Lineup,
        away_lineup: &Lineup,
    ) -> EngineResult<Self> {
        Self::from_pitch(pitch, home_lineup, away_lineup)
    }

    pub fn from_pitch(
        pitch: &Pitch,
        home_lineup: &Lineup,
        away_lineup: &Lineup,
    ) -> EngineResult<Self> {
        let mut home_anchors = HashMap::with_capacity(home_lineup.len());
        let mut away_anchors = HashMap::with_capacity(away_lineup.len());
        let mut all_anchors = HashMap::with_capacity(home_lineup.len() + away_lineup.len());

        for assignment in home_lineup.assignments() {
            let player_id = assignment.player().id();
            let coord = project_slot(pitch, assignment.slot());
            let pos = VectorPosition::from_components(coord.x_meters(), coord.y_meters(), 0.0);
            home_anchors.insert(player_id, pos);
            all_anchors.insert(player_id, pos);
        }

        for assignment in away_lineup.assignments() {
            let player_id = assignment.player().id();
            if all_anchors.contains_key(&player_id) {
                return Err(EngineError::LineupConflict(player_id));
            }
            let coord = project_slot_mirrored(pitch, assignment.slot());
            let pos = VectorPosition::from_components(coord.x_meters(), coord.y_meters(), 0.0);
            away_anchors.insert(player_id, pos);
            all_anchors.insert(player_id, pos);
        }

        let expected_total = home_lineup.len() + away_lineup.len();
        if all_anchors.len() != expected_total {
            return Err(EngineError::AnchorCountMismatch {
                expected: expected_total,
                actual: all_anchors.len(),
            });
        }

        Ok(Self {
            home_anchors,
            away_anchors,
            all_anchors,
        })
    }

    pub fn from_venue(
        venue: &Venue,
        home_lineup: &Lineup,
        away_lineup: &Lineup,
    ) -> EngineResult<Self> {
        if venue.kind() != VenueKind::MatchStadium {
            return Err(EngineError::InvalidVenueKind);
        }
        let length = venue
            .pitch_length_mirim()
            .ok_or_else(|| EngineError::MissingPitchDimensions(venue.id()))?;
        let width = venue
            .pitch_width_mirim()
            .ok_or_else(|| EngineError::MissingPitchDimensions(venue.id()))?;
        let pitch = Pitch::from_mirim(length, width)?;
        Self::from_pitch(&pitch, home_lineup, away_lineup)
    }

    pub fn get(&self, player_id: &Uuid) -> Option<VectorPosition> {
        self.all_anchors.get(player_id).copied()
    }

    pub fn home_anchor(&self, player_id: &Uuid) -> Option<VectorPosition> {
        self.home_anchors.get(player_id).copied()
    }

    pub fn away_anchor(&self, player_id: &Uuid) -> Option<VectorPosition> {
        self.away_anchors.get(player_id).copied()
    }

    pub fn home_anchors(&self) -> &HashMap<Uuid, VectorPosition> {
        &self.home_anchors
    }

    pub fn away_anchors(&self) -> &HashMap<Uuid, VectorPosition> {
        &self.away_anchors
    }

    pub fn all_anchors(&self) -> &HashMap<Uuid, VectorPosition> {
        &self.all_anchors
    }

    pub fn is_home_player(&self, player_id: &Uuid) -> bool {
        self.home_anchors.contains_key(player_id)
    }

    pub fn is_away_player(&self, player_id: &Uuid) -> bool {
        self.away_anchors.contains_key(player_id)
    }

    pub fn len(&self) -> usize {
        self.all_anchors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.all_anchors.is_empty()
    }
}