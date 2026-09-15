use crate::domain::season::StandingsEntry;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use uuid::Uuid;

pub fn seed_from_uuid(id: Uuid) -> u64 {
    let bytes = id.as_bytes();
    let mut b1 = [0u8; 8];
    let mut b2 = [0u8; 8];
    b1.copy_from_slice(&bytes[0..8]);
    b2.copy_from_slice(&bytes[8..16]);
    u64::from_le_bytes(b1) ^ u64::from_le_bytes(b2)
}

pub fn resolve_random(
    tied_group: &[StandingsEntry],
    seed: u64,
) -> Vec<StandingsEntry> {
    if tied_group.len() <= 1 {
        return tied_group.to_vec();
    }

    let mut result = tied_group.to_vec();
    let mut group_seed = seed;
    for entry in tied_group {
        let t_seed = seed_from_uuid(entry.team_id());
        group_seed = group_seed.wrapping_add(t_seed).rotate_left(13);
    }
    let mut rng = StdRng::seed_from_u64(group_seed);
    result.shuffle(&mut rng);
    result
}