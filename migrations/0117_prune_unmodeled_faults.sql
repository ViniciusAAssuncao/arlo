ALTER TABLE fault_definitions ADD COLUMN active INTEGER NOT NULL DEFAULT 1;

CREATE TEMP TABLE retired_fault_codes (code TEXT PRIMARY KEY);

INSERT INTO retired_fault_codes (code) VALUES
    ('artrine_protection_violation'),
    ('defenseless_player_hit'),
    ('delay_of_game'),
    ('equipment_violation'),
    ('excessive_time_call_delay'),
    ('false_drive_declaration'),
    ('fan_aggression'),
    ('feigning_injury'),
    ('illegal_call_to_action_pass'),
    ('illegal_countdown_execution'),
    ('illegal_drive_registration'),
    ('illegal_equipment_check'),
    ('illegal_formation'),
    ('illegal_formation_numbering'),
    ('illegal_line_engagement'),
    ('illegal_marking_scheme'),
    ('late_hit'),
    ('late_hit_out_of_bounds'),
    ('peel_back_block'),
    ('piling_on'),
    ('play_call_delay'),
    ('referee_contact'),
    ('sideline_encroachment'),
    ('spearing_defenseless'),
    ('wrong_substitution_protocol');

DELETE FROM fault_activation_contexts
WHERE fault_code IN (SELECT code FROM retired_fault_codes);

DELETE FROM fault_punishment_options
WHERE fault_definition_id IN (
    SELECT id FROM fault_definitions
    WHERE code IN (SELECT code FROM retired_fault_codes)
);

UPDATE fault_definitions
SET active = 0
WHERE code IN (SELECT code FROM retired_fault_codes);

DELETE FROM fault_definitions
WHERE active = 0
  AND id NOT IN (
      SELECT fault_definition_id FROM match_fouls
      WHERE fault_definition_id IS NOT NULL
  )
  AND id NOT IN (
      SELECT fault_definition_id FROM match_foul_punishments
      WHERE fault_definition_id IS NOT NULL
  )
  AND id NOT IN (
      SELECT fault_definition_id FROM match_referee_decisions
      WHERE fault_definition_id IS NOT NULL
  );

DROP TABLE retired_fault_codes;
