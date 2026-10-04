from typing import Any, Dict, List, Optional
import uuid
from arlo_db_tool.schema.entity_spec import CompositeEntitySpec, EntitySpec
from arlo_db_tool.sql.value_formatting import format_sql_value

def build_insert_statement(spec: EntitySpec, values: Dict[str, Any]) -> str:
    columns: List[str] = []
    formatted_values: List[str] = []

    for field in spec.fields:
        raw_val = None
        if field.name in values:
            raw_val = values[field.name]
        elif field.column in values:
            raw_val = values[field.column]
        elif field.default is not None:
            raw_val = field.default

        columns.append(field.column)
        formatted_values.append(format_sql_value(raw_val, field.field_type))

    cols_clause = ", ".join(columns)
    vals_clause = ", ".join(formatted_values)
    return f"INSERT INTO {spec.table_name} ({cols_clause}) VALUES ({vals_clause});"

def build_update_statement(spec: EntitySpec, pk_value: Any, values: Dict[str, Any]) -> str:
    pk_field = spec.primary_key_field()
    if pk_field is None:
        raise ValueError(f"EntitySpec {spec.name} has no primary key defined")

    set_clauses: List[str] = []
    for field in spec.fields:
        if field.primary_key:
            continue

        raw_val = None
        has_key = False
        if field.name in values:
            raw_val = values[field.name]
            has_key = True
        elif field.column in values:
            raw_val = values[field.column]
            has_key = True

        if has_key:
            formatted = format_sql_value(raw_val, field.field_type)
            set_clauses.append(f"{field.column} = {formatted}")

    if not set_clauses:
        return ""

    pk_formatted = format_sql_value(pk_value, pk_field.field_type)
    set_str = ", ".join(set_clauses)
    return f"UPDATE {spec.table_name} SET {set_str} WHERE {pk_field.column} = {pk_formatted};"

def build_delete_statement(spec: EntitySpec, pk_value: Any) -> str:
    pk_field = spec.primary_key_field()
    if pk_field is None:
        raise ValueError(f"EntitySpec {spec.name} has no primary key defined")

    pk_formatted = format_sql_value(pk_value, pk_field.field_type)
    return f"DELETE FROM {spec.table_name} WHERE {pk_field.column} = {pk_formatted};"

def build_composite_insert_statements(
    spec: CompositeEntitySpec,
    values: Dict[str, Any],
    shared_id: Optional[str] = None,
) -> List[str]:
    statements: List[str] = []
    scoped_values = dict(values)

    if shared_id is not None:
        pk_field = spec.primary_key_field()
        if pk_field is not None:
            scoped_values[pk_field.name] = shared_id
            scoped_values[pk_field.column] = shared_id

    for sub_spec in spec.specs:
        statements.append(build_insert_statement(sub_spec, scoped_values))

    return statements

def build_composite_update_statements(
    spec: CompositeEntitySpec,
    pk_value: Any,
    values: Dict[str, Any],
) -> List[str]:
    statements: List[str] = []
    for sub_spec in spec.specs:
        stmt = build_update_statement(sub_spec, pk_value, values)
        if stmt:
            statements.append(stmt)
    return statements

def build_composite_delete_statements(
    spec: CompositeEntitySpec,
    pk_value: Any,
) -> List[str]:
    statements: List[str] = []
    for sub_spec in reversed(spec.specs):
        statements.append(build_delete_statement(sub_spec, pk_value))
    return statements

def build_attribute_insert_statements(
    table_name: str,
    fk_column: str,
    entity_id: str,
    attribute_values: Dict[str, Any],
) -> List[str]:
    statements: List[str] = []
    for def_id, raw_val in attribute_values.items():
        if raw_val is None or str(raw_val).strip() == "":
            continue
        try:
            val = int(raw_val)
        except (ValueError, TypeError):
            continue
        stmt = (
            f"INSERT INTO {table_name} ({fk_column}, attribute_definition_id, value) "
            f"VALUES ('{entity_id}', '{def_id}', {val});"
        )
        statements.append(stmt)
    return statements

def build_attribute_update_statements(
    table_name: str,
    fk_column: str,
    entity_id: str,
    attribute_values: Dict[str, Any],
    original_attributes: Dict[str, Dict[str, Any]],
) -> List[str]:
    statements: List[str] = []
    for def_id, raw_val in attribute_values.items():
        if raw_val is None or str(raw_val).strip() == "":
            continue
        try:
            val = int(raw_val)
        except (ValueError, TypeError):
            continue

        if def_id in original_attributes:
            orig = original_attributes[def_id]
            orig_val = orig.get("value")
            if val != orig_val:
                stmt = (
                    f"UPDATE {table_name} SET value = {val} "
                    f"WHERE {fk_column} = '{entity_id}' AND attribute_definition_id = '{def_id}';"
                )
                statements.append(stmt)
        else:
            stmt = (
                f"INSERT INTO {table_name} ({fk_column}, attribute_definition_id, value) "
                f"VALUES ('{entity_id}', '{def_id}', {val});"
            )
            statements.append(stmt)
    return statements

def build_attribute_delete_statements(
    table_name: str,
    fk_column: str,
    entity_id: str,
) -> List[str]:
    return [f"DELETE FROM {table_name} WHERE {fk_column} = '{entity_id}';"]

def build_player_position_insert_statements(
    player_id: str,
    positions: Dict[str, int],
) -> List[str]:
    statements: List[str] = []
    for pos_code, proficiency in positions.items():
        stmt = (
            f"INSERT INTO player_positions (player_id, position, proficiency) "
            f"VALUES ('{player_id}', '{pos_code}', {proficiency});"
        )
        statements.append(stmt)
    return statements

def build_player_position_update_statements(
    player_id: str,
    current_positions: Dict[str, int],
    original_positions: Dict[str, int],
) -> List[str]:
    statements: List[str] = []
    for pos_code, prof in current_positions.items():
        if pos_code in original_positions:
            if original_positions[pos_code] != prof:
                stmt = (
                    f"UPDATE player_positions SET proficiency = {prof} "
                    f"WHERE player_id = '{player_id}' AND position = '{pos_code}';"
                )
                statements.append(stmt)
        else:
            stmt = (
                f"INSERT INTO player_positions (player_id, position, proficiency) "
                f"VALUES ('{player_id}', '{pos_code}', {prof});"
            )
            statements.append(stmt)
    
    for pos_code in original_positions:
        if pos_code not in current_positions:
            stmt = f"DELETE FROM player_positions WHERE player_id = '{player_id}' AND position = '{pos_code}';"
            statements.append(stmt)
            
    return statements

def build_player_position_delete_statements(player_id: str) -> List[str]:
    return [f"DELETE FROM player_positions WHERE player_id = '{player_id}';"]

def build_manager_tactical_profile_delete_statements(manager_id: str) -> List[str]:
    return [
        f"DELETE FROM manager_preferred_formations WHERE manager_tactical_profile_id IN (SELECT id FROM manager_tactical_profiles WHERE manager_id = '{manager_id}');",
        f"DELETE FROM manager_tactical_profiles WHERE manager_id = '{manager_id}';"
    ]

def build_manager_tactical_profile_insert_statements(
    manager_id: str,
    tp_values: Dict[str, Any],
    pf_values: List[str]
) -> List[str]:
    profile_id = str(uuid.uuid4())
    stmts = []

    cols = [
        "id", "manager_id", "offensive_approach", "defensive_approach", 
        "rotation_policy", "artrine_dependency", "flexibility_tendency", 
        "passing_range_preference", "aeriality_preference", "structure_preference", 
        "physicality_preference", "transition_pace_preference", "press_block_shape_preference"
    ]

    vals = [
        f"'{profile_id}'",
        f"'{manager_id}'",
        f"'{tp_values['offensive_approach']}'",
        f"'{tp_values['defensive_approach']}'",
        f"'{tp_values['rotation_policy']}'",
        f"'{tp_values['artrine_dependency']}'",
        str(tp_values['flexibility_tendency']),
        str(tp_values['passing_range_preference']),
        str(tp_values['aeriality_preference']),
        str(tp_values['structure_preference']),
        str(tp_values['physicality_preference']),
        str(tp_values['transition_pace_preference']),
        str(tp_values['press_block_shape_preference']),
    ]
    stmts.append(f"INSERT INTO manager_tactical_profiles ({', '.join(cols)}) VALUES ({', '.join(vals)});")

    for f_id in pf_values:
        pf_id = str(uuid.uuid4())
        stmts.append(f"INSERT INTO manager_preferred_formations (id, manager_tactical_profile_id, formation_id) VALUES ('{pf_id}', '{profile_id}', '{f_id}');")

    return stmts

def build_manager_tactical_profile_update_statements(
    manager_id: str,
    tp_enabled: bool,
    tp_values: Dict[str, Any],
    pf_values: List[str],
    orig_tp: Optional[Dict[str, Any]],
    orig_pf: List[str]
) -> List[str]:
    if not tp_enabled:
        if orig_tp:
            return build_manager_tactical_profile_delete_statements(manager_id)
        return []

    if not orig_tp:
        return build_manager_tactical_profile_insert_statements(manager_id, tp_values, pf_values)

    stmts = []
    profile_id = str(orig_tp["id"])
    set_clauses = []
    str_fields = ["offensive_approach", "defensive_approach", "rotation_policy", "artrine_dependency"]
    float_fields = [
        "flexibility_tendency", "passing_range_preference", "aeriality_preference", 
        "structure_preference", "physicality_preference", "transition_pace_preference", 
        "press_block_shape_preference"
    ]

    for f in str_fields:
        if tp_values[f] != orig_tp.get(f):
            set_clauses.append(f"{f} = '{tp_values[f]}'")

    for f in float_fields:
        try:
            ov = float(orig_tp.get(f, 0.0))
        except (ValueError, TypeError):
            ov = 0.0
        if abs(tp_values[f] - ov) > 1e-6:
            set_clauses.append(f"{f} = {tp_values[f]}")

    if set_clauses:
        stmts.append(f"UPDATE manager_tactical_profiles SET {', '.join(set_clauses)} WHERE id = '{profile_id}';")

    added = [x for x in pf_values if x not in orig_pf]
    removed = [x for x in orig_pf if x not in pf_values]

    for r in removed:
        stmts.append(f"DELETE FROM manager_preferred_formations WHERE manager_tactical_profile_id = '{profile_id}' AND formation_id = '{r}';")

    for a in added:
        new_id = str(uuid.uuid4())
        stmts.append(f"INSERT INTO manager_preferred_formations (id, manager_tactical_profile_id, formation_id) VALUES ('{new_id}', '{profile_id}', '{a}');")

    return stmts
