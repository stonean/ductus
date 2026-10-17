---
spec: 007-govern-workflow
last-run: 2026-10-17T03:28:20Z
reviewed-against: 12c10248b12e488b5589c9604b0a1dde0b85b92c
diff-base: 0ea710b48ac2ec19ac874ed4495e31d3644c388a
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 11
scope: 23
skipped-passes: []
reviewed-digest:
  scenarios/ductus-self-update-precheck.md: 2248181fdd0fbe3aaa55763915d3d381cc06a12b36b68c443ba7cf369c842148
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 0
  undispositioned: 0
---

# Review — 007-govern-workflow

## Summary

Re-review after the stale-content round rewrote 007 to current state (e5dac6c3): spec.md, plan.md and the ductus-self-update-precheck scenario, whose change staled the review. Diff base 0ea710b4, the parent of the reopen commit; scope 23 entries. Rule files: all 11 under framework/rules/ loaded — quality-cross.md and configuration-cross.md read in full, the nine surface files read at rule-ID and Statement level, since the scope is markdown prose with no backend or frontend code and no surface rule's trigger fires on it. Examined 11. Read in full: 007's spec.md, plan.md and scenario; 012's spec.md, plan.md and data-model.md; 021's spec.md and plan.md. Read in part: 009's spec.md (AC18, the window's only change to it); README.md (the install and agent-list sections, lines 160-182 and 318-328, which hold the window's change); framework/bootstrap/ductus.md (Instructions through the Pre-flight Phase opening, Recording the source through Collect Project Inputs, File Fetching through Re-Run Behavior — not Source resolution, runtime detection, acquisition or wiring, nor the config schema block). Not examined, 12: framework/bootstrap/govern.md (confirmed byte-identical to ductus.md with cmp, not read); framework/bootstrap/configure/claude.md, the nine framework/commands/*.md plan entries (analyze, clarify, groom, help, implement, plan, specify, status, target) and the .claude/commands/ductus/*.md pattern entry, all unchanged since the diff base and outside the rewrite; the generator reports the mirrors in sync. Each rewritten claim was checked against ductus.md, ductus-procedure.md (What This Command Does NOT Do through Directory Creation, read out of scope as a source), git history and the tree. 0 MUST, 0 SHOULD, 0 low-confidence; no observations.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

*None.*

## Skipped passes

*None.*

## Unexamined governance

*None.*
