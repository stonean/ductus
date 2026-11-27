---
spec: 020-code-review
status: done
dependencies: []
cross-spec-impact: [022-deterministic-runtime, 024-rule-loader, 034-performance-backend-rules, 037-observability-backend-rules, 038-concurrency-backend-rules, 039-reliability-backend-rules, 058-findings-route-at-discovery, 066-audit-records-and-the-completion-gate, 068-api-backend-rules, 069-accessibility-frontend-rules, 070-performance-frontend-rules, 076-waived-and-deferred-findings-are-distinct-dispositions]
next-criterion: 20
---

# 020 — `/ductus:review` code review command with blocking gate

Adds `/ductus:review`, a verb-named slash command that audits implementation code against the framework's rules across five dimensions (reuse, quality, security, efficiency, simplicity), writes a `review.md` artifact alongside the spec, and gates the `in-progress → done` transition via three reinforcing mechanisms.

> **Signpost:** [057 — Analyze artifact and record relocation](../057-analyze-artifact-and-record-relocation/spec.md) gave the run record one home, `review.md`'s frontmatter. `spec.md` carries no `review:` block and the spec template ships without one; a residual block is a **Blocking** validation finding. The schema is declared in the constitution's [§text-first-artifacts Frontmatter Schema](../../framework/constitution.md#frontmatter-schema) → **Audit records**, which is canonical for it, and this spec's `data-model.md` records the shape for `review.md` alone.
>
> AC5's third mechanism, the shipped CI step in `framework/templates/ci/adopter-generators.yml`, reads the review record from `review.md` and the analyze record from `analysis.md`, and bounds the specs it exempts for having no record with a committed high-water mark, so the step cannot go green over a corpus while checking nothing. `/{project}` does not install CI files, so an adopter's copy is detected and reported by the migration procedure rather than rewritten (`framework/migrations/audit-record-relocate.md`, step 5).
>
> **Signpost:** [079](../079-review-and-analysis-work-converges/spec.md) changed the review this spec delivered. Its default window starts where the spec's last review ended, its scope holds the spec's own change and names what it leaves out, a run decides a non-gating observation by rule, and a reopening that changed no code owes no review (081 counts the spec root and `.ductus/` as artifacts, so a scenario or data-model edit owes an analysis only). `compute-review-scope` and `write-review` state the rules.
>
> **Signpost:** [081](../081-reviews-and-checks-follow-what-changed/spec.md) made the review record state what its passes did not read: `unread`, which step 11 of `framework/commands/review.md` passes and `data-model.md`'s record table lists. `unread` names what the spec's next review reads again. A commit naming no spec and touching no spec directory is no spec's: its code is reviewed under the project record (spec 084).
>
> **Signpost:** [084 — Reopen for owed code, review every code change](../084-reopen-for-owed-code-review-every-code-change/spec.md) added `--project`, a review of the code no spec claims recorded in `{specs-root}/review.md`, which `--all` now includes, and gave the review one status write on the spec it reviews: `done → in-progress` once it unticks a criterion or defers a finding to that spec's own work, read from `check-owed-work`. On a `done` spec a ticked criterion judged not met changes only once the owner, or an invocation told to decide, says which side is correct.
>
> **Signpost:** [086 — Review records hold only what their scope judged](../086-review-records-hold-only-what-their-scope-judged/spec.md) changed what a review records and carries: `write-review` refuses a finding or observation on a file outside the run's scope, steps 8 and 9 pass each entry `process-decisions` and `process-waivers` drop for not belonging to the spec with the expired ones, and a first review's scope holds the files the spec's attributed commits changed.

## Summary

Add `/ductus:review`, a comprehensive code review slash command covering reuse,
quality, security, efficiency, and simplicity. Reviews are written as
`review.md` artifacts alongside the spec they audit. MUST violations block
the spec from advancing to `done` via three reinforcing mechanisms
(`/ductus:implement` halt, `/ductus:analyze` drift detection, optional CI gate),
consistent with the constitution's quality standards and the **Design
Principles** rule that framework features must not depend on human diligence.

`/ductus:review` audits **code against rules**. It is complementary to
`/ductus:analyze`, which audits **artifacts against each other**.

## Motivation

Before this spec the constitution referenced quality standards but no command
enforced them on implementation code: the command set it was written against
(`git ls-tree --name-only 30e1fbaa~1 framework/commands/`) held none that read
code. `/ductus:analyze` kept spec/plan/tasks artifacts internally consistent;
nothing checked whether the code that landed satisfied the spec, the security
rules added in spec 008, or basic quality expectations. Adopters had to
remember to run external review tools — a discipline dependency the framework
should remove.

## Acceptance Criteria

- [x] AC1: `/ductus:review` exists as a verb-named slash command in the same shape as `/ductus:analyze`, distributed through the standard `framework/commands/` → `.claude/commands/ductus/` regeneration pipeline.
- [x] AC2: Running `/ductus:review` against an `in-progress` target produces `specs/NNN-feature/review.md` with findings categorized into MUST, SHOULD, and low-confidence sections.
- [x] AC3: The command loads `framework/rules/security-backend.md` and `framework/rules/security-frontend.md` as authoritative security criteria. The five-dimension review model (security, reuse, quality, efficiency, simplicity) is applied to every targeted feature.
- [x] AC4: `review.md`'s own frontmatter holds the run record `/ductus:review` populates: `last-run`, `must-violations`, `should-violations`, `blocking`, optional `waivers`. It is created lazily, by a spec's first review. `spec.md` carries no block — a residual one is a Blocking validation finding — and the template ships without one, so a newly created spec opens with `status` and `dependencies` alone.
- [x] AC5: **Blocking gate**: a spec whose review record reports `blocking: true` cannot reach `done`. All three mechanisms read the record from `review.md`, and each treats an artifact that exists but carries no parseable record as **undeterminable**, never never-reviewed.
  - `/ductus:implement` halts before marking `done` and emits the blocking message.
  - `/ductus:analyze` reports a violation when a spec at `status: done` has `blocking: true` or a missing `last-run` in `review.md`. Its grandfather rule is keyed on the **artifact's** absence, never on a block's, which would exempt every spec in a migrated corpus.
  - The shipped CI template includes a check that fails PRs in the same conditions, and holds the specs it exempts for having no record to a committed high-water mark (see the signpost above).
- [x] AC6: Re-running `/ductus:review` against unchanged code produces identical `review.md` content modulo timestamp and SHA fields (idempotency invariant).
- [x] AC7: `--fix` applies only conservative auto-fixes per the scope rules in the command file. Behavior-changing fixes are never auto-applied.
- [x] AC8: Waivers require explicit `--waive <rule-id> --reason "..."`, are recorded in `review.md`'s frontmatter `waivers` list, and expire automatically when the file location or rule ID they were attached to changes. The list is open-schema — `write-review`'s waiver parser (`RawWaiverFull` in `runtime/src/primitives/write_review.rs`) keeps its `#[serde(flatten)]` extras, so an organization's custom waiver fields survive every re-render.
- [x] AC9: `write-review`'s result carries `exit-code`, `0` when not blocking and `1` when blocking; the CLI and `ductus exec` exit 0 once the record is written, and CI gates on the committed record's `blocking` instead (AC5's CI check; owner decision, 2026-11-19).
- [x] AC10: The README's Commands section lists `/ductus:review` under **Pipeline — advance state** with a one-line purpose.
- [x] AC11: **Tech-stack alignment gate**: before running review passes, `/ductus:review` confirms the project's `AGENTS.md` `Tech Stack` section exists and appears consistent with the implementation in scope. Misalignment or a missing/empty section is a blocking error, not a warning. Adopters can persist a successful check by setting `.ductus/config.toml [review] tech-stack-verified = true`, after which subsequent runs skip the check until the operator manually clears the key.
- [x] AC12: **Empty scope**: a target with an empty resolved scope (no implementation files) produces a `review.md` recording 0 findings across all five passes, `blocking: false`, and exits `0`.
- [x] AC13: **Cross-pass dedupe**: when the same finding (matching rule ID, file, site, and overlapping line range) is produced by more than one pass, only the highest-severity instance is retained in `must-violations` and `should-violations`; lower-severity duplicates are dropped from the counts and report.
- [x] AC14: **Flag parsing is specified and surfaced**: the command body documents how `$ARGUMENTS` is parsed for every flag in the Flags table, and `argument-hint` names each of them, so no flag is documented without being surfaced. A `--since` with no value and an unrecognized flag are each reported to the operator rather than silently absorbed. `/audit` holds `argument-hint` and the Flags table in agreement, so a flag added later cannot reopen the gap without a finding.
- [x] AC15: **A clean review states what it read**: `review.md` records `examined` — how many in-scope files the passes read — against a `scope` the primitive derives itself, so a review that examined its scope and found nothing is distinguishable from one whose passes never ran. An unstated `examined` is recorded as absent rather than as zero. Nothing checks `examined` against `scope`; a person reads both at the completion gate. This cannot prove the passes ran — an overstated numerator is as available as an omitted one — and it is not claimed to; it makes the claim explicit and checkable, which is the bar `QUAL-CLAIM-001` sets and the one `write-analysis`'s `unexamined` already cleared.
- [x] AC16: A host that can start a subagent runs review's five passes and the criteria step's judging, steps 3 to 7.2 of `framework/commands/review.md`, in one fresh subagent briefed as `framework/reference/review.md` §Passes in a subagent states, and continues at step 7.3 with only the findings, observations, criterion verdicts and coverage it returns, so the rule files and in-scope files never enter its own context; a read the subagent reports short is named in the Summary and kept out of `examined`. A host that cannot start one runs them itself, and `ductus exec` asks for each pass in turn, then for the criteria.
- [x] AC17: `/{project}:review` does not load a rule file whose header names `/{project}:analyze` and not `/{project}:review` among the enforcers of its rules: `discover-rule-files` leaves each such file out of a review's selection, with a notice naming it as judged by analysis, and keeps one whose header names neither, and a fixture rule file of each kind proves both.
- [x] AC18: A pass's finding, an observation or a reviewer's claim is recorded only when its trigger is shown possible in the project, from its code, configuration, history or deployment shape, as well as its mechanism being checked against the tree. A case shown only by building it is a false positive, recorded nowhere and fixed in no round, and `framework/reference/review.md` §Run review passes says so.
- [x] AC19: A stored waiver or violation deferral covers only the findings at its `(rule, file, site)` anchor, up to the number it covers: a finding of its rule at another site in the same file, more findings at its site than it covers, and any finding under an anchor recorded without a site each count and are reported for a new decision with the notice the scenario `waiver-expiry` gives; `--waive` and `--defer` refuse a finding that names no site; and a fixture of each case fails with its guard removed.

## Non-goals

- Replacing `/ductus:analyze`. The two commands target different artifacts.
- Making `/ductus:review` a pipeline-advance command in its own right. It is a
  gate, not a state transition.
- Shipping language- or framework-specific rule packs beyond the existing
  security rules. Adopters extend `framework/rules/` themselves.

## Affected files

| File | Change | Strategy |
| --- | --- | --- |
| `framework/commands/review.md` | **create** — full command file (see [Embedded artifacts](#embedded-artifacts)) | update |
| `framework/commands/implement.md` | edit — add blocking check before `status: done` transition | update |
| `framework/commands/analyze.md` | edit — add review-drift check, integrate with `--fix` | update |
| `framework/templates/spec/spec.md` | carries no `review:` block: the record is `review.md`'s | update |
| `framework/templates/ci/adopter-generators.yml` | edit — add the audit-record check, reading `review.md` and `analysis.md` | update |
| `framework/constitution.md` | edit — reference the review gate in the pipeline section | update |
| `README.md` | edit — add `/ductus:review` row to Pipeline commands table; document waivers | update |
| `scripts/regenerate-commands.sh` (or equivalent) | run after edits to regenerate `.claude/commands/ductus/review.md` | n/a |
| `.claude/commands/ductus/review.md` | regenerated output | derived |

## Tasks

1. Create `framework/commands/review.md` from the embedded artifact below.
2. Edit `framework/commands/implement.md`: before any logic that writes
   `status: done`, read the review record's `blocking`. If `true` (or
   `last-run` is missing entirely), halt with the message
   [Blocking message](#blocking-message) names and exit without modifying
   status.
3. Edit `framework/commands/analyze.md`: extend the audit to flag any spec
   at `status: done` whose review record has `blocking: true` or no `last-run`
   as a validation failure. Wire `--fix` to revert affected specs from
   `done` → `in-progress` and emit a notice (never silent).
4. Edit `framework/templates/ci/adopter-generators.yml` to add a step that
   exits non-zero if any spec at `status: done` has a review record in
   `review.md` with `blocking: true` or no `last-run` (see
   [Frontmatter schema](#frontmatter-schema)).
5. Edit `framework/constitution.md` pipeline section: add `/ductus:review`
   between `/ductus:implement` and the `done` transition, with a sentence
   explaining the blocking gate.
6. Edit `README.md`: add the `/ductus:review` row to the Pipeline (advance
   state) table; add a short Waivers subsection under Slash Commands; update
   any pipeline diagrams.
7. Run the regeneration script to produce `.claude/commands/ductus/review.md`.
8. Run `/ductus:analyze --all` against the ductus repo itself to confirm
   nothing in ductus's own specs broke.
9. Add a scenario at `specs/020-code-review/scenarios/waiver-expiry.md`
   capturing the waiver auto-expiry behavior — this is the subtlest
   requirement and warrants a focused scenario.

## Open questions

*None — all resolved.*

## Resolved questions

- **MUST waivers and second sign-off** — single-author waivers remain the
  framework standard; `/ductus:review` does not require a second `co-waived-by`
  field. Nothing in ductus can verify that a second value belongs to a
  different person — a waiver's `waived-by` is the author's own
  `git config user.email` (`framework/reference/review.md` §Waivers), which the
  runtime records as given — so encoding the requirement in frontmatter would be
  performative — exactly the "depend on human diligence" anti-pattern
  §pipeline-boundaries exists to avoid. The real second review happens at
  PR-merge time, where the waiver and the unfixed finding land in the diff
  and human code review applies org policy. Adopters whose policy does
  require two-author waivers can layer fields like `co-waived-by` onto the
  `waivers` entries — the §text-first-artifacts open-schema rule
  guarantees `/ductus:review` and `/ductus:analyze` will not error on unknown
  fields — and gate them in their own CI.
- **Stack detection source** — `/ductus:review` continues to read `AGENTS.md`
  `Tech Stack` to choose between `security-backend.md` and
  `security-frontend.md`. `AGENTS.md` `Tech Stack` *is* the canonical sink
  for tech-stack metadata — there is no separate surface to point at. It is the only tech-stack surface.
- **Quality-pass confidence threshold** — fixed as a `high` or `low` tier per finding (a numeric threshold of 80 when this was decided); not exposed via
  `.ductus/config.toml`. The threshold is an opinion about LLM calibration, not
  about project domain — adopters have no meaningful information to tune it,
  only an incentive to raise it when reviews are inconvenient. Per
  §pipeline-boundaries ("never depend on human diligence"), making the gate
  tunable would let teams effectively waive it by setting the threshold to
  100. `.ductus/config.toml` is reserved for genuine project-level decisions
  (pinned files, paths, agent identity per specs 017 and 019); a
  framework-wide quality opinion doesn't qualify. If model calibration
  shifts, the framework updates the value uniformly for all adopters.
- **`--all` scope** — `--all` reviews every feature whose status is
  `in-progress` or `done`. Excluding `done` would make the blocking gate
  retroactively blind to new MUST rules added after a feature shipped: the
  `/ductus:analyze` drift check fires only when the review record's `blocking`
  is already `true` or its `last-run` is missing, so rules introduced after
  the last review never re-flip the flag on shipped code. A `done` spec's body
  is a living document (§spec-lifecycle), and the code it describes keeps
  living too, so it should stay compliant with current rules. Single-target `/ductus:review` already accepts
  `done` (step 1 of `framework/commands/review.md` halts only when status is
  *not* in `{in-progress, done}`);
  `--all` simply enumerates the same set the gate already permits.

## Frontmatter schema

This record is `review.md`'s frontmatter, never the spec's. The example shows
its field set. The canonical declaration is
[§text-first-artifacts Frontmatter Schema](../../framework/constitution.md#frontmatter-schema) → **Audit records**, which is where the schema ships to adopters; the shape
`/ductus:review` writes is recorded field-by-field in
[data-model.md](data-model.md).

> **Signpost:** the fix-round loop's fields — `fix-rounds`, `re-reads`, `diff-checked`, `not-re-read`, `budget-stop` and `round-waivers` — and the `[review] re-reads-off` key are defined by [078 — Review fix rounds are re-read within a bound](../078-review-fix-rounds-are-re-read-within-a-bound/data-model.md).

```yaml
---
spec: 020-code-review                 # the feature slug
last-run: 2026-05-10T14:32:00Z        # ISO 8601, set by /ductus:review
reviewed-against: <sha>               # HEAD sha at review time; provenance only
must-violations: 0                    # count after waivers applied
should-violations: 3
low-confidence: 2
blocking: false                       # true iff must-violations > 0
waivers:                              # optional, omitted when empty
  - rule: BE-AUTHN-001
    file: src/api/internal.ts
    reason: "Endpoint is internal-only behind mTLS"
    waived-at: 2026-05-10T14:40:00Z
    waived-by: dev@example.com
---
```

> **Signpost:** [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md) set how a run treats its observations. Each is fixed, deferred or waived with its reason in the run that finds it, and the record counts those outcomes in a `dispositions:` map (`fixed`, `deferred`, `waived`, `undispositioned`) and stores each deferred or waived decision in a `decisions:` list beside `waivers:`. The pre-done gate blocks an `in-progress` spec while the record has no map at all; an undecided observation does not hold `done` (058's `findings-cost-in-proportion-to-risk`), a broken contract apart (079). [data-model.md](data-model.md) records both fields.
>
> **Signpost:** [076 — Waived and deferred findings are distinct dispositions](../076-waived-and-deferred-findings-are-distinct-dispositions/spec.md)
> sets the violation dispositions. A MUST or SHOULD violation is fixed, waived or deferred. A waiver means it
> will not be fixed, with its reason, and names no future work, so a waiver
> carries no ticket. `/ductus:review --defer <rule-id>` defers it to its work —
> `--target` (with `--task`), `--work-item`, or both — stored under
> `decisions:` keyed by site (`` rule {rule-id} at {file} in `{site}` ``, AC19). A violation its deferral releases
> leaves the counts and renders under **Deferred findings**; any other stays
> counted, carries `gating: true`, and still holds `done`.
> [data-model.md](data-model.md) records both.
>
> **Signpost:** [066 — Audit Records and the Completion Gate](../066-audit-records-and-the-completion-gate/spec.md) added the criteria step's verdicts to the record, `criteria-judged`, `criteria-met`, `criteria-not-met` and `criteria-verdicts`, and a not-met criterion sets `blocking`, so `blocking` is `must-violations > 0` or a not-met criterion. [data-model.md](data-model.md) records the fields.

When an anchored `file` no longer exists or its `rule` is no longer triggered
at that location, the waiver's anchor is dropped on the next `/ductus:review` run and
the underlying finding re-blocks if it's still present elsewhere. A
waiver `--waive` records lists `anchors:`, each a `{file, site, covers}` (AC19), and an entry holding `file` and no `anchors`, a departure
waiver's aside, was recorded before sites and expires on its spec's next review
(the scenario `waiver-expiry`; data-model.md records the shape).

## Blocking message

The live texts are the primitives', canonical for command behavior: the
blocked message of `check-review-gate`, which step 11 of
`framework/commands/implement.md` halts with, and the `review-state-drift`
findings of `check-artifacts`, which `/ductus:analyze` reports (**Review state
drift** in `framework/reference/analyze.md`). The text `/ductus:review` emits
when tech-stack alignment fails is in
`framework/reference/review.md` §Blocking message.

---

## Embedded artifacts

This section held frozen copies of the command sources this spec delivered:
`framework/commands/review.md` in full, plus the required edits to
`framework/commands/implement.md` and `framework/commands/analyze.md`. They are
removed. What 020 delivered is recorded below; the live sources are the
authority, and `30e1fbaa` holds the original bytes.

- **`framework/commands/review.md`** — the command itself: the five review
  dimensions, the rule-file loading and stack filter, the waiver flow, and the
  report and run record written to `review.md`. 020 wrote the record into a
  `review:` block in spec frontmatter as well; 057 left it one home.
- **`framework/commands/implement.md`** — the pre-`done` review gate. The
  `in-progress` → `done` transition halts while the review record's `last-run`
  is unset or its `blocking` is `true`. The gate has grown since — it now reads
  a longer list in order, the analyze record among them — and its live shape is
  the command source's.
- **`framework/commands/analyze.md`** — the review-drift check, which reports a
  `done` spec whose review record is missing or blocking, and reverts it under
  `--fix`. Its grandfather rule is keyed on `review.md`'s absence since 057; it
  was keyed on the block's, which the relocation made true of every spec.

**Why the copies are gone rather than refreshed.** A snapshot inside a fenced
code block is invisible to every link check, anchor resolver and audit family,
so it rots with no signal — and it is read in full by every contributor and
every agent that opens this file, in every session, while a pointer costs one
line. This one had fallen more than half behind: its copy of
`framework/commands/review.md` ran 429 lines against a source of 896, and the
section's three copies ran 561 (`wc -l` of each at `fc70afc6~1`, the section
counted from its heading; `fc70afc6`'s message says 894, the source's size at
`6fe820ee`). It was found by a person
reading the spec, who logged it to the inbox item `fc70afc6` discharged.
[§drift-prevention](../../framework/constitution.md#drift-prevention)'s
*Canonical sources* names the command source as authoritative for command
behavior and requires a pointer rather than a copy; the rule and a worked
example are `050-constitution`'s scenario
`a-canonical-source-is-pointed-at-not-copied`, cited in prose rather than
linked so the citation induces no dependency edge. Git history is where a
document's prior state lives, so nothing is lost by removing the duplicate.
