from typing import List, Optional, Sequence, Tuple
from arlo_db_tool.domain.attendance_reference_draft import (
    AttendanceReferenceSuggestion,
    AttendanceReferenceTeam,
)

I32_MAX = 2_147_483_647


def _clamp(value: float, minimum: float, maximum: float) -> float:
    return max(minimum, min(maximum, value))


def _round_people(value: float) -> int:
    bounded = _clamp(value, 0.0, float(I32_MAX))
    step = 100 if bounded >= 1000.0 else 10
    return int(round(bounded / step) * step)


def _fit_ratio(
    teams: Sequence[AttendanceReferenceTeam],
    field_name: str,
) -> Optional[Tuple[float, float]]:
    points = []
    for team in teams:
        if team.capacity is None or team.capacity <= 0:
            continue
        reference = getattr(team, field_name)
        if reference is None:
            continue
        ratio = reference / team.capacity
        points.append((team.prestige / 1000.0, ratio))

    if len(points) < 3:
        return None

    mean_x = sum(x for x, _ in points) / len(points)
    mean_y = sum(y for _, y in points) / len(points)
    denominator = sum((x - mean_x) ** 2 for x, _ in points)
    if denominator <= 1e-9:
        return None

    slope = sum((x - mean_x) * (y - mean_y) for x, y in points) / denominator
    intercept = mean_y - slope * mean_x
    return intercept, slope


def calibration_sample_count(teams: Sequence[AttendanceReferenceTeam]) -> int:
    return sum(
        1
        for team in teams
        if team.capacity is not None
        and team.capacity > 0
        and team.min_attendance is not None
        and team.max_attendance is not None
        and team.min_attendance <= team.max_attendance
    )


def suggest_attendance_references(
    teams: Sequence[AttendanceReferenceTeam],
    demand_factor: float = 1.0,
    use_calibration: bool = True,
) -> List[AttendanceReferenceSuggestion]:
    eligible = [team for team in teams if team.capacity is not None and team.capacity > 0]
    if not eligible:
        return []

    prestige_values = [team.prestige for team in eligible]
    prestige_min = min(prestige_values)
    prestige_max = max(prestige_values)
    min_fit = _fit_ratio(teams, "min_attendance") if use_calibration else None
    max_fit = _fit_ratio(teams, "max_attendance") if use_calibration else None
    calibrated = min_fit is not None and max_fit is not None
    factor = _clamp(float(demand_factor), 0.5, 1.5)
    suggestions: List[AttendanceReferenceSuggestion] = []

    for team in eligible:
        absolute = _clamp(team.prestige / 1000.0, 0.0, 1.0)
        if prestige_max == prestige_min:
            relative = 0.5
        else:
            relative = (team.prestige - prestige_min) / (prestige_max - prestige_min)
        support = 0.55 * absolute + 0.45 * relative
        min_ratio = 0.30 + 0.32 * support
        max_ratio = 0.68 + 0.50 * support

        if calibrated:
            min_pred = _clamp(min_fit[0] + min_fit[1] * absolute, 0.08, 1.20)
            max_pred = _clamp(max_fit[0] + max_fit[1] * absolute, 0.25, 1.60)
            min_ratio = 0.70 * min_pred + 0.30 * min_ratio
            max_ratio = 0.70 * max_pred + 0.30 * max_ratio

        min_ratio = _clamp(min_ratio * factor, 0.05, 1.25)
        max_ratio = _clamp(max_ratio * factor, 0.15, 1.75)
        if max_ratio < min_ratio + 0.12:
            max_ratio = min(1.75, min_ratio + 0.12)

        minimum = _round_people(team.capacity * min_ratio)
        maximum = _round_people(team.capacity * max_ratio)
        if maximum <= minimum:
            maximum = min(I32_MAX, minimum + (100 if minimum >= 1000 else 10))

        suggestions.append(
            AttendanceReferenceSuggestion(
                team_id=team.id,
                team_name=team.name,
                prestige=team.prestige,
                capacity=team.capacity,
                current_min=team.min_attendance,
                current_max=team.max_attendance,
                suggested_min=minimum,
                suggested_max=maximum,
                used_calibration=calibrated,
            )
        )

    return suggestions
