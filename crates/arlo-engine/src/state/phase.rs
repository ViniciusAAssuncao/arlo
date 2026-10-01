#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MatchPhase {
    Ready,
    Live,
    Stopped,
    BonusPhase,
    KickFoul,
    PeriodBreak,
    Finished,
}
