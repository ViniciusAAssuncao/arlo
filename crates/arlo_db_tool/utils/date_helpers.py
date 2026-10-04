from datetime import datetime, timezone
from typing import List, Tuple

DEFAULT_MONTHS: List[Tuple[int, str]] = [
    (1, "01 - Janeiro"),
    (2, "02 - Fevereiro"),
    (3, "03 - Março"),
    (4, "04 - Abril"),
    (5, "05 - Maio"),
    (6, "06 - Junho"),
    (7, "07 - Julho"),
    (8, "08 - Agosto"),
    (9, "09 - Setembro"),
    (10, "10 - Outubro"),
    (11, "11 - Novembro"),
    (12, "12 - Dezembro"),
]

def date_to_unix_seconds(year: int, month: int = 1, day: int = 1) -> int:
    y = max(1, min(int(year), 9999))
    m = max(1, min(int(month), 12))
    d = max(1, min(int(day), 31))

    while d > 28:
        try:
            dt = datetime(y, m, d, 0, 0, 0, tzinfo=timezone.utc)
            return int(dt.timestamp())
        except ValueError:
            d -= 1

    dt = datetime(y, m, d, 0, 0, 0, tzinfo=timezone.utc)
    return int(dt.timestamp())

def unix_seconds_to_date(unix_seconds: int) -> Tuple[int, int, int]:
    try:
        dt = datetime.fromtimestamp(int(unix_seconds), tz=timezone.utc)
        return dt.year, dt.month, dt.day
    except Exception:
        return 2000, 1, 1