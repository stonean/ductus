# 073 — Decided Rule Departures Are Recorded Data Model

The structures this feature adds. The `rule-verdicts` map's own shape is 082's ([data-model](../082-rule-assessments-converge-across-runs/data-model.md)) and the analysis record's fields are 066's; this file owns the departure entry, the `waived` verdict's fields, `check-departures`' result and the review waiver's marker. Once implemented, each primitive's compiled description (`ductus <name> --help`) is the authority for its behavior.

## Departure entry

A list item under a spec's `## Applicable Rules` heading, outside fenced code and HTML comments:

```markdown
- departs: `RULE-ID`
  - reason: <non-empty text>
  - cited: §<anchor>
```

| Part | Required | Meaning |
| --- | --- | --- |
| `departs:` and one backticked rule ID | yes | The rule the spec deliberately does not satisfy, of any tier |
| `reason:` | yes | Why, in the spec author's words |
| `cited:` | no | One `§anchor` reference to the governing document that decided the departure in advance |

Any other child key, a second rule ID, or an empty `reason:` makes the entry malformed. A malformed entry waives nothing.

## Attribution

| Field | Value |
| --- | --- |
| `decided-by` | The author email of the oldest commit in the unbroken run, ending at `HEAD`, in which `spec.md` holds an entry for the rule; `unattributed` when the repository is shallow or absent, the entry is not in `HEAD`'s `spec.md`, or a revision cannot be read |
| `decided-at` | That commit's author time, RFC 3339 UTC to the second with `Z`; absent when `unattributed` |

## `waived` rule verdict

The third value of 082's rule-verdict map, beside `addressed` and `finding`:

```yaml
rule-verdicts:
  BE-COMPAT-001:
    verdict: waived
    reason: the credential field is removed rather than versioned; clients were told in advance
    decided-by: author@example.com
    decided-at: 2026-11-12T18:04:11Z
    cited: §shared-compatibility-exceptions
```

`decided-at` is absent when `decided-by` is `unattributed`, and `cited` when the entry has none. The host supplies only `verdict: waived`; `write-analysis` fills the rest from the entry on every write.

## `check-departures` result

| Field | Type | Meaning |
| --- | --- | --- |
| `entries` | list of `{rule, tier, reason, cited, decided-by, decided-at, line}` | Every well-formed entry; `tier` is the rule's, absent when the rule is not loaded |
| `findings` | list of `{family, tier, message, path}` | Family `departure-entries`: a missing `reason`, an unknown or retired rule (with no rule file loaded, only a token shaped like no rule ID, which is unknown), a rule two entries name (Blocking); a deprecated rule, a `cited:` that is not exactly one `§` reference (Advisory); an unresolved reference is step 4's |
| `unexamined` | list of `{reason, line}` | A citation that could not be judged (`anchor-target-unjudgeable`, `anchor-target-unreadable`), a rule not installed (`rule-not-installed`), or, with no rule file loaded, every entry's rule (`rule-citations-not-checked`, with no unknown, retired or deprecated finding); never a pass. Each repeats a target step 4 or step 5 records, so a caller counts it once; a token that is no rule-ID shape is still an unknown-rule finding, as step 5 records no citation for it |

## Review waiver marker

A waiver `write-review` derives from an entry is an ordinary `waivers:` entry (`framework/reference/review.md` §Waivers) with one open-schema field:

| Field | Value |
| --- | --- |
| `departure` | `true`; `process-waivers` expires the waiver when its rule has no entry |
| `waived-by` | the entry's `decided-by`, `unattributed` included |
| `waived-at` | the entry's `decided-at`; the run's own time when `unattributed` |
