# 022 — Deterministic Runtime Tasks

Tasks derived from the [plan](plan.md). Complete in order. Each task is small enough to complete and verify in a single session; later tasks depend on earlier ones.

## 129. The pi command candidate

- [x] Implement the behavior described in `scenarios/the-pi-command-candidate.md`
- [x] Add {cli-config-dir}/prompts/{project}-{command_name}.md as the last candidate of Host::command_file_candidates (064), keeping the plural-then-singular order of the two pre-existing shapes
- [x] Extend the command_file_candidates_cover_both_layouts_plural_first test to the third element and add the .pi session-fixture test (the flat project-hyphenated form resolves last)
- [x] Sync the config-dir enumerations in the write-session doc comment and the check-corpus-links generated-copies comment to include .pi
- [x] Record the three-shape resolution order in 022's data-model.md under Per-project file resolution

- **Done when**: the third candidate resolves last with the pre-existing two in order, both pinned by tests, and the enumerations and data-model carry the shape. The release that ships it is 064's task 13, cut only after 022 returns to done.

## 130. Extend check-command-flags with the argument-hint token direction

- [x] Implement the behavior described in `scenarios/argument-hint-needs-a-token.md`

- **Done when**: check-command-flags reports a declared argument-hint whose body holds no substitution token; target/link/prune carry the token; and Family 30 is green with the new denominator on stderr.

## 131. Implement scenario: [every-agents-generated-copies-are-excluded](scenarios/every-agents-generated-copies-are-excluded.md)

- [x] Implement the behavior described in `scenarios/every-agents-generated-copies-are-excluded.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 132. Implement scenario: [a-multi-number-renumber-is-one-rewrite](scenarios/a-multi-number-renumber-is-one-rewrite.md)

- [x] Implement the behavior described in `scenarios/a-multi-number-renumber-is-one-rewrite.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 133. Implement scenario: [per-agent-scaffold-paths-ship-to-adopter](scenarios/per-agent-scaffold-paths-ship-to-adopter.md)

- [x] Implement the behavior described in `scenarios/per-agent-scaffold-paths-ship-to-adopter.md`

- **Done when**: `criterion-path-existence` records a `ships-to-adopter` skip, not a finding, for a criterion naming any registered agent's `ductus` install path, settings file, or project-scoped MCP target — pinned by tests that include an agent not dogfooded here and a cell carrying a trailing note — while a path no registry row derives still flags, and an unreadable registry leaves the Shared Files contribution intact.

## 134. Implement scenario: [a-renamed-file-contributes-its-rewrites](scenarios/a-renamed-file-contributes-its-rewrites.md)

- [x] Implement the behavior described in `scenarios/a-renamed-file-contributes-its-rewrites.md`

- **Done when**: `SweepIndex::build` and Family 19 both detect renames with the same explicitly stated threshold and rename limit, neither inheriting git or libgit2 defaults nor `diff.renames`; `mechanical_sweep_parity` carries a renamed-directory fixture asserting both halves derive the same pairs including the renamed files' pairs, and fails when either half falls back to delete-plus-add.

## 135. Implement scenario: [a-revision-qualified-path-is-not-a-working-tree-reference](scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md)

- [x] Implement the behavior described in `scenarios/a-revision-qualified-path-is-not-a-working-tree-reference.md`

- **Done when**: the scenario's described behavior is correctly implemented and tested.

## 136. Implement scenario: [review-scope-includes-uncommitted-work](scenarios/review-scope-includes-uncommitted-work.md)

- [x] Implement the behavior described in `scenarios/review-scope-includes-uncommitted-work.md`

- **Done when**: `compute-review-scope` diffs from the diff base to the working tree (index and untracked included, ignored excluded), and resolves the base to `HEAD` when `spec.md` on disk reads `in-progress` and the spec at `HEAD` does not. Tests cover an uncommitted edit, a staged edit, an untracked file, an ignored file, `--since=HEAD`, an uncommitted `done → in-progress` reopen, an uncommitted `planned → in-progress` start, and an uncommitted `in-progress → done` flip; each test of changed behavior fails against the old code.

## 137. Implement scenario: [fetch-archive-names-its-timeout](scenarios/fetch-archive-names-its-timeout.md)

- [ ] Implement the behavior described in `scenarios/fetch-archive-names-its-timeout.md`

- **Done when**: `fetch_archive.rs` declares `FETCH_TIMEOUT` (30 s) beside the module's other bounds and sets it on the client it builds; a test drives the client with a short timeout against a proxy that never answers and fails when the client ignores the timeout it is given; plan.md's transport section names the constant and states the bound as the wait for the response and each body read.

## 138. Correct plan.md's YAML safe-loading sentence

- [ ] plan.md's `serde_norway` bullet says YAML is deserialized only into typed structures; five modules also parse into `serde_norway::Value` — state both, and why a generic value is still safe loading

- **Done when**: the bullet names both deserialization targets, and its safe-loading claim rests on what `serde_norway` does with a tagged node rather than on the typed-only premise.
