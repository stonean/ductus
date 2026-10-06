---
description: Audit code against rules — security, reuse, quality, efficiency, simplicity. Writes review.md; blocks done on MUST violations.
argument-hint: "[--all] [--fix] [--security|--simplicity|--quality] [--since=<ref>] [--waive <rule-id> --reason <text>] [feature]"
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

Quality gate before `done`: audit the feature's implementation against the project's rule files across five dimensions (security, reuse, quality, efficiency, simplicity), record the findings in `specs/NNN/review.md` — each observation with its disposition — and set `blocking` in that file's own frontmatter record so `/ductus:implement`, `/ductus:analyze`, and the CI hook can hold the spec out of `done` while MUST violations stand. Waivers (with recorded justification) are the sanctioned escape.

## Context

Use the session target: invoke `resolve-session` and display any notices and unreadable session files it returns (on the markdown-only path, read the shared default `.ductus/session.toml` instead — §concurrent-features). If `$ARGUMENTS` carries a feature identifier, use it to override the session target — resolve that override through `resolve-feature` (exact directory name, feature number, or unique partial slug; `ambiguous` and `not-found` are domain outcomes to surface). With `--all` the target is every spec at `in-progress` or `done` and a feature identifier is redundant; report it rather than silently ignoring it. If no session target is set and no feature argument is provided, stop and tell the user to run `/ductus:target` first.

### Parsing `$ARGUMENTS`

Parse `$ARGUMENTS` for flags in any position, then treat the remaining text as the optional feature identifier. Every flag is per-invocation and none is persisted to the session file — what to review is an execution-time decision, not session state. [Flags](#flags) below is the authoritative table of what each one _does_; this step is how each is _recognized_:

- **`--all`**, **`--fix`**, **`--security`**, **`--simplicity`**, **`--quality`** — bare toggles, no value.
- **`--since=<ref>`** — takes its value in `--since=<ref>` form; pass it to `compute-review-scope` as `since` (step 1). A bare `--since` with no value is an operator error: report it and stop. Do not fall back to the default diff base — a silent fallback reviews a different window than the one asked for, and reports it under a heading that claims otherwise.
- **`--waive <rule-id> --reason "<text>"`** — a pair; each is an operator error without the other. Repeatable to waive more than one finding in a single invocation.

Report an unrecognized `--flag` and stop. Never absorb it into the feature identifier: treating `--sinse=HEAD~5` as a feature name resolves to `not-found` at best, and at worst to a real feature whose review then silently covers the wrong scope.

## Scope Boundaries

- Reads the target spec, its `plan.md` (for Affected Files), the in-scope source files, the selected rule files, `AGENTS.md`, `.ductus/config.toml`, and the `decisions:` list in its own `review.md`. The fix-and-route step (step 10) additionally reads what a disposition needs: a candidate covering spec's `spec.md` and `tasks.md` for the Groom decision tree, as `/ductus:groom` does. Do NOT review files outside the resolved scope, and do NOT introduce review criteria from outside the project's rule files and `AGENTS.md`.
- Writes `specs/NNN/review.md` — the whole record, in that file's own frontmatter (via `write-review`). `spec.md` is **not** written: the record has one home (spec 057); with `--waive`, appends a waiver entry; with `--fix`, applies auto-fixable findings to the working tree; and the fix-and-route step (step 10) writes each observation's confirmed disposition — a chore fix, or a route to the home the Groom decision tree chooses, including a `done → in-progress` reopen of the spec routed to — only after `gate-confirm` returns a confirmed decision. No other files are modified, and nothing is written to `specs/inbox.md` — the reviewed spec's own status transitions belong to `/ductus:implement`.
- Reference: §runtime-host-integration, §bug-handling, §brownfield-inbox (Finding dispositions), §text-first-artifacts, §spec-phase (spec-root resolution) (constitution loaded by `/ductus:target` — do not re-read).

## Inputs

- **Target** — the current `/ductus:target` feature, or every feature with
  status `in-progress` or `done` when invoked with `--all`.
- **Rules** — every file under the project's rule-file directory
  (`framework/rules/` in ductus's own repo, `specs/rules/` in adopter
  projects) selected by the suffix-based discovery in step 2
  (`discover-rule-files`); their content is the authoritative review
  criteria. RFC 2119 language is authoritative:
  **MUST/MUST NOT** are blocking violations, **SHOULD/SHOULD NOT** are
  advisory.
- **Scope** — files referenced by the target's `plan.md` under `Affected Files`,
  unioned with any files modified since the spec advanced to `in-progress` —
  see step 1 (`compute-review-scope`).
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
    audit trail for the override.
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
| `--fix` | Apply auto-fixable findings (see [Auto-fix scope](#auto-fix-scope) below) |
| `--since=<ref>` | Override the diff base (default: the parent of the commit at which the spec advanced to `in-progress`, so work committed with the transition is inside the window) |
| `--waive <rule-id> --reason "<text>"` | Record a waiver for a MUST violation (see [Waivers](#waivers)) |

## Pipeline position

`/ductus:review` runs after `/ductus:implement` has produced code and before the spec
can advance to `done`. The recommended sequence is:

```text
/ductus:implement   →   /ductus:review   →   /ductus:analyze   →   spec status: done
```

`/ductus:implement` MUST NOT mark a spec `done` while the target's `review.md`
records `must-violations: > 0`. See [Blocking semantics](#blocking-semantics).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write).

Run once per targeted feature (every in-progress or done spec under `--all`, otherwise the current `/ductus:target`), in order. Resolve a `[feature]` argument through `resolve-feature` (exact name / number / unique partial slug), and enumerate the `--all` set from `dashboard`'s per-spec status inventory (`specs[].status ∈ {in-progress, done}`) rather than a directory scan. The detailed walk — rule-selection notices, waiver semantics, the report skeleton, and the pass definitions — lives under the Markdown-only reference section below.

1. Invoke `compute-review-scope` to resolve the diff base (the **parent** of the commit the spec advanced to in-progress at, so work committed together with an `/ductus:amend` back-edge flip is inside the window rather than excluded from it; or a `--since` override, which is used verbatim), and the review file scope (the **union** of the plan's Affected Files and the files modified since the diff base — both sets, because either alone can omit what the review exists to look at). When the scope is empty, skip the passes and waivers and jump to step 9 — an observation the reviewer supplies is still dispositioned, since the reviewer's judgment is the input rather than the diff — and step 11 emits the nothing-to-review-yet, non-blocking report. **A failed call is not an empty scope.** On a wide diff base this result can exceed the host's tool-output cap, which returns an error naming the size and a saved-output path _instead of_ a result; a scope array in the hundreds is ordinary for a spec whose base predates months of history. Re-run the call through the runtime binary with the output redirected to a file and read `diff-base` and the array lengths from there — never route a failed call to the empty-scope branch, because that branch records a non-blocking 0-findings review byte-identical to a genuinely clean one, which is precisely the conflation `examined` and `scope` exist to prevent. Otherwise confirm tech-stack alignment first (host judgment, not a primitive): read the active config file; when its `[review] tech-stack-verified` flag is true, skip the check; else compare the AGENTS.md Tech Stack section against the code in scope, halting with the tech-stack-misalignment message on a mismatch, and — on success — confirm before persisting the flag (the same confirm-before-write gate the other pipeline steps use; see the tech-stack alignment step in the markdown-only reference) and write `[review] tech-stack-verified = true` to the **active config file** (the newest existing of `.ductus/config.toml`, `.govern/config.toml`, or the legacy root `.govern.toml`, else `.ductus/config.toml`; specs 042 and 049 — a write outside the `/ductus` migrations never creates a partial `.ductus/config.toml` alongside a lingering older file). Only the flag read is deterministic — the alignment judgment stays with the host.
2. Invoke `discover-rule-files` to select this run's rule files — suffix classification, the `[rules] surfaces` selection, and the disabled-rule-files filter — and emit the ordered notice lines it returns verbatim.
3. <!-- llm:performReview --> Run the **security** pass over the in-scope files against the loaded security rules, returning one finding per violation (rule id, severity, file, line range, confidence, explanation).
4. <!-- llm:performReview --> Run the **reuse** pass: flag logic that duplicates existing utilities or belongs in shared code.
5. <!-- llm:performReview --> Run the **quality** pass: detect bugs, missing error handling, unhandled edge cases, and contract violations; low-confidence findings are recorded separately and do not block.
6. <!-- llm:performReview --> Run the **efficiency** pass: flag N+1 queries, repeated work, and unbounded loops over user-controlled input.
7. <!-- llm:performReview --> Run the **simplicity** pass: flag overengineering, premature abstraction, and dead branches; mark a finding auto-fixable when a simpler form is mechanically derivable. A dimension-restricting flag (`--security` / `--simplicity` / `--quality`) skips the unselected passes.
8. Invoke `process-waivers` to classify the waivers recorded in `review.md` against the findings the passes just accumulated (apply / expire / retain / malformed / duplicate), emitting each notice it returns. **On a dimension-restricted run (`--security` / `--simplicity` / `--quality`), pass the skipped dimensions as `skipped-passes`** so a waiver whose rule did not fire is _retained_, not expired — the partial run cannot see the dimensions it didn't run, so it must not prune their waivers. The applied set is excluded from the blocking count; the expired set is dropped on the next write; the retained set is left in the frontmatter untouched. On an unrestricted run `skipped-passes` is empty and a waiver expires only when its file is gone or its rule genuinely no longer fires.
<!-- audit:ignore-promotion -->
9. Process stored decisions (host responsibility): read the `decisions:` list in `review.md` and match each observation the passes returned to the stored decision describing the same issue — observation text is the reviewer's own wording, so the match is the host's judgment, not a byte comparison. Then call the process-decisions primitive with the feature, `record: review`, `fired` — for each observation, the stored key it matched, or else its rendered line (its text, then an em dash and its path in backticks) — and `restricted` set when any pass did not run (a dimension-restricting flag, or an empty scope that skipped them all), so a decision whose observation a skipped pass would have produced is retained rather than expired. Each `matched` decision disposes of its observation under the stored outcome, with nothing asked, and the observation carries that key to step 11 as its `decision-key`; each `expired` decision goes to step 11 to be dropped; `retained` decisions stay untouched. Emit the result's `notices` verbatim. When the primitive refuses because the list does not parse, report that, disposition each observation as though nothing were stored, and tell the operator the record cannot be written until the list is repaired.

<!-- audit:ignore-promotion -->
10. Fix and route (host responsibility): for each observation no stored decision matched, propose one disposition, and write it only after the gate-confirm primitive returns a confirmed decision — a chore fixed in the run; a route to the home the **Groom decision tree** in `groom.md` chooses (its single canonical statement; do not restate it here), where a route to a `done` spec names the `done → in-progress` reopen in its confirmation and performs it through the set-status primitive with `from: done`; or a discard with the operator's reason. When a chore fix wrote, re-run the affected passes before step 11, as `--fix` does, so the record describes the fixed tree. A declined proposal leaves the observation discarded with the operator's reason, or undispositioned; a fix that fails or proves not to be mechanical is reverted and re-proposed as a route or a discard; a confirmed route whose write fails is undispositioned, and the failure is reported beside the observation. **With no operator to confirm** — `ductus exec`, whose walker no-ops this step by design, or any host that cannot ask — propose nothing and write nothing: each unmatched observation is recorded undispositioned. MUST and SHOULD violations are not dispositioned here; they keep their fix-or-waive model. The detail is under **Observations** in the markdown-only reference below.

11. Invoke `write-review` with the accumulated pass findings, the accumulated pass **observations** — each with its `disposition` (`outcome`, plus `target` for a route or `reason` for a discard) and, when step 9 matched it, its `decision-key` — the waiver results (`applied` / `expired`), `expired-decisions` from step 9, and the scope to render `specs/NNN-feature/review.md` — record and report together in one file. Supply the required scalars the primitives don't produce — `reviewed-at` (the current UTC timestamp) and `reviewed-against` (HEAD sha), both host-provided (as the session-write's `set-at` is); `diff-base` comes from step 1; **`examined`, how many of the in-scope files the passes above actually read**; and `decided-by` (`git config user.email`), which the primitive requires when any observation is newly routed or discarded. The primitive resolves the scope itself and records it as `scope`, so `examined` is a numerator against a denominator no caller supplies. It applies the cross-pass dedup (highest-severity-wins on rule + file + overlapping range), buckets findings into MUST / SHOULD / low-confidence / waived, prunes expired waivers and expired decisions (preserving any adopter-authored fields on the survivors), records the skipped passes, renders each observation with its disposition, derives the `dispositions:` map and the `decisions:` list — storing each newly routed or discarded observation under its key, while a re-matched decision keeps its original stamp — and sets blocking when MUST violations remain. Nothing is written to `specs/inbox.md`. With `--fix`, apply the auto-fixable findings, re-run the affected passes, and invoke `write-review` a second time for the post-fix counts. The result also carries `dispositions`, which you render as the `observations` row, and `analyze-freshness`, the state of the feature's `analysis.md` record — its recorded digest compared against the spec's analyze subjects as they are now — which you render as the `analyze` row, both described under [Output](#output). Neither affects the exit code.

## Markdown-only reference

The numbered Instructions above are the deterministic path — the runtime's primitives own the rule-file selection, waiver arithmetic, scope resolution, and report scaffolding, and the five passes cross the boundary at the `performReview` extension point. When no runtime is available, walk the detailed procedure below by hand, for each targeted feature, in order.

### 1. Resolve target and scope

1. Resolve the working feature from `--all` or the current `/ductus:target`.
   If neither yields a target, halt with `no target — run /ductus:target first`.
2. Read the spec frontmatter. If `status` is not in `{in-progress, done}`,
   halt with `review only runs against in-progress or done specs`.
3. Build the file scope per [Inputs](#inputs). If the resolved scope is
   empty (no implementation files yet), write a `review.md` recording 0
   findings across all five passes, `blocking: false`, and exit `0` — there
   is nothing to review yet. Skip steps 4–5 and the rest of this run, except
   that an observation the reviewer supplies is still dispositioned, as
   **Observations** under [4. Write `review.md`](#4-write-reviewmd) describes,
   before the record is written: the reviewer's judgment is the input, not
   the diff.
   **Distinguish an empty scope from a scope you could not read.** A tool
   that returns an error, a size-cap notice, or a saved-output pointer has
   told you nothing about the scope; resolve it by another route and read
   it in full. Recording the 0-findings report there would state that the
   passes examined an empty scope when they examined nothing at all.
4. **Tech-stack alignment check.**
   - Read the active config file (resolution rule in Instructions step 1;
     spec 042). If `[review] tech-stack-verified = true`, skip to
     step 5.
   - Otherwise, read `AGENTS.md`'s `Tech Stack` section and inspect the file
     scope (extensions, imports, runtime/manifest markers). Confirm the
     documented stack appears consistent with the implementation. A
     missing or empty `Tech Stack` section, or an inconsistency between
     documentation and code, halts the run with the
     [tech-stack-misalignment](#blocking-message) message and exits `1`.
   - On a successful check, prompt the operator once (routing the prompt
     through `gate-confirm` on the runtime path, as the other
     confirm-before-write pipeline steps do): _"Tech-stack alignment
     confirmed. Persist this so future runs skip the check? (Y/n)"_. On
     `Y`, write `[review] tech-stack-verified = true` to the active config
     file (same resolution as Instructions step 1). On `n` or skip, the check runs again on the next
     invocation. To re-run the check after a stack change, the operator
     removes the line manually — `/ductus:review` does not auto-reset.
5. Discover rule files by suffix. List `framework/rules/*.md` in ductus's
   own repository, or `specs/rules/*.md` in adopter projects. For each
   file, classify by basename suffix:
   - `*-backend.md` → backend surface
   - `*-frontend.md` → frontend surface
   - `*-cross.md` → cross-cutting (applies to every stack)
   - anything else → unrecognized — load for every stack and emit one
     stdout line per file:

     ```text
     rule file <name> has unrecognized suffix — loading for all stacks; rename to -backend.md, -frontend.md, or -cross.md
     ```

   Determine the **surface selection** for this run. Read `.ductus/config.toml`
   `[rules] surfaces` (see [Inputs](#inputs)):

   - **Set to a valid list** (every member in `{backend, frontend}`) —
     keep the rule files whose surface is listed in `surfaces`, plus
     every `*-cross.md`. This explicit operator-set selection _replaces_
     the detected-stack filter; the stack from step 4 is not consulted
     for rule-file selection. The **empty list** (`surfaces = []`) is a
     valid value of this case and means **cross-only**: no
     `*-backend.md`/`*-frontend.md` file is kept, only `*-cross.md`. The
     empty list is distinct from the key being unset (below) — it is the
     operator explicitly declaring "no surface rules, only cross-cutting
     ones," not a request to derive.
   - **Set to a degenerate value** — fail fast (do not silently ignore,
     do not warn-and-continue), consistent with `CFG-ENV-003`'s
     fail-fast-on-invalid-configuration posture:
     - **Unrecognized member.** A member outside `{backend, frontend}` —
       a typo like `"fullstack"`, or `"cross"` (cross-cutting files are
       unconditional, not a selectable surface) — halts the run with
       `/ductus:review: invalid [rules] surfaces member "<value>" — accepted members are "backend" and "frontend" (use [] for cross-only; -cross.md files always apply)`.
       A list mixing valid and invalid members (`["backend", "fullstack"]`)
       fails on the invalid member; a valid member does not rescue it.
     - **Type mismatch.** A non-list value (`surfaces = "backend"`, a
       bare string) halts the run with
       `/ductus:review: [rules] surfaces must be a list of strings, got <type>`.
   - **Unset** — fall back to the detected stack from step 4: keep the
     matching surface, keep every `*-cross.md` (pre-033 behavior).

   In every non-error case, keep every unrecognized-suffix file
   unconditionally.

   Then apply the **disabled-rule-files filter**. Read `.ductus/config.toml`
   `[[review.disabled-rule-files]]` (see [Inputs](#inputs)). For each
   entry, in list order:

   - **Drop + notice (selected match).** `file` matches the
     basename of a file currently in the post-selection set. Remove
     it from the set and emit one line:

     ```text
     disabled-rule-file: <filename> — <reason> (<config-file>)
     ```

     `<config-file>` is the repo-relative resolved config file the
     disable came from — `.ductus/config.toml`, or `.govern/config.toml`
     / the legacy root `.govern.toml` on a pre-migration layout. The
     ladder is three tiers, not two (`schema/paths.rs`'s `CONFIG_CHAIN`);
     `status.md` states the same set in the same sentence shape. Collapse internal
     whitespace in `reason` (including newlines from TOML multi-line
     strings) to single spaces before emitting — the notice is
     single-line by contract.

   - **No-op notice (non-selected match).** `file` matches a
     basename in the rule-file directory but the file was NOT in the
     post-selection set (different surface). Emit one line and
     change nothing:

     ```text
     disabled-rule-file (no-op): <filename> not selected by stack detection
     ```

     This is honest about state — the entry is currently a no-op,
     becomes load-bearing if the project's stack changes later.

   - **Unknown warning.** `file` does not match any basename in the
     rule-file directory. Emit one line and change nothing:

     ```text
     unknown disabled-rule-file: <filename> (no such file in the rule-file directory)
     ```

     This covers renamed/moved files; not a fatal error.

   - **Malformed warning.** Entry is missing `file` or `reason`, or
     `reason`'s trimmed length is < 16 Unicode codepoints. Skip the
     entry (no file is dropped) and emit one line naming the offending
     index (same pattern as **Malformed and duplicate waivers** below):

     ```text
     malformed disabled-rule-file at review.disabled-rule-files[N]: <reason>
     ```

     The entry is NOT auto-removed; the operator cleans it up.

   - **Duplicate warning.** Same `file` listed twice. Only the first
     entry applies; each subsequent duplicate emits one line and is
     not auto-pruned:

     ```text
     duplicate disabled-rule-file: <filename> — entry [N] ignored
     ```

   All four warning forms emit to stdout and **do not affect the exit
   code**. `/ductus:review`'s exit status is driven exclusively by MUST
   violations (see [Output](#output)). `.ductus/config.toml` hygiene is a
   separate concern.

   Finally, emit a single stdout line naming what was selected:

   ```text
   loading rule files: <comma-separated basenames>
   ```

   Disabled files are excluded from this list. The notice fires AFTER
   all disabled-rule-file lines, so a normal run reads top-down as:
   any `disabled-rule-file: …` notices, then `loading rule files: …`.
   This is the discoverability surface — adopters can confirm which
   files were considered without parsing the report.

### 2. Load rules

Load these inputs inline as the authoritative review criteria:

- Every rule file selected by the suffix-based discovery in step 5
- Any rule file outside the rule-file directory (e.g., `docs/rules/internal-api.md`)
  referenced from `AGENTS.md` — see [Notes for adopters](#notes-for-adopters)
- `AGENTS.md` `Code Style`, `Testing`, `Gotchas`, and `Boundaries` sections
- The target spec's acceptance criteria and any `scenarios/*.md` files

These are the **only** sources of normative rules for the review. Do not
introduce review criteria from outside the project.

### 3. Run review passes

Run the five passes below. When a flag restricts dimensions (`--security`,
`--simplicity`, `--quality`), skip the unselected passes and record them as
`skipped` in the report.

When the same finding (matching rule ID, file, and overlapping line range)
is produced by more than one pass, retain only the highest-severity instance
in `must-violations` and `should-violations`; lower-severity duplicates are
dropped from the counts and the report. Pass-of-record for the surviving
finding is the highest-severity pass that flagged it.

#### Security pass

Walk every file in scope against the loaded security rules. For each finding,
record: rule ID, severity (MUST or SHOULD), file path, line range, the rule
text, and a one-sentence explanation of how the code violates it. **Do not
flag patterns that are not in the loaded rules** — the project's rule set is
authoritative.

That rule holds for every pass below, and it is not a reason to drop what you
noticed: anything real that matches no loaded rule is an **observation**, and
every pass may return them alongside its findings — see the Observations
rules under [4. Write `review.md`](#4-write-reviewmd). Recording one puts it
in front of the fix-and-route step, so there is nothing separate to remember.

#### Reuse pass

Identify logic that duplicates existing utilities or that should be extracted
into shared code. Cross-reference with `specs/system.md` for established
patterns and shared infrastructure. Severity is SHOULD unless the duplication
contradicts an explicit MUST in `AGENTS.md` `Boundaries`.

#### Quality pass

Detect bugs, missing error handling, unhandled edge cases, off-by-one errors,
and contract violations against `specs/errors.md`. Each finding carries a
`confidence` tier — `high` or `low` (the string the `write-review` contract
consumes, compared case-insensitively). A `low`-confidence finding is recorded
in the Low-confidence section regardless of severity and is excluded from the
blocking count; use it when the finding is plausible but unconfirmed.

#### Efficiency pass

Flag N+1 queries, repeated work, unbounded loops over user-controlled input,
and other performance issues. Severity is SHOULD by default; promote to MUST
when the inefficiency is also a security concern (e.g. unbounded input is a
DoS vector covered by the security rules).

#### Simplicity pass

Identify overengineering: premature abstraction, unnecessary indirection,
configuration that could be a constant, branches that are dead under the
current spec. Severity is SHOULD. If a simpler form is mechanically derivable,
mark the finding `auto-fixable`.

### 4. Write `review.md`

Write the report to `specs/NNN-feature/review.md`. A scenario-targeted run still writes to the same spec-level path; the `scenario:` frontmatter field records which scenario was reviewed and `reviewed-against` records the commit. Re-running review (scenario- or feature-targeted) supersedes the prior `review.md` wholesale.

```markdown
---
spec: 042-example-feature
last-run: 2026-05-10T14:32:00Z
reviewed-against: <sha-of-HEAD>
diff-base: <sha of the parent of the in-progress transition commit>
must-violations: 0
should-violations: 3
low-confidence: 2
examined: 38
scope: 46
skipped-passes: []
reviewed-digest:
  scenarios/retry.md: 3f2a…
blocking: false
dispositions:
  fixed: 1
  routed: 1
  discarded: 1
  undispositioned: 0
---

# Review — 042-example-feature

## Summary

<one paragraph: overall posture, count by severity, blocking status>

## MUST violations (blocking)

<one heading per finding; `*None.*` when the section is empty>

## SHOULD violations (advisory)

## Low-confidence findings

## Waived findings

## Observations

<one bullet per observation this run recorded, each with its disposition; `*None.*` when empty>

## Skipped passes

<`*None.*` when none>

## Unexamined governance

<one bullet per registered shared constitution this run could not read; `*None.*` when none>
```

**Unexamined governance** names any `[constitutions.*]` entry (spec 055) the run could not read, with its reason — `not checked out`, or `no constitution.md in checkout`. `write-review` resolves the registry itself rather than taking it as an argument, so a report cannot omit the section: a review that ran under fewer rules than the project's config declares says so, instead of letting the finding counts stand in for a clean result. That is `QUAL-CLAIM-001` applied to the review's own inputs. `*None.*` covers both "none registered" and "all registered sources read" — from the report's side those are the same claim, because in neither case did anything go unexamined.

Each bullet carries the entry's `description` when it has one, between the path and the reason: `` - `acme` (../governance) — Acme platform engineering rules — not checked out — its rules were NOT loaded for this review ``. An alias alone is a config key someone chose, so the description is what tells an operator _which_ checkout they are missing rather than sending them to look the alias up. It renders here precisely because the document could not be read — the value comes from the config, not the checkout, so it is available when the document is not. An entry with no description renders exactly as it did before. The line is single-line by contract, so an over-long description is truncated with `…` rather than wrapped, and internal whitespace is collapsed before it arrives (a TOML multi-line string makes an embedded newline reachable).

**`examined` and `scope` are what make a clean report mean something.** The counts alone cannot distinguish _the passes read the scope and found nothing_ from _the passes never ran_ — both write `0/0/0`, the same `reviewed-digest`, and `blocking: false`. `write-analysis` has required `unexamined` since spec 047 for exactly this reason, and the review half carried no equivalent until a review was recorded over a scope nothing had read. `scope` is derived by the primitive, so the denominator cannot be shrunk to match whatever was read; `examined` is the reviewer's claim about the numerator, and an **unstated** one is recorded as absent rather than as zero — a claim never made and a claim that came back empty are different facts. No check reads the pair: Family 31 did, and was retired with the record's second home (spec 057), so these are read by a person at the completion gate. Nothing could prove the passes ran anyway — an overstated numerator is as available as an omitted one — so what this buys is that the claim is explicit and re-derivable instead of invisible.

Every empty section renders the literal `*None.*` line — the `write-review` primitive emits it, and the markdown-only path writes the same so the two paths produce byte-identical reports. The **Observations** heading carries no suffix.

The **Observations** section is the home for something the reviewer judged
real that maps to **no loaded rule**. Inventing a rule for it is not the answer
— the rule set is authoritative — and the free-text Summary is not either,
because `write-review` regenerates the Summary wholesale on the next run and it
is per-spec, so a cross-cutting note filed there is both erased and misfiled.
An observation is **not** a finding: it never enters the
`must-violations` / `should-violations` / `low-confidence` counts, never
affects `review.blocking`, and never changes the exit code. Each entry carries
its own one-line text and, optionally, the path it anchors to. Leading the text
with a category (`security` / `leak` / `convention` / `bug` / `perf` / `other`)
helps a reader route it, but nothing parses it.

**Every observation gets a disposition before the record is written**
(§brownfield-inbox, Finding dispositions), and the section renders it beside
the observation, the path clause omitted when the observation has none:

```text
- {text} — `{path}` — **fixed**
- {text} — `{path}` — **routed** to `{target}`
- {text} — `{path}` — **discarded**: {reason}
- {text} — `{path}` — **undispositioned**
```

- **Fixed** — a chore: mechanical, and adding no durable requirement
  (§bug-handling's durability test). It is confirmed before it writes, and
  the affected passes re-run before the record is written, as `--fix` does,
  so the record describes the fixed tree. A fix that fails, or turns out not
  to be mechanical, is reverted and is not a chore; a chore is never counted
  fixed while its fix is not on disk.
- **Routed** — written to the home the **Groom decision tree** in `groom.md`
  chooses; that is its single canonical statement, and this reference does
  not restate it. The homes are a task, a scenario with its task, or a body
  edit on this spec; a scenario or body edit on another existing spec; a new
  spec, created through `/ductus:specify`'s procedure in the same run; or
  an amendment to a rule file the project owns. A route to a `done` spec names
  the `done → in-progress` reopen in its confirmation and performs it with
  `from: done`, so a status that changed since the proposal surfaces rather
  than being overwritten. A declined spec creation leaves the observation
  discarded or undispositioned, never routed to a spec that does not exist,
  and a confirmed route whose write fails is undispositioned, with the failure
  named beside it.
- **Discarded** — with the reason it is out of scope. It is the disposition
  that ends a loop: an observation about the pipeline's own machinery rather
  than the code under review, routed to a `done` spec, reopens that spec, and
  that spec's next review produces the next observation.
- **Undispositioned** — nobody decided: the operator declined without a
  reason, a route's write failed, or nobody could be asked. `ductus exec`'s
  walker no-ops steps 9 and 10, so every observation an exec run threads
  through is recorded undispositioned. The pre-done gate holds `done` while
  `dispositions.undispositioned` is above zero, until a later review decides
  each one.

**Nothing reaches the inbox.** Observations were once written through to
`specs/inbox.md`, the one destination no gate reads, so a spec could reach
`done` while they waited there unseen. An observation supplied to a run whose
scope is empty is still dispositioned — the reviewer's judgment is the input,
not the diff.

**Decisions persist.** Each routed or discarded observation is stored in
`review.md`'s `decisions:` list, beside `waivers:`, keyed on its rendered line
— the text, then an em dash and the path in backticks — with its outcome, its
target or reason, `decided-at`, and `decided-by`. Observation text is the
reviewer's own wording and does not reproduce byte for byte, so the next run
matches each new observation to the stored decision describing the same issue
and passes that decision's key as the observation's `decision-key`; a matched
observation counts under its stored outcome and is not asked about again. A
stored decision nothing matched is pruned on an unrestricted run and retained
when a pass did not run, exactly as a waiver is. A missed match costs one
repeated question, never a silent waiver, because an unmatched decision is
pruned rather than applied to something else. A malformed entry is reported,
applies to nothing, and is kept; an entry with no fields at all holds no
decision, so a re-render drops it. A duplicate key is reported and ignored,
and goes when its key expires or is given a new decision, because a new
decision replaces every stored entry for its key. Waivers have their own
rules, under [Malformed and duplicate waivers](#malformed-and-duplicate-waivers). A
`decisions:` list that does not parse is reported by `validate-frontmatter`,
and `write-review` refuses to write over it rather than read it as empty.

An observation whose subject later becomes a real rule finding needs nothing
special — the finding counts, and the observation is the reviewer's to drop on
the next run.

The finding sections reconcile the same way. A SHOULD or low-confidence entry
whose disposition is "keep as-is" belongs under **Waived findings** with its
rationale, not under its original heading — an accepted trade-off left filed
as a violation is indistinguishable from unfinished work. An entry fixed after
the report was written keeps its place, gains a **Status** line naming the
commit or scenario that closed it, and drops out of the frontmatter count. The
counts state what is _outstanding_, so they and the body must agree.
Observation dispositions are recorded in `dispositions:`, which `write-review`
derives from the observations — there is no need, and no room, for a field of
your own: `write-review` emits a fixed field set and would drop it on the next
run.

Each finding follows this shape:

```markdown
### MUST: <rule-id> — <one-line summary>

- **File**: `path/to/file.ts:42-55`
- **Rule**: <verbatim rule text from the rule file (framework/rules/... or specs/rules/...)>
- **Finding**: <one to three sentences>
- **Auto-fixable**: yes | no
- **Suggested fix**: <code block or prose>
```

The report is regenerated on every `/ductus:review` run — never appended.
Findings the user has explicitly waived (see [Waivers](#waivers)) carry across
runs as long as their anchor (rule + file) is still valid.

### 5. Apply `--fix` (optional)

When `--fix` is set, after writing the report:

1. Apply every finding marked `auto-fixable: yes` whose severity is SHOULD,
   plus MUST findings whose suggested fix is purely mechanical (e.g. removing
   a hardcoded secret, adding a missing CSRF token attribute).
2. **Never** auto-apply fixes that alter externally observable behavior, change
   error messages or status codes, or modify schema. These require a manual pass.
3. Re-run only the affected passes against the modified files. Update
   `review.md` with the post-fix counts.
4. Stage the modified files but do not commit. The user owns the commit.

### 6. Record the run in `review.md`

The record is `review.md`'s own frontmatter, written in the same pass as the
report body below it. `spec.md` is not touched — one fact, one home
(spec 057):

```yaml
---
spec: 020-code-review
last-run: 2026-05-10T14:32:00Z
reviewed-against: <sha>
diff-base: <sha>
must-violations: 0
should-violations: 3
low-confidence: 2
examined: 38
scope: 46
skipped-passes: []
reviewed-digest:
  scenarios/retry.md: 3f2a…
  data-model.md: 9c1b…
blocking: false
dispositions:
  fixed: 0
  routed: 1
  discarded: 0
  undispositioned: 0
waivers: []
decisions:
  - key: "convention: retry backoff is hard-coded — `src/retry.ts`"
    outcome: routed
    target: specs/020-code-review/scenarios/retry-config.md
    decided-at: 2026-05-10T14:32:00Z
    decided-by: dev@example.com
---
```

The timestamp is spelled `last-run`, matching the analyze record. It was
`reviewed-at` here and `last-run` in the spec's `review:` block — one instant
under two names, back when the record had two homes, which the reconciliation
check had to key _by meaning rather than by name_ to compare at all. With one home the second spelling had nothing left to
justify it, and the relocation migration folds the pair.

`reviewed-digest` is the record's description of **what this review read** — a
per-path digest of the spec's durable contracts (`scenarios/*.md`,
`data-model.md`) taken from disk. The completion gate's staleness check
compares it against those files as they are now, rather than diffing
`reviewed-against` against `HEAD`: this command reviews the _working tree_
while that field records a _commit_, so a scenario written during the session
came back as a contract that had changed since the review when it had changed
only since the commit the review was labelled with. `reviewed-against` stays as
provenance, read for the mechanical-sweep rename exemption alone.

`review.md` and `spec.md` are deliberately outside the digest. `review.md` is
this command's own output, so counting it would stale every review the instant
it was recorded. `spec.md` is outside for the same reason historically —
`write-review` used to stamp a block into it — and that rationale lapsed when
the record moved; the set is left unchanged regardless, because adding
`spec.md` would stale every review on any spec-body edit, which is a behavior
change rather than a relocation. That is the inverse of the analyze record's
subject set, which _includes_ them, because they are this command's outputs
and that command's inputs. A spec with no scenarios and no data model records
`reviewed-digest: {}` — taken and empty, which reads as current, and is
distinct from an absent digest, which cannot be judged at all.

`blocking: true` when `must-violations > 0`. This is the field other commands
read. (`write-review` writes `last-run`, `reviewed-against`, `must-violations`,
`should-violations`, `low-confidence`, `scope`, `reviewed-digest`, `blocking`,
and `dispositions` on every run; `examined` when the run stated it, and
`reviewed-unreadable` when a contract could not be read; plus the `waivers`
and `decisions` lists when present.)

`dispositions:` counts the run's **observations** — `fixed`, `routed`,
`discarded`, `undispositioned` — never its MUST and SHOULD violations, which
keep their own counts. The map is always written, all four counts, because its
absence has a meaning: a record without it predates dispositions, and the
pre-done gate blocks an `in-progress` spec on it until this command re-runs.
Absence is not zero — reading it as zero would pass exactly the old review
whose observations went to the inbox unseen.

## Blocking semantics

A spec MUST NOT advance from `in-progress` to `done` while its `review.md`
records `blocking: true`. This is enforced as follows:

1. **`/ductus:implement`** — before marking `status: done`, its `check-review-gate`
   runs every check in a fixed order, first failure wins; the full order and
   its message texts are canonical in the pre-done gate step of
   `framework/commands/implement.md`, and only the two review checks are
   restated here. Ahead of them run whether the spec is already `done`, the
   feature directory's markdown lint, unresolved scenario open questions, and
   an undischarged cross-spec impact — any of which halts before the review
   record is consulted. Behind them run the review staleness check, the
   analyze checks, and — last — the two disposition checks: a `review.md`
   or `analysis.md` with no `dispositions:` map, then a record whose
   `undispositioned` is above zero. The two review checks read the
   record from `review.md`: a missing/null `last-run` — or **no `review.md` at
   all**, which is the never-run state — halts with

   ```text
   blocked: spec has not been reviewed — run /ductus:review before completing
   ```

   and only `blocking: true` halts with the MUST-violations message plus
   waive guidance:

   ```text
   blocked: spec has N MUST violation(s) — see specs/NNN-feature/review.md
   resolve the violations and re-run /ductus:review, or waive with /ductus:review --waive
   ```

2. **`/ductus:analyze`** — adds a check to its existing audit: if the spec's
   status is `done` but its `review.md` records `blocking: true` or a missing
   `last-run`, this is a validation failure, and so is a `done` spec whose
   `review.md` records undispositioned observations (disposition drift). Composable with `--fix`:
   `/ductus:analyze --fix` reverts `done` → `in-progress` and emits a notice
   (it never silently downgrades; the notice is the point).

3. **CI hook** — the shipped GHA template at
   `framework/templates/ci/adopter-generators.yml` fails when any spec at
   `status: done` has a record with `blocking: true` or a missing `last-run`,
   in `review.md` or in `analysis.md`. A `done` spec with **no** `review.md`
   (or no `analysis.md`) has never been reviewed (or analyzed) and is exempt —
   matching `/ductus:analyze`'s own grandfather rule — but that exemption is
   **bounded** by a committed high-water mark, because the set cannot
   legitimately grow. Keying it on the absence of a `spec.md` block instead is
   what the template used to do, and the relocation made that predicate true of
   every spec.

This implements the constitution's quality gate via three mutually reinforcing
mechanisms rather than relying on any single one — consistent with the
**Design Principles** rule: never depend on human diligence.

## Blocking message

Emitted by `/ductus:review` when tech-stack alignment fails (missing/empty
`AGENTS.md` `Tech Stack` section, or documented stack inconsistent with
implementation):

```text
blocked: tech-stack alignment failed — AGENTS.md Tech Stack {missing | inconsistent with code in scope}

  expected: <stack inferred from scope, e.g., "TypeScript + React frontend">
  documented: <AGENTS.md Tech Stack contents, or "(empty)">

reconcile AGENTS.md Tech Stack with the implementation, then re-run /ductus:review.
to skip this check on future runs after manual reconciliation, add
[review] tech-stack-verified = true to .ductus/config.toml.
```

## Waivers

A MUST violation can be waived only with explicit, recorded justification:

```text
/ductus:review --waive <rule-id> --reason "<text>"
```

This appends one entry to `review.md`'s frontmatter. Its `file` lists every
in-scope file the rule fires at in this run — a single path when there is one:

```yaml
waivers:
  - rule: SEC-BE-014
    file: src/api/internal.ts
    reason: "Endpoint is internal-only behind mTLS; rule applies to public APIs"
    waived-at: 2026-05-10T14:40:00Z
    waived-by: <git config user.email>
  - rule: SEC-BE-021
    file:
      - src/api/admin.ts
      - src/api/metrics.ts
    reason: "Both routes are bound to the loopback interface"
    waived-at: 2026-05-11T09:15:00Z
    waived-by: <git config user.email>
```

A list is one judgment — one `reason`, `waived-at` and `waived-by` — over
each path it names. A later `--waive` for the same rule records a new entry
rather than appending to an existing one, because an entry's `waived-at` and
`waived-by` attest to one judgment made once. An operator may still merge
entries that share a rule, reason and author by hand; the framework never
merges them itself.

Waived findings drop out of the `must-violations` count (there is no separate
`waived-violations` field; `write-review` reports the waived count
only in its transient result). They appear in `review.md` under the **Waived
findings** section. They survive across `/ductus:review` runs as long as the
rule ID and file location still match; if either changes, the waiver expires
and the finding re-blocks. Line numbers are not part of the waiver anchor —
the contract is `(rule, file)`, so code moving within the file does not
expire the waiver. An entry listing several paths is that many anchors
sharing their other fields: each path applies, expires and is retained on its
own, and is matched literally — a pattern such as `src/**/*.ts` is not
expanded, so it names a path that does not exist and expires as one.

### Per-run waiver processing

On every `/ductus:review` run, after the review passes have produced their
findings (see **Run review passes**) and before counting them into `must-violations`
or writing the record, walk the recorded `waivers` and classify each anchor — each
path an entry lists, under its rule — against those findings. A waiver can only be judged against findings that exist — when
an empty scope skips the passes entirely, leave the waivers untouched; and on
a dimension-restricted run, waivers anchored to skipped dimensions apply
unchanged rather than expiring:

1. **Apply** when the file exists at the anchored path AND the rule still
   fires there. The finding appears under **Waived findings** in
   `review.md` with the waiver's `reason`; it is excluded from
   `must-violations`.
2. **Expire** when either the file no longer exists at the anchored path
   (renamed, deleted, moved) or the rule no longer fires there (offending
   code fixed, rule removed, rule renamed — IDs are permanent per
   `specs/008-security-rules/data-model.md`, so a renamed rule is a
   different rule). On expiry, drop the path from the entry on the next
   frontmatter write — and the entry, when it was the last path — AND emit
   one line on stdout:

   ```text
   waiver expired: rule {rule-id} at {file} ({reason})
   ```

   The notice is the point of the action; expiry MUST NOT be silent. When
   the same rule still fires anywhere in scope after a drop, the finding
   re-counts toward `must-violations` and `review.blocking` flips to
   `true` if it was previously `false`.
3. **Do not extend** a waiver to a file it does not list. If the same rule
   fires at a path other than the waiver's anchors, that is a separate
   finding; the operator records a fresh `--waive` if it is also intentional.

The record writes `file` as a single path when an entry holds one and as a
list when it holds more, so a list pruned to one path reads like any
one-path entry.

### Malformed and duplicate waivers

- A waiver entry missing any of `rule`, `file`, `reason`, `waived-at`, or
  `waived-by` is **skipped** with a one-line warning naming the offending
  entry (e.g. `malformed waiver at review.waivers[2]: missing 'reason'`).
  The entry is kept on the write; the operator must clean it up to silence
  the warning. Malformed entries are operator-authored state, not garbage
  for the framework to collect. A `file` that is an empty list, or a list
  holding a blank path, is missing too. Two exceptions: an entry with no
  fields at all holds nothing to keep, so a re-render drops it; and pruning
  matches on `(rule, file)` alone, so a malformed entry that lists an
  expired waiver's rule and file loses that path, and is dropped when no
  path remains. A `file` holding anything but a path or a list of paths
  fails the `waivers:` parse, as any mistyped field does.
- Two or more claims on the same `(rule, file)` pair, in different entries
  or twice in one list: **only the first applies**. Each later claim emits a
  one-line warning (`duplicate waiver: rule {rule-id} at {file} — entry [N]
  ignored`); only that pair of entry N is ignored, and its other paths are
  classified as usual. Duplicates are kept until the pair expires, when every
  claim on it is pruned together.

The `waivers` list follows the §text-first-artifacts open-schema
rule. Adopters MAY add fields (e.g., `co-waived-by`, `approved-by-team`,
`ticket`) to enforce org-specific waiver policy in their own CI; `/ductus:review`
and `/ductus:analyze` will not error on unknown fields, and `write-review`
preserves them verbatim on a surviving waiver when it re-renders the block, so
an org policy field is never dropped by a later review run.

## Auto-fix scope

`--fix` is conservative by design. It applies fixes when **all** of these hold:

- The finding is marked `auto-fixable: yes`
- The fix does not change function signatures, return types, or schema
- The fix does not change observable HTTP status codes, error messages, or
  log formats
- The fix does not delete tests
- The fix is contained to files already in the review scope

When in doubt, leave the finding unfixed and let the user apply the
`Suggested fix` manually.

## Output

Stdout summary (always), followed by the path to `review.md`:

```text
/ductus:review — 042-example-feature

  security    ✓ 0 MUST   2 SHOULD
  reuse       ✓ 0 MUST   1 SHOULD
  quality     ✓ 0 MUST   0 SHOULD   (2 low-confidence)
  efficiency  ✓ 0 MUST   0 SHOULD
  simplicity  ✓ 0 MUST   0 SHOULD

  observations 3 — 1 fixed, 1 routed, 1 discarded, 0 undispositioned
  analyze     ✗ last run 2026-09-06 against 683a1e0 — this review supersedes it
  blocking: no
  report:   specs/042-example-feature/review.md

  next: /ductus:analyze, then the spec can advance to done
```

### The `observations` row

The row renders on **every** run from the `dispositions` field `write-review`
returns, as the observation total followed by all four counts — including a
run with no observations, which renders `observations 0`, because
examined-and-empty and not-computed must not be the same output. It is
informational and never affects the exit code, but an `undispositioned` count
above zero is what the pre-done gate will block on, so the row names it rather
than leaving the operator to find it at the gate:

```text
  observations 2 — 0 fixed, 1 routed, 0 discarded, 1 undispositioned — the done gate holds until each is decided
```

The standing inbox count is not a review row. It is a count of the todos a
person has logged, which has no bearing on the spec under review, and
`/ductus:status` renders it on every run.

### The `analyze` row

`/ductus:review` is the command that most reliably _invalidates_ an analyze
record — it rewrites `review.md`, `--fix`
rewrites code, and an operator resolving MUST violations rewrites more — and it
used to say nothing about it. The pipeline mandates
`/ductus:review → /ductus:analyze → done` and the pre-done gate enforces
both records, so the only thing carrying an operator from a passing review to
the second gate was memory: the diligence dependency §design-principles rejects
(spec 047, `analyze-record-freshness`).

The row renders on **every** run, in one of four states, from the
`analyze-freshness` field `write-review` returns:

```text
  analyze     ✗ never analyzed — run /ductus:analyze before done
  analyze     ✗ last run 2026-09-06 against 683a1e0 — this review supersedes it
  analyze     ✓ last run 2026-09-06 against 683a1e0 — current
  analyze     ? freshness undeterminable — the record carries no analyzed-digest
```

`current` is why this is a computed row rather than a fixed reminder: a
`/ductus:review` that changed nothing leaves a genuinely current record, and
reporting it superseded would be the false alarm that teaches operators to skip
the row.

The state is a **content** comparison: the digest the analysis recorded of what
it read, against the spec's analyze subjects as they are now. It is not a
commit comparison, which is why this row and the pre-done gate cannot disagree
— there is no reference point left for them to differ on, and neither reports a
record as stale merely because content the analysis already examined has since
been committed. Writing this review supersedes the record because `review.md`
is itself an analyze subject, not because `HEAD` moved.

The review record works the same way, over its own narrower subject set —
one comparison, two subject sets. See `reviewed-digest` under
[Record the run in `review.md`](#6-record-the-run-in-reviewmd).

A record carrying no `analyzed-digest` — every record written before this
existed — renders the fourth state. It is not current and not stale: nothing on
disk says what that run examined.

**The row is a notice, not a gate.** `/ductus:review` has no authority over
the `done` transition and does not acquire one here: `blocking`, the exit code,
and the review record's own `blocking` are all unchanged by it. The `next:` line
follows for the same reason — it names the command the operator owes, it does
not withhold anything.

When MUST violations are present:

```text
/ductus:review — 042-example-feature

  security    ✗ 2 MUST   1 SHOULD
  reuse       ✓ 0 MUST   0 SHOULD
  quality     ✗ 1 MUST   0 SHOULD
  efficiency  ✓ 0 MUST   0 SHOULD
  simplicity  ✓ 0 MUST   0 SHOULD

  blocking: yes — 3 MUST violations
  report:   specs/042-example-feature/review.md

  spec cannot advance to done. Resolve violations and re-run /ductus:review,
  or run /ductus:review --waive <rule-id> --reason "..." for each waivable finding.
  then /ductus:analyze, which the done gate also requires.
```

The `analyze` row renders on the blocking path too. A blocked review does not
make the second gate go away, and the fixes that clear the violations are
exactly what supersedes the recorded analysis.

Exit code: `0` when not blocking, `1` when blocking. Allows CI to gate on the
exit code without parsing the report.

## Idempotency

Re-running `/ductus:review` against an unchanged target reproduces an identical
`review.md` (modulo `reviewed-at` and `reviewed-against`). This is a
derive-don't-ask invariant: review output is a function of code + rules,
never of session state.

## Notes for adopters

- Projects that customize shipped rule files (e.g.,
  `specs/rules/security-backend.md`) pin them in `.ductus/config.toml`
  `[pinned] files` to prevent `/ductus` from overwriting their additions.
  `/ductus:review` reads whatever is on disk — pinned or not.
- Files inside the rule-file directory (`specs/rules/` in adopter
  projects; `framework/rules/` in ductus's own repo) are auto-discovered
  by directory walk (see step 2, `discover-rule-files`). No `AGENTS.md` reference is
  required. Adding a new file at `specs/rules/<domain>-{backend,frontend,cross}.md`
  with a recognized suffix is the only step needed; the suffix selects
  which stacks load it.
- The `AGENTS.md` rule-file reference survives strictly for adopter-local
  rule files placed **outside** `specs/rules/` — e.g.,
  `docs/rules/internal-api.md`. The framework cannot directory-walk
  arbitrary adopter paths, so an explicit `AGENTS.md` reference is the
  discovery signal for these files.
- A rule file with an unrecognized suffix loads for every stack and
  emits a one-line stdout warning (see step 2, `discover-rule-files`). The default
  is "load + warn," never "silent skip." Rename to one of the closed
  suffixes — `-backend.md`, `-frontend.md`, `-cross.md` — to silence
  the warning.
- A rule file can be explicitly excluded from a given project's review
  via `.ductus/config.toml` `[[review.disabled-rule-files]]` (see
  [Inputs](#inputs) for the schema and step 2 (`discover-rule-files`) for the
  filter behavior). The override is project-wide and requires a
  mandatory `reason` — the reason is the audit trail. Use this when
  the stack-derived selection is correct (the rule file applies) but
  the team is not yet ready to enforce that file's rules (e.g., an
  internal admin UI that has not adopted full WCAG AA). Waivers
  remain the right tool for individual `(rule, file)` exceptions; the
  opt-out is for whole-file deferrals.
- The five-dimension model is fixed. Domain-specific concerns (accessibility,
  i18n, licensing) belong in additional rule files, not new passes.
