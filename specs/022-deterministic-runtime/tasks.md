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

- **Done when**: `/gov:plan`'s validation gate refuses to advance `clarified → planned` while no commit touches `specs/{feature}`, reporting a domain outcome naming the fix rather than an operational error; `derive-boundary` on an uncommitted spec dir returns an empty boundary plus guidance instead of `no commits found that touch specs/{feature}`, and enforcement stays fail-closed on that empty result; a seeded `write-boundary` still admits the walk via the seeded ∪ derived union; the markdown-only prose for both commands carries the same guidance; tests cover the uncommitted-dir gate refusal and the empty-boundary outcome; `cargo test` green.

## 71. Implement scenario: [unchecked-done-when-clause-tally](scenarios/unchecked-done-when-clause-tally.md)

- [x] Implement the behavior described in `scenarios/unchecked-done-when-clause-tally.md`

- **Done when**: `mark-task` ticks an unchecked checkbox-form `Done when` clause once every real subtask of that task is complete, leaving the block visually coherent without adding the clause to the subtask index space (a two-subtask task still reports total 2 and `--subtask-index 2` stays out of range); `/gov:implement`'s per-task report distinguishes "all subtasks checked" from "task block fully checked" so an unticked clause is surfaced rather than rounded up; the checked, canonical-bold, and bulletless forms are unaffected; completing an already-complete task produces no diff; tests cover the unchecked-clause tick, the index-contract invariance, and idempotence; `cargo test` green.

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

- **Done when**: `schema/paths.rs` exposes `resolve_config(repo) -> (PathBuf, &'static str)` performing one existence probe for both the read path and the provenance name; `discover_rule_files::load_govern_toml` and `dashboard::load_config` each return the resolved name alongside the parsed content and their render paths consume it instead of re-probing via `config_display_name`; `DashboardConfig`'s serialized shape is unchanged so no MCP golden re-blesses; `config_path` / `config_display_name` survive for single-half callers with doc comments pointing both-halves callers at `resolve_config`; `cargo test` green

## 80. Implement scenario: [sibling-symlink-trust-boundary](scenarios/sibling-symlink-trust-boundary.md)

- [x] Implement the behavior described in `scenarios/sibling-symlink-trust-boundary.md`

- **Done when**: `check_artifacts.rs` gains `traverses_symlink(target, base)` using `std::fs::symlink_metadata` over every component at or below the feature directory, so the answer never depends on a link's destination and repeat runs stay byte-identical; the up-front scenario-readability pass short-circuits the read for a linked entry so its destination is never opened; `read_target_state` tests before `is_file()` and returns the existing `target-unparseable` reason; lexical resolution and the `starts_with` containment test are unchanged; a `#[cfg(unix)]` regression test asserts a symlinked sibling lands in `skipped` with `target-unparseable` and that `../../../etc/passwd` still resolves to `None`; `cargo test` green
