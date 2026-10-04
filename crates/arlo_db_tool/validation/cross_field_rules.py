from typing import Any, Dict, List

def validate_competition(values: Dict[str, Any]) -> List[str]:
    errors: List[str] = []
    scope = values.get("scope")
    country_id = values.get("country_id")
    has_country = country_id is not None and str(country_id).strip() != ""

    if scope in ("Regional", "National") and not has_country:
        errors.append("country_id is required for Regional and National competitions")
    elif scope in ("Continental", "International") and has_country:
        errors.append("country_id must be null for Continental and International competitions")

    return errors

def validate_federation(values: Dict[str, Any]) -> List[str]:
    errors: List[str] = []
    scope = values.get("scope")
    continent_id = values.get("continent_id")
    fed_id = values.get("id")
    parent_fed_id = values.get("parent_federation_id")

    has_continent = continent_id is not None and str(continent_id).strip() != ""

    if scope == "Continental" and not has_continent:
        errors.append("continent_id is required for Continental federations")
    elif scope != "Continental" and has_continent:
        errors.append("continent_id must be null for non-Continental federations")

    if fed_id and parent_fed_id and str(fed_id).strip() == str(parent_fed_id).strip():
        errors.append("parent_federation_id cannot reference the federation itself")

    return errors

def validate_venue(values: Dict[str, Any]) -> List[str]:
    errors: List[str] = []
    kind = values.get("kind")
    length = values.get("pitch_length_mirim")
    width = values.get("pitch_width_mirim")

    has_length = length is not None and str(length).strip() != ""
    has_width = width is not None and str(width).strip() != ""

    if kind == "MatchStadium":
        if not has_length:
            errors.append("pitch_length_mirim is required for MatchStadium")
        else:
            try:
                l_val = float(length)
                if l_val < 140.0 or l_val > 150.0:
                    errors.append("pitch_length_mirim must be between 140.0 and 150.0")
            except (ValueError, TypeError):
                errors.append("pitch_length_mirim must be a valid number")

        if not has_width:
            errors.append("pitch_width_mirim is required for MatchStadium")
        else:
            try:
                w_val = float(width)
                if w_val < 80.0 or w_val > 90.0:
                    errors.append("pitch_width_mirim must be between 80.0 and 90.0")
            except (ValueError, TypeError):
                errors.append("pitch_width_mirim must be a valid number")

    elif kind == "TrainingCenter":
        if has_length:
            errors.append("pitch_length_mirim must be null for TrainingCenter")
        if has_width:
            errors.append("pitch_width_mirim must be null for TrainingCenter")

    return errors

def validate_player(values: Dict[str, Any]) -> List[str]:
    errors: List[str] = []
    captaincy_role = values.get("captaincy_role")
    team_id = values.get("team_id")

    has_captaincy = captaincy_role is not None and str(captaincy_role).strip() != ""
    has_team = team_id is not None and str(team_id).strip() != ""

    if has_captaincy and not has_team:
        errors.append("team_id is required when captaincy_role is set")

    return errors

def validate_title(values: Dict[str, Any]) -> List[str]:
    errors: List[str] = []
    winner_team_id = values.get("winner_team_id")
    winner_federation_id = values.get("winner_federation_id")

    has_team = winner_team_id is not None and str(winner_team_id).strip() != ""
    has_fed = winner_federation_id is not None and str(winner_federation_id).strip() != ""

    if not has_team and not has_fed:
        errors.append("Either winner_team_id or winner_federation_id must be provided")
    elif has_team and has_fed:
        errors.append("Cannot set both winner_team_id and winner_federation_id")

    return errors
