use crate::domain::collective_agreement::annual_blackout_window::AnnualBlackoutWindow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CollectiveAgreementRule {
    AnnualBlackoutWindow(AnnualBlackoutWindow),
}
