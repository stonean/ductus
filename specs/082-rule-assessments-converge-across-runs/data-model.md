# 082 — Rule assessments converge across runs Data Model

Four fields join the analysis record, whose other fields [066's data model](../066-audit-records-and-the-completion-gate/data-model.md) defines. They live in `analysis.md`'s frontmatter, beside `analyzed-digest` and `unexamined-by-reason`, and no template carries them: `write-analysis` writes the record whole.

## Record fields

| Field | Type | Meaning |
| --- | --- | --- |
| `rule-verdicts` | map of rule ID → verdict | Each loaded rule that fired, absent when none did |
| `unjudged-rules` | list of rule IDs | Each assessable loaded rule the run neither judged nor reused, absent when none |
| `rule-digest` | map of rule-file basename → sha256 | The rule files the run loaded, each over its rules' ID, Statement and Verification in heading order; absent when no rule file was loaded |
| `section-digest` | map of section key → 16 hex | Each section of the design subjects, its digest the first 16 hex characters of SHA-256 over its text (AC7); absent when no design subject exists |

A verdict is one of three shapes:

- `addressed` — `{verdict: addressed, cite: "<path>#<Heading>"}`. `cite` names a design subject of the feature (`spec.md`, `plan.md`, `data-model.md`, `scenarios/<slug>.md`) and a heading it contains. One that does not resolve is not stored: the rule goes to `unjudged-rules`.
- `finding` — `{verdict: finding, tier: must|should, at: "<path>#<Heading>", message: "<one line>"}`. `tier` is the tier the finding was counted in, its own severity, which is not always its rule's text tier (`write-analysis` refuses one that differs unless the Statement carries both MUST and SHOULD); a run that reuses the verdict reports and floors it there.
- `waived` — `{verdict: waived}`, which a host supplies for a fired rule the spec departs from, and which `write-analysis` completes with `reason`, `decided-by`, `decided-at` and `cited` from the spec's departure entry on every write. It is no finding and counts into no tier. Spec 073 adds this shape ([073 — Decided rule departures are recorded](../073-decided-rule-departures-are-recorded/spec.md)); its data model owns the fields, `check-departures` the entries, and the compiled description of `write-analysis` the refusals and the carry-forward.

A loaded rule in neither `rule-verdicts` nor `unjudged-rules` did not fire. A rule with no Verification or no RFC 2119 keyword is in none of the fields: it stays counted under `rule-assessments-not-checked` only.

```yaml
analyzed-digest:
  plan.md: 3f1c…
  spec.md: a90e…
unexamined: 4
unexamined-by-reason:
  rule-assessments-not-checked: 4
rule-digest:
  quality-cross.md: 51ab…
  security-backend.md: c07d…
section-digest:
  "plan.md": 0b6e…
  "plan.md#Observability": 9d21…
  "spec.md": 77c4…
  "spec.md#Acceptance Criteria": e310…
rule-verdicts:
  BE-LOG-002: {verdict: addressed, cite: "plan.md#Observability"}
  QUAL-CLAIM-001: {verdict: finding, tier: must, at: "spec.md#Acceptance Criteria", message: "AC2's record reads as complete when part was judged"}
unjudged-rules:
  - BE-AUTHN-004
  - BE-AUTHN-005
```

## Relations

- A section key is `<path>#<Heading>`, the heading's own text from its line to the next heading of any level, or `<path>` alone for the text before the file's first heading, frontmatter included. Headings of one file that read alike share a key, digested together in order, and `spec.md` is digested with its criteria's ticks normalized.
- A verdict is current while its rule's own text is the text the run judged and the sections it concerns are unchanged. Its rule's text is so when its `rule-digest` entry matches the file or, since [088's data model](../088-rule-judgments-cost-only-what-changed/data-model.md), when the file's copy at the record's `analyzed-against`, digesting as that entry says, holds the rule with the same Statement and Verification, and no rule its Statement or Verification names changed. Its sections are: with the design subjects (`spec.md`, `plan.md`, `data-model.md`, `scenarios/*.md`, never `tasks.md` or `review.md`) as the prior record's `analyzed-digest` says, every verdict; otherwise, by `section-digest`, a not-fired, `addressed` or `waived` one for every section but the changed ones (added, removed or digesting differently), against which its rule is judged again, an `addressed` one only while its `cite`'s section is unchanged, and a `finding` while its `at`'s section is, its rule judged in full once that changes. A record without `section-digest` falls back to the whole files. A verdict never needs a digest of its own.
- `unjudged-rules` has one entry per rule counted into `rule-assessments-not-checked` by derivation. That count in `unexamined-by-reason`, a reason-to-count map (066), also holds the rules with no tier or Verification, so it is never smaller than the list.
- A record with no `rule-digest` predates these fields and yields no reusable verdict.
- A `finding` passed for a rule whose earlier verdict was not fired or `addressed`, at a section unchanged since, is no verdict: `write-analysis` stores it nowhere but `contradictions`, and no tier counts it.
- A `waived` verdict is carried forward like another current one, but its attribution is read again from the spec's history on every write, and one whose departure entry is gone is recorded among `unjudged-rules` ([073](../073-decided-rule-departures-are-recorded/spec.md)).
- A map or list that does not parse makes the record undeterminable (§text-first-artifacts).

## `discover-rule-files` with a `feature`

With `all`, `rules` and `feature` set, the result gains `verdicts`:

```json
{
  "reusable": [
    { "id": "BE-LOG-002", "verdict": "addressed", "cite": "plan.md#Observability" },
    { "id": "QUAL-CLAIM-001", "verdict": "finding", "tier": "must", "at": "spec.md#Acceptance Criteria", "message": "..." }
  ],
  "judge": ["BE-AUTHN-001"],
  "rejudge": ["BE-AUTHN-004", "BE-LOG-002"],
  "changed-sections": ["plan.md#Rollout"]
}
```

`reason` is absent while the prior record's verdicts can be reused, even when none fired; otherwise it is `no-prior-verdicts`, `subjects-changed` (a design subject changed and the record has no `section-digest`, or one cannot be read), `rule-files-changed` (the rule files loaded now are not the set the record's `rule-digest` names) or `record-unreadable` (also a notice), and `judge` holds every assessable rule. `judge` names the rules to judge in full and `rejudge` those to judge against `changed-sections` alone, each changed section's key in key order, the added and removed among them; a `reusable` `addressed` or `waived` verdict of a rule in `rejudge` stands unless that judgment replaces it. `reusable-count`, `judge-count`, `rejudge-count` and `changed-sections-count` are inline in both forms. A rule of a readable file absent from `reusable`, `judge` and `rejudge` is current and did not fire; the rules of a file that cannot be read are in neither, and the file is named under the result's `unreadable`. `reusable` can also carry a `waived` verdict with its attribution ([073 — Decided rule departures are recorded](../073-decided-rule-departures-are-recorded/spec.md)). Both lists follow `bounded_lists::place`'s inline-or-file form. Since spec 088 a changed rule file's rules are compared one by one, and `verdicts` names under `rejudged-files` each changed file whose copy could not stand in, every rule of which is in `judge`; [088's data model](../088-rule-judgments-cost-only-what-changed/data-model.md) holds its shape and reasons.

## `write-analysis` arguments and result

- `rule-verdicts`: map of rule ID → verdict, this run's newly judged fired rules. CLI `--rule-verdicts-json FILE`.
- `judged-rules`: list of rule IDs the run judged, fired or not. CLI `--judged-rule`, repeatable.
- A bare `unexamined` total, passed with no `unexamined-by-reason`, is recorded under the reason `unclassified` beside the reasons `write-analysis` derives, so `unexamined` stays the breakdown's sum (066).
- Result gains `contradictions`: list of `{rule, earlier, at, message, counted}` with `earlier` one of `not-fired` or `addressed` and `counted` false for a finding at an unchanged section, which is recorded nowhere else (AC7), true for one at a location naming no section, which also counts (AC3), bounded as `bounded_lists::place` bounds a list: `form` and `contradictions-count` are inline in both forms, and past the read size the list is absent and `lists-file` names the file holding it. The body gains a `## Contradictions` section. Spec 073 adds `waived` (each waived rule with its reason, attribution and citation) and `waived-count`, bounded the same way, and a `## Waived rules` section ([073 — Decided rule departures are recorded](../073-decided-rule-departures-are-recorded/spec.md)).

## `assessSpecQuality` response

An optional `addressed-by` (`"<path>#<Heading>"`) beside `passed` and `finding`: with `passed: true` it makes the verdict `addressed`, and without it a `passed: true` is not fired. What the walk records for a rule the spec departs from is stated with the response in [022's data model](../022-deterministic-runtime/data-model.md) (spec 073).
