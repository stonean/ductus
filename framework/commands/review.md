---
description: Audit code against rules — security, reuse, quality, efficiency, simplicity. Writes review.md; blocks done on MUST violations.
argument-hint: "[--all] [--fix] [--security|--simplicity|--quality] [--since=<ref>] [--waive <rule-id> --reason <text>] [feature]"
---

# /{project}:review

Run a comprehensive code review against the targeted feature's implementation,
covering reuse, quality, security, efficiency, and simplicity. Produces a
`review.md` artifact alongside the spec. **Blocks the spec from reaching `done`
when MUST violations are present.**

`/{project}:review` audits **code against rules**. It is complementary to `/{project}:analyze`,
which audits **artifacts against each other**. Both should pass before a spec
advances to `done`.

## Purpose

Quality gate before `done`: audit the feature's implementation against the project's rule files across five dimensions (security, reuse, quality, efficiency, simplicity), record the findings in `specs/NNN/review.md` — each observation with its disposition — and set `blocking` in that file's own frontmatter record so `/{project}:implement`, `/{project}:analyze`, and the CI hook can hold the spec out of `done` while MUST violations stand. Waivers (with recorded justification) are the sanctioned escape.

## Context

Use the session target: invoke `resolve-session` and display any notices and unreadable session files it returns (on the markdown-only path, read the shared default `.ductus/session.toml` instead — §concurrent-features). If `$ARGUMENTS` carries a feature identifier, use it to override the session target — resolve that override through `resolve-feature` (exact directory name, feature number, or unique partial slug; `ambiguous` and `not-found` are domain outcomes to surface). With `--all` the target is every spec at `in-progress` or `done` and a feature identifier is redundant; report it rather than silently ignoring it. If no session target is set and no feature argument is provided, stop and tell the user to run `/{project}:target` first.

### Parsing `$ARGUMENTS`

Parse `$ARGUMENTS` for flags in any position, then treat the remaining text as the optional feature identifier. Every flag is per-invocation and none is persisted to the session file — what to review is an execution-time decision, not session state. [Flags](#flags) below is the authoritative table of what each one _does_; this step is how each is _recognized_:

- **`--all`**, **`--fix`**, **`--security`**, **`--simplicity`**, **`--quality`** — bare toggles, no value.
- **`--since=<ref>`** — takes its value in `--since=<ref>` form; pass it to `compute-review-scope` as `since` (step 1). A bare `--since` with no value is an operator error: report it and stop. Do not fall back to the default diff base — a silent fallback reviews a different window than the one asked for, and reports it under a heading that claims otherwise.
- **`--waive <rule-id> --reason "<text>"`** — a pair; each is an operator error without the other. Repeatable to waive more than one finding in a single invocation.

Report an unrecognized `--flag` and stop. Never absorb it into the feature identifier: treating `--sinse=HEAD~5` as a feature name resolves to `not-found` at best, and at worst to a real feature whose review then silently covers the wrong scope.

## Scope Boundaries

- Reads the target spec, its `plan.md` (for Affected Files), the in-scope source files, the selected rule files, `AGENTS.md`, `.ductus/config.toml`, and the `decisions:` list in its own `review.md`. The fix-and-route step (step 10) additionally reads what a disposition needs: a candidate covering spec's `spec.md` and `tasks.md` for the Groom decision tree, as `/{project}:groom` does. Do NOT review files outside the resolved scope, and do NOT introduce review criteria from outside the project's rule files and `AGENTS.md`.
- Writes `specs/NNN/review.md` — the whole record, in that file's own frontmatter (via `write-review`). `spec.md` is **not** written: the record has one home (spec 057); with `--waive`, appends a waiver entry; with `--fix`, applies auto-fixable findings to the working tree; and the fix-and-route step (step 10) writes each observation's confirmed disposition — a chore fix, or a route to the home the Groom decision tree chooses, including a `done → in-progress` reopen of the spec routed to — only after `gate-confirm` returns a confirmed decision. No other files are modified, and nothing is written to `specs/inbox.md` — the reviewed spec's own status transitions belong to `/{project}:implement`.
- Reference: §runtime-host-integration, §bug-handling, §brownfield-inbox (Finding dispositions), §text-first-artifacts, §spec-phase (spec-root resolution) (constitution loaded by `/{project}:target` — do not re-read).

## Inputs

- **Target** — the current `/{project}:target` feature, or every feature with
  status `in-progress` or `done` when invoked with `--all`.
- **Rules** — every file under the project's rule-file directory
  (`framework/rules/` in ductus's own repo, `specs/rules/` in adopter
  projects) selected by the suffix-based discovery in step 2
  (`discover-rule-files`); their content is the authoritative review
  criteria. RFC 2119 language is authoritative:
  **MUST/MUST NOT** are blocking violations, **SHOULD/SHOULD NOT** are
  advisory.
- **Scope** — files referenced by the target's `plan.md` under `Affected Files`,
  unioned with any files modified since the spec advanced to `in-progress`,
  committed or not — see step 1 (`compute-review-scope`).
- **Config** — three `.ductus/config.toml` keys influence this command:
  - `[review] tech-stack-verified` (boolean, default `false`): when
    `true`, the tech-stack alignment check (see step 1) is
    skipped on every run until the operator clears the key. Set
    automatically (with operator confirmation) on the first successful
    alignment check.
  - `[[review.disabled-rule-files]]` (array-of-tables, default empty):
    each entry has a required `file` field (basename of a file in the
    rule-file directory — `framework/rules/` here, `specs/rules/` in
    adopter projects — e.g., `"accessibility-frontend.md"`) and a
    required `reason` field (free-text justification; trimmed length
    ≥ 16 Unicode codepoints). Files listed here are excluded from
    rule-file selection regardless of stack detection. Consulted in
    step 2 (`discover-rule-files`). Reason is mandatory — it is the
    audit trail for the override. A malformed entry fails fast in
    step 2, before any rule file is selected.
  - `[rules] surfaces` (array of strings, default unset): the project's
    rule surfaces, members in `{"backend", "frontend"}` (full-stack lists
    both; `*-cross.md` files are unconditional and not members). When set,
    it is the source of truth for surface selection in step 2
    (`discover-rule-files`) and replaces stack detection; when unset, step 2
    falls back to the detected stack (and with no detected surfaces supplied,
    the primitive loads **all** recognized surfaces). The **empty list**
    (`[]`) is valid and means cross-only (not the same as unset); an
    unrecognized member (including `"cross"`) or a non-list value fails fast
    in step 2. Collected and persisted
    by `/ductus` (`ductus.md`, **Collect Project Inputs**).

## Flags

| Flag | Behavior |
| --- | --- |
| _(none)_ | Review the current target across all dimensions |
| `--all` | Review every feature with status `in-progress` or `done`. Composes with all other flags. |
| `--security` | Run only the security pass |
| `--simplicity` | Run only the reuse / quality / efficiency / simplicity passes |
| `--quality` | Run only the correctness / bug-detection pass |
| `--fix` | Apply auto-fixable findings (see **§Auto-fix scope** in `.ductus/reference/review.md`) |
| `--since=<ref>` | Override the diff base (default: the parent of the commit at which the spec advanced to `in-progress`, so work committed with the transition is inside the window, or `HEAD` when that transition is not committed yet) |
| `--waive <rule-id> --reason "<text>"` | Record a waiver for a MUST violation (see **§Waivers** in `.ductus/reference/review.md`) |

## Pipeline position

`/{project}:review` runs after `/{project}:implement` has produced code and before the spec
can advance to `done`. The recommended sequence is:

```text
/{project}:implement   →   /{project}:review   →   /{project}:analyze   →   spec status: done
```

`/{project}:implement` MUST NOT mark a spec `done` while the target's `review.md`
records `must-violations: > 0`. See **§Blocking semantics** in `.ductus/reference/review.md`.

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write).

**Where a named section lives.** This command's markdown-only reference — the procedure for the path without the runtime, and the policy these steps apply — is `.ductus/reference/review.md`. A section named here, or cited elsewhere as part of this command, that has no heading in this file is there; read it when a step points at it.

Run once per targeted feature (every in-progress or done spec under `--all`, otherwise the current `/{project}:target`), in order. Resolve a `[feature]` argument through `resolve-feature` (exact name / number / unique partial slug), and enumerate the `--all` set from `dashboard`'s per-spec status inventory (`specs[].status ∈ {in-progress, done}`) rather than a directory scan. The detailed walk — rule-selection notices, waiver semantics, the report skeleton, and the pass definitions — lives in `.ductus/reference/review.md`.

1. Invoke `compute-review-scope` to resolve the diff base (the **parent** of the commit the spec advanced to in-progress at, so work committed together with an `/{project}:amend` back-edge flip is inside the window rather than excluded from it; `HEAD` when that transition is not committed yet; or a `--since` override, which is used verbatim), and the review file scope (the **union** of the plan's Affected Files and the files modified since the diff base, in the working tree as well as in commits — both sets, because either alone can omit what the review exists to look at). When the scope is empty, skip the passes and waivers and jump to step 9 — an observation the reviewer supplies is still dispositioned, since the reviewer's judgment is the input rather than the diff — and step 11 emits the nothing-to-review-yet, non-blocking report. **A failed call is not an empty scope.** On a wide diff base this result can exceed the host's tool-output cap, which returns an error naming the size and a saved-output path _instead of_ a result; a scope array in the hundreds is ordinary for a spec whose base predates months of history. Re-run the call through the runtime binary with the output redirected to a file and read `diff-base` and the array lengths from there — never route a failed call to the empty-scope branch, because that branch records a non-blocking 0-findings review byte-identical to a genuinely clean one, which is precisely the conflation `examined` and `scope` exist to prevent. Otherwise confirm tech-stack alignment first (host judgment, not a primitive): read the active config file; when its `[review] tech-stack-verified` flag is true, skip the check; else compare the AGENTS.md Tech Stack section against the code in scope, halting with the tech-stack-misalignment message (**§Blocking message** in `.ductus/reference/review.md`) on a mismatch, and — on success — confirm before persisting the flag (the same confirm-before-write gate the other pipeline steps use; see the tech-stack alignment check under **§Resolve target and scope** in `.ductus/reference/review.md`) and write `[review] tech-stack-verified = true` to the **active config file** (the newest existing of `.ductus/config.toml`, `.govern/config.toml`, or the legacy root `.govern.toml`, else `.ductus/config.toml`; specs 042 and 049 — a write outside the `/ductus` migrations never creates a partial `.ductus/config.toml` alongside a lingering older file). Only the flag read is deterministic — the alignment judgment stays with the host.
2. Invoke `discover-rule-files` to select this run's rule files — suffix classification, the `[rules] surfaces` selection, and the disabled-rule-files filter — and emit the ordered notice lines it returns verbatim.
3. <!-- llm:performReview --> Run the **security** pass over the in-scope files against the loaded security rules, returning one finding per violation (rule id, severity, file, line range, confidence, explanation).
4. <!-- llm:performReview --> Run the **reuse** pass: flag logic that duplicates existing utilities or belongs in shared code.
5. <!-- llm:performReview --> Run the **quality** pass: detect bugs, missing error handling, unhandled edge cases, and contract violations; low-confidence findings are recorded separately and do not block.
6. <!-- llm:performReview --> Run the **efficiency** pass: flag N+1 queries, repeated work, and unbounded loops over user-controlled input.
7. <!-- llm:performReview --> Run the **simplicity** pass: flag overengineering, premature abstraction, and dead branches; mark a finding auto-fixable when a simpler form is mechanically derivable. A dimension-restricting flag (`--security` / `--simplicity` / `--quality`) skips the unselected passes.
8. Invoke `process-waivers` to classify the waivers recorded in `review.md` against the findings the passes just accumulated (apply / expire / retain / malformed / duplicate — **§Per-run waiver processing** in `.ductus/reference/review.md` and **§Malformed and duplicate waivers** in `.ductus/reference/review.md`), emitting each notice it returns. **On a dimension-restricted run (`--security` / `--simplicity` / `--quality`), pass the skipped dimensions as `skipped-passes`** so a waiver whose rule did not fire is _retained_, not expired — the partial run cannot see the dimensions it didn't run, so it must not prune their waivers. The applied set is excluded from the blocking count; the expired set is dropped on the next write; the retained set is left in the frontmatter untouched. On an unrestricted run `skipped-passes` is empty and a waiver expires only when its file is gone or its rule genuinely no longer fires.
<!-- audit:ignore-promotion -->
9. Process stored decisions (host responsibility): read the `decisions:` list in `review.md` and match each observation the passes returned to the stored decision describing the same issue — observation text is the reviewer's own wording, so the match is the host's judgment, not a byte comparison. Then call the process-decisions primitive with the feature, `record: review`, `fired` — for each observation, the stored key it matched, or else its rendered line (its text, then an em dash and its path in backticks) — and `restricted` set when any pass did not run (a dimension-restricting flag, or an empty scope that skipped them all), so a decision whose observation a skipped pass would have produced is retained rather than expired. Each `matched` decision disposes of its observation under the stored outcome, with nothing asked, and the observation carries that key to step 11 as its `decision-key`; each `expired` decision goes to step 11 to be dropped; `retained` decisions stay untouched. Emit the result's `notices` verbatim. When the primitive refuses because the list does not parse, report that, disposition each observation as though nothing were stored, and tell the operator the record cannot be written until the list is repaired.

<!-- audit:ignore-promotion -->
10. Fix and route (host responsibility): for each observation no stored decision matched, propose one disposition, and write it only after the gate-confirm primitive returns a confirmed decision — a chore fixed in the run; a route to the home the **§Groom decision tree** in `.ductus/reference/groom.md` chooses (its single canonical statement; do not restate it here), where a route to a `done` spec names the `done → in-progress` reopen in its confirmation and performs it through the set-status primitive with `from: done`; or a discard with the operator's reason. When a chore fix wrote, re-run the affected passes before step 11, as `--fix` does, so the record describes the fixed tree. A declined proposal leaves the observation discarded with the operator's reason, or undispositioned; a fix that fails or proves not to be mechanical is reverted and re-proposed as a route or a discard; a confirmed route whose write fails is undispositioned, and the failure is reported beside the observation. **With no operator to confirm** — `ductus exec`, whose walker no-ops this step by design, or any host that cannot ask — propose nothing and write nothing: each unmatched observation is recorded undispositioned. MUST and SHOULD violations are not dispositioned here; they keep their fix-or-waive model.
    The detail is under **Observations** in **§Write `review.md`** in `.ductus/reference/review.md`.

11. Invoke `write-review` with the accumulated pass findings, the accumulated pass **observations** — each with its `disposition` (`outcome`, plus `target` for a route or `reason` for a discard) and, when step 9 matched it, its `decision-key` — the waiver results (`applied` / `expired`), `expired-decisions` from step 9, and the scope to render `specs/NNN-feature/review.md` — record and report together in one file. Supply the required scalars the primitives don't produce — `reviewed-at` (the current UTC timestamp) and `reviewed-against` (HEAD sha), both host-provided (as the session-write's `set-at` is); `diff-base` comes from step 1; **`examined`, how many of the in-scope files the passes above actually read**; and `decided-by` (`git config user.email`), which the primitive requires when any observation is newly routed or discarded. The primitive resolves the scope itself and records it as `scope`, so `examined` is a numerator against a denominator no caller supplies. It applies the cross-pass dedup (highest-severity-wins on rule + file + overlapping range), buckets findings into MUST / SHOULD / low-confidence / waived, prunes expired waivers and expired decisions (preserving any adopter-authored fields on the survivors), records the skipped passes, renders each observation with its disposition, derives the `dispositions:` map and the `decisions:` list — storing each newly routed or discarded observation under its key, while a re-matched decision keeps its original stamp — and sets blocking when MUST violations remain. Nothing is written to `specs/inbox.md`. With `--fix`, apply the auto-fixable findings (**§Apply `--fix` (optional)** in `.ductus/reference/review.md`, within **§Auto-fix scope** in `.ductus/reference/review.md`), re-run the affected passes, and invoke `write-review` a second time for the post-fix counts. The result also carries `dispositions`, which you render as the `observations` row, and `analyze-freshness`, the state of the feature's `analysis.md` record — its recorded digest compared against the spec's analyze subjects as they are now — which you render as the `analyze` row, both described under **§The `observations` row** in `.ductus/reference/review.md` and **§The `analyze` row** in `.ductus/reference/review.md`. Neither affects the exit code.
