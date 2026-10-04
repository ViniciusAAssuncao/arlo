from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

RULE_CATEGORIES = [
    "Calendar",
    "Teams",
    "Scoring",
    "Phases",
    "TieBreaker",
    "PromotionRelegation",
    "FieldGeometry",
]

RULE_SPEC = EntitySpec(
    name="Rule",
    table_name="rules",
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
            name="category",
            column="category",
            field_type=FieldType.ENUM,
            enum_values=RULE_CATEGORIES,
            nullable=False,
        ),
        FieldSpec(
            name="rule_key",
            column="rule_key",
            field_type=FieldType.TEXT,
            nullable=False,
        ),
        FieldSpec(
            name="value",
            column="value",
            field_type=FieldType.TEXT,
            nullable=False,
        ),
    ],
)
