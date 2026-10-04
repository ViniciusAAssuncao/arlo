from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

LEAGUE_SPEC = EntitySpec(
    name="League",
    table_name="leagues",
    fields=[
        FieldSpec(
            name="competition_id",
            column="competition_id",
            field_type=FieldType.UUID_FK,
            primary_key=True,
            nullable=False,
            fk_target_table="competitions",
            fk_target_column="id",
            fk_where="kind = 'League'",
        ),
        FieldSpec(
            name="division_index",
            column="division_index",
            field_type=FieldType.INTEGER,
            nullable=False,
            min_value=0,
            default=0,
        ),
    ],
)
