import math
import re
from typing import Any, List, Optional, Sequence
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

HEX_COLOR_PATTERN = re.compile(r"^#[0-9a-fA-F]{6}$")

def validate_not_empty(value: Any, field_name: str) -> Optional[str]:
    if value is None:
        return f"{field_name} must not be empty"
    if isinstance(value, str) and not value.strip():
        return f"{field_name} must not be empty"
    return None

def validate_positive_finite(value: Any, field_name: str) -> Optional[str]:
    if value is None:
        return f"{field_name} must not be empty"
    try:
        val = float(value)
    except (ValueError, TypeError):
        return f"{field_name} must be a valid number"
    if not math.isfinite(val) or val <= 0.0:
        return f"{field_name} must be positive and finite"
    return None

def validate_integer_range(value: Any, min_val: int, max_val: int, field_name: str) -> Optional[str]:
    if value is None:
        return f"{field_name} must not be empty"
    try:
        val = int(value)
    except (ValueError, TypeError):
        return f"{field_name} must be a valid integer"
    if val < min_val or val > max_val:
        return f"{field_name} must be between {min_val} and {max_val}"
    return None

def validate_float_range(value: Any, min_val: float, max_val: float, field_name: str) -> Optional[str]:
    if value is None:
        return f"{field_name} must not be empty"
    try:
        val = float(value)
    except (ValueError, TypeError):
        return f"{field_name} must be a valid float"
    if not math.isfinite(val) or val < min_val or val > max_val:
        return f"{field_name} must be between {min_val} and {max_val}"
    return None

def validate_hex_color(value: Any, field_name: str) -> Optional[str]:
    if value is None or not isinstance(value, str):
        return f"{field_name} must be a valid hex color string"
    if not HEX_COLOR_PATTERN.match(value.strip()):
        return f"{field_name} must be a valid 6-digit hex color starting with #"
    return None

def validate_enum_value(value: Any, allowed_values: Sequence[str], field_name: str) -> Optional[str]:
    if value is None or str(value) not in allowed_values:
        return f"{field_name} must be one of {list(allowed_values)}"
    return None

def validate_field_spec(field: FieldSpec, value: Any) -> List[str]:
    errors: List[str] = []
    
    is_none_or_blank = value is None or (isinstance(value, str) and not value.strip())
    
    if is_none_or_blank:
        if not field.nullable:
            errors.append(f"{field.name} is required")
        return errors

    if field.field_type in (FieldType.TEXT, FieldType.UUID_PK, FieldType.UUID_FK):
        err = validate_not_empty(value, field.name)
        if err:
            errors.append(err)

    elif field.field_type in (FieldType.INTEGER, FieldType.UNIX_TIMESTAMP):
        try:
            val = int(value)
            if field.min_value is not None and field.max_value is not None:
                err = validate_integer_range(val, int(field.min_value), int(field.max_value), field.name)
                if err:
                    errors.append(err)
            elif field.min_value is not None:
                if val < int(field.min_value):
                    errors.append(f"{field.name} must be at least {int(field.min_value)}")
            elif field.max_value is not None:
                if val > int(field.max_value):
                    errors.append(f"{field.name} must be at most {int(field.max_value)}")
        except (ValueError, TypeError):
            errors.append(f"{field.name} must be a valid integer")

    elif field.field_type == FieldType.REAL:
        try:
            val = float(value)
            if field.min_value is not None and field.max_value is not None:
                err = validate_float_range(val, float(field.min_value), float(field.max_value), field.name)
                if err:
                    errors.append(err)
            elif field.min_value is not None:
                if val < float(field.min_value):
                    errors.append(f"{field.name} must be at least {field.min_value}")
            elif field.max_value is not None:
                if val > float(field.max_value):
                    errors.append(f"{field.name} must be at most {field.max_value}")
        except (ValueError, TypeError):
            errors.append(f"{field.name} must be a valid float")

    elif field.field_type == FieldType.HEX_COLOR:
        err = validate_hex_color(value, field.name)
        if err:
            errors.append(err)

    elif field.field_type == FieldType.ENUM:
        if field.enum_values:
            err = validate_enum_value(value, field.enum_values, field.name)
            if err:
                errors.append(err)

    return errors
