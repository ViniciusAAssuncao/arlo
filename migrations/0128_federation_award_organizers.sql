ALTER TABLE award_definitions ADD COLUMN organizer_policy_override TEXT CHECK (organizer_policy_override IN ('FederationCommittee'));

ALTER TABLE award_instances ADD COLUMN organizer_federation_id TEXT REFERENCES federations(id);
