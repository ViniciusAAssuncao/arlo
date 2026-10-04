from arlo_db_tool.schema.entities.person_fields import get_person_fields
from arlo_db_tool.schema.entity_spec import CompositeEntitySpec, EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

REFEREE_TIERS = [
    "Regional",
    "National",
    "Continental",
    "International",
]

REFEREE_PERSON_SPEC = EntitySpec(
    name="Person",
    table_name="persons",
    fields=get_person_fields(),
)

REFEREE_INNER_SPEC = EntitySpec(
    name="RefereeInner",
    table_name="referees",
    fields=[
        FieldSpec(
            name="id",
            column="id",
            field_type=FieldType.UUID_PK,
            primary_key=True,
        ),
        FieldSpec(
            name="primary_league_id",
            column="primary_league_id",
            field_type=FieldType.UUID_FK,
            nullable=True,
            fk_target_table="competitions",
            fk_target_column="id",
            fk_where="kind = 'League'",
        ),
        FieldSpec(
            name="tier",
            column="tier",
            field_type=FieldType.ENUM,
            enum_values=REFEREE_TIERS,
            nullable=False,
            default="National",
        ),
    ],
)

REFEREE_SPEC = CompositeEntitySpec(
    name="Referee",
    specs=[REFEREE_PERSON_SPEC, REFEREE_INNER_SPEC],
)
