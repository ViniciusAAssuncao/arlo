# Technical debt

## Human-controlled matches

Human manager decisions remain deferred indefinitely. The match runner exposes a partial decision inbox, but normal day advancement completes each fixture in one batch and does not persist a resumable match state. Human challenges, tactical switches, lineup control, and frontend interactions must be connected only when a full pause, resume, and persistence flow is designed. Automated manager decisions continue independently.

## Injury withdrawal decisions

Substitutions have no per-match limit. Human-controlled managers receive a keep-or-withdraw decision at the next Out for injuries that permit continued play. An incapacitating injury, including a complete ACL rupture, removes the player immediately; a human manager chooses the reserve during that injury interruption when one is available. Automated managers choose whether to withdraw an athlete with a non-incapacitating injury at the next Out using diagnosis-specific recovery, athlete condition, reserve suitability, match context, and manager attributes. The medical model must distinguish partial from complete ACL rupture and record the chosen treatment path; a reconstruction case cannot inherit only the generic grade-and-body-region recovery duration.

The initial treatment choice is statistical because the game has no clinical assessment or medical staff decision yet. Revisit treatment choice, return-to-play criteria, and injury-specific recovery profiles when medical staff and manager controls are added. The outside-match catalog separates common illness onset from match actions. Diagnoses without a plausible trigger in the current match or daily model were removed from the active catalog. Historical injury records may retain inert definitions so past injuries remain readable.

Every current injury definition has a recovery profile, and outside-match conditions have a profile for their configured grade. Catalog loading rejects future definitions without one. Recovery for chronic diagnoses represents the current episode's return to play, not a cure of the underlying condition. Catastrophic diagnoses remain very rare emergencies with long absences; the game does not simulate death or permanent disability.

## Day advancement consistency

Day advancement now keeps the calendar on the previous date until recovery, scheduled events, and matches finish. Recovery and matchday persistence advance their durable checkpoints in the same transactions as their effects. Scheduled-event handlers still perform several independent writes and may add in-memory triggers. If interrupted while executing those handlers, the day remains in EventsRunning and automatic replay stops to avoid duplicate or skipped effects. Make each scheduled event transactional and replayable before enabling automatic recovery from that phase.

## Manager challenges and peace-referee review

When human manager challenges and manager AI are implemented, a challenge must require the peace referee to reveal the factual outcome of the disputed play. The referee decision event already records the factual incident, the head referee's original call, and the spontaneous peace-referee intervention. Challenge handling must reuse that decision without resampling the incident.

## Catalog-driven officiating

Fault definitions and punishment options belong to the database catalog. An incident may receive multiple permitted punishments. Loss of down, Drive, or territory applies to the offending team's own preserved state when it is defending. The legacy foul row records the first punishment for compatibility; `match_foul_punishments` records every selected punishment. Readers needing the complete set must use the new table.

## Faults awaiting their own match actions

Confirmed scope: excess official designations of Artrine, Passer, Goalguard, and bounded tactical roles are reviewed as `illegal_substitution` for both AI and human lineups. Everyone designated to the exceeded position or role is expelled. If no designated Artrine remains, an active teammate handles the mandatory opening reception without Artro or Drive eligibility; Field Points and Kick Fouls remain possible.

Do not generate these faults from unrelated duels or random match time. The remaining inactive definitions for `referee_verbal_abuse`, `dissent`, `excessive_celebration`, `minor_unsportsmanlike`, `taunting`, and `unsportsmanlike_conduct` can gain their own behavior events tied to match context and player attributes. Excess official designations are already reviewed as `illegal_substitution` at the initial lineup review; future in-match role reassignment must undergo the same review. Faults that require unmodeled equipment, procedural violations, physical incidents away from play, or unsupported play states were removed from the active catalog. Historical foul records may retain inert definitions so past decisions remain readable.

`deliberate_injury_attempt` and `weapon_use` are removed from the catalog. A designated FalseArtrine can commit `false_artrine_fraud` on reception by attempting to claim an Artro, or `working_communicator_non_artrine` during a Call-to-Action by misusing the communicator. The actual official Artrine is never eligible for either fault. Confirmed infractions always invalidate the full continuous play; expulsion is optional. `InvalidatePreviousPlay` restores the sporting state and derived statistics while match time and physical injuries remain.

Goalguard recoveries now record zone and hands-or-feet use; `illegal_goalguard_handling` is possible only when hands are used outside the First Zone. Late Passer contact now records the defender and contact severity after the opening pass is released. `passer_contact_late`, `roughing_the_passer`, and `roughing_passer_late` are adjudicated from that action rather than unrelated route duels.
