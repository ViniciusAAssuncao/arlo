CREATE TABLE fault_activation_contexts (
    fault_code TEXT NOT NULL,
    context TEXT NOT NULL,
    offender_role TEXT NOT NULL,
    weight REAL NOT NULL CHECK (weight > 0),
    PRIMARY KEY (fault_code, context, offender_role)
);

INSERT INTO fault_activation_contexts (fault_code, context, offender_role, weight)
SELECT m.code, m.context, m.offender_role, m.weight
FROM (
    SELECT 'RouteContest' AS context, 'Defense' AS offender_role, 'defensive_holding' AS code, 1.0 AS weight
    UNION ALL SELECT 'RouteContest', 'Defense', 'illegal_route_interference', 0.7
    UNION ALL SELECT 'RouteContest', 'Defense', 'face_guard_violation', 0.5
    UNION ALL SELECT 'RouteContest', 'Either', 'illegal_use_of_hands', 0.7
    UNION ALL SELECT 'RouteContest', 'Either', 'illegal_hold', 0.8
    UNION ALL SELECT 'RouteContest', 'Offense', 'illegal_block', 0.5
    UNION ALL SELECT 'RouteContest', 'Offense', 'blindside_block', 0.15
    UNION ALL SELECT 'RouteContest', 'Defense', 'artrine_protection_violation', 0.08
    UNION ALL SELECT 'RouteContest', 'Defense', 'roughing_the_artrine', 0.06
    UNION ALL SELECT 'RouteContest', 'Defense', 'roughing_the_passer', 0.03
    UNION ALL SELECT 'RouteContest', 'Offense', 'illegal_cut_block', 0.24
    UNION ALL SELECT 'RouteContest', 'Offense', 'chop_block', 0.13
    UNION ALL SELECT 'RouteContest', 'Offense', 'crackback_block', 0.13
    UNION ALL SELECT 'RouteContest', 'Offense', 'clipping_block', 0.06
    UNION ALL SELECT 'RouteContest', 'Offense', 'peel_back_block', 0.12
    UNION ALL SELECT 'RouteContest', 'Either', 'illegal_line_engagement', 0.26
    UNION ALL SELECT 'BallSecurityCarry', 'Defense', 'illegal_tackle', 0.8
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'tripping_opponent', 0.6
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'dangerous_tripping', 0.08
    UNION ALL SELECT 'BallSecurityCarry', 'Defense', 'face_mask', 0.10
    UNION ALL SELECT 'BallSecurityCarry', 'Defense', 'illegal_face_mask_tackle', 0.08
    UNION ALL SELECT 'BallSecurityCarry', 'Defense', 'horse_collar_tackle', 0.06
    UNION ALL SELECT 'BallSecurityCarry', 'Defense', 'body_slam', 0.03
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'unnecessary_roughness', 0.12
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'helmet_contact', 0.05
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'low_block_illegal', 0.06
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'defenseless_player_hit', 0.02
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'head_butting', 0.001
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'striking_opponent', 0.001
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'punching_opponent', 0.001
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'biting_spitting', 0.001
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'eye_gouging', 0.001
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'groin_strike', 0.001
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'kicking_opponent', 0.001
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'throat_strike', 0.001
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'spearing', 0.01
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'spearing_defenseless', 0.008
    UNION ALL SELECT 'BallSecurityCarry', 'Defense', 'piling_on', 0.04
    UNION ALL SELECT 'BallSecurityCarry', 'Defense', 'roughing_the_artrine', 0.05
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'illegal_block', 0.20
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'illegal_use_of_hands', 0.20
    UNION ALL SELECT 'BallSecurityCarry', 'Either', 'illegal_hold', 0.24
    UNION ALL SELECT 'CallToAction', 'Offense', 'false_start', 1.0
    UNION ALL SELECT 'CallToAction', 'Offense', 'illegal_motion_offense', 0.6
    UNION ALL SELECT 'CallToAction', 'Offense', 'illegal_shift_offense', 0.5
    UNION ALL SELECT 'CallToAction', 'Defense', 'encroachment', 0.7
    UNION ALL SELECT 'Drive', 'Defense', 'illegal_artro_blocking', 1.0
    UNION ALL SELECT 'ShotAttempt', 'Defense', 'kicker_interference', 1.0
    UNION ALL SELECT 'ShotAttempt', 'Defense', 'roughing_kicker', 0.3
    UNION ALL SELECT 'ShotAttempt', 'Defense', 'late_hit_kicker', 0.08
    UNION ALL SELECT 'OutOfBounds', 'Defense', 'late_hit_after_out', 0.4
    UNION ALL SELECT 'OutOfBounds', 'Defense', 'late_hit_out_of_bounds', 0.4
) AS m;
