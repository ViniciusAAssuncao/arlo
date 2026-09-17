use crate::lineup_runtime::lineup::Lineup;
use crate::play_resolution::field_context::PitchState;
use arlo_domain::{ ArtroPlacement, PitchZone, Position, PositionLine, SlotRole };
use arlo_tactics::TeamInstructions;
use serde::{ Deserialize, Serialize };
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PlayerZonePresence {
    pub player_id: Uuid,
    pub team_id: Uuid,
    pub position: Position,
    pub role: SlotRole,
    pub primary_zone: PitchZone,
    pub primary_channel: ArtroPlacement,
    pub density_weight: f64,
    pub is_offense: bool,
}

impl PlayerZonePresence {
    pub fn new(
        player_id: Uuid,
        team_id: Uuid,
        position: Position,
        role: SlotRole,
        primary_zone: PitchZone,
        primary_channel: ArtroPlacement,
        density_weight: f64,
        is_offense: bool
    ) -> Self {
        Self {
            player_id,
            team_id,
            position,
            role,
            primary_zone,
            primary_channel,
            density_weight: density_weight.clamp(0.1, 2.0),
            is_offense,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ZoneOccupancySummary {
    offense_counts: HashMap<(PitchZone, ArtroPlacement), usize>,
    defense_counts: HashMap<(PitchZone, ArtroPlacement), usize>,
    offense_density: HashMap<(PitchZone, ArtroPlacement), f64>,
    defense_density: HashMap<(PitchZone, ArtroPlacement), f64>,
}

impl ZoneOccupancySummary {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_presence(&mut self, presence: &PlayerZonePresence) {
        let key = (presence.primary_zone, presence.primary_channel);
        if presence.is_offense {
            *self.offense_counts.entry(key).or_insert(0) += 1;
            *self.offense_density.entry(key).or_insert(0.0) += presence.density_weight;
        } else {
            *self.defense_counts.entry(key).or_insert(0) += 1;
            *self.defense_density.entry(key).or_insert(0.0) += presence.density_weight;
        }
    }

    pub fn offense_count(&self, zone: PitchZone, channel: ArtroPlacement) -> usize {
        self.offense_counts.get(&(zone, channel)).copied().unwrap_or(0)
    }

    pub fn defense_count(&self, zone: PitchZone, channel: ArtroPlacement) -> usize {
        self.defense_counts.get(&(zone, channel)).copied().unwrap_or(0)
    }

    pub fn offense_density(&self, zone: PitchZone, channel: ArtroPlacement) -> f64 {
        self.offense_density.get(&(zone, channel)).copied().unwrap_or(0.0)
    }

    pub fn defense_density(&self, zone: PitchZone, channel: ArtroPlacement) -> f64 {
        self.defense_density.get(&(zone, channel)).copied().unwrap_or(0.0)
    }

    pub fn net_density(&self, zone: PitchZone, channel: ArtroPlacement) -> f64 {
        self.offense_density(zone, channel) - self.defense_density(zone, channel)
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct RoleZoneMap {
    presences: HashMap<Uuid, PlayerZonePresence>,
    occupancy: ZoneOccupancySummary,
}

impl RoleZoneMap {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn presences(&self) -> &HashMap<Uuid, PlayerZonePresence> {
        &self.presences
    }

    pub fn occupancy(&self) -> &ZoneOccupancySummary {
        &self.occupancy
    }

    pub fn get_presence(&self, player_id: &Uuid) -> Option<&PlayerZonePresence> {
        self.presences.get(player_id)
    }

    pub fn players_in_sector(&self, zone: PitchZone, channel: ArtroPlacement) -> Vec<Uuid> {
        self.presences
            .values()
            .filter(|p| p.primary_zone == zone && p.primary_channel == channel)
            .map(|p| p.player_id)
            .collect()
    }

    pub fn lead_defender_in(&self, zone: PitchZone, channel: ArtroPlacement) -> Option<Uuid> {
        self.presences
            .values()
            .filter(|p| !p.is_offense && p.primary_zone == zone && p.primary_channel == channel)
            .max_by(|a, b| {
                a.density_weight.partial_cmp(&b.density_weight).unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|p| p.player_id)
    }
}

pub fn derive_position_channel(position: Position, slot_index: usize) -> ArtroPlacement {
    match position {
        Position::WingOffense | Position::TightWing | Position::WideEnd | Position::WideBlocker => {
            if slot_index % 2 == 0 {
                ArtroPlacement::LeftLateral
            } else {
                ArtroPlacement::RightLateral
            }
        }
        Position::OutsideZonerback | Position::DefensiveEnd | Position::Corridor => {
            if slot_index % 2 == 0 {
                ArtroPlacement::LeftLateral
            } else {
                ArtroPlacement::RightLateral
            }
        }
        _ => ArtroPlacement::Central,
    }
}

pub fn derive_position_zone(
    position: Position,
    is_offense: bool,
    pitch_state: &PitchState,
    instructions: &TeamInstructions
) -> PitchZone {
    if position == Position::Goalguard {
        return PitchZone::FirstZone;
    }

    if is_offense {
        let mentality_val = instructions.in_possession().mentality().value();
        match position.line() {
            PositionLine::OffensiveLine => {
                if pitch_state.normalized_proximity() >= 0.7 || mentality_val > 0.3 {
                    PitchZone::FirstZone
                } else {
                    PitchZone::SecondZone
                }
            }
            PositionLine::BackLine => {
                if pitch_state.normalized_proximity() >= 0.75 {
                    PitchZone::SecondZone
                } else {
                    PitchZone::Central
                }
            }
            PositionLine::DefenseLine => PitchZone::Central,
            PositionLine::Goalguard => PitchZone::FirstZone,
        }
    } else {
        let dline_val = instructions.out_of_possession().defensive_line_height().value();
        match position.line() {
            PositionLine::Goalguard => PitchZone::FirstZone,
            PositionLine::DefenseLine => {
                if dline_val < -0.3 { PitchZone::FirstZone } else { PitchZone::SecondZone }
            }
            PositionLine::BackLine => {
                if dline_val > 0.3 { PitchZone::Central } else { PitchZone::SecondZone }
            }
            PositionLine::OffensiveLine => PitchZone::Central,
        }
    }
}

pub fn build_role_zone_map(
    offense_lineup: &Lineup,
    offense_team_id: Uuid,
    offense_instructions: &TeamInstructions,
    defense_lineup: &Lineup,
    defense_team_id: Uuid,
    defense_instructions: &TeamInstructions,
    pitch_state: &PitchState
) -> RoleZoneMap {
    let mut presences = HashMap::with_capacity(offense_lineup.len() + defense_lineup.len());
    let mut occupancy = ZoneOccupancySummary::new();

    for assignment in offense_lineup.assignments() {
        let pid = assignment.player().id();
        let pos = assignment.slot().offensive_position();
        let role = assignment.slot_role();
        let channel = derive_position_channel(pos, assignment.formation_slot_index());
        let zone = derive_position_zone(pos, true, pitch_state, offense_instructions);
        let density =
            1.0 +
            assignment.player_instructions().in_possession().involvement_priority().value() * 0.3;

        let presence = PlayerZonePresence::new(
            pid,
            offense_team_id,
            pos,
            role,
            zone,
            channel,
            density,
            true
        );
        occupancy.add_presence(&presence);
        presences.insert(pid, presence);
    }

    for assignment in defense_lineup.assignments() {
        let pid = assignment.player().id();
        let pos = assignment.slot().defensive_position();
        let role = assignment.slot_role();
        let channel = derive_position_channel(pos, assignment.formation_slot_index());
        let zone = derive_position_zone(pos, false, pitch_state, defense_instructions);
        let density =
            1.0 +
            assignment.player_instructions().out_of_possession().engagement_bias().value() * 0.3;

        let presence = PlayerZonePresence::new(
            pid,
            defense_team_id,
            pos,
            role,
            zone,
            channel,
            density,
            false
        );
        occupancy.add_presence(&presence);
        presences.insert(pid, presence);
    }

    RoleZoneMap {
        presences,
        occupancy,
    }
}
