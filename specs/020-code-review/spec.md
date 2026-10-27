---
spec: 020-code-review
status: in-progress
dependencies: []
cross-spec-impact: [066-audit-records-and-the-completion-gate]
next-criterion: 16
---

# 020 — `/ductus:review` code review command with blocking gate

Adds `/ductus:review`, a verb-named slash command that audits implementation code against the framework's rules across five dimensions (reuse, quality, security, efficiency, simplicity), writes a `review.md` artifact alongside the spec, and gates the `in-progress → done` transition via three reinforcing mechanisms.

> **Superseded in part by [057 — Analyze artifact and record relocation](../057-analyze-artifact-and-record-relocation/spec.md), which moved the record this spec's gate reads and changed the mechanism of the third gate.**
>
> **The record has one home, and it is `review.md`.** 020 put the run record in
> *two* places — a `review:` block in the spec's frontmatter that the gates read,
> and `review.md`'s own frontmatter that a person read — with the same facts in
> both. 057 merged them into `review.md`'s frontmatter, removed the block from
> `spec.md` and from the spec template, and made a residual block a **Blocking**
> validation finding rather than an open-schema extra. The record's *fields* are
> unchanged in substance and gained the three that had lived only in `review.md`
> (`diff-base`, `captured-issues`, `skipped-passes`; 058 has since replaced
> `captured-issues` with a `dispositions:` map); the one rename is
> `reviewed-at` → `last-run`, so the pair that used to be spelled two ways for
> one instant is spelled once. The schema is now declared in the constitution's
> [§text-first-artifacts Frontmatter Schema](../../framework/constitution.md#frontmatter-schema) → **Audit records**, which is canonical for it; this spec's
> `data-model.md` records the same shape for `review.md` alone and no longer has
> a second table to keep in agreement.
>
> **The CI gate's predicate changed, and this is the part that was a defect
> rather than a relocation.** AC5's third mechanism — the shipped CI step in
> `framework/templates/ci/adopter-generators.yml` — skipped a spec whose
> frontmatter carried no `review:` block, on the reasoning that such a spec
> predated `/ductus:review`. The relocation makes that predicate true of *every*
> spec, so the step would have gone green over a whole corpus while checking
> nothing — the check-that-cannot-run shape §design-principles names. 057
> replaced it with a committed high-water mark (`.ductus/audit-record-baseline.txt`)
> in the ratchet form Family 37 already used, and extended the step to the
> analyze record in `analysis.md` beside the review one. The same vacuity had
> reached audit Family 19 and `mechanical_sweep_parity`, both re-pointed in the
> same change. `/{project}` does not install CI files, so an adopter's copy is
> detected and reported by the migration procedure rather than rewritten.
>
> **Audit Family 31 is gone.** It reconciled the two copies of the record, which
> is a check with no subject once there is one copy; AC15 is annotated below.

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

The constitution references quality standards but no command enforces them on
implementation code. `/ductus:analyze` ensures spec/plan/tasks artifacts are
internally consistent; nothing checks whether the code that landed actually
satisfies the spec, the security rules added in spec 008, or basic quality
expectations. Adopters currently have to remember to run external review
tools — a discipline dependency the framework should remove.

## Acceptance Criteria

- [x] AC1: `/ductus:review` exists as a verb-named slash command in the same shape as `/ductus:analyze`, distributed through the standard `framework/commands/` → `.claude/commands/ductus/` regeneration pipeline.
- [x] AC2: Running `/ductus:review` against an `in-progress` target produces `specs/NNN-feature/review.md` with findings categorized into MUST, SHOULD, and low-confidence sections.
- [x] AC3: The command loads `framework/rules/security-backend.md` and `framework/rules/security-frontend.md` as authoritative security criteria. The five-dimension review model (security, reuse, quality, efficiency, simplicity) is applied to every targeted feature.
- [x] AC4: Spec frontmatter gains a `review:` block populated by the command: `last-run`, `must-violations`, `should-violations`, `blocking`, optional `waivers`. **Relocated by 057**: the record is `review.md`'s own frontmatter and `spec.md` carries no block — a residual one is a Blocking validation finding. The five fields, their meanings and the lazy-population rule are unchanged; the file they land in is not, and the template ships without the block, so a newly created spec opens with `status` and `dependencies` alone.
- [x] AC5: **Blocking gate**: a spec whose review record reports `blocking: true` cannot reach `done`. **Relocated by 057**: all three mechanisms read the record from `review.md` rather than from a `review:` block in `spec.md`, and each gained the same third state — an artifact that exists but carries no parseable record is **undeterminable**, never never-reviewed.
  - `/ductus:implement` halts before marking `done` and emits the blocking message.
  - `/ductus:analyze` reports a violation when a spec at `status: done` has `blocking: true` or a missing `last-run` in `review.md`. Its grandfather rule is keyed on the **artifact's** absence; keyed on the block's, as it was, it would exempt every spec in a migrated corpus.
  - The shipped CI template includes a check that fails PRs in the same conditions. **Its predicate changed** — see the signpost above: the block-presence skip was replaced by a committed high-water mark, because the relocation made that skip true of every spec.
- [x] AC6: Re-running `/ductus:review` against unchanged code produces identical `review.md` content modulo timestamp and SHA fields (idempotency invariant).
- [x] AC7: `--fix` applies only conservative auto-fixes per the scope rules in the command file. Behavior-changing fixes are never auto-applied.
- [x] AC8: Waivers require explicit `--waive <rule-id> --reason "..."`, are recorded in spec frontmatter, and expire automatically when the file location or rule ID they were attached to changes. **Relocated by 057**: the `waivers` list is recorded in `review.md`'s frontmatter. It stays open-schema — the three raw parsers that own it keep their `#[serde(flatten)]` extras, so an organization's custom waiver fields survive the move — and the expiry rule is untouched.
- [x] AC9: Exit code is `0` when not blocking, `1` when blocking — for CI use.
- [x] AC10: The README slash commands table lists `/ductus:review` under **Pipeline (advance state)** with a one-line purpose.
- [x] AC11: **Tech-stack alignment gate**: before running review passes, `/ductus:review` confirms the project's `AGENTS.md` `Tech Stack` section exists and appears consistent with the implementation in scope. Misalignment or a missing/empty section is a blocking error, not a warning. Adopters can persist a successful check by setting `.ductus/config.toml [review] tech-stack-verified = true`, after which subsequent runs skip the check until the operator manually clears the key.
- [x] AC12: **Empty scope**: a target with an empty resolved scope (no implementation files) produces a `review.md` recording 0 findings across all five passes, `blocking: false`, and exits `0`.
- [x] AC13: **Cross-pass dedupe**: when the same finding (matching rule ID, file, and overlapping line range) is produced by more than one pass, only the highest-severity instance is retained in `must-violations` and `should-violations`; lower-severity duplicates are dropped from the counts and report.
- [x] AC14: **Flag parsing is specified and surfaced**: the command body documents how `$ARGUMENTS` is parsed for every flag in the Flags table, and `argument-hint` names each of them, so no flag is documented without being surfaced. A `--since` with no value and an unrecognized flag are each reported to the operator rather than silently absorbed. `/audit` holds `argument-hint` and the Flags table in agreement, so a flag added later cannot reopen the gap without a finding.
- [x] AC15: **A clean review states what it read**: `review.md` and the spec's `review:` block both record `examined` — how many in-scope files the passes read — against a `scope` the primitive derives itself, so a review that examined its scope and found nothing is distinguishable from one whose passes never ran. The two are byte-identical without it: same counts, same digest, same `blocking: false`. An unstated `examined` is recorded as absent rather than as zero. **Half superseded by 057.** The `examined`/`scope` half holds exactly and is unchanged — both now live in `review.md` alone, which is what the criterion's first clause asked for once the second home was removed. The enforcement half is gone: `/{project}:audit` Family 31 reported `examined: 0` over a non-empty scope and an absent `examined` on a record carrying a scope as distinct findings, and 057 retired the family with the reconciliation check it was built on. **Nothing reads `examined` now; it is read at the completion gate by a person.** That is a real loss and is recorded rather than smoothed — but the family could not have carried this claim far anyway, since one `/{project}:review` call wrote both sides from the same values, so it never compared two independent observations. This cannot prove the passes ran — an overstated numerator is as available as an omitted one — and it is not claimed to; it makes the claim explicit and checkable, which is the bar `QUAL-CLAIM-001` sets and the one `write-analysis`'s `unexamined` already cleared.

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
| `framework/templates/spec/spec.md` | edit — add `review:` block to frontmatter schema | update |
| `framework/templates/ci/adopter-generators.yml` | edit — add review-blocking check | update |
| `framework/constitution.md` | edit — reference the review gate in the pipeline section | update |
| `README.md` | edit — add `/ductus:review` row to Pipeline commands table; document waivers | update |
| `scripts/regenerate-commands.sh` (or equivalent) | run after edits to regenerate `.claude/commands/ductus/review.md` | n/a |
| `.claude/commands/ductus/review.md` | regenerated output | derived |

The template row and the CI row record what 020 changed, not what those
files hold now: 057 removed the `review:` block from `framework/templates/spec/spec.md`
and replaced the CI step's block-presence predicate.

## Tasks

1. Create `framework/commands/review.md` from the embedded artifact below.
2. Edit `framework/commands/implement.md`: before any logic that writes
   `status: done`, read the target spec's `review.blocking`. If `true` (or
   `review.last-run` is missing entirely), halt with the message specified
   in the [Blocking message](#blocking-message) section below and exit
   without modifying status.
3. Edit `framework/commands/analyze.md`: extend the audit to flag any spec
   at `status: done` with `review.blocking: true` or missing `review.last-run`
   as a validation failure. Wire `--fix` to revert affected specs from
   `done` → `in-progress` and emit a notice (never silent).
4. Edit `framework/templates/spec/spec.md` frontmatter
   to include the `review:` block schema (see [Frontmatter schema](#frontmatter-schema)).
5. Edit `framework/templates/ci/adopter-generators.yml` to add a step that
   exits non-zero if any `specs/*/spec.md` has `status: done` with
   `review.blocking: true` or missing `review.last-run`.
6. Edit `framework/constitution.md` pipeline section: add `/ductus:review`
   between `/ductus:implement` and the `done` transition, with a sentence
   explaining the blocking gate.
7. Edit `README.md`: add the `/ductus:review` row to the Pipeline (advance
   state) table; add a short Waivers subsection under Slash Commands; update
   any pipeline diagrams.
8. Run the regeneration script to produce `.claude/commands/ductus/review.md`.
9. Run `/ductus:analyze --all` against the ductus repo itself to confirm
   nothing in ductus's own specs broke.
10. Add a scenario at `specs/020-code-review/scenarios/waiver-expiry.md`
    capturing the waiver auto-expiry behavior — this is the subtlest
    requirement and warrants a focused scenario.

## Open questions

*None — all resolved.*

## Resolved questions

- **MUST waivers and second sign-off** — single-author waivers remain the
  framework standard; `/ductus:review` does not require a second `co-waived-by`
  field. Ductus has no runtime that could verify a second value belongs to a
  different person, so encoding the requirement in frontmatter would be
  performative — exactly the "depend on human diligence" anti-pattern
  §pipeline-boundaries exists to avoid. The real second review happens at
  PR-merge time, where the waiver and the unfixed finding land in the diff
  and human code review applies org policy. Adopters whose policy does
  require two-author waivers can layer fields like `co-waived-by` onto the
  `review.waivers` entries — the §text-first-artifacts open-schema rule
  guarantees `/ductus:review` and `/ductus:analyze` will not error on unknown
  fields — and gate them in their own CI.
- **Stack detection source** — `/ductus:review` continues to read `AGENTS.md`
  `Tech Stack` to choose between `security-backend.md` and
  `security-frontend.md`. `AGENTS.md` `Tech Stack` *is* the canonical sink
  for tech-stack metadata — there is no separate surface to point at. No
  change to the draft. **Premise corrected 2026-09-15:** this rested on
  spec 004 writing that section during `/ductus:init`. That command was
  retired and `004-tech-stack-selection` was retired with it, since every
  criterion it carried described init's questionnaire. The conclusion is
  unaffected — the section is still what `/ductus:review` reads, and it is
  still the only tech-stack surface.
- **Quality-pass confidence threshold** — fixed at 80; not exposed via
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
  existing `/ductus:analyze` drift check only fires when `review.blocking` is
  already `true` or `review.last-run` is missing, so rules introduced after
  the last review never re-flip the flag on shipped code. The §drift-prevention
  "done specs are frozen archaeology" rule this argued against — deleted since,
  by 023's `living-specs` scenario — applied to the spec body, never to the
  code the spec describes; that code keeps living and should stay compliant
  with current rules. Single-target `/ductus:review` already accepts
  `done` (the gate halts only when status is *not* in `{in-progress, done}`);
  `--all` simply enumerates the same set the gate already permits.

## Frontmatter schema

**Relocated by 057 — this record is `review.md`'s frontmatter, not the spec's.**
As 020 delivered it, the block below was added to `spec.md`; the example is kept
because the field set is what this spec specified, and it is re-rooted rather
than restated. The canonical declaration is
[§text-first-artifacts Frontmatter Schema](../../framework/constitution.md#frontmatter-schema) → **Audit records**, which is where the schema ships to adopters; the shape
`/ductus:review` writes is recorded field-by-field in
[data-model.md](data-model.md).

```yaml
---
spec: 020-code-review                 # the feature slug, added by the merge
last-run: 2026-05-10T14:32:00Z        # ISO 8601, set by /ductus:review
                                      # spelled reviewed-at until 057
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

The merged record also carried the fields that had lived only on `review.md`'s
side — `diff-base`, `captured-issues`, `skipped-passes` — plus `examined`,
`scope` and `reviewed-digest`, which 022 added later. No field was dropped in
the merge; where both copies carried a key, the spec block's value won, because
it was the copy every gate actually read.

> **Changed by [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md).**
> `captured-issues` is gone, with the inbox write it counted. Each observation
> is fixed, routed, or discarded with its reason in the run that finds it, and
> the record counts those outcomes in a `dispositions:` map (`fixed`, `routed`,
> `discarded`, `undispositioned`) and stores each routed or discarded decision
> in a `decisions:` list beside `waivers:`. The pre-done gate blocks an
> `in-progress` spec while the record has no map at all; an undecided
> observation does not hold `done` (058's `findings-cost-in-proportion-to-risk`). [data-model.md](data-model.md) records both fields.

When any waiver's `file` no longer exists or `rule` is no longer triggered
at that location, the waiver is dropped on the next `/ductus:review` run and
the underlying finding re-blocks if it's still present elsewhere.

## Blocking message

> **These are the texts 020 specified, not the texts emitted today**, and the
> divergence predates 057 — 022 rewrote both when the gate became
> `check-review-gate`. `/ductus:implement` now halts with `blocked: spec has {n}
> MUST violation(s) — see specs/NNN-feature/review.md`, and `/ductus:analyze`'s
> drift check reports `Review drift: done spec has unresolved MUST violations —
> see review.md` and `Review drift: done spec missing review — run
> /{project}:review`. The live texts are the command sources'
> (`framework/commands/implement.md` step 5, and `framework/commands/analyze.md`'s
> **Review state drift** section), which are canonical for command behavior; the quotes
> below are annotated rather than restated so this spec keeps the record of what
> it asked for.

Emitted by `/ductus:implement` when it would otherwise mark a spec `done`:

```text
blocked: spec NNN has N MUST violation(s) — see specs/NNN-feature/review.md

resolve the violations and re-run /ductus:review,
or run /ductus:review --waive <rule-id> --reason "..." for each waivable finding.
```

Emitted by `/ductus:analyze` when it detects drift:

```text
review-drift: spec NNN at status=done with review.blocking=true
  → revert to in-progress and re-run /ductus:review (use --fix to revert)
```

Emitted by `/ductus:review` when tech-stack alignment fails (missing/empty
`AGENTS.md` `Tech Stack` section, or documented stack inconsistent with
implementation):

```text
blocked: tech-stack alignment failed — AGENTS.md Tech Stack {missing | inconsistent with code in scope}

  expected: <stack inferred from scope, e.g., "TypeScript + React frontend">
  documented: <AGENTS.md Tech Stack contents, or "(empty)">

reconcile AGENTS.md Tech Stack with the implementation, then re-run /ductus:review.
to skip this check on future runs after manual reconciliation, add
[review] tech-stack-verified = true to .ductus/config.toml.
```

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
  eleven things in order, the analyze record among them — and its live shape is
  the command source's.
- **`framework/commands/analyze.md`** — the review-drift check, which reports a
  `done` spec whose review record is missing or blocking, and reverts it under
  `--fix`. Its grandfather rule is keyed on `review.md`'s absence since 057; it
  was keyed on the block's, which the relocation made true of every spec.

**Why the copies are gone rather than refreshed.** A snapshot inside a fenced
code block is invisible to every link check, anchor resolver and audit family,
so it rots with no signal — and it is read in full by every contributor and
every agent that opens this file, in every session, while a pointer costs one
line. This one had fallen more than half behind: 561 lines against a
`framework/commands/review.md` that had grown to 894. It was found by a person
reading the spec, which is the only instrument that has ever found one.
[§drift-prevention](../../framework/constitution.md#drift-prevention)'s
*Canonical sources* names the command source as authoritative for command
behavior and requires a pointer rather than a copy; the rule and a worked
example are `050-constitution`'s scenario
`a-canonical-source-is-pointed-at-not-copied`, cited in prose rather than
linked so the citation induces no dependency edge. Git history is where a
document's prior state lives, so nothing is lost by removing the duplicate.
