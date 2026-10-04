from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType
from arlo_db_tool.validation.cross_field_rules import validate_player

CAPTAINCY_ROLES = [
    "Captain",
    "ViceCaptain",
]

PLAYER_SPEC = EntitySpec(
    name="Player",
    table_name="players",
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
            name="height_m",
            column="height_m",
            field_type=FieldType.REAL,
            nullable=False,
            min_value=0.5,
            max_value=2.5,
            default=1.85,
        ),
        FieldSpec(
            name="birthdate_unix_seconds",
            column="birthdate_unix_seconds",
            field_type=FieldType.UNIX_TIMESTAMP,
            nullable=False,
            default=946684800,
        ),
        FieldSpec(
            name="nationality_id",
            column="nationality_id",
            field_type=FieldType.UUID_FK,
            nullable=False,
            fk_target_table="countries",
            fk_target_column="id",
        ),
        FieldSpec(
            name="team_id",
            column="team_id",
            field_type=FieldType.UUID_FK,
            nullable=True,
            fk_target_table="teams",
            fk_target_column="id",
        ),
        FieldSpec(
            name="squad_number",
            column="squad_number",
            field_type=FieldType.INTEGER,
            nullable=True,
            min_value=0,
            max_value=100,
        ),
        FieldSpec(
            name="captaincy_role",
            column="captaincy_role",
            field_type=FieldType.ENUM,
            enum_values=CAPTAINCY_ROLES,
            nullable=True,
        ),
    ],
    cross_validator=validate_player,
)
