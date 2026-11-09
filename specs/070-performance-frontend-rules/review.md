---
spec: 070-performance-frontend-rules
last-run: 2026-11-09T01:38:47Z
reviewed-against: 0509135a6db3d819546fd77f2f5c65446d4bdae0
diff-base: ccfd94be2c8df56dfeebeec12856bb14526c74ad
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 4
scope: 4
excluded: 0
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Review — 070-performance-frontend-rules

## Summary

Review of 070's first reopening over `ccfd94be..0509135a` (base-source `reopen`, the parent of `a9095a13`, the commit that took 070 in-progress), scope 4, examined 4. `framework/rules/performance-frontend.md` was read whole, all 22 rules, against the loaded rule files, the rule schema in `specs/008-security-rules/data-model.md` (§Rule entry fields, §Verification phrasing, §Severity classification) and 070's AC1–AC12; `spec.md`, `plan.md` and `tasks.md` were read whole, each concrete claim against the rule file. By script: 22 rules, 6 MUST and 16 SHOULD, no Statement mixing tiers; none of the five rules task 1 moved to SHOULD keeps a MUST or a flag, and none of the six MUST rules names them; AC1, AC2 and AC5–AC10 hold, `check-rule-ids` finding all 27 cited IDs and `discover-rule-files` selecting the file under `frontend` and not under `backend`. External claims rest on task 3's check (`4e9e5278`, which names each of the 57 documents and its URL): three read-only agents checked every claim against copies fetched 2026-11-09, and every quote behind its 11 corrections was confirmed by script. This run re-checked against the same copies the two quotes spec.md's Resolved Questions make, Osmani's "e.g. < 170KB JS (min/gzip) for mobile" and web.dev's WiFi-desktop CSS row. 0 MUST, 0 SHOULD. One observation, fixed by the rule: `plan.md`'s design record named the thresholds article by its old title, quoted `FE-VITALS-001`'s Statement as it read before task 2, and gave a posture form the header does not use. Fix rounds: round 1 (`plan.md`) exempt, then empty; budget-stop false. The loop ran in a detached worktree of `0509135a`, because another session's uncommitted files sat in the main tree; they are outside this record and were not read. The primitives ran from a copy of `runtime/target/release/ductus` as built 2026-11-08T23:50:57Z, driven over stdio. No path was excluded.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- convention: plan.md's design record named the thresholds article by the title task 3 corrected, quoted FE-VITALS-001's Statement as it read before task 2, and gave the header's posture form as "; the rest are SHOULD" where the header names its SHOULD case as api-backend.md's does — `specs/070-performance-frontend-rules/plan.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

*None.*

## Unexamined governance

*None.*
