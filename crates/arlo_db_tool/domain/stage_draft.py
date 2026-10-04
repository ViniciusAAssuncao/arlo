from dataclasses import dataclass, field
from typing import List, Optional
import uuid
from arlo_db_tool.domain.entry_rule_pool_draft import EntryRulePoolDraft
from arlo_db_tool.domain.schedule_block_draft import ScheduleBlockDraft

@dataclass
class StageDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    stage_order_index: int = 0
    stage_type: str = "RoundRobinTable"
    leg_format: Optional[str] = None
    entry_rule_pools: List[EntryRulePoolDraft] = field(default_factory=list)
    schedule_blocks: List[ScheduleBlockDraft] = field(default_factory=list)

    def renumerar(self) -> None:
        for idx, pool in enumerate(self.entry_rule_pools):
            pool.pool_order_index = idx
        for idx, block in enumerate(self.schedule_blocks):
            block.block_order_index = idx

    def renumber(self) -> None:
        self.renumerar()
