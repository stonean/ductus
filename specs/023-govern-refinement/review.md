---
spec: 023-govern-refinement
scenario: configure-permission-pattern-safety
reviewed-at: 2026-08-27T15:08:41Z
reviewed-against: d0425a87c4e6da3e4d3c82dec2088151c8cad235
diff-base: d0425a87c4e6da3e4d3c82dec2088151c8cad235
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 023-govern-refinement

## Summary

Task 21 only — reviewed against the uncommitted working tree at `d0425a8`, not the primitive's default window. `compute-review-scope`'s default diff-base is this spec's last review (`9c06b2d`, 2026-08-19), which sweeps 170 files of unrelated 048/049/050 work; `--since=HEAD` collapses `modified-since` to empty because the work is uncommitted, leaving only the old plan's file list, much of which no longer exists. Neither scoping targets the change, so the five passes ran over the working-tree diff directly: `scripts/audit/permission-wildcard-position.sh` (new, ~155 lines of bash), its registration in `run-all.sh` / `framework/commands/audit.md` / `scripts/audit/README.md`, and the prose edits to `framework/bootstrap/configure/claude.md` plus its generated mirror.

0 MUST, 0 SHOULD outstanding. Security: the change removes an arbitrary-execution hole rather than adding surface — the new script reads files only, with no eval and no user-controlled input. Reuse: correctly sources `lib.sh` for `ROOT` / `drift` / `emit` rather than re-declaring them; no shared markdown-section helper exists to duplicate. Quality: extraction verified complete — all 23 `Bash(...)` allow bullets in the section are matched, none silently dropped by the sed pattern, and the `IFS=:` read preserves colons in entry text. The script satisfies QUAL-CLAIM-001 by construction: an empty extraction on a file that exists is a finding rather than a pass, skipped hosts are named on stderr, and the clean-exit line quantifies what was examined instead of asserting a global property. Efficiency: two bounded file scans.

One simplicity finding was raised and fixed during the review rather than filed: the section bounds were derived in two steps — sed out the heading's list number, then re-grep by that number for its line — with the list number itself never used. Replaced by a single `grep -nE '^[0-9]+\.'`, which keeps the bounds derived (renumbering the command's steps moves the window rather than breaking it) without the intermediate. Re-verified after the change: clean run reports 23 + 29 entries and exits 0; the negative test (reintroducing `Bash(git -C * status *)`) still exits 1 and reports `claude.md:42`; shellcheck clean; full audit suite green across all 29 families.

Note on freshness: `reviewed-against` records HEAD, but the reviewed code is uncommitted. Re-run after committing if the frontmatter sha needs to name the reviewed state exactly.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] bug: `lint-markdown` fails whenever `npx` is a shell function rather than a binary — the primitive spawns `npx` directly, so nvm's lazy-loader shim yields `I/O error on <repo>: No such file or directory (os error 2)`. Hits both the MCP tool at runtime and `runtime/tests/mcp.rs:86` (`lint_markdown_returns_violations_array`), which fails on a clean tree in such an environment (captured during 023-govern-refinement task 21)

## Observations

- convention: the `permissions.allow` section of `configure/claude.md` now contains a literal `Bash(git -C * status *)` inside its rationale prose, as the counter-example explaining why the seven `-C` entries were removed. On the markdown-only path an agent builds the canonical allow array by reading this section, and one that pattern-matches `Bash(...)` loosely rather than reading only the bulleted entries could re-add the exact entry the prose forbids. Family 29 anchors to the bullet form and is unaffected; the exposure is the host-side read. Consider whether the counter-example should sit outside the canonical section, or whether the markdown-only prose should state that only bullet entries are canonical. — `framework/bootstrap/configure/claude.md:50`

## Skipped passes

*None.*
