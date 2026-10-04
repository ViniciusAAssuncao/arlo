from dataclasses import dataclass, field
from typing import List, Optional
import uuid

@dataclass
class ScheduleBlockDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    block_order_index: int = 0
    block_kind: str = "GroupRoundRobin"
    group_a_id: Optional[str] = None
    group_b_id: Optional[str] = None
    mirrored: Optional[bool] = None
    pool_kind: Optional[str] = None
    rounds_count: Optional[int] = None
    pool_group_ids: List[str] = field(default_factory=list)
