# 085 — The audit-record gate reads records through the runtime Data Model

The shapes `check-audit-records` takes and returns, and the baseline file it reads. Its compiled description (`CHECK_AUDIT_RECORDS` in `runtime/src/schema/descriptions.rs`) is the source of its rules; this file states the shapes and field meanings.

## Arguments

None. The subject is the corpus under the configured spec root (`[paths] specs-root`, default `specs`).

## Result

| Field | Type | Meaning |
| --- | --- | --- |
| `specs-root` | string | The spec root walked, project-relative |
| `examined-count` | integer | Specs whose `spec.md` frontmatter was read, whatever their status |
| `done-count` | integer | Of those, specs at `status: done`, whose records were read |
| `skipped-count` | integer | Feature directories not examined, each in `skipped` |
| `findings-count` | integer | Entries in `findings` |
| `baseline` | object | `path` (`.ductus/audit-record-baseline.txt`) and `state`: `absent`, `read`, `malformed`, `unreadable` or `outside-project` |
| `exempt` | array of two | One per record, `review` then `analyze`: `record`, `key` (`review-exempt`, `analyze-exempt`), `count`, `mark` and `over` (`count` above `mark`) |
| `findings` | array | Each `{kind, path, message}`, `path` project-relative |
| `skipped` | array | Each `{feature, reason}`, `reason` `untracked` or `no-spec-file` |
| `review-exempt-specs`, `analyze-exempt-specs` | array of slugs | The `done` specs with no `review.md`, or no `analysis.md`, in corpus order |
| `form` | `inline` or `file` | Past `[artifacts] read-size-bytes` (50,000 by default) `findings`, `skipped` and both exempt lists are absent, each count stays inline, and `lists-file` names the JSON file under `.ductus/results/` holding them whole |

A spec root that is absent or cannot be listed fails the call with an error naming it; it is never a result with `examined-count` zero.

## Finding kinds

| `kind` | `path` | When |
| --- | --- | --- |
| `outside-project` | the feature directory | It is, or holds, a symbolic link leaving the project; nothing under it is read |
| `spec-unreadable` | its `spec.md` | `spec.md` cannot be read, or its frontmatter does not parse or names no `status` |
| `record-unreadable` | the record's file | A `done` spec's `review.md` or `analysis.md` exists and does not load (the loader's reason in `message`) |
| `no-last-run` | the record's file | A present record's `last-run` is null, absent or blank |
| `no-blocking` | the record's file | A present record has no `blocking` key, or its probe did not load |
| `blocking` | the record's file | A present record carries `blocking: true` |
| `baseline-malformed` | the baseline file | A known key's value is not a whole number |
| `baseline-unreadable` | the baseline file | The file exists and cannot be read |
| `baseline-outside-project` | the baseline file | The path is a link leaving the project |
| `exempt-over-mark` | the exempt spec's `spec.md` | Its record's exempt set is over its mark; one finding per spec in the set |

An absent record is no finding: it joins its record's exempt set. A `done` spec with one record present and the other absent counts only in the absent record's set.

## CLI report and exit status

The CLI prints the whole answer on stderr before the result is bounded, one line each:

- `check-audit-records: finding: {path}: {message}` per finding, `path` project-relative;
- `check-audit-records: examined {n} spec(s) under '{specs-root}'; {d} at done; {s} skipped`;
- `check-audit-records:   {key}: {count} (baseline {mark})` per exempt set.

It exits non-zero when `findings-count` is above zero or `examined-count` is zero.

## Baseline file

`.ductus/audit-record-baseline.txt`, project-relative, committed. One `key: value` per line; text after `#` is a comment; a line without `:` is ignored.

| Key | Value | Absent |
| --- | --- | --- |
| `review-exempt` | whole number, the high-water mark of `done` specs with no `review.md` | mark 0 |
| `analyze-exempt` | whole number, the same for `analysis.md` | mark 0 |

A key the primitive does not know is ignored. The file absent is state `absent`, every mark zero.
