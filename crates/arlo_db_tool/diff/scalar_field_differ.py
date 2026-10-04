from typing import Any, Dict, Optional, Sequence

def diff_scalar_fields(
    original: Optional[Any],
    current: Optional[Any],
    field_names: Sequence[str],
) -> Dict[str, Any]:
    changes: Dict[str, Any] = {}
    if original is None and current is None:
        return changes
    if original is None:
        for name in field_names:
            changes[name] = getattr(current, name, None)
        return changes
    if current is None:
        for name in field_names:
            changes[name] = None
        return changes

    for name in field_names:
        old_val = getattr(original, name, None)
        new_val = getattr(current, name, None)
        if old_val != new_val:
            changes[name] = new_val
    return changes