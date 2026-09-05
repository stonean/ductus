---
section: "Behavior"
---

# Analyze-run-durability

## Context

This spec made analyze's **findings** outlive the session that produced them. It did not make the **run** outlive it, and nothing else did either.

The pipeline is `/{project}:implement → /{project}:review → /{project}:analyze → done`. Both gates were required; only one was recorded:

| | recorded by | read by | held fresh by |
| --- | --- | --- | --- |
| `/{project}:review` | `write-review` → `review:` block | `check-review-gate` | Family 19, Family 31, `review-state-drift` |
| `/{project}:analyze` | *nothing* | *nothing* | *nothing* |

A spec that had passed both gates and a spec that had passed only the review were **byte-identical on disk**. There was no field to compare, so nothing could enforce the second gate, so the only thing holding it was whoever remembered to run it. That is a diligence dependency, which [§design-principles](../../../framework/constitution.md#design-principles) rejects outright — and it is one the framework rejected in every other pipeline gate while leaving this one open.

**It was reached.** On 2026-09-05, during the session that produced the 0.43.0 release, two specs (`022` and `026`) were advanced to `done` on the review gate alone. `/{project}:analyze` was not run against either. The release was cut from that state and published to crates.io, where a version can never be reused. The analysis was run afterwards, when a human asked whether it had been — it came back clean, so the artifact was sound, but that is luck rather than a property of the system. The gap was found by being asked, which is the definition of the failure mode.

The shape is the one this project keeps meeting: a check that did not run is indistinguishable from one that passed. `QUAL-CLAIM-001` names it for a code path's *output*; here the missing output was the whole record.

## Behavior

`/{project}:analyze` records its own run in an `analyze:` frontmatter block, and `/{project}:implement`'s pre-done gate reads it.

**The record.** Written on **every** run — clean, dirty, or empty-scope — because the record's whole purpose is that its *absence* means something. A run that declined to write one when it found nothing would be indistinguishable from a run that never happened, which is the defect restated one level in.

```yaml
analyze:
  last-run: 2026-09-05T19:00:00Z
  analyzed-against: <HEAD sha>
  hard-fail: 0
  blocking-findings: 0
  advisory: 4
  unexamined: 2
  blocking: false
```

It is **not** a copy of `review:`, and the two differences are the design:

- **`advisory` is recorded and never gates.** An outstanding SHOULD blocks `done` at the review gate, because §implement-phase says advisory is not ignorable there. Analyze's advisory tier is a different contract: its members are checks introduced advisory *with their own published promotion criteria* — grounding, Applicable Rules citations, link-adjacent decision drift, criterion-path existence. Gating on them here would promote every one of them at once, past the criteria each declares, which is the opposite of what those criteria are for.
- **`unexamined` has no counterpart in `review:` at all**, and it is what keeps the record honest. A clean analyze is two states, not one, and this command's own **Unexamined targets** section already says so: *"clean with nothing skipped is verified-clean, clean with something skipped is partially examined."* A record carrying only finding counts would collapse exactly that distinction into the reassuring reading — inside the artifact a later gate trusts, which is the worst place in the system for it.

**The gate.** `check-review-gate` gains two block reasons, ordered after every `review:` check: `not-analyzed` (block absent or `last-run` null) and `analyze-findings` (`blocking: true`, i.e. `hard-fail` or `blocking-findings` above zero). The order follows the pipeline — a spec whose review is missing or failing has not reached the point where analysis is the next thing owed, and naming the later gate for an earlier defect sends a contributor to the wrong command.

**The drift family.** `check-artifacts` gains `analyze-state-drift`, the counterpart to `review-state-drift`: a `done` spec with `analyze.last-run` unset or `analyze.blocking: true`.

## The grandfather rule, and why it is not a hiding place

Every `done` spec written before the record exists without one — 54 of them at introduction. The drift family exempts them.

[046](../../046-scenario-open-question-visibility/spec.md) refused precisely this shape of exemption, on the grounds that a sanctioned hiding place is worse than the gap it papers over, and the criterion-label check backfilled the corpus rather than grandfathering it. So the precedent runs *against* exempting, and the exemption needs its difference stated rather than assumed.

The difference is what a backfill would have to **assert**. A criterion label is derivable from the artifact: that backfill computed `max(highest label, next-criterion)` and wrote a value that was already true. An analyze record asserts *that a run happened* — which nothing on disk substantiates, and which no derivation can recover. Backfilling it would mean writing an unverified claim into the field a later gate trusts, which is the exact failure this record was added to prevent, committed by the mechanism itself.

So the exemption stands, and it is made **bounded instead of silent**:

- `/{project}:audit` **Family 37** reports the exempt population against a committed high-water mark (`scripts/audit/analyze-record-baseline.txt`). At or below it the family is clean and prints the count; above it, every member is a finding.
- The set **cannot legitimately grow**. The gate has no grandfather clause, so nothing can reach `done` without a record. Growth means the gate was bypassed — which is the original defect recurring, and worth failing on.
- Draining is a ratchet: re-analyzing a spec removes it from the set, and the maintainer lowers the baseline. A backlog *below* the baseline is named on stderr, because a bound nobody tightens is decoration.

The exemption is therefore counted, capped, monotonic, and visible on every audit run — which is what 046 actually objected to the absence of.

## Edge Cases

- **Empty scope.** The record is still written. A spec with no implementation files yet has been analyzed and found nothing to analyze, which is a different claim from never having been looked at.
- **Malformed frontmatter.** No record. The analysis would have hard-failed on that spec, and writing a clean record into it inverts the whole mechanism; the primitive returns a YAML error and touches nothing.
- **A spec below `done`.** The drift family is silent — the block populates lazily on first run, exactly as `review:` does. The *gate* is not silent, because it only runs at the completion attempt.
- **Re-analysis.** The block is replaced wholesale, and every sibling frontmatter key survives. The primitive reports `replaced` so a caller can tell a re-analysis from a spec leaving the grandfathered set.
- **A newline in a host-supplied scalar.** Flattened to a space rather than rejected: both fields are machine-generated (a timestamp and a sha), so there is no user intent to preserve — only a frontmatter injection to defuse.

## Resolved Questions

- **Why not make analyze write `analysis.md`, mirroring `review.md`?** Because the findings already have a durable home this spec gave them — the inbox — and a second one would be two records of the same thing with no mechanism keeping them consistent, which is the drift Family 31 exists to catch for the review pair. The frontmatter block records *that the run happened and what tier its findings fell in*; the inbox records *what they were*. One fact per home.
- **Does this break the read-only contract?** It restates it. Analyze never mutates an artifact it audits, and `--fix` remains the only path that does. The new write is about the *run*, not the *subject* — the same thing `write-review` does, and the reason that gate was enforceable while this one was not. The scope boundary now names its three writes explicitly rather than claiming a purity the inbox capture had already ended.
- **Should the gate also check analyze freshness, as it does for review?** Not yet, and the reason is that the review analogue took a concrete failure (`gvrn-v0.26.2` shipping three commits of unreviewed change) to justify its complexity. A staleness check needs a durable-contract set to diff, and analyze's subject is the whole feature directory including `tasks.md`, which is rewritten on every task — so the naive version would fire on nearly every run and be learned-ignored. Recording `analyzed-against` now means the data is there when a real instance appears; asserting on it before then would be inventing a gate to match a symmetry rather than a need.
