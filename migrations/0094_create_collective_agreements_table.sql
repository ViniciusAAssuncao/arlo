CREATE TABLE IF NOT EXISTS collective_agreements (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    rule_kind TEXT NOT NULL,
    start_month_order_index INTEGER NOT NULL,
    start_day_of_month INTEGER NOT NULL,
    end_month_order_index INTEGER NOT NULL,
    end_day_of_month INTEGER NOT NULL,
    end_year_offset INTEGER NOT NULL,
    created_at_unix_seconds INTEGER NOT NULL
);