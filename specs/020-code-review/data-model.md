---
status: draft
---

# 020 — `/ductus:review` Data Model

Data structures introduced by [020 — `/ductus:review`](spec.md). Authoritative shapes; the spec body and `framework/commands/review.md` reference these. (The spec's embedded copy of that command source was replaced by a pointer on 2026-09-13 — the live source is the authority for command behavior, per §drift-prevention's canonical-sources rule.)

> **This document had two tables for one record until 057, and that is the drift it produced.**
> As 020 delivered it, the run record lived both in a `review:` block in `spec.md`
> frontmatter and in `review.md`'s own frontmatter, and both were recorded below.
> They fell behind in *different* ways — the block's table was missing `examined`,
> `scope` and `reviewed-digest`, and `review.md`'s was missing `captured-issues`,
> `examined` and `scope` — so the document designated authoritative contradicted
> 020's own AC15, and nothing reported it. `057-analyze-artifact-and-record-relocation`
> merged the two into `review.md` alone. One table remains, below; the schema's
> canonical declaration is [§text-first-artifacts Frontmatter Schema](../../framework/constitution.md#frontmatter-schema) →
> **Audit records**, and this table records what `/ductus:review` writes.

## Waiver record

One entry per waiver judgment: one rule, waived at one or more files for one reason. Lives under `waivers` in `review.md`'s frontmatter — in spec frontmatter, under `review.waivers`, before 057 relocated the record. The list itself is open-schema — adopters MAY add fields like `co-waived-by`, `approved-by-team`, `ticket`.

```yaml
- rule: SEC-BE-014
  file: src/api/internal.ts
  reason: "Endpoint is internal-only behind mTLS; rule applies to public APIs"
  waived-at: 2026-05-10T14:40:00Z
  waived-by: dev@example.com
- rule: SEC-BE-021
  file:
    - src/api/admin.ts
    - src/api/metrics.ts
  reason: "Both routes are bound to the loopback interface"
  waived-at: 2026-05-11T09:15:00Z
  waived-by: dev@example.com
```

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `rule` | string (rule ID) | yes | E.g. `SEC-BE-014`. Must reference a known rule at write time. |
| `file` | string (relative path), or a list of them | yes | The paths the waiver is anchored to, each its own `(rule, file)` anchor and matched literally — never expanded as a pattern. Written as a single path when the entry holds one and as a list when it holds more. An empty list, or a list holding a blank path, is missing. Added by [waiver-file-lists](scenarios/waiver-file-lists.md); a one-path entry is unchanged. |
| `reason` | string | yes | Free-text justification. Empty string is invalid. |
| `waived-at` | ISO 8601 timestamp | yes | Set by `/ductus:review --waive`. |
| `waived-by` | string (email) | yes | Sourced from `git config user.email`. |
| (additional fields) | any | no | Open-schema; ignored by `/ductus:review` and `/ductus:analyze`. |

### Expiry rule

A waiver anchor expires (its path is dropped from `review.md`'s frontmatter on the next `/ductus:review` run — from the spec's, before 057 — and the entry with its last path) when **either** of the following holds:

- The anchored path no longer exists in the repository (renamed or deleted).
- The named `rule` no longer fires at that path (rule removed, or the violating code was fixed).

When the underlying finding still exists elsewhere in scope after expiry, it re-counts toward its severity's count: a MUST toward `must-violations`, flipping the record's `blocking` flag back to `true`; a SHOULD toward `should-violations`, which holds `done` at the pre-done gate. The detailed edge-case behavior is in [`scenarios/waiver-expiry.md`](scenarios/waiver-expiry.md).

## `review.md` artifact

Written to `specs/NNN-feature/review.md` — one review artifact per spec, regardless of whether the run targeted a feature or a scenario. A scenario-targeted run records which scenario it covered via the `scenario:` frontmatter field. Regenerated wholesale on each run; the most recent `/ductus:review` invocation supersedes any prior report.

### Frontmatter

The run record, and since 057 the **only** copy of it. Written in the order
below on every run.

> **Changed by [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md).**
> `captured-issues` is gone: observations are no longer written to the inbox,
> so the run has nothing to count there. The record carries a `dispositions:`
> map counting what the run did with its observations, and a `decisions:` list
> storing each routed or discarded one beside `waivers:`. The shapes of both are
> recorded in [058's data model](../058-findings-route-at-discovery/data-model.md).

```yaml
---
spec: 020-code-review
scenario: waiver-expiry              # only on a scenario-targeted run
last-run: 2026-05-10T14:32:00Z
reviewed-against: <sha-of-HEAD>
diff-base: <sha of the parent of the in-progress transition commit>
must-violations: 0
should-violations: 3
low-confidence: 2
examined: 8
scope: 11
skipped-passes: []
reviewed-digest:
  data-model.md: <sha256>
  scenarios/waiver-expiry.md: <sha256>
blocking: false
dispositions:
  fixed: 0
  routed: 1
  discarded: 0
  undispositioned: 0
waivers: []                          # omitted entirely when empty
decisions:                           # omitted entirely when empty
  - key: "convention: retry backoff is hard-coded — `src/retry.ts`"
    outcome: routed
    target: specs/020-code-review/scenarios/retry-config.md
    decided-at: 2026-05-10T14:32:00Z
    decided-by: dev@example.com
---
```

| Field | Type | Required | Notes |
| --- | --- | --- | --- |
| `spec` | string | yes | Feature slug — matches the parent directory. |
| `scenario` | string | no | Written only when the run was scenario-targeted; absent otherwise. |
| `last-run` | ISO 8601 timestamp or null | yes | Set on each run. Spelled `reviewed-at` here until 057; the spec block called the same instant `last-run`, and one home leaves one spelling. Null only on a record that has never run. |
| `reviewed-against` | string (Git SHA) | yes | HEAD SHA at review time. **Provenance, never the staleness basis** — read for one thing, the mechanical-sweep rename exemption. |
| `diff-base` | string (Git SHA) | yes | The SHA where the spec advanced to `in-progress`, or the value passed to `--since=<ref>`. |
| `must-violations` | integer ≥ 0 | yes | Post-waiver count. |
| `should-violations` | integer ≥ 0 | yes | Advisory severity count. |
| `low-confidence` | integer ≥ 0 | yes | Quality-pass findings below 80 confidence. Excluded from `must-violations`. |
| `examined` | integer ≥ 0 | no | How many in-scope files the five passes actually **read**. Recorded as absent rather than zero when unstated — an unstated claim and a stated zero are different. Added by 022; see AC15. |
| `scope` | integer ≥ 0 | yes | The denominator `examined` is asserted against, derived by `write-review` rather than supplied. Always written, because it was always computed. Added by 022. |
| `skipped-passes` | list of strings | yes | Empty when no flag restricts dimensions. Permitted values: `security`, `reuse`, `quality`, `efficiency`, `simplicity`. |
| `reviewed-digest` | map of path → sha256 | yes | Per-path digest of the spec's **durable contracts** (`scenarios/*.md` and `data-model.md`) as the run read them. What freshness is computed from. Always written, empty map included — *digest taken over a spec with no durable contracts* must stay distinct from *pre-digest record*, which reports freshness as undeterminable rather than current. Added by 022. |
| `reviewed-unreadable` | list of strings | no | Durable contracts that exist but could not be read, recorded rather than digested as empty. Omitted when none. |
| `blocking` | boolean | yes | MUST equal `must-violations > 0`. Derived by `write-review`, never accepted from the caller. Read by `/ductus:implement`, `/ductus:analyze`, the CI template. |
| `dispositions` | map: `fixed`, `routed`, `discarded`, `undispositioned` → integer ≥ 0 | yes | What the run did with its **observations**; MUST and SHOULD violations keep their own counts. Derived by `write-review` from each observation's disposition, and always written with all four counts. A record without it predates dispositions, and the pre-done gate blocks an `in-progress` spec on it. Replaced `captured-issues` in 058. |
| `waivers` | list of waiver records | no | Omitted entirely when empty. Schema above is open per §text-first-artifacts. |
| `decisions` | list of decision records | no | Stored routed and discarded observation decisions, keyed on the observation's rendered line, each with `outcome`, `target` or `reason`, `decided-at`, and `decided-by`. Omitted entirely when empty. Open-schema like `waivers`. Added by 058. |

**Validation severity** (per [§text-first-artifacts Frontmatter Schema](../../framework/constitution.md#frontmatter-schema) →
**Audit records**, which has declared both records since 057):

- **Hard fail** — a `review.md` that exists but carries no parseable record. The artifact's *absence* is the never-reviewed state and is not a defect; its presence without a readable record is undeterminable, and the two must never render alike.
- **Blocking** — a `done` spec with `blocking: true` or a missing `last-run`; and a `review:` or `analyze:` block still present in a spec's frontmatter, which the open-schema rule deliberately does not cover.
- **Informational** — unknown fields on the record (open-schema rule).

### Body sections (in order)

Rendered wholesale on every run; each section is always present and reads
`*None.*` when empty, so a section's absence is a defect rather than a clean
result.

| Section | When emitted |
| --- | --- |
| `## Summary` | Always |
| `## MUST violations (blocking)` | Always (empty when none) |
| `## SHOULD violations (advisory)` | Always (empty when none) |
| `## Low-confidence findings` | Always (empty when none) |
| `## Waived findings` | Always (empty when none) |
| `## Observations` | Always (empty when none). Reviewer observations that map to no loaded rule, each rendered beside its disposition: fixed, routed with its target, discarded with its reason, or undispositioned. Nothing is written to the inbox. Added by 022; dispositions by 058, which also removed the section 047 had added beside this one to list inbox captures |
| `## Skipped passes` | Always (empty when none) |
| `## Unexamined governance` | Always (empty when none). Added by 055 |

### Finding record

Each finding under MUST/SHOULD/Low-confidence sections:

```markdown
### MUST: <rule-id> — <one-line summary>

- **File**: `path/to/file.ts:42-55`
- **Rule**: <verbatim rule text from framework/rules/...>
- **Finding**: <one to three sentences>
- **Auto-fixable**: yes | no
- **Suggested fix**: <code block or prose>
```

Findings under **Waived findings** include an additional `**Waived**: <reason from the waiver in review.md's frontmatter>` field.

### Idempotency invariant

For a given `(code-in-scope, loaded-rules, spec-acceptance-criteria, scenarios, waivers)` input set, the body of `review.md` is byte-identical across runs. Only `last-run` (spelled `reviewed-at` before 057) and `reviewed-against` in the frontmatter are permitted to differ. This is the basis of acceptance criterion 6.

## `.ductus/config.toml [review]` section

New TOML section in the project's `.ductus/config.toml`. Added by `/ductus:review` (with operator confirmation) on the first successful tech-stack alignment check. `.ductus/config.toml` is shared adopter-side state per AGENTS.md (Workflow); this spec documents the section it adds rather than touching spec 019.

```toml
[review]
tech-stack-verified = true
```

| Key | Type | Default | Notes |
| --- | --- | --- | --- |
| `tech-stack-verified` | boolean | `false` (when key absent) | When `true`, `/ductus:review` skips the tech-stack alignment pre-flight on every run until the operator removes the line. There is no auto-reset. |

The section is open-schema; future review-related persisted decisions can land under `[review]` without schema migration.

## Cross-references

- Spec lifecycle and §spec-requirements: [`framework/constitution.md`](../../framework/constitution.md).
- Open-schema rule for frontmatter: [`framework/constitution.md`](../../framework/constitution.md) §text-first-artifacts.
- Security rule ID conventions cited under `waivers[].rule`: [`framework/rules/security-backend.md`](../../framework/rules/security-backend.md), [`framework/rules/security-frontend.md`](../../framework/rules/security-frontend.md).
