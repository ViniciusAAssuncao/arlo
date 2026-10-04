from arlo_db_tool.schema.entities.position_codes import SHORT_POSITION_CODES
from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

PLAYER_POSITION_SPEC = EntitySpec(
    name="PlayerPosition",
    table_name="player_positions",
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
            name="position",
            column="position",
            field_type=FieldType.ENUM,
            enum_values=SHORT_POSITION_CODES,
            nullable=False,
        ),
        FieldSpec(
            name="proficiency",
            column="proficiency",
            field_type=FieldType.INTEGER,
            nullable=False,
            min_value=0,
            max_value=10,
            default=10,
        ),
    ],
)