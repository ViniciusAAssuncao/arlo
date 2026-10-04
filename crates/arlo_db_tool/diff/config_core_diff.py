from dataclasses import dataclass, field
from typing import Any, Dict, Optional
from arlo_db_tool.domain.config_core_fields import CONFIG_CORE_FIELDS
from arlo_db_tool.domain.league_calendar_config_draft import LeagueCalendarConfigDraft
from arlo_db_tool.diff.scalar_field_differ import diff_scalar_fields

@dataclass
class ConfigCoreDiffResult:
    changed_fields: Dict[str, Any] = field(default_factory=dict)

    @property
    def has_changes(self) -> bool:
        return bool(self.changed_fields)

def diff_config_core(
    original: Optional[LeagueCalendarConfigDraft],
    current: Optional[LeagueCalendarConfigDraft],
) -> ConfigCoreDiffResult:
    field_names = [f.name for f in CONFIG_CORE_FIELDS]
    changed = diff_scalar_fields(original, current, field_names)
    return ConfigCoreDiffResult(changed_fields=changed)