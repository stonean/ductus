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

`decided-at` is absent when `decided-by` is `unattributed`, and `cited` when the entry has none. On the interactive path the host supplies only `verdict: waived`, and under `ductus exec` the walker records it from an ordinary answer; `write-analysis` fills the rest from the entry on every write.

## `check-departures` result

| Field | Type | Meaning |
| --- | --- | --- |
| `form` | `inline` or `file` | Where the three lists are: `file` past `[artifacts] read-size-bytes` |
| `entries-count`, `findings-count`, `unexamined-count` | integers | Each list's length, inline in both forms, so a zero is an empty list and never an absent one |
| `lists-file` | path | In the file form, the file under `.ductus/results/check-departures/{feature}/` holding `entries`, `findings` and `unexamined` whole |
| `entries` | list of `{rule, tier, reason, cited, decided-by, decided-at, line}` | Every well-formed entry; `tier` is the rule's, absent when the rule is not loaded |
| `findings` | list of `{family, tier, message, path}` | Family `departure-entries`: no rule ID, no or an empty `reason:`, a second rule ID, another or a repeated child key, an unknown or retired rule (with no rule file loaded, only a token that is not, whole, shaped like a rule ID, which is unknown), a rule two entries name (Blocking); a deprecated rule, a `cited:` that is not exactly one `§` reference (Advisory); an unresolved reference is step 4's |
| `unexamined` | list of `{reason, line}` | A citation that could not be judged (`anchor-target-unjudgeable`, `anchor-target-unreadable`), a rule not installed (`rule-not-installed`), or, with no rule file loaded, every entry's rule (`rule-citations-not-checked`, with no unknown, retired or deprecated finding); never a pass. Each repeats a target step 4 or step 5 records, so a caller counts it once; a token that is no rule-ID shape is still an unknown-rule finding, as step 5 records no citation for it |

**Size** (BE-PAYLOAD-001). The three lists grow with a spec's entries, so the result is bounded as `process-decisions`' is (066's data model, its **Size**): while it fits `[artifacts] read-size-bytes` it is inline, and past it `form` is `"file"` and none of the lists is inline. The file form's own size is 192 bytes of compact JSON plus the feature directory's name and each count digit past the first (measured on the built binary: 197 bytes for `001-x`, each count one digit). Consumers read the lists from `lists-file` in the file form: step 5.1 of `framework/commands/analyze.md` and, under `ductus exec`, the walker, which restores them before `AnalyzeTally` reads the result.

## Review waiver marker

A waiver `write-review` derives from an entry is an ordinary `waivers:` entry (`framework/reference/review.md` §Waivers) with one open-schema field:

| Field | Value |
| --- | --- |
| `departure` | `true`; `process-waivers` expires the waiver when its rule has no entry |
| `waived-by` | the entry's `decided-by`, `unattributed` included |
| `waived-at` | the entry's `decided-at`; the run's own time when `unattributed` |

Every write refreshes a stored departure waiver's `reason`, `waived-by` and `waived-at` from its entry, except that while the entry is `unattributed` the stored `waived-at` (the run time it was first stored) is kept. A departure waiver covers only files in the run's review scope. The entry decides over a deferral of its violations: a new deferral is refused, a stored one is superseded and named in `write-review`'s `superseded-deferrals`.
