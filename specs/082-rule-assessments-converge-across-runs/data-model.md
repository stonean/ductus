# 082 — Rule assessments converge across runs Data Model

Three fields join the analysis record, whose other fields [066's data model](../066-audit-records-and-the-completion-gate/data-model.md) defines. They live in `analysis.md`'s frontmatter, beside `analyzed-digest` and `unexamined-by-reason`, and no template carries them: `write-analysis` writes the record whole.

## Record fields

| Field | Type | Meaning |
| --- | --- | --- |
| `rule-verdicts` | map of rule ID → verdict | Each loaded rule that fired, absent when none did |
| `unjudged-rules` | list of rule IDs | Each assessable loaded rule the run neither judged nor reused, absent when none |
| `rule-digest` | map of rule-file basename → sha256 | The rule files the run loaded, each over its rules' ID, Statement and Verification in heading order; absent when no rule file was loaded |

A verdict is one of two shapes:

- `addressed` — `{verdict: addressed, cite: "<path>#<Heading>"}`. `cite` names a design subject of the feature (`spec.md`, `plan.md`, `data-model.md`, `scenarios/<slug>.md`) and a heading it contains. One that does not resolve is not stored: the rule goes to `unjudged-rules`.
- `finding` — `{verdict: finding, at: "<path>#<Heading>", message: "<one line>"}`. The rule's tier is not stored; it follows from the rule text the digest covers.

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
rule-verdicts:
  BE-LOG-002: {verdict: addressed, cite: "plan.md#Observability"}
  QUAL-CLAIM-001: {verdict: finding, at: "spec.md#Acceptance Criteria", message: "AC2's record reads as complete when part was judged"}
unjudged-rules:
  - BE-AUTHN-004
  - BE-AUTHN-005
```

## Relations

- A verdict is current while the prior record's `analyzed-digest` entries for the design subjects (`spec.md`, `plan.md`, `data-model.md`, `scenarios/*.md`, never `tasks.md` or `review.md`) match the tree and its `rule-digest` entry for the rule's file matches the file. `analyzed-digest` is the record's own field, taken from the subjects the run read, so a verdict never needs a digest of its own.
- `unjudged-rules` has one entry per rule counted into `rule-assessments-not-checked` by derivation. That count in `unexamined-by-reason`, a reason-to-count map (066), also holds the rules with no tier or Verification, so it is never smaller than the list.
- A record with no `rule-digest` predates these fields and yields no reusable verdict.
- A map or list that does not parse makes the record undeterminable (§text-first-artifacts).

## `discover-rule-files` with a `feature`

With `rules` and `feature` set, the result gains `verdicts`:

```json
{
  "reason": null,
  "reusable": [
    { "id": "BE-LOG-002", "tier": "must", "verdict": "addressed", "cite": "plan.md#Observability" },
    { "id": "QUAL-CLAIM-001", "tier": "must", "verdict": "finding", "at": "spec.md#Acceptance Criteria", "message": "..." }
  ],
  "judge": ["BE-AUTHN-001", "BE-AUTHN-004"]
}
```

`reason` is `no-prior-verdicts` or `subjects-changed` when nothing is reusable, then `judge` holds every assessable rule. A rule absent from `reusable` and `judge` is current and did not fire. Both lists follow `bounded_lists::place`'s inline-or-file form.

## `write-analysis` arguments and result

- `rule-verdicts`: map of rule ID → verdict, this run's newly judged fired rules. CLI `--rule-verdicts-json FILE`.
- `judged-rules`: list of rule IDs the run judged, fired or not. CLI `--judged-rule`, repeatable.
- Result gains `contradictions`: list of `{rule, earlier}` with `earlier` one of `not-fired` or `addressed`, and the body a `## Contradictions` section.

## `assessSpecQuality` response

An optional `addressed-by` (`"<path>#<Heading>"`) beside `passed` and `finding`: with `passed: true` it makes the verdict `addressed`, and without it a `passed: true` is not fired.
