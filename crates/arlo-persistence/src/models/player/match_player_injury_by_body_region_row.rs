use arlo_domain::BodyRegion;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPlayerInjuryByBodyRegionRow {
    pub id: String,
    pub match_id: String,
    pub player_id: String,
    pub body_region: String,
    pub injuries_count: i32,
}

impl MatchPlayerInjuryByBodyRegionRow {
    pub fn new(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        body_region: impl Into<String>,
        injuries_count: u32,
    ) -> Self {
        Self {
            id: id.to_string(),
            match_id: match_id.to_string(),
            player_id: player_id.to_string(),
            body_region: body_region.into(),
            injuries_count: injuries_count as i32,
        }
    }

    pub fn from_stats(
        id: Uuid,
        match_id: Uuid,
        player_id: Uuid,
        region: BodyRegion,
        count: u32,
    ) -> Self {
        let region_str = match region {
            BodyRegion::Head => "Head",
            BodyRegion::Neck => "Neck",
            BodyRegion::Shoulder => "Shoulder",
            BodyRegion::Arm => "Arm",
            BodyRegion::Hand => "Hand",
            BodyRegion::Trunk => "Trunk",
            BodyRegion::Hip => "Hip",
            BodyRegion::Groin => "Groin",
            BodyRegion::Thigh => "Thigh",
            BodyRegion::Knee => "Knee",
            BodyRegion::Calf => "Calf",
            BodyRegion::Ankle => "Ankle",
            BodyRegion::Foot => "Foot",
        };
        Self::new(id, match_id, player_id, region_str, count)
    }
}
