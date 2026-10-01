pub fn encode_match_seed(seed: u64) -> i64 {
    i64::from_ne_bytes(seed.to_ne_bytes())
}

pub fn decode_match_seed(value: i64) -> u64 {
    u64::from_ne_bytes(value.to_ne_bytes())
}
