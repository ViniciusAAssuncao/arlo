import hashlib
import json
import pathlib
import sqlite3


ROOT = pathlib.Path(__file__).resolve().parents[2]
ARTIFACTS = ROOT / "target" / "performance"


def owned_database(path):
    path = pathlib.Path(path).resolve()
    if not path.is_relative_to(ARTIFACTS.resolve()) or path.suffix != ".db":
        raise ValueError("Only owned databases under target/performance are writable")
    return path


def read_database(path):
    path = pathlib.Path(path).resolve()
    wal = pathlib.Path(str(path) + "-wal")
    if wal.exists() and wal.stat().st_size:
        raise ValueError(f"Close the writer and checkpoint before reading {path}")
    connection = sqlite3.connect(path.as_uri() + "?mode=ro&immutable=1", uri=True)
    connection.execute("PRAGMA cache_size=-64000")
    connection.execute("PRAGMA temp_store=MEMORY")
    connection.execute("PRAGMA mmap_size=268435456")
    return connection


def snapshot(source, destination):
    destination = owned_database(destination)
    if destination.exists():
        raise ValueError("Snapshot destination must not exist")
    destination.parent.mkdir(parents=True, exist_ok=True)
    source = pathlib.Path(source).resolve()
    with read_database(source) as original, sqlite3.connect(destination) as target:
        original.backup(target)
    return {
        "source": str(source),
        "destination": str(destination),
        "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
        "bytes": source.stat().st_size,
    }


def normalize_migrations(database, archive):
    archive = pathlib.Path(archive).resolve()
    if not archive.is_relative_to(ARTIFACTS.resolve()):
        raise ValueError("Expected a historical archive under target/performance")
    changed = []
    with sqlite3.connect(owned_database(database)) as connection:
        for directory in (archive / "migrations", archive / "crates/arlo-controller/migrations"):
            for migration in sorted(directory.glob("*.sql")):
                version = int(migration.name.split("_")[0])
                checksum = hashlib.sha384(migration.read_bytes()).digest()
                connection.execute(
                    "UPDATE _sqlx_migrations SET checksum=? WHERE version=?",
                    (checksum, version),
                )
                changed.append(version)
    return {"normalized_versions": changed}


def compare(left, right):
    if isinstance(left, dict) and isinstance(right, dict):
        return left.keys() == right.keys() and all(compare(v, right[k]) for k, v in left.items())
    if isinstance(left, (list, tuple)) and isinstance(right, (list, tuple)):
        return len(left) == len(right) and all(compare(a, b) for a, b in zip(left, right))
    if isinstance(left, float) or isinstance(right, float):
        return isinstance(left, (float, int)) and isinstance(right, (float, int)) and abs(left - right) <= 1e-10
    return left == right


def canonical_capture(value):
    identities = {value["match_id"]: "match"}
    for side in ("home", "away"):
        team = value[side]
        identities[team["lineup"]["id"]] = side + "-lineup"
        for index, profile in enumerate(team["profiles"]):
            identities[profile["id"]] = f"{side}-profile-{index}"
        for index, plan in enumerate(team.get("plans", [])):
            identities.setdefault(plan["id"], f"{side}-plan-{index}")
            identities.setdefault(plan["layout"]["lineup"]["id"], f"{side}-layout-{index}")
    for index, call in enumerate(value["calls"]):
        identities[call["id"]] = f"call-{index}"

    def walk(item):
        if isinstance(item, dict):
            return {key: walk(child) for key, child in item.items()}
        if isinstance(item, list):
            return [walk(child) for child in item]
        if isinstance(item, str):
            return identities.get(item, item)
        return item

    return walk(value)


def compare_captures(before, after):
    count = 0
    for path in sorted(pathlib.Path(before).glob("*.input.json")):
        other = pathlib.Path(after) / path.name
        left = canonical_capture(json.loads(path.read_text(encoding="utf-8")))
        right = canonical_capture(json.loads(other.read_text(encoding="utf-8")))
        if not compare(left, right):
            raise ValueError(f"Preparation diverged: {path.name}")
        count += 1
    if not count:
        raise ValueError("No captured fixtures")
    return {"equal_preparations": count}
