from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType
from arlo_db_tool.validation.cross_field_rules import validate_title

TITLE_SPEC = EntitySpec(
    name="Title",
    table_name="titles",
    fields=[
        FieldSpec(
            name="id",
            column="id",
            field_type=FieldType.UUID_PK,
            primary_key=True,
        ),
        FieldSpec(
            name="competition_id",
            column="competition_id",
            field_type=FieldType.UUID_FK,
            nullable=False,
            fk_target_table="competitions",
            fk_target_column="id",
        ),
        FieldSpec(
            name="season_label",
            column="season_label",
            field_type=FieldType.TEXT,
            nullable=False,
        ),
        FieldSpec(
            name="winner_team_id",
            column="winner_team_id",
            field_type=FieldType.UUID_FK,
            nullable=True,
            fk_target_table="teams",
            fk_target_column="id",
        ),
        FieldSpec(
            name="winner_federation_id",
            column="winner_federation_id",
            field_type=FieldType.UUID_FK,
            nullable=True,
            fk_target_table="federations",
            fk_target_column="id",
        ),
    ],
    cross_validator=validate_title,
)
