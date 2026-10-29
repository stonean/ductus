# 075 — AGENTS.md holds what every session needs Plan

Implements [075 — AGENTS.md holds what every session needs](spec.md).

## Overview

Four specs own most of the surfaces this spec changes, so their changes land as scenarios on them, each back-linking here with the task `create-scenario` appends: `063-oversized-artifacts-warn-with-a-fix` measures the project files, `041-task-pruning` reduces them, `055-shared-constitution` measures shared constitutions, and `050-constitution` writes the routing text. This spec's own tasks are what no other spec owns: the `ductus:memory-routing` block, the four project templates, the shipped hook's step, the config documentation, the sweep, and the release that ships them together.

## Technical Decisions

### Work lands on the owning specs

Each owned surface changes through a scenario on its owner, per `AGENTS.md`'s rule to route shared runtime work to its owning spec; the owner's back-link discharges this spec's `cross-spec-impact:` (§cross-spec-impact). Recording a scenario takes `063` and `055` from `done` to `in-progress` (§spec-lifecycle, new scenario); `041` and `050` are already `in-progress`. Each primitive's shapes go in its owner's `data-model.md`, never here, so each shape has one canonical source.

### `check-artifact-size` measures the project files (063)

- **Arguments.** `feature` becomes optional (`runtime/src/schema/primitives.rs`, `CheckArtifactSizeArgs`). `project-files` adds the four subjects: `AGENTS.md` at the project root and `{specs-root}/system.md`, `events.md` and `errors.md`. `staged` measures each file's index copy against its `HEAD` copy. It requires `project-files` and excludes `feature`. A call with neither `feature` nor `project-files` is an argument error.
- **Result.** The shared keys stay at the top level (`threshold`, `examined`, `oversized`, `skipped`, `notices`). Two flattened `Option` halves follow, sharing no key, per `AGENTS.md`'s gotcha on two-shape results. The spec half is `feature`, `status`, `decisions` and `path`. The project half is `project-files`, one record per file: path, state (`measured`, `absent`, `unreadable`) and bytes, plus `head-bytes` and `refused` in staged mode. A feature-only call returns exactly today's keys and values, so `/clarify` and `/plan` see no change.
- **Reporting.** An oversized project file joins `oversized` under its own `ArtifactKind`. Its one fix, a new `FixKind`, is `/prune --project`, and it carries no `on-done`. Pages and warnings are computed as for spec subjects (`runtime/src/primitives/check_artifact_size.rs`, `run`). The limit is `resolve_threshold` unchanged: one key for all four files (spec, Resolved Questions). With a feature given, a stored discard in that feature's `analysis.md` covers a project file as it covers a spec subject (`decide`), so it re-fires on growth into another page.
- **Absent and unreadable.** An absent project file is reported as state `absent`. This departs from 063's rule that a missing spec subject goes unreported, because this spec requires a missing project file to get its own result. An unreadable one goes to `skipped` under `artifact-unreadable`, as now.
- **Staged mode.** It reads the index and `HEAD` blobs through `ProjectRepository::discover` (`runtime/src/primitives/mod.rs`), converting paths with `to_git`, so a project in a repository subdirectory (spec 059) measures the right blobs. A file is refused when its staged copy is over the limit and either has no `HEAD` copy or is larger than it. A file absent from the index is `absent` and passes, which is the deleted-file edge case. The CLI exits 1 when any file is refused or unreadable, through `emit_result_gated` (`runtime/src/main.rs`), and prints each one's file, size, limit and fix on stderr, as `report_broken_links` does for `check-corpus-links`.
- **`/analyze`.** Step 17 passes `project-files: true` with the feature. On the exec path, the walker dispatches a step with no per-step arguments, so `derive_analyze_seeds` seeds the key (`runtime/src/interpreter/mod.rs`, applied only when the command is `analyze`). `/clarify` and `/plan` stay feature-only, since the spec places the check in the hook, `/analyze` and `/target` only.

### The shipped hook refuses growth past the limit (075)

`framework/bootstrap/hooks/ductus-pre-commit` runs `check-artifact-size --project-files --staged` after its re-stage block, beside `check-corpus-links`. Like that check, it only reads, and a blocking read placed before the re-stage would leave the derived rewrites unstaged (the hook's own comment on `check-corpus-links`' placement). The hook's capability loop probes subcommand names only, and older runtimes already have `check-artifact-size`. So the hook captures `check-artifact-size --help` into a variable and checks it for `--project-files`, rather than piping it (§grounding), and halts with the loop's too-old message when the flag is missing. This repository's `.githooks/pre-commit` is unchanged. `scripts/lint-governing-size.sh` already holds this repository's `AGENTS.md` to a hard 50,000-byte ceiling, which is stricter, and this repository has no `system.md`, `events.md` or `errors.md`.

### `resolve-constitutions` measures what it loads (055)

`write_review.rs` and `write_analysis.rs` call `resolve_constitutions::run` directly. So measurement goes in a wrapper the primitive entry dispatches, and those two callers keep the unmeasured resolution. An invalid `read-size-bytes` must not newly halt `write-review` or `write-analysis` (`QUAL-DELEG-001`). Each loaded `ConstitutionRecord` gains its `bytes`, whether it is over the limit, and a runtime-worded warning naming the alias, size, limit and the owning `repo` as the fix. A skipped record carries none of these, and `/target` reports it as *not measured*. The result carries `threshold` only when `examined` is above 0, so a project with no registry reads no new key (055 AC1). `/target` step 4 renders the size beside each alias and never blocks on it.

### `/prune --project` and a `prune-project` primitive (041)

- **Flag.** `--project` needs no session target. It is refused with `--all`, `--reset` or `--force`, as `--all` refuses `--force` (`framework/reference/prune.md`, `--all`).
- **Primitive.** `prune-project` takes the deterministic half, as `prune-plan` does for plan sections (041 AC16).
  - **Preview** segments the four files into entries. Per file it returns state, bytes and over-limit. Per entry it returns its locator (file, enclosing `##` heading, line range, kind), bytes and digest, never the entry's text. For an `events.md` or `errors.md` entry it also returns the identifier, which is the first code span of its `###` heading or of its table row's first cell, and the literal search behind any removal proposal: the files searched (tracked files outside the spec root) and each one containing the identifier.
  - **Apply** removes or replaces the listed entries by digest. It refuses the whole write when any listed entry changed since the preview, as `prune-plan` does. It can create a missing `system.md`, `events.md` or `errors.md` from its template, compiled into the primitive as `prune-tasks` compiles the tasks body. Writes are atomic and leave no backup.
- **Entry grammar** (041's `data-model.md`). An entry is a `###`-or-deeper heading section, or, inside a `##` section's own body, a top-level list item with its nested lines, a table body row, or another blank-line-separated block. HTML comments and the `ductus:memory-routing` and `ductus:constitutions` regions are never entries, and `##` headings are structure.
- **Host judgment.** The host proposes each entry's home by the spec's table, writes the home first, then calls apply (the order `framework/commands/prune.md` step 8 uses for plan sections).
  - A procedure goes to `docs/` or a README beside the code. A skill is named but never written, since ductus defines no skill location (`framework/templates/project/agents.md`, Skills).
  - A one-feature decision goes to that spec's Resolved Questions, or an existing scenario the operator names. `/prune` never creates a scenario, because `create-scenario` appends an implement task for behavior already built (063, Out of scope). An edit to a `done` spec reopens it by §spec-lifecycle, and the reopen is named before the write.
  - An organization rule is named with its shared constitution's `repo`. It leaves `AGENTS.md` only after the operator confirms it has landed there.
  - An every-adopter entry is checked against `.ductus/constitution.md`, `.ductus/reference/` and the runtime, then becomes a pointer, is deleted with its reason, or is trimmed to one line. In the last case the run's report carries a drafted upstream report with the project's names removed. Nothing is filed or written outside the project.
  - Step 8's plan-section routing uses the same definitions (AC10).

### The constitution's text (050)

AC1's test and homes go in §drift-prevention *Shared knowledge stays in git*, and AC13's definitions go in the §spec-phase tree. `framework/constitution.md` is capped at 50,000 bytes (`scripts/lint-governing-size.sh`, `CEILINGS`) and measured 49,993 (`wc -c`, 2026-10-29, a figure that includes another session's uncommitted edits). So 050's task cuts at least as many bytes as it adds, in the same commit. That task also rewrites 050's scenario `artifacts-are-as-short-as-their-required-content`, whose rule that existing artifacts are not swept stops holding for the four project files.

### The memory-routing block and the templates (075)

- **Block.** The one line in `framework/bootstrap/ductus.md` §Memory routing, step 1, names `AGENTS.md` for what every session needs and each other home with when to read it. `merge-managed-block` writes it on every run, so existing adopters receive it. `framework/bootstrap/govern.md` must stay byte-identical (`scripts/audit/transitional-bootstrap-parity.sh`).
- **Templates.** The `agents.md` template's header points at the constitution's test and drops its instruction to append a project-only rule to the matching section. Its Project Structure keeps the three top-level rows and replaces the `specs/` subtree with a pointer to §spec-phase. The `system.md`, `events.md` and `errors.md` headers each point at §spec-phase. These reach new adoptions only (spec, Surfaces).
- **This repository's `AGENTS.md`.** Its header carries the same "append to the matching section" instruction, so the sweep changes it to match.

### Verification

- **Owner primitives.** Runtime tests in each owner prove the deterministic halves: AC5's one-byte-over, at-limit, absent and unreadable cases; staged refusal; AC15's over-limit constitution; and `prune-project`'s segmentation, stale refusal, home creation and literal search. Each test is shown to fail with its behavior removed (`QUAL-TEST-001`).
- **AC14.** Family 22 (`scripts/audit/adopter-shell-behavior.sh`) already runs the shipped hook in adopter-shaped trees, so the grow, add, shrink, unchanged and delete cases join it.
- **AC6, AC11 and AC12.** These rest on host judgment, so they are verified by a `claude -p` run of `/ductus:prune --project` over a fixture project under `/tmp`, with the result cited in the landing commit, as 072 verified its AC7.

### Landing order and release

`--ref=main` adopters run whatever is on `main`, so 041's walk lands no later than 063's project mode and this spec's hook step, both of which name `/prune --project`. AC7 then holds for any tag: none can contain the check without the walk, which `git tag --contains` verifies. The tag itself is cut after the touched specs reach `done`, per `AGENTS.md`'s Workflow, so it is no task here.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Modify | AC1 routing, AC13 tree lines, offsetting cut (050's task) |
| `framework/bootstrap/ductus.md` | Modify | Memory-routing line; `[artifacts]` schema comment |
| `framework/bootstrap/govern.md` | Modify | Byte-identical mirror of `ductus.md` |
| `framework/templates/project/agents.md` | Modify | Header pointer; Project Structure without the spec tree |
| `framework/templates/project/system.md` | Modify | Header pointer |
| `framework/templates/project/events.md` | Modify | Header pointer |
| `framework/templates/project/errors.md` | Modify | Header pointer |
| `framework/bootstrap/hooks/ductus-pre-commit` | Modify | Staged size step and its capability probe |
| `framework/commands/prune.md` | Modify | `--project` walk; step 8 routing (041's task) |
| `framework/reference/prune.md` | Modify | Markdown-only project-file walk (041's task) |
| `framework/commands/analyze.md` | Modify | Step 17 passes `project-files` (063's task) |
| `framework/reference/analyze.md` | Modify | §Artifact size covers the project files (063's task) |
| `framework/commands/target.md` | Modify | Step 4 reports each constitution's size (055's task) |
| `framework/runtime-tools.txt` | Modify | Register `prune-project` |
| `runtime/src/primitives/check_artifact_size.rs` | Modify | Project subject set and staged mode |
| `runtime/src/primitives/prune_project.rs` | Create | Segment, search, apply |
| `runtime/src/primitives/resolve_constitutions.rs` | Modify | Measuring wrapper |
| `runtime/src/schema/primitives.rs` | Modify | Args and result shapes |
| `runtime/src/interpreter/mod.rs` | Modify | Analyze seed; `prune-project` dispatch arm |
| `runtime/src/main.rs` | Modify | `prune-project` command; gated exit for staged mode |
| `runtime/src/mcp/server.rs` | Modify | `prune-project` tool |
| `runtime/src/schema/registry.rs` | Modify | `prune-project` in `PRIMITIVE_REGISTRY` |
| `framework/bootstrap/configure/claude.md` | Regenerate | `prune-project` permission, via `scripts/gen-configure-mcp.sh` |
| `framework/bootstrap/configure/auggie.md` | Regenerate | `prune-project` permission, via `scripts/gen-configure-mcp.sh` |
| `scripts/audit/adopter-shell-behavior.sh` | Modify | Family 22 hook cases |
| `scripts/gen-help-tables.sh` | Modify | `/prune --project` help entry |
| `README.md` | Modify | `[artifacts]` entry (AC16) |
| `AGENTS.md` | Modify | Header pointer (this repository) |
| `specs/063-oversized-artifacts-warn-with-a-fix/data-model.md` | Modify | Project half of the result |
| `specs/041-task-pruning/data-model.md` | Modify | Entry grammar; `prune-project` schema |
| `specs/055-shared-constitution/data-model.md` | Modify | Measured `ConstitutionRecord` |

## Data Model

This spec adds no `data-model.md`: each shape belongs to the primitive's owner. 063's `data-model.md` gains the project half of `CheckArtifactSizeResult`, 041's gains the entry grammar and `prune-project`'s schema, and 055's gains the measured `ConstitutionRecord`, each in the task its scenario appends.

## Trade-offs

- **Measuring project files in `/clarify` and `/plan`** was rejected. The spec places the check in the hook, `/analyze` and `/target`, and neither command reads those files.
- **A separate, unbackticked `/analyze` step for project files** was rejected. The exec path would never run it, while seeding keeps one call and one decision match.
- **Extending `prune-plan`** was rejected. Its unit is a design-record section of one feature's plan, while entries have their own grammar and span four project-level files.
- **Replacing `scripts/lint-governing-size.sh`** with the staged mode was rejected. It holds the constitution, which is no project file, to a hard ceiling, and 072 owns it.
- **Measuring shared constitutions in `check-artifact-size`** was rejected. They live outside the project and its index, and `/target` already resolves them through `resolve-constitutions`.
- **Limitations.**
  - A proposed home is host judgment, proven by a recorded agent run rather than a deterministic test.
  - A literal search misses an identifier built at runtime. The search says so, and the operator confirms each removal.
  - On the markdown-only path, a missing home is created with its `#` heading only, because an adopter's tree holds no copy of the project templates (`framework/bootstrap/ductus.md`, Shared Files, installs them as the files themselves). This is the same bound `prune-tasks`' compiled reset body has.

## Open Questions Resolved

- **Does writing a moved entry into source exceed "the one widening" of `/prune`'s scope?** No. That sentence sits in the spec's paragraph on judging catalog entries and concerns reading source. AC6 requires each confirmed entry to land in its row's home, and one row's home is a comment at the site. So `/prune --project` writes to every home in the table, each only after confirmation.

## Cross-spec impact

| Spec | Owes | Discharged by |
| --- | --- | --- |
| `063-oversized-artifacts-warn-with-a-fix` | Project subject set, staged mode, `/analyze` advisory | Its scenario back-linking this spec, and its task |
| `041-task-pruning` | `/prune --project`, `prune-project`, step 8 routing | Its scenario back-linking this spec, and its task |
| `055-shared-constitution` | `/target`'s size report | Its scenario back-linking this spec, and its task |
| `050-constitution` | AC1 and AC13 text with an offsetting cut; its succinctness scenario's sweep rule | Its scenario back-linking this spec, and its task |
