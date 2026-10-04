import random
from typing import Dict, NamedTuple, Optional


class HeightDistribution(NamedTuple):
    mean: float
    std_dev: float
    min_height: float
    max_height: float


POSITION_LINE_HEIGHT_DISTRIBUTIONS: Dict[str, HeightDistribution] = {
    "Goalguard": HeightDistribution(mean=1.92, std_dev=0.04, min_height=1.82, max_height=2.06),
    "DefenseLine": HeightDistribution(mean=1.88, std_dev=0.05, min_height=1.76, max_height=2.02),
    "BackLine": HeightDistribution(mean=1.84, std_dev=0.05, min_height=1.72, max_height=1.98),
    "OffensiveLine": HeightDistribution(mean=1.82, std_dev=0.05, min_height=1.70, max_height=1.95),
    "General": HeightDistribution(mean=1.83, std_dev=0.06, min_height=1.68, max_height=2.00),
}

POSITION_CODE_TO_LINE: Dict[str, str] = {
    "C-O": "OffensiveLine",
    "W-O": "OffensiveLine",
    "MC": "OffensiveLine",
    "TW": "OffensiveLine",
    "CW": "OffensiveLine",
    "C": "OffensiveLine",
    "A": "BackLine",
    "P": "BackLine",
    "P-R": "BackLine",
    "W-E": "BackLine",
    "R-E": "BackLine",
    "L": "BackLine",
    "F": "BackLine",
    "CB": "DefenseLine",
    "DE": "DefenseLine",
    "RB": "DefenseLine",
    "D-B": "DefenseLine",
    "W-B": "DefenseLine",
    "OZB": "DefenseLine",
    "MZB": "DefenseLine",
    "G": "Goalguard",
}


def position_to_position_line(position_code: str) -> str:
    norm = position_code.strip().upper().replace(" ", "").replace("_", "")
    if norm in POSITION_CODE_TO_LINE:
        return POSITION_CODE_TO_LINE[norm]
    for code, line in POSITION_CODE_TO_LINE.items():
        if code.replace("-", "") == norm or code == norm:
            return line
    return "General"


def sample_height(
    position_line: Optional[str] = None,
    position_code: Optional[str] = None,
    rng: Optional[random.Random] = None,
) -> float:
    r = rng if rng is not None else random.Random()

    line_key = "General"
    if position_line and position_line in POSITION_LINE_HEIGHT_DISTRIBUTIONS:
        line_key = position_line
    elif position_code:
        line_key = position_to_position_line(position_code)

    dist = POSITION_LINE_HEIGHT_DISTRIBUTIONS.get(
        line_key, POSITION_LINE_HEIGHT_DISTRIBUTIONS["General"]
    )
    raw = r.gauss(dist.mean, dist.std_dev)
    clamped = max(dist.min_height, min(dist.max_height, raw))
    return round(clamped, 2)
