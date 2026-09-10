use crate::instructions::transition::press_block_shape::PressBlockShape;

pub fn nearest_block_notation(shape: PressBlockShape, eligible_defenders: u32) -> (u32, u32) {
    let raw_bite = (shape.bite_ratio() * eligible_defenders as f64).round() as i64;
    let bite_count = raw_bite.clamp(0, eligible_defenders as i64) as u32;
    let cover_count = eligible_defenders - bite_count;
    (bite_count, cover_count)
}
