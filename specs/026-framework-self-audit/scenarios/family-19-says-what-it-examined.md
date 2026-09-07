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
review-freshness: examined N spec(s) at status: done carrying a resolvable reviewed-against; G grandfathered (no review: block); U unresolvable
```

The line is a **claim about the subject**, not a finding, so it is written to stdout and does not affect the exit code. `run_check` keeps its current contract — non-zero means findings — and a clean Family 19 keeps producing no findings; what changes is that its output stops being empty.

The three counts are the ones the family already computes while walking, so this is a report of existing state rather than new work. Grandfathered and unresolvable specs are named separately because they are the two ways a spec can be *in* the corpus and *out* of the comparison, and folding them into the examined count would restate the conflation the line exists to remove.

**The aggregator is unchanged.** Teaching `run_check` to demand a coverage line from every family would be the broader fix, and it is deliberately not taken here: three families already emit one voluntarily, the fourth is this one, and a framework-wide contract is its own scenario with its own measurement. What is owed now is that the family whose header cites `QUAL-CLAIM-001` stops violating it.

## Edge Cases

- **A repo with no `done` specs.** The line still prints, with `examined 0`. That is the case it most needs to distinguish — zero examined and zero found are the same silence today.
- **Every spec grandfathered.** `examined 0; G grandfathered` is a materially different claim from `examined 54`, and a reader acting on a clean release gate needs to see which one they got.
- **The family exits non-zero with findings.** The coverage line still prints, above the findings, so a run that found three stale specs also says how many it looked at.
- **A shallow clone.** Unresolvable `reviewed-against` is already its own finding, so such a spec is counted under `unresolvable` and is not silently absorbed into `examined`.

## Open Questions

- Should Family 19 compare `reviewed-digest` rather than diffing `reviewed-against`..HEAD? Spec 022's tasks 107 and 108 moved `check-review-gate` to a content comparison, so the two enforcement points this family's header calls "two enforcement points for one rule" no longer share a mechanism. The commit diff is still sound *here* — Family 19 runs at release time, where everything is committed and the working-tree horizon that motivated the digest cannot bite — so this is a coherence question, not a correctness one. It is load-bearing for the coverage line above: a digest-reading Family 19 gains a third exclusion class (a record carrying no `reviewed-digest` is `undeterminable`, neither fresh nor stale), and today every spec in the corpus but 022 is in it, so the line would report `examined 1` rather than `examined 54` and the family would enforce almost nothing until reviews are re-run. Decide before implementing the line, since the answer changes what it counts.

## Resolved Questions

*None yet.*
