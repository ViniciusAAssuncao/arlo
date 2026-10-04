from arlo_db_tool.schema.entity_spec import EntitySpec
from arlo_db_tool.schema.field_spec import FieldSpec, FieldType

CALENDAR_DAY_SOCIAL_ROLES = [
    "workday",
    "rest_day",
]

CALENDAR_WEEK_DAY_SPEC = EntitySpec(
    name="CalendarWeekDay",
    table_name="calendar_week_days",
    fields=[
        FieldSpec(
            name="id",
            column="id",
            field_type=FieldType.UUID_PK,
            primary_key=True,
        ),
        FieldSpec(
            name="calendar_system_id",
            column="calendar_system_id",
            field_type=FieldType.UUID_FK,
            nullable=False,
            fk_target_table="calendar_systems",
            fk_target_column="id",
        ),
        FieldSpec(
            name="order_index",
            column="order_index",
            field_type=FieldType.INTEGER,
            nullable=False,
            min_value=0,
        ),
        FieldSpec(
            name="name",
            column="name",
            field_type=FieldType.TEXT,
            nullable=False,
        ),
        FieldSpec(
            name="social_role",
            column="social_role",
            field_type=FieldType.ENUM,
            nullable=True,
            enum_values=CALENDAR_DAY_SOCIAL_ROLES,
        ),
    ],
)
