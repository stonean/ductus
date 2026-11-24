# 073 — Decided Rule Departures Are Recorded Data Model

The structures this feature adds. The `rule-verdicts` map's own shape is 082's ([data-model](../082-rule-assessments-converge-across-runs/data-model.md)) and the analysis record's fields are 066's; this file owns the departure entry, the `waived` verdict's fields, `check-departures`' result and the review waiver's marker. Once implemented, each primitive's compiled description (`ductus <name> --help`) is the authority for its behavior.

## Departure entry

A list item under a spec's `## Applicable Rules` heading, outside fenced code and HTML comments:

```markdown
- departs: `RULE-ID`
  - reason: <non-empty text>
  - cited: §<anchor>
  - covers:
    - `<file>` in `<site>`
```

| Part | Required | Meaning |
| --- | --- | --- |
| `departs:` and one backticked rule ID | yes | The rule the spec deliberately does not satisfy, of any tier, its ID a code span as `CommonMark` reads one, so a backtick with no matching run opens none |
| `reason:` | yes | Why, in the spec author's words |
| `cited:` | no | One `§anchor` reference to the governing document that decided the departure in advance |
| `covers:` and its items | no | The sites a scoped entry decides (§Scoped entries in `spec.md`), each item two code spans joined by `in`: a file from the project root, then a site spelled as review records one, `(top level)` included, each span read as `CommonMark` reads one, so a site holding a backtick takes a longer delimiter. Without it the entry covers the spec's whole review scope |

Any other child key, a repeated key, a second rule ID, or an empty `reason:` makes the entry malformed, and so does a `covers:` with text after its colon or a line under it that is no list item, one listing no item, an item that is not two non-blank code spans joined by `in`, an item an earlier one repeats, or a file that is absolute or holds a `..` component. A malformed entry waives nothing.

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
| `entries` | list of `{rule, tier, reason, cited, decided-by, decided-at, line}` | Every well-formed entry; `tier` is the rule's, absent when the rule is not loaded or its Statement carries no MUST or SHOULD |
| `findings` | list of `{family, tier, message, path}` | Family `departure-entries`: no rule ID, no or an empty `reason:`, a second rule ID, another or a repeated child key, an unknown or retired rule (with no rule file loaded, only a token that is not, whole, shaped like a rule ID, which is unknown), a rule two entries name (Blocking); a deprecated rule, a `cited:` that is not exactly one `§` reference (Advisory); an unresolved reference is step 4's |
| `unexamined` | list of `{reason, line}` | A citation that could not be judged (`anchor-target-unjudgeable`, `anchor-target-unreadable`), a rule not installed (`rule-not-installed`), or, with no rule file loaded, every entry's rule (`rule-citations-not-checked`, with no unknown, retired or deprecated finding); never a pass. Each repeats a target step 4 or step 5 records, so a caller counts it once; a token that is no rule-ID shape is still an unknown-rule finding, as step 5 records no citation for it |

**Size** (BE-PAYLOAD-001). The three lists grow with a spec's entries, so the result is bounded as `process-decisions`' is (066's data model, its **Size**): while it fits `[artifacts] read-size-bytes` it is inline, and past it `form` is `"file"` and none of the lists is inline. The file form's own size is 192 bytes of compact JSON plus the feature directory's name and each count digit past the first (measured on the built binary: 197 bytes of compact JSON for `001-x`, 198 as the CLI prints it with its trailing newline, each count one digit). Consumers read the lists from `lists-file` in the file form: step 5.1 of `framework/commands/analyze.md` and, under `ductus exec`, the walker, which restores them before `AnalyzeTally` reads the result.

## `process-waivers` result

`process-waivers` is 020's and 022's primitive; 073's departure waivers grow its lists by one anchor per rule and in-scope file, so its result is bounded as `process-decisions`' is.

| Field | Type | Meaning |
| --- | --- | --- |
| `form` | `inline` or `file` | Where the four lists are: `file` past `[artifacts] read-size-bytes` |
| `applied-count`, `expired-count`, `retained-count`, `notices-count` | integers | Each list's length, inline in both forms |
| `lists-file` | path | In the file form, the file under `.ductus/results/process-waivers/{feature}/` holding `applied`, `expired`, `retained` and `notices` whole, each key present even when empty |

**Size** (BE-PAYLOAD-001). While the result fits `[artifacts] read-size-bytes` the four lists are inline, an empty one as `[]`; past it `form` is `"file"` and none is inline, so a count above zero with no list is a moved list, never a short one. A result with all four lists empty is inline and reads no setting, so a malformed read size halts only a result with a list to bound (`QUAL-DELEG-001`). The file form's own size is 206 bytes of compact JSON plus the feature directory's name and each count digit past the first, measured on the built binary over a repository holding one applied waiver (`ductus process-waivers`, `read-size-bytes = 10`, each count one digit): 211 bytes for feature `001-x` (212 with the CLI's trailing newline) and 213 for `001-xyz`. Consumers read the lists from `lists-file` in the file form: steps 8 and 11 of `framework/commands/review.md`, and under `ductus exec` the walker, which restores them (`restore_lists`) before `write-review` binds its `applied-waivers` and `expired-waivers` from the context's `applied` and `expired` keys.

## Review waiver marker

A waiver `write-review` derives from an entry is an ordinary `waivers:` entry (`framework/reference/review.md` §Waivers) with one open-schema field:

| Field | Value |
| --- | --- |
| `departure` | `true`; `process-waivers` expires the waiver when its rule has no entry |
| `waived-by` | the entry's `decided-by`, `unattributed` included |
| `waived-at` | the entry's `decided-at`; the run's own time when `unattributed` |

Every write refreshes a stored departure waiver's `reason`, `waived-by` and `waived-at` from its entry, except that while the entry is `unattributed` the stored `waived-at` (the run time it was first stored) is kept. A departure waiver covers only files in the run's review scope. An unscoped entry's waiver lists its files under `file`, covering every site in each; a scoped entry's holds `anchors:`, each a `file` and a `site` with no `covers`, covering the findings at that site alone, and `process-waivers` expires an anchor its entry no longer lists, and one stored in a form the entry has left, scoped or not. The entry decides over a deferral of its violations at the sites it covers: a new deferral is refused, a stored one is superseded and named in `write-review`'s `superseded-deferrals`. `write-review`'s `departure-notices` names each place a scoped entry's coverage is lost: a covered file not in the tree or leaving the project, which is never probed, and a finding of its rule in a covered file the scope holds at a site the entry does not list or naming none, each with the entry's `spec.md` line and its covered sites.

**Size** (BE-PAYLOAD-001). `malformed-departures`, `superseded-deferrals` and `departure-notices` grow with a spec's departure entries, its stored deferrals and its scoped entries' covered sites, so they are bounded with `analyze-stale-paths` (066's data model, its **Size**): while the result fits `[artifacts] read-size-bytes` each is inline, absent when empty, and past it `form` is `"file"`, none is inline, and the one file `lists-file` names holds `analyze-stale-paths`, `malformed-departures`, `superseded-deferrals` and `departure-notices` whole, each key present even when empty. `malformed-departures-count`, `superseded-deferrals-count` and `departure-notices-count` are inline in both forms, so a count above zero with no list is a moved list, never a short one. A result with none of the four lists to move reads no setting. The file form's own size is 591 bytes of compact JSON plus three times the length of the feature directory's name, which the result names three times, measured on the built binary (`ductus write-review` over a two-commit repository, `never-run` analysis, two malformed entries and one scoped entry covering a file not in the tree, so one-digit counts, `examined` 1000 and `scope` 1): 606 bytes for feature `001-x`, 607 as the CLI prints it with its trailing newline, and 612 for `001-xyz`. Of those, the three counts are 90 (`,"malformed-departures-count":2` and `,"superseded-deferrals-count":0` are 31 bytes each, `,"departure-notices-count":1` is 28). Add each digit past the first of a count, `examined`, `scope` and the violation counts, and a `stale` `analyze-freshness`' fields other than `paths`. Consumers read the lists from `lists-file` in the file form: step 11 of `framework/commands/review.md`, and under `ductus exec` the walker restores them (`restore_lists`) before any step reads the result; nothing there reads these three lists.
