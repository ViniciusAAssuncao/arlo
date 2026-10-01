use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RoundRobinMatch {
    pub round_index: u32,
    pub home_team_id: Uuid,
    pub away_team_id: Uuid,
    pub is_neutral_venue: bool,
}

impl RoundRobinMatch {
    pub fn new(round_index: u32, home_team_id: Uuid, away_team_id: Uuid) -> Self {
        Self {
            round_index,
            home_team_id,
            away_team_id,
            is_neutral_venue: false,
        }
    }

    pub fn with_neutral(
        round_index: u32,
        home_team_id: Uuid,
        away_team_id: Uuid,
        is_neutral_venue: bool,
    ) -> Self {
        Self {
            round_index,
            home_team_id,
            away_team_id,
            is_neutral_venue,
        }
    }
}

pub fn generate_single_round_robin(team_ids: &[Uuid]) -> Vec<RoundRobinMatch> {
    if team_ids.len() < 2 {
        return Vec::new();
    }

    let is_odd = team_ids.len() % 2 != 0;
    let mut circle: Vec<Option<Uuid>> = team_ids.iter().map(|&id| Some(id)).collect();
    if is_odd {
        circle.push(None);
    }

    let total_teams = circle.len();
    let total_rounds = total_teams - 1;
    let matches_per_round = total_teams / 2;
    let mut fixtures = Vec::with_capacity(total_rounds * matches_per_round);

    for round in 0..total_rounds {
        for k in 0..matches_per_round {
            let team_a = circle[k];
            let team_b = circle[total_teams - 1 - k];

            let (Some(t_a), Some(t_b)) = (team_a, team_b) else {
                continue;
            };

            let (home, away) = if k == 0 {
                if round % 2 == 0 {
                    (t_a, t_b)
                } else {
                    (t_b, t_a)
                }
            } else if k % 2 == 1 {
                (t_a, t_b)
            } else {
                (t_b, t_a)
            };

            fixtures.push(RoundRobinMatch::with_neutral(
                round as u32,
                home,
                away,
                false,
            ));
        }

        if total_teams > 2 {
            let last = circle.pop().unwrap();
            circle.insert(1, last);
        }
    }

    fixtures
}
