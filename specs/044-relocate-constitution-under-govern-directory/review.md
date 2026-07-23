---
spec: 044-relocate-constitution-under-govern-directory
reviewed-at: 2026-07-23T03:19:20Z
reviewed-against: 500498bf9d37df47db706f557fcda5325a993b63
diff-base: 632325ef268d16f3e3443332ee831335d1875264
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 044-relocate-constitution-under-govern-directory

## Summary

Path-relocation sweep reviewed across all five dimensions against the 11 loaded rule files: 0 MUST, 0 SHOULD, 0 low-confidence. The diff contains markdown procedure/template/doc rewrites, the constitution-relocate registry entry and procedure file, and the no-behavior-change gvrn 0.24.0 version cut — no application code paths, so the surface rules (backend/frontend) cannot fire, and no CFG-*/QUAL-* pattern appears in the changed content. Deterministic checks corroborate: migrations.toml parses with all 8 entries and the audit's migration-coverage invariants pass; the stale-reference greps over framework/commands, templates, bootstrap, and own docs return only the sanctioned framework-source and .govern destination forms; scripts/audit/cross-doc-consistency.sh and ssot-invariants.sh exit 0; markdownlint 0 issues across 354 files; cargo test 852 passed with no golden re-bless (confirming the parity fixtures' constitution paths are synthetic and unaffected). The one issue captured during the window was resolved in-window — see Captured issues. Not blocking.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [auto-capture, 044 task 7] `specs/019-config-decisions/data-model.md` canonical config-schema doc still described the pre-042 layout (`.govern.toml` paths; pinned example predating 044) — resolved in-window by 652d48c (user-directed mechanical sweep + post-completion note); inbox entry cleared by the same commit

## Skipped passes

*None.*
