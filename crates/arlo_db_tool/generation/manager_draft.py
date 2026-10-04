from dataclasses import dataclass, field
from typing import Dict, Optional
import uuid


@dataclass
class ManagerDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    name: str = ""
    height_m: float = 1.80
    birthdate_unix_seconds: int = 0
    nationality_id: str = ""
    team_id: Optional[str] = None
    control_mode: str = "Ai"
    attributes: Dict[str, int] = field(default_factory=dict)
