from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType
from arlo_db_tool.validation.cross_field_rules import validate_competition

COMPETITION_SCOPES = [
    "Regional",
    "National",
    "Continental",
    "International",
]

COMPETITION_KINDS = [
    "League",
    "Cup",
    "Friendly",
]

COMPETITION_SPEC = EntitySpec(
    name="Competition",
    table_name="competitions",
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
            name="federation_id",
            column="federation_id",
            field_type=FieldType.UUID_FK,
            nullable=False,
            fk_target_table="federations",
            fk_target_column="id",
        ),
        FieldSpec(
            name="country_id",
            column="country_id",
            field_type=FieldType.UUID_FK,
            nullable=True,
            fk_target_table="countries",
            fk_target_column="id",
        ),
        FieldSpec(
            name="scope",
            column="scope",
            field_type=FieldType.ENUM,
            enum_values=COMPETITION_SCOPES,
            nullable=False,
        ),
        FieldSpec(
            name="kind",
            column="kind",
            field_type=FieldType.ENUM,
            enum_values=COMPETITION_KINDS,
            nullable=False,
        ),
        FieldSpec(
            name="prestige",
            column="prestige",
            field_type=FieldType.INTEGER,
            nullable=False,
            min_value=0,
            max_value=200,
            default=0,
        ),
    ],
    cross_validator=validate_competition,
)
