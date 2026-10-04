from dataclasses import dataclass
from typing import Optional


@dataclass(frozen=True)
class AttendanceReferenceTeam:
    id: str
    name: str
    prestige: int
    capacity: Optional[int]
    venue_name: Optional[str]
    min_attendance: Optional[int]
    max_attendance: Optional[int]


@dataclass(frozen=True)
class AttendanceReferenceSuggestion:
    team_id: str
    team_name: str
    prestige: int
    capacity: int
    current_min: Optional[int]
    current_max: Optional[int]
    suggested_min: int
    suggested_max: int
    used_calibration: bool
