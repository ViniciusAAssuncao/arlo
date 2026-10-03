INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'AerialDuel', offender_role, weight
FROM fault_activation_contexts
WHERE context = 'RouteContest';

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'ShortDistribution', offender_role, weight
FROM fault_activation_contexts
WHERE context = 'RouteContest';

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'LongDistribution', offender_role, weight
FROM fault_activation_contexts
WHERE context = 'RouteContest';

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'CrossDistribution', offender_role, weight
FROM fault_activation_contexts
WHERE context = 'RouteContest';

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'RunBreakthrough', offender_role, weight
FROM fault_activation_contexts
WHERE context = 'BallSecurityCarry';

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'ArtroBreakthrough', offender_role, weight
FROM fault_activation_contexts
WHERE context = 'BallSecurityCarry';

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'BallSecurityDistribution', offender_role, weight
FROM fault_activation_contexts
WHERE context = 'BallSecurityCarry';

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'PassProtection', offender_role, weight
FROM fault_activation_contexts
WHERE context IN ('RouteContest', 'BallSecurityCarry')
  AND fault_code IN (
      'illegal_block',
      'blindside_block',
      'illegal_cut_block',
      'chop_block',
      'crackback_block',
      'clipping_block',
      'peel_back_block',
      'illegal_use_of_hands',
      'illegal_hold',
      'illegal_line_engagement',
      'roughing_the_passer',
      'unnecessary_roughness',
      'helmet_contact'
  );

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'CentralBlock', offender_role, weight
FROM fault_activation_contexts
WHERE context IN ('RouteContest', 'BallSecurityCarry')
  AND fault_code IN (
      'illegal_block',
      'blindside_block',
      'illegal_cut_block',
      'chop_block',
      'crackback_block',
      'clipping_block',
      'peel_back_block',
      'illegal_use_of_hands',
      'illegal_hold',
      'illegal_line_engagement',
      'unnecessary_roughness',
      'helmet_contact'
  );

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'LateralBlock', offender_role, weight
FROM fault_activation_contexts
WHERE context IN ('RouteContest', 'BallSecurityCarry')
  AND fault_code IN (
      'illegal_block',
      'blindside_block',
      'illegal_cut_block',
      'chop_block',
      'crackback_block',
      'clipping_block',
      'peel_back_block',
      'illegal_use_of_hands',
      'illegal_hold',
      'illegal_line_engagement',
      'unnecessary_roughness',
      'helmet_contact'
  );

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'FinishingAttempt', offender_role, weight
FROM fault_activation_contexts
WHERE context = 'ShotAttempt';

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'FieldGoalAttempt', offender_role, weight
FROM fault_activation_contexts
WHERE context = 'ShotAttempt';

INSERT OR IGNORE INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT fault_code, 'KickBlockAttempt', offender_role, weight
FROM fault_activation_contexts
WHERE context = 'ShotAttempt';
