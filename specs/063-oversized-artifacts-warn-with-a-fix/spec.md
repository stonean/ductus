---
status: in-progress
dependencies: [041-task-pruning, 052-spec-supersession-and-consolidation, 058-findings-route-at-discovery]
next-criterion: 18
---

# 063 — Oversized artifacts warn with a fix

The pipeline warns when a spec artifact is larger than an agent can read in one call, and every warning names a fix the adopter can act on at the spec's current status. The check is advisory: it surfaces the problem where fixing it is cheapest and never blocks a transition on its own.

## Motivation

An agent's file reader returned only part of a large file. Claude Code's Read returned a partial first page once a file passed 25,000 tokens; the cap counted tokens, not lines (measured 2026-09-28 against this repository's plan markdown: a 727-line, ~54 KB file came back as a partial first page, while 2,500 short lines read whole).

`/implement` read the spec, plan, tasks and targeted scenario once, at setup (`framework/commands/implement.md`, Progressive context loading). An adopter's 2,200-line `plan.md` was therefore never read in full by that read, and whether the rest was read depended on the agent choosing to page through it. That is the failure §grounding names in *A partial read is not a read*, and nothing enforced the rule: no command reported an artifact's size, so the first sign of an oversized file was a decision made from part of it.

The fixes that existed were not named anywhere a reader met the problem. Each depended on which file had grown and on the spec's status: pruning a spent `tasks.md` with [/prune](../041-task-pruning/spec.md), splitting or trimming the spec, promoting a scenario to its own spec (§scenario-promotion). Some of them reopened a `done` spec and some did not.

A trimmed plan also grew back. `/plan` told the agent that code snippets, function signatures and package paths belonged in a plan's Technical Decisions (`framework/commands/plan.md`, Create the plan), so each new plan re-accumulated the code sketches a trim had removed.

## The check

The check measures each **subject artifact** of a spec in bytes and compares it to a **threshold**:

- **Subjects** are `spec.md`, `plan.md`, `tasks.md`, `data-model.md`, and each scenario file under the spec. `research.md` is not a subject, because no command reads it. `review.md` and `analysis.md` are not subjects either. Their write primitives regenerate each whole on every run, so none of the fixes below applies to them, and the commands that read them read the record in their frontmatter.
- **Bytes, not tokens.** The runtime has no tokenizer, and the cap that matters differs per host and per host setting (`CLAUDE_CODE_FILE_READ_MAX_OUTPUT_TOKENS` changes Claude Code's, per its [environment variables reference](https://code.claude.com/docs/en/env-vars)), so a byte count is the measurable proxy. The threshold defaults to 50,000 bytes, and a project may set its own in `.ductus/config.toml`. The threshold is the only setting: there is no switch that turns the check off. The setting is documented where adopters look for configuration: the README's Configuration section, which lists the keys a project may edit by hand, and the commented schema in `framework/bootstrap/ductus.md` §Project Configuration. Each states the default, that the value is a whole number of bytes, and that it can raise or lower the threshold but not switch the check off.
- **A heuristic, not a guarantee.** No byte count is reliably under the cap: one file read whole at 58,284 bytes while another was cut off at about 54 KB (see Resolved Questions). A warning therefore says the file is larger than the configured read size and *may* not be read in one call. It never says the file *will* be truncated, which a byte count cannot show.
- **Read-page count.** A subject's read-page count is its size divided by the threshold, rounded up. A file at or under the threshold is one page and is never reported.

It runs in three places:

- **`/clarify`**, because splitting a spec is cheapest while it is `draft` or `clarified`, before a plan and tasks exist for the part that moves.
- **`/plan`'s readiness check**, beside the checks it already runs before proposing `planned`.
- **`/analyze`**, as an advisory finding, so it gets a disposition like any other finding ([058](../058-findings-route-at-discovery/spec.md)): fixed, routed, or discarded with a reason. It never has to be *fixed* to reach `done`, only decided.

At `/clarify` and `/plan` it is a warning in the command's output. Neither command blocks its transition on it.

## The warning names a fix

Each warning names the file, its size, the threshold, and its read-page count, then the fixes for that kind of artifact:

| Artifact | Fixes |
| --- | --- |
| `tasks.md` | `/prune` the spent task sections. A `tasks.md` still over the threshold after pruning means the spec itself is too big, so the fixes for `spec.md` apply. |
| `spec.md`, `plan.md` | Split the spec, when a slice can reach `done` on its own. Or trim it: in a plan, the code sketches and the Affected Files rows for finished work. |
| `data-model.md` | Split the spec, or trim the file. Never into sub-files: review staleness tracks `data-model.md` by its exact name (`is_review_contract` in `runtime/src/primitives/analyze_subjects.rs`), so content moved to a sibling file would drop out of the review's subjects. |
| a scenario | Promote it to its own spec (§scenario-promotion). |
| any subject, in `/analyze` | Discard the finding with a reason. |

The fixes a warning names depend on where it fires and on the spec's status:

- **A discard is named only in `/analyze`.** `/clarify` and `/plan` keep no findings record, so there is nothing to discard into.
- **At `done`, the warning says which fixes reopen the spec**, by the test §spec-lifecycle applies to every edit under a spec: an edit that changes a claim the spec makes takes the `done → in-progress` back-edge. Splitting the spec and promoting a scenario always reopen it, because each removes requirements from what the original asserts, even though another spec now asserts them. Trimming reopens it unless the trim changes no claim. Pruning `tasks.md` and a discard never reopen it.
- **No warning names a command that does not exist.** No command splits a spec ([/consolidate](../052-spec-supersession-and-consolidation/spec.md) only merges), so a split is described as the manual route it is: create the new spec with `/specify`, move the content into it, and link it from the original.

## A discard re-fires on growth

A discarded warning stays discarded while its file keeps the read-page count it had when the discard was recorded. Once the file grows into another page, the warning fires again and needs a new decision, since the reason given for one page does not cover two. A file that shrinks does not re-fire a discard.

## Plans stop regrowing

`/plan` no longer tells the agent that code snippets belong in a plan. A technical decision states the decision and its rationale, and names the code it concerns by path rather than reproducing it, so a plan trimmed once stays trimmed.

## Edge Cases

- **A subject that does not exist** is neither reported nor counted as not examined. At `draft`, `plan.md` and `tasks.md` normally do not exist yet, and their absence is a state rather than a gap in the check.
- **A subject that cannot be read** is reported as not examined, never counted as under the threshold.
- **The size is the file's bytes on disk**, line endings and multi-byte characters included. Nothing is normalized first.
- **A file exactly at the threshold** is one read page and is not reported. One byte over is two pages.
- **Several oversized subjects** get one warning each. A spec with many scenarios is checked file by file.
- **An invalid configured threshold** (anything but a positive whole number of bytes) is reported, and the check runs at the 50,000-byte default and says that it did. A project's setting is never replaced silently.
- **Changing the threshold recounts every subject's pages.** A discard recorded at one page count fires again if the new threshold puts its file at a higher count.
- **The markdown-only path** measures with the host's own file tools. A subject whose size those tools cannot report is reported as not examined, never as under the threshold.

## Out of scope

These gaps remain after a warning, and each is its own change:

- **Splitting a spec.** No command splits one, though splitting is the main fix for an oversized `spec.md`, `plan.md` or `data-model.md`.
- **Moving existing spec content into a scenario.** `/amend` records only new input, and its scenario route appends an implement task for behavior that is already built.
- **Reading a plan by section.** A large plan that really is one concern has no fix but paging or a discard.

## Acceptance Criteria

- [x] AC1: `/analyze` reports an advisory finding for each subject artifact (`spec.md`, `plan.md`, `tasks.md`, `data-model.md`, each scenario) larger than the threshold, naming the file, its size in bytes, the threshold, and its read-page count.
- [x] AC2: `/clarify` and `/plan`'s readiness check report the same over-threshold subjects as a warning, and neither blocks its transition on it.
- [x] AC3: `research.md`, `review.md` and `analysis.md` are never reported, whatever their size.
- [x] AC4: With no threshold configured, the threshold is 50,000 bytes; a threshold set in `.ductus/config.toml` replaces it.
- [x] AC5: An invalid configured threshold is reported, and the check runs at the 50,000-byte default and says so; the default never replaces a project's setting silently.
- [x] AC6: Each warning names the fixes for its artifact kind from the table in [The warning names a fix](#the-warning-names-a-fix).
- [x] AC7: A discard is offered in `/analyze` only.
- [x] AC8: On a `done` spec, the warning says that splitting the spec and promoting a scenario reopen it, that trimming reopens it unless the trim changes no claim, and that pruning `tasks.md` and a discard do not.
- [x] AC9: A warning that recommends a split describes the manual route and names no command that does not exist.
- [x] AC10: A discarded finding fires again once its file grows into a higher read-page count than the one recorded with the discard, and not before.
- [x] AC11: A subject that cannot be read is reported as not examined.
- [x] AC12: On the markdown-only path, a subject whose size the host's file tools cannot report is reported as not examined.
- [x] AC13: `/plan`'s guidance no longer tells the agent that code snippets belong in the plan.
- [x] AC14: A warning says its file *may* not be read in one call and never claims the file *will* be truncated.
- [x] AC15: A subject artifact that does not exist is neither reported nor counted as not examined.
- [x] AC16: A subject's size is its byte count on disk, and a file exactly at the threshold is not reported.
- [x] AC17: The threshold setting is documented in `README.md`'s Configuration section and in `framework/bootstrap/ductus.md` §Project Configuration, each stating its default of 50,000 bytes, that its value is a whole number of bytes, and that it cannot switch the check off.

## Applicable Rules

- `QUAL-CLAIM-001` — a subject that could not be read, or a check that could not run, is reported as such and never as "nothing over the threshold".

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Is splitting a spec in scope?** No. Operator decision, 2026-10-04, at `/ductus:groom`: this spec covers the warning, the status-aware fixes it names, the discard path, and `/plan`'s guidance on code in plans. A split command is its own spec if one is wanted, and the warning describes the manual route until then.
- **What is the default threshold, and is it under the cap?** 50,000 bytes, as a heuristic rather than a guarantee. It is the round number below the smallest file recorded past a cap. No byte count can be guaranteed under the cap, because the cap varies by host and setting and token density varies by content. Two measurements disagree: on 2026-09-28 a ~54 KB, 727-line plan in this repository came back from Claude Code's Read as a partial first page, and on 2026-10-04 the Read tool of a different Claude Code session returned all 58,284 bytes (325 lines) of `specs/022-deterministic-runtime/spec.md` in one call. Both measurements count bytes on disk and record what the host's Read returned for the whole file. So the threshold is configurable, and the warning says *may*, not *will*.
- **Can a project switch the check off?** No. It can only set the threshold. A host with a larger cap needs a larger threshold, and a single oversized file is discarded in `/analyze`. An off switch would be a setting with no known host to justify it, which spec 022 rejected for per-command opt-outs on the same grounds: configuration surface is permanent technical debt, and the need is speculative (022, Resolved Questions, *Per-command opt-out*). Without a switch, an adopter cannot quietly silence the check and miss the one file that really is too big. If a host with no read cap appears, a later change adds the switch with that host as its example.
- **Are `review.md` and `analysis.md` subjects?** No. A warning must name a fix, and none applies to them: their write primitives regenerate each whole on every run, so it cannot be pruned, split, trimmed or promoted. Their readers read the frontmatter record: `/analyze` reads the review record from `review.md`'s frontmatter and the `decisions:` list from `analysis.md`'s (`framework/commands/analyze.md`), and `/implement`'s gate reads both through `check-review-gate`. They are also far under the threshold. On 2026-10-04, `wc -c` over every `review.md` and `analysis.md` under `specs/` in this repository found the largest to be 23,160 bytes (`specs/059-project-in-a-repository-subdirectory/review.md`).
- **Does a split reopen the original `done` spec when its content is only re-homed?** Yes. §spec-lifecycle counts an edit as mechanical only when it "changes no claim the spec makes — no requirement added, removed, or reworded", and it applies that test one spec at a time. Moving a requirement into a new spec removes it from what the original asserts, so the split is a meaningful edit to the original, whoever asserts the requirement afterwards. Promoting a scenario is the same: §scenario-promotion replaces the scenario with a dependency reference in the parent spec. The same test corrected an earlier draft of this spec, which said every fix except pruning `tasks.md` and a discard reopens the spec. A trim reopens it only when it changes a claim, as `/prune` reopens a spec only when a decision it moves home edits the design record.
