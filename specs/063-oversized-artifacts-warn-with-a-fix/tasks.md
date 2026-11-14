---
next-task: 18
---

# 063 — Oversized artifacts warn with a fix Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 17. Bound check-artifact-size's result (BE-PAYLOAD-001)

- [ ] `check-artifact-size`'s `oversized`, `skipped` and `notices` go through `bounded_lists::place`: each list's `-count` inline in both forms, and past `[artifacts] read-size-bytes` the lists in the one file `lists-file` names. The owner deferred 063's analysis finding here on 2026-11-13 (`analysis.md`'s stored decision, `gating: true`): the three lists grow with a spec directory's scenario count while about twenty sibling primitives bound theirs, and 080's research classified this primitive as not growing with the corpus
- [ ] `CHECK_ARTIFACT_SIZE`, `data-model.md`'s `CheckArtifactSizeResult` and `plan.md` commit to the bound, and every caller that reads the three lists (`framework/commands/analyze.md` step 17, `clarify.md`'s and `plan.md`'s size steps, the exec walker's analyze tally in `runtime/src/interpreter/analyze_tally.rs`) reads them from `lists-file` when `form` is `file`
- [ ] A test proves the inline form under the read size and the file form past it, each failing under its mutant, and `framework-checks.yml` step (u) passes

- **Done when**: `check-artifact-size` returns its three lists inline, or in a lists-file past the read size, proven by tests its mutants turn red; the whole `cargo test` passes; and 063's next review reads the change and its next analysis records BE-PAYLOAD-001 addressed.
