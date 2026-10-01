DELETE FROM fault_activation_contexts
WHERE fault_code IN ('deliberate_injury_attempt', 'weapon_use');

DELETE FROM fault_punishment_options
WHERE fault_definition_id IN (
    SELECT id FROM fault_definitions
    WHERE code IN ('deliberate_injury_attempt', 'weapon_use')
);

DELETE FROM fault_definitions
WHERE code IN ('deliberate_injury_attempt', 'weapon_use');
