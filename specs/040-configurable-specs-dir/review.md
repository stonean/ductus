---
spec: 040-configurable-specs-dir
scenario: command-prose-resolves-spec-root
reviewed-at: 2026-08-19T13:51:00Z
reviewed-against: f4c3bbd3058b7babf0c757c815341802cfe21c8e
diff-base: 130f4cf5dc9e02ee3045446527c3e5b115411d65
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 040-configurable-specs-dir

## Summary

Re-run against f4c3bbd, the commit that contains the work, for the reason recorded in 022's review this cycle: the prior run reviewed the change while uncommitted, so `reviewed-against` named a commit predating it and Family 19 correctly reported the review stale once the command-prose-resolves-spec-root scenario landed. No finding changed; the reviewed tree is byte-identical to the one the prior run examined. No MUST or SHOULD violations. The change is prose-only — no runtime code was touched, so the security and efficiency passes have no subject, and the reuse pass confirmed the correct outcome: the fix reuses specify.md's existing Spec-root resolution note verbatim rather than inventing a second form. The quality pass found and the change fixed one defect in the fix itself: the note read "every `specs/…` path below", making coverage depend on where it sat, with two host-acted sites above their note; reworded to "in this command" across all six files including specify.md so one form exists and position stops mattering. Triage confirmed neither of those two was a live defect — implement.md step 13 describes a filter diff-cross-spec owns, and amend.md:35 is a scope declaration whose acted-on counterpart at :60 is covered. Two breakages introduced during the sweep were caught by the gates and fixed: a step naming two backticked primitives, and MD028 where the new note landed adjacent to the agent-runtimes blockquote.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

*None.*

## Skipped passes

*None.*
