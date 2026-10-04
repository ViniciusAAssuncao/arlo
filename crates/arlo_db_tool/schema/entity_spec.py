from typing import Any, Callable, Dict, List, Optional
from arlo_db_tool.schema.field_spec import FieldSpec

class EntitySpec:
    def __init__(
        self,
        name: str,
        table_name: str,
        fields: List[FieldSpec],
        cross_validator: Optional[Callable[[Dict[str, Any]], List[str]]] = None,
    ):
        self.name = name
        self.table_name = table_name
        self.fields = fields
        self.cross_validator = cross_validator

    def primary_key_field(self) -> Optional[FieldSpec]:
        for field in self.fields:
            if field.primary_key:
                return field
        return None

    def get_field(self, name: str) -> Optional[FieldSpec]:
        for field in self.fields:
            if field.name == name:
                return field
        return None

    def get_field_by_column(self, column: str) -> Optional[FieldSpec]:
        for field in self.fields:
            if field.column == column:
                return field
        return None

class CompositeEntitySpec:
    def __init__(
        self,
        name: str,
        specs: List[EntitySpec],
        cross_validator: Optional[Callable[[Dict[str, Any]], List[str]]] = None,
    ):
        self.name = name
        self.specs = specs
        self.cross_validator = cross_validator

    def primary_key_field(self) -> Optional[FieldSpec]:
        if self.specs:
            return self.specs[0].primary_key_field()
        return None

    def get_all_fields(self) -> List[FieldSpec]:
        seen = set()
        result = []
        for spec in self.specs:
            for field in spec.fields:
                if field.name not in seen:
                    seen.add(field.name)
                    result.append(field)
        return result

    def get_field(self, name: str) -> Optional[FieldSpec]:
        for spec in self.specs:
            field = spec.get_field(name)
            if field is not None:
                return field
        return None
