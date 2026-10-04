from dataclasses import dataclass
from typing import Dict, List
from arlo_db_tool.deletion.cascade_graph import TABLE_DELETION_ORDER


@dataclass
class DeletionCountSummary:
    table_name: str
    count: int
    id_list: List[str]


@dataclass
class DeletionPreviewResult:
    target_description: str
    summaries: List[DeletionCountSummary]
    total_records: int

    @property
    def has_deletions(self) -> bool:
        return self.total_records > 0


def build_deletion_preview(
    target_description: str,
    resolved_ids: Dict[str, List[str]],
) -> DeletionPreviewResult:
    summaries: List[DeletionCountSummary] = []
    total = 0

    for table in TABLE_DELETION_ORDER:
        ids = resolved_ids.get(table, [])
        if ids:
            cnt = len(ids)
            total += cnt
            summaries.append(
                DeletionCountSummary(
                    table_name=table,
                    count=cnt,
                    id_list=ids,
                )
            )

    return DeletionPreviewResult(
        target_description=target_description,
        summaries=summaries,
        total_records=total,
    )
