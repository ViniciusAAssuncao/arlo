# Attendance model proposal

## Confirmed inputs

- `teams.min_attendance` and `teams.max_attendance` are home attendance references, not hard bounds.
- Stadium capacity is a physical bound.
- Fictional weekdays have a social role. Matches have no kickoff time.

## Model boundary

Calculate attendance once, before a match starts, from information available at that time. Preserve the realized value for historical and financial use. Do not recompute old matches when team form, prestige, or calibration changes.

The model has three distinct outputs: match appeal, total attendance, and the number of attendees identified with each team. Spectators with no declared team are a third group. Home advantage in access to tickets is distinct from team popularity.

## Team reference demand

Treat the two team fields as anchors for ordinary home fixtures at the team's home venue. A provisional interpretation is the 15th and 95th percentiles of ordinary-match attendance. That interpretation gives a greater chance of attendance below `min_attendance` than above `max_attendance`, while still allowing both. It requires owner confirmation before calibration.

Use an occupancy-scale model so capacity is respected. For an ordinary home match with capacity `C`, transform the two anchors into occupancy fractions, clamp only for numerical stability, and fit a baseline location and spread on the logit scale. Add effects for form, match appeal, calendar role, and a small asymmetric residual. Apply the inverse logit and cap at `C`. This keeps the computational cost constant per match.

Team popularity changes slowly and should reflect prestige, historical titles, and longer-run competitive strength. Form is a separate short-run signal, smoothed across recent matches and the current season. A bad run should reduce demand without erasing a major club's core support.

## Match appeal

Derive a latent appeal score from stage stakes, competition prestige and scope, rivalry when represented, opponent popularity, and the consequences of the result for standings or qualification. Squad absences may change the score modestly. Outcome balance can contribute but should not dominate: published football studies disagree on whether close contests raise stadium attendance. Use only pre-match data.

## Calendar role without kickoff time

Classify each fictional weekday as `workday` or `rest_day`. Derive first workday after rest and workday before rest from adjacent days in the calendar cycle. Assign an average attendance effect for each context. Because there is no kickoff time, these effects represent averages over plausible match times and should be weaker than a model that knows the hour. Unclassified legacy calendars have a neutral calendar effect until configured.

## Venue and audience composition

Determine venue context from the actual venue, its owner and country, and the teams' home venues and countries. Do not rely solely on a fixture's neutral flag. At a team's usual home stadium, separate demand from access: a popular visitor may have high latent demand but limited places. Derive the visitor allocation from match and venue context, with a structural safeguard against visitors taking a home-majority crowd. This allocation is a model output, not a universal fixed percentage.

At an effectively neutral venue, estimate local demand for both teams plus unaffiliated spectators. Split the available seats using relative local popularity and event appeal. Team prestige alone is a provisional proxy for international reach; country-level location is the finest geography currently available. Thus, exact overseas fan splits should remain uncertain until richer market data exist.

## Data and calibration

Keep coefficients, noise distribution, and model version explicit. Premier League occupancy is useful for checking elite, capacity-constrained clubs, not as a universal target for every Arlo team. Calibrate wider-league variation from match-level attendance and capacity data if available. Keep the model output deterministic for a fixed match seed and version.

Current schema additions expose the team reference values and weekday social roles. The calculation, persistence of realized attendance, and detailed supporter split remain to be implemented after the model choices are reviewed.

## External references

- [Premier League 2024/25 season in numbers](https://www.premierleague.com/en/news/4316617): reported 98.8% stadium utilization.
- [Cox, Spectator Demand, Uncertainty of Results, and Public Interest](https://journals.sagepub.com/doi/10.1177/1527002515619655): English Premier League evidence on outcome uncertainty and stadium demand.
- [Forrest and Simmons, Outcome uncertainty and attendance demand in sport](https://rss.onlinelibrary.wiley.com/doi/abs/10.1111/1467-9884.00314): English football attendance demand.
- [UEFA Safety and Security Regulations, Article 19](https://documents.uefa.com/r/UPE0QDp~FJso7vSx8slqLQ/FkF0azjdXsQD0LjTXUqHyQ): example of venue access rules for away supporters.
