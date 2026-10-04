from dataclasses import dataclass, field
from typing import Optional
import uuid


@dataclass
class VenueDraft:
    id: str = field(default_factory=lambda: str(uuid.uuid4()))
    name: str = ""
    kind: str = "MatchStadium"
    owner_team_id: Optional[str] = None
    country_id: str = ""
    capacity: Optional[int] = 35000
    pitch_length_mirim: Optional[float] = 145.0
    pitch_width_mirim: Optional[float] = 85.0
