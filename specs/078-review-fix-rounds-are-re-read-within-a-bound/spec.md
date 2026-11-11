---
status: done
dependencies: [020-code-review]
next-criterion: 16
cross-spec-impact: [020-code-review, 066-audit-records-and-the-completion-gate]
---

# 078 — Review fix rounds are re-read within a bound

`/{project}:review` gains a fix-round loop: the changes made to address a review's findings are re-read by an independent agent before the record is written, and the loop's scope, its check of prescribed edits, and its length are bounded by what the pipeline derives, not by the fixing agent's diligence. It extends [020's review](../020-code-review/spec.md).

> **Signpost:** `079-review-and-analysis-work-converges` changed this loop: a round drops a path only another spec's commits changed, its brief leaves out exempt paths even in a mixed round, and a first `write-review` marks the loop open until the last clears it. See [079](../079-review-and-analysis-work-converges/spec.md).

## Motivation

`/{project}:review` had no fix-round loop. The passes produced findings, each was fixed, deferred or waived, and `write-review` recorded the run; a fix made during the run was checked by nobody but the agent that made it. This repository ran the loop by hand under a rule in its own `AGENTS.md`: an independent agent re-read each fix round that changed code or a durable contract, and the loop stopped at a round with no MUST or SHOULD. Adopters had none of it.

Spec 076's review showed both sides. The second round's re-read found a MUST the earlier rounds had recorded as fixed. The loop also ran five fix rounds and four re-reads in one session, and most of that was avoidable:

- two rounds existed because the fixer's sweep of a changed rule missed sentences, while a loaded rule already asked for a sweep by meaning;
- one decision went to the owner because the fixer widened a re-read's brief past the round it re-read;
- one re-read verified a round that only applied edits the previous re-read had prescribed.

No instruction the agent had loaded prevented any of them, and §design-principles rules out a bound that depends on diligence. This spec bounds the loop with inputs the pipeline derives.

## Behavior

### The loop

- Once a review's findings and observations are dispositioned, and before its final `write-review` call, each **fix round** — the changes made to address findings since the previous re-read, or since the passes for the first — is re-read by an independent agent, unless every path it changed is a markdown file in the reviewed spec's own directory other than `analysis.md` or a **durable contract** (a scenario or the data model).
- Such a round — one editing only `spec.md`, `plan.md`, `tasks.md` and the like — is not re-read, and the record's Summary names it: each of those edits stales the analyze record, so `/{project}:analyze` reads it again before `done`, except a criterion's tick, which is outside the analysis digest (066's scenario `checkbox-state-is-outside-the-analysis-digest`) and is verified against the tree before the spec closes (§implement-phase). The derived path list decides, never the host's judgment.
- The re-read's brief states the severity bar. **MUST**: the round's code does what the spec or data model forbids, fails a behavior they require, or breaks one that held. **SHOULD**: a sentence the round made untrue or left contradicting another it edited, or a test it added that would pass with its behavior removed. **NOTE**: a contrived input, a style point, or anything outside the round.
- The loop stops at the first round with nothing changed since its base: after a re-read with no MUST and no SHOULD, or a matched diff check with none open, unless a NOTE is fixed afterwards, which is a round of its own. Each NOTE takes a disposition as an observation does.
- The loop covers `/{project}:review`'s fixes only. `/{project}:analyze`'s are mechanical chores its re-check reads, and `/{project}:implement`'s land before a review whose passes read them.
- The loop is on by default. A project may switch it off in `.ductus/config.toml`'s `[review]` table, with a mandatory reason, as `[[review.disabled-rule-files]]` requires one; the data model names the key and its validation, and a malformed entry halts the run. With the loop off, each round that needed a re-read is recorded as not re-read and waived with the config's reason, and each run prints that reason.

### The scope is derived

- A re-read's scope is the round's changed paths, derived from the repository: the files inside the project that differ between the round's base and a snapshot of the working tree taken as the round is briefed, less those only commits another spec claims changed and those the exemption above covers. Its brief carries that list and the severity bar, and no scope the fixer wrote, so a brief cannot widen past its round.
- A round's base is a **snapshot** of the working tree: a commit object of the whole tree, untracked files included, that moves no ref, index or HEAD. A runtime primitive takes one when the passes end and each time a re-read is briefed, so a snapshot is the tree its re-read reads. `/{project}:review` commits nothing.
- A finding outside the round's paths is a NOTE: it is dispositioned as an observation and never changes the round.

### A prescribed edit is checked by diff

- A re-read may prescribe an edit: the file, the text it replaces, and the replacement. It writes its prescriptions to a file its brief names, outside the round's scope, and a runtime primitive reads that file, so no prescription passes through the fixer.
- A round that only applies prescribed edits is verified by comparing its diff with the prescriptions — every hunk prescribed, and every MUST and SHOULD prescription applied, a file's NOTE edits being the operator's to decline, all of them or none — instead of by another re-read: the primitive replaces each old text exactly once in the base snapshot's copy of its file and compares the result with a snapshot of the working tree, as git stores both (`runtime/src/primitives/check_prescriptions.rs`, `compare`), so a checkout that converts line endings on disk compares alike, and an old text is matched as git stores it, which is the text the round's `git diff` shows, since that diff runs between two snapshots (the brief's `git diff --relative {base} {snapshot}`, in the data model's `brief-re-read`). A round with any change the re-read did not prescribe is re-read, and the primitive names the paths that differ.

### The budget

- After two re-reads that each found a MUST or SHOULD, the host stops and reports to the operator — the open findings, the rounds run, and, where the host reports it, the re-reads' cost — instead of starting another round. The operator decides whether the loop continues, or each open finding is deferred or waived.
- The count belongs to one `/{project}:review` run and starts at zero on each.

### The record

- `write-review` records the loop's outcome in `review.md`'s frontmatter from structured arguments, and renders it in the Summary, whether the host supplied it or `write-review` generated it: the fix rounds run, the re-reads finished, the rounds verified by the diff check, the rounds that needed a re-read and have no finished one (declined, or cut off), and whether the budget stopped the loop. The data model names the fields.
- A record without those fields predates the loop or comes from `ductus exec`, and reads as unexamined, never as zero.
- A round that needed a re-read and has none finished is either re-read or **waived** with the operator's reason, which the record carries for that run. `check-review-gate` holds `done` while the latest record counts such a round with no waiver.

### Hosts

- A host that can start an independent agent starts one for each re-read. One that cannot writes the re-read's brief to a file, and the operator runs it in a fresh session of any host, whose report comes back as the re-read's. A round the operator declines to have re-read is recorded as not re-read and waived with the reason the operator gives when declining, so a skipped check never reads as a clean one.

## Edge Cases

- A run whose findings were all waived has no fix round and no re-read, unless another spec's commit lands during it, which makes a round `exempt` (spec 079); one with deferrals has the rounds their writes make, and the run's own `review.md` is never a round's path.
- A round whose derived scope is empty changed nothing and is not re-read.
- A re-read that does not finish — cut off, or its agent stopped — is not a re-read. It is reported as incomplete and never counts as clean; its round is re-read again or waived with a reason.
- A round that applies prescribed edits and also changes anything else is re-read whole. A formatter run after the edits is such a change, so the check runs as soon as the prescriptions are applied.
- A prescription whose old text occurs in the base file zero times or more than once fails the check, and the round is re-read.
- A prescriptions file that is missing or does not parse is a check that could not run, reported as such, and the round is re-read.
- A snapshot, scope or diff-check call that fails is not an empty scope and not a passed check: its round counts as needing a re-read and having none.
- A round verified by the diff check is not a re-read and does not count toward the budget. Once the budget is reached, every further round waits for the operator's decision.
- With no operator to ask and no agent to start, a round that needed a re-read is recorded as not re-read with no waiver, so the gate holds `done` until an interactive run re-reads or waives it. Under `ductus exec`, whose walker no-ops the fix and loop steps (`framework/commands/review.md` step 12, its last sentence; the plan's *The exec path makes no round*), there is no round and the record carries no loop field.
- A commit this spec or no spec claims, made between rounds, changes no round's scope, since the diff runs from the snapshot to the working tree; a commit only another spec claims takes the paths it alone changed out of the round.
- In a tree another session also edits, the derived scope includes that session's uncommitted changes made since the round's base, while a path only commits another spec claims changed leaves the round. Isolating sessions with `git worktree` is the framework's answer (§concurrent-features); a finding on a path no fix in the round touched is an out-of-round NOTE.

## Acceptance Criteria

- [x] AC1: `framework/commands/review.md` runs a fix-round loop between dispositioning and its final `write-review`: each round with a changed path AC2 does not exempt is re-read by an independent agent whose brief states the severity bar, and the loop stops when nothing changes after a re-read with no MUST or SHOULD. **Superseded in part:** since `079-review-and-analysis-work-converges`'s task 3 (`0a35facc`), a path only another spec's commits changed is set apart from the round, so a round whose every other path AC2 exempts is recorded `exempt` and not re-read.
- [x] AC2: A round whose every derived path is a markdown file in the reviewed spec's own directory other than `analysis.md`, a scenario or the data model is not re-read, and the record's Summary names it.
- [x] AC3: A re-read's brief carries the round's changed paths, derived from the repository, and the severity bar, and no scope the fixing agent wrote. **Superseded in part:** since `079-review-and-analysis-work-converges`'s task 3 (`0a35facc`), the brief leaves out the paths AC2 exempts, even in a round that needs a re-read, and those only another spec's commits changed.
- [x] AC10: A round's base is a snapshot of the working tree, untracked files included, taken by a runtime primitive that moves no ref, index or HEAD, when the passes end and each time a re-read is briefed; the review commits nothing.
- [x] AC4: A finding a re-read makes outside the round's paths is recorded as an observation with a disposition and changes nothing in the round.
- [x] AC5: A round that only applies edits a re-read prescribed is verified by comparing its diff with the prescriptions, and a round with any unprescribed change is re-read. **Superseded in part:** since `079-review-and-analysis-work-converges`'s task 3 (`0a35facc`), a change only another spec's commits made is not an unprescribed change, so it leaves the diff check's verdict unchanged.
- [x] AC11: A re-read's prescriptions are old→new text pairs in a file its brief names, outside the round's scope, and the diff check reads that file through a runtime primitive, so the host passes it no prescription.
- [x] AC6: After two re-reads that each found a MUST or SHOULD, the host starts no further round without the operator's decision, and reports the open findings and the rounds run.
- [x] AC7: A re-read that does not finish is reported as incomplete and never counted as clean: its round counts in the record as needing a re-read and having none.
- [x] AC8: On a host that cannot start an independent agent, the review procedure writes the re-read's brief to a file for the operator to run in a fresh session, and a round the operator declines to have re-read counts in `review.md`'s frontmatter as not re-read and waived with the operator's reason.
- [x] AC12: `review.md`'s frontmatter records, through `write-review`'s structured arguments, the run's fix rounds, finished re-reads, rounds verified by the diff check, rounds that needed a re-read and have none finished, and whether the budget stopped the loop, and the Summary, supplied or generated, renders them; a record without these fields reads as unexamined, never as zero. **Superseded in part:** since `079-review-and-analysis-work-converges`'s task 3 (`0a35facc`), a record carrying `fix-rounds-open: true`, which a run's first `write-review` writes and its last clears, blocks `done` whether or not it holds these fields.
- [x] AC13: `check-review-gate` blocks `done` while the latest review record counts a round that needed a re-read, has no finished re-read, and carries no waiver with a reason.
- [x] AC14: The loop runs unless `.ductus/config.toml`'s `[review]` table switches it off with a reason; then each round that needed a re-read is recorded as not re-read and waived with that reason, each run prints it, and a switch without a valid reason halts the run naming the setting and the file.
- [x] AC15: A snapshot, scope or diff-check call that fails, or a prescriptions file that is missing or does not parse, leaves its round counted as needing a re-read; it never reads as an empty scope or a passed check.
- [x] AC9: This repository's `AGENTS.md` re-read rule points at the framework's statement instead of restating it.

## Applicable Rules

- `QUAL-CLAIM-001` — a re-read that did not finish must not read as one that found nothing
- `CFG-ENV-001`, `CFG-ENV-002`, `CFG-ENV-008` — the `[review]` switch is declared, inventoried, and parsed with a fail-fast error

## Open Questions

*None — all resolved.*

## Resolved Questions

- **What replaces the independent re-read on a host that cannot start an independent agent?** A fresh session the operator runs from the written brief, else a record that the round was not re-read (owner, 2026-11-03). The agent registry (`framework/bootstrap/ductus.md`, *Agent registry*) records no host's subagent support, so the procedure must not depend on knowing it. Rejected: the operator doing the re-read, which makes independence rest on who wrote the fix, and no loop on such a host, which leaves every fix there unchecked.
- **Where does a re-read's derived scope come from?** A tree-to-tree diff between the round's base and a snapshot of the working tree taken as the round is briefed, bounded to the project by `compute-review-scope`'s own helpers (owner, 2026-11-03). The owner first chose `compute-review-scope`'s `modified-since` with the base as `--since`; that diff runs from the base through the index to the working tree (`runtime/src/primitives/compute_review_scope.rs`, `diff_since`), and a snapshot base holds unstaged edits the index does not, so every round re-listed every earlier round's unstaged fixes, which 078 task 3's test found (owner, 2026-11-03). Rejected: `modified-since`, for that reason; making `diff_since` compare the working tree without the index, which hashes every file on every review; building a snapshot inside `compute-review-scope`, which writes nothing outside `.ductus/results/`; and the host's own `git diff`, a hand-rolled equivalent of a primitive.
- **What marks a round's base?** A snapshot of the working tree, untracked files included, as a commit object that moves no ref, index or HEAD, taken by a new primitive when the passes end and each time a re-read is briefed (owner, 2026-11-03). `--since` must resolve to a commit and is diffed to the working tree (`runtime/src/primitives/compute_review_scope.rs`, `run` and `diff_since`), and a review runs before its work is committed, so only a snapshot scopes a round to itself; taking one writes an object, so it is not a mode of the read-only `compute-review-scope`. Rejected: a commit per round, which has `/{project}:review` commit partial work into the adopter's history, and HEAD's sha recorded without committing, which scopes each round to the whole uncommitted change.
- **How is a prescribed edit represented so the diff check is mechanical?** Old→new text pairs the re-read writes to a file its brief names, outside the round's scope, which a runtime primitive parses and applies to the base snapshot (owner, 2026-11-03); the data model fixes the encoding. It is the form 076's rounds applied by hand (commits a0427653, 104cdf69 and c5ad7370). Rejected: a fixed format in the re-read's report that the host parses, which makes the check's input the fixer's transcription, and a patch file, whose hunk headers and context a re-read writes by hand.
- **Does the record carry the loop's outcome as frontmatter fields, or only in the Summary?** Frontmatter fields, from structured `write-review` arguments, also rendered in the Summary (owner, 2026-11-03), a supplied one as well as a generated one (owner, 2026-11-09). §design-principles asks a check that could not reach part of its subject for "a count of what was examined", and the count of rounds left unchecked belongs where a check can parse it, as `examined` does (`runtime/src/schema/primitives.rs`, `WriteReviewArgs.examined`). Rejected: the Summary only, which a person can read and no check can.
- **Does `check-review-gate` hold `done` while the record counts a round that needed a re-read and has none finished?** Yes, unless the round is waived with the operator's reason; declining a re-read is that waiver, and a cut-off re-read is re-run or waived (owner, 2026-11-03). §design-principles gives known outstanding work three dispositions and rules out "a caveat beside a completion claim", and the first resolution above lets the operator decline. Rejected: blocking on every such round, which leaves declining no path to `done` but a whole `/{project}:review` re-run, and a passing gate's guidance (`runtime/src/primitives/check_review_gate.rs`, `passing_notices`), which is that caveat.
- **Is the loop on by default, or opt-in?** On by default, with a `[review]` switch that turns it off only with a mandatory reason, which each record carries as the rounds' waiver (owner, 2026-11-03). §design-principles rules out a check that runs only when someone remembers to ask for it, and `[[review.disabled-rule-files]]` is the precedent for a review switch that requires a reason (`framework/reference/review.md`). The one cost on record is 076's, four re-reads on a corpus-wide sweep (the Motivation above), whose size no artifact states with a method; a typical round's cost is unmeasured. Rejected: on with no switch, which makes a project that decided against re-reads decline every round, and opt-in, which leaves every project that never opts in where the Motivation starts.
- **Does the loop also cover `/{project}:analyze`'s fixes and `/{project}:implement`'s disposition fixes?** No, `/{project}:review`'s only (owner, 2026-11-03). Review's fixes land after its passes and before its record, and a code change stales no review (`framework/reference/review.md`, the durable-contracts rule), so nothing else reads them. Analyze's fixes are mechanical chores its step-20 re-check re-detects (`framework/commands/analyze.md`), and implement's land before a review whose scope holds them; a review finding deferred to a task on its own spec keeps holding `done` until a later review stops producing it (§brownfield-inbox, Finding dispositions). Rejected: adding analyze, or analyze and implement, which re-reads fixes another check already reads.
- **Which changed paths make a round need a re-read?** Any path except a markdown file in the reviewed spec's own directory other than `analysis.md`, a scenario or the data model (owner, 2026-11-03; worded as AC2, owner, 2026-11-04). Every `.md` there but `analysis.md` is an analyze subject (`runtime/src/primitives/analyze_subjects.rs`, `is_analyze_subject`), so an edit to one stales the analysis and the gate holds `done` until `/{project}:analyze` reads it again; scenarios and the data model are the review's durable contracts (`is_review_contract`). Rejected: exempting all markdown but those, which leaves command, reference and doc prose unread, where 076's re-read SHOULDs were (commit a0427653), and exempting nothing, which re-reads what the analyze gate already sends back to `/{project}:analyze`.
- **Why split from code-review:** 020 owns `/{project}:review`'s passes, its record and its blocking gate; 078 adds a loop over the fixes a review makes, with its own primitives, `[review]` key and budget. The overlap Family 7 sees — `framework/commands/review.md`, `framework/bootstrap/ductus.md`, `framework/constitution.md` — is where the loop joins the command, the config inventory and the record schema, not a remainder of 020's work, so folding would hold a closeable feature to 020's unrelated outstanding work (owner, 2026-11-03).
