from enum import Enum, auto
from typing import Any, List, Optional, Union

class FieldType(Enum):
    TEXT = auto()
    INTEGER = auto()
    REAL = auto()
    BOOLEAN = auto()
    UUID_PK = auto()
    UUID_FK = auto()
    ENUM = auto()
    UNIX_TIMESTAMP = auto()
    HEX_COLOR = auto()

class FieldSpec:
    def __init__(
        self,
        name: str,
        column: str,
        field_type: FieldType,
        nullable: bool = False,
        min_value: Optional[Union[int, float]] = None,
        max_value: Optional[Union[int, float]] = None,
        enum_values: Optional[List[str]] = None,
        fk_target_table: Optional[str] = None,
        fk_target_column: Optional[str] = None,
        fk_where: Optional[str] = None,
        default: Any = None,
        primary_key: bool = False,
    ):
        self.name = name
        self.column = column
        self.field_type = field_type
        self.nullable = nullable
        self.min_value = min_value
        self.max_value = max_value
        self.enum_values = enum_values or []
        self.fk_target_table = fk_target_table
        self.fk_target_column = fk_target_column
        self.fk_where = fk_where
        self.default = default
        self.primary_key = primary_key or (field_type == FieldType.UUID_PK)
