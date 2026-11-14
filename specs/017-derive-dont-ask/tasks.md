---
next-task: 47
---

# 017 — Derive, Don't Ask Tasks

Tasks derived from the [plan](plan.md). Complete in order.

The work is grouped into seven phases. Each phase ends with a natural commit boundary. Tasks within a phase can usually be done in one session; cross-phase dependencies are noted at the phase level.

## Phase A — Follow-on scenarios

### 46. The hook stops a commit whose markdown `--fix` cannot repair (owner decision, 2026-11-13)

- [ ] `.githooks/pre-commit` keeps `markdownlint-cli2 --fix`'s output and stops the commit, printing the violations, when the repository's own `node_modules/.bin/markdownlint-cli2` ran and exited 1; the `npx` fallback stays swallowed, since its exit 1 can also be a failed fetch. Found when `a2a0952d` and `6a587130` each committed an MD028 the hook hid, and framework-checks step (c) failed on the push
- [ ] The hook's header comment and its markdown block comment, `plan.md` §Technical Decisions' hook paragraph, and `spec.md`'s description of the hook's markdown step state the blocking behavior
- [ ] `scripts/tests/test-pre-commit-staging.sh` holds a case whose staged MD028 the hook refuses, and the case fails with the behavior removed

- **Done when**: A commit staging a blank line inside a blockquote (MD028) is refused by the hook naming the violation, a commit carrying only violations `--fix` repairs still lands with them fixed, and `generators.yml`'s and `framework-checks.yml`'s steps pass locally.
