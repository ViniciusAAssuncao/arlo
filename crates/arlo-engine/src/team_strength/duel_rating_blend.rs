pub fn blend_duel_rating(individual_rating: f64, team_rating: f64, individual_weight: f64) -> f64 {
    let w = individual_weight.clamp(0.0, 1.0);
    individual_rating * w + team_rating * (1.0 - w)
}
