---
status: draft
dependencies: [010-agent-autonomy, 058-findings-route-at-discovery]
cross-spec-impact: [050-constitution]
next-criterion: 7
---

# 074 — Delegated Gates Leave Proposals for Review

An operator can hand a command's gates over for later review: the command writes each decision it would have asked for as a proposal where its gate reads it, continues with work that does not depend on it, commits nothing, and on the operator's return walks the proposals one at a time.

## Motivation

On 2026-10-15 an operator told an agent "continue to work this until done. do not commit anything, i want to review", and no mode fit (verified 2026-10-26 at HEAD c0952038). Per-step confirmation was the default. `/{project}:implement --auto` skipped the per-task commit prompt and the next-step offer and recorded a disposition task and kept working, but it still paused at the completion gate, stuck detection, out-of-bounds writes, spec and plan edits or new tasks, and risky actions — and it committed, the one thing this operator had ruled out. Every other decision had no delegated path: status transitions waited on explicit approval; each fix or route was confirmed before it wrote, and with no operator nothing was written and each finding was recorded undispositioned; open questions blocked `/{project}:plan` until `/{project}:clarify` resolved them.

Proposing already happened, but only in the moment: `/{project}:clarify` proposed an answer with its rationale, and `/{project}:analyze` and `/{project}:review` proposed a disposition and wrote it once `gate-confirm` returned a confirmed decision. Nothing stored an unconfirmed proposal — stored decisions were routed or discarded, dispositions fixed, routed, discarded or undispositioned. So an agent left alone stopped at the first gate or went past it, and neither left its would-be decisions where the operator could review them. The workaround was prose — a grounded, tagged proposal written where the owner would see it — which no field marked, no command listed and no gate kept from being read as decided, against the constitution's "never design a pipeline feature that depends on human diligence" and "work that is not complete MUST never be indistinguishable from work that is".

## Delegated mode

The mode is a per-invocation flag on the commands that gate on the operator, never session state or spec frontmatter, which [010 — Agent autonomy](../010-agent-autonomy/spec.md) Resolved Q4 rules out; 010 weighed only full autonomy against the gated `--auto`, which this leaves unchanged. A proposal records who proposed it, when, and its grounding, in the artifact the gate it stands in for reads. It is a state no gate counts as decided, the counterpart of the dispositions in [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md): a proposed answer leaves its question open, a proposed disposition leaves its finding undispositioned, no proposed status change is made, and nothing is committed. On return the operator walks the pending proposals one at a time, per the constitution's "decisions go to the owner one at a time", and each accepted one is applied through the normal write path.

## Acceptance Criteria

- [ ] AC1: `/{project}:implement`, `/{project}:clarify`, `/{project}:analyze` and `/{project}:review` accept a per-invocation delegated flag, and no session file or spec frontmatter enables the mode.
- [ ] AC2: Under the flag, each decision the command would put to the operator is written as a proposal to the artifact its gate reads, carrying who proposed it, when, and its grounding, and the command continues with work that does not depend on it.
- [ ] AC3: A pending proposal is counted as decided by no gate: its question stays open, its finding stays undispositioned, no proposed status change is made, and the pre-`done` gate blocks while a proposal on the spec is pending.
- [ ] AC4: Under the flag nothing is committed or pushed, and a push or risky action is gated as in every mode.
- [ ] AC5: Pending proposals are walked one at a time; an accepted proposal is applied through the normal write path (`set-status`, `gate-confirm`, the question and disposition primitives), a rejected one is removed, and work recorded as depending on it is named so it can be unwound.
- [ ] AC6: `--auto` behaves as it did before this spec.

## Open Questions

- Which command walks pending proposals on return — a new command, `/{project}:status`, or each gating command on its next run — and does `/{project}:status` show a spec's pending count?
- Where does a proposal live for each gate kind: a proposed answer beside its question, a proposed disposition in the audit record, a proposed status change where?
- Is dependent work left undone, or done and recorded as depending on a named proposal so a rejection shows what to unwind?
- Under the flag, how does uncommitted work interact with the review window and the analysis digest, both of which read the working tree?
- Which specs besides 050 take a declared cross-spec impact — 000's gate enforcement, 014's clarify, 046's status view, 057's records?
- Does the pre-`done` gate accept the flag itself, or is a proposed `done` transition only a pending proposal the gate blocks on?
