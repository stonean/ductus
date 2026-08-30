# 051 — Branch-scoped spec numbering Tasks

Tasks derived from the [plan](plan.md). Complete in order.

Phase 1 (tasks 1–5) makes branch-scoped directories creatable and visible. Phase 2 (6–8) adds the fold target and its detection. Phase 3 (9–13) adds the fold-back command. Each phase leaves the corpus in a working state.

**All 21 tasks are complete.** Tasks 14–21 were added during implementation — command-level work the original breakdown missed, documentation the runtime had outgrown, a captured defect, and the two acceptance criteria that turned out to claim more than the code delivered. The plan's **Implementation notes** section records which commit landed each, why each was added, and the decisions these checkboxes cannot express.

## 1. Introduce the directory-form parse

- [x] Add `FeatureForm` and `parse_feature_dir` to `runtime/src/primitives/mod.rs` per the [data model](data-model.md) grammar
- [x] Redefine `is_feature_slug` in terms of it and remove `feature_number`, updating its two callers (`resolve_feature.rs`, `create_feature.rs`) to match on `Sequential`
- [x] Unit-test both forms plus the rejections: no `.`-free branch form, leading-zero `n`, empty identifier, empty slug, uppercase input, `1234.1-slug` yielding no sequential number

- **Done when**: `parse_feature_dir` is the only place either directory form is recognized, `feature_number` no longer exists, and the existing `is_feature_slug` acceptance and rejection tests still pass unchanged.

## 2. Make both forms visible to every corpus reader

- [x] Confirm `list_feature_dirs` and `is_spec_path` accept both forms through the new parse
- [x] Order branch-scoped directories by identifier then `n`, sequential ones by number, with a defined total order across the mixed corpus
- [x] Add tests over a fixture spec root holding both forms, asserting `dashboard`, `derive-dependencies`, and `derive-references` each see the branch-scoped directory

- **Done when**: a spec root containing `050-a` and `1234.1-b` yields both directories from every corpus-reading surface, in a stable documented order (AC5, AC15).

## 3. Add branch-scoped creation to `create-feature`

- [x] Add the `branch-id` and `fold-into` arguments and the `identifier` result field to the schema
- [x] Require the fold target: branch-scoped creation without one is refused, and no "no target" path exists
- [x] Sanitize `branch-id` through `derive_slug`; refuse an identifier that sanitizes to empty through the existing `InvalidArgument` path
- [x] Compute `.{n}` as `max + 1` over `BranchScoped` forms matching the sanitized identifier
- [x] Test: first and second spec under one identifier; identifiers differing only in case landing in one namespace; `PROJ-1111` and `1111-PROJ` sanitizing as specified; an identifier containing `.`; an existing directory returning `created: false`

- **Done when**: creation with no `branch-id` produces byte-identical behavior to today, creation with one produces `{identifier}.{n}-{slug}` with the sanitized identifier returned, and a branch-scoped spec cannot be created without the operator having chosen a fold target or explicitly declined one (AC1, AC8, AC9, AC10, AC11, AC12, AC26, AC27, AC30).

## 4. Keep the sequential counter independent

- [x] Restrict `next_feature_number` to `Sequential` forms
- [x] Test: a spec root of `050-a` plus `1234.1-b` yields `051-…` next; a root holding only branch-scoped directories yields `001-…`

- **Done when**: no branch-scoped directory can influence the sequential counter (AC2, AC3, AC4, AC14).

## 5. Teach feature resolution both forms

- [x] Match sequential identifiers against `Sequential` numbers and branch identifiers against `BranchScoped` identifiers, exactly per form
- [x] Return every match so a string naming both forms produces the existing `Ambiguous` outcome
- [x] Test: `123` against `123-a` and `1234.1-b`; `1234` against the branch set; `051` against both `051-a` and `051.1-b`

- **Done when**: resolving `123` never matches `1234.1-…`, and an identifier naming both forms is reported ambiguous rather than silently resolved (AC16).

## 6. Add the `folds-into` frontmatter field

- [x] Write `folds-into:` from `create-feature`'s `fold-into` argument; omit the key when the argument is absent
- [x] Add the `validate-frontmatter` shape check: present ⇒ *parses* as a sequential feature name. Do not check that the feature exists — the target normally lives on the upstream branch and is absent here
- [x] Test that a `folds-into` naming a feature absent from the corpus produces no finding, and that one naming a branch-scoped feature does
- [x] Document the key in `framework/templates/spec/spec.md`
- [x] Test that `set-status`, `derive-dependencies`, and `label-criteria` each leave the key byte-identical

- **Done when**: the key round-trips through every existing frontmatter writer untouched, a malformed `folds-into` is a reported finding, and one naming an absent feature is not (AC19, AC20, AC31, AC32).

## 7. Add `check-unfolded-specs`

- [x] Implement the primitive per the data model, reporting each branch-scoped directory with its `folds-into` and status, plus an `examined` count
- [x] Register it in `runtime/src/schema/primitives.rs`, the MCP server, and `framework/runtime-tools.txt`
- [x] Test: a corpus with no branch-scoped directories reports empty with a non-zero `examined`
- [x] Teach the pipeline view that a declared `folds-into` is outstanding work: the spec is reported as carrying a pending fold, never as `done`, and a target that does not resolve in this tree is called out on the same line
- [x] Block `in-progress → done` in the pre-`done` gate while `folds-into` is present, naming the pending fold — the same category as an unresolved scenario question
- [x] Test: a spec with `folds-into` reports outstanding and fails the gate, and one whose target does not resolve is reported as needing correction

- **Done when**: every surviving branch-scoped directory is reported with its declared fold target, a pending fold holds its spec short of `done` in both the pipeline view and the gate, and an empty result is distinguishable from an unexamined corpus (AC21, AC34, AC35).

## 8. Surface un-folded specs in `/ductus:analyze`

- [x] Add the check to `framework/commands/analyze.md` as a reported finding
- [x] Verify the command still parses under `runtime/src/parser/mod.rs`'s step-numbering assertions

- **Done when**: `/ductus:analyze` reports surviving branch-scoped specs, and the command-parse test passes (AC21).

## 9. Add `rewrite-spec-links`

- [x] Implement the primitive: re-point inbound body links from a retiring or renamed feature directory at the fold target, reporting `rewritten` and `examined`
- [x] Include `folds-into` fields naming that directory — the frontmatter pointer moves with the body links, not after them
- [x] Leave frontmatter alone — `dependencies:` and `references:` regenerate from body links via the pre-commit hook
- [x] Test: sibling `../{feature}/spec.md` links, scenario-targeted links, a `folds-into` naming the moved directory, and a corpus with no inbound pointers

- **Done when**: every inbound pointer to the retiring directory — body links and `folds-into` fields alike — names the fold target, and `dependencies:`/`references:` were left to the generators rather than hand-edited (AC22, AC23, AC33).

## 10. Add `retire-feature`

- [x] Implement the primitive: remove a `BranchScoped` directory, refusing when the fold target does not exist — this is the one place the target's existence is enforced, since nothing before the merge can see it — and refusing a `Sequential` feature outright
- [x] Return `retired: false` as the domain outcome for an already-absent directory
- [x] Test both refusals and the already-gone case

- **Done when**: a branch-scoped directory can be retired only when its fold target exists, and a sequential feature can never be retired by this primitive (AC28).

## 11. Add the fold-routing extension point

- [x] Define the request/response types in `runtime/src/schema/extensions.rs` and the payload builder in `runtime/src/interpreter/payload.rs`
- [x] Keep the vocabulary separate from `routeInboxItem`'s closed five-route set
- [x] Test the payload shape, including a branch-scoped spec that carries its own scenarios

- **Done when**: the extension point returns a body-edit-or-scenario decision per branch-scoped spec, with the target section or scenario slug named.

## 12. Add the `/ductus:fold` command

- [x] Write `framework/commands/fold.md`: enumerate branch-scoped specs, route each at the extension point, confirm with the operator, then per spec — apply the content, rewrite links, reopen a `done` upstream spec with a guarded `set-status`, and retire the directory
- [x] Operate on the session-targeted spec, the one `/ductus:status` surfaced as carrying a pending fold
- [x] Add `fold` to the parser's command list in `runtime/src/parser/mod.rs`
- [x] Regenerate the help and installer surfaces (`scripts/gen-help-tables.sh`, `scripts/gen-claude-commands.sh`, `framework/bootstrap/*`)

- **Done when**: folding a branch-scoped spec into a `done` upstream spec leaves that spec `in-progress` with the content applied, the directory gone, no dangling links, and every installer-parity audit family passing (AC6, AC7, AC18, AC24, AC25, AC29).

## 13. Update the constitution

- [x] Amend §numbering to define both directory forms, naming the branch-scoped one as temporary
- [x] Amend §spec-lifecycle to record that a branch-scoped spec is a staging form discharged by fold-back, reconciling it with the anti-proliferation stance
- [x] Run the audit families that check cross-doc agreement

- **Done when**: both sections describe the shipped behavior, and `scripts/audit/run-all.sh` reports no new findings (AC17).

## 14. Teach `/ductus:specify` branch-scoped creation

- [x] Add the branch-scoped path to `framework/commands/specify.md`: pass `branch-id` and `fold-into` to `create-feature` when the operator asks for a branch-scoped spec
- [x] Prompt for the identifier when branch-scoped creation is requested without one — offering a candidate extracted from the current git branch name when one can be extracted, and prompting with no candidate when it cannot (AC13)
- [x] Echo the **sanitized** identifier `create-feature` returns at the confirmation prompt, before any directory exists — the operator's input is not what they get (AC10)
- [x] Require a fold target on that path: there is no way to create a branch-scoped spec that names none (AC37, AC30)
- [x] Verify the command still parses under `runtime/src/parser/mod.rs`'s step-numbering assertions, and re-bless `runtime/tests/golden/specify-basic.jsonl` if the dispatch sequence changed

- **Done when**: `/ductus:specify` can create a branch-scoped spec end to end — identifier prompted and sanitized-value echoed before creation, fold target required — and sequential creation with no identifier is unchanged (AC10, AC13, AC30, AC37). Added during implementation: the plan's Affected Files names `framework/commands/specify.md` but the original task breakdown covered no command-level work for it, leaving these four criteria without a task.

## 15. Document the pending-fold gate check in `/ductus:implement`

- [x] Add the pending-fold check to the completion gate's documented check order in `framework/commands/implement.md` — third, between the unresolved-scenario-questions check and the `review:` block, which is where `check-review-gate` runs it
- [x] State why it sits there: a declared `folds-into` is outstanding work in the same category as an unresolved scenario question, and a staging spec is retired rather than completed
- [x] Regenerate the installed command copies and re-run the audit

- **Done when**: the gate's documented check order matches the order `check-review-gate` evaluates, so the command source — canonical for command behavior per §drift-prevention — no longer contradicts the runtime (AC35).

## 16. Document the pending fold in `/ductus:status`'s pipeline view

- [x] Record in `framework/commands/status.md` that a spec declaring `folds-into` is rendered as carrying a pending fold rather than as `done`, with the frontmatter status kept beside the qualification
- [x] Record that the Next Action cell is handed to the fold at every status, and that an unresolved scenario question still outranks it — content is settled before it is moved
- [x] Record that a fold target which does not resolve in this tree is called out on the same line, and that this is a report rather than a check: before the merge the target normally lives on the branch this one forked from
- [x] Regenerate the installed command copies and re-run the audit

- **Done when**: the pipeline view's documented rendering matches what `dashboard` emits for a spec carrying `folds-into`, including the Next Action precedence and the unresolvable-target callout (AC34).

## 17. Add `routeFold` to spec 022's extension-point enumeration

- [x] Add `routeFold` to the closed extension-point set in `specs/022-deterministic-runtime/data-model.md` — both the protocol envelope's `extension-point` union and the extension-point section that enumerates the set
- [x] Document its request/response shape as the sibling points are documented, and say what distinguishes it from `routeInboxItem`: that vocabulary answers *where in the corpus does this work belong*, while a fold has already been told where
- [x] Leave 022's status alone — a canonical record synced to shipped behavior is a mechanical edit under §spec-lifecycle

- **Done when**: 022's data model enumerates every extension point the runtime actually exposes, and the cross-spec write is explained in its commit message rather than surfacing at the gate as an unaccounted sibling-spec change (§cross-spec-impact).

## 18. Let the sequential form survive its own counter passing 999

- [x] Widen `parse_sequential` to `{3+ digits}-{any}`, so the name `create-feature`'s `{number:03}` minimum-width pad produces for the 1000th spec is one the predicate accepts
- [x] Keep the name/number mapping injective: reject a run longer than three digits that carries a leading zero, since `{number:03}` never emits one
- [x] Test: `1000-slug` parses as `Sequential { number: 1000 }`; `0500-a` is rejected; `05-a` and `050` stay rejected; a 1000-spec corpus yields `1000-` next and resolves `1000` to it
- [x] Record the widened form in the constitution's §numbering — the pad is a minimum width, not a fixed one
- [x] Remove the item from `specs/inbox.md`

- **Done when**: a spec created past 999 is visible to every corpus reader, `create-feature`'s formatter and `parse_feature_dir`'s membership rule agree on the whole range of `u32`, and the inbox item is discharged rather than left for `/ductus:groom`.

## 19. Contain the `routeFold` payload's fold-target read

- [x] Read the upstream spec through `read_repo_file`'s canonicalize-and-contain check in `build_route_fold_request`, rather than the raw `std::fs::read_to_string` that bypasses it — `folds-into` is hand-authored frontmatter, and the same function already reads the source spec through the guarded helper two lines earlier (BE-INPUT-004)
- [x] Test that a `folds-into` carrying a traversal yields an empty `target-content` rather than a file from outside the repo

- **Done when**: no path built from a spec artifact's declared value reaches the filesystem in the payload builder without the containment check every sibling read already applies, and the escape is covered by a test (BE-INPUT-004).

## 20. Make an interrupted fold safe to re-run

- [x] Give `append-task` a dedup guard keyed on the scenario pointer: when `slug` names a scenario an existing task block already references, return that task's number with `appended: false` rather than adding a second task for the same work — the guard `append-inbox` already has, absent here
- [x] Leave the slug-less case alone: a custom body carries no pointer to key on, so it appends as it does today
- [x] State the resumption contract in `framework/commands/fold.md`: every step is a no-op when its effect is already present, so an interrupted fold is completed by re-running it against the same spec — and say which step provides that for each of the seven writes
- [x] Give the body-edit step its own re-run guard in the same prose: the routing step already holds both documents, so content already present in the target section is recognized and not folded twice
- [x] Test that a second `append-task` naming the same scenario appends nothing

- **Done when**: no step of a fold double-applies when the fold is re-run after an interruption, the two steps that could (`append-task` and the body edit) are guarded, and the resumption contract is written down where the operator reads it (AC29).

## 21. A fold invalidates the upstream spec's recorded review

- [x] Add an `invalidate-review` primitive: null the spec's `review.last-run` and `reviewed-against`, zero the counts, clear `blocking`, and preserve operator-authored `waivers` — returning `invalidated: false` when there was no current review, so a re-run converges
- [x] Wire it at the six Rust sites plus `framework/runtime-tools.txt`, per the registration gotcha in `AGENTS.md`
- [x] Call it from `framework/commands/fold.md` against the **upstream** spec, on every route — not only the `done` reopen, since a fold into an already-`in-progress` spec leaves the same stale review behind
- [x] Say why in the command prose: the staleness check reads durable contracts, and a body-edit fold writes only `spec.md`, which it deliberately excludes — so without this the upstream spec returns to `done` on a review that never saw the code the fold brought with it
- [x] Test the invalidation, the preserved waivers, and the already-invalid re-run

- **Done when**: an upstream spec that has received a fold cannot reach `done` until `/ductus:review` has run against it again, on either route (AC24).

## 22. Preserve line endings through a file rewrite

- [x] Implement the behavior described in `scenarios/rewrites-preserve-line-endings.md`
- [x] Extract the line-ending-preserving rewrite into one shared helper, so the three primitives that need it stop each carrying their own detection
- [x] Route `rewrite-spec-links` through it — it is the writer that currently converts a CRLF file to LF whenever it re-points a single link
- [x] Test a CRLF fixture end to end: one link changes, every other byte survives, and the endings are unchanged

- **Done when**: no primitive that rewrites an existing text file changes its line endings as a side effect, and the detection lives in one place rather than three.

## 23. Establish the fold target before the corpus-wide rewrite

- [x] Implement the behavior described in `scenarios/fold-target-checked-before-the-rewrite.md`
- [x] Check the fold target's existence in `framework/commands/fold.md` before step 11's `rewrite-spec-links`, on the same terms `retire-feature` enforces — a directory holding a `spec.md`, not merely a directory
- [x] Leave `retire-feature`'s own check in place: it guards the one irreversible step and the primitive is callable on its own, so a caller's promise is not a substitute
- [x] Correct step 12's prose, which currently names its refusal as the answer to an unresolved `folds-into` — by then the question is already settled
- [x] Renumber and re-verify the command's step assertions, and re-bless any golden whose dispatch sequence changed

- **Done when**: a fold whose target does not resolve refuses with the corpus untouched, and no inbound link is ever re-pointed at a spec whose existence has not been established (AC29).

## 24. Take the numbering grammar to the shell surfaces

- [x] Implement the behavior described in `scenarios/the-numbering-grammar-reaches-every-surface.md`
- [x] Widen the staged-spec filter in **both** hook copies — `.githooks/pre-commit` and the shipped `framework/bootstrap/hooks/ductus-pre-commit` — to accept three-or-more digits while still rejecting padding past three, keeping the leading-segment wildcard audit Family 22 depends on
- [x] Have `scripts/lint-frontmatter.sh` and the two audit families (`sibling-coupling.sh`, `introducing-drift.sh`) obtain the corpus from the runtime rather than globbing `[0-9][0-9][0-9]-*`, following the entry-point-in-shell / logic-in-a-primitive shape §runtime-boundary asks for
- [x] Add a check that the hook pattern and `parse_feature_dir` agree, so the two cannot drift apart again silently
- [x] Verify a `1000-` spec is staged by the hook, linted, and examined by both families — and that `0500-a` is skipped by all of them, matching the runtime's rejection

- **Done when**: a spec directory the runtime recognizes is recognized by every shell surface of the project, both hook copies included, and no surface carries its own copy of the digit rule that could drift from `parse_feature_dir` unnoticed (AC15).

## 25. Cut the release that carries this work to adopters

- [x] Bump all three artifacts to `0.36.0` in one commit — the repo-root `version` file, `runtime/Cargo.toml`, and a matching `runtime/CHANGELOG.md` section — since `/audit` Family 20 fails when they disagree and `/ductus` reads `version` to decide which runtime to acquire
- [x] Minor rather than patch: four new primitives, a new command, and new `create-feature` arguments are added surface, not a bug fix
- [x] Write the CHANGELOG section as a record of what changed for an adopter — the second directory form, `/{project}:fold` and the four primitives behind it, the `folds-into` key, and the two defects fixed on the way (line endings across every text writer, and the numbering grammar reaching the shell surfaces)
- [x] Run `cargo build --release --offline` once so the lockfile absorbs the version change, then confirm `--locked` succeeds again — per the `AGENTS.md` gotcha, `--locked` refuses the crate's own version bump
- [x] Run `scripts/audit/run-all.sh` locally before proposing the tag: it is a hard release gate in the tag pipeline, and a finding there aborts the publish, the asset upload and the SBOM
- [x] Stop at the commit. The push and the `ductus-v0.36.0` tag are the operator's — the crates.io publish can be yanked but never unpublished

- **Done when**: the three version artifacts agree at `0.36.0` with a CHANGELOG section describing this feature, the audit passes locally, and the tree is one reviewed commit away from a release the operator can push — with the push and tag deliberately left to them.
