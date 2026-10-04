from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType
from arlo_db_tool.validation.cross_field_rules import validate_venue

VENUE_KINDS = [
    "MatchStadium",
    "TrainingCenter",
]

VENUE_SPEC = EntitySpec(
    name="Venue",
    table_name="venues",
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
            name="kind",
            column="kind",
            field_type=FieldType.ENUM,
            enum_values=VENUE_KINDS,
            nullable=False,
        ),
        FieldSpec(
            name="owner_team_id",
            column="owner_team_id",
            field_type=FieldType.UUID_FK,
            nullable=True,
            fk_target_table="teams",
            fk_target_column="id",
        ),
        FieldSpec(
            name="country_id",
            column="country_id",
            field_type=FieldType.UUID_FK,
            nullable=False,
            fk_target_table="countries",
            fk_target_column="id",
        ),
        FieldSpec(
            name="capacity",
            column="capacity",
            field_type=FieldType.INTEGER,
            nullable=True,
            min_value=0,
        ),
        FieldSpec(
            name="pitch_length_mirim",
            column="pitch_length_mirim",
            field_type=FieldType.REAL,
            nullable=True,
            min_value=140.0,
            max_value=150.0,
        ),
        FieldSpec(
            name="pitch_width_mirim",
            column="pitch_width_mirim",
            field_type=FieldType.REAL,
            nullable=True,
            min_value=80.0,
            max_value=90.0,
        ),
    ],
    cross_validator=validate_venue,
)
