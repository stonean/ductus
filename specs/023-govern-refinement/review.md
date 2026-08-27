---
spec: 023-govern-refinement
scenario: configure-permission-pattern-safety
reviewed-at: 2026-08-27T15:34:10Z
reviewed-against: 1714745696474f85350463ec30effe3c84a15af3
diff-base: d0425a87c4e6da3e4d3c82dec2088151c8cad235
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 2
skipped-passes: []
---

# Review — 023-govern-refinement

## Summary

Re-run to bind the review to the commit that carries the work. The prior run recorded `reviewed-against: d0425a87` while the reviewed code sat uncommitted on top of it; once `1714745` landed, Family 19 (review freshness) correctly reported the spec's review as stale, because `scenarios/configure-permission-pattern-safety.md` — a durable contract — changed after the recorded sha. The local pre-commit gate had passed only because the scenario was still uncommitted and therefore invisible to that diff. This run records `reviewed-against: 1714745`.

The reviewed content is byte-identical to the prior run's: no source changed between the working tree that was reviewed and the commit that captured it. The findings therefore stand unchanged at 0 MUST, 0 SHOULD.

Scope resolves properly now that the work is committed. `--since=d0425a87` yields a `modified-since` of exactly the twelve committed files rather than the prior run's two failure modes — the default window's 170 files of unrelated 048/049/050 work, and `--since=HEAD`'s empty set. The passes covered `scripts/audit/permission-wildcard-position.sh` (new, ~155 lines of bash), its registration in `run-all.sh` / `framework/commands/audit.md` / `scripts/audit/README.md`, and the prose edits to `framework/bootstrap/configure/claude.md` plus its generated mirror.

Security: the change removes an arbitrary-execution hole rather than adding surface — the new script reads files only, with no eval and no user-controlled input. Reuse: correctly sources `lib.sh` for `ROOT` / `drift` / `emit`; no shared markdown-section helper exists to duplicate. Quality: extraction verified complete — all 23 `Bash(...)` allow bullets are matched, none dropped by the sed pattern, and the `IFS=:` read preserves colons in entry text. QUAL-CLAIM-001 is satisfied by construction: an empty extraction on a file that exists is a finding rather than a pass, skipped hosts are named on stderr, and the clean-exit line quantifies what was examined instead of asserting a global property. Efficiency: two bounded file scans. Simplicity: the one finding raised in the prior run — a two-step section-bound derivation whose intermediate list number was never used — was fixed before that report and remains fixed here, replaced by a single `grep -nE '^[0-9]+\.'` that keeps the bounds derived.

Verification re-confirmed against the committed tree: clean run reports 23 + 29 entries and exits 0; the negative test (reintroducing `Bash(git -C * status *)`) exits 1 and reports `claude.md:42`; shellcheck clean; full audit suite green across all 29 families.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] bug: `lint-markdown` fails whenever `npx` is a shell function rather than a binary — the primitive spawns `npx` directly, so nvm's lazy-loader shim (where `command -v npx` prints `npx` with no path) yields `I/O error on <repo>: No such file or directory (os error 2)`. Hits both the MCP tool at runtime and `runtime/tests/mcp.rs:86` (`lint_markdown_returns_violations_array`), which fails on a clean tree in such an environment; the reported path is the repo root, which misdirects toward a missing fixture rather than a missing executable. Consider resolving the binary via a login shell or `node_modules/.bin` lookup, and surfacing spawn failure as "npx not found" distinctly from a fixture I/O error (captured during 023-govern-refinement task 21)
- [ ] convention: the `permissions.allow` section of `configure/claude.md` now contains a literal `Bash(git -C * status *)` inside its rationale prose, as the counter-example explaining why the seven `-C` entries were removed. On the markdown-only path an agent builds the canonical allow array by reading this section, and one that pattern-matches `Bash(...)` loosely rather than reading only the bulleted entries could re-add the exact entry the prose forbids. Family 29 anchors to the bullet form and is unaffected; the exposure is the host-side read. Consider whether the counter-example should sit outside the canonical section, or whether the markdown-only prose should state that only bullet entries are canonical. — `framework/bootstrap/configure/claude.md:50` (captured during review of 023-govern-refinement)

## Observations

- convention: the `permissions.allow` section of `configure/claude.md` now contains a literal `Bash(git -C * status *)` inside its rationale prose, as the counter-example explaining why the seven `-C` entries were removed. On the markdown-only path an agent builds the canonical allow array by reading this section, and one that pattern-matches `Bash(...)` loosely rather than reading only the bulleted entries could re-add the exact entry the prose forbids. Family 29 anchors to the bullet form and is unaffected; the exposure is the host-side read. Consider whether the counter-example should sit outside the canonical section, or whether the markdown-only prose should state that only bullet entries are canonical. — `framework/bootstrap/configure/claude.md:50`

## Skipped passes

*None.*
