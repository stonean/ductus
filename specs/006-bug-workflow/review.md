---
spec: 006-bug-workflow
last-run: 2026-10-30T21:38:46Z
reviewed-against: bca9a2fd3d67437745e252b930bda3615f22edb5
diff-base: 194e43f79514a2cf90d879c44661e24c7b1e0874
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 17
scope: 41
skipped-passes: []
reviewed-digest: {}
blocking: false
dispositions:
  fixed: 1
  routed: 1
  discarded: 0
  undispositioned: 0
decisions:
  - key: "other: 058's plan pins the deferred inbox migration to introduced_in = \"0.60.0\" and to sorting after remove-legacy-constitutions-region, and task 2 repeats the 0.60.0 assumption, but the entry waits on 013's precheck task and may land after 0.60.0 ships or after another 0.60.0 entry — `specs/058-findings-route-at-discovery/plan.md`"
    outcome: routed
    target: specs/058-findings-route-at-discovery/plan.md
    decided-at: 2026-10-30T21:38:46Z
    decided-by: andy@stone.dev
---

# Review — 006-bug-workflow

## Summary

Reviewed the window 006's reopen opened: commit bca9a2fd (diff base 194e43f7), which removed the inbox template's guidance comment and placeholder item, moved the status-note rule into constitution §brownfield-inbox, and changed inbox-guidance-refresh to remove the comment when the template carries none. The five passes read that window's diff of 17 source files in full: framework/commands/groom.md and log.md, framework/constitution.md, framework/migrations.toml, framework/migrations/inbox-guidance-refresh.md, framework/templates/project/inbox.md, four runtime/src/primitives files (doc comments only), specs/006-bug-workflow/spec.md, 022's data-model.md and append-inbox-comment-aware-write scenario, 058's spec.md, plan.md and tasks.md, and specs/inbox.md. Of the scope's other 24 entries: the four generated copies (.claude/commands/ductus/groom.md and log.md, .pi/prompts/ductus-groom.md and ductus-log.md) were not read, and are confirmed by the generators-in-sync check on a fresh clone of 194e43f7 plus the patch; 11 plan Affected Files are unchanged since the diff base and were not read (framework/templates/spec/scenario.md and spec.md, framework/commands/amend.md, analyze.md, help.md and status.md, their four .claude/commands/ductus copies, README.md); 9 hold another session's uncommitted edits, not this spec's work, and were not read (six framework/rules files, specs/008-security-rules/spec.md and tasks.md, specs/068-api-backend-rules/spec.md). Rules: all 23 cross-stack statements (quality-cross, configuration-cross) were read; of the 276 stack-specific rules, only BE-INPUT-004, BE-RACE-003 and BE-COORD-002 touch this change's subjects, and none fires, because the migration's path resolution and its idempotency check are unchanged; the frontend, API, performance, observability and reliability files govern UI, HTTP, databases and services, none of which the window touches. The loaded rule files include the other session's uncommitted edits; none is to a file whose rules fired here. No MUST or SHOULD violation. Two observations: the ragged doc comments were rewrapped with their words unchanged (a prose-only round, so not re-read by an independent agent); the deferred migration's version claim was routed to 058's plan and task 2, which now take the version of the release that ships the entry.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- convention: two runtime doc comments reflowed by hand in bca9a2fd are ragged — append_inbox.rs's module doc runs one line to 109 columns, and iter_bullets' doc breaks after "Shared by the inbox count" — `runtime/src/primitives/append_inbox.rs` — **fixed**
- other: 058's plan pins the deferred inbox migration to introduced_in = "0.60.0" and to sorting after remove-legacy-constitutions-region, and task 2 repeats the 0.60.0 assumption, but the entry waits on 013's precheck task and may land after 0.60.0 ships or after another 0.60.0 entry — `specs/058-findings-route-at-discovery/plan.md` — **routed** to `specs/058-findings-route-at-discovery/plan.md`

## Skipped passes

*None.*

## Unexamined governance

*None.*
