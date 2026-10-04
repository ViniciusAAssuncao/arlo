from dataclasses import dataclass, field
from typing import Dict, Optional
import uuid


@dataclass
class RefereeDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    name: str = ""
    height_m: float = 1.82
    birthdate_unix_seconds: int = 0
    nationality_id: str = ""
    primary_league_id: Optional[str] = None
    tier: str = "National"
    attributes: Dict[str, int] = field(default_factory=dict)
