import random
from typing import Dict, List, Optional, Tuple

POSITION_LINE_GROUPS: Dict[str, List[str]] = {
    "OffensiveLine": ["C-O", "W-O", "MC", "TW", "CW", "C"],
    "BackLine": ["A", "P", "P-R", "W-E", "R-E", "L", "F"],
    "DefenseLine": ["CB", "DE", "RB", "D-B", "W-B", "OZB", "MZB"],
    "Goalguard": ["G"],
}

POSITION_TO_LINE: Dict[str, str] = {
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


def get_positions_for_line(line: str) -> List[str]:
    return list(POSITION_LINE_GROUPS.get(line, []))


def get_line_for_position(position_code: str) -> str:
    norm = position_code.strip().upper().replace(" ", "").replace("_", "")
    if norm in POSITION_TO_LINE:
        return POSITION_TO_LINE[norm]
    for code, line in POSITION_TO_LINE.items():
        if code.replace("-", "") == norm or code == norm:
            return line
    return "OffensiveLine"


def sample_secondary_positions(
    primary_position: str,
    max_count: int = 2,
    secondary_chance: float = 0.40,
    min_proficiency: int = 5,
    max_proficiency: int = 9,
    rng: Optional[random.Random] = None,
) -> List[Tuple[str, int]]:
    r = rng if rng is not None else random.Random()
    if r.random() >= secondary_chance:
        return []

    line = get_line_for_position(primary_position)
    available = [p for p in POSITION_LINE_GROUPS.get(line, []) if p != primary_position]
    if not available:
        return []

    count = r.randint(1, min(max_count, len(available)))
    chosen = r.sample(available, count)
    results = []
    for pos in chosen:
        prof = r.randint(
            min(min_proficiency, max_proficiency),
            max(min_proficiency, max_proficiency),
        )
        results.append((pos, prof))
    return results
