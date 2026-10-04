ALTER TABLE award_definitions ADD COLUMN minimum_age INTEGER CHECK (minimum_age >= 0);
ALTER TABLE award_definitions ADD COLUMN maximum_age INTEGER CHECK (maximum_age >= 0);
