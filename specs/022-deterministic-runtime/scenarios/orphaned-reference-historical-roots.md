---
section: "Follow-on scenarios"
---

# Orphaned-reference-historical-roots

## Context

`check-orphaned-references` exists to catch what a migration chain leaves behind: a reference in an adopter-owned file to a framework path that has since moved. Required by [027](../../027-bootstrap-migration-registry/spec.md)'s `migration-chain-reference-integrity` and surfaced at the batch end of `/{project}`'s Pre-run Migrations and in `/{project}:analyze`.

It could not see the orphan it was written for. `managed_roots` returned only the **current** roots — `.ductus/`, `.githooks/`, and the configured spec root — and a reference is examined only when it carries one of them as a prefix. But the orphan a chain creates is a reference to the path *before* the move, which by definition carries the *old* root. The check was therefore blind in exactly the dimension that mattered.

Measured on the real adopter bootstrap for [048](../../048-govern-acquired-runtime/spec.md)'s AC10, 2026-08-17. The generators moved root `scripts/` → `.govern/scripts/` → `.ductus/scripts/` across `govern-dir-consolidate` and `ductus-rename`. The adopter's `AGENTS.md` still named `scripts/gen-spec-deps.sh`. The primitive returned clean across all four referrers, and the operator found the stale path by reading the file. Nothing in the pipeline did.

The blind spot was known and worked around rather than fixed: this primitive's own test carried the comment *"`.govern/` is not a managed root, so the stale reference must be caught by naming a path under one that does not resolve"*, and constructed its fixture accordingly. A test that routes around the gap documents it.

## Behavior

**Historical managed roots are matched alongside current ones.** `managed_roots` gains `.govern/` — the 042-era per-project directory retired by `049` — and the pre-042 generator locations. A reference to a path under any of them that does not resolve is a finding, exactly as for a current root.

**The pre-042 entries are prefixes, not a directory.** `scripts/gen-` and `scripts/lib/` rather than `scripts/`, because the framework owned those and never the whole directory. A `scripts/` root would make every unresolved adopter script a finding — noise this check must not produce, and the reason the current-roots list was scoped in the first place.

**The result declares the prefixes it matched.** A new `matched-prefixes` field carries the list the run actually used. `examined` already bounds the claim by *subject* — which referrers were read; this bounds it by *scope* — which reference forms were recognized. The two are not interchangeable: a reference carrying no listed prefix is not reported under `skipped`, because nothing recognized it as a reference at all, so a clean result without this field asserts *no orphans* while meaning *no orphans among paths carrying one of these prefixes*.

**Nothing else about the check changes.** Read-only, reports rather than repairs, same referrer set, same pattern and adopter-destination exemptions, same `attribution` semantics.

## Edge Cases

- A reference to a path under a historical root that **does** resolve — an adopter who kept a `.govern/` directory deliberately — is not a finding, the same as for a current root: the check reports unresolved references, not retired vocabulary.
- An adopter-owned `scripts/build.sh` that does not resolve: not a finding, and covered by a test, because it carries neither historical prefix.
- A reference nested under a longer path, `.ductus/scripts/gen-spec-deps.sh`: matched once under `.ductus/`, not twice, because the existing `preceded_by_path_char` guard rejects a match that begins mid-token. That guard is what keeps `scripts/gen-` from double-reporting every current-root generator reference.
- This repository's own referrers: verified to produce zero new findings, since every `scripts/gen-` occurrence in `AGENTS.md`, `README.md` and `.githooks/pre-commit` sits inside a longer `.ductus/scripts/…` path and is skipped by that guard.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
