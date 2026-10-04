from typing import List
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

def get_person_fields() -> List[FieldSpec]:
    return [
        FieldSpec(
            name="id",
            column="id",
            field_type=FieldType.UUID_PK,
            primary_key=True,
        ),
        FieldSpec(
            name="name",
            column="name",
            field_type=FieldType.TEXT,
            nullable=False,
        ),
        FieldSpec(
            name="height_m",
            column="height_m",
            field_type=FieldType.REAL,
            nullable=False,
            min_value=0.5,
            max_value=2.5,
            default=1.85,
        ),
        FieldSpec(
            name="birthdate_unix_seconds",
            column="birthdate_unix_seconds",
            field_type=FieldType.UNIX_TIMESTAMP,
            nullable=False,
            default=946684800,
        ),
        FieldSpec(
            name="nationality_id",
            column="nationality_id",
            field_type=FieldType.UUID_FK,
            nullable=False,
            fk_target_table="countries",
            fk_target_column="id",
        ),
    ]
