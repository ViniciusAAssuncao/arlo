use arlo_stats::PlayerReceivingStats;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchPlayerReceivingRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub targets: i32,
    pub receptions: i32,
    pub drops: i32,
    pub catch_rate: f64,
    pub drop_rate: f64,
    pub receiving_mirins: f64,
    pub run_after_catch_mirins: f64,
    pub longest_reception_mirim: f64,
    pub average_mirins_per_reception: f64,
    pub average_rac_per_reception: f64,
}

impl MatchPlayerReceivingRow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        targets: u32,
        receptions: u32,
        drops: u32,
        catch_rate: f64,
        drop_rate: f64,
        receiving_mirins: f64,
        run_after_catch_mirins: f64,
        longest_reception_mirim: f64,
        average_mirins_per_reception: f64,
        average_rac_per_reception: f64,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            targets: targets as i32,
            receptions: receptions as i32,
            drops: drops as i32,
            catch_rate,
            drop_rate,
            receiving_mirins,
            run_after_catch_mirins,
            longest_reception_mirim,
            average_mirins_per_reception,
            average_rac_per_reception,
        }
    }

    pub fn from_stats(id: Uuid, match_id: Uuid, stats: &PlayerReceivingStats) -> Self {
        Self::new(
            id,
            match_id,
            stats.player_id,
            stats.targets,
            stats.receptions,
            stats.drops,
            stats.catch_rate(),
            stats.drop_rate(),
            stats.receiving_mirins,
            stats.run_after_catch_mirins,
            stats.longest_reception_mirim,
            stats.average_mirins_per_reception(),
            stats.average_rac_per_reception(),
        )
    }
}
