# 025 — Rule-file opt-out via `.ductus/config.toml` Plan

Implements [025 — Rule-file opt-out via `.ductus/config.toml`](spec.md).

## Overview

When this spec was planned, the feature was a narrow, markdown-only change. There was no new module, no new parser, no new CLI surface — `.ductus/config.toml` was read ad-hoc by the agent runtime in each command that needs it, following the precedent established by [spec 020's](../020-code-review/spec.md) handling of `[review] tech-stack-verified`. This spec adds one new array-of-tables key (`[[review.disabled-rule-files]]`) and threads its consultation into `/ductus:review`'s rule-file selection step (§Behavior step 5), with discoverability notices on stdout and a one-line callout in `/ductus:status`. Two files in the spec's original Affected list are dropped after planning (see Trade-offs). Selection has since moved into 022's `discover-rule-files` primitive, which is where the malformed-entry halt of scenario `a-malformed-disabled-entry-fails-fast` lands.

## Technical Decisions

### Where the disabled list is consulted

`framework/commands/review.md` §Behavior step 5 is the single read site. The sequence after this spec lands:

1. Discover `framework/rules/*.md` by directory walk.
2. Classify each file by suffix; filter the recognized set by the detected stack (existing 024 logic).
3. **New:** read `.ductus/config.toml` `[[review.disabled-rule-files]]`. For each entry:
   - If `file` matches a file in the post-stack-filter set, drop it and emit the `disabled-rule-file: <name> — <reason> (.ductus/config.toml)` notice.
   - If `file` matches a `framework/rules/` basename that was NOT selected by stack detection, emit the `disabled-rule-file (no-op): <name> not selected by stack detection` notice (no drop — there was nothing to drop).
   - If `file` does not match any `framework/rules/` basename, emit the `unknown disabled-rule-file: <name> (no such file in framework/rules/)` warning (already covered by AC3).
   - A duplicate entry follows the warn-and-skip pattern documented in §Malformed and duplicate waivers. A malformed entry is different: every entry is validated before any rule file is selected, and a malformed one halts the run naming its index and the failing field (`CFG-ENV-008`; scenario `a-malformed-disabled-entry-fails-fast`).
4. Emit the existing `loading rule files: <list>` notice with disabled files excluded.

Reading order matters: the disabled-rule-file notices fire **before** the `loading rule files` notice so the stdout transcript reads top-down as "what got excluded, then what's loading."

### Notice ordering and whitespace handling

The single-line notice format is fixed by AC2. Internal whitespace (including newlines from TOML multi-line strings) in `reason` is collapsed to single spaces when emitting the stdout line — TOML `"""..."""` strings are valid input but the notice is rendered single-line. Implemented as: `reason.split_whitespace().join(' ')`-equivalent in the agent runtime, or the markdown-instruction equivalent.

### Exit code invariance

Warnings (duplicate, unknown, no-op) emit to stdout but never set the exit code. For a run that completes, `/ductus:review`'s exit code remains `1` iff `must-violations > 0` (§Output). A malformed entry is not a warning: it halts the run before selection with `PrimitiveError::InvalidDisabledRuleFile` (`runtime/src/primitives/discover_rule_files.rs`, `validate_disabled_entries`), so the review never completes, records no `blocking`, and exits non-zero (`CFG-ENV-008`); `blocking: true` keeps meaning a MUST violation. Every `file` and `reason` a notice or the halt renders passes through `flatten_line` first, so committed config cannot forge a notice line (`BE-INPUT-011`).

That change is **breaking** under `BE-COMPAT-001`'s reading of behavior, though no field of `discover-rule-files`' result changes: a configuration that returned a selection with a `malformed …` notice now returns an error, and that notice form is gone. The runtime is pre-1.0, so the release that carries it is a minor bump (Cargo's caret requirements make the minor version the compatibility boundary below 1.0), and its one consumer, `framework/commands/review.md`, ships in the same release (§runtime-boundary). This is enforced by the AC4 wording and called out again in the §Output section of the updated review.md.

### `/ductus:status` integration

`framework/commands/status.md` step 6 already lists below-the-table callouts (blocked specs, recovery-state specs, tags-in-use). Add a fourth callout when `.ductus/config.toml` `[[review.disabled-rule-files]]` is non-empty:

```text
disabled rule files: <N> (.ductus/config.toml) — <comma-separated basenames>
```

Single line; no verbose mode. Adopters who need the reasons read `.ductus/config.toml` directly. Surfacing the list at all is the AC6 contract — the dashboard is the discoverability surface, not a full pretty-printer.

The dashboard reads only each entry's `file` (`runtime/src/primitives/dashboard.rs`, `DisabledRuleFile`), so an entry missing its `file` fails `/ductus:status`'s config parse with a TOML error naming the file, while a short or missing `reason` — which the dashboard does not read — does not stop it. The named halt for every malformed shape is `/ductus:review`'s, the reader that consumes the reason.

### Constitution edit

`framework/constitution.md` §rules has a subsection on filename-suffix (lines 285–295, added by spec 024). Append a short paragraph after that subsection — not a new subsection — naming the file-level opt-out and pointing at `framework/commands/review.md` §Inputs. The constitution describes the framework's contracts; the implementation details live in the command file.

### Example TOML block

The canonical `.ductus/config.toml` schema example lives in `framework/bootstrap/ductus.md` §Project Configuration (showing `[pinned]` and, at the time, `[workflows]`). Add a commented-out `[[review.disabled-rule-files]]` block alongside, so adopters running through bootstrap see the schema at the same place they see the others. This replaces the spec body's reference to a `framework/templates/project/ductus-toml.md` file that does not exist.

### `framework/commands/analyze.md` — no edit needed

Verified during planning: `framework/commands/analyze.md` does not read `.ductus/config.toml` at all (grep returns no matches). AC7 ("`/ductus:analyze` does NOT error on the new key") is structurally satisfied by analyze never seeing the file. The Affected files row from spec.md is dropped — there is nothing to extend.

### `scripts/lint-ductus-toml.sh` — out of scope

Verified during planning: the script does not exist. The spec body Affected files row carries an `(if it exists)` qualifier, and the Q3 resolution treats `.ductus/config.toml` hygiene as a separate single-purpose tool. Adding the script as part of 025 is scope creep — runtime warnings (stdout) already cover the operator-feedback path. Defer to a future spec when demand emerges.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/commands/review.md` | edit | §Inputs (add `[[review.disabled-rule-files]]` Config bullet); §Behavior step 5 (apply disabled-files filter after stack filter, emit per-entry notices); §Output (document the new notices); §Notes for adopters (one bullet on the override and a link to `.ductus/config.toml`'s `[[review.disabled-rule-files]]`) |
| `framework/commands/status.md` | edit | Step 6 — add a fourth below-the-table callout when the disabled list is non-empty |
| `framework/constitution.md` | edit | §rules — append a brief paragraph after the filename-suffix subsection mentioning the file-level opt-out |
| `framework/bootstrap/ductus.md` | edit | Example TOML block in §Project Configuration — add a commented-out `[[review.disabled-rule-files]]` example alongside `[pinned]` (and, at the time, `[workflows]`, since sunset by 043) |
| `specs/025-rule-opt-out/spec.md` | edit | Mark all 9 AC checkboxes after implementation is verified; record the analyze/lint-script drops in the frontmatter `review` block via `/ductus:review` |

## Trade-offs

### Considered and rejected

- **Surface `.ductus/config.toml` parsing in a shared TOML reader module.** Ductus has no shared parser — each command's markdown instructions tell the agent to read the file ad-hoc. Following the precedent set by [spec 020's](../020-code-review/spec.md) handling of `[review] tech-stack-verified` keeps the change shape uniform; introducing a shared module for one new key is premature.
- **Extend `framework/commands/analyze.md` to validate `.ductus/config.toml`.** Analyze reads markdown spec artifacts, not `.ductus/config.toml`. Adding a `.ductus/config.toml` validator to analyze would couple two unrelated concerns (artifact-vs-artifact audits vs. config hygiene); single-purpose `scripts/lint-ductus-toml.sh` is the right home if and when it exists.
- **Create `scripts/lint-ductus-toml.sh` as part of this spec.** Spec body line 68's `(if it exists)` qualifier and the Q3 resolution both treat the lint as a separate concern. The feature works without it — a malformed entry halts `/ductus:review` itself, naming the entry. Deferring keeps 025 narrow.
- **Verbose `/ductus:status` listing of disabled files with reasons.** The dashboard is meant to be glanceable; full reasons live in `.ductus/config.toml`. One-line callout with basenames is the AC6 contract.
- **Auto-removing malformed disabled entries.** Same posture as malformed waivers (review.md §Malformed and duplicate waivers): operator-authored state is not framework-collected garbage. The run halts naming the entry, and the operator fixes it.
- **Warning and skipping a malformed entry**, as malformed waivers do. This was the design until the owner's 2026-10-19 decision that `CFG-ENV-008` stands: a skipped entry leaves the rule file enforced while the operator believes it disabled, and a present value that does not parse fails fast rather than being dropped.

### Known limitations

- The disabled list is consulted once per `/ductus:review` invocation, at step 5. There is no in-process rescan or hot-reload — consistent with the rest of `.ductus/config.toml` reads in the codebase.
- The no-op notice (`disabled-rule-file (no-op): ...`) depends on stack detection having produced a definite result. The existing tech-stack alignment gate (§Behavior step 4) guarantees this either via `tech-stack-verified = true` or a fresh check; if both fail the run halts before step 5, so the no-op branch is never reached without a stack.
- Adopters who rename a rule file in their fork (e.g., `accessibility-frontend.md` → `a11y-frontend.md`) and forget to update `.ductus/config.toml` get the `unknown disabled-rule-file` warning and the rule file stays enforced. This is the AC3 design — quiet re-enablement would be a worse failure mode.
