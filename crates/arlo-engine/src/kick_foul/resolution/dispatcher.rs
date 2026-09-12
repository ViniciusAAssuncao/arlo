use crate::error::EngineResult;
use crate::kick_foul::decision::{evaluate_kick_foul_decision_utilities, sample_kick_foul_decision};
use crate::kick_foul::pending::KickFoulPending;
use crate::kick_foul::resolution::block_phase::resolve_kick_block_duel;
use crate::kick_foul::resolution::outcome::KickFoulResolutionOutcome;
use crate::kick_foul::resolution::participants::select_kick_foul_participants;
use crate::kick_foul::resolution::restart_phase::resolve_kick_foul_restart;
use crate::kick_foul::resolution::shoot_phase::resolve_kick_foul_shot;
use crate::match_decision::scoring::ScoringDecision;
use crate::resolution::{AttributedDuelOutcome, DuelContext};
use crate::world_state::context_analyzer::analyze_match_state;
use crate::world_state::match_state::MatchState;
use arlo_domain::{KickFoulDecisionKind, Player};
use rand::Rng;
use smallvec::{smallvec, SmallVec};

pub fn resolve_kick_foul<R: Rng + ?Sized>(
    state: &MatchState,
    pending: &KickFoulPending,
    rng: &mut R,
) -> EngineResult<KickFoulResolutionOutcome> {
    let offense_team_id = pending.awarded_team_id();
    let defense_team_id = if offense_team_id == state.home_team_id() {
        state.away_team_id()
    } else {
        state.home_team_id()
    };

    let is_home_offense = offense_team_id == state.home_team_id();
    let offense_lineup = state.lineup_for_team_arc(offense_team_id);
    let defense_lineup = state.lineup_for_team_arc(defense_team_id);
    let unavailable = state.unavailable_player_ids();

    let offense_players: Vec<&Player> = offense_lineup
        .assignments()
        .iter()
        .map(|a| a.player())
        .filter(|p| !unavailable.contains(&p.id()))
        .collect();
    let defense_players: Vec<&Player> = defense_lineup
        .assignments()
        .iter()
        .map(|a| a.player())
        .filter(|p| !unavailable.contains(&p.id()))
        .collect();

    let offense_role_index = state.role_index_for_team(offense_team_id);
    let tables = state.teams.player_attribute_tables();
    let attribute_keys = state.attribute_keys();
    let pitch = state.pitch();

    let participants = select_kick_foul_participants(
        &offense_players,
        &defense_players,
        offense_role_index,
        tables,
        rng,
    )?;

    let taker_id = participants.kicker.id();
    let kicker_table = state.attribute_table_for(&taker_id);

    let pressure = analyze_match_state(state);
    let utilities =
        evaluate_kick_foul_decision_utilities(kicker_table, pending.scoring_tier(), &pressure);
    let decision = sample_kick_foul_decision(kicker_table, &utilities, rng);

    let offense_instructions = state.instructions_for_team(offense_team_id);
    let defense_instructions = state.instructions_for_team(defense_team_id);
    let physicality_offset = crate::team_identity::physicality::offensive_contact_logit_offset(
        offense_instructions.in_possession().physicality(),
    );
    let aggression_offset = crate::team_identity::aggression::duel_logit_offset(
        defense_instructions.out_of_possession().aggression(),
    );
    let duel_context = DuelContext::with_offsets(
        is_home_offense,
        !is_home_offense,
        aggression_offset,
        0.0,
        physicality_offset,
    );

    match decision {
        KickFoulDecisionKind::Shoot => {
            let block_duel = resolve_kick_block_duel(
                &participants,
                tables,
                attribute_keys,
                &duel_context,
                rng,
            );
            let mut duels: SmallVec<[AttributedDuelOutcome; 2]> = smallvec![block_duel.clone()];

            if !block_duel.outcome().attacker_won() {
                Ok(KickFoulResolutionOutcome::new(
                    ScoringDecision::NoOpportunity,
                    None,
                    duels.into_vec(),
                    true,
                    decision,
                    taker_id,
                ))
            } else {
                let (scoring_decision, shot_duel) = resolve_kick_foul_shot(
                    participants.kicker,
                    participants.goalguard,
                    pending.scoring_tier(),
                    offense_team_id,
                    attribute_keys,
                    tables,
                    &duel_context,
                    rng,
                );
                duels.push(shot_duel);

                Ok(KickFoulResolutionOutcome::new(
                    scoring_decision,
                    None,
                    duels.into_vec(),
                    false,
                    decision,
                    taker_id,
                ))
            }
        }
        KickFoulDecisionKind::Cross
        | KickFoulDecisionKind::ShortPass
        | KickFoulDecisionKind::LongLaunch => {
            let kicker_pos = pending.spot();
            let restart = resolve_kick_foul_restart(
                participants.kicker,
                kicker_pos,
                &participants.target_candidates,
                &defense_players,
                decision,
                tables,
                state.spatial_map(),
                pitch,
                attribute_keys,
                &duel_context,
                rng,
            );
            let duels: SmallVec<[AttributedDuelOutcome; 2]> =
                restart.duels.iter().cloned().collect();

            Ok(KickFoulResolutionOutcome::new(
                ScoringDecision::NoOpportunity,
                Some(restart),
                duels.into_vec(),
                false,
                decision,
                taker_id,
            ))
        }
    }
}