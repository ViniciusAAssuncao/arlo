from arlo_db_tool.schema.entities.person_fields import get_person_fields
from arlo_db_tool.schema.entity_spec import CompositeEntitySpec, EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

MANAGER_CONTROL_MODES = [
    "Ai",
    "Human",
]

OFFENSIVE_APPROACHES = [
    "Positional",
    "Functional",
    "Direct",
    "Balanced",
]

DEFENSIVE_APPROACHES = [
    "HighPress",
    "MidBlock",
    "DeepLowBlock",
]

ROTATION_POLICIES = [
    "StrictCore",
    "Situational",
    "HighRotation",
]

ARTRINE_DEPENDENCIES = [
    "SystemDriven",
    "ArtrineCentric",
]

MANAGER_PERSON_SPEC = EntitySpec(
    name="Person",
    table_name="persons",
    fields=get_person_fields(),
)

MANAGER_INNER_SPEC = EntitySpec(
    name="ManagerInner",
    table_name="managers",
    fields=[
        FieldSpec(
            name="id",
            column="id",
            field_type=FieldType.UUID_PK,
            primary_key=True,
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
            name="control_mode",
            column="control_mode",
            field_type=FieldType.ENUM,
            enum_values=MANAGER_CONTROL_MODES,
            nullable=False,
            default="Ai",
        ),
    ],
)

MANAGER_SPEC = CompositeEntitySpec(
    name="Manager",
    specs=[MANAGER_PERSON_SPEC, MANAGER_INNER_SPEC],
)
