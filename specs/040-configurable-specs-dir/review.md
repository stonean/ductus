---
spec: 040-configurable-specs-dir
scenario: command-prose-resolves-spec-root
reviewed-at: 2026-08-19T13:39:02Z
reviewed-against: d35bbc2d0a91b367be87cae68378cae8065bec67
diff-base: 130f4cf5dc9e02ee3045446527c3e5b115411d65
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 1
skipped-passes: []
---

# Review — 040-configurable-specs-dir

## Summary

Reviewed the command-prose-resolves-spec-root scenario: the Spec-root resolution note propagated to amend.md, groom.md, implement.md, log.md and review.md, the two argument-literal rewrites, and the regenerated command copies. No MUST or SHOULD violations; the spec is not blocked. The change is prose-only — no runtime code was touched, so the security and efficiency passes have no subject and the reuse pass confirmed the correct outcome: the fix reuses specify.md's existing note verbatim rather than inventing a second form. The quality pass found and the change fixed one defect in the fix itself: the note read "every `specs/…` path below", making its coverage depend on where it sat, and two host-acted sites (amend.md's Scope Boundaries declaration, implement.md step 13) sat above their note. Reworded to "in this command" across all six files including specify.md, so one form exists and coverage no longer depends on position. Triage of those two confirmed neither was a live defect — implement.md step 13 describes a filter diff-cross-spec owns, and amend.md:35 is a scope declaration whose acted-on counterpart at :60 is covered — but the positional fragility was real and is gone. Two parseability/lint breakages introduced during the sweep were caught by the gates and fixed: a step naming two backticked primitives (log.md step 3, where append-inbox is a cross-reference not a dispatch) and MD028 blank-line-inside-blockquote where the new note landed adjacent to the agent-runtimes blockquote, resolved by folding it in as a second paragraph.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

- [ ] bug: command prose hardcodes `specs/` instead of resolving `[paths] specs-root` — resolved by this scenario; the 12 host-acted sites now carry the Spec-root resolution note and the two argument-literals (log.md step 3, groom.md step 1) were rewritten.

## Observations

*None.*

## Skipped passes

*None.*
