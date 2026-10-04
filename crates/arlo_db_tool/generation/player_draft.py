from dataclasses import dataclass, field
from typing import Dict, List, Optional
import uuid


@dataclass
class PlayerPositionDraft:
    position: str
    proficiency: int = 10


@dataclass
class PlayerDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    name: str = ""
    height_m: float = 1.85
    birthdate_unix_seconds: int = 0
    nationality_id: str = ""
    team_id: Optional[str] = None
    squad_number: Optional[int] = None
    captaincy_role: Optional[str] = None
    positions: List[PlayerPositionDraft] = field(default_factory=list)
    attributes: Dict[str, int] = field(default_factory=dict)
