---
next-task: 51
---

# 017 — Derive, Don't Ask Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## Phase A — Follow-on scenarios

### 46. The hook stops a commit whose markdown `--fix` cannot repair (owner decision, 2026-11-13)

Landed on 2026-11-14 after 000's review record; with the real `markdownlint-cli2`, a staged MD009 was fixed in the index and a staged MD028 refused, the index left as staged. Two facts the implementation found: `xargs` reports every exit from 1 to 125 as 123, so the linter is called with an argument array to see its exit 1; and with this repository's config, `markdownlint-cli2` lints every markdown file the config's globs name, not only those passed, so any repair-proof violation in the tree stops a commit.

- [x] `.githooks/pre-commit` keeps `markdownlint-cli2 --fix`'s output and stops the commit, printing the violations, when the repository's own `node_modules/.bin/markdownlint-cli2` ran and exited 1; the `npx` fallback stays swallowed, since its exit 1 can also be a failed fetch. Found when `a2a0952d` and `6a587130` each committed an MD028 the hook hid, and framework-checks step (c) failed on the push
- [x] The hook's header comment and its markdown block comment, `plan.md` §Technical Decisions' hook paragraph, and `spec.md`'s description of the hook's markdown step state the blocking behavior
- [x] `scripts/tests/test-pre-commit-staging.sh` holds a case whose staged MD028 the hook refuses, and the case fails with the behavior removed

- **Done when**: A commit staging a blank line inside a blockquote (MD028) is refused by the hook naming the violation, a commit carrying only violations `--fix` repairs still lands with them fixed, and `generators.yml`'s and `framework-checks.yml`'s steps pass locally.

### 47. The owner's rule decision of 2026-11-19 for `framework/rules/configuration-cross.md`

- [x] `CFG-ENV-013`: a credential for a throwaway local or test resource that grants no access to a production or shared system is not a secret under this rule, stated in the Statement and the Verification (a `docker-compose.yml` `POSTGRES_PASSWORD: postgres`, a committed `.env.test` key for a local emulator). The encrypted-copy permission stays as strict as it is
- [x] The blockquote in `spec.md` saying `SEC-DEPS-001` takes over the deprecated `BE-DEPS-006` is corrected, since spec 008 removes that never-tagged stub now; first check no criterion in another spec asserts the signpost
- [x] Left as it is: `CFG-ENV-011` stays a MUST

- **Done when**: each item is stated in the rule file as decided, the spec records the requirement (scenario `a-throwaway-credential-is-not-a-secret`, and a Resolved Questions entry per decision with its reason) and corrects any criterion the change makes false, any external claim the new text makes is fetched and not recalled, bytes before and after are measured per rule and per file, `runtime/CHANGELOG.md` `[Unreleased]` gains an entry per changed rule, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change

### 48. A path-list commit leaves the shared index current (deferred from 083 task 7, operator decision 2026-11-23)

- [x] Implement the behavior described in `scenarios/a-path-list-commit-leaves-the-shared-index-current.md`
- [x] `.githooks/pre-commit` carries each generated output it adds to a path-list commit into the shared index as well, leaving every other shared-index entry as it was
- [x] `scripts/tests/test-pre-commit-staging.sh` holds the path-list case, and it fails with the behavior removed
- [x] Correct `docs/developing-ductus.md` §Running a second session, whose account of a pathspec commit leaving the old generated copies staged in the main index, and its `git restore --staged` remedy, this behavior makes false; until this task lands, that account and remedy hold for every session in this tree

- **Done when**: after `git commit -- <paths>` whose staged sources regenerate an output, `git diff --cached` names no carried output, the test case proves it in both directions, and `generators.yml`'s and `framework-checks.yml`'s steps pass locally

### 49. The hook's runtime checks never write the commit's index (owner decision, 2026-11-25)

- [x] `.githooks/pre-commit` runs `cargo fmt --check`, `cargo clippy` and `cargo test` with `GIT_INDEX_FILE`, `GIT_DIR` and `GIT_WORK_TREE` unset, so a test that runs git in a fixture repository can never write the index of the commit in hand. Found 2026-11-25: a commit of 066's review from a linked worktree with `runtime/tests/golden/review-basic.jsonl` staged passed every runtime check, then held 7 fixture entries (`hooks/pre-push`, `specs/001-a/spec.md`, …) naming objects the repository lacks, and the hook's size check could not read it; inferred, not yet run: the tests inherit the hook's `GIT_INDEX_FILE`, which is absolute in a linked worktree
- [x] `scripts/tests/test-pre-commit-staging.sh` holds a case in which a commit from a linked worktree with a `runtime/` path staged keeps its index entries, the runtime checks stubbed, and it fails with the variables left set

- **Done when**: a commit from a linked worktree with a `runtime/` path staged leaves its index entries as they were, the new case proves it and fails with the variables left set, and `framework-checks.yml`'s steps pass locally.

### 50. A path-list commit carries every file the hook rewrites into the shared index (owner decision, 2026-11-25)

- [ ] Widen `scenarios/a-path-list-commit-leaves-the-shared-index-current.md` from generated outputs to every file the hook rewrites, a markdown file `markdownlint-cli2 --fix` changed among them: task 48's `carry_to_shared` in `.githooks/pre-commit` filters to `$generated_outputs`, so a named markdown file `--fix` rewrote in `git commit -- <paths>` still leaves its pre-fix copy staged in the shared index (found by code reading while landing task 48, 2026-11-25)
- [ ] `carry_to_shared` drops the generated-outputs filter; the starting-blob guard still leaves another session's staged entry as it was
- [ ] `scripts/tests/test-pre-commit-staging.sh` holds a path-list case for a markdown file `--fix` rewrites, and it fails with the filter restored

- **Done when**: after `git commit -- <paths>` naming a markdown file the hook's `--fix` rewrites, `git diff --cached` names no carried file, the case proves it in both directions, and `generators.yml`'s and `framework-checks.yml`'s steps pass locally.
