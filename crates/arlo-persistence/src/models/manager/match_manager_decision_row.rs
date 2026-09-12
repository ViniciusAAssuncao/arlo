use arlo_stats::ManagerDecisionLog;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchManagerDecisionRow {
    pub id: String,
    pub match_id: String,
    pub team_id: String,
    pub substitutions_made: i32,
    pub time_calls_used: i32,
    pub challenges_won: i32,
    pub challenges_lost: i32,
    pub tactical_profile_switches: i32,
}

impl MatchManagerDecisionRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        team_id: Uuid,
        substitutions_made: u32,
        time_calls_used: u32,
        challenges_won: u32,
        challenges_lost: u32,
        tactical_profile_switches: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            team_id: team_id.to_string(),
            substitutions_made: substitutions_made as i32,
            time_calls_used: time_calls_used as i32,
            challenges_won: challenges_won as i32,
            challenges_lost: challenges_lost as i32,
            tactical_profile_switches: tactical_profile_switches as i32,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, team_id: Uuid, log: &ManagerDecisionLog) -> Self {
        Self::new(
            id,
            match_id,
            team_id,
            log.substitutions_made,
            log.time_calls_used,
            log.challenges_won,
            log.challenges_lost,
            log.tactical_profile_switches,
        )
    }
}