# 063 — Oversized artifacts warn with a fix Plan

Implements [063 — Oversized artifacts warn with a fix](spec.md).

## Overview

One new read-only runtime primitive, `check-artifact-size`, measures a feature's subject artifacts against a configured threshold and returns, for each one over it, its size, read-page count, the fixes for its kind with each fix's effect on a `done` spec, and a rendered warning. `/clarify`, `/plan` and `/analyze` each gain one step that invokes it. The first two show the warnings and never gate on them; `/analyze` records each as an advisory finding in a new `artifact-size` family, and the primitive tells it which of those a stored discard still covers. The threshold is a new `[artifacts] read-size-bytes` key, documented in the README and in the bootstrap's configuration schema. `/plan`'s guidance stops placing code snippets in plans.

## Technical Decisions

### A standalone primitive, called by three commands

The check is its own primitive rather than a tenth `check-artifacts` family. A numbered step dispatches exactly one primitive, so a step naming two is a parse error (`runtime/src/parser/mod.rs`, module docs and the two-primitive rejection). `check-artifacts` takes only `feature` and runs all nine of its families together (`CheckArtifactsArgs` in `runtime/src/schema/primitives.rs`), so `/clarify` and `/plan` could only reach a family inside it by also running eight families that do not belong in their output. A standalone primitive serves all three commands from one implementation, and `/analyze` already composes a separately invoked primitive into its findings: step 16 invokes `prune-plan` and records its output as the `plan-record` advisory family (`framework/commands/analyze.md`, step 16).

The primitive is read-only. It writes nothing in any mode, so no command gains a write and no gate is added.

### Subjects, absence, and readability

The subjects are `spec.md`, `plan.md`, `tasks.md`, `data-model.md`, and every `*.md` directly under `scenarios/`, listed in the shared scenario ordering (case-insensitive, raw-byte tiebreak) that `/clarify`'s scenario report already uses. `research.md`, `review.md`, `analysis.md` and any file the project added are never examined.

The `feature` argument comes from the host, so it passes `validate_no_traversal` before it is joined under the spec root — the `BE-INPUT-004` defence-in-depth check every primitive taking a host-supplied path runs (022's scenario `config-sourced-paths-and-the-traversal-boundary`).

A subject that does not exist is skipped with no entry (AC15): at `draft` a missing `plan.md` is a state, and `check-artifacts`' completeness family already judges a missing artifact where its status requires one. A subject that exists but cannot be opened for reading is listed under `skipped` with the reason `artifact-unreadable` (AC11), the same reason and the same could-not-be-read class `check-artifacts` and `write-analysis` already use (`runtime/src/schema/primitives.rs`, the closed reason set on the unexamined-by-reason field), so `/analyze` counts it as unexamined and restricts decision expiry exactly as it does for its other unreadable targets. An unreadable `scenarios/` directory is one `skipped` entry for the directory.

### Size is the file's length on disk

The size is the file's metadata length after the open succeeds, with no read of its content and no normalization (AC16). That is the number of bytes an agent's reader receives, CRLF and multi-byte characters included, and it costs one stat per file rather than a read of a data model as large as 022's `specs/022-deterministic-runtime/data-model.md`, 117,747 bytes on 2026-10-04.

### Read-page count and the reporting line

The read-page count is the size divided by the threshold, rounded up, with a floor of one. A subject is reported when its count is two or more, which is exactly when its size exceeds the threshold, so a file at the threshold is not reported and one byte over it is (AC16).

### The threshold: `[artifacts] read-size-bytes`

The threshold is read from a new `[artifacts]` section of `.ductus/config.toml`, key `read-size-bytes`, default 50,000 (AC4). The section is named for what it governs, spec artifacts, rather than for one command, because three commands read it; the unit is in the key name because the value is a bare integer. The default and the valid range's one bound are named constants side by side in the primitive, `DEFAULT_READ_SIZE_BYTES` (50,000) and `MIN_READ_SIZE_BYTES` (1), so the range is stated once (`CFG-CONST-003`, `CFG-CONST-004`). There is no maximum: a host's read cap has no known ceiling.

The file is located through `paths::resolve_config` (`runtime/src/schema/paths.rs`), the same newest-wins ladder (`.ductus/` → `.govern/` → legacy root) every config reader uses, and the key is held as a raw `toml::Value` so a wrong type is reported rather than failing the parse, as `discover-rule-files` does for `[rules] surfaces` (`runtime/src/primitives/discover_rule_files.rs`, `load_ductus_toml` and `RulesSection`). The result names the threshold's source:

- **default** — the key is unset, or the config file is absent.
- **config** — a positive integer was read and used.

Any other value halts (AC5). A string, a fraction, a boolean or an array does not parse as a whole number (`CFG-ENV-008`); zero or a negative parses but falls below `MIN_READ_SIZE_BYTES` (`CFG-ENV-005`, which names environment variables and is held to this file-based key the same way). Both, and an `artifacts` that is not a table, fail with `PrimitiveError::InvalidReadSize`, which names the setting, the value as TOML renders it (flattened to one line, so committed config cannot forge a second line in the message), the file, and the accepted form. The default never stands in for a value the project set, so every command that invokes the primitive halts on it, as it does on an unparseable config file. An invalid value ran at the default with a notice until the owner's 2026-10-19 decision that `CFG-ENV-008` stands, which the scenario [an invalid threshold fails fast](scenarios/an-invalid-threshold-fails-fast.md) records.

That change is **breaking** under `BE-COMPAT-001`: the `check-artifact-size` result loses `threshold.rejected` and the `invalid` member of `threshold.source`, and a configuration that used to return a result now returns an error. The runtime is pre-1.0, where Cargo's reading of SemVer makes the minor version the compatibility boundary (a `^0.y.z` requirement accepts only `0.y` releases; the Cargo Book, *Specifying Dependencies*, Caret requirements), so the release that carries it is a minor bump, as 0.53.0's removal of `write-review` and `compute-review-scope` result fields was (`runtime/CHANGELOG.md`, 0.53.0, Removed). Every consumer of the result — the three command sources and the exec walker's analyze tally — ships in that same release (§runtime-boundary).

A config file that does not parse as TOML is an operational error, as it is for `discover-rule-files` and every other config reader: the project's configuration is broken for every command, and a size check is not the place to paper over that.

A key in `[artifacts]` that the shared config-key registry (`runtime/src/schema/config_keys.rs`, 022's `config-readers-name-unknown-keys`) does not recognize — `read-size-byte` for `read-size-bytes` — is named in a notice rather than dropped, and the check runs at the threshold the recognized keys give (`specs/063-oversized-artifacts-warn-with-a-fix/scenarios/an-unknown-artifacts-key-is-named.md`, for spec 019's `CFG-ENV-012` decision). The notice is a new member of `notices`, not a new field, so the result's shape is unchanged; a malformed `read-size-bytes` still halts before any notice is returned.

There is no key that disables the check (spec, Resolved Questions).

### The fixes, and their effect on a `done` spec

The primitive computes the fixes from the subject's kind, following the spec's table, so the wording is the runtime's and is tested rather than left to each host:

- `tasks.md` — prune its spent sections with `/{project}:prune`; if it is still over the threshold after pruning, the spec itself is too big, and the `spec.md` fixes apply.
- `spec.md` and `plan.md` — split the spec when a slice can reach `done` on its own, or trim (for a plan, its code sketches and the Affected Files rows for finished work).
- `data-model.md` — split the spec or trim the file, never into sub-files, because review staleness tracks it by its exact name (`is_review_contract` in `runtime/src/primitives/analyze_subjects.rs`).
- a scenario — promote it to its own spec.

Every fix carries an `on-done` effect, set only when the spec is `done` (AC8): `never` for pruning `tasks.md`, `always` for split and promote, and `if-claim-changes` for trim. Below `done` it is absent, because no fix reopens a spec that is not `done`. The split fix's text describes the manual route (create the new spec with `/{project}:specify`, move the content, link it from the original) and names no split command, since none exists (AC9).

The discard is not in this list. It is a disposition only `/analyze` can record, and `/analyze` already offers it for every undecided advisory finding at its fix-and-route step, so a primitive-level flag for it would duplicate that offer and need a per-step argument the exec walker has no way to bind: a dispatched primitive's arguments are the walk's shared context, adjusted only by rebinding hooks keyed on the primitive's name (`runtime/src/interpreter/mod.rs`, the bindings built before the `call!` dispatch). `/analyze`'s step text says the discard is the further fix, which is how the warning names it in `/analyze` only (AC7).

Command names are rendered as `/{project}:…`, the placeholder the runtime's existing guidance strings use (for example the `check-review-gate` and `write-review` guidance in `runtime/src/primitives/`).

### The rendered warning

Each oversized subject carries a one-line `warning` the commands print as given: the path, its size in bytes, the threshold, the read-page count, the statement that an agent *may* not read it in one call, then the fixes with their `done` effects. The wording never says *will* (AC14), and a unit test asserts both the presence of *may* and the absence of *will* over every kind and status.

### `/analyze`: the `artifact-size` family and the decided rule

`/analyze` gains a step after step 16, so steps 17–21 become 18–22. It records each oversized subject as an **advisory** finding (AC1), family `artifact-size`, message `` `{path}` is {bytes} bytes, {pages} read pages at the {threshold}-byte read size ``, path the subject's. The message carries the size in bytes so the record states it; matching does not depend on the bytes, below.

Stored-decision matching at step 17 (18 after the renumber) is the host's judgment over reworded findings (`framework/commands/analyze.md`, step 17), and judgment cannot implement AC10's rule that a discard holds until the file grows into another page. So the primitive decides it, the way `prune-plan` reports each plan section as `decided` from the stored discards (`runtime/src/primitives/prune_plan.rs`, the `decided` closure over `read_decisions` and `same_key`):

- It reads `analysis.md`'s `decisions:` list through the existing `decisions::read_decisions`, which parses the frontmatter YAML with `serde_norway` into typed entries, the safe-loading mode `BE-INPUT-008` asks a plan to name (022's plan, under its dependency list). The spec's `status` is read the same way.
- A subject is **decided** when a stored `discarded` decision's key is an `artifact-size` key for the same path whose recorded page count is **greater than or equal to** the subject's current count. The key's path and page count are parsed from the fixed message format above.
- A decided subject carries that stored key as `decision-key`. `/analyze` passes it as the subject's `fired` key, so `process-decisions` reports the decision `matched` and nothing is asked. An undecided subject fires its own `{family} — {message}` key, as every other finding does.

The rule gives exactly the spec's behavior: growth into another page fires again (AC10); shrinking, or growth within the same page, stays decided; a threshold change recounts the pages and is judged by the same comparison. A subject that drops under the threshold stops firing, and its stored discard expires the ordinary way.

When `analysis.md` is absent, nothing is decided. When its `decisions:` list does not parse, the primitive does **not** fail. `/clarify` must not break on a broken analysis record it does not own. Instead it reports the list as unreadable, leaves every subject undecided, and `/analyze`'s step treats that as step 17 (18) already treats a refusal of the list: it asks about each finding as though nothing were stored. This deliberately differs from `prune-plan`, which errors, because `prune-plan` is only ever invoked by commands that own that record.

`/analyze`'s markdown-only reference lists the `artifact-size` family as advisory in its per-check severity assignment, with its own section stating the subjects, the threshold, the decided rule, and that it stays advisory for good: a size is a heuristic about a host it cannot see, so it is never promoted to blocking.

### `/clarify` and `/plan`: a warning, never a gate

`/clarify` gains a step after step 9 (`label-criteria`), so the size is measured after the questions' answers have been written into the spec, and steps 10–13 become 11–14 (AC2). `/plan` gains a step after step 7, the task breakdown, so `plan.md` and `tasks.md` are measured as written, and steps 8–10 become 9–11. Each step prints every warning and notice, and states that the result never blocks the transition; an invalid threshold halts the command instead, as above. `/plan`'s **Validation gate** reference lists the size check as advisory beside markdownlint, the other advisory check there. A scenario-targeted `/clarify` run does not take the step.

### The markdown-only path

Each new step says what the host does without the runtime (AC12): measure each subject's size with its own file tools, render the same warning, and list any subject whose size those tools cannot report as not examined, never as under the threshold.

### `/plan` stops placing code in plans

`framework/commands/plan.md`'s Technical Decisions guidance (Create the plan, step 2) has a decision name the code it concerns by path, citing `path:line`, rather than reproducing it, because a sketch goes stale as soon as the code lands and regrows a trimmed plan (AC13). It no longer says that code snippets, function signatures, and package paths belong in the plan. The plan template carries no code guidance of its own (`framework/templates/spec/plan.md`), so it is unchanged.

### Documentation

The setting is documented in the two places the spec names (AC17): a bullet in `README.md`'s Configuration list of hand-editable keys, and a commented `[artifacts]` block in `framework/bootstrap/ductus.md` §Project Configuration, mirrored in its transitional twin `framework/bootstrap/govern.md` (the two are held together by the transitional-bootstrap parity audit family). Each states the 50,000-byte default, that the value is a whole number of bytes, and that it raises or lowers the threshold but cannot switch the check off. `docs/analyze.md` gains the family, and `runtime/CHANGELOG.md`'s `[Unreleased]` section gains the entry.

### Registration

The primitive is registered everywhere a primitive is, following the `prune-plan` registration as the checklist: `PRIMITIVE_REGISTRY`, the clap subcommand in `runtime/src/main.rs`, the MCP tool in `runtime/src/mcp/server.rs`, the exec dispatch arm in `runtime/src/interpreter/mod.rs`, `framework/runtime-tools.txt`, and the permission entries in `framework/bootstrap/configure/claude.md` and `framework/bootstrap/configure/auggie.md`. The exec tally in `runtime/src/interpreter/analyze_tally.rs` counts each undecided oversized subject as advisory and each skipped subject as unexamined under `artifact-unreadable`. Exec matches no stored decision, so, as with `prune-plan`, a decided subject is counted live there. The `plan-basic` and `analyze-basic` goldens are re-blessed for the new steps.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/primitives/check_artifact_size.rs` | Create | The primitive: subjects, size, pages, threshold, fixes, warning, decided rule |
| `runtime/src/primitives/mod.rs` | Modify | Module declaration |
| `runtime/src/schema/primitives.rs` | Modify | Args and result types |
| `runtime/src/schema/registry.rs` | Modify | Registry entry |
| `runtime/src/main.rs` | Modify | CLI subcommand |
| `runtime/src/mcp/server.rs` | Modify | MCP tool |
| `runtime/src/interpreter/mod.rs` | Modify | Exec dispatch |
| `runtime/src/interpreter/analyze_tally.rs` | Modify | Exec tally, and the step-number comments the renumber moves |
| `runtime/tests/walker.rs` | Modify | Exec coverage of the three new steps |
| `runtime/tests/golden/plan-basic.jsonl`, `runtime/tests/golden/analyze-basic.jsonl` | Modify | Re-blessed for the new steps |
| `framework/commands/clarify.md` | Modify | New step 10, renumber, markdown-only text |
| `framework/commands/plan.md` | Modify | New step 8, renumber, Validation gate bullet, code-in-plans guidance |
| `framework/commands/analyze.md` | Modify | New step 17, renumber, Artifact size reference section |
| `framework/runtime-tools.txt` | Modify | Tool coverage |
| `framework/bootstrap/configure/claude.md`, `framework/bootstrap/configure/auggie.md` | Modify | Permission entries |
| `framework/bootstrap/ductus.md`, `framework/bootstrap/govern.md` | Modify | `[artifacts]` in §Project Configuration |
| `README.md` | Modify | Configuration bullet |
| `docs/analyze.md` | Modify | The family |
| `runtime/CHANGELOG.md` | Modify | `[Unreleased]` entry |
| `specs/058-findings-route-at-discovery/scenarios/only-unreadable-targets-retain-decisions.md`, `specs/058-findings-route-at-discovery/plan.md` | Modify | Step-number sync for the `/analyze` renumber |

## Data Model

[data-model.md](data-model.md) defines the `[artifacts]` configuration key, the primitive's argument and result shapes, the subject and fix records, the `artifact-size` finding's message format, and the decided rule's parse of a stored key.

## Trade-offs

- **A tenth `check-artifacts` family** would have reached `/analyze` with no renumbering, and was rejected: `/clarify` and `/plan` could not call it alone, and changing `check-artifacts`' family count or argument shape rewrites `done` claims in 067 (AC1 names its families), which would reopen 067 for a change that is not about it.
- **A `families` filter on `check-artifacts`** was rejected for the same reopen, and because it would couple `/clarify` to a primitive whose other families judge artifacts `/clarify` does not touch.
- **Leaving decided-matching to the host's judgment** was rejected: AC10 is a numeric rule, judgment over reworded messages cannot hold it, and `prune-plan` already shows the deterministic shape.
- **Failing on an unparseable `decisions:` list**, as `prune-plan` does, was rejected because `/clarify` and `/plan` would then fail on a defect in a record they do not own.
- **Reading each file to measure it** was rejected for the stat: the length on disk is the quantity the spec defines, and reading a large data model to count it would spend the very cost the warning is about.
- **Known limitation: bytes are a proxy.** The warning can fire on a file a host reads whole and miss one a token-dense host truncates. The spec accepts this, words the warning as *may*, and leaves the threshold to the project.
- **Known limitation: the renumber.** Steps after the new ones move in all three commands; the step-reference audit family and `check-step-references` are what catch a reference missed in the sweep.

## Cross-spec impact

- **058 — Findings route at discovery** owes a step-number sync, not a requirement change: its scenario `only-unreadable-targets-retain-decisions` cites `/analyze` step 17 twice and its plan cites step 20, which become 18 and 21. The edit maps one identifier to its current value and changes no claim, so it is mechanical under §spec-lifecycle case (a) and 058 stays `done`. It is not a sweep Family 19 excuses, though: that exemption counts a token pair only across files whose whole diff is a substitution, and `analyze.md`'s diff also inserts a step, so 058's review reads stale after the sync until it is re-run, which task 8 owes. No other spec cites the moved steps; the sweep was `grep` over every `.md` and `.rs` file outside `.claude/` for step 17–22 references near `analyze` and step 8–13 references near `plan` or `clarify`.
- No other spec's claims change. 022's primitive library is described as an initial set, not a complete one, and `check-artifacts`' contract is untouched.
