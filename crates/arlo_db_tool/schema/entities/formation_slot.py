from arlo_db_tool.schema.entities.position_codes import SHORT_POSITION_CODES
from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

FORMATION_SLOT_SPEC = EntitySpec(
    name="FormationSlot",
    table_name="formation_slots",
    fields=[
        FieldSpec(
            name="id",
            column="id",
            field_type=FieldType.UUID_PK,
            primary_key=True,
        ),
        FieldSpec(
            name="formation_id",
            column="formation_id",
            field_type=FieldType.UUID_FK,
            nullable=False,
            fk_target_table="formations",
            fk_target_column="id",
        ),
        FieldSpec(
            name="slot_index",
            column="slot_index",
            field_type=FieldType.INTEGER,
            nullable=False,
            min_value=0,
            max_value=13,
            default=0,
        ),
        FieldSpec(
            name="position",
            column="position",
            field_type=FieldType.ENUM,
            enum_values=SHORT_POSITION_CODES,
            nullable=False,
        ),
        FieldSpec(
            name="pitch_length_ratio",
            column="pitch_length_ratio",
            field_type=FieldType.REAL,
            nullable=False,
            min_value=0.0,
            max_value=1.0,
            default=0.5,
        ),
        FieldSpec(
            name="pitch_width_ratio",
            column="pitch_width_ratio",
            field_type=FieldType.REAL,
            nullable=False,
            min_value=0.0,
            max_value=1.0,
            default=0.5,
        ),
    ],
)
