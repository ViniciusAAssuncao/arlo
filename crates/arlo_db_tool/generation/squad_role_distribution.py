from typing import Dict

BASE_SQUAD_POSITION_QUOTAS: Dict[str, int] = {
    "G": 2,
    "C-O": 2,
    "W-O": 2,
    "MC": 1,
    "TW": 1,
    "CW": 1,
    "C": 1,
    "A": 2,
    "P": 2,
    "P-R": 1,
    "W-E": 1,
    "R-E": 1,
    "L": 1,
    "F": 1,
    "CB": 2,
    "DE": 1,
    "RB": 1,
    "D-B": 1,
    "W-B": 1,
    "OZB": 1,
    "MZB": 1,
}

TIER_1_ADDITIONAL_QUOTAS: Dict[str, int] = {
    "CB": 1,
    "W-O": 1,
    "P-R": 1,
}

TIER_2_ADDITIONAL_QUOTAS: Dict[str, int] = {
    "G": 1,
    "A": 1,
    "DE": 1,
}

TIER_3_ADDITIONAL_QUOTAS: Dict[str, int] = {
    "P": 1,
    "C-O": 1,
    "L": 1,
}


def get_base_squad_quotas() -> Dict[str, int]:
    return dict(BASE_SQUAD_POSITION_QUOTAS)


def build_squad_quotas_for_prestige(prestige: int) -> Dict[str, int]:
    quotas = get_base_squad_quotas()
    p = max(0, min(1000, prestige))

    if p >= 350:
        for pos, count in TIER_1_ADDITIONAL_QUOTAS.items():
            quotas[pos] = quotas.get(pos, 0) + count

    if p >= 600:
        for pos, count in TIER_2_ADDITIONAL_QUOTAS.items():
            quotas[pos] = quotas.get(pos, 0) + count

    if p >= 800:
        for pos, count in TIER_3_ADDITIONAL_QUOTAS.items():
            quotas[pos] = quotas.get(pos, 0) + count

    return quotas
