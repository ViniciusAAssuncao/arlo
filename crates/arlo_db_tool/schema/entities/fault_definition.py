from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

FAULT_SEVERITIES = [
    "Minor",
    "Moderate",
    "Severe",
    "Flagrant",
]

FAULT_DEFINITION_SPEC = EntitySpec(
    name="FaultDefinition",
    table_name="fault_definitions",
    fields=[
        FieldSpec(
            name="id",
            column="id",
            field_type=FieldType.UUID_PK,
            primary_key=True,
        ),
        FieldSpec(
            name="code",
            column="code",
            field_type=FieldType.TEXT,
            nullable=False,
        ),
        FieldSpec(
            name="description",
            column="description",
            field_type=FieldType.TEXT,
            nullable=False,
        ),
        FieldSpec(
            name="severity",
            column="severity",
            field_type=FieldType.ENUM,
            enum_values=FAULT_SEVERITIES,
            nullable=False,
        ),
    ],
)
