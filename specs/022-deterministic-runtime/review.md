---
spec: 022-deterministic-runtime
reviewed-at: 2026-08-30T19:54:06Z
reviewed-against: df072b17da4db2e2b8b8c4d6c72b95884e9c5075
diff-base: 1ab47904dd7cd253abd93eed6e1451b2bdd0bc96
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 3
skipped-passes: []
---

# Review — 022-deterministic-runtime

## Summary

Scope was `check-corpus-links` (spec 022 scenario `adopter-corpus-link-integrity`) and its five registration sites, plus the two pre-commit hooks that invoke it. The runtime primitives under `runtime/src/primitives/` belonging to spec 052 — the annotation writer, the `retire-feature` gate, and the `supersession-reciprocity` family — are in this window by scope union but are 052's deliverable and are reviewed there.

All five passes ran. The quality pass found three defects in the new primitive, every one of them the check failing the contract it was written to enforce: an unbounded directory walk that `is_dir()`'s symlink-following turns into a stack overflow; an unlistable subdirectory contributing nothing silently, which is `QUAL-CLAIM-001` committed by the check built to catch it; and a root-absolute link target resolved against the filesystem root rather than the repo root, so `/specs/x.md` was tested against the wrong path entirely. All three were inside the reviewed task's own scope, so they were fixed rather than recorded — `100c289`, with two added tests (18 total on the primitive). The hook call moved to the end of both hooks in the same change: it is the only read-only step, and a blocking read placed before the staging block left a blocked commit with the derived rewrites unstaged, so a blocked commit and a passing one left the tree in different states.

Nothing outstanding. The security, reuse, efficiency, and simplicity passes produced no findings: the primitive is read-only, opens no network or process, resolves paths lexically rather than through the filesystem, and reuses the runtime's tested `inline_code_spans` and `is_frontmatter_fence` scanners rather than re-implementing markdown structure parsing. Two observations map to no loaded rule and are captured to the inbox.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] Chore: sweep the README for current functionality — add /fold to the Commands section, document /specify's --branch, --branch-id, and --fold-into flags (other rows document theirs), cover the branch-scoped {identifier}.{n}-{slug} numbering form, and note that /status reports pending folds. Leave /audit absent — it is maintainer-only by design. The recurrence guard is homed at 026 scenario readme-command-parity; this is the one-time correction it will first surface.
- [ ] other: `merge-permissions` `revoke` serves only the Claude permission shape, so the other three hosts have no retirement path — a formerly-canonical entry that ever ships in `configure/opencode.md`, `configure/auggie.md`, or `configure/antigravity.md` would survive in adopter trees indefinitely, the exact gap spec 023's retirement just closed for `claude.md`.
- [ ] convention: spec 051's `data-model.md` still states `retire-feature`'s `feature` argument as "Must parse as `BranchScoped`; a sequential feature is refused", which 052 made incomplete — the refusal is now gated behind an explicit opt-in that `/{project}:consolidate` passes.

## Observations

- bug: `scripts/audit/broken-relative-links.sh` (Family 26) resolves a root-absolute link target against the filesystem root, not the repo root — `os.path.join(here, '/specs/x.md')` discards `here` entirely, exactly the defect just fixed in `check-corpus-links`. No such link exists in this corpus today, so the family is not currently wrong about anything; it would be the moment one is written, and it would be wrong in the dangerous direction on a machine that happens to hold that absolute path. — `scripts/audit/broken-relative-links.sh`
- other: Family 26 and the new `check-corpus-links` primitive now perform substantially the same check over overlapping subjects — the family covers the whole repository including maintainer-only files, the primitive covers the spec corpus and is what adopters actually run. The scenario deliberately left delegation undecided ('Family 26 is not necessarily retired by this'), and the Family 30 shape — logic in the runtime, script as entry point — is the obvious candidate. Deciding it would also collapse the divergence recorded in the observation above, since there would be one implementation to fix rather than two to keep in step. — `scripts/audit/broken-relative-links.sh`

## Skipped passes

*None.*
