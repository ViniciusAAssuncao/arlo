from typing import Dict, List, Tuple
from arlo_db_tool.schema.entities.position_codes import SHORT_POSITION_CODES

POSITION_CODES = SHORT_POSITION_CODES

POSITION_DISPLAY_NAMES = {
    "C-O": "Center Offense (C-O)",
    "W-O": "Wing Offense (W-O)",
    "MC": "Midcenter (MC)",
    "TW": "Tight Wing (TW)",
    "CW": "Center Tight (CW)",
    "C": "Corridor (C)",
    "A": "Artrine (A)",
    "P": "Passer (P)",
    "P-R": "Pass Rusher (P-R)",
    "W-E": "Wide End (W-E)",
    "R-E": "Running End (R-E)",
    "L": "Lineback (L)",
    "F": "Fullback (F)",
    "CB": "Centerback (CB)",
    "DE": "Defensive End (DE)",
    "RB": "Rougieback (RB)",
    "D-B": "Defensive Blocker (D-B)",
    "W-B": "Wide Blocker (W-B)",
    "OZB": "Outside Zonerback (OZB)",
    "MZB": "Middle Zonerback (MZB)",
    "G": "Goalguard (G)",
}

POSITION_CODE_ALIASES = {
    "C-O": "C-O",
    "CO": "C-O",
    "CENTEROFFENSE": "C-O",
    "CENTER_OFFENSE": "C-O",
    "W-O": "W-O",
    "WO": "W-O",
    "WINGOFFENSE": "W-O",
    "WING_OFFENSE": "W-O",
    "MC": "MC",
    "MIDCENTER": "MC",
    "TW": "TW",
    "TIGHTWING": "TW",
    "TIGHT_WING": "TW",
    "CW": "CW",
    "CT": "CW",
    "CENTERTIGHT": "CW",
    "CENTER_TIGHT": "CW",
    "C": "C",
    "COR": "C",
    "CORRIDOR": "C",
    "A": "A",
    "ART": "A",
    "ARTRINE": "A",
    "P": "P",
    "PAS": "P",
    "PASSER": "P",
    "P-R": "P-R",
    "PR": "P-R",
    "PASSRUSHER": "P-R",
    "PASS_RUSHER": "P-R",
    "W-E": "W-E",
    "WE": "W-E",
    "WIDEEND": "W-E",
    "WIDE_END": "W-E",
    "R-E": "R-E",
    "RE": "R-E",
    "RUNNINGEND": "R-E",
    "RUNNING_END": "R-E",
    "L": "L",
    "LB": "L",
    "LINEBACK": "L",
    "F": "F",
    "FB": "F",
    "FULLBACK": "F",
    "CB": "CB",
    "CENTERBACK": "CB",
    "DE": "DE",
    "DEFENSIVEEND": "DE",
    "DEFENSIVE_END": "DE",
    "RB": "RB",
    "RGB": "RB",
    "ROUGIEBACK": "RB",
    "D-B": "D-B",
    "DB": "D-B",
    "DEFENSIVEBLOCKER": "D-B",
    "DEFENSIVE_BLOCKER": "D-B",
    "W-B": "W-B",
    "WB": "W-B",
    "WIDEBLOCKER": "W-B",
    "WIDE_BLOCKER": "W-B",
    "OZB": "OZB",
    "OUTSIDEZONERBACK": "OZB",
    "OUTSIDE_ZONERBACK": "OZB",
    "MZB": "MZB",
    "MIDDLEZONERBACK": "MZB",
    "MIDDLE_ZONERBACK": "MZB",
    "G": "G",
    "GG": "G",
    "GOALGUARD": "G",
}

def center_offense_profile() -> List[Tuple[str, float]]:
    return [
        ("finishing", 4.5),
        ("composure", 4.0),
        ("anticipation", 3.5),
        ("strength", 3.0),
        ("balance", 3.0),
        ("acceleration", 3.0),
        ("decisions", 2.5),
        ("positioning", 2.5),
        ("technique", 2.5),
        ("pace", 2.5),
        ("stamina", 2.0),
        ("work_rate", 2.0),
        ("determination", 2.0),
    ]

def wing_offense_profile() -> List[Tuple[str, float]]:
    return [
        ("pace", 4.5),
        ("acceleration", 4.5),
        ("dribbling", 4.0),
        ("crossing", 4.0),
        ("agility", 4.0),
        ("technique", 3.5),
        ("flair", 3.0),
        ("finishing", 3.0),
        ("stamina", 3.0),
        ("composure", 2.5),
        ("passing", 2.5),
        ("vision", 2.5),
        ("anticipation", 2.5),
        ("balance", 2.5),
        ("work_rate", 2.0),
        ("decisions", 2.0),
    ]

def midcenter_profile() -> List[Tuple[str, float]]:
    return [
        ("passing", 4.5),
        ("vision", 4.5),
        ("technique", 4.0),
        ("decisions", 4.0),
        ("composure", 3.5),
        ("teamwork", 3.5),
        ("anticipation", 3.5),
        ("stamina", 3.0),
        ("positioning", 3.0),
        ("work_rate", 3.0),
        ("balance", 2.5),
        ("agility", 2.5),
    ]

def tight_wing_profile() -> List[Tuple[str, float]]:
    return [
        ("dribbling", 4.5),
        ("agility", 4.0),
        ("acceleration", 4.0),
        ("crossing", 3.5),
        ("technique", 3.5),
        ("pace", 3.5),
        ("balance", 3.0),
        ("passing", 3.0),
        ("stamina", 3.0),
        ("work_rate", 3.0),
        ("anticipation", 2.5),
        ("composure", 2.5),
        ("decisions", 2.5),
        ("finishing", 2.0),
    ]

def center_tight_profile() -> List[Tuple[str, float]]:
    return [
        ("passing", 4.0),
        ("strength", 4.0),
        ("balance", 3.5),
        ("decisions", 3.5),
        ("composure", 3.5),
        ("technique", 3.5),
        ("positioning", 3.0),
        ("teamwork", 3.0),
        ("work_rate", 3.0),
        ("stamina", 3.0),
        ("anticipation", 2.5),
        ("vision", 2.0),
    ]

def corridor_profile() -> List[Tuple[str, float]]:
    return [
        ("stamina", 4.5),
        ("work_rate", 4.5),
        ("pace", 4.0),
        ("acceleration", 4.0),
        ("teamwork", 3.5),
        ("crossing", 3.5),
        ("passing", 3.5),
        ("positioning", 3.0),
        ("anticipation", 3.0),
        ("balance", 2.5),
        ("agility", 2.5),
        ("decisions", 2.5),
        ("determination", 2.0),
    ]

def artrine_profile() -> List[Tuple[str, float]]:
    return [
        ("passing", 4.5),
        ("vision", 4.5),
        ("technique", 4.0),
        ("decisions", 4.0),
        ("composure", 3.5),
        ("anticipation", 3.5),
        ("teamwork", 3.0),
        ("balance", 2.5),
        ("stamina", 2.5),
        ("flair", 2.0),
        ("work_rate", 2.0),
        ("concentration", 2.0),
    ]

def passer_profile() -> List[Tuple[str, float]]:
    return [
        ("passing", 5.0),
        ("vision", 4.5),
        ("decisions", 4.5),
        ("composure", 4.0),
        ("technique", 4.0),
        ("anticipation", 3.5),
        ("concentration", 3.0),
        ("teamwork", 3.0),
        ("strength", 2.5),
        ("balance", 2.5),
        ("stamina", 2.0),
        ("leadership", 2.0),
        ("positioning", 2.0),
        ("determination", 2.0),
    ]

def pass_rusher_profile() -> List[Tuple[str, float]]:
    return [
        ("acceleration", 4.5),
        ("pace", 4.5),
        ("strength", 4.0),
        ("work_rate", 3.5),
        ("bravery", 3.5),
        ("agility", 3.5),
        ("anticipation", 3.0),
        ("stamina", 3.0),
        ("determination", 3.0),
        ("balance", 2.5),
        ("decisions", 2.0),
        ("positioning", 2.0),
        ("concentration", 1.5),
        ("composure", 1.5),
    ]

def wide_end_profile() -> List[Tuple[str, float]]:
    return [
        ("pace", 4.5),
        ("acceleration", 4.5),
        ("agility", 4.0),
        ("balance", 3.5),
        ("anticipation", 3.0),
        ("stamina", 3.0),
        ("flair", 2.5),
        ("positioning", 2.5),
        ("composure", 2.5),
        ("technique", 2.5),
        ("crossing", 2.0),
        ("work_rate", 2.0),
        ("bravery", 2.0),
        ("determination", 1.5),
    ]

def running_end_profile() -> List[Tuple[str, float]]:
    return [
        ("acceleration", 4.5),
        ("pace", 4.5),
        ("balance", 4.0),
        ("agility", 4.0),
        ("strength", 3.5),
        ("stamina", 3.5),
        ("bravery", 3.0),
        ("work_rate", 3.0),
        ("anticipation", 3.0),
        ("composure", 2.5),
        ("decisions", 2.5),
        ("determination", 2.5),
        ("dribbling", 2.0),
        ("positioning", 1.5),
        ("flair", 1.5),
    ]

def lineback_profile() -> List[Tuple[str, float]]:
    return [
        ("positioning", 4.0),
        ("strength", 4.0),
        ("anticipation", 4.0),
        ("decisions", 3.5),
        ("bravery", 3.5),
        ("stamina", 3.5),
        ("work_rate", 3.5),
        ("concentration", 3.0),
        ("balance", 2.5),
        ("acceleration", 2.5),
        ("teamwork", 2.0),
        ("determination", 2.0),
        ("composure", 1.5),
    ]

def fullback_profile() -> List[Tuple[str, float]]:
    return [
        ("stamina", 4.5),
        ("pace", 4.0),
        ("acceleration", 4.0),
        ("positioning", 3.5),
        ("work_rate", 3.5),
        ("crossing", 3.0),
        ("passing", 3.0),
        ("teamwork", 3.0),
        ("anticipation", 3.0),
        ("balance", 2.5),
        ("decisions", 2.5),
        ("agility", 2.0),
        ("concentration", 2.0),
        ("bravery", 2.0),
        ("determination", 1.5),
    ]

def centerback_profile() -> List[Tuple[str, float]]:
    return [
        ("positioning", 4.5),
        ("strength", 4.0),
        ("anticipation", 3.5),
        ("bravery", 3.5),
        ("composure", 3.0),
        ("decisions", 3.0),
        ("concentration", 3.0),
        ("balance", 2.5),
        ("pace", 2.0),
        ("acceleration", 2.0),
        ("stamina", 2.0),
        ("teamwork", 2.0),
    ]

def defensive_end_profile() -> List[Tuple[str, float]]:
    return [
        ("strength", 4.5),
        ("acceleration", 4.0),
        ("pace", 4.0),
        ("bravery", 3.5),
        ("work_rate", 3.5),
        ("stamina", 3.5),
        ("anticipation", 3.0),
        ("agility", 3.0),
        ("positioning", 2.5),
        ("balance", 2.5),
        ("decisions", 2.0),
        ("determination", 2.0),
        ("concentration", 2.0),
    ]

def rougieback_profile() -> List[Tuple[str, float]]:
    return [
        ("positioning", 4.0),
        ("pace", 4.0),
        ("acceleration", 4.0),
        ("agility", 3.5),
        ("anticipation", 3.5),
        ("stamina", 3.5),
        ("bravery", 3.0),
        ("concentration", 3.0),
        ("work_rate", 3.0),
        ("balance", 2.5),
        ("decisions", 2.5),
        ("strength", 2.0),
        ("teamwork", 2.0),
        ("determination", 1.5),
    ]

def defensive_blocker_profile() -> List[Tuple[str, float]]:
    return [
        ("strength", 5.0),
        ("positioning", 4.0),
        ("bravery", 4.0),
        ("balance", 3.5),
        ("concentration", 3.0),
        ("composure", 3.0),
        ("decisions", 3.0),
        ("anticipation", 2.5),
        ("stamina", 2.0),
        ("work_rate", 2.0),
        ("teamwork", 1.5),
        ("determination", 1.5),
    ]

def wide_blocker_profile() -> List[Tuple[str, float]]:
    return [
        ("pace", 4.5),
        ("acceleration", 4.0),
        ("agility", 3.5),
        ("stamina", 3.5),
        ("positioning", 3.5),
        ("work_rate", 3.0),
        ("balance", 3.0),
        ("anticipation", 3.0),
        ("bravery", 2.5),
        ("strength", 2.5),
        ("decisions", 2.0),
        ("concentration", 2.0),
        ("teamwork", 1.5),
        ("determination", 1.5),
    ]

def outside_zonerback_profile() -> List[Tuple[str, float]]:
    return [
        ("positioning", 4.5),
        ("pace", 4.0),
        ("acceleration", 4.0),
        ("anticipation", 4.0),
        ("agility", 3.0),
        ("stamina", 3.0),
        ("decisions", 3.0),
        ("concentration", 2.5),
        ("composure", 2.5),
        ("balance", 2.0),
        ("work_rate", 2.0),
        ("teamwork", 2.0),
        ("determination", 1.5),
    ]

def middle_zonerback_profile() -> List[Tuple[str, float]]:
    return [
        ("positioning", 4.5),
        ("anticipation", 4.5),
        ("decisions", 3.5),
        ("composure", 3.5),
        ("teamwork", 3.0),
        ("strength", 3.0),
        ("stamina", 3.0),
        ("concentration", 3.0),
        ("balance", 2.0),
        ("bravery", 2.0),
        ("acceleration", 2.0),
        ("pace", 2.0),
        ("leadership", 1.5),
    ]

def goalguard_profile() -> List[Tuple[str, float]]:
    return [
        ("reflexes", 5.0),
        ("handling", 4.5),
        ("positioning", 4.5),
        ("rushing_out", 3.5),
        ("communication", 3.5),
        ("concentration", 3.5),
        ("agility", 3.5),
        ("composure", 3.0),
        ("anticipation", 3.0),
        ("bravery", 3.0),
        ("distribution", 2.5),
        ("decisions", 2.0),
    ]

def manager_profile() -> List[Tuple[str, float]]:
    return [
        ("tactical_knowledge", 4.5),
        ("offense_planning", 4.0),
        ("defense_organization", 4.0),
        ("artro_strategy", 3.5),
        ("adaptability", 3.5),
        ("in_game_adjustments", 3.5),
        ("artrine_communication", 3.0),
        ("time_call_management", 3.0),
        ("challenge_judgment", 3.0),
        ("man_management", 3.0),
        ("discipline", 2.5),
        ("load_management", 2.5),
        ("player_development", 2.0),
        ("judging_ability", 2.0),
        ("judging_potential", 2.0),
    ]

POSITION_PROFILES: Dict[str, List[Tuple[str, float]]] = {
    "C-O": center_offense_profile(),
    "W-O": wing_offense_profile(),
    "MC": midcenter_profile(),
    "TW": tight_wing_profile(),
    "CW": center_tight_profile(),
    "C": corridor_profile(),
    "A": artrine_profile(),
    "P": passer_profile(),
    "P-R": pass_rusher_profile(),
    "W-E": wide_end_profile(),
    "R-E": running_end_profile(),
    "L": lineback_profile(),
    "F": fullback_profile(),
    "CB": centerback_profile(),
    "DE": defensive_end_profile(),
    "RB": rougieback_profile(),
    "D-B": defensive_blocker_profile(),
    "W-B": wide_blocker_profile(),
    "OZB": outside_zonerback_profile(),
    "MZB": middle_zonerback_profile(),
    "G": goalguard_profile(),
}

def get_position_profile(pos: str) -> List[Tuple[str, float]]:
    norm = pos.strip().upper().replace(" ", "").replace("_", "").replace("-", "")
    code = POSITION_CODE_ALIASES.get(norm, POSITION_CODE_ALIASES.get(pos.strip().upper(), "P"))
    return POSITION_PROFILES.get(code, passer_profile())

def get_manager_profile() -> List[Tuple[str, float]]:
    return manager_profile()
