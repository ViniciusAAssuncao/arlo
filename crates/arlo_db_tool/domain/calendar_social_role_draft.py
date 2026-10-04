from dataclasses import dataclass
from typing import Optional


@dataclass(frozen=True)
class CalendarSocialDay:
    id: str
    order_index: int
    name: str
    social_role: Optional[str]
