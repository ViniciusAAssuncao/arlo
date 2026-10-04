from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType
from arlo_db_tool.validation.cross_field_rules import validate_federation

FEDERATION_SCOPES = [
    "Regional",
    "National",
    "Continental",
    "International",
]

FEDERATION_SPEC = EntitySpec(
    name="Federation",
    table_name="federations",
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
            name="scope",
            column="scope",
            field_type=FieldType.ENUM,
            enum_values=FEDERATION_SCOPES,
            nullable=False,
        ),
        FieldSpec(
            name="continent_id",
            column="continent_id",
            field_type=FieldType.UUID_FK,
            nullable=True,
            fk_target_table="continents",
            fk_target_column="id",
        ),
        FieldSpec(
            name="parent_federation_id",
            column="parent_federation_id",
            field_type=FieldType.UUID_FK,
            nullable=True,
            fk_target_table="federations",
            fk_target_column="id",
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
    cross_validator=validate_federation,
)
