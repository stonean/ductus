# 051 — Branch-scoped spec numbering Plan

Implements [051 — Branch-scoped spec numbering](spec.md).

## Overview

The feature divides cleanly into three phases, each independently useful and each landing behind the previous one:

1. **Numbering** — a second directory form the corpus can hold and every surface can see. One parse function, one widened predicate, one optional argument on `create-feature`.
2. **The fold target** — a declared frontmatter field recording which upstream spec a branch-scoped spec stands in for, plus the detection check that flags specs which outlive their branch.
3. **Fold-back** — the command that discharges a branch-scoped spec into its upstream home, re-points inbound links, and retires the directory.

Phase 1 alone makes branch-scoped specs usable, with fold-back performed by hand; phases 2 and 3 mechanize the discharge. The phases are sequenced this way because phase 3's writes depend on phase 2's field, which depends on phase 1's grammar.

The governing constraint throughout: the three-digit convention is enforced in exactly one predicate, and every corpus-reading surface goes through it. Widening that one place is what makes branch-scoped directories visible everywhere at once; adding a second copy of the rule anywhere is how these generators drifted before (`runtime/src/primitives/mod.rs:1449`).

## Technical Decisions

### One directory-form parse, replacing the boolean predicate

`is_feature_slug` (`runtime/src/primitives/mod.rs:1424`) is a byte scan asserting three ASCII digits then `-`, and `feature_number` (`:1438`) parses the first three bytes independently of it. Both are replaced by a single `parse_feature_dir(name) -> Option<FeatureForm>` returning either `Sequential { number }` or `BranchScoped { identifier, n }`; `is_feature_slug` becomes `parse_feature_dir(name).is_some()`, and the callers that want a sequential number ask the parsed form for one.

Two consequences are the point of doing it this way rather than adding a second predicate:

- `list_feature_dirs` (`:1582`) and `is_spec_path` (`:1454`) are the only filesystem entry points, so both directory forms become visible to `dashboard`, `resolve-feature`, `create-feature`, `derive-dependencies`, and `derive-references` in one change (AC5, AC15).
- The `1234.1-slug` → `123` misparse disappears by construction: a branch-scoped form has no sequential number to return, so no caller can read one from it (AC16). Today `feature_number` would answer `Some(123)` for that directory, and `resolve-feature` (`runtime/src/primitives/resolve_feature.rs:107`) matches on exactly that value.

The parse is a hand-rolled scan rather than a regex, matching the style of the predicate it replaces. `regex` is already a dependency (`runtime/Cargo.toml:30`), so this is a consistency choice, not an availability one.

**Grammar.** `{identifier}.{n}-{slug}`, where `identifier` and `slug` both match `^[a-z0-9]+(?:-[a-z0-9]+)*$` — the form `validate_slug` enforces (`runtime/src/primitives/mod.rs:882`) — and `n` is a positive integer with no leading zeros. The parse splits on the **first** `.`, which is unambiguous because the identifier is sanitized to a grammar that excludes `.` (AC11). A name containing no `.` takes the sequential path unchanged.

### `create-feature` gains optional arguments rather than a sibling primitive

Branch-scoped creation is `create-feature` with `branch-id` supplied, not a second primitive. A separate `create-branch-feature` would duplicate slug derivation, template resolution, the atomic mode-preserving copy, and the already-exists refusal — and would put the two numbering rules in two places, which is the drift this plan is otherwise avoiding.

- `branch-id` absent → the existing code path, byte-for-byte (AC4).
- `branch-id` present → sanitized through `derive_slug` (`runtime/src/primitives/create_feature.rs:96`), which lowercases ASCII alphanumerics and collapses every other run to a single hyphen, yielding the `validate_slug` grammar directly (AC9, AC10). An identifier sanitizing to empty is refused through the existing `InvalidArgument` path that already refuses an empty derived slug (`:41`), before any directory is created (AC26).
- The counter is `max + 1` over the parsed `BranchScoped` forms whose identifier matches, mirroring `next_feature_number`'s `max + 1` (`:117`) — the two counters are the same rule over two filters (AC8).
- The existing-directory refusal (`created: false`, `:58`) is inherited unchanged, which is also what makes concurrent creation under one identifier safe: both callers compute the same number and the loser is refused rather than overwriting (AC27).
- `fold-into` is a second optional argument, written into the new spec's frontmatter (below).

The sanitized identifier is returned in the result so the calling command can echo it at the confirmation prompt before the directory exists (AC10).

### The fold target is a declared frontmatter key

`folds-into: {feature}` on the branch-scoped spec, written at creation and **required** there: `create-feature` refuses branch-scoped creation without a target. A branch-scoped spec exists in order to be folded, so the field is never absent from one, and no renumbering path exists in code — a spec that should stand on its own is renamed by hand with the key removed.

This is safe against every existing frontmatter writer because none of them re-serializes the block — each splices its own line and leaves the rest byte-identical: `set-status` edits the `status:` line at a recorded offset (`runtime/src/primitives/set_status.rs`), `derive-dependencies` splices only the `dependencies:` key and any block-form continuation (`runtime/src/primitives/derive_dependencies.rs:207`), and `label-criteria` rewrites or inserts only `next-criterion:` (`runtime/src/primitives/label_criteria.rs:256`). So the key survives every generator untouched (AC19).

It is deliberately *not* an inline body link: `derive-dependencies` harvests sibling links into `dependencies:`, which would assert an edge that misstates the relationship (AC20). `validate-frontmatter` checks only `status` and `dependencies` (`runtime/src/primitives/validate_frontmatter.rs:69`, `:91`) and does not reject unknown keys, so no carve-out is needed.

**Validation checks shape, not resolvability.** The target routinely names a spec the declaring branch cannot see — a branch-scoped spec exists *because* upstream diverged, and the spec it folds into normally lives on that upstream branch, sometimes created there after this branch forked. A check requiring a resolvable target would fire on the feature's normal case, and could not see the tree that would satisfy it in any event. So the check is: `folds-into`, when present, parses as a sequential feature name (AC32) — which also forbids chaining a branch-scoped spec into another branch-scoped spec — and an unresolvable target is never a finding before the merge (AC31). Existence is enforced at fold-back by `retire-feature`, which is the first moment the two trees are joined and the only moment the answer matters (AC28).

### Feature resolution reports ambiguity rather than choosing

`resolve-feature` matches a numeric identifier against `feature_number` (`runtime/src/primitives/resolve_feature.rs:107`) and folds multi-match into an `Ambiguous` outcome through `classify` (`:122`). With two directory forms, an identifier can legitimately name both a sequential spec and a branch namespace — `051` against `051-slug` and `051.1-slug`. Matching stays exact per form and both matches are returned, so the existing `Ambiguous` outcome reports the collision instead of a rule silently preferring one form (AC16).

### Fold-back is a command over deterministic primitives

`/ductus:fold` follows `/ductus:groom`'s shape: a semantic extension point decides, per branch-scoped spec, whether its content becomes a body edit or a scenario on the upstream spec, and primitives perform every write. The new extension point is distinct from `routeInboxItem` — that vocabulary is a closed five-route set for inbox items (`runtime/src/interpreter/payload.rs:540`) and does not describe this choice.

Three new primitives carry the deterministic half; the rest is existing ones (`create-scenario`, `append-task`, `set-status`, `read-spec`):

- `rewrite-spec-links` — re-point every inbound pointer to a retiring or renamed feature directory: body links across the corpus **and** any `folds-into` field naming it (AC33). The frontmatter key is included deliberately — it is the one pointer whose job is to survive until the merge, so a rename that repaired body links and left it behind would break exactly the thing the field exists for. The derived frontmatter needs no separate handling: `derive-dependencies` and `derive-references` regenerate `dependencies:` and `references:` from body links on the next commit through the pre-commit hook (`.githooks/pre-commit:68`), so fixing the body links fixes the indexes (AC23).
- `retire-feature` — remove the branch-scoped directory, refusing when the fold target does not exist so a retirement can never strand content (AC28). This is where the target's existence is finally enforced, because fold-back runs *after* the merge, on the upstream branch — the first tree in which both specs exist. This is where the target's existence is finally enforced, because fold-back runs *after* the merge, on the upstream branch, which is the first tree where both exist.
- `check-unfolded-specs` — report branch-scoped directories present in the working tree, with their `folds-into` targets, for `/ductus:analyze` to surface (AC21).

A declared fold target is also **outstanding work**, not merely a recorded intent, so it participates in the two surfaces that report outstanding work: `dashboard` reports a spec carrying `folds-into` as pending rather than `done`, and `check-review-gate` blocks `in-progress → done` while it is present. The gate placement mirrors the unresolved-scenario-questions check it sits beside — both say the same thing, that a spec with undischarged obligations is not complete. The consequence is that the branch-scoped form has no `done` state at all: it is retired, not completed (AC34, AC35, AC36).

Atomicity is per branch-scoped spec (AC29): each spec's fold completes its writes and its retirement before the next begins, so an interruption leaves every spec either fully folded or untouched. This is the granularity the existing primitives already give — each is individually atomic — rather than a transaction spanning the whole run, which nothing in the runtime provides.

`check-orphaned-references` is the verifier, not the repair path. Its report-don't-repair stance rests on a migration knowing only its own hop (`runtime/src/primitives/check_orphaned_references.rs:8`, `:16`); fold-back knows both endpoints, so it rewrites and the check confirms the result is clean (AC22).

### Reopening the upstream spec uses the existing back-edge

Folding into a `done` spec calls `set-status` with `from: done, to: in-progress` — the same guarded call `/ductus:groom` and `/ductus:amend` already make for the scenario back-edge. The `from` guard surfaces a concurrent edit as an operational error rather than a silent overwrite. No new edge is added to §spec-lifecycle (AC24, AC25); a second fold into an already-reopened spec finds it `in-progress` and leaves the status alone. An upstream spec that is not `done` is likewise untouched.

### Documentation and registration changes

Adding a directory form and a command touches the framework's own registries, and the audit families check exactly these agreements:

- `framework/constitution.md` §numbering (`:676`) states both forms; §spec-lifecycle (`:135`) records that a branch-scoped spec is a staging form discharged by fold-back (AC17).
- `framework/commands/fold.md` is the new command source. It must parse under the procedure parser, which asserts every command's step numbering (`runtime/src/parser/mod.rs:1165`), and its name joins that test's command list.
- `framework/runtime-tools.txt` gains the three new primitive names — `scripts/lint-tool-coverage.sh` verifies that every tool a command references is one the runtime exposes.
- Installer and help surfaces: `scripts/gen-help-tables.sh`, `scripts/gen-claude-commands.sh`, and the bootstrap manifests under `framework/bootstrap/`, checked by `scripts/audit/installer-command-parity.sh` and `installer-registry-parity.sh`.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/mod.rs` | Modify | `parse_feature_dir` + `FeatureForm`; redefine `is_feature_slug`, retire `feature_number` in favour of the parsed form; `feature_dir_cmp` for the mixed corpus; `is_slug_grammar` factored out of `validate_slug` |
| `runtime/src/primitives/dashboard.rs` | Modify | **Not predicted.** A third `feature_number` caller: the Dependencies column would have rendered a branch-scoped dependency as its first three digits |
| `runtime/tests/mixed_corpus.rs` | Create | Both forms seen by every corpus reader; the fold target's survival through every frontmatter writer |
| `runtime/src/primitives/create_feature.rs` | Modify | `branch-id` / `fold-into` arguments, branch-scoped counter, sanitized-identifier result field |
| `runtime/src/primitives/resolve_feature.rs` | Modify | Per-form matching; ambiguity across forms |
| `runtime/src/primitives/validate_frontmatter.rs` | Modify | `folds-into` shape check — parse only, never resolvability |
| `runtime/src/primitives/rewrite_spec_links.rs` | Create | Re-point inbound body links and `folds-into` fields at the fold target |
| `runtime/src/primitives/retire_feature.rs` | Create | Remove a folded branch-scoped directory, guarded on the target existing |
| `runtime/src/primitives/check_unfolded_specs.rs` | Create | Report surviving branch-scoped directories |
| `runtime/src/schema/primitives.rs` | Modify | Argument and result shapes for the above |
| `runtime/src/schema/extensions.rs` | Modify | The fold-routing extension point's request/response types |
| `runtime/src/interpreter/payload.rs` | Modify | Payload builder for the new extension point |
| `runtime/src/mcp/server.rs` | Modify | Register the three new tools |
| `runtime/src/parser/mod.rs` | Modify | Add `fold` to the command-parse test list |
| `framework/commands/fold.md` | Create | The fold-back command source |
| `framework/commands/specify.md` | Modify | Branch-scoped creation path and the identifier confirmation prompt |
| `framework/commands/analyze.md` | Modify | Surface `check-unfolded-specs` findings |
| `framework/constitution.md` | Modify | §numbering second form; §spec-lifecycle staging reconciliation |
| `framework/runtime-tools.txt` | Modify | Register the three new tool names |
| `framework/templates/spec/spec.md` | Modify | Document the optional `folds-into` frontmatter key |
| `scripts/gen-help-tables.sh`, `scripts/gen-claude-commands.sh` | Modify | Regenerate command surfaces for `/ductus:fold` |
| `framework/bootstrap/*` | Modify | Install the new command for each agent host |

## Trade-offs

### Considered and rejected

- **A `create-branch-feature` sibling primitive** — rejected for duplicating slug derivation, template resolution, the atomic copy, and the already-exists refusal, and for putting the two numbering rules in two places.
- **A second membership predicate alongside `is_feature_slug`** — rejected for the same reason the shell generators were consolidated: a duplicated membership rule is how they drifted (`runtime/src/primitives/mod.rs:1449`).
- **Preferring one directory form when an identifier matches both** — rejected in favour of the existing `Ambiguous` outcome. A silent preference is a wrong answer the operator cannot see.
- **A run-spanning transaction for fold-back** — rejected: the runtime provides per-file atomic writes, not multi-file transactions, and per-spec atomicity is both achievable and sufficient, since a spec is the unit an operator would retry.
- Alternatives rejected during clarification — a committed mode setting, a git-history or persisted-file monotonic counter, a body link as the fold target, a redirect stub, manual fold-back, and rejecting non-conforming identifiers — are recorded with their reasoning in the spec's Resolved Questions and are not restated here.

### Known limitations

- **A retired `.{n}` number is reusable.** In-repo links are re-pointed before retirement, so the exposure is references outside the repository — a pull-request comment, a commit message, a ticket. Accepted; no in-repo counter can govern those.
- **A fold target is unverifiable until the merge.** Nothing before fold-back can tell a correct target from a typo, because the tree that would settle it is on another branch. A mistyped target survives the whole branch and surfaces as a fold-back refusal.
- **Fold-back is not automatic at merge.** Nothing runs on the merge itself; the detection check reports surviving branch-scoped directories after the fact. This is deliberate — fold-back is a reviewed step — but it means an un-folded spec is visible only once someone runs the check.
- **Phase 1 ships a form the framework can hold but not yet discharge.** Between phases 1 and 3, fold-back is a manual procedure. Sequencing the phases the other way is not possible, since fold-back's writes depend on the grammar and the field.
- **`specs/system.md` does not exist in this repository**, so the cross-validation against shared architecture conventions that the plan phase normally performs had nothing to read here. The framework's own equivalent — the constitution and the audit families — was used instead.

## Implementation notes

Recorded during implementation. All 21 tasks are complete and committed; this section is the record of what shipped and why, not a handoff. It carries what the task checkboxes cannot.

### What shipped

| Tasks | Commits |
| --- | --- |
| 1–6 | `d71c4ff`, `b136a94`, `508b48d`, `32ba568`, `e86f612` |
| 7 (split across two) | `75a0cfd` (primitive + registration), `2170c4f` (dashboard + gate) |
| 8–12 | `b145136`, `542c9f1`, `6e5662b`, `6a92d3c`, `ace609e` |
| 13–14 | `3d1adb2` (constitution), `238e0ca` (`/ductus:specify`) |
| 15–19 | `c776f09`, `0daf542`, `ea93425`, `599f0ef`, `9be5b00` |
| 20–21 | `1657a4d` |

**Tasks 14–21 were added during implementation**, each because something the original breakdown did not cover turned out to be load-bearing. Task 14 because the plan's Affected Files named `specify.md` while no task touched it, leaving four criteria unowned. Tasks 15–16 because task 7 shipped gate and pipeline-view behavior without documenting it in the command sources §drift-prevention makes canonical. Task 17 because 022's data model enumerated a closed extension-point set that `routeFold` had already joined. Task 18 to discharge an inbox defect rather than route it. Task 19 from the security pass of the review. Tasks 20–21 because AC24 and AC29 claimed more than the implementation delivered, and the gate would not tick them.

### Write boundary

The work spanned more than the derived boundary throughout. `derive-boundary` grows only as commits land, so it reports a subset of what a session needs; the later tasks additionally needed `framework/constitution.md`, `framework/commands/**`, and `scripts/**`. The boundary is not seedable from the session file — `write-boundary` is an exec-walker context value, not a `session.toml` key — so **the grant has to come from the operator each session that needs it.** Ask for it once at the start rather than halting mid-task.

### Verification each task is held to

Run all of these before calling a task done, from the repo root unless noted:

- `cargo test` and `cargo clippy --all-targets` (under `runtime/`) — clippy must be silent, not merely non-erroring; `unwrap_used` and `expect_used` are `warn`, so a new test module needs `#![allow(clippy::unwrap_used, clippy::expect_used)]` like every sibling.
- `npx markdownlint-cli2`
- `scripts/audit/run-all.sh` — must exit 0. Its `check-zero` precondition compares the installed command copies against their sources, so **run `scripts/gen-claude-commands.sh` first** after any `framework/commands/**` edit or the audit fails on drift the pre-commit hook would have fixed anyway.
- `scripts/lint-tool-coverage.sh`
- `cargo fmt` before committing — the pre-commit hook runs rustfmt plus the full suite and rejects on a formatting diff.

Commit after each task, with a message explaining *why*, not just what.

### Sequencing already applied

Task 3 wrote the `folds-into` key rather than leaving it to task 6, because accepting a `fold-into` argument without persisting it would have left a knowingly-broken tree between the two. Task 6 kept the validation, the template documentation, and the round-trip tests.

Task 4 needed only its second test case: the restriction to `Sequential` landed with task 1's parse, and its first case was already covered by task 3.

Task 7 absorbed the pipeline-view requirement (AC34–AC36) when it arrived, and was landed as two commits against one checkbox — the primitive and its registration, then the two reporting surfaces. The seam held; if a later task grows the same way, the same split is available without renumbering anything.

### Decisions the checkboxes do not carry

These were judgement calls, not transcriptions of the plan. A later change that contradicts one is reopening a decision, not fixing an oversight.

- **`check-unfolded-specs` halts on a branch-scoped directory with no readable `spec.md`** rather than skipping it. The check's entire output is what each spec declares, so a row it cannot honestly fill would understate the backlog it exists to report.
- **`dashboard` keeps the frontmatter status beside the qualification** — `done (fold pending)`, not `fold pending` — because `in-progress` and a hand-edited `done` are different situations for the operator even though both are held short of complete. The Next Action cell is handed to the fold for every status; an unresolved scenario question still outranks it, since content gets settled before it gets moved.
- **Neither `dashboard` nor `check-review-gate` checks that the fold target resolves.** That silence is load-bearing: before the merge the target normally lives on the branch this tree forked from, so a resolvability check would refuse the feature's normal case. The dashboard *reports* the absence on the row; `retire-feature` enforces it.
- **`/ductus:analyze`'s finding is advisory and the doc says it stays advisory**, with the reason attached: nothing in the tree records that a branch merged, so the check cannot separate an open branch from a merged one. Promoting it would fire on every open branch.
- **`rewrite-spec-links` matches by whole path segment**, not a fixed `../` prefix — that is what makes a link from a scenario (`../../050-a/spec.md`) work without a second rule and stops `1234.1-staged` matching inside `1234.1-staged-cache`. A scenario-targeted `to` replaces each link's tail and drops any `#fragment`, because the retiring directory's files did not survive individually. Cross-service URLs are skipped. The scan is scoped to `.md` under the spec root — a link from `README.md` is outside it, and `check-orphaned-references` is what would report that.
- **`retire-feature`'s refusals are errors while already-gone is an outcome.** A refusal means the call should not have been made and a walker must stop; a missing directory means a previous run finished that spec, so a re-run converges. A target directory holding no `spec.md` does not satisfy the existence check — it is not a home content can have landed in.
- **`routeFold` is a separate extension point from `routeInboxItem`**, deliberately. That set answers *where in the corpus does this work belong*; a fold has already been told where. Widening the inbox vocabulary would break the closedness its other callers depend on.
- **`/ductus:fold` re-targets the session after retiring the directory** (step 12). Without it every follow-on command lands on a path that is gone. The source spec's own scenarios cross over on *either* route — the route decides the body's shape, never its scenarios'.
- **`framework/bootstrap/govern.md` is held byte-identical to `ductus.md`** by audit Family 21. Adding the `fold.md` installer row meant copying the file across; any future manifest edit needs the same.
- **`/ductus:specify`'s branch-scoped step is prose, not a `gate-confirm`.** A gate step dispatches unconditionally on the exec path, so making it one would have put a prompt in front of every sequential creation — the thing AC4 forbids. The re-blessed `specify-basic.jsonl` shows the consequence: step numbers shift by one and no new event appears, which is the evidence that the sequential walk is byte-for-byte the walk it was.
- **The sequential form accepts three digits *or more*, but rejects padding beyond the minimum.** `1000-slug` is a feature directory; `0500-a` is not. Keeping the name/number mapping injective is the reason — two spellings of 500 would make `next_feature_number` and `resolve-feature` disagree — not tidiness.
- **A fold invalidates the upstream spec's review explicitly rather than relying on the staleness check.** That check reads durable contracts, and a body-edit fold writes only `spec.md`, which it deliberately excludes. The fold knows what the diff cannot see, so it says so. Waivers survive the invalidation: it asserts the review is out of date, not that an operator's judgement about a finding was withdrawn.
- **An interrupted fold is *recoverable*, not atomic, and the command says so.** The runtime has no multi-file transaction. Every per-spec write is a no-op where a previous run already landed — which is what `append-task`'s new scenario-pointer dedup provides for the one step that lacked it — so the recovery is a second run rather than a rollback. Claiming per-spec all-or-nothing would have been claiming a transaction.

### How the completion gate was cleared

Three things stood between the last task and `done`, and each is recorded here because how it was settled is a decision, not a formality.

1. **The spec had never been reviewed**, so `check-review-gate` blocked with `not reviewed`. Reviewed after the last code task rather than before — a verdict recorded against a tree still being changed is stale the moment the next task lands, which is exactly what happened once anyway: the first pass was recorded at `6efa502`, tasks 20–21 then moved `data-model.md`, and the review was re-run (`8fd2c7a`) rather than caveated.
2. **All 34 acceptance criteria were unchecked.** They were verified individually and marked by label. Two did not pass on the first walk: **AC24** (a body-edit fold left the upstream spec's review stale, because the gate's staleness check excludes `spec.md`) and **AC29** (per-primitive atomicity is not the per-spec all-or-nothing the criterion claimed). Both became tasks 21 and 20 rather than being ticked on a generous reading.
3. **Spec 022's `data-model.md` did not document `routeFold`.** The row was added (`ea93425`) — a deliberate **cross-spec write** the plan's Affected Files did not sanction, so it is named in its commit rather than left to surface at the gate as an unexplained sibling-spec change (§cross-spec-impact). It made 022's own review stale, which was cleared by re-reviewing 022 (`6efa502`).

The defect captured to `specs/inbox.md` during this feature — a spec numbered past 999 gets a name `parse_feature_dir` rejects, because `create-feature`'s `{number:03}` pad is a minimum width and the predicate demanded exactly three digits — was fixed here rather than routed (task 18, `599f0ef`), and its inbox bullet removed. Two observations from the reviews remain in the inbox for `/ductus:groom`: `rewrite-spec-links` does not preserve CRLF line endings where two siblings deliberately do, and `/ductus:fold` rewrites corpus-wide links before the step that enforces the fold target exists.

### Conventions this feature established

`parse_feature_dir` is the only place a feature directory is recognized, and `feature_dir_cmp` the only place the mixed corpus is ordered. Both are recorded in `AGENTS.md` under Gotchas, along with clap's `{n}` expansion in `--help` text (which bit twice while documenting the new arguments) and the sites a new command has to be registered at.
