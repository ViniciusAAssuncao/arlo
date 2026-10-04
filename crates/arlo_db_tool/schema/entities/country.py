from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

COUNTRY_SPEC = EntitySpec(
    name="Country",
    table_name="countries",
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
        FieldSpec(
            name="continent_id",
            column="continent_id",
            field_type=FieldType.UUID_FK,
            nullable=False,
            fk_target_table="continents",
            fk_target_column="id",
        ),
        FieldSpec(
            name="federation_id",
            column="federation_id",
            field_type=FieldType.UUID_FK,
            nullable=True,
            fk_target_table="federations",
            fk_target_column="id",
        ),
    ],
)
