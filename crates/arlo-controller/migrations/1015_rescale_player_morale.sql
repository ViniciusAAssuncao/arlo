UPDATE player_condition
SET impulse_current_value = MIN(120, MAX(0, impulse_current_value * 2)),
    impulse_baseline = 100.0;
