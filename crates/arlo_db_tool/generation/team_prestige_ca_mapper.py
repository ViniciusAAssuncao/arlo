from typing import NamedTuple, Sequence, Tuple
from arlo_db_tool.ca.constants import MAX_CURRENT_ABILITY, MIN_CURRENT_ABILITY


class PrestigeCaTier(NamedTuple):
    min_prestige: int
    max_prestige: int
    ca_min: int
    ca_max: int


PRESTIGE_CA_TIERS: Sequence[PrestigeCaTier] = (
    PrestigeCaTier(min_prestige=0, max_prestige=150, ca_min=40, ca_max=75),
    PrestigeCaTier(min_prestige=151, max_prestige=300, ca_min=65, ca_max=95),
    PrestigeCaTier(min_prestige=301, max_prestige=500, ca_min=85, ca_max=120),
    PrestigeCaTier(min_prestige=501, max_prestige=700, ca_min=110, ca_max=145),
    PrestigeCaTier(min_prestige=701, max_prestige=850, ca_min=135, ca_max=170),
    PrestigeCaTier(min_prestige=851, max_prestige=1000, ca_min=155, ca_max=195),
)


def map_prestige_to_ca_range(
    prestige: int,
    tiers: Sequence[PrestigeCaTier] = PRESTIGE_CA_TIERS,
) -> Tuple[int, int]:
    clamped_prestige = max(0, min(1000, prestige))
    for tier in tiers:
        if tier.min_prestige <= clamped_prestige <= tier.max_prestige:
            return (
                max(MIN_CURRENT_ABILITY, tier.ca_min),
                min(MAX_CURRENT_ABILITY, tier.ca_max),
            )

    if clamped_prestige < tiers[0].min_prestige:
        return (tiers[0].ca_min, tiers[0].ca_max)
    return (tiers[-1].ca_min, tiers[-1].ca_max)