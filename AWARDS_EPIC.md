# Awards epic

## Confirmed rules

- Match MVP uses deterministic probabilistic selection. The award catalog supplies its criteria, normalization scopes, and selection temperature.
- Match MVP belongs to the committee of the match's league. The committee may have no official name. A match edition records the league when its competition is a league.
- Awards and competition titles remain separate concepts.
- An award definition specifies eligibility, an evaluation window, a trigger, criteria, nomination and selection policies. An edition persists its definition snapshot, seed, model version, nominees, vote tallies, and winner.

## Implemented foundation

- `arlo-domain` owns award types and policies.
- `arlo-catalog` loads active definitions, criteria, restrictions, voter groups, ballot points, and organization metadata from the database.
- `arlo-awards` resolves eligibility, nominations, normalized utility, seeded selection, and synthetic ranked ballots without SQL or clock access.
- `arlo-persistence` stores editions atomically and idempotently and exposes match winners and subject history.
- `arlo-controller` resolves and persists Match MVP in both completed-match paths within the match transaction. The performance endpoint reads the saved winner.
- `arlo-controller` also exposes a generic award resolution entry point for callers that have assembled evidence for other award types.

## Future iterations

- Add evidence builders and lifecycle scheduling for season, interval, and calendar awards. The current Match MVP integration uses match performance evidence.
- Add award race projections and historical snapshots after the award lifecycle is available.
- Resolve ownership of match awards for competitions that are not leagues before assigning an organizer to those editions.
- Revisit provisional Match MVP criterion weights and temperature after historical calibration.
