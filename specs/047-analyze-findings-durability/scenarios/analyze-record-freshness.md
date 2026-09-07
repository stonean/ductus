---
section: "Behavior"
---

# Analyze-record-freshness

## Context

047 made the analyze **run** durable and 022 gave `check-review-gate` a record to read. Both stopped at the record's *existence*. `analyzed-against` was written and deliberately never asserted — [022's `write-analysis-and-the-second-gate`](../../022-deterministic-runtime/scenarios/write-analysis-and-the-second-gate.md) records the reasoning verbatim: *"recorded now so a future staleness check has data; asserting on it today would invent a gate to match a symmetry."*

That was right about symmetry and wrong about the hazard, because the hazard is not symmetric. The pipeline mandates `/{project}:review → /{project}:analyze → done`, which makes `/{project}:review` the one command that reliably invalidates an existing analyze record: it writes `review.md` and the spec's `review:` block, `--fix` rewrites code, and an operator resolving MUST violations rewrites more. Afterwards the recorded run describes a tree that no longer exists — and the gate checks only that `analyze.last-run` is *set*. So `review → fix → done` passes on an analysis from before the fixes.

`/{project}:implement`'s own staleness check states the shape: *"a review that predates the code it nominally covers satisfies every other check."* Substitute analyze and the sentence stays true; nothing catches it.

The second half is quieter, and it is what surfaced this. `/{project}:review` never reports the analyze state at all — its `Output` block prints five pass rows, an optional `captured` line, `blocking`, and the report path, and the blocking path's next-step names `/{project}:review` twice and `/{project}:analyze` never. The only thing carrying an operator from a passing review to the second gate is memory: the diligence dependency §design-principles rejects, and the same failure 047 closed one level up.

## Behavior

One requirement, two enforcement points — the record's freshness is **surfaced** where it goes stale and **enforced** where it matters.

**The subject set.** A spec's analyze record is stale when any `.md` under `specs/{feature}/` — `spec.md`, `plan.md`, `tasks.md`, `data-model.md`, `scenarios/*.md`, `review.md` — differs from what `analyzed-against` names. Analyze's families read all of them, so all of them are its subject. The single exclusion is a diff confined to the spec's own `analyze:` frontmatter block: that is the record's own write, and counting it would make every run stale itself. A diff that is a uniform rename under §spec-lifecycle says what it said before and is not stale, matching the review check's identical carve-out.

**`/{project}:review` reports the state.** Every run renders one `analyze` row in its stdout summary, in all three states:

```text
  analyze     ✗ never analyzed — run /{project}:analyze before done
  analyze     ✗ last run 2026-09-06 against 683a1e0 — this review supersedes it
  analyze     ✓ last run 2026-09-06 against 683a1e0 — current
```

The third state is why this is a computed line and not a fixed reminder. A `/{project}:review` that changed nothing leaves a genuinely current record, and reporting it stale would be the false alarm that teaches operators to skip the row.

**The notice reads the working tree; the gate reads committed trees.** `/{project}:review` has just written `review.md` and the `review:` block and `HEAD` has not moved, so a committed-tree diff would report `current` at the exact moment it stopped being true. This is the distinction `/{project}:implement`'s gate already draws when it names durable contracts it could not examine — the notice answers at a different moment than the gate, so it uses the evidence available at that moment.

**`check-review-gate` gains the staleness check, ordered last.** After `not-analyzed` and `analyze-findings`, for the reason review staleness is ordered after review blocking: it is the weakest claim of the three — the others say the analysis is missing or failing, this one says a passing analysis is out of date. It blocks, naming the changed paths, and directs the operator to re-run `/{project}:analyze`.

**The notice is a notice, not a gate.** `/{project}:review` has no authority over the `done` transition and does not acquire one here; its exit code is unchanged by the analyze row.

## Edge Cases

- **No `analyze:` block at all.** The not-analyzed check fires first and staleness never evaluates — first-failure-wins, as on the review side.
- **`analyzed-against` names a sha absent from the tree** (rebase, shallow clone). Reported as undeterminable, never passed silently — the behavior `stale_review_block` already carries for an unresolvable `reviewed-against`.
- **Empty-scope review.** The row still renders; a record can legitimately read `current` there.
- **Dimension-restricted review** (`--security` / `--quality` / `--simplicity`). The row still renders — a partial review supersedes the record no less than a full one does.
- **`--all`.** One row per feature, inside that feature's block.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Why is `review.md` in the subject set, when the review staleness check deliberately excludes it from its own?** Because the two checks have different subjects. A review reads *code*, so `review.md` is its output and not its input. An analysis reads *artifacts*, and `review.md` plus the `review:` block are among them — `review-state-drift` asserts on that block directly. Excluding it would exempt the exact edit that most often invalidates the record.
- **Why not simply re-run `/{project}:analyze` automatically after `/{project}:review`?** Because that couples two commands with different scopes and failure modes, and it removes the operator's chance to fix MUST violations before the analysis reads them. The gate already sequences them; what was missing was the signal, not the automation.
