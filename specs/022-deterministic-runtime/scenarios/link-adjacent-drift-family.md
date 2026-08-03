---
section: "Follow-on scenarios"
---

# Link-adjacent-drift-family

## Context

An artifact can carry a link to a sibling and prose describing that sibling's state, and the two can disagree. The prose calls a question `unresolved` while the linked scenario reports none; it says the work `does not exist` while the linked spec is `done`. Nothing mechanical catches it: the grounding check verifies that a claim is *cited*, not that it is *true*, so a stale claim that correctly cites its source passes.

[045 — Decision-state drift detection](../../045-decision-state-drift-detection/spec.md) owns the requirement, its acceptance criteria, and the constitution amendment behind it; this scenario carries the runtime work, per that spec's Implementation ownership split. Depends on [block-element-scanner](block-element-scanner.md) for the prose unit and on [check-artifacts-skipped-targets](check-artifacts-skipped-targets.md) for the unexamined-target record.

## Behavior

**A sixth `check-artifacts` family, `link-adjacent-drift`, advisory.** It scans `spec.md`, `plan.md`, `tasks.md`, and `scenarios/*.md` — scenarios enumerated through the shared scenario-file listing, so the scanned set matches the one every other surface counts.

**A sibling link is one that lexically resolves inside the feature directory.** Each inline link's target is resolved against the containing file's own directory, so `../spec.md` cited from a scenario is a sibling while `../022-deterministic-runtime/spec.md` cited from a spec is not. Resolution is lexical rather than canonicalized: a target may legitimately not exist, and canonicalization both fails on a missing path and makes the result depend on symlinks. Targets carrying a URL scheme and bare-fragment targets are rejected before resolution; a fragment on a sibling target is stripped and the file part used.

**The tell list is closed at six, framework-fixed, with no configuration surface**: `open question` (and `open questions`), `unresolved`, `still open`, `not yet`, `does not exist`, `left unimplemented`. A tell counts only at a byte offset outside every inline code span — without that exemption, every document *describing* this check would trip it, starting with the spec that defines it.

**Evaluation is per link, and a finding requires an actual contradiction.** A block carrying three links is scanned three times and fires only for the target whose readable state contradicts the tell. Question-state tells (`open question`, `unresolved`, `still open`) are contradicted by an open-question count of zero; implementation-state tells (`not yet`, `does not exist`, `left unimplemented`) by a spec target at `in-progress` or `done`. Every other pairing yields nothing.

`does not exist` is an implementation-state tell rather than a file-existence one. A link that resolves always points at a present file, so a presence test could only ever fire and never filter — and a test that cannot fail is not a test. The full-repo run confirmed it empirically before this scenario shipped: its one finding across 47 specs was a false positive, prose calling an *override mechanism* absent while linking to a scenario that was not.

**A scenario target is evaluated on the two signals it has** — its open-question count and its file existence. It carries no lifecycle status, so an implementation-state tell against a scenario produces no finding rather than a guess. Deriving that state from the scenario's task checkbox was rejected: the scenario-consistency family already documents that a spent task pruned per §tasks-phase never counts against its scenario, so an absent task means "pruned" as often as "unimplemented" — wrong in exactly the mature-spec case where it would matter.

**One finding per (block, link) pair**, carrying the citing file, the starting line, the link target, every tell that fired in list order, and the target's contradicting state. Not one per tell: `does not exist yet` matches two tells at once, and reporting one authorial claim twice would both mislead and inflate the promotion threshold.

**An unreadable target is recorded, never escalated.** A missing target, an unparseable one, or one carrying no state the tell's class can be evaluated against is added to `skipped` and produces no finding.

## Edge Cases

- A feature directory whose prose agrees with its link targets produces zero findings and an empty `skipped` — the ordinary result, and the one that must stay quiet for the check to be worth running.
- A tell inside a fenced code block, an HTML comment, a blockquote, or an inline code span produces nothing. The blockquote exemption is what lets a spec quote the stale claim it documents; the code-span exemption is what lets one write the tell list down.
- A link whose target exists but whose cited *section* no longer says what the citing prose claims produces no finding. Verifying it needs a fragment anchor or semantic reading, and it is a recorded non-goal rather than a gap.
- A long paragraph can pair a tell with an unrelated link. Bounded by per-link evaluation and by advisory severity; the residue is a glance, not a blocked gate.
- Prose calling something absent while linking to a scenario is **not** flagged: the tell needs a lifecycle status, a scenario has none, and the target is recorded as `no-readable-state` instead. This is the case that drove `does not exist` out of the existence class in the first place.
- `review.md` and `data-model.md` are outside the scanned set. A review record is pinned to its `reviewed-against` sha and describes the state at that commit, so its prose is correct as written and would generate systematic false positives.
- Repeat runs over an unchanged feature directory produce identical findings and skips: the tell list, the block order, and the reason set are all fixed, and nothing on the path reads wall-clock time or filesystem ordering.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
