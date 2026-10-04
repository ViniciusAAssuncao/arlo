from dataclasses import dataclass, field
from typing import List
import uuid

@dataclass
class CompetitionGroupDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    order_index: int = 0
    name: str = ""
    team_ids: List[str] = field(default_factory=list)
