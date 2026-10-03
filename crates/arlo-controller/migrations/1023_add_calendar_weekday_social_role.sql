ALTER TABLE calendar_week_days ADD COLUMN social_role TEXT CHECK (social_role IN ('workday', 'rest_day'));
