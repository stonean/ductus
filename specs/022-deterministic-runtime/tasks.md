# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 65. Implement scenario: [mcp-arg-unknown-field-strictness](scenarios/mcp-arg-unknown-field-strictness.md)

- [x] Implement the behavior described in `scenarios/mcp-arg-unknown-field-strictness.md`

- **Done when**: an unknown field in an MCP tool call is rejected with a naming error via a derived per-primitive field allowlist; the exec path's superset-context binding is unaffected; a test covers a misspelled kebab arg on both surfaces; `cargo test` green.

## 66. Implement scenario: [fetch-archive-dns-rebinding](scenarios/fetch-archive-dns-rebinding.md)

- [x] Implement the behavior described in `scenarios/fetch-archive-dns-rebinding.md`

- **Done when**: `fetch-archive` connects only to the address `validate_fetch_url` screened (pinned `SocketAddr` or a connect-time re-check against the internal-address predicate), so a host that rebinds between validation and connect cannot reach an internal address; a connect-time internal address is a naming error, not a silent connect; a test covers the rebind case; `cargo test` green.

## 67. Implement scenario: [append-inbox-comment-aware-write](scenarios/append-inbox-comment-aware-write.md)

- [x] Implement the behavior described in `scenarios/append-inbox-comment-aware-write.md`

- **Done when**: `append-inbox`'s write side is comment/fence-aware like its read side: a bullet appended to an `inbox.md` that ends inside an unclosed HTML comment (or code fence) lands in a position `count_inbox_bullets` counts, never inside the comment; well-formed inboxes append unchanged; a test covers the unclosed-comment case; `cargo test` green.

## 68. Implement scenario: [write-review-known-field-quoting](scenarios/write-review-known-field-quoting.md)

- [x] Implement the behavior described in `scenarios/write-review-known-field-quoting.md`

- **Done when**: `write-review` renders known waiver fields through the same `yaml_string` quoting as the extra fields, so a bare-numeric/bool/null-like known-field value is quoted and round-trips through `RawWaiver`; timestamp-shaped values (`waived-at`) produce no golden-fixture churn (verified before landing); a test covers a bool-like `reason`; `cargo test` green.

## 69. Implement scenario: [skipscanner-inline-code-exemption](scenarios/skipscanner-inline-code-exemption.md)

- [x] Implement the behavior described in `scenarios/skipscanner-inline-code-exemption.md`

- **Done when**: `SkipScanner.skip` treats an HTML-comment or code-fence delimiter inside a backtick inline-code span as inert (a line mentioning a backticked comment-open delimiter opens no skip region), so `read-tasks` / `mark-task` / `dashboard` / the section walkers see structure after such a line; a genuine comment or fence in ordinary text is still skipped; a test covers a task/spec line with a backticked comment-open delimiter that must not hide a following heading; `cargo test` green.

## 70. Implement scenario: [derive-boundary-uncommitted-spec-dir](scenarios/derive-boundary-uncommitted-spec-dir.md)

- [x] Implement the behavior described in `scenarios/derive-boundary-uncommitted-spec-dir.md`

- **Done when**: `/ductus:plan`'s validation gate refuses to advance `clarified → planned` while no commit touches `specs/{feature}`, reporting a domain outcome naming the fix rather than an operational error; `derive-boundary` on an uncommitted spec dir returns an empty boundary plus guidance instead of `no commits found that touch specs/{feature}`, and enforcement stays fail-closed on that empty result; a seeded `write-boundary` still admits the walk via the seeded ∪ derived union; the markdown-only prose for both commands carries the same guidance; tests cover the uncommitted-dir gate refusal and the empty-boundary outcome; `cargo test` green.

## 71. Implement scenario: [unchecked-done-when-clause-tally](scenarios/unchecked-done-when-clause-tally.md)

- [x] Implement the behavior described in `scenarios/unchecked-done-when-clause-tally.md`

- **Done when**: `mark-task` ticks an unchecked checkbox-form `Done when` clause once every real subtask of that task is complete, leaving the block visually coherent without adding the clause to the subtask index space (a two-subtask task still reports total 2 and `--subtask-index 2` stays out of range); `/ductus:implement`'s per-task report distinguishes "all subtasks checked" from "task block fully checked" so an unticked clause is surfaced rather than rounded up; the checked, canonical-bold, and bulletless forms are unaffected; completing an already-complete task produces no diff; tests cover the unchecked-clause tick, the index-contract invariance, and idempotence; `cargo test` green.

## 72. Implement scenario: [block-element-scanner](scenarios/block-element-scanner.md)

- [x] Implement the behavior described in `scenarios/block-element-scanner.md`

- **Done when**: a `pub(crate)` block splitter yields each table row, list item, and paragraph with its starting line number; every line passes through `SkipScanner` first so fenced code and HTML comments never reach a block, and `SkipScanner` itself is unmodified (the `tasks.md` parsers that share it are provably unaffected); blockquote lines are dropped by the splitter; `inline_code_spans` is `pub(crate)` with no behavior change; tests cover each block kind, all four exempt contexts, an unterminated fence running to EOF, and a bullet opening with no preceding blank line; `cargo test` green.

## 73. Implement scenario: [check-artifacts-skipped-targets](scenarios/check-artifacts-skipped-targets.md)

- [x] Implement the behavior described in `scenarios/check-artifacts-skipped-targets.md`

- **Done when**: `CheckArtifactsResult` carries `skipped` as a list of `{family, reason, path}` over the closed reason set `target-missing` / `target-unparseable` / `no-readable-state`; `clean` still means `findings.is_empty()`; the five pre-existing families return an empty `skipped` and no existing test expectation changes; a target skipped by two families yields one record per family while the same target skipped twice by one family yields one; tests cover the `clean: true` with non-empty `skipped` state as legal; `cargo test` green.

## 74. Implement scenario: [link-adjacent-drift-family](scenarios/link-adjacent-drift-family.md)

- [x] Implement the behavior described in `scenarios/link-adjacent-drift-family.md`

- **Done when**: `check-artifacts` gains an advisory `link-adjacent-drift` family scanning `spec.md`, `plan.md`, `tasks.md`, and `scenarios/*.md`; a sibling link is resolved lexically against the containing file's directory with no `canonicalize` call, URL-scheme and bare-fragment targets rejected; the six closed tells match only outside inline code spans; evaluation is per link and emits one finding per (block, link) pair naming the citing file and line, the target, the tells that fired in list order, and the contradicting state; a scenario target is evaluated on open-question count and existence only, with implementation-state tells producing nothing; unreadable targets land in `skipped` rather than `findings`; a consistent feature directory produces zero findings; repeat runs are byte-identical; `cargo test` green.

## 75. Implement scenario: [criterion-path-existence-family](scenarios/criterion-path-existence-family.md)

- [x] Implement the behavior described in `scenarios/criterion-path-existence-family.md`

- **Done when**: `check-artifacts` gains an advisory `criterion-path-existence` family scanning `## Acceptance Criteria` on `done` specs only, reading through `read-spec`'s parsed criteria; candidate paths are taken from inline code spans only under the documented grammar, resolved repo-root-relative and satisfied by a file or a directory with a trailing slash stripped; body prose naming a deleted path produces no finding; the grammar's rejections (a slash-command reference, a `path:line` citation, a glob, a URL, a flag) are each covered by a test; a fixture reproducing 026's AC5 after `531e3ea` yields a finding for each of the two dead paths and none at a non-`done` status; `cargo test` green.

## 76. Implement scenario: [mark-task-untick-symmetry](scenarios/mark-task-untick-symmetry.md)

- [x] Implement the behavior described in `scenarios/mark-task-untick-symmetry.md`

- **Done when**: `mark-task` unticks a ticked checkbox-form `Done when` clause in the same atomic write whenever the flip leaves any real subtask of that task unchecked, mirroring the tick direction it already implements; the clause stays outside the subtask index space (a two-subtask task still reports total 2 and `--subtask-index 2` stays out of range); the canonical bold and bulletless forms are unaffected; an already-coherent block still produces no write; tests cover the untick, the round trip (tick → untick → tick), a task with no real subtasks, and index-contract invariance; `cargo test` green.

## 77. Implement scenario: [review-scope-parse-fidelity](scenarios/review-scope-parse-fidelity.md)

- [x] Implement the behavior described in `scenarios/review-scope-parse-fidelity.md`

- **Done when**: `parse_affected_files` resets its header state at a non-table line so a section holding several tables emits no header row as a path, and a first cell carrying a qualifier yields its backticked span rather than the whole cell; `compute-review-scope`'s captured-issues intersects the diff's added lines with the bullets the shared comment-aware grammar finds in the post-image inbox, so comment lines are never counted; verified against the real 017 input (plan-affected 43 entries, zero malformed, captured-issues 1 real bullet, down from 8 bogus `File` entries and ~30 comment lines); tests cover the multi-table section, the qualified cell, and the comment-block case; `cargo test` green.

## 78. Implement scenario: [numbered-heading-grammar-single-source](scenarios/numbered-heading-grammar-single-source.md)

- [x] Implement the behavior described in `scenarios/numbered-heading-grammar-single-source.md`

- **Done when**: `primitives/mod.rs` owns `split_numbered_heading` (borrowed `Option<(&str, &str)>`) and `heading_is_numeric` (defined as `.is_some()` on it) as the single `pub(crate)` home for the `N.` task-heading grammar; the three private copies in `read_tasks.rs`, `prune_tasks.rs`, and `mod.rs` are gone; `prune_tasks`'s task branch parses once rather than testing a predicate then re-parsing; the moved unit tests cover the union of what the three copies each asserted (`12. Title`, `3 quick wins`, `Not numbered`, bare `12`, `12.`) plus a predicate/splitter agreement check; `cargo test` green

## 79. Implement scenario: [config-resolution-single-probe](scenarios/config-resolution-single-probe.md)

- [x] Implement the behavior described in `scenarios/config-resolution-single-probe.md`

- **Done when**: `schema/paths.rs` exposes `resolve_config(repo) -> (PathBuf, &'static str)` performing one existence probe for both the read path and the provenance name; `discover_rule_files::load_ductus_toml` and `dashboard::load_config` each return the resolved name alongside the parsed content and their render paths consume it instead of re-probing via `config_display_name`; `DashboardConfig`'s serialized shape is unchanged so no MCP golden re-blesses; `config_path` / `config_display_name` survive for single-half callers with doc comments pointing both-halves callers at `resolve_config`; `cargo test` green

## 80. Implement scenario: [sibling-symlink-trust-boundary](scenarios/sibling-symlink-trust-boundary.md)

- [x] Implement the behavior described in `scenarios/sibling-symlink-trust-boundary.md`

- **Done when**: `check_artifacts.rs` gains `traverses_symlink(target, base)` using `std::fs::symlink_metadata` over every component at or below the feature directory, so the answer never depends on a link's destination and repeat runs stay byte-identical; the up-front scenario-readability pass short-circuits the read for a linked entry so its destination is never opened; `read_target_state` tests before `is_file()` and returns the existing `target-unparseable` reason; lexical resolution and the `starts_with` containment test are unchanged; a `#[cfg(unix)]` regression test asserts a symlinked sibling lands in `skipped` with `target-unparseable` and that `../../../etc/passwd` still resolves to `None`; `cargo test` green

## 81. Implement scenario: [criterion-non-assertion-phrasings](scenarios/criterion-non-assertion-phrasings.md)

- [x] Implement the behavior described in `scenarios/criterion-non-assertion-phrasings.md`

- **Done when**: `NON_ASSERTION_MARKERS` carries `deleted` (replacing the narrower `is deleted` / `are deleted` pair), `(was` + space, and `target paths`, documented as five groups rather than four with the migration-subject group named; the exemption test covers all three new phrasings alongside the six existing ones; a repo-wide sweep drops `criterion-path-existence` from 25 findings to 21, suppressing exactly the four residual false positives (003's parenthetical rename, 043's migration targets, 045's two past-tense-agent paths) and nothing else, verified by diffing findings on `(spec, path)` keys; the 19 adopter-scope findings and the 2 true positives (005, 025) are untouched; `cargo test` green

## 82. Implement scenario: [criterion-adopter-scope-destinations](scenarios/criterion-adopter-scope-destinations.md)

- [x] Implement the behavior described in `scenarios/criterion-adopter-scope-destinations.md`

- **Done when**: `adopter_destinations` derives the Shared Files manifest destination set from `framework/bootstrap/ductus.md` once per feature, taking only cells that are exactly one backticked span; `ships_to_adopter` matches a candidate that equals a destination or is a directory containing one; the check runs after the resolve and `root-absent` arms and records `skipped { reason: "ships-to-adopter" }` rather than dropping the candidate; an absent or unparseable manifest yields an empty set so nothing is suppressed and findings are still emitted; the repo-wide sweep drops `criterion-path-existence` from 21 findings to 2 with 9 `ships-to-adopter` skips, leaving exactly the two true positives (005, 025); the new reason is documented in 022's data-model, the `check-artifacts-skipped-targets` scenario, 045's data-model, and `SkippedTarget`'s doc comment; three regression tests cover the destination match, the directory match, the no-manifest case, and a non-shipped stale path; `cargo test` green

## 83. Implement scenario: [review-staleness-gate](scenarios/review-staleness-gate.md)

- [x] Implement the behavior described in `scenarios/review-staleness-gate.md`

- **Done when**: `ReviewGateBlock` gains a `ReviewStale` variant and `check-review-gate` evaluates it as gate check 5, after the `blocking` check; staleness is scoped to the plan's **Affected Files** via `compute_review_scope::read_plan_affected` (promoted to `pub(crate)` rather than reimplemented), with directory entries matching everything beneath them and the spec's own `review.md` / `spec.md` excluded as bookkeeping; the check fails open on a missing git repo, an unresolvable `reviewed-against`, or a plan with no Affected Files table; four tests cover the stale case (reproducing the `gvrn-v0.26.2` shape), an out-of-scope change staying current, review bookkeeping not counting, and the fail-open path; `cargo test` green

## 84. Implement scenario: [scenario-open-question-signal](scenarios/scenario-open-question-signal.md) — clarify reports scenario open questions

- [x] `framework/commands/clarify.md`: report outstanding scenario open questions on a feature-targeted run, from the `scenario-open-questions` field `read-spec` already returns at step 2 — naming every carrying scenario and the scenario-targeted command, in every gate branch where the field is non-empty (the `already {status}` stop, the `done` stop, and the `draft` rows before advancing to `clarified`)
- [x] `framework/commands/clarify.md:43`: rewrite the Scope Boundaries line — scenario questions are surfaced but not resolved; spec-level and scenario-level questions stay independent for *resolution* only
- [x] `framework/commands/clarify.md:42`: add the narrow markdown-only carve-out permitting reads of `scenarios/*.md` `## Open Questions` sections — those sections only, never scenario bodies
- [x] Re-render the installed commands with `scripts/gen-claude-commands.sh` (the framework source is canonical; `.claude/commands/ductus/` is a build artifact that does not update on its own)
- [x] Cross-spec (§cross-spec-impact): correct `046/spec.md:48-53` (the independent-resolution decision's second half) and its acceptance criterion `:117`, add a criterion for the reporting surface, and update `046/plan.md:27,29` and `046/tasks.md:92`. This is a meaningful body edit on a `done` spec — take 046's back-edge to `in-progress` via `/ductus:amend` before editing
- [x] Confirm no gate moved: `draft → clarified` still advances with scenario questions outstanding, and the `already {status}` and `done` branches still modify no file

- **Done when**: feature-targeted `/ductus:clarify` names every scenario carrying open questions and the scenario-targeted command to resolve them, in all three affected gate branches; a feature with an empty `scenario-open-questions` list sees no change to any branch (the report is suppressed, not rendered as "0 outstanding"); no branch that reported gains a write; `draft → clarified` still advances carrying scenario questions, with the pre-`done` gate still the only mechanized block; 046's design decision and acceptance criteria no longer assert that feature-targeted clarify is unchanged, and 046 has taken its back-edge to `in-progress`; `scripts/gen-claude-commands.sh` re-run so the installed copy matches; `npx markdownlint-cli2` clean on every modified file

## 85. Implement scenario: [append-primitive-marker-normalization](scenarios/append-primitive-marker-normalization.md)

- [x] Implement the behavior described in `scenarios/append-primitive-marker-normalization.md`

- **Done when**: a shared `strip_bullet_marker` helper in `primitives/mod.rs` delegates to the existing `bullet_text` grammar rather than hand-rolling a second matcher; `append-task` applies it to each `body` item, `append-question` to `question` before both the dedup comparison and the insert, and `append-inbox` to both `text` and `dedup-prefix`; each affected argument's schema description records that the primitive renders the marker and strips a caller-supplied one; regression tests cover a marker-prefixed body item rendering exactly one checkbox, a marker-bearing `dedup-prefix` matching an existing bullet, a marker-prefixed question deduping against its unmarked twin, and `--fix`-style leading-dash content surviving untouched; `cargo test` green

## 86. Implement scenario: criterion-label-assignment

Runtime half of [013's `criterion-identifiers`](../013-text-first-artifacts/scenarios/criterion-identifiers.md); 013's task 24 carries the artifact half and depends on this one shipping first. Sub-items are ordered; the unchecked ones are what remains.

- [x] `label-criteria` primitive — idempotent labelling pass, `max(body, next-criterion)` assignment, atomic write, 10 unit tests. Registered in `primitives/mod.rs`, `schema/primitives.rs`, `schema/registry.rs`, `main.rs`, `mcp/server.rs`, `interpreter/mod.rs`, and `framework/runtime-tools.txt`.
- [x] `mark-criterion` accepts `label` alongside `criterion-index` (exactly one; both or neither refused; unknown label errors rather than no-ops), and `read-spec` reports each criterion's `label`. Both resolve through `label_criteria::parse_label` — one implementation, deliberately, since a second matching rule is the drift the reconcile-pass review already caught. `plan-basic.jsonl` re-blessed for the added `"label":null` field.
- [x] `check-artifacts` gains a `criterion-labels` family (advisory): duplicate `AC{n}` within one spec; `next-criterion` at or below the highest label in the body; an unlabelled criterion once the backfill has landed. Each checkable from the artifact alone — no git history read. Follow the seven existing families' conventions, including whether the family contributes to `skipped` (it does not: its subject is always fully examinable). The "unlabelled" gate is the counter's *presence*, not a date: 013 defines an absent `next-criterion` as "no labels assigned yet", so the backfill is what makes the check universal and no per-spec grandfather state is needed. 10 unit tests.
- [x] `framework/commands/analyze.md` gains the matching **Acceptance-criterion labels** reference section, and step 8 names eight families. Not scope creep: each family "MIRRORS the markdown-only reference — the primitive introduces no policy of its own", so a family with no reference section is both undocumented policy and a hole in the two-paths guarantee (§runtime-host-integration). Re-run `scripts/gen-claude-commands.sh`.
- [x] **Walker fix, found by review of this change:** the exec walker now pins `feature`/`path` after a retargeting primitive. Adding `label-criteria` between `create-feature` and `write-session` in `/ductus:specify` exposed it — results merge at the top level by bare key, `create-feature`'s `path` is the spec *directory* and `label-criteria`'s is the spec *file*, and on a repo with **no** session file neither key is seeded, so the file path won and `/ductus:specify` wrote a session target pointing at `spec.md`. The seeded-key guard hid it wherever a session file already existed, and the parity golden hid it because its fixture seeds `path`. The same gap is pre-existing on `/ductus:target`, where `read-spec` sits between `resolve-feature` and `write-session`; the pin closes both. Two regression tests drive an **unseeded** walker (the case no existing test covered). **The release entry must cover the `/ductus:target` half — it is a user-visible fix independent of the labelling work.**
- [x] `data-model.md` records `label-criteria`'s result shape, `mark-criterion`'s widened addressing, `read-spec`'s `label` field, the new family, and the session-target pin — it is the canonical registry of primitive result shapes and check families.
- [x] Version bump + `runtime/CHANGELOG.md` + `ductus-v*` tag. **Shipped as `ductus-v0.28.0` on 2026-08-16** — five target assets with sidecars, an SBOM, `ductus 0.28.0` on crates.io, and acquisition verified on all five platforms. The operator bar below was met at the tag: the self-audit exits 0, every affected spec's `check-artifacts` is clean, and both reviewed specs record their findings as fixed rather than deferred. Original note follows. **Deferred by operator decision (2026-08-14): no tag until [048 — Ductus-Acquired Runtime](../048-govern-acquired-runtime/spec.md) is done**, so this work rides that release rather than cutting its own. Run `scripts/audit/run-all.sh` locally before tagging — the tag pipeline treats it as a hard release gate. **The operator's completion bar for this tag (2026-08-15) is wider than the audit:** every identified piece of work ships first, including all MUST and SHOULD findings from `/ductus:review` and every issue `/ductus:analyze` reports across the affected specs. A finding deferred past the tag is not deferred, it is shipped. Until the tag ships, the primitive exists only in `main` and in local builds; no adopter has it.

- **Done when**: the scenario's described behavior is correctly implemented and tested; the labelling pass assigns `AC{n}:` labels idempotently and maintains `next-criterion`, `mark-criterion` resolves a label as well as an index, `read-spec` reports labels, the `check-artifacts` family reports duplicate labels and a stale counter, `specs/022-deterministic-runtime/data-model.md` records the primitive's result shape and the new family, and the release is tagged.

## 87. Implement scenario: [project-directory-resolution-chain](scenarios/project-directory-resolution-chain.md)

Runtime half of [049's rename](../049-rename-govern-to-ductus/spec.md): the per-project directory moves, so config and session resolution grows a third tier rather than swapping one out. 049 keeps the sweep, the migration, and its own acceptance criteria.

- [x] Replace the `(new, legacy)` pair in `runtime/src/schema/paths.rs` with one ordered chain per file (`CONFIG_CHAIN`, `SESSION_CHAIN`), walked by every read and write resolver so precedence is stated once
- [x] Reads return the newest existing tier and fall back to the oldest; writes return the newest existing tier and fall back to the newest — the only case in which the two differ
- [x] Tests enumerate every subset of both chains, plus the 049 guarantee that a project on the middle tier alone never resolves to an un-migrated `.ductus/`
- [x] `data-model.md` records the chain as the canonical per-project file resolution

- **Done when**: `cargo test` is green, an adopter on any of the three layouts resolves to their own newest tier, a fresh project writes to `.ductus/`, and no primitive moves a file between tiers — the bootstrap migration stays the sole cutover.

## 88. Implement scenario: [review-staleness-on-done-specs](scenarios/review-staleness-on-done-specs.md) — check-artifacts reports a stale review on a done spec

- [x] **Measure first.** Corpus-wide flag count established 2026-08-16 *before* the design was finalised, as the scenario requires: 19 of 46 `done` specs, all 19 false positives from 049's rename sweep. See the scenario's Resolved Questions.
- [x] **Fix the shared path.** `primitives::mechanical_sweep` ports Family 19's mechanical-sweep exemption to Rust and `stale_review_block` applies it, so the transition gate stops firing on rename sweeps; `revparse_single` replaces `Oid::from_str` so an abbreviated `reviewed-against` no longer fails the check open; `tests/mechanical_sweep_parity.rs` pins the Rust and Python implementations to agree over the real corpus.
- [x] **Re-argue the case.** Done 2026-08-16, and the answer is **do not build the arm**. The scenario's premise — that nothing re-evaluates a `done` spec's verdict — is false: `framework-checks.yml` runs the full self-audit, Family 19 included, on every push to `main` (branch filter, no `paths:` filter), and a durable contract is by definition a file the same push touches. The arm would be a third implementation of a rule that already has one too many. See the scenario's Resolved Questions.
- [x] **Moot, by the decision above.** The remaining bullets as authored — the `skipped` fail-open reasons, the grandfather rule, the data-model family-registry entry, `analyze.md`'s mirrored policy, the blast-radius run, and the arm's unit tests — all describe the `check-artifacts` arm that is not being built. They are recorded here rather than deleted so the shape of what was considered stays legible. The blast-radius run *was* performed (bullet 1) and is what produced the decision.

- **Done when**: ~~the arm ships~~ — superseded. This task is complete on its **investigation and its fix**: the corpus-wide flag count was measured before any design choice was finalised (19 of 46, all false positives); the divergence that number exposed was fixed at the shared path (`primitives::mechanical_sweep`, plus the abbreviated-sha fail-open) and shipped in `ductus-v0.29.1`; the two implementations that cannot be merged are pinned to agree by `tests/mechanical_sweep_parity.rs`; and the arm's own case was re-argued against evidence and declined, with the reasoning recorded in the scenario's Resolved Questions rather than left as an unexplained gap. `cargo test`, `cargo clippy -- -D warnings`, `scripts/audit/run-all.sh` and `npx markdownlint-cli2` all clean. The scenario's Behavior section is superseded by those Resolved Questions; a reader should start there.

## 89. Implement scenario: [unreadable-scenario-is-reported](scenarios/unreadable-scenario-is-reported.md) — an unread scenario is distinguishable from a clean one

- [x] `collect_scenario_open_questions` returns a `ScenarioQuestionScan` carrying `questions` **and** `unreadable` rather than a bare `Vec`, staying the single shared reader all four surfaces consume
- [x] `read-spec` surfaces `scenario-files-unreadable`, `skip_serializing_if = "Vec::is_empty"` so the ordinary payload is byte-unchanged and no parity golden re-blesses
- [x] `check-artifacts` records each unread scenario as a `skipped` target with family `scenario-open-questions` and the existing `artifact-unreadable` reason — not a finding, and not a new reason
- [x] `check-review-gate` still returns no block when the question list is empty, whatever the unread set holds — fail-open, matching the staleness check's posture
- [x] `dashboard` discards it explicitly, with the reason named in code so the discard reads as a decision
- [x] 022's data-model records the new field, the skipped record, and the corrected never-merged rationale; `framework/commands/clarify.md`'s report reference names the unread scenarios on both paths
- [x] Tests: an unreadable scenario reported and not counted as clean; a fully-examined feature reporting nothing unread (so empty *means* examined); the existing link-adjacent-drift skip assertion scoped by family and extended to cover the second record

- **Done when**: `read-spec` returns `scenario-files-unreadable` alongside `scenario-open-questions`, absent when empty; `check-artifacts` records each unread scenario as an `artifact-unreadable` skipped target under the `scenario-open-questions` family; no gate gains a block and `check-review-gate` still fails open; 022's data-model and `clarify.md`'s report reference both state the new contract so the markdown-only path matches; the parity goldens are unchanged (proving the empty case is byte-identical); `cargo test`, `cargo clippy -- -D warnings`, `scripts/audit/run-all.sh` and `npx markdownlint-cli2` all clean.

## 90. Implement scenario: [review-observations-write-through](scenarios/review-observations-write-through.md) — recording an observation is capturing it

- [x] Implement the behavior described in `scenarios/review-observations-write-through.md`

- **Done when**: `write-review` accepts an `observations` array whose entries are excluded from the MUST / SHOULD / low-confidence counts and from `blocking`; each entry is appended to `{specs-root}/inbox.md` in the same call, dedup-guarded on a stable prefix so a re-run over an unchanged repo appends nothing; the report gains an `## Observations` section rendering `*None.*` when empty; an unwritable inbox fails the call rather than rendering a report that claims capture that did not happen; embedded newlines are rejected as `append-inbox` already rejects them; `framework/commands/review.md` documents the array and the section so the markdown-only path writes both in the same order; 022's data-model records the widened argument and the new section; `cargo test` green and `npx markdownlint-cli2` clean.

## 91. Implement scenario: [specify-routes-before-scaffolding](scenarios/specify-routes-before-scaffolding.md) — routing binds wherever work enters

- [x] Implement the behavior described in `scenarios/specify-routes-before-scaffolding.md`

- **Done when**: `/{project}:specify` presents derived routing candidates and confirms the choice before `create-feature` writes anything; candidates come from the rule-file directory, the spec corpus, and the runtime-work signal (a named primitive, `check-artifacts` family, result field, or `runtime/src/` path routing to 022 as a scenario); the tree is groom's, reused rather than duplicated; "could not derive candidates" is reported distinctly from "no candidates found"; naming a `done` candidate also names the back-edge it implies; a groom-initiated specify skips the gate rather than asking twice; a corpus with no candidates proceeds unchanged so a fresh adopter sees no new friction; `framework/commands/specify.md` documents the gate for the markdown-only path and the generated copies are re-rendered; `cargo test`, `scripts/lint-procedure-parseability.sh` and `npx markdownlint-cli2` clean.

## 92. Implement scenario: [merge-managed-block-renamed-subsection](scenarios/merge-managed-block-renamed-subsection.md) — a renamed subsection must not strand the old block's tail

- [x] `merge_managed_block.rs`: in `walk_body_extent`, an on-disk group matching no canonical group no longer ends the block when a later on-disk group still aligns with a remaining canonical one — the retired-subsection case, distinguished from adopter content by its continuation
- [x] Bound the lookahead (`MAX_SKIPPED_RETIRED_GROUPS`) so an adopter's pasted duplicate far below cannot extend the block and swallow everything above it
- [x] Preserve the trailing-append invariant unchanged: adopter content after the block is still never consumed by group alignment
- [x] Tests: a mid-block renamed subsection replaced rather than stranded, with no orphaned comment header and the adopter tail intact; a long unmatched run past the bound preserved as adopter territory; all existing line-prefix tests green
- [x] Update the module doc, which currently records the stranding as a deliberate trade-off

- **Done when**: `merge-managed-block` replaces a `line-prefix` subsection whose patterns were all renamed instead of stopping the walk at it, so no tail of the old block survives below the merged one and the dedup pass leaves no orphaned comment headers; the lookahead is bounded and the bound's rationale is stated in code; `scenarios/merge-managed-block-trailing-append.md`'s invariant still holds and its tests are unchanged; the real case that surfaced it — spec 048's adopter bootstrap, whose `.gitignore` kept a dead `.govern.session.toml` plus headerless `# IDE` / `# OS` — merges clean; `cargo test`, `cargo clippy -- -D warnings` and `npx markdownlint-cli2` all clean.

## 93. Implement scenario: [orphaned-reference-check](scenarios/orphaned-reference-check.md) — one primitive for two call sites, with attribution that degrades out loud

- [x] Implement the behavior described in `scenarios/orphaned-reference-check.md`

- **Done when**: `check-orphaned-references` ships: reports adopter-owned files whose framework-owned path references do not resolve (referrer, missing path, line), read-only and never repairing; attribution is `registry` when `framework/migrations.toml` is readable and `watermark` otherwise, echoing `[migrations].last_applied`, with the two rendered distinguishably; unreadable referrers land in `skipped` with a reason so an empty `findings` means examined-and-clean only when `skipped` is empty; wired at all seven Rust sites plus `framework/runtime-tools.txt`; `framework/commands/analyze.md` surfaces it under §Project-level consistency and `framework/bootstrap/ductus.md` calls it at migration-batch end; `specs/022-deterministic-runtime/data-model.md` records the result shape; generated command copies re-rendered; `cargo test`, `cargo clippy`, `npx markdownlint-cli2`, `scripts/lint-procedure-parseability.sh` and `scripts/audit/run-all.sh` all clean; released under a `ductus-v*` tag.

## 94. check-orphaned-references matches historical roots and declares its scope

- [x] `managed_roots` gains `.govern/`, `scripts/gen-`, `scripts/lib/`, with the historical-root reasoning in the doc comment
- [x] `CheckOrphanedReferencesResult` gains `matched-prefixes`, populated from the roots the run used
- [x] the pre-existing test that worked around the blind spot is corrected to assert the retired-root reference is now reported
- [x] tests: the pre-042 generator orphan reports; an adopter-owned script does not; a clean result declares its prefixes
- [x] 022 `data-model.md`: result JSON, the managed-roots bullet, and the scope-honesty bullet
- [x] version bump, CHANGELOG entry, and a `ductus-v*` tag — the change reaches no adopter without it

- **Done when**: `managed_roots` returns the historical roots `.govern/`, `scripts/gen-` and `scripts/lib/` alongside the current ones; the result carries `matched-prefixes`; a bare pre-042 generator reference is reported, an adopter-owned `scripts/build.sh` is not, and a clean result names the prefixes it matched — each covered by a test. 022's `data-model.md` records the field and the historical-root reasoning. Released, since the change is under `runtime/src/`.

## 95. Implement scenario: [review-scope-union](scenarios/review-scope-union.md) — a gate must look at what changed

- [x] `compute-review-scope` returns `scope` as the deduplicated, sorted union of `plan-affected` and `modified-since` instead of whichever set is larger
- [x] The regression test asserts the file the work touched is in scope — the exact file the larger-of rule dropped — rather than asserting set equality with one input
- [x] Module doc and `ComputeReviewScopeResult`'s field doc state the union
- [x] `framework/commands/review.md` §Inputs and Instructions step 1 drop the "not a union" wording; generated command copies re-rendered
- [x] `specs/020-code-review/spec.md`'s scope definition updated via the back-edge — it states the rule in prose and a behavior change makes it a stale claim
- [x] `review-runtime-acceleration.md`'s two statements of the old rule corrected, and 022's §spec note that cited it as the reason a re-review is large
- [x] Version bump, CHANGELOG entry, and a `ductus-v*` tag — a `runtime/src/` change reaches no adopter without it

- **Done when**: `compute-review-scope` returns the union; a review of a follow-on scenario on a mature spec includes the files that scenario touched; the four prose surfaces that stated larger-of agree with shipped behavior; `cargo test`, `cargo clippy -- -D warnings`, `npx markdownlint-cli2` and `scripts/audit/run-all.sh` all clean; released under a `ductus-v*` tag.
