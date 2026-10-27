---
section: "Behavior"
---

# Analyze-record-freshness

## Context

047 made the analyze **run** durable and 022 gave `check-review-gate` a record to read. Both stopped at the record's *existence*. `analyzed-against` was written and deliberately never asserted — [066's `write-analysis-and-the-second-gate`](../../066-audit-records-and-the-completion-gate/scenarios/write-analysis-and-the-second-gate.md) records the reasoning verbatim: *"recorded now so a future staleness check has data; asserting on it today would invent a gate to match a symmetry."*

That was right about symmetry and wrong about the hazard, because the hazard is not symmetric. The pipeline mandates `/{project}:review → /{project}:analyze → done`, which makes `/{project}:review` the one command that reliably invalidates an existing analyze record: it writes `review.md` — which is itself an analyze subject — `--fix` rewrites code, and an operator resolving MUST violations rewrites more. (When this was written it wrote the spec's `review:` block too; `057-analyze-artifact-and-record-relocation` moved that record into `review.md`, which narrows the mechanism to one file and leaves the hazard exactly as stated.) Afterwards the recorded run describes a tree that no longer exists — and the gate checks only that `analyze.last-run` is *set*. So `review → fix → done` passes on an analysis from before the fixes.

`/{project}:implement`'s own staleness check states the shape: *"a review that predates the code it nominally covers satisfies every other check."* Substitute analyze and the sentence stays true; nothing catches it.

The second half is quieter, and it is what surfaced this. `/{project}:review` never reports the analyze state at all — its `Output` block prints five pass rows, an optional `captured` line, `blocking`, and the report path, and the blocking path's next-step names `/{project}:review` twice and `/{project}:analyze` never. The only thing carrying an operator from a passing review to the second gate is memory: the diligence dependency §design-principles rejects, and the same failure 047 closed one level up.

## Behavior

One requirement, two enforcement points — the record's freshness is **surfaced** where it goes stale and **enforced** where it matters.

**The subject set.** A spec's analyze record covers every `.md` under `specs/{feature}/` — `spec.md`, `plan.md`, `tasks.md`, `data-model.md`, `scenarios/*.md` and `review.md`. Analyze's families read all of them, so all of them are its subject. Two exclusions: the run's **own artifact**, because it is written *after* the subjects are read and counting it would make every run stale itself; and a change that is a uniform §spec-lifecycle rename, which says what it said before, matching the review check's identical carve-out. A `done` spec's comparison also ignores an in-place edit to `plan.md` or `tasks.md`, which §spec-lifecycle gives no back-edge when it adds no unchecked task, and still counts a deleted one; the digest records both (`041-task-pruning`'s `a-plan-edit-does-not-reopen-a-done-spec`, 2026-10-26).

The first exclusion moved with the record, and got wider on the way. When this scenario was written the record was a `analyze:` block in `spec.md` frontmatter, so the exclusion was block surgery on a subject that stayed in the set. `057-analyze-artifact-and-record-relocation` made the record `analysis.md`'s frontmatter, so **`spec.md` is digested whole** — the block-surgery path for it is gone rather than re-pointed — and **`analysis.md` is not a subject at all**.

Excluding only its frontmatter was tried first and was not enough. `/{project}:analyze` writes that file's *body* in the same call that writes the record, so a run whose Summary or counts differed from the previous one digested a body it was about to replace and reported itself stale; converging took a second identical run. The file is this command's **output**, exactly as `review.md` is `/{project}:review`'s and is excluded from the review digest for the identical stated reason, and nothing reads its body — the one family that reads the file reads the record. The reason is unchanged throughout and is what carried across each widening: a record written after its subjects are read can never digest itself.

**Staleness is a content comparison, not a commit comparison.** The record carries `analyzed-digest`: a per-path digest of the subject set **as the run actually read it**. Staleness is a digest mismatch. `analyzed-against` stays in the record as provenance — where `HEAD` was — and is no longer the basis of the check.

This is not a refinement of the sha-diff design; it replaces it, because that design produced false blocks. `/{project}:analyze` reads artifacts **from disk** — the working tree — while `analyzed-against` records a *commit*. Those are the same subject only when the tree is clean, and at the moment analyze runs it usually is not: `/{project}:review` has just written `review.md` (and, before `057-analyze-artifact-and-record-relocation` gave that record one home, the spec's `review:` block with it), and `mark-task` and `mark-criterion` rewrote `tasks.md` and `spec.md` minutes earlier. So the record claimed a commit it had not examined, the gate diffed that commit against `HEAD`, and the paths the analysis *had* read came back as changes — blocking a record that was genuinely current. A gate with false positives is the one people route around, which is the failure this file's own history records twice.

A digest of what was read cannot make that mistake. It also removes the reason the two surfaces ever needed different reference points: a commit sha cannot describe a working tree, so the gate had to read commits and the notice had to read the tree, and they could return different answers from the same implementation. One digest answers the same way wherever it is asked.

**`/{project}:review` reports the state.** Every run renders one `analyze` row in its stdout summary, in all four states:

```text
  analyze     ✗ never analyzed — run /{project}:analyze before done
  analyze     ✗ last run 2026-09-06 against 683a1e0 — this review supersedes it
  analyze     ✓ last run 2026-09-06 against 683a1e0 — current
  analyze     ? freshness undeterminable — the record carries no analyzed-digest
```

The fourth is the digest-less record this scenario's own Edge Cases describe: it is neither current nor stale, and rendering it as either would be the conflation the record exists to prevent. The `current` state is why this is a computed line and not a fixed reminder. A `/{project}:review` that changed nothing leaves a genuinely current record, and reporting it stale would be the false alarm that teaches operators to skip the row.

**One reference point, so the row and the gate cannot disagree.** Both compute the current subject-set digest and compare it to the recorded one. There is no committed-tree horizon left for either to be blind to, and therefore nothing for a passing verdict to be silent about — which retires the `QUAL-CLAIM-001` finding this scenario's first implementation earned rather than dispositioning it.

The **rename exemption still needs the commits**, and that is the one place `analyzed-against` is read. When digests differ, the mismatching paths are candidates; the existing mechanical-sweep index filters those whose change was a uniform rename, using `analyzed-against` and `HEAD` as it does for the review check. When those trees are unavailable — a shallow clone, a rebase — the exemption cannot run and the candidates are reported rather than silently dropped. An uncommitted rename sweep therefore over-reports as stale; that is the safe direction, and re-running a read-only analyze costs nothing.

**`check-review-gate` gains the staleness check, ordered last.** After `not-analyzed` and `analyze-findings`, for the reason review staleness is ordered after review blocking: it is the weakest claim of the three — the others say the analysis is missing or failing, this one says a passing analysis is out of date. It blocks, naming the changed paths, and directs the operator to re-run `/{project}:analyze`.

**The notice is a notice, not a gate.** `/{project}:review` has no authority over the `done` transition and does not acquire one here; its exit code is unchanged by the analyze row.

## Edge Cases

- **No record at all** — no `analysis.md` (an absent `analyze:` block in `spec.md`, before `057-analyze-artifact-and-record-relocation`). The not-analyzed check fires first and staleness never evaluates — first-failure-wins, as on the review side. 057 added a third state beside absent and present: an `analysis.md` that exists but carries no parseable record is **undeterminable**, and the gate reports it as that rather than as never-analyzed.
- **A record with no `analyzed-digest`** — every record written before this scenario. Freshness is **undeterminable**, not current and not stale: nothing on disk says what those runs examined, and inventing either answer is the conflation the record exists to prevent. Undeterminable does not block, so the gate stops enforcing freshness for a spec until its next `/{project}:analyze` writes a digest. That is self-healing rather than a permanent hole, and it is deliberately not a grandfather clause: the record is not *exempt*, it is *unreadable*, and the gate says so.
- **`analyzed-against` does not resolve** (rebase, shallow clone). Staleness still answers — it is a content comparison and does not need the commit. Only the rename exemption is lost, and its candidates are reported rather than dropped.
- **A subject that cannot be read** when the digest is computed. The path is recorded as unreadable rather than digested as empty; an unreadable subject is not a matching one.
- **Empty-scope review.** The row still renders; a record can legitimately read `current` there.
- **Dimension-restricted review** (`--security` / `--quality` / `--simplicity`). The row still renders — a partial review supersedes the record no less than a full one does.
- **`--all`.** One row per feature, inside that feature's block.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Why is `review.md` in the subject set, when the review staleness check deliberately excludes it from its own?** Because the two checks have different subjects. A review reads *code*, so `review.md` is its output and not its input. An analysis reads *artifacts*, and `review.md` is among them — `review-state-drift` asserts on the record it carries directly. (It asserted on the spec's `review:` block until `057-analyze-artifact-and-record-relocation` gave that record one home; the family reads the same fields from `review.md` now, so the reason this file is a subject is unchanged and there is one fewer file to name.) Excluding it would exempt the exact edit that most often invalidates the record.
- **Why not simply re-run `/{project}:analyze` automatically after `/{project}:review`?** Because that couples two commands with different scopes and failure modes, and it removes the operator's chance to fix MUST violations before the analysis reads them. The gate already sequences them; what was missing was the signal, not the automation.
- **Why did the first implementation compare commits at all, and what does its replacement cost?** Because `analyzed-against` already existed and reading it looked like reuse. It was not: the field is provenance, and using it as the staleness basis silently redefined the record's subject from *what analyze read* to *what was committed at the time*. The tell was there and went unread — the design needed two reference points and a documented asymmetry between them, and a rule that needs an asymmetry to hold is usually describing a mismatch rather than a trade-off. The digest costs one field and a hash of files already being read; it retires `Compare`'s two modes, the working-tree/committed split, and the `QUAL-CLAIM-001` blind spot together.
- **Why is a digest-less record undeterminable rather than falling back to the sha diff?** Because the sha diff is the behavior being removed for producing false blocks, and a fallback would keep it live on exactly the population that cannot be checked any other way — every record predating this change. Undeterminable is the honest answer and it clears itself on the next run.
