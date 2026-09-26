UPDATE attribute_definitions
SET key = 'aggressiveness',
    display_name = 'Agressividade'
WHERE key = 'controlled_aggression';

INSERT INTO attribute_definitions (id, key, display_name, category, applies_to)
SELECT
    lower(hex(randomblob(4))) || '-' || lower(hex(randomblob(2))) || '-4' || substr(lower(hex(randomblob(2))),2) || '-a' || substr(lower(hex(randomblob(2))),2) || '-' || lower(hex(randomblob(6))),
    'aggressiveness',
    'Agressividade',
    'Mental',
    'Player'
WHERE NOT EXISTS (
    SELECT 1 FROM attribute_definitions WHERE key = 'aggressiveness'
);