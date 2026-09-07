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

`review-freshness.sh` prints nothing and exits 0. The aggregator is built for that — `run_all`'s `run_check` emits a per-family header only on a non-zero exit — so silence is the designed pass shape. The problem is that silence is *also* the shape of a family that aborted before it examined anything, and `run_check` cannot tell the two apart either: it reads the exit code alone.

That is not a hypothetical failure mode in this repo. [`scripts/audit/sibling-coupling.sh`](family-19-review-freshness.md) used a GNU awk extension, so on every macOS machine the extraction aborted, the family found nothing, and it exited 0 — a release gate that was dead locally and alive in CI. `AGENTS.md` §Design Principles records that as the single most expensive failure mode this repo has produced, and this file's own header cites `QUAL-CLAIM-001` while leaving its own result indistinguishable.

Observed 2026-09-07 while closing spec 022. The self-audit had to be trusted as the release gate, Family 19 returned zero bytes, and the only way to establish that it had run was to copy the script, instrument the enumeration loop, and re-run it. It had run — 54 specs, 022 among them. Nothing in the family's output said so, and nothing would have said otherwise.

## Behavior

**`review-freshness.sh` closes every run with one coverage line on stdout, clean or not.** It names the count it enumerated and the exclusions it applied, in the shape its three siblings already use:

```text
review-freshness: examined N spec(s) at status: done — D by reviewed-digest, P by commit-diff proxy; G grandfathered (no review: block); U unresolvable
```

The line is a **claim about the subject**, not a finding, so it is written to stdout and does not affect the exit code. `run_check` keeps its current contract — non-zero means findings — and a clean Family 19 keeps producing no findings; what changes is that its output stops being empty.

Grandfathered and unresolvable specs are named separately because they are the two ways a spec can be *in* the corpus and *out* of the comparison, and folding them into the examined count would restate the conflation the line exists to remove. The digest/proxy split is named for a sharper reason, given below: the two arms do not carry the same strength of claim, and a single `examined` count would assert the stronger one over both.

### The comparison has two arms

**A record carrying `reviewed-digest` is judged by content; one without it keeps the commit diff.** Spec 022's tasks 107 and 108 moved `check-review-gate` to a per-path digest of what the review actually read, and where that digest exists this family reads the same field, so the two enforcement points agree by construction rather than by coincidence.

The proxy arm is not a grandfather clause and is not the runtime's rejected `sha-diff fallback`. `check-review-gate` treats a digest-less record as `undeterminable` and passes it, because at the completion gate the sha comparison is *actively wrong* — the working tree it would answer about is exactly the tree the horizon hides. That reasoning does not transfer here: Family 19 runs at release time against committed trees, where the sha diff is a weaker but sound proxy, and the alternative to a weak answer is no answer at all for the 53 records that predate the field. A release gate that examines 1 spec of 54 is not more honest than one that examines 54 and says which 53 it examined weakly.

**The proxy arm carries a known false positive, and naming it is why the split is reported.** A review recorded against a dirty tree stamps `reviewed-against: HEAD` — a commit that does not yet contain the contracts the review read — so committing them afterwards makes the diff fire on content the review had in hand. That is the failure `the-committed-tree-horizon` records from spec 047, and on the proxy arm it is guarded only by the commit-then-review-then-commit convention in `AGENTS.md` §Workflow, which is a human discipline of exactly the kind §design-principles refuses to depend on. The digest arm is immune to it. So the split is not decoration: it says how many of the run's verdicts rest on a discipline rather than on a record.

**The population migrates on its own.** Every `/{project}:review` run writes a digest, so each re-review moves one spec from the proxy arm to the digest arm and the false positive drains with it. Measured 2026-09-07: 1 of 54 specs carries `reviewed-digest` (022) and 2 carry `analyzed-digest` (022, 047). When the proxy count reaches zero the arm can be deleted, and the line will say when that has happened.

**The aggregator is unchanged.** Teaching `run_check` to demand a coverage line from every family would be the broader fix, and it is deliberately not taken here: three families already emit one voluntarily, the fourth is this one, and a framework-wide contract is its own scenario with its own measurement. What is owed now is that the family whose header cites `QUAL-CLAIM-001` stops violating it.

**The aggregator is unchanged.** Teaching `run_check` to demand a coverage line from every family would be the broader fix, and it is deliberately not taken here: three families already emit one voluntarily, the fourth is this one, and a framework-wide contract is its own scenario with its own measurement. What is owed now is that the family whose header cites `QUAL-CLAIM-001` stops violating it.

## Edge Cases

- **A repo with no `done` specs.** The line still prints, with `examined 0`. That is the case it most needs to distinguish — zero examined and zero found are the same silence today.
- **Every spec grandfathered.** `examined 0; G grandfathered` is a materially different claim from `examined 54`, and a reader acting on a clean release gate needs to see which one they got.
- **The family exits non-zero with findings.** The coverage line still prints, above the findings, so a run that found three stale specs also says how many it looked at.
- **A shallow clone.** Unresolvable `reviewed-against` is already its own finding, so such a spec is counted under `unresolvable` and is not silently absorbed into `examined`.
- **A record with a digest and an unresolvable `reviewed-against`.** The digest arm needs no commit, so the spec is examined rather than counted `unresolvable` — the existing finding fires only when the proxy arm is the one that cannot answer.
- **A digest recorded over an empty subject set.** `reviewed-digest: {}` is a spec with no scenarios and no data model; it is *taken and empty*, which reads as current, and it belongs on the digest arm rather than falling through to the proxy.
- **A digest naming a contract that no longer exists.** A deleted scenario the digest covered is a change, matching the runtime's own handling; it is stale, not unexaminable.
- **Every record carries a digest.** The proxy count reaches zero, the line says so, and the arm has become dead code the next contributor can delete on that evidence.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Should Family 19 compare `reviewed-digest` rather than diffing `reviewed-against`..HEAD?** Both, on two arms: the digest where a record carries one, the commit diff where it does not. Resolved 2026-09-07.

  The question was framed as coherence rather than correctness, and that framing was wrong. Reading `review-freshness.sh` against `analyze_subjects.rs` shows the false positive the digest eliminated is still *reachable* here: `write-review` stamps `reviewed-against: HEAD` while reviewing the working tree, so a contract reviewed before it was committed makes the diff fire on content the review had already read — the failure `the-committed-tree-horizon` records from 047. At release time the only thing preventing it is the commit-then-review-then-commit convention in `AGENTS.md` §Workflow, and §design-principles refuses to let a check depend on a human discipline. So keeping the commit diff alone was not an option, and the question is a correctness one after all.

  Going digest-only was rejected on measurement rather than principle: 1 of 54 specs carries `reviewed-digest` today, so a digest-only Family 19 would examine one spec and report the other 53 undeterminable, turning a release gate into a formality until the corpus turns over. The two-arm form keeps enforcement at 54 while making 022 immune immediately, and every subsequent review migrates one more spec off the proxy.

  The runtime's rejection of a `sha-diff fallback` was weighed and does not transfer. It was decided for `check-review-gate`, where the sha answers about a tree the horizon hides and is therefore *wrong*; here the trees are committed and the sha is merely *weaker*. The cost of adopting it is that the coverage line must report the split, which is the same line this scenario already requires — so the resolution makes that line load-bearing rather than cosmetic, and gives the arm a documented exit condition: when the proxy count reaches zero, delete it.
