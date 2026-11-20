---
next-task: 48
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

- [ ] `CFG-ENV-013`: a credential for a throwaway local or test resource that grants no access to a production or shared system is not a secret under this rule, stated in the Statement and the Verification (a `docker-compose.yml` `POSTGRES_PASSWORD: postgres`, a committed `.env.test` key for a local emulator). The encrypted-copy permission stays as strict as it is
- [ ] The blockquote in `spec.md` saying `SEC-DEPS-001` takes over the deprecated `BE-DEPS-006` is corrected, since spec 008 removes that never-tagged stub now; first check no criterion in another spec asserts the signpost
- [ ] Left as it is: `CFG-ENV-011` stays a MUST

- **Done when**: each item is stated in the rule file as decided, the spec records the requirement (scenario `a-throwaway-credential-is-not-a-secret`, and a Resolved Questions entry per decision with its reason) and corrects any criterion the change makes false, any external claim the new text makes is fetched and not recalled, bytes before and after are measured per rule and per file, `runtime/CHANGELOG.md` `[Unreleased]` gains an entry per changed rule, the whole `cargo test` and both artifact workflows' steps pass, and the spec's next review and analysis read the change
