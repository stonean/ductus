---
spec: 022-deterministic-runtime
scenario: lint-markdown-tool-resolution
reviewed-at: 2026-08-27T18:16:43Z
reviewed-against: 27ec059d499b41dd1f47be5f605df9024bceb508
diff-base: 8393841
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Reviewed at `27ec059`, the commit carrying the work. 0 MUST, 0 SHOULD.

Scope: `runtime/src/primitives/lint_markdown.rs` (tool resolution, the reworked spawn-failure path, six new tests), `runtime/src/primitives/mod.rs` (the new `ToolLaunch` variant), the three version sites, the 022 scenario and task, plus two carried edits — the `configure/claude.md` canonical-boundary chore and the Family 29 script repair it exposed.

Security: the change reduces surface rather than adding it. The vendored-binary branch is a path existence check followed by a direct spawn — no shell, no `PATH` search of adopter-controlled directories beyond what `Command` already did, and no profile sourcing. The scenario's rejection of a login-shell resolution is the load-bearing decision here: it would have been the expedient fix and a genuine code-execution surface, since sourcing a contributor's profile to locate a linter runs arbitrary shell under the primitive's permission. The existing leading-`-` argument guard (which blocks `--config` loading arbitrary JS) is untouched and still runs before resolution.

Quality: the guidance predicate is correctly narrow. `launch_guidance` fires only on `NotFound` **and** only on the `npx` branch, so neither a permissions error nor a broken vendored binary carries a `PATH` explanation it has no basis for — that would be the same misattribution this scenario removes, pointed the other way, and it is covered by two dedicated tests. Extracting it from the closure is what made it directly testable; forcing a real `ENOENT` from a spawn would have required manipulating the test process's environment. The unspawnable-vendored-binary test uses a directory rather than permission bits, which is reliably unspawnable on every platform and does not depend on CI normalizing modes.

Reuse and simplicity: `resolve_markdownlint` returns `(program, via_npx)` rather than branching at two call sites, and the Windows `.cmd` distinction moved into it so both branches state it once. `ToolLaunch` is a new variant rather than a reuse of `Io` because the two carry different subjects — `Io` names a path the primitive was operating on, and using it for a spawn failure is precisely what named the repository instead of the missing program. Efficiency: one added `Path::exists` per lint.

Verified end to end against the real repository, not fixtures alone: under `PATH=/usr/bin:/bin` the primitive now reports `could not launch npx: No such file or directory (os error 2) — not found on PATH…` with the nvm explanation, where it previously reported `I/O error on <repo>: No such file or directory`. The `check-review-gate` caller inherits this unchanged.

Release state: all three version sites read 0.33.0 with a matching CHANGELOG section; Family 20 and `lint-release-ordering.sh` pass; goldens needed no re-bless (version is a placeholder) and the release binary was rebuilt first. Full gate green — 1026 lib tests, parity 11/11, mcp 26/26, every other target; `cargo fmt --check` and `clippy --release --all-targets --locked -- -D warnings` clean; six lint scripts, `shellcheck -S warning` over the tracked shell set, `markdownlint-cli2 '**/*.md'`, and the 29-family audit all clean. The audit is re-run after this commit and before tagging.

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
