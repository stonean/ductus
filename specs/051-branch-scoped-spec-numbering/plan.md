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
- The `1234.1-slug` → `123` misparse disappears by construction: a branch-scoped form has no sequential number to return, so no caller can read one from it (AC16). Before this spec `feature_number` answered `Some(123)` for that directory, and `resolve-feature` matched on exactly that value.

The parse is a hand-rolled scan rather than a regex, matching the style of the predicate it replaces. `regex` is already a dependency (`runtime/Cargo.toml:30`), so this is a consistency choice, not an availability one.

**Grammar.** `{identifier}.{n}-{slug}`, where `identifier` and `slug` both match `^[a-z0-9]+(?:-[a-z0-9]+)*$` — the form `validate_slug` enforces (`runtime/src/primitives/mod.rs:882`) — and `n` is a positive integer with no leading zeros. The parse splits on the **first** `.`, which is unambiguous because the identifier is sanitized to a grammar that excludes `.` (AC11). A name containing no `.` takes the sequential path unchanged.

**The sequential form is three digits or more, with no padding beyond the minimum.** `1000-slug` is a feature directory; `0500-a` is not. Keeping the name/number mapping injective is the reason, not tidiness: two spellings of 500 would make `next_feature_number` and `resolve-feature` disagree.

### `create-feature` gains optional arguments rather than a sibling primitive

Branch-scoped creation is `create-feature` with `branch-id` supplied, not a second primitive. A separate `create-branch-feature` would duplicate slug derivation, template resolution, the atomic mode-preserving copy, and the already-exists refusal — and would put the two numbering rules in two places, which is the drift this plan is otherwise avoiding.

- `branch-id` absent → the existing code path, byte-for-byte (AC4).
- `branch-id` present → sanitized through `derive_slug` (`runtime/src/primitives/create_feature.rs:96`), which lowercases ASCII alphanumerics and collapses every other run to a single hyphen, yielding the `validate_slug` grammar directly (AC9, AC10). An identifier sanitizing to empty is refused through the existing `InvalidArgument` path that already refuses an empty derived slug (`:41`), before any directory is created (AC26).
- The counter is `max + 1` over the parsed `BranchScoped` forms whose identifier matches, mirroring `next_feature_number`'s `max + 1` (`:117`) — the two counters are the same rule over two filters (AC8).
- The existing-directory refusal (`created: false`) is inherited, and concurrent creation under one identifier is safe because the directory is claimed atomically: both callers compute the same number, `claim_feature_dir` (`runtime/src/primitives/create_feature.rs`) makes it with `std::fs::create_dir`, which the operating system refuses with `AlreadyExists` for every caller but the first, and the loser returns `created: false` rather than writing over the winner's `spec.md` (AC27, `BE-RACE-003`).
- `fold-into` is a second optional argument, written into the new spec's frontmatter (below).
- Every feature name a host supplies to these primitives (`create-feature`'s derived directory and `fold-into`, `rewrite-spec-links`' `from` and `to`, `retire-feature`'s `feature` and `fold-target`) passes `validate_feature_argument` before anything is opened, created or removed: the lexical screen, then canonicalization and whole-component containment, as 022's `plan.md` §Path arguments states for every feature-argument site (`BE-INPUT-004`).

The sanitized identifier is returned in the result so the calling command can echo it at the confirmation prompt before the directory exists (AC10).

**`/ductus:specify`'s branch-scoped step is prose, not a `gate-confirm`.** A gate step dispatches unconditionally on the exec path, so making it one would put a prompt in front of every sequential creation — the thing AC4 forbids. The re-blessed `specify-basic.jsonl` shows the consequence: step numbers shift by one and no new event appears, which is the evidence that the sequential walk is byte-for-byte the walk it was.

### The fold target is a declared frontmatter key

`folds-into: {feature}` on the branch-scoped spec, written at creation and **required** there: `create-feature` refuses branch-scoped creation without a target. A branch-scoped spec exists in order to be folded, so the field is never absent from one, and no renumbering path exists in code — a spec that should stand on its own is renamed by hand with the key removed.

This is safe against every existing frontmatter writer because none of them re-serializes the block — each splices its own line and leaves the rest byte-identical: `set-status` edits the `status:` line at a recorded offset (`runtime/src/primitives/set_status.rs`), `derive-dependencies` splices only the `dependencies:` key and any block-form continuation (`runtime/src/primitives/derive_dependencies.rs:207`), and `label-criteria` rewrites or inserts only `next-criterion:` (`runtime/src/primitives/label_criteria.rs:256`). So the key survives every generator untouched (AC19).

It is deliberately *not* an inline body link: `derive-dependencies` harvests sibling links into `dependencies:`, which would assert an edge that misstates the relationship (AC20). `validate-frontmatter` checked only `status` and `dependencies` (`runtime/src/primitives/validate_frontmatter.rs:69`, `:91`) and does not reject unknown keys, so the key needed no carve-out to be accepted; what it does gain is the shape check below.

**Validation checks shape, not resolvability.** The target routinely names a spec the declaring branch cannot see — a branch-scoped spec exists *because* upstream diverged, and the spec it folds into normally lives on that upstream branch, sometimes created there after this branch forked. A check requiring a resolvable target would fire on the feature's normal case, and could not see the tree that would satisfy it in any event. So the check is: `folds-into`, when present, parses as a sequential feature name (AC32) — which also forbids chaining a branch-scoped spec into another branch-scoped spec — and an unresolvable target is never a finding before the merge (AC31). Existence is enforced at fold-back by `retire-feature`, which is the first moment the two trees are joined and the only moment the answer matters (AC28).

### Feature resolution reports ambiguity rather than choosing

`resolve-feature` matches a numeric identifier against the form `parse_feature_dir` returns (`runtime/src/primitives/resolve_feature.rs`) and folds multi-match into an `Ambiguous` outcome through `classify`. With two directory forms, an identifier can legitimately name both a sequential spec and a branch namespace — `051` against `051-slug` and `051.1-slug`. Matching stays exact per form and both matches are returned, so the existing `Ambiguous` outcome reports the collision instead of a rule silently preferring one form (AC16).

### Fold-back is a command over deterministic primitives

`/ductus:fold` follows `/ductus:groom`'s shape: a semantic extension point decides, per branch-scoped spec, whether its content becomes a body edit or a scenario on the upstream spec, and primitives perform every write. The new extension point is distinct from `routeInboxItem` — that vocabulary is a closed five-route set for inbox items (`runtime/src/interpreter/payload.rs:540`) and does not describe this choice. That set answers *where in the corpus does this work belong*; a fold has already been told where, by `folds-into`, and widening the inbox vocabulary would break the closedness its other callers depend on.

Three new primitives carry the deterministic half; the rest is existing ones (`create-scenario`, `append-task`, `set-status`, `read-spec`):

- `rewrite-spec-links` — re-point every inbound pointer to a retiring or renamed feature directory: body links across the corpus, any `folds-into` field naming it (AC33), and the spec-root paths in the referrers `check-orphaned-references` reads outside the corpus (`CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit`; task 29), found with that check's own scanner. The frontmatter key is included deliberately — it is the one pointer whose job is to survive until the merge, so a rename that repaired body links and left it behind would break exactly the thing the field exists for. The derived frontmatter needs no separate handling: `derive-dependencies` and `derive-references` regenerate `dependencies:` and `references:` from body links on the next commit through the pre-commit hook (`.githooks/pre-commit:68`), so fixing the body links fixes the indexes (AC23). It matches by whole path segment, not a fixed `../` prefix — that is what makes a link from a scenario (`../../050-a/spec.md`) work without a second rule and stops `1234.1-staged` matching inside `1234.1-staged-cache`. A scenario-targeted `to` re-points a reference to one of the retiring spec's scenarios at the scenario fold step 7 carried under the same slug, keeping its `#fragment`, and replaces every other tail with the routed scenario, dropping its fragment, because those files did not survive individually. Cross-service URLs are skipped. Every referrer, `system.md` included, is read before anything is written, and one it cannot safely rewrite — a symbolic link, one whose parent resolves outside the project, or one that is not UTF-8 text — is named in the result's `skipped` with its reason, for `check-orphaned-references` to report what it leaves. A feature-only `to` keeps each reference's tail, which is right for `spec.md` and for `scenarios/{slug}.md`, carried under the same slug; a reference that keeps any other tail lands on the target's file of that name, which the fold did not write, so it is listed in `kept-tails` for the operator to check.
- `retire-feature` — remove the branch-scoped directory, refusing when the fold target does not exist so a retirement can never strand content (AC28). This is where the target's existence is finally enforced, because fold-back runs *after* the merge, on the upstream branch — the first tree in which both specs exist. Its refusals are errors while already-gone is an outcome: a refusal means the call should not have been made and a walker must stop, while a missing directory means a previous run finished that spec, so a re-run converges. A target directory holding no `spec.md` does not satisfy the existence check — it is not a home content can have landed in.
- `check-unfolded-specs` — report branch-scoped directories present in the working tree, with their `folds-into` targets, for `/ductus:analyze` to surface (AC21). It halts on a branch-scoped directory with no readable `spec.md` rather than skipping it: the check's entire output is what each spec declares, so a row it cannot honestly fill would understate the backlog it exists to report. `/ductus:analyze`'s finding is advisory and stays advisory, because nothing in the tree records that a branch merged, so the check cannot separate an open branch from a merged one; promoting it would fire on every open branch.

A declared fold target is **owed work that never holds `done`**. `dashboard` qualifies the Status cell of a spec carrying `folds-into` as `{status} (fold pending)` at every status, keeping the frontmatter status beside the qualification because `in-progress` and `done` are different situations for the operator: the first still has its own pipeline to finish, the second has only the fold left (AC34). The Next Action follows the order *done first, then fold* — below `done` the status-driven action keeps the cell, and at `done` the fold takes it — and an unresolved scenario question still outranks the fold, since content gets settled before it gets moved. `check-review-gate` does not read `folds-into` at all (AC35, scenario `a-pending-fold-never-holds-done`): a staging spec completes its own pipeline like any other, and the owed fold is carried past `done` by `dashboard` and `check-unfolded-specs`. Neither surface gates on whether the fold target resolves, because before the merge the target normally lives on the branch this tree forked from and a resolvability check would refuse the feature's normal case; the dashboard reports the absence, and `retire-feature` enforces it.

Fold-back processes one branch-scoped spec at a time (AC29): each spec's fold completes its writes and its retirement before the next begins, so an interruption leaves every spec other than the one in flight untouched. The one in flight is **recoverable, not atomic**. The runtime has no multi-file transaction, so every per-spec write is a no-op where a previous run already landed — `append-task`'s scenario-pointer dedup provides that for the one step that lacked it — and the recovery is a second run rather than a rollback. Claiming per-spec all-or-nothing would have been claiming a transaction.

After retiring the directory, `/ductus:fold` re-points every session in the working tree that named the folded spec at the upstream spec, through `retarget-sessions` (spec 062); without it every follow-on command in those sessions lands on a path that is gone. The source spec's own scenarios cross over on *either* route — the route decides the body's shape, never its scenarios'.

`check-orphaned-references` is the verifier, not the repair path. Its report-don't-repair stance rests on a migration knowing only its own hop (`runtime/src/primitives/check_orphaned_references.rs:8`, `:16`); fold-back knows both endpoints, so it rewrites and the check confirms the result is clean (AC22).

### Reopening the upstream spec uses the existing back-edge

Folding into a `done` spec calls `set-status` with `from: done, to: in-progress` — the same guarded call `/ductus:groom` and `/ductus:amend` already make for the scenario back-edge. The `from` guard surfaces a concurrent edit as an operational error rather than a silent overwrite. No new edge is added to §spec-lifecycle (AC24, AC25); a second fold into an already-reopened spec finds it `in-progress` and leaves the status alone. An upstream spec that is not `done` is likewise untouched.

**The fold invalidates the upstream spec's review explicitly** rather than relying on the staleness check. That check reads durable contracts, and a body-edit fold writes only `spec.md`, which it deliberately excludes; the fold knows what the diff cannot see, so it says so. Waivers survive the invalidation: it asserts the review is out of date, not that an operator's judgement about a finding was withdrawn.

### Documentation and registration changes

Adding a directory form and a command touches the framework's own registries, and the audit families check exactly these agreements:

- `framework/constitution.md` §numbering (`:676`) states both forms; §spec-lifecycle (`:135`) records that a branch-scoped spec is a staging form discharged by fold-back (AC17).
- `framework/commands/fold.md` is the new command source. It must parse under the procedure parser, which asserts every command's step numbering (`runtime/src/parser/mod.rs:1165`), and its name joins that test's command list.
- `framework/runtime-tools.txt` gains the three new primitive names — `scripts/lint-tool-coverage.sh` verifies that every tool a command references is one the runtime exposes.
- Installer and help surfaces: `scripts/gen-help-tables.sh`, `scripts/gen-claude-commands.sh`, and the bootstrap manifests under `framework/bootstrap/`, checked by `scripts/audit/installer-command-parity.sh` and `installer-registry-parity.sh`. `framework/bootstrap/govern.md` is held byte-identical to `ductus.md` by audit Family 21, so the `fold.md` installer row was copied across, and any future manifest edit needs the same.

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
- **A run-spanning transaction for fold-back** — rejected: the runtime provides per-file atomic writes, not multi-file transactions, and per-spec recovery by re-running is both achievable and sufficient, since a spec is the unit an operator would retry.
- Alternatives rejected during clarification — a committed mode setting, a git-history or persisted-file monotonic counter, a body link as the fold target, a redirect stub, manual fold-back, and rejecting non-conforming identifiers — are recorded with their reasoning in the spec's Resolved Questions and are not restated here.

### Known limitations

- **A retired `.{n}` number is reusable.** In-repo links are re-pointed before retirement, so the exposure is references outside the repository — a pull-request comment, a commit message, a ticket. Accepted; no in-repo counter can govern those.
- **A fold target is unverifiable until the merge.** Nothing before fold-back can tell a correct target from a typo, because the tree that would settle it is on another branch. A mistyped target survives the whole branch and surfaces as a fold-back refusal.
- **Fold-back is not automatic at merge.** Nothing runs on the merge itself; the detection check reports surviving branch-scoped directories after the fact. This is deliberate — fold-back is a reviewed step — but it means an un-folded spec is visible only once someone runs the check.
- **Phase 1 ships a form the framework can hold but not yet discharge.** Between phases 1 and 3, fold-back is a manual procedure. Sequencing the phases the other way is not possible, since fold-back's writes depend on the grammar and the field.
- **`specs/system.md` does not exist in this repository** (`ls specs/system.md` finds none, 2026-10-28), so the cross-validation against shared architecture conventions that the plan phase normally performs had nothing to read here. The framework's own equivalent — the constitution and the audit families — was used instead.
