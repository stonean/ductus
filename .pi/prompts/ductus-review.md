---
description: Audit code against rules — security, reuse, quality, efficiency, simplicity. Writes review.md; blocks done on MUST violations.
argument-hint: "[--all] [--fix] [--security|--simplicity|--quality] [--since=<ref>] [--waive <rule-id> --reason <text>] [--defer <rule-id> [--target <path> [--task <n>]] [--work-item <id>] [--reason <text>]] [feature]"
---

# /ductus:review

Run a comprehensive code review against the targeted feature's implementation,
covering reuse, quality, security, efficiency, and simplicity. Produces a
`review.md` artifact alongside the spec. **Blocks the spec from reaching `done`
when MUST violations are present.**

`/ductus:review` audits **code against rules**. It is complementary to `/ductus:analyze`,
which audits **artifacts against each other**. Both should pass before a spec
advances to `done`.

## Purpose

Quality gate before `done`: audit the feature's implementation against the project's rule files across five dimensions (security, reuse, quality, efficiency, simplicity), record the findings in `specs/NNN/review.md` — each observation with its disposition — and set `blocking` in that file's own frontmatter record so `/ductus:implement`, `/ductus:analyze`, and the CI hook can hold the spec out of `done` while MUST violations stand. Waivers (with recorded justification) and deferrals that release a violation (**§Deferrals** in `.ductus/reference/review.md`) are the sanctioned exits.

## Context

Use the session target: invoke `resolve-session` and display any notices and unreadable session files it returns. If `$ARGUMENTS` carries a feature identifier, use it to override the session target, resolved through `resolve-feature` (`ambiguous` and `not-found` are domain outcomes to surface). With `--all` the target is every spec at `in-progress` or `done` and a feature identifier is redundant; report it rather than silently ignoring it. If no session target is set and no feature argument is provided, stop and tell the user to run `/ductus:target` first.

### Parsing `$ARGUMENTS`

Parse `$ARGUMENTS` for flags in any position, then treat the remaining text as the optional feature identifier. No flag is persisted to the session file. [Flags](#flags) below is the authoritative table of what each one _does_; this step is how each is _recognized_:

- **`--all`**, **`--fix`**, **`--security`**, **`--simplicity`**, **`--quality`** — bare toggles, no value.
- **`--since=<ref>`** — takes its value in `--since=<ref>` form; pass it to `compute-review-scope` as `since` (step 1). A bare `--since` with no value is an operator error: report it and stop. Do not fall back to the default diff base, which reviews a different window than the one asked for.
- **`--waive <rule-id> --reason "<text>"`** — a pair; each is an operator error without the other. Repeatable to waive more than one finding in a single invocation.
- **`--defer <rule-id>`** — with `--target <path>` (plus `--task <n>` when the target is a `tasks.md`), `--work-item <id>`, or both, and an optional `--reason "<text>"`. Each `--target`, `--task`, `--work-item` and `--reason` belongs to the `--waive` or `--defer` before it. A `--defer` naming no work, a `--task` without a `tasks.md` target, a `--waive` carrying a `--target`, `--task` or `--work-item` (a waiver names no future work), and one rule both waived and deferred in one invocation are operator errors. Repeatable.

Report an unrecognized `--flag` and stop. Never absorb it into the feature identifier, where it can resolve to a real feature whose review then covers the wrong scope.

## Scope Boundaries

- Reads the target spec, its `plan.md` (for Affected Files), the in-scope source files, the selected rule files' rules as step 2's `discover-rule-files` result gives them, `AGENTS.md`, `.ductus/config.toml`, and the `decisions:` list in its own `review.md`. The fix-and-route step (step 10) additionally reads what a disposition needs: a candidate covering spec's `spec.md` and `tasks.md` for the **§Groom decision tree** in `.ductus/reference/groom.md`, as `/ductus:groom` does. Do NOT review files outside the resolved scope, and do NOT introduce review criteria from outside the project's rule files and `AGENTS.md`.
- Writes `specs/NNN/review.md` — the whole record, in that file's own frontmatter (via `write-review`). `spec.md` is **not** written. With `--waive`, appends a waiver entry; with `--defer`, records a deferral under `decisions:`, including, once confirmed, a `done → in-progress` reopen of a `done` spec a violation is deferred to; with `--fix`, applies auto-fixable findings to the working tree; and step 10 writes each observation's disposition — a chore fix by the rule, or, only after `gate-confirm` returns a confirmed decision, a deferral of a broken contract to the spec's own task or the home the **§Groom decision tree** in `.ductus/reference/groom.md` chooses, including a `done → in-progress` reopen of the spec deferred to. The fix-round loop (steps 10 and 12) writes its round files under `.ductus/review-rounds/{feature}/`, `.ductus/review-rounds/.gitignore`, and its snapshots as git objects no ref points at. Past the read size, steps 1, 2, 8, 9, 11 and 12's primitives, `dashboard` under `--all`, and `read-section` write lists files under `.ductus/results/`. No other files are modified, and nothing is written to `specs/inbox.md` — the reviewed spec's own status transitions belong to `/ductus:implement`.
- Reference: §runtime-host-integration, §bug-handling, §brownfield-inbox (Finding dispositions), §text-first-artifacts, §spec-phase (spec-root resolution) (constitution loaded by `/ductus:target` — do not re-read).

## Inputs

- **Target** — the current `/ductus:target` feature, or every feature with
  status `in-progress` or `done` when invoked with `--all`.
- **Rules** — every file under the project's rule-file directory
  (`framework/rules/` in ductus's own repo, `specs/rules/` in adopter
  projects) selected in step 2 (`discover-rule-files`); their content is the
  authoritative review criteria. RFC 2119 language is authoritative:
  **MUST/MUST NOT** are blocking violations, **SHOULD/SHOULD NOT** are
  advisory.
- **Scope** — the spec's own change since its window's start, committed or not:
  `compute-review-scope`'s description states the rule, and what it leaves out
  is named in the record — see step 1.
- **Config** — five `.ductus/config.toml` keys influence this command:
  - `[review] tech-stack-verified` (boolean, default `false`): when `true`,
    step 1's tech-stack alignment check is skipped until the operator clears
    the key. Set, with operator confirmation, on the first successful check.
  - `[[review.disabled-rule-files]]` (array-of-tables, default empty): each
    entry has a required `file` (a basename in the rule-file directory, e.g.,
    `"accessibility-frontend.md"`) and a required `reason` (free text, the
    override's audit trail; trimmed length ≥ 16 Unicode codepoints), and is
    excluded from step 2's selection regardless of stack detection. A
    malformed entry fails fast in step 2.
  - `[rules] surfaces` (array of strings, default unset): the project's rule
    surfaces, members in `{"backend", "frontend"}` (`*-cross.md` files are
    unconditional and not members). When set, it replaces stack detection in
    step 2; when unset, step 2 falls back to the detected stack (and with no
    detected surfaces supplied, the primitive loads **all** recognized
    surfaces). The **empty list** (`[]`) is valid and means cross-only, not
    the same as unset; an unrecognized member (including `"cross"`) or a
    non-list value fails fast in step 2. Collected and persisted by `/ductus`
    (`ductus.md`, **Collect Project Inputs**).
  - `[review] re-reads-off` (string, default unset): switches step 12's
    fix-round loop off; the value is the reason, at least 16 characters
    once trimmed. Each round that needed a re-read is then recorded as not
    re-read and waived with it, and each run prints it. An empty value
    counts as unset. A malformed value halts the run at step 10, before any
    fix, naming the setting and the file.
  - `[artifacts] read-size-bytes` (integer, default 50,000): the size past
    which a step's result moves its lists to the file `lists-file` names, as
    each primitive's description states. A malformed value halts the call
    that reads it.

## Flags

| Flag | Behavior |
| --- | --- |
| _(none)_ | Review the current target across all dimensions |
| `--all` | Review every feature with status `in-progress` or `done`. Composes with all other flags. |
| `--security` | Run only the security pass |
| `--simplicity` | Run only the reuse / quality / efficiency / simplicity passes |
| `--quality` | Run only the correctness / bug-detection pass |
| `--fix` | Apply auto-fixable findings (see **§Auto-fix scope** in `.ductus/reference/review.md`) |
| `--since=<ref>` | Override where the window starts; the scope rule still applies (default: the later of the spec's last reopen and its last finished review, or `HEAD` while the reopen is uncommitted) |
| `--waive <rule-id> --reason "<text>"` | Record a waiver for a MUST or SHOULD violation (see **§Waivers** in `.ductus/reference/review.md`) |
| `--defer <rule-id>` `--target <path>` `--task <n>` `--work-item <id>` | Defer a MUST or SHOULD violation to the work that will address it (see **§Deferrals** in `.ductus/reference/review.md`) |

## Pipeline position

`/ductus:review` runs after `/ductus:implement` has produced code and before the spec
can advance to `done`. The recommended sequence is:

```text
/ductus:implement   →   /ductus:review   →   /ductus:analyze   →   spec status: done
```

`/ductus:implement` MUST NOT mark a spec `done` while the target's `review.md`
records `must-violations: > 0`. See **§Blocking semantics** in `.ductus/reference/review.md`.

## Instructions

> **For agent runtimes**: the Invoke steps below call the ductus runtime's MCP tools; without the server, stop and say to restart or re-run `/ductus` (§runtime-host-integration).

**Where a named section lives.** This command's reference is `.ductus/reference/review.md`. A section named here, or cited elsewhere as part of this command, that has no heading in this file is there. When a step points at one, invoke `read-section` with that path and every section the step names, in one call, rather than reading the whole file.

Run once per targeted feature, in order. Resolve a `[feature]` argument through `resolve-feature`, and enumerate the `--all` set from `dashboard`'s per-spec status inventory (`specs[].status ∈ {in-progress, done}`) rather than a directory scan. Before step 3, read **§Load rules** and **§Run review passes** in `.ductus/reference/review.md`, which hold the review's inputs and each pass's definition and severity.

1. When the spec's status is not `in-progress` or `done`, halt with the message item 2 of **§Resolve target and scope** in `.ductus/reference/review.md` gives. When the spec's `tasks.md` holds an unchecked box under any heading but a task this run's own deferral names, halt this feature's run naming the boxes: a review runs once implementation is complete, since nothing can say whether a task still open would change what it reads, and the write-review primitive refuses before then too. Invoke `compute-review-scope`; the primitive's description states how the diff base, `base-source`, the scope and `review-owed` are decided. When `review-owed` is false and neither `--since` nor the operator asks for a review, report `review-owed: false` — this reopening changed no code, so it owes `/ductus:analyze` and no review — and stop this feature's run, writing nothing. When `scope-count` is zero, skip the passes and waivers and jump to step 9 — an observation the reviewer supplies is still dispositioned — and step 11 emits the nothing-to-review-yet, non-blocking report; when `deleted-count` is above zero the scope is empty because every path it would hold was deleted, so that report and the record's Summary name each path in `deleted` and say no pass could read it, and a review owed for deletions never reads as a window that changed nothing; when that write refuses, since the record counts a finding it would clear unread, report the refusal and stop this feature's run. **A failed call is not an empty scope, and neither is an absent list.** Never route a failed call or a file-form result to the empty-scope branch, which records a non-blocking 0-findings review byte-identical to a genuinely clean one. Otherwise confirm tech-stack alignment first (host judgment, not a primitive): read the active config file; when its `[review] tech-stack-verified` flag is true, skip the check; else compare the AGENTS.md Tech Stack section against the code in scope, halting with the tech-stack-misalignment message (**§Blocking message** in `.ductus/reference/review.md`) on a mismatch, and — on success — confirm before persisting the flag (see the tech-stack alignment check under **§Resolve target and scope** in `.ductus/reference/review.md`) and write `[review] tech-stack-verified = true` to the **active config file** (the newest existing of `.ductus/config.toml`, `.govern/config.toml`, or the legacy root `.govern.toml`, else `.ductus/config.toml`; specs 042 and 049 — a write outside the `/ductus` migrations never creates a partial `.ductus/config.toml` alongside a lingering older file).
2. Invoke `discover-rule-files` to select this run's rule files — suffix classification, the `[rules] surfaces` selection, and the disabled-rule-files filter — and emit the ordered notice lines it returns verbatim. Pass `rules`, and under `extra-files` each rule file outside the rule-file directory that `AGENTS.md` references, so each file's rules, ID, tier and Statement, arrive in its `rule-files`, or past the read size in the file `lists-file` names, rather than through reading the rule files. Read that file with the host's file reader, in parts by line offset when it is long, never through the shell.
3. <!-- llm:performReview --> A host that can start a subagent runs this pass and steps 4 to 7 in one fresh subagent and continues at step 8 with only the findings, observations and coverage it returns, so the rule files and in-scope files never enter the host's context; a host that cannot runs them itself (**§Passes in a subagent** in `.ductus/reference/review.md`). Run the **security** pass over the in-scope files against the loaded `security-*.md` rules, returning one finding per violation (rule id, severity, file, line range, confidence, explanation), each reported under its owner by the rule's **Not flagged:** routing (**§Security pass** in `.ductus/reference/review.md`).
4. <!-- llm:performReview --> Run the **reuse** pass: flag logic that duplicates existing utilities or belongs in shared code.
5. <!-- llm:performReview --> Run the **quality** pass: detect bugs, missing error handling, unhandled edge cases, and contract violations, and apply the loaded `quality-cross.md` and every other loaded rule file no other pass takes (**§Load rules** in `.ductus/reference/review.md`); low-confidence findings are recorded separately and do not block.
6. <!-- llm:performReview --> Run the **efficiency** pass: flag N+1 queries, repeated work, and unbounded loops over user-controlled input, and apply the loaded `performance-*.md` rules.
7. <!-- llm:performReview --> Run the **simplicity** pass: flag overengineering, premature abstraction, and dead branches; mark a finding auto-fixable when a simpler form is mechanically derivable. A dimension-restricting flag (`--security` / `--simplicity` / `--quality`) skips the unselected passes. Name in the Summary each loaded rule file whose pass did not run, as unapplied (**§Load rules** in `.ductus/reference/review.md`).
8. Invoke `process-waivers` to classify the waivers recorded in `review.md` against the findings the passes just accumulated (**§Per-run waiver processing** in `.ductus/reference/review.md` and **§Malformed and duplicate waivers** in `.ductus/reference/review.md`), emitting each notice it returns (in the file form, `form` is `file`, the `applied`, `expired`, `retained` and `notices` lists are under those names in `lists-file`, and each `-count` says how many). **On a dimension-restricted run (`--security` / `--simplicity` / `--quality`), pass the skipped dimensions as `skipped-passes`**, so a waiver whose rule did not fire is _retained_, not expired, and pass `since` when the run took `--since`, so one on a file the scope does not hold is too. The applied set is excluded from the blocking count; the expired set is dropped on the next write; the retained set is left in the frontmatter untouched.
<!-- audit:ignore-promotion -->
9. Process stored decisions (host responsibility): read the `decisions:` list in `review.md` and match each observation the passes returned to the stored decision describing the same issue — observation text is the reviewer's own wording, so the match is the host's judgment, not a byte comparison. Then call the process-decisions primitive with the feature, `record: review`, `fired` — for each observation, the stored key it matched, or else its rendered line (its text, then an em dash and its path in backticks) — `restricted` set when any pass did not run (a dimension-restricting flag, or an empty scope that skipped them all), and `since` when the run took `--since`, so a decision on a file the scope does not hold is retained. Fire each violation's key too — `rule {rule-id} at {file}`, or the stored key a deferral of it was matched to. Each `matched` decision disposes of its finding under the stored outcome, with nothing asked, and the finding carries that key to step 11 as its `decision-key`, except that a waiver the rule decided does not dispose of an observation this run judges a broken contract: it goes to step 11 to be dropped, and step 10 decides the observation. An `expired` decision whose key names a file the scope holds is checked against that file first, since a pass that read the file in part leaves it unfired: when its issue still stands, fire its key and call the primitive again. Each other `expired` decision goes to step 11 to be dropped; `retained` decisions stay untouched. Each `lapsed` decision (spec 076) goes to step 11 to be dropped with the expired ones, and its finding needs a new disposition: an observation's is decided in step 10, while a violation counts again until it is fixed, waived with `--waive` or deferred anew with `--defer`; with nobody to ask, it is undispositioned and, when gating, blocks. Emit the result's `notices` verbatim. When the primitive refuses because the list does not parse, report that, disposition each observation as though nothing were stored, and tell the operator the record cannot be written until the list is repaired.

<!-- audit:ignore-promotion -->
10. Fix and route (host responsibility). Before anything is written, take the run's first fix-round base: call the start-fix-rounds primitive with the feature and keep its `base` and its `budget` for step 12; a failed call leaves no base, and step 12 records any fix the run writes as one round not re-read. A malformed `[review] re-reads-off` fails it before anything is written: report the error and stop the run. Then apply the disposition rule to each observation no stored decision disposed of, asking nobody and marking each `by-rule`: a chore is fixed in the run, and anything else is waived with its reason, except as follows. A defect that makes this review's own result wrong is never waived: it is fixed in the run or deferred as a broken contract is. An observation showing the spec's own code breaks its contract is marked `contract-break`, and is fixed in the run or deferred to the spec's own task: such a deferral is the only write that reaches the operator, and it is written only after the gate-confirm primitive returns a confirmed decision, with the operator free to name another home by the **§Groom decision tree** in `.ductus/reference/groom.md`, an external work-item ID beside it or in its place, or a waiver with their reason. A deferral to a `done` spec names the `done → in-progress` reopen in its confirmation and performs it through the set-status primitive with `from: done`. When a chore fix wrote, re-run the affected passes before step 11, as `--fix` does, so the record describes the fixed tree. A fix that fails or proves not to be mechanical is reverted, then waived by the rule, or on a broken contract proposed as a deferral; a declined deferral leaves the observation waived with the operator's reason, or undispositioned; a confirmed deferral whose write fails is undispositioned, and the failure is reported beside the observation. **With no operator to confirm**, a host that cannot ask still applies the rule, leaving each broken contract undispositioned, which holds `done` until a run with an operator decides it, and `ductus exec`, whose walker no-ops this step by design, writes nothing: each unmatched observation is recorded undispositioned. MUST and SHOULD violations are not dispositioned here: each is fixed, waived with `--waive`, or deferred with `--defer` (**§Deferrals** in `.ductus/reference/review.md`).
    The detail is under **Observations** in **§Write `review.md`** in `.ductus/reference/review.md`.

11. Invoke `write-review`, passing `loop-open` so the record marks the fix-round loop open, with the accumulated pass findings, the accumulated pass **observations** — each with its `disposition` (`outcome`, plus a deferral's `target`, `task` and `work-item`, or a waiver's `reason`), its `path`, which a broken contract left open must name, and, when step 9 matched it, its `decision-key`; each finding `--defer` named, or step 9 matched to a stored deferral, carries its deferral the same way — the waiver results (`applied` / `expired`, read from `lists-file` when step 8 returned the file form, with the anchor of a stored waiver a `--defer` replaces moved from `applied` to `expired`), `expired-decisions` — step 9's expired and lapsed decisions, a rule's waiver of a broken contract, plus a stored deferral a `--waive` replaces — `new-waivers`, the rule and reason of each `--waive`, whose files the primitive derives from the findings, and the scope. A departure entry in the spec's `## Applicable Rules` needs no `--waive`: the primitive derives its waiver itself (**§Waivers** in `.ductus/reference/review.md`), and the result's `malformed-departures` is reported beside the record, one line each, since an entry listed there waived nothing; in the file form (`form` is `file`) the list is under that name in `lists-file`, and `malformed-departures-count` says how many. Supply the required scalars the primitives don't produce — `reviewed-at` (the current UTC timestamp) and `reviewed-against` (HEAD sha); `diff-base` from step 1; **`examined`, how many of the in-scope files the passes above actually read**, a numerator against the `scope` the primitive resolves itself; **`unread`, the scope entries the passes did not read**, `[]` when they read every one, since another spec's window takes each entry it leaves out as read (**§Window and coverage** in `.ductus/reference/review.md`); and `decided-by` (`git config user.email`), which the primitive requires when any observation or violation is newly deferred or waived. With `--fix`, apply the auto-fixable findings (**§Apply `--fix` (optional)** in `.ductus/reference/review.md`, within **§Auto-fix scope** in `.ductus/reference/review.md`), re-run the affected passes, and invoke `write-review` a second time, `loop-open` included, for the post-fix counts. Render the result's `dispositions` as the `observations` row and its `analyze-freshness` as the `analyze` row (**§The `observations` row** in `.ductus/reference/review.md` and **§The `analyze` row** in `.ductus/reference/review.md`). Neither affects the exit code.

<!-- audit:ignore-promotion -->
12. Re-read the run's fixes, round by round (host responsibility; the walk and its policy are **§Fix-round loop** in `.ductus/reference/review.md`). From step 10's base, repeat: call the brief-re-read primitive with the feature and the round's base. When it reports `re-reads-off`, print the reason whatever the round's kind, record the round unless it is `empty`, and stop. On `empty`, stop. On `exempt`, record the round and take its `snapshot` as the next base. On `re-read`, hand an independent agent the returned `brief` path and nothing else — a host that cannot start one prints the path for the operator to run in a fresh session — then call the read-re-read primitive with the same base: only a `complete` result is a re-read. Fix each MUST and SHOULD it found; when the fixes apply only its prescribed edits, call the check-prescriptions primitive with that base, and on `matched` record the next round as diff-checked, with its `snapshot` as the base; any other change or verdict re-reads the round from the re-read's `snapshot`. A failed call or an unfinished re-read leaves its round not re-read, as does a round the operator declines to have re-read, with the reason they give. Disposition each NOTE as an observation. After as many re-reads that found a MUST or SHOULD as step 10's `budget` names, start no round without the operator's decision. Then invoke write-review once more with step 11's arguments but for `loop-open`, whose absence clears the mark, the loop's NOTEs and any finding left open at a stop added to its observations, plus `fix-rounds`, one entry per round in order, each with its result's `excluded` list. Under `ductus exec` the walker no-ops this step and step 10, so there is no round and the record carries no loop field.
