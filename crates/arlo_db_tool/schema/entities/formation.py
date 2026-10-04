from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

FORMATION_SPEC = EntitySpec(
    name="Formation",
    table_name="formations",
    fields=[
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
    ],
)
