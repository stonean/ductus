---
section: "Record freshness"
---

# Checkbox-state-is-outside-the-analysis-digest

## Context

`spec.md` is one of the analysis's subjects, so its digest covers each acceptance criterion's checkbox. Ticking a criterion after `/{project}:analyze` changes the file, and `check-review-gate` then answers `blocked-by: analyze-stale` naming `spec.md`. Measured 2026-10-25 on 0.59.11 and HEAD f24c55c6: an in-progress spec whose gate passed blocked once `mark-criterion --label ACn` unticked one criterion, and passed again once it was ticked back. Adopters learn by trial to tick before analyzing, or to analyze twice.

## Behavior

- Ticking or unticking an acceptance criterion between the analyze run and the gate does not, by itself, make the gate report the analysis stale, or the order the completion steps must follow is stated where `/{project}:implement`'s completion steps run, so no operator meets the block unannounced.
- Any other change to `spec.md`'s content after the analyze run still makes the analysis stale.
- The review digest is unaffected: `spec.md` is not one of its durable contracts.

## Edge Cases

- A criterion's text edited while its box is ticked is a content change, not a checkbox change, and stales the analysis.
- A criterion added or removed is a content change.
- A tick reverted to its analyzed state reads as current whichever design is chosen.

## Open Questions

- Which design keeps a criterion tick from staling the analysis: digest `spec.md` with each criterion's checkbox state normalized out (the gate already reads the ticks itself), or keep the digest whole and state in `/{project}:implement`'s completion steps that criteria are ticked before the final `/{project}:analyze`?

## Resolved Questions

*None yet.*
