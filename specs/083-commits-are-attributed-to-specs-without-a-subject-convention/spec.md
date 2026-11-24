---
status: in-progress
dependencies: [079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed]
cross-spec-impact: [066-audit-records-and-the-completion-gate]
next-criterion: 17
---

# 083 — Commits are attributed to specs without a subject convention

`/{project}:implement` commits a task's code together with that task's tick in `tasks.md`, so the commit touches the spec's directory and is attributed to the spec by the rule that already reads directories, whatever its subject says. `compute-review-scope` reports how many of a window's commits no spec claimed, so code that nothing attributed is visible rather than silent.

## Motivation

- **An adopter's subjects could never name a spec.** An adopter reported on 2026-11-23 that its git host rejects any subject not matching `^\d+ - \S`, a pre-receive hook enforcing a work-item id. At `0db66fcc`, `subject_scope_forms` in `runtime/src/primitives/commit_attribution.rs` read a spec only from a subject's leading `kind(scope)` and named none when the text before the first `(` held whitespace, so `123 - fix(022): x` named no spec. The adopter measured its four repositories with `git log --since=2026-10-01 --format=%s` and `grep -cE`: of 2,152 subjects, none matched `^[^ (]+\(`, the form the parser read, and 1,483 named a spec as `^[0-9]+ - [0-9]{3}(\.[0-9]+)?[:,( ]`. Those repositories are outside this one, so the figures are the adopter's and were not re-derived here.
- **Unattributed code held every open spec.** A code commit that named no spec and touched no spec directory stayed in the window of every spec holding it until a committed review credited its paths, as [081 — Reviews and checks follow what changed](../081-reviews-and-checks-follow-what-changed/spec.md)'s AC13 has it.
- **The directory rule needed no subject.** At `0db66fcc`, `attribute_window` in `runtime/src/primitives/compute_review_scope.rs` gave a commit whose subject named no spec, and which touched one spec's directory, to that spec alone: every path it changed was in that spec's review scope and in no other's, under [079 — Review and analysis work converges](../079-review-and-analysis-work-converges/spec.md)'s AC6. Measured in this repository at `0db66fcc` with `git log --no-merges --since=2026-10-01 --name-only`, counting a commit as code when it listed a path outside 081's artifact set (the spec root, `.ductus/`, `AGENTS.md`, `CLAUDE.md`): 704 of 987 code commits also touched a spec directory.
- **The task's commit did not have to carry its tick.** `/{project}:implement`'s walk ticked a task in `tasks.md` and then prompted for a commit, or with `--auto` committed on its own (steps 7 and 8 of `framework/reference/implement.md` §Walk through tasks (per task, in order)), but said nothing of what that commit held.
- **The miss was silent.** No result counted the commits no spec claimed, so a window held by unattributed code looked the same as one that was not.

## Behavior

- **A task's code lands with its tick.** After each task, `/{project}:implement` commits the task's code and that task's tick in `tasks.md` together: on its own with `--auto`, and without it once the operator confirms the commit, whose proposed subject the confirmation shows and the operator may change. It stages only the paths the task edited and `tasks.md`, never the whole tree, and pushes nothing. The commit touches the spec's directory, so it belongs to the spec by the directory rule, and when its subject names no spec every path it changed is that spec's to review and no other open spec's.
- **No subject convention.** Attribution reads no setting and no subject form beyond the leading `kind(scope)` it read before this spec, and the framework asks for none.
- **The count is reported.** `compute-review-scope` reports how many of the window's commits it examined and how many of them no spec claimed, whether or not a committed review has credited their paths. A commit is unclaimed when no spec claims it and it changed at least one path in the project, so a clean merge is examined and never unclaimed. With no window to walk, an empty diff base, both counts are absent rather than zero. Which of their paths still owe a review stays in `owed-by` and `scope`, and which were credited in `excluded`. `write-review` renders both counts in the `## Summary` it regenerates in `review.md`, so every review shows them, and the record's frontmatter gains no field.

## Edge Cases

- **A task committed in several commits.** Only the commit holding the tick touches the spec's directory. The earlier ones are claimed by nothing and stay in every open spec's window until a committed review credits their paths, and the count includes them.
- **A subject that names another spec.** It still gives the commit's code to the spec it names, as 081 has it; the spec whose tick the commit holds reviews only its own directory's files.
- **A commit touching another spec's directory too**, a tick there or a cross-spec edit. It belongs to both, and each takes its own plan's paths and the unplanned ones, as 079's AC6 has it.
- **A commit the hook refuses.** The walk halts and reports the refusal. The task's code and tick stay on disk uncommitted, and the next task does not start.
- **An operator who declines the commit.** Nothing is committed, the changes stay on disk, and the walk goes on to the next-step offer as it did before this spec. A later commit holding the task's code without its tick is unclaimed and counted.
- **A spec with no window.** An empty diff base walks no commit, so the counts are absent, never a zero that reads as a window fully attributed.
- **A task that changes no code.** Its commit holds only artifacts and owes an analysis and no review, as 081 has it.
- **Code committed outside `/{project}:implement`.** It is attributed as before this spec and counted when no spec claims it.
- **A squash merge on the host.** It carries the code and the tick in one commit, so the spec still claims it. A merge commit claims nothing and contributes only what it changed itself, as before.

## Acceptance Criteria

- [x] AC10: `framework/commands/implement.md` and `framework/reference/implement.md` §Walk through tasks (per task, in order) have the commit after a task's tick hold the task's code and that tick together, with `--auto` and without it.
- [x] AC15: Without `--auto`, the walk makes that commit itself once the operator confirms it, showing the proposed subject, which the operator may change. It stages only the paths the task edited and `tasks.md`, and pushes nothing. A declined commit commits nothing and the walk goes on to the next-step offer; a refused commit halts the walk, reporting the refusal, before the next task.
- [x] AC11: A commit whose subject is `123 - 022: x`, holding a code change and a tick in spec 022's `tasks.md`, puts every changed path in 022's review scope and none in another open spec's. A fixture proves both directions.
- [x] AC12: A commit holding the same code change and no tick, under the same subject, stays in every open spec's review scope. A fixture proves it.
- [x] AC13: `compute-review-scope`'s result reports how many of the window's commits it examined and how many of them no spec claimed, whether or not a committed review credited their paths, and its compiled description states both. A fixture proves a window holding unclaimed commits reports them, one whose unclaimed commits a committed review credited still reports them, one holding none reports zero beside the count examined, a clean merge is examined and not unclaimed, and a spec with an empty diff base reports neither count.
- [x] AC16: `write-review` renders, in the `## Summary` of the `review.md` it writes, how many of the window's commits the scope examined and how many of them no spec claimed, or that the scope had no window, and the review record's frontmatter gains no field. A test proves the line.
- [x] AC14: Commit attribution reads no project setting, and no framework command asks a person or an agent for a commit-subject form.

## Applicable Rules

- `QUAL-CLAIM-001` — a window whose commits no spec claimed reports how many, so it is never indistinguishable from one fully attributed.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Does attribution read a commit-subject form the project declares?** No (operator, 2026-11-23): the operator declined to make attribution depend on a commit-subject format. Instead a task's code lands with its tick, so the directory rule attributes it: the agent's recommendation, adopted by the operator, 2026-11-23. What this gives up: code committed outside `/{project}:implement`, or before its task's tick, gains no attribution and stays in every open spec's window until a committed review credits its paths; the count shows how much. Rejected: a declared subject form (a regular expression with one capture group, a prefix followed by a fixed token grammar, or a choice of named forms), which made attribution depend on a subject format; a `prepare-commit-msg` hook writing a spec trailer from the session target, which added a shipped hook and misattributed a commit made with a stale session target or none; and the count alone, which left the adopter's code in every window. The adopter's questions on the form's shape, on reading it instead of `kind(scope)` or beside it, and on its configuration section were withdrawn with the form.
- **Without `--auto`, who makes the task's commit?** The walk, once the operator confirms it: the agent's recommendation, adopted by the operator, 2026-11-23. The operator's part per task is one confirmation, so attribution depends on no one remembering to stage the tick (§design-principles). The confirmation shows the proposed subject because a host may require a form the agent does not know: the adopter's pre-receive hook rejects a subject without its work-item prefix, which a commit meets only at push. What this gives up: the operator no longer composes each task's commit by hand, and can still change its subject. Rejected: asking the operator to commit the code with its tick, which made attribution depend on the operator including the tick, a miss the count would show only afterwards.
- **Which commits does the count report as unclaimed?** Every commit in the window no spec claimed, whether or not a committed review credited its paths: the agent's recommendation, adopted by the operator, 2026-11-23. The count measures attribution, so a crediting review leaves it unchanged, and a project whose commits are never attributed keeps seeing N of M. What still owes a review is already in the result per path, in `owed-by` and `scope`, with credited paths in `excluded` (`take_unclaimed` in `runtime/src/primitives/compute_review_scope.rs` decides credit per path). Rejected: counting only uncredited commits, which fell to zero after one crediting review, hiding the miss again, and restated `owed-by` per commit; and both counts, which added a third field and a rule for a commit credited in part.
- **Where does the count appear?** In `compute-review-scope`'s result and in the `## Summary` `write-review` regenerates in `review.md`: the agent's recommendation, adopted by the operator, 2026-11-23. `write-review` already derives its denominator from the same scope (`compute` in `runtime/src/primitives/compute_review_scope.rs`) and already appends lines it derives itself to that section, so the operator sees the count on every review with no schema change. What this gives up: no gate or other primitive reads the count from the record. Rejected: the result alone, which an operator saw only by calling the primitive, leaving the miss as silent in practice as the adopter reported; and a frontmatter field in the review record as well, which edited the constitution's review-record table and the data models of 066 and 020, reopening 020 too, for a field no gate reads.
