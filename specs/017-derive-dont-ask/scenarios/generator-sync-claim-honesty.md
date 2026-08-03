---
section: "Behavior"
---

# Generator-sync-claim-honesty

## Context

`gen-spec-deps.sh` prints `No changes (all specs in sync)` whenever its rewrite count is zero. Zero means "I rewrote nothing", not "everything is in sync": [tracked-specs-not-worktree](tracked-specs-not-worktree.md) scopes `list_specs()` to `git ls-files`, so an untracked draft is never examined — and the message makes a positive claim about exactly the files the generator cannot vouch for.

An adopter hit this on 2026-08-01. Running the generator by hand right after `/{project}:specify` printed "all specs in sync" while `dependencies` stayed `[]`; after `git add`, the same run derived the three real dependencies. Their mechanism diagnosis was exact, and their remedy — revert the tracked-files exclusion — was wrong, which is why this scenario exists to correct only the reporting.

**The exclusion is specified behavior and must not be reverted.** The original worktree-glob implementation would rewrite an untracked draft's frontmatter on any unrelated commit, force-`git add` it into that commit, and, with a circular link in a half-written draft, fail the cycle check under `set -e` and block the unrelated commit outright. Skipping untracked drafts is the fix for a worse problem.

This is the same defect class the framework already names as `QUAL-CLAIM-001` — a fully-implemented path whose *output* overstates what it verified — applied to the generators rather than the runtime.

## Behavior

**A generator does not assert what it did not examine.** On a zero rewrite count, `gen-spec-deps.sh` reports what it actually enumerated and what it skipped, rather than a global in-sync claim:

```text
No changes (N tracked specs in sync; M untracked spec(s) skipped — git add to include)
```

The `M` clause is omitted when nothing was skipped, so the ordinary all-tracked case stays a clean one-line message. The exclusion itself is unchanged — this is a counting and reporting change, not a behavior change.

**`gen-cross-service-refs.sh` gets the same treatment.** It enumerates through the same `list_specs()` and prints the same shape of claim about references, so it carries the identical defect and the identical fix.

**The other two are assessed, not assumed.** `gen-help-tables.sh` ("help.md in sync") and `gen-configure-mcp.sh` ("mcp-allow blocks in sync") share the message *shape* but regenerate from fixed sources rather than through `list_specs()`. Each is checked against the same question — can its zero count ever mean "did not examine?" — and its message is corrected only if the answer is yes. A uniform edit applied without that check would be its own unfounded claim.

## Edge Cases

- Every spec tracked: the skipped clause is omitted and the message reads as it does today, with the count added.
- No specs at all: the message reports zero examined rather than claiming sync over an empty set.
- The pre-commit hook stages before running, so a commit always resolves the untracked case — the fix is for the manual invocation, where the reporter lost time.
- A generator that rewrote something is unaffected: the claim only misleads on the zero-count path.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
