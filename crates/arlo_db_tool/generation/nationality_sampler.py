import random
from typing import Optional, Sequence
import uuid


def sample_nationality(
    available_country_ids: Sequence[str],
    preferred_country_id: Optional[str] = None,
    preferred_bias: float = 0.70,
    rng: Optional[random.Random] = None,
) -> str:
    r = rng if rng is not None else random.Random()

    if not available_country_ids:
        if preferred_country_id and str(preferred_country_id).strip():
            return str(preferred_country_id).strip()
        return str(uuid.uuid4())

    valid_country_ids = [str(cid).strip() for cid in available_country_ids if str(cid).strip()]
    if not valid_country_ids:
        if preferred_country_id and str(preferred_country_id).strip():
            return str(preferred_country_id).strip()
        return str(uuid.uuid4())

    if preferred_country_id and str(preferred_country_id).strip():
        pref_id = str(preferred_country_id).strip()
        if r.random() < preferred_bias:
            return pref_id

        other_country_ids = [cid for cid in valid_country_ids if cid != pref_id]
        if other_country_ids:
            return r.choice(other_country_ids)
        return pref_id

    return r.choice(valid_country_ids)
