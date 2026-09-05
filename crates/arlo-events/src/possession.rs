use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CountdownReason {
    OutOfBoundsAfterTurnover,
    OutOfBoundsPlayEnd,
    TurnoverOnDowns,
    RefereeStoppage,
    AfterScore,
    PeriodStart,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Turnover {
    previous_offense_team_id: Uuid,
    new_offense_team_id: Uuid,
    recovering_player_id: Option<Uuid>,
    in_live_play: bool,
    x_mirim: f64,
    y_mirim: f64,
}

impl Turnover {
    pub fn new(
        previous_offense_team_id: Uuid,
        new_offense_team_id: Uuid,
        recovering_player_id: Option<Uuid>,
        in_live_play: bool,
        x_mirim: f64,
        y_mirim: f64,
    ) -> Self {
        Self {
            previous_offense_team_id,
            new_offense_team_id,
            recovering_player_id,
            in_live_play,
            x_mirim,
            y_mirim,
        }
    }

    pub fn previous_offense_team_id(&self) -> Uuid {
        self.previous_offense_team_id
    }

    pub fn new_offense_team_id(&self) -> Uuid {
        self.new_offense_team_id
    }

    pub fn recovering_player_id(&self) -> Option<Uuid> {
        self.recovering_player_id
    }

    pub fn in_live_play(&self) -> bool {
        self.in_live_play
    }

    pub fn x_mirim(&self) -> f64 {
        self.x_mirim
    }

    pub fn y_mirim(&self) -> f64 {
        self.y_mirim
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OutOfBounds {
    last_possession_team_id: Uuid,
    last_player_id: Option<Uuid>,
    out_x_mirim: f64,
    out_y_mirim: f64,
    was_immediate_loss: bool,
}

impl OutOfBounds {
    pub fn new(
        last_possession_team_id: Uuid,
        last_player_id: Option<Uuid>,
        out_x_mirim: f64,
        out_y_mirim: f64,
        was_immediate_loss: bool,
    ) -> Self {
        Self {
            last_possession_team_id,
            last_player_id,
            out_x_mirim,
            out_y_mirim,
            was_immediate_loss,
        }
    }

    pub fn last_possession_team_id(&self) -> Uuid {
        self.last_possession_team_id
    }

    pub fn last_player_id(&self) -> Option<Uuid> {
        self.last_player_id
    }

    pub fn out_x_mirim(&self) -> f64 {
        self.out_x_mirim
    }

    pub fn out_y_mirim(&self) -> f64 {
        self.out_y_mirim
    }

    pub fn was_immediate_loss(&self) -> bool {
        self.was_immediate_loss
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
pub enum PossessionEvent {
    Turnover(Turnover),
    OutOfBounds(OutOfBounds),
    CountdownToSizeStarted(CountdownToSizeStarted),
    DownAdvanced(DownAdvanced),
}

impl From<Turnover> for PossessionEvent {
    fn from(ev: Turnover) -> Self {
        Self::Turnover(ev)
    }
}

impl From<OutOfBounds> for PossessionEvent {
    fn from(ev: OutOfBounds) -> Self {
        Self::OutOfBounds(ev)
    }
}

impl From<CountdownToSizeStarted> for PossessionEvent {
    fn from(ev: CountdownToSizeStarted) -> Self {
        Self::CountdownToSizeStarted(ev)
    }
}

impl From<DownAdvanced> for PossessionEvent {
    fn from(ev: DownAdvanced) -> Self {
        Self::DownAdvanced(ev)
    }
}