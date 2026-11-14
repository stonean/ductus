# 056 — Bootstrap archive-boundary split Plan

Implements [056 — Bootstrap archive-boundary split](spec.md).

## Overview

Move nine level-2 sections out of `framework/bootstrap/ductus.md` into a new `framework/bootstrap/ductus-procedure.md`, leave one pointer behind, allowlist the new file as reference prose, and re-copy `govern.md`. The nine sections form **five contiguous blocks**, so the move is five excisions rather than nine, and every one is a verbatim cut-and-paste — no prose is rewritten in the move itself.

The whole change is `framework/` plus one non-published data file under `runtime/`. No runtime source changes, no version bump, no release tag. That is a conclusion from the consumer walk in `spec.md` §What the split does not reach, not an assumption, and task 5 re-earns it by running the families rather than re-reading the table.

## Technical Decisions

### The boundary is already settled; this plan does not re-derive it

`spec.md` §Measured carries the section assignment and the coverage assertion behind it. This plan consumes that table and does not restate it — restating it would create the second copy of a canonical record that [§drift-prevention](../../framework/constitution.md#drift-prevention) forbids.

### Five excisions, not nine

The nine moving sections are contiguous in five runs (line numbers against `framework/bootstrap/ductus.md` at `dfc4a3fc`, 1186 lines):

| Block | Lines | Sections | Bytes |
| --- | --- | --- | --- |
| A | 464–504 | `§Pre-run Migrations` | 6,067 |
| B | 655–748 | `§Frontmatter Migration` | 4,769 |
| C | 818–873 | `§Security Audit (brownfield)` | 5,332 |
| D | 959–1025 | `§Hook Installation` | 8,061 |
| E | 1051–1187 | `§What This Command Does NOT Do`, `§Edge Cases`, `§Post-Scaffolding Output`, `§Idempotency`, `§Directory Creation` | 14,387 |

Block E is the whole tail of the file, which is why the move is cheaper than the section count suggests. Excising bottom-up (E, D, C, B, A) keeps every earlier block's line numbers valid while the edit is in flight — the same by-position hazard `AGENTS.md` records for `remove-inbox-item`, where each removal shifts the lines under the next one.

**The cut is verbatim.** Sections move byte-identical; any prose that has to change because it now lives in another file is a *separate, named* edit in task 2, so the diff distinguishes "moved" from "rewritten". Bundling them would make the move unreviewable.

### One pointer, placed where the archive first exists

`ductus.md` gains a single `## The archive half` section immediately after `§File Fetching` — the first point in the file at which `{tempdir}/ductus-main/` exists (since 061, `{framework-root}`, the root derived from the extraction) — naming the file, what it holds, and when to read it. One pointer rather than five in-place stubs: five stubs would add back bytes the split exists to remove, and each would be a place for the two files to drift.

The address is `{tempdir}/ductus-main/framework/bootstrap/ductus-procedure.md` (since 061, `{framework-root}/framework/bootstrap/ductus-procedure.md`, the root derived from the extraction). No new resolution mechanism is introduced: `§Archive fetch and extract` already computes that framework root and calls it "the local mirror of the `ductus` repo for the rest of the run", and that section stays installed.

### The archive half is reference prose, and is allowlisted as such

`scripts/lint-procedure-parseability.sh:80` globs `framework/bootstrap/*.md`, so the new file is in its scope automatically. Probed in both directions: a bootstrap-shaped file carrying frontmatter but no `## Instructions` returns exit 2 — `legacy prose — no parseable Instructions section` — which fails the lint unless the file is listed in `runtime/legacy-prose-commands.txt`.

Allowlisting is correct rather than expedient. `ductus exec ductus` walks `§Instructions`, which stays installed; nothing dispatches from the archive half, so it genuinely is prose. Manufacturing an `## Instructions` section to score exit 0 would also oblige `check_step_references.rs`'s `BOOTSTRAP_FILES` to gain the file — turning a `framework/` change into a `runtime/` one with a version bump and a `ductus-v` tag.

`runtime/legacy-prose-commands.txt` is listed under `exclude` in `runtime/Cargo.toml:10-15` and is referenced from no `runtime/src/**` file and no `build.rs`, so it is neither compiled into the binary nor published in the crate. Editing it changes nothing an adopter receives — hence no version bump. It **is** in `framework-checks.yml`'s path filter and under `runtime/**` for `runtime.yml`, so the commit runs the full cargo suite in the pre-commit hook and both workflows in CI.

### The allowlist header is corrected in the same change

That file's header asserts *"Each non-blank, non-comment line is a repo-relative path to a `framework/commands/*.md` file"*. Adding a `framework/bootstrap/` entry falsifies it. Correcting the header is part of the same edit rather than a follow-up: a shipped claim that goes false in the commit that falsifies it is the stale-prose failure `AGENTS.md` §Workflow records, and leaving it would be committing that defect knowingly.

### `govern.md` is re-copied, and gains no counterpart

Audit Family 21 holds `framework/bootstrap/govern.md` byte-identical to `ductus.md`, and `scripts/audit/transitional-bootstrap-parity.sh`'s header states why: an unmigrated adopter's self-update byte-compares against that path and **writes whatever it finds** over their installed command. So it must be a copy of the post-split installed half — `cp framework/bootstrap/ductus.md framework/bootstrap/govern.md` in the same commit.

No `govern-procedure.md` is created. Such a run reaches the archive half through the archive, which is one tree for every adopter, by the path the installed half names.

### Verification is by running the consumers, not by reading the assignment

Ten consumers were classified in the spec. Task 5 re-earns that classification by executing each family and confirming a non-empty extraction, because `adopter_destinations` fails toward an **empty set** rather than an error: if the manifest tables had moved, it would have silently stopped suppressing adopter-destination findings, and reading the table would never have shown it.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/bootstrap/ductus-procedure.md` | Create | The archive half — nine sections, frontmatter, H1, back-pointer |
| `framework/bootstrap/ductus.md` | Modify | Excise five blocks; add one `## The archive half` pointer; repair prose the move falsifies |
| `framework/bootstrap/govern.md` | Modify | `cp` of the post-split `ductus.md` (Family 21) |
| `runtime/legacy-prose-commands.txt` | Modify | Allowlist the archive half; correct the header's `framework/commands/*.md`-only claim |
| `specs/056-bootstrap-archive-boundary-split/plan.md` | Modify | Record the post-split measurement (AC11) |

Not changed, and each checked rather than assumed: every `scripts/audit/*.sh`, every `runtime/src/**` file, `install.sh`, `README.md`, and `docs/shared-constitution.md`. The three prose anchors into this file (`README.md:239`, `README.md:306`, `docs/shared-constitution.md:169`) resolve to `#project-configuration` and `#agent-registry`, both of which stay installed.

## Trade-offs

**Considered and rejected — cut at `§Collect Project Inputs` on section order.** Yields a 57% cut and is wrong: `§Instructions` step 1 builds `manifest-entries` and the project inputs before the step-2 fetch, so the manifest tables and the inputs section are pre-extraction data. Taking it would have moved the Shared Files tables out from under `adopter_destinations`, which fails toward an empty set and would have degraded silently.

**Considered and rejected — give the archive half an `## Instructions` section.** Scores exit 0 on the parseability lint with no allowlist entry, but obliges `check_step_references.rs` to cover it, which makes this a `runtime/` change carrying a version bump, a `ductus-v0.49.8` tag, and a 022 scenario. Rejected as a large cost paid to avoid a one-line allowlist entry, and as dishonest besides: nothing dispatches from the archive half.

**Considered and rejected — place the archive half outside the `framework/bootstrap/*.md` glob** (e.g. `framework/bootstrap/procedure/ductus.md`). Defensible on the merits — the file is not exec-reachable, so the lint's stated rationale does not apply — but dodging a check by directory reads later as an oversight rather than a decision.

**Considered and rejected — split `§Edge Cases` across both halves.** Its entries describe both halves, so splitting it is the only way to put each entry beside its subject. Rejected: nothing dispatches from it, it is a reader's index, and two partial copies of one list is the drift shape this framework exists to prevent. It moves whole.

**Known limitation — the cut is 26.3%, not the ~40% this was scoped at.** `§Project Configuration` (12,627 B) and `§File Fetching` (5,165 B) both turn out to be pre-extraction reads. The saving is real — 38,616 bytes off every curl, install, context load and byte-compare — but smaller than the backlog item estimated, and the spec records why.

**Known limitation — nothing detects a future section landing on the wrong side.** The boundary is prose discipline: no check asserts that a section in the installed half reads nothing from the extracted tree, or that one in the archive half is never needed before extraction. A new section can be added to either file and nothing will object. Stating it rather than implying a mechanism that does not exist, per [§design-principles](../../framework/constitution.md#design-principles).
