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

**A third state exists and is reported too.** `gen-spec-deps.sh` enumerates every *tracked* spec (its cycle check needs the whole graph) but writes only its rewrite targets, which under `--staged` are the staged specs alone. A tracked-but-unstaged spec whose derived field has drifted is therefore examined, found drifted, and deliberately left alone — neither "in sync" nor "not examined". A zero rewrite count reported it as the first, which is the same defect one level in:

```text
No changes (N tracked spec(s) in sync; D drifted spec(s) left unwritten — not staged; M untracked spec(s) skipped — git add to include)
```

Each clause appears only when its count is non-zero. This case was believed specific to `gen-spec-deps.sh`, on the reasoning that `gen-cross-service-refs.sh` writes every spec it enumerates, so for it "enumerated" and "written" are the same set and its claim was already sound.

**That reasoning was wrong, and the correction is recorded below** — see *The third state applies to both derivations (2026-08-27)*. The two generators are now the `derive-dependencies` and `derive-references` primitives, and both report the drifted-but-unwritten state.

**`gen-cross-service-refs.sh` gets the same treatment.** It enumerates through the same `list_specs()` and prints the same shape of claim about references, so it carries the identical defect and the identical fix.

**The other two are assessed, not assumed.** `gen-help-tables.sh` ("help.md in sync") and `gen-configure-mcp.sh` ("mcp-allow blocks in sync") share the message *shape* but regenerate from fixed sources rather than through `list_specs()`. Each is checked against the same question — can its zero count ever mean "did not examine?" — and its message is corrected only if the answer is yes. A uniform edit applied without that check would be its own unfounded claim.

### Outcome of that assessment (2026-08-16)

The assessment above was specified here but its result was never written down, so for two releases this scenario read as pending work and `quality-cross.md`'s `QUAL-CLAIM-001` Source note recorded both generators as *"have not been assessed against this rule"*. `/ductus:review` on [013](../../013-text-first-artifacts/review.md) performed it. The answers differ, which is why the uniform edit this scenario warns against would have been wrong in both directions.

**`gen-configure-mcp.sh` — no; its message stands.** It processes a fixed set of four agent-source files unconditionally, compares each with `cmp`, and exits 4 when the tool manifest yields zero tools. Its subject is always fully examined, so a zero count cannot mean "did not examine". This is the rule's documented compliant case: *a documented total function whose subject is always fully examinable.* Left unchanged — correcting it would have been the unfounded claim in reverse.

**`gen-help-tables.sh` — yes; corrected.** It built its five tables from a command list hardcoded in the script rather than from `framework/commands/`, so a command that existed but was unlisted was never examined while the run still reported `No changes (help.md in sync)` at exit 0. Reproduced by adding a scratch command file: the generator reported sync while `help.md` never mentioned it. Nothing else covered the gap — `help.md` appears in `scripts/audit/` only inside a prose comment in `installer-command-parity.sh`, whose subject is `ductus.md`'s installer manifest, and whose header concedes help.md merely *"tends to get updated"* — the author-diligence dependency the framework forbids.

The correction is a verified claim rather than a reworded one: the command groups are arrays feeding both the rendered tables and a coverage assertion against the directory (minus the same maintainer-only exclusion `installer-command-parity.sh` uses), and an unlisted command now exits 6 naming the command and the remedy. The message names its subject — `No changes (14 command(s) in sync)`. Because `check-zero` runs this generator, `/ductus:audit` and the release gate inherit the check.

**The reporting this scenario specifies is now tested.** Its three clauses had no assertion anywhere — `run_gen` captured stdout that no test read — so a regression to a bare "all specs in sync" would have gone unnoticed. Test R in `scripts/tests/test-gen-spec-deps.sh` covers the in-sync count, the skipped clause and its omission, and the drifted clause and its omission; reverting the message to its pre-scenario form fails three of its assertions.

### The third state applies to both derivations (2026-08-27)

The claim above — that the drifted-but-unwritten state is specific to the dependency derivation — held only for a field derived from the spec body alone, and `references:` is not one.

A `dependencies:` edge is a pure function of the body, so it can only drift when its own spec is edited, which stages that spec and makes it visible to a staged-mode run. A `references:` entry is a function of the body **and** the `[services]` registry: the harvest resolves each link's repo URL through that registry to produce the `service:` alias. Rename a service alias and every referencing spec drifts while none of them is touched — and an untouched spec is never staged.

So "enumerated equals written" was true of the reference derivation only because it had narrowed its enumeration to the staged set, which is precisely what made the drift unreportable. An adopter carried dead references for nine commits after a `[services]` rename while the pre-commit hook reported the tree in sync every time; the dependency derivation would have reported the same tree, on the same commit, as drifted.

The conclusion inverts: the reference derivation needs the full walk **more** than the dependency one does, not less. `derive-references` now enumerates every tracked spec and filters only the write, and its result carries `unwritten` — the same field, with the same meaning, as its sibling's. The corrected reasoning lives in [022's `derive-references-unstaged-drift-is-reported`](../../022-deterministic-runtime/scenarios/derive-references-unstaged-drift-is-reported.md), which is the authority for the primitive's behavior; this section exists so the scenario that asserted the opposite does not still read as current.

## Edge Cases

- Every spec tracked: the skipped clause is omitted and the message reads as it does today, with the count added.
- No specs at all: the message reports zero examined rather than claiming sync over an empty set.
- The pre-commit hook stages before running, so a commit always resolves the untracked case — the fix is for the manual invocation, where the reporter lost time.
- A generator that rewrote something is unaffected: the claim only misleads on the zero-count path.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
