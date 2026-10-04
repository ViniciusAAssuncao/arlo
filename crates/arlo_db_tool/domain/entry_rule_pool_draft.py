from dataclasses import dataclass, field
from typing import Optional
import uuid

@dataclass
class EntryRulePoolDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    pool_order_index: int = 0
    pool_kind: str = "AllTeams"
    count: Optional[int] = None
    position_index: Optional[int] = None
    range_start_position: Optional[int] = None
    range_end_position: Optional[int] = None
    external_competition_id: Optional[str] = None
