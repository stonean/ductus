---
section: "Follow-on scenarios"
---

# The-committed-tree-horizon

## Context

Two primitives answer a question about the **working tree** by comparing **committed trees**, and both were built that way because a sha was already at hand. [047's `analyze-record-freshness`](../../047-analyze-findings-durability/scenarios/analyze-record-freshness.md) removed the third instance and named the shape; this scenario covers the two that remain.

The pattern: a command reads files from disk, records the sha `HEAD` happened to be at, and a later check diffs that sha against `HEAD`. The recorded sha is **provenance** — where the repo was — not a description of what was examined. Those coincide only when the tree is clean, and at the moment these commands run it usually is not.

**`stale_review_block` / `reviewed-against`.** `/{project}:review`'s scope is the union of the plan's Affected Files and the files modified since the diff base, so it reviews the working tree. It records `reviewed-against: HEAD`. `check-review-gate`'s staleness check then diffs that sha against `HEAD` over `scenarios/*.md` and `data-model.md`, so a scenario written during the session — reviewed, then committed — comes back as a durable contract that changed since the review. It did not change since the review; it changed since the *commit* the review was labelled with. Observed on 2026-09-07 while completing 047: its own review read the rewritten `analyze-record-freshness` scenario, and committing that scenario produced `blocked: review is stale` naming the file the review had just read.

The review half already noticed the symptom and chose to report rather than block — `unexaminable_contracts_guidance` exists precisely to say "a durable contract is uncommitted, so staleness could not be determined against it". That is an acknowledgement of the horizon, not a removal of it: the notice fires when the contract is *still* uncommitted, and goes quiet exactly when the false positive becomes reachable.

**`compute-review-scope` / `captured-issues`.** It derives the review window's inbox additions with a `diff-base..HEAD` diff over `{specs-root}/inbox.md`. Issues captured *during* the session are uncommitted, so the report whose whole purpose is to surface mid-task captures at the gate reports none of them. Observed in the same session: three items sat in `inbox.md` while `captured-issues` came back empty.

Why it survived this long: it bites less often than the analyze half did. Only `scenarios/*.md` and `data-model.md` are review contracts, and those churn far less than `tasks.md` and `spec.md`, which `mark-task` and `mark-criterion` rewrite immediately before the gate.

## Behavior

The `captured-issues` half of this scenario was removed by 058: `compute-review-scope` computes no inbox window.

**The check compares what was examined, not where `HEAD` was.** The record carries a digest of the content the command actually read, and the check is a digest comparison. `reviewed-against` and `diff-base` stay in their records as provenance.

- **`write-review`** records a digest of the review's **durable contracts** — the `scenarios/*.md` and `data-model.md` files the staleness check compares — taken from disk as the review read them. The subject set is unchanged; only the reference point moves. `review.md` stays outside it because `write-review` writes the record there, so counting it would stale every review the instant it was recorded; `spec.md`, which `write-review` no longer touches (spec 057), stays outside because adding it would stale every review on any spec-body edit.
- **`check-review-gate`** compares that digest against the same files now, through `analyze_subjects::freshness_of`. `reviewed-against` is read for one thing, the mechanical-sweep rename exemption, which genuinely needs two trees — and its candidates are reported rather than dropped when those trees are unavailable.

**`unexaminable_contracts_guidance` is removed, not retained alongside.** Its entire subject is the horizon this change eliminates; keeping it would leave a notice that can only ever fire on a state the check now handles, which is worse than no notice because a reader would take its silence as meaning something. The `QUAL-CLAIM-001` obligation it discharged moves to the digest comparison itself, whose undeterminable arm reports a record it cannot judge.

**A record with no digest is undeterminable** — not current, not stale, and not a sha-diff fallback. It does not block, so freshness stops being *enforced* for a spec until its next review writes a digest, and it clears on that run. 047's equivalent decision is the precedent: the record is not exempt, it is unreadable, and the verdict says which.

## Edge Cases

- **A durable contract deleted after the review.** A subject the digest covered that no longer exists is a change, matching 047's handling of a deleted analyze subject.
- **A contract that cannot be read** when the digest is taken, or when it is compared. Recorded as unreadable rather than digested as empty; an unreadable contract is not a matching one.
- **`reviewed-against` unresolvable** (rebase, shallow clone). Staleness still answers — it needs no commit. Only the rename exemption is lost, and its candidates are reported.
- **An empty review scope.** `write-review` still records a digest; a spec with no durable contracts records an empty one, which is distinct from an absent one and reads as current rather than undeterminable.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Why not one primitive for both records, since they now do the same thing?** Because the subject sets differ and must: a review's contracts are `scenarios/*.md` and `data-model.md`, an analysis's subjects are every `.md` under the feature. What is shared is the *mechanism*, and that is worth sharing — `analyze_subjects::subject_digest` already takes a directory and a membership predicate, so the review's digest is the same function with a narrower predicate. The claims stay separate; the arithmetic does not.
- **Why remove `unexaminable_contracts_guidance` rather than leave it as belt-and-braces?** Because a check that cannot fire is indistinguishable from one that passed, which is the rule it was written to serve. Its silence currently means "every contract is committed"; after this change it would mean nothing at all, and a reader cannot tell those apart.
