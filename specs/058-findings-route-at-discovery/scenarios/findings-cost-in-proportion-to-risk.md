---
section: "Only gating findings hold `done`"
---

# Findings Cost in Proportion to Risk

## Context

Since this spec, every finding a run produces needs a disposition before `done`. That covers every MUST and SHOULD, every advisory analyze finding, every review observation, and every issue `/{project}:implement` surfaces outside the task in hand, which it records as a disposition task. The cost compounds:

- **Disposition work spawns disposition work.** `022-deterministic-runtime` reopened on 2026-10-19 with 22 tasks, seven of which existed only to disposition findings that other tasks produced. Its task 21 is *Disposition the findings from task 18's measurement*.
- **Closing a change is several runs, each needing owner-confirmed dispositions.** `072-governing-documents-stay-within-one-read` closed with 0 MUST and 2 SHOULD. Getting there took three review agents, a fix round, the independent re-read `AGENTS.md`'s fix-round entry requires, and an analyze round. Together they produced 18 review observations and 10 analyze findings.
- **Review and analyze outnumber building.** In October 2026, review and analyze commits outnumbered build commits about three to one: 151 against 44, classified by commit subject prefix.

072 measured this loop as the second untracked cause of the slowdown it addressed, and left it out of its own scope.

## Behavior

- **Only gating findings hold `done`.** Advisory analyze findings and review observations are still recorded, with a disposition whenever one is made. They stop holding `done`: the pre-`done` gate's last check reads only the gating counts — hard-fail and blocking analyze findings and unwaived MUST and SHOULD review violations — never a record's `undispositioned`, and every other gate check is unchanged.
- **Disposition work does not spawn disposition work.** A non-blocking finding surfaced while working a disposition task is recorded on that task, not appended as a new one.
- **Only a substantive fix round is re-read.** A review fix round gets an independent re-read only when it changed code or a durable contract; a prose-only round is not re-read. `AGENTS.md`'s fix-round entry says so, within that file's ceiling.
- **The constitution amendment fits under its size ceiling**, either by removing as many bytes as it adds or by landing after `050-constitution`'s cut.
- **The proof is a fixture.** A spec whose only undispositioned findings are advisory passes `check-review-gate`, and a spec with an undispositioned blocking finding still fails it.

## Edge Cases

- **A stored decision for an advisory finding.** It keeps matching and expiring as today; it simply no longer gates.
- **A disposition task's finding that turns out to be blocking.** It is still recorded as its own task.
- **Records written before the change.** A record whose `dispositions:` map counts advisory findings as `undispositioned` is read under the new rule and does not block.
- **A SHOULD waived at review.** A waiver excludes it from `should-violations`, so a waived SHOULD does not hold `done`; an unwaived one does, whatever the `dispositions:` map counts.

## Open Questions

*None.*

## Resolved Questions

- **Should undispositioned advisory findings and review observations stop holding `done`?** Yes (owner pre-authorized 2026-10-26). They stay recorded and visible, decisions persist, and the inbox stays closed to them, while the gate keeps holding everything that can be wrong. Mechanically, a record's `dispositions:` map does not split its counts by tier, and the gating tiers already gate through earlier checks: an analyze hard-fail or blocking finding keeps `blocking: true` until the re-check no longer produces it, and a review MUST sets the review record's `blocking`. So the gate's undispositioned check stops reading the map's `undispositioned` count and reads the gating counts instead, of which only the review record's `should-violations` has no earlier check, so in practice an unwaived SHOULD now holds `done` at the gate, as §implement-phase already required of the author; the MUST and analyze counts stay in the sum as a guard against a hand-edited record whose `blocking` disagrees with them. The map stays, and a record with none still blocks as predating dispositions.
- **Should a non-blocking finding surfaced while working a disposition task be recorded on that task rather than as a new one?** Yes (owner pre-authorized 2026-10-26), because it ends the loop 022's tasks show while losing no finding. The finding is written into the disposition task's body with its own disposition, and the task is checked once every finding on it has one; a blocking one still gets its own task.
- **Should `AGENTS.md`'s fix-round re-read apply only to rounds that changed code or a durable contract?** Yes (owner pre-authorized 2026-10-26), matching its existing stop rule, which already leaves a prose-only round unread.
- **Does analyze's disposition drift still block a `done` spec and revert under `--fix`?** No (owner pre-authorized 2026-10-26). It reported a `done` spec whose record counts undecided findings as contradicting its status, and once those findings stop holding `done` there is no contradiction: a gating finding already surfaces as review-state or analyze-state drift. Kept blocking, it would hold every `done` spec with an undecided observation and force a reopen to clear it, the loop this scenario ends. So it is reported as an advisory finding, and `--fix` no longer reverts on it. Nor does any other trigger revert a `done` spec whose own new analysis is blocking, a case this revert used to catch when the finding was left undecided: analyze-state drift reports it on every run until a fix clears it or a route onto the spec reopens it.
