---
spec: 022-deterministic-runtime
scenario: lint-markdown-tool-resolution
reviewed-at: 2026-08-27T18:18:22Z
reviewed-against: 530dc7a377de3630a73765cd03b0ad55cb1cd06b
diff-base: 8393841
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reviewed at `530dc7a`. 0 MUST, 0 SHOULD.

Re-run because `ReviewStale` blocked the transition: the prior review recorded `27ec059`, and `530dc7a` then corrected a relative-link depth in `scenarios/lint-markdown-tool-resolution.md` — a durable contract. The gate refused the `done` transition and named the file, which is the check working rather than a problem: a one-character path fix is still an edit to the artifact a review reads. The link error itself was caught by Family 26 only *after* the commit, because that family enumerates tracked files and the new scenario was untracked before it — the same git-visibility property that makes "re-run the audit after committing, before tagging" a rule rather than a suggestion. Content is otherwise unchanged from the `27ec059` review, whose findings stand.

Scope: `runtime/src/primitives/lint_markdown.rs` (tool resolution, the reworked spawn-failure path, six new tests), `runtime/src/primitives/mod.rs` (the new `ToolLaunch` variant), the three version sites, the 022 scenario and task, plus two carried edits — the `configure/claude.md` canonical-boundary chore and the Family 29 script repair it exposed.

Security: the change reduces surface rather than adding it. The vendored-binary branch is a path existence check followed by a direct spawn — no shell, no profile sourcing, no widening of what `Command` already searched. The scenario's rejection of a login-shell resolution is the load-bearing decision: it would have been the expedient fix and a genuine code-execution surface, since sourcing a contributor's profile to locate a linter runs arbitrary shell under this primitive's permission. The existing leading-`-` guard (which blocks `--config` loading arbitrary JS) is untouched and still runs before resolution.

Quality: the guidance predicate is correctly narrow — `launch_guidance` fires only on `NotFound` **and** only on the `npx` branch, so neither a permissions error nor a broken vendored binary carries a `PATH` explanation it has no basis for. That would be this same misattribution pointed the other way, and two dedicated tests hold it. Extracting the predicate from the closure is what made it testable; forcing a real `ENOENT` would have meant manipulating the test process's environment. The unspawnable-binary test uses a directory rather than permission bits — reliably unspawnable everywhere, and not dependent on CI preserving modes.

Reuse and simplicity: `resolve_markdownlint` returns `(program, via_npx)` rather than branching at two call sites, and the Windows `.cmd` distinction moved into it so both branches state it once. `ToolLaunch` is a new variant rather than a reuse of `Io` because the two carry different subjects — `Io` names a path the primitive was operating on, and pressing it into service for a spawn failure is exactly what named the repository instead of the missing program. Efficiency: one added `Path::exists` per lint.

Verified end to end against the real repository: under `PATH=/usr/bin:/bin` the primitive reports `could not launch npx: … — not found on PATH…` with the nvm explanation, where it previously reported `I/O error on <repo>: No such file or directory`. `check-review-gate` inherits it unchanged.

Release state: all three version sites read 0.33.0 with a matching CHANGELOG section; Family 20 and `lint-release-ordering.sh` pass; goldens needed no re-bless and the release binary was rebuilt first. Full gate green — 1026 lib tests, parity 11/11, mcp 26/26; fmt and clippy clean; six lint scripts, shellcheck, markdownlint, and the 29-family audit all clean against committed state.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
