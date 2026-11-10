---
section: "Follow-on scenarios"
---

# Family-19-says-what-it-examined

## Context

Family 19 is the only one of the four `done`-spec families that reports nothing on a clean run.

Its three siblings each close their run with a coverage line naming what they looked at:

```text
done-spec-criteria: examined 54 spec(s) at status: done under specs/ (untracked skipped: 0)
review-block-agreement: compared 54 spec(s) carrying both a review block and a review.md; 0 carried only one …
analyze-record: 54 done spec(s) examined; 0 carry a review record and no analyze record …
```

*Two siblings now.* `057-analyze-artifact-and-record-relocation` retired Family 31
(`review-block-agreement`) along with the second copy of the record it
reconciled, so the middle line is quoted here as it stood when this scenario was
written. The **argument** it supports is unaffected and is arguably stronger for
the retirement: the surviving families still emit a coverage line, Family 19
still did not, and one of the three that shamed it into doing so has since been
removed for reporting agreement over a subject that could not disagree.

`review-freshness.sh` prints nothing and exits 0. The aggregator is built for that — `run_all`'s `run_check` emits a per-family header only on a non-zero exit — so silence is the designed pass shape. The problem is that silence is *also* the shape of a family that aborted before it examined anything, and `run_check` cannot tell the two apart either: it reads the exit code alone.

That is not a hypothetical failure mode in this repo. [`scripts/audit/sibling-coupling.sh`](../../../scripts/audit/sibling-coupling.sh) used a GNU awk extension, so on every macOS machine the extraction aborted, the family found nothing, and it exited 0 — a release gate that was dead locally and alive in CI. `AGENTS.md` §Design Principles recorded that as the single most expensive failure mode this repo has produced, until 072 cut the entry to a pointer at the constitution's §design-principles (`4eebe91a`), and the script itself cites `QUAL-CLAIM-001` while leaving its own result indistinguishable.

Observed 2026-09-07 while closing spec 022. The self-audit had to be trusted as the release gate, Family 19 returned zero bytes, and the only way to establish that it had run was to copy the script, instrument the enumeration loop, and re-run it. It had run — 54 specs, 022 among them. Nothing in the family's output said so, and nothing would have said otherwise.

## Behavior

**`review-freshness.sh` closes every run with one coverage line on stderr, clean or not.** It names the count it enumerated and the exclusions it applied, in the shape its siblings use:

```text
review-freshness: examined N spec(s) at status: done by reviewed-digest; G grandfathered (no review.md); U undeterminable; R never run; F not answered
```

The line is a **claim about the subject**, not a finding, so it does not affect the exit code; `run_check` passes stderr through above the findings it echoes from stdout. `run_check` keeps its current contract — non-zero means findings — and a clean Family 19 keeps producing no findings; what changes is that its output stops being empty.

Grandfathered, undeterminable, never-run and unanswered specs are named separately because they are the ways a spec can be *in* the corpus and *out* of the comparison, and folding them into the examined count would restate the conflation the line exists to remove. **The grandfather predicate is keyed on the absence of `review.md`, and that spelling is load-bearing rather than cosmetic.** It read *no `review:` block* until `057-analyze-artifact-and-record-relocation` moved the record, which made that predicate true of every spec in the corpus — so the family would have reported every spec grandfathered, examined nothing, and gone **green**, which is this scenario's own subject arriving through the predicate instead of through the exit code. The relocation re-pointed it at the artifact in the same change, and the coverage line is what would have made the failure visible: `examined 0; 54 grandfathered` is not a shape a reader mistakes for a clean gate.

### The comparison has one arm

**A record is judged by its `reviewed-digest`, the field `check-review-gate` reads, through the same runtime code** (066 plan, *Family 19 calls the runtime*), so the two enforcement points agree by construction. A record with no digest is undeterminable, a finding: nothing says what that review read. From 2026-09-07 such a record was judged by a commit-diff proxy instead, which carried a known false positive (a contract reviewed before it was committed); the line reported the digest/proxy split so a reader could see how many verdicts rested on that weaker claim, and named a proxy count of zero as the arm's exit condition. It reached zero (0 of 35 `done` specs, 2026-11-04), and the arm went with the family's own copy of the rule.

**The aggregator is unchanged.** Teaching `run_check` to demand a coverage line from every family would be the broader fix, and it is deliberately not taken here: three families already emit one voluntarily, the fourth is this one, and a framework-wide contract is its own scenario with its own measurement. What is owed now is that the family whose header cites `QUAL-CLAIM-001` stops violating it.

## Edge Cases

- **A repo with no `done` specs.** The line still prints, with `examined 0`. That is the case it most needs to distinguish — zero examined and zero found are the same silence today.
- **Every spec grandfathered.** `examined 0; G grandfathered` is a materially different claim from `examined 54`, and a reader acting on a clean release gate needs to see which one they got. This stopped being hypothetical: 057's record relocation would have produced exactly it had the predicate not been re-pointed at `review.md` in the same change.
- **The family exits non-zero with findings.** The coverage line still prints, above the findings, so a run that found three stale specs also says how many it looked at.
- **A shallow clone.** The digest needs no commit, so the spec is examined; only the rename exemption, which needs both trees, cannot run, and a contract changed since the review reads stale rather than exempt.
- **A runtime call that fails.** The spec is counted under `not answered` and reported as a finding, never absorbed into `examined`.
- **A digest recorded over an empty subject set.** `reviewed-digest: {}` is a spec with no scenarios and no data model; it is *taken and empty*, which reads as current, not undeterminable.
- **A digest naming a contract that no longer exists.** A deleted scenario the digest covered is a change, matching the runtime's own handling; it is stale, not unexaminable.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Should Family 19 compare `reviewed-digest` rather than diffing `reviewed-against`..HEAD?** Both, on two arms: the digest where a record carries one, the commit diff where it does not. Resolved 2026-09-07.

  The question was framed as coherence rather than correctness, and that framing was wrong. Reading `review-freshness.sh` against `analyze_subjects.rs` shows the false positive the digest eliminated is still *reachable* here: `write-review` stamps `reviewed-against: HEAD` while reviewing the working tree, so a contract reviewed before it was committed makes the diff fire on content the review had already read — the failure `the-committed-tree-horizon` records from 047. At release time the only thing preventing it is the commit-then-review-then-commit convention in `AGENTS.md` §Workflow, and §design-principles refuses to let a check depend on a human discipline. So keeping the commit diff alone was not an option, and the question is a correctness one after all.

  Going digest-only was rejected on measurement rather than principle: 1 of 54 specs carries `reviewed-digest` today, so a digest-only Family 19 would examine one spec and report the other 53 undeterminable, turning a release gate into a formality until the corpus turns over. The two-arm form keeps enforcement at 54 while making 022 immune immediately, and every subsequent review migrates one more spec off the proxy.

  The runtime's rejection of a `sha-diff fallback` was weighed and does not transfer. It was decided for `check-review-gate`, where the sha answers about a tree the horizon hides and is therefore *wrong*; here the trees are committed and the sha is merely *weaker*. The cost of adopting it is that the coverage line must report the split, which is the same line this scenario already requires — so the resolution makes that line load-bearing rather than cosmetic, and gives the arm a documented exit condition: when the proxy count reaches zero, delete it.
- **Is the proxy arm deleted?** Yes, with the family's own copy of the rule, when it came to call the runtime (owner, 2026-11-04; 066 plan, *Family 19 calls the runtime*). Its exit condition held: no `done` spec was on the arm (0 of 35), and `/{project}:audit` is maintainer-only, so no adopter corpus keeps pre-digest records it would protect. A record with no digest now reports undeterminable.
