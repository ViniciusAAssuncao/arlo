from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

PLAYER_ATTRIBUTE_SPEC = EntitySpec(
    name="PlayerAttribute",
    table_name="player_attributes",
    fields=[
        FieldSpec(
            name="player_id",
            column="player_id",
            field_type=FieldType.UUID_FK,
            primary_key=True,
            nullable=False,
            fk_target_table="players",
            fk_target_column="id",
        ),
        FieldSpec(
            name="attribute_definition_id",
            column="attribute_definition_id",
            field_type=FieldType.UUID_FK,
            nullable=False,
            fk_target_table="attribute_definitions",
            fk_target_column="id",
            fk_where="applies_to = 'Player'",
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