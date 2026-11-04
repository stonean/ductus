---
section: "Record freshness"
---

# Checkbox-state-is-outside-the-analysis-digest

## Context

`spec.md` is one of the analysis's subjects, so its digest covers each acceptance criterion's checkbox. Ticking a criterion after `/{project}:analyze` changes the file, and `check-review-gate` then answers `blocked-by: analyze-stale` naming `spec.md`. Measured 2026-10-25 on 0.59.11 and HEAD f24c55c6: an in-progress spec whose gate passed blocked once `mark-criterion --label ACn` unticked one criterion, and passed again once it was ticked back. Adopters learn by trial to tick before analyzing, or to analyze twice.

## Behavior

- Ticking or unticking an acceptance criterion between the analyze run and the gate does not, by itself, make the gate report the analysis stale: `spec.md` is digested with each acceptance criterion's checkbox state normalized out, alike where `write-analysis` records the digest and where the gate compares it.
- Any other change to `spec.md`'s content after the analyze run still makes the analysis stale.
- The review digest is unaffected: `spec.md` is not one of its durable contracts.

## Edge Cases

- A criterion's text edited while its box is ticked is a content change, not a checkbox change, and stales the analysis.
- A criterion added or removed is a content change.
- A tick reverted to its analyzed state reads as current.
- `[x]` and `[X]` are both a tick and normalize alike, under the checkbox grammar `read-spec` and `mark-criterion` share.
- Only an item that grammar reads as an acceptance criterion is normalized; a checkbox elsewhere in `spec.md` is content and stales the analysis.
- Assigning a criterion's `AC{n}:` label changes its text, so it stales the analysis as any text edit does.
- A record written before this change digested `spec.md` whole, so over a ticked criterion it reads stale once, and the gate names it, until the next analysis.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Which design keeps a criterion tick from staling the analysis?** `spec.md` is digested with each acceptance criterion's checkbox state normalized out, so ticking or unticking one leaves the analysis current, while any other change to `spec.md` still stales it (owner, 2026-11-03). Nothing an analysis judges reads a tick: the criterion-path family examines every criterion of a `done` spec, ticked or not (`check_artifacts.rs`, `check_criterion_path_existence`), no rule file's Verification names a criterion's tick state, and the transition reads the ticks itself (`framework/commands/implement.md` step 12). Rejected: keeping the whole-file digest and stating in `/{project}:implement`'s completion steps that criteria are ticked before the final `/{project}:analyze`, which announces the block rather than removing it and costs every first completion a second analyze and implement run. Measured 2026-11-03 at `65ec2d7f`, over the 69 specs with an `analysis.md`, comparing each recorded `analyzed-digest` with a whole-file sha256 of every subject (the rename exemption not applied): no record is current, so the change stales none here. Each of the 35 `done` records differs on `spec.md`, which the completing `set-status` edits; 32 of the 34 `in-progress` records differ on another subject too, and the other two (015, 061) on `spec.md`.
- **Does freshness also accept a record whose `spec.md` digest is the whole-file form an earlier runtime wrote?** No: every record is compared by the normalized form alone, so a record written before the change over a spec with a ticked criterion reads stale once, named by the gate, until its next analysis (owner, 2026-11-03). Accepting both forms would keep a second comparison path for records no runtime writes any more, and by the measurement above it spares no record here.
