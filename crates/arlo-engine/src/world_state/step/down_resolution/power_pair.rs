use crate::team_strength::{LeagueStrengthScale, TeamMatchPower, TeamStrengthProfile};
use crate::home_advantage::HomeAdvantageProfile;
use crate::resolution::DuelKind;
use crate::team_strength::{calculate_strength_z_gap, map_z_gap_to_rating};
use arlo_domain::Position;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DuelPowerPair {
    pub team_attacker_rating: f64,
    pub team_defender_rating: f64,
    pub attacker_individual_weight: f64,
    pub defender_individual_weight: f64,
}

pub fn derive_power_pair(
    offense_power: TeamMatchPower,
    defense_power: TeamMatchPower,
    duel_kind: DuelKind,
    league_scale: &LeagueStrengthScale,
    strength_profile: &TeamStrengthProfile,
    home_advantage_profile: &HomeAdvantageProfile,
    attacker_pos: Option<Position>,
    defender_pos: Option<Position>,
    attacker_is_home: bool,
    defender_is_home: bool,
) -> DuelPowerPair {
    let (att_power, att_mean, att_stddev) = match duel_kind {
        DuelKind::ShortDistribution | DuelKind::LongDistribution | DuelKind::CrossDistribution | DuelKind::PassProtection | DuelKind::KickBlockAttempt => {
            (offense_power.control_power(), league_scale.control_mean, league_scale.control_stddev)
        },
        _ => {
            (offense_power.offensive_power(), league_scale.offense_mean, league_scale.offense_stddev)
        }
    };
    
    let def_power = defense_power.defensive_power();
    let def_mean = league_scale.defense_mean;
    let def_stddev = league_scale.defense_stddev;

    let mut z_gap = calculate_strength_z_gap(
        att_power, def_power,
        att_mean, att_stddev,
        def_mean, def_stddev,
    );

    if attacker_is_home {
        z_gap += home_advantage_profile.power_z_boost();
    }
    if defender_is_home {
        z_gap -= home_advantage_profile.power_z_boost();
    }

    let (team_att, team_def) = map_z_gap_to_rating(z_gap, strength_profile.z_gap_gain);

    let get_weight = |pos: Option<Position>| {
        match pos {
            Some(Position::Artrine) => strength_profile.artrine_individual_weight,
            Some(Position::Passer) => strength_profile.passer_individual_weight,
            Some(Position::Goalguard) => strength_profile.goalguard_individual_weight,
            _ => strength_profile.default_individual_weight,
        }
    };

    DuelPowerPair {
        team_attacker_rating: team_att,
        team_defender_rating: team_def,
        attacker_individual_weight: get_weight(attacker_pos),
        defender_individual_weight: get_weight(defender_pos),
    }
}
