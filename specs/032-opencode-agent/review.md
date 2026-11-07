---
spec: 032-opencode-agent
last-run: 2026-11-07T19:43:46Z
reviewed-against: 7a6b45d50a941aa448282dd8ffc52410c21f851c
diff-base: 76f29984428aab6c2d0b43ffb7f61e9e2bcd9d19
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 5
scope: 5
excluded: 0
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: ebb4c01f36087843f6d082875ed3eca96e36b022aa6a3bea1d8030cc2ae5cf04
blocking: false
dispositions:
  fixed: 1
  deferred: 0
  waived: 0
  undispositioned: 0
---

# Review — 032-opencode-agent

## Summary

Five passes over 5 of 5 in-scope files, the second review of this reopening. The window is the default, base `76f29984` with `base-source: last-review`, the commit holding the first review's record; it was not overridden. It holds `262331c1`, the analysis that fixed the plan's Trade-offs claim about `exec` command resolution, `f8d05589`, which recorded task 13, and `7a6b45d5`, task 13's fix.

The first review read `data-model.md` in full but checked only the bullet spec 079 had changed, so it missed that AC3 and Region 1 still quoted the `mcp.ductus` command as `["ductus", "mcp"]`; the completion gate's criterion check found it. `a4a33584` (spec 048) made the command the `.ductus/bin/ductus` pointer, which `framework/bootstrap/ductus.md` §MCP wiring writes. This review read each file's window diff in full and `analysis.md` whole: the data model's Region 1 now quotes the pointer command, as the bootstrap does; AC3 is annotated as superseded in part by 048 and stays ticked for what 032 delivered; plan Decision 3 names the pointer, Decision 4 states only the shape and needs none; and the Trade-offs bullet's citation, `runtime/src/host.rs` resolving `{dir}/command/{project}/<name>.md` for the `opencode` layout since `7694aadf`, holds. The Verified OpenCode Layout table keeps the probe's command, as the record of what was verified.

All 12 selected rule files were loaded, every rule's Statement read, 304 in all; no rule's surface is in scope. 0 MUST, 0 SHOULD, 0 low-confidence findings. One observation, a chore fixed by the rule: task 13's box claimed a Decision 4 correction that was never needed. The fix-round base `b94bdbc5` was taken before that fix; its one round changed only `tasks.md`, an analyze subject, so it is exempt and was not re-read, and the next round was empty.

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

- other — task 13's box says plan Decisions 3 and 4 were corrected, but Decision 4 states only the generic shape `{type, command:[…], enabled}`, which needed no correction — `specs/032-opencode-agent/tasks.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

*None.*

## Unexamined governance

*None.*
