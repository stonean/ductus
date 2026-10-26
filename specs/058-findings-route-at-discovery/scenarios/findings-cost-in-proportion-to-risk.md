---
section: "Undispositioned findings block `done`"
---

# Findings Cost in Proportion to Risk

## Context

Since this spec, every finding a run produces needs a disposition before `done`. That covers every MUST and SHOULD, every advisory analyze finding, every review observation, and every issue `/{project}:implement` surfaces outside the task in hand, which it records as a disposition task. The cost compounds:

- **Disposition work spawns disposition work.** `022-deterministic-runtime` reopened on 2026-10-19 with 22 tasks, seven of which existed only to disposition findings that other tasks produced. Its task 21 is *Disposition the findings from task 18's measurement*.
- **Closing a change is several runs, each needing owner-confirmed dispositions.** `072-governing-documents-stay-within-one-read` closed with 0 MUST and 2 SHOULD. Getting there took three review agents, a fix round, the independent re-read `AGENTS.md`'s fix-round entry requires, and an analyze round. Together they produced 18 review observations and 10 analyze findings.
- **Review and analyze outnumber building.** In October 2026, review and analyze commits outnumbered build commits about three to one: 151 against 44, classified by commit subject prefix.

072 measured this loop as the second untracked cause of the slowdown it addressed, and left it out of its own scope.

## Behavior

- **Only gating findings hold `done`.** Advisory analyze findings and review observations are still recorded, with a disposition whenever one is made. They stop holding `done`: the pre-`done` gate's `undispositioned` count covers only hard-fail and blocking analyze findings and MUST and SHOULD review violations, and every other gate check is unchanged.
- **Disposition work does not spawn disposition work.** A non-blocking finding surfaced while working a disposition task is recorded on that task, not appended as a new one.
- **Only a substantive fix round is re-read.** A review fix round gets an independent re-read only when it changed code or a durable contract; a prose-only round is not re-read. `AGENTS.md`'s fix-round entry says so, within that file's ceiling.
- **The constitution amendment fits under its size ceiling**, either by removing as many bytes as it adds or by landing after `050-constitution`'s cut.
- **The proof is a fixture.** A spec whose only undispositioned findings are advisory passes `check-review-gate`, and a spec with an undispositioned blocking finding still fails it.

## Edge Cases

- **A stored decision for an advisory finding.** It keeps matching and expiring as today; it simply no longer gates.
- **A disposition task's finding that turns out to be blocking.** It is still recorded as its own task.
- **Records written before the change.** A record whose `dispositions:` map counts advisory findings as `undispositioned` is read under the new rule and does not block.

## Open Questions

- Should undispositioned advisory findings and review observations stop holding `done`? Recommended: yes. They stay recorded and visible, decisions persist, and the inbox stays closed to them, while the gate keeps holding everything that can be wrong.
- Should a non-blocking finding surfaced while working a disposition task be recorded on that task rather than as a new one? Recommended: yes, because it ends the loop 022's tasks show while losing no finding.
- Should `AGENTS.md`'s fix-round re-read apply only to rounds that changed code or a durable contract? Recommended: yes, matching its existing stop rule, which already leaves a prose-only round unread.

## Resolved Questions

*None yet.*
