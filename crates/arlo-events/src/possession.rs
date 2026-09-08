use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CountdownReason {
    OutOfBoundsPlayEnd,
    OutOfBoundsAfterTurnover,
    TurnoverOnDowns,
    AfterScore,
    ArbitralStoppage,
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
pub struct CountdownToSizeStarted {
    offense_team_id: Uuid,
    size_x_mirim: f64,
    reason: CountdownReason,
}

impl CountdownToSizeStarted {
    pub fn new(offense_team_id: Uuid, size_x_mirim: f64, reason: CountdownReason) -> Self {
        Self {
            offense_team_id,
            size_x_mirim,
            reason,
        }
    }

    pub fn offense_team_id(&self) -> Uuid {
        self.offense_team_id
    }

    pub fn size_x_mirim(&self) -> f64 {
        self.size_x_mirim
    }

    pub fn reason(&self) -> CountdownReason {
        self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DownAdvanced {
    previous_down: u32,
    new_down: u32,
    mirins_advanced_this_down: f64,
    total_mirins_advanced_in_series: f64,
    first_down_achieved: bool,
    scrimmage_x_mirim: f64,
}

impl DownAdvanced {
    pub fn new(
        previous_down: u32,
        new_down: u32,
        mirins_advanced_this_down: f64,
        total_mirins_advanced_in_series: f64,
        first_down_achieved: bool,
        scrimmage_x_mirim: f64,
    ) -> Self {
        Self {
            previous_down,
            new_down,
            mirins_advanced_this_down,
            total_mirins_advanced_in_series,
            first_down_achieved,
            scrimmage_x_mirim,
        }
    }

    pub fn previous_down(&self) -> u32 {
        self.previous_down
    }

    pub fn new_down(&self) -> u32 {
        self.new_down
    }

    pub fn mirins_advanced_this_down(&self) -> f64 {
        self.mirins_advanced_this_down
    }

    pub fn total_mirins_advanced_in_series(&self) -> f64 {
        self.total_mirins_advanced_in_series
    }

    pub fn first_down_achieved(&self) -> bool {
        self.first_down_achieved
    }

    pub fn scrimmage_x_mirim(&self) -> f64 {
        self.scrimmage_x_mirim
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutOfBounds {
    last_possession_team: Uuid,
    last_player: Option<Uuid>,
    out_point_x: f64,
    out_point_y: f64,
    was_immediate_loss: bool,
}

impl OutOfBounds {
    pub fn new(
        last_possession_team: Uuid,
        last_player: Option<Uuid>,
        out_point_x: f64,
        out_point_y: f64,
        was_immediate_loss: bool,
    ) -> Self {
        Self {
            last_possession_team,
            last_player,
            out_point_x,
            out_point_y,
            was_immediate_loss,
        }
    }

    pub fn last_possession_team(&self) -> Uuid {
        self.last_possession_team
    }

    pub fn last_player(&self) -> Option<Uuid> {
        self.last_player
    }

    pub fn out_point_x(&self) -> f64 {
        self.out_point_x
    }

    pub fn out_point_y(&self) -> f64 {
        self.out_point_y
    }

    pub fn was_immediate_loss(&self) -> bool {
        self.was_immediate_loss
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Turnover {
    previous_offense: Uuid,
    new_offense: Uuid,
    recovering_player: Option<Uuid>,
    lost_by_player_id: Option<Uuid>,
    in_live_play: bool,
    point_x: f64,
    point_y: f64,
}

impl Turnover {
    pub fn new(
        previous_offense: Uuid,
        new_offense: Uuid,
        recovering_player: Option<Uuid>,
        lost_by_player_id: Option<Uuid>,
        in_live_play: bool,
        point_x: f64,
        point_y: f64,
    ) -> Self {
        Self {
            previous_offense,
            new_offense,
            recovering_player,
            lost_by_player_id,
            in_live_play,
            point_x,
            point_y,
        }
    }

    pub fn previous_offense(&self) -> Uuid {
        self.previous_offense
    }

    pub fn previous_offense_team_id(&self) -> Uuid {
        self.previous_offense
    }

    pub fn new_offense(&self) -> Uuid {
        self.new_offense
    }

    pub fn new_offense_team_id(&self) -> Uuid {
        self.new_offense
    }

    pub fn recovering_player(&self) -> Option<Uuid> {
        self.recovering_player
    }

    pub fn recovering_player_id(&self) -> Option<Uuid> {
        self.recovering_player
    }

    pub fn lost_by_player_id(&self) -> Option<Uuid> {
        self.lost_by_player_id
    }

    pub fn in_live_play(&self) -> bool {
        self.in_live_play
    }

    pub fn point_x(&self) -> f64 {
        self.point_x
    }

    pub fn point_y(&self) -> f64 {
        self.point_y
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PossessionEvent {
    CountdownToSizeStarted(CountdownToSizeStarted),
    DownAdvanced(DownAdvanced),
    OutOfBounds(OutOfBounds),
    Turnover(Turnover),
}

impl From<CountdownToSizeStarted> for PossessionEvent {
    fn from(event: CountdownToSizeStarted) -> Self {
        Self::CountdownToSizeStarted(event)
    }
}

impl From<DownAdvanced> for PossessionEvent {
    fn from(event: DownAdvanced) -> Self {
        Self::DownAdvanced(event)
    }
}

impl From<OutOfBounds> for PossessionEvent {
    fn from(event: OutOfBounds) -> Self {
        Self::OutOfBounds(event)
    }
}

impl From<Turnover> for PossessionEvent {
    fn from(event: Turnover) -> Self {
        Self::Turnover(event)
    }
}
