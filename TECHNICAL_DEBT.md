# Technical debt

## Injury withdrawal decisions

When manager AI and human manager injury controls are implemented, an injury that permits continued play must present a keep-or-withdraw decision. An incapacitating injury, including a complete ACL rupture, removes the player immediately. The medical model must distinguish partial from complete ACL rupture and record the chosen treatment path; a reconstruction case cannot inherit only the generic grade-and-body-region recovery duration.

The initial treatment choice is statistical because the game has no clinical assessment or medical staff decision yet. Revisit treatment choice, return-to-play criteria, and injury-specific recovery profiles when medical staff and manager controls are added. Review which catalog diagnoses can arise from a match action after the injury system has matured; the first event generator currently samples the catalog by contact or non-contact mechanism.

## Manager challenges and peace-referee review

When human manager challenges and manager AI are implemented, a challenge must require the peace referee to reveal the factual outcome of the disputed play. The referee decision event already records the factual incident, the head referee's original call, and the spontaneous peace-referee intervention. Challenge handling must reuse that decision without resampling the incident.

## Catalog-driven officiating

Fault definitions and punishment options belong to the database catalog. An incident may receive multiple permitted punishments. Loss of down, Drive, or territory applies to the offending team's own preserved state when it is defending. The legacy foul row records the first punishment for compatibility; `match_foul_punishments` records every selected punishment. Readers needing the complete set must use the new table.

## Faults awaiting their own match actions

Do not generate these faults from unrelated duels or random match time. Introduce their triggering action and evidence before making them eligible for officiating:

- Substitution protocol: `illegal_substitution`, `wrong_substitution_protocol`.
- Equipment and inspection: `equipment_violation`, `illegal_equipment_check`.
- Injury or weapon evidence: `deliberate_injury_attempt`, `weapon_use`.
- Manager and restart timing: `delay_of_game`, `excessive_time_call_delay`, `play_call_delay`, `illegal_countdown_execution`.
- Lineup and communication fraud: `illegal_formation`, `illegal_formation_numbering`, `illegal_marking_scheme`, `false_artrine_fraud`, `working_communicator_non_artrine`.
- Artro or Drive declaration fraud: `false_drive_declaration`, `illegal_drive_registration`.
- Deliberate behavior and incidents away from the current duel: `feigning_injury`, `fan_aggression`, `referee_contact`, `referee_verbal_abuse`, `dissent`, `excessive_celebration`, `minor_unsportsmanlike`, `taunting`, `unsportsmanlike_conduct`, `sideline_encroachment`.
- Post-whistle contact without an Out or shot attempt: `late_hit`.
- First-zone goalkeeper handling: `illegal_goalguard_handling` needs a handling action.
- Pass and specialist procedural fouls: `illegal_call_to_action_pass`, `passer_contact_late`, `roughing_passer_late` require corresponding contested actions.

The four catalogue definitions that permit `InvalidatePreviousPlay` remain outside activation until those actions exist. Their effect requires full rollback of the play and every derived statistic, as confirmed by the owner.
