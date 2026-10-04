from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

INJURY_MECHANISMS = [
    "Contact",
    "NonContact",
]

BODY_REGIONS = [
    "Head",
    "Neck",
    "Shoulder",
    "Arm",
    "Hand",
    "Trunk",
    "Hip",
    "Groin",
    "Thigh",
    "Knee",
    "Calf",
    "Ankle",
    "Foot",
]

INJURY_DEFINITION_SPEC = EntitySpec(
    name="InjuryDefinition",
    table_name="injury_definitions",
    fields=[
        FieldSpec(
            name="id",
            column="id",
            field_type=FieldType.UUID_PK,
            primary_key=True,
        ),
        FieldSpec(
            name="code",
            column="code",
            field_type=FieldType.TEXT,
            nullable=False,
        ),
        FieldSpec(
            name="description",
            column="description",
            field_type=FieldType.TEXT,
            nullable=False,
        ),
        FieldSpec(
            name="mechanism",
            column="mechanism",
            field_type=FieldType.ENUM,
            enum_values=INJURY_MECHANISMS,
            nullable=False,
        ),
        FieldSpec(
            name="body_region",
            column="body_region",
            field_type=FieldType.ENUM,
            enum_values=BODY_REGIONS,
            nullable=False,
        ),
        FieldSpec(
            name="relative_frequency",
            column="relative_frequency",
            field_type=FieldType.REAL,
            nullable=False,
            min_value=0.0001,
            default=1.0,
        ),
    ],
)
