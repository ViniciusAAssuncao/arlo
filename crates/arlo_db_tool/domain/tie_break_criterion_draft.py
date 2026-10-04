from dataclasses import dataclass, field
import uuid

@dataclass
class TieBreakCriterionDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    order_index: int = 0
    criterion_kind: str = "IspaTotal"
