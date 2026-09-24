use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IntercalationPlacement {
    BeforeFirstMonth,
    AfterLastMonth,
    AppendToMonth { month_order_index: u32 },
}
