---
section: "Behavior"
---

# Generator-sync-claim-honesty

## Context

`gen-spec-deps.sh`, the shell generator `derive-dependencies` later replaced, printed `No changes (all specs in sync)` whenever its rewrite count was zero. Zero meant "I rewrote nothing", not "everything is in sync": [tracked-specs-not-worktree](tracked-specs-not-worktree.md) scopes enumeration to `git ls-files`, so an untracked draft is never examined — and the message made a positive claim about exactly the files the derivation could not vouch for.

An adopter hit this on 2026-08-01. Running the generator by hand right after `/{project}:specify` printed "all specs in sync" while `dependencies` stayed `[]`; after `git add`, the same run derived the three real dependencies. Their mechanism diagnosis was exact, and their remedy — revert the tracked-files exclusion — was wrong, which is why this scenario exists to correct only the reporting.

**The exclusion is specified behavior and must not be reverted.** The original worktree-glob implementation would rewrite an untracked draft's frontmatter on any unrelated commit, force-`git add` it into that commit, and, with a circular link in a half-written draft, fail the cycle check under `set -e` and block the unrelated commit outright. Skipping untracked drafts is the fix for a worse problem.

This is the defect class the framework names as `QUAL-CLAIM-001` — a fully-implemented path whose *output* overstates what it verified.

## Behavior

**A derivation does not assert what it did not examine.** `derive-dependencies` and `derive-references` report three sets rather than a global in-sync claim: the tracked specs they read (`examined`), the untracked specs they skipped (`untracked-skipped` — `git add` to include), and the specs they examined, found drifted, and deliberately left unwritten because they are not staged (`unwritten`). An empty `updated` beside a non-empty `untracked-skipped` or `unwritten` is not "in sync", and the result says so. The tracked-files exclusion itself is unchanged — this is reporting, not behavior.

**The third set exists for both derivations.** A `references:` entry depends on the `[services]` registry as well as the body, so renaming a service alias drifts every referencing spec without touching or staging any of them. `derive-references` therefore enumerates every tracked spec and filters only the write, exactly as `derive-dependencies` does; [067's `derive-references-unstaged-drift-is-reported`](../../067-analyze-and-audit-checks/scenarios/derive-references-unstaged-drift-is-reported.md) is the authority for that primitive's behavior.

**The two remaining shell generators are assessed, not assumed.** `gen-help-tables.sh` ("help.md in sync") and `gen-configure-mcp.sh` ("mcp-allow blocks in sync") share the message *shape* but regenerate from fixed sources rather than by enumerating specs. Each is checked against the same question — can its zero count ever mean "did not examine?" — and its message is corrected only if the answer is yes. A uniform edit applied without that check would be its own unfounded claim.

### Outcome of that assessment (2026-08-16)

`/ductus:review` on [013](../../013-text-first-artifacts/review.md) performed it. The answers differ, which is why the uniform edit this scenario warns against would have been wrong in both directions.

**`gen-configure-mcp.sh` — no; its message stands.** It processes a fixed set of four agent-source files unconditionally, compares each with `cmp`, and exits 4 when the tool manifest yields zero tools. Its subject is always fully examined, so a zero count cannot mean "did not examine". This is the rule's documented compliant case: *a documented total function whose subject is always fully examinable.* Left unchanged — correcting it would have been the unfounded claim in reverse.

**`gen-help-tables.sh` — yes; corrected.** It built its five tables from a command list hardcoded in the script rather than from `framework/commands/`, so a command that existed but was unlisted was never examined while the run still reported `No changes (help.md in sync)` at exit 0. Reproduced by adding a scratch command file: the generator reported sync while `help.md` never mentioned it. Nothing else covered the gap — `help.md` appears in `scripts/audit/` only inside a prose comment in `installer-command-parity.sh`, whose subject is `ductus.md`'s installer manifest, and whose header concedes help.md merely *"tends to get updated"* — the author-diligence dependency the framework forbids.

The correction is a verified claim rather than a reworded one: the command groups are arrays feeding both the rendered tables and a coverage assertion against the directory (minus the same maintainer-only exclusion `installer-command-parity.sh` uses), and an unlisted command now exits 6 naming the command and the remedy. The message names its subject — `No changes (N command(s) in sync)`. Because `check-zero` runs this generator, `/ductus:audit` and the release gate inherit the check.

**The reporting is tested.** `derive-dependencies`' unit tests assert `examined`, `untracked-skipped` and `unwritten` against a fixture holding an untracked draft and an unstaged drifted spec, and the golden test `untracked_specs_are_never_rewritten_and_are_reported` holds the untracked case end to end.

## Edge Cases

- Every spec tracked: `untracked-skipped` is empty.
- No specs at all: `examined` is zero, so the result claims nothing about an empty set.
- The pre-commit hook stages before running, so a commit always resolves the untracked case — the fix is for the manual invocation, where the reporter lost time.
- A run that rewrote something is unaffected: the claim only misleads on the zero-count path.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
