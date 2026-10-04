from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

MANAGER_ATTRIBUTE_SPEC = EntitySpec(
    name="ManagerAttribute",
    table_name="manager_attributes",
    fields=[
        FieldSpec(
            name="manager_id",
            column="manager_id",
            field_type=FieldType.UUID_FK,
            primary_key=True,
            nullable=False,
            fk_target_table="persons",
            fk_target_column="id",
            fk_where="id IN (SELECT id FROM managers)",
        ),
        FieldSpec(
            name="attribute_definition_id",
            column="attribute_definition_id",
            field_type=FieldType.UUID_FK,
            nullable=False,
            fk_target_table="attribute_definitions",
            fk_target_column="id",
            fk_where="applies_to = 'Manager'",
        ),
        FieldSpec(
            name="value",
            column="value",
            field_type=FieldType.INTEGER,
            nullable=False,
            min_value=0,
            max_value=20,
            default=10,
        ),
    ],
)