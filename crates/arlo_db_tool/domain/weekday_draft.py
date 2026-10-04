from dataclasses import dataclass, field
import uuid

@dataclass
class WeekdayDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    weekday_order_index: int = 0
