---
section: "Check Families"
---

# Families-follow-the-command-reference-split

## Context

[071 — Command reference split](../../071-command-reference-split/spec.md) moved each pipeline command's `## Markdown-only reference` out of `framework/commands/<command>.md` into `framework/reference/<command>.md`, which `/ductus` installs at `.ductus/reference/<command>.md`, one copy shared by every agent. Several families read their subject out of that section. Left alone, two would have failed (Family 18 could not find its restatement, Family 28 its family list). Two more would have kept passing while no longer covering the moved text: Family 4 scanned only `framework/commands/`, and Family 39 counted a command's pointer into its reference file without judging it.

## Behavior

- **Family 4** (`placeholder-roundtrip.sh`) scans `framework/reference/*.md` beside the command sources, under a stricter rule: a reference file is shared by every agent, so `{cli-config-dir}` and every agent's config directory (`.claude/`, `.augment/`, `.agents/`, `.opencode/`, `.pi/`) are findings there. A reference file inherits its command file's file-scope ignore marker, so the maintainer-only `audit` command states it once.
- **Family 16** (`installer-command-parity.sh`) gains a reference arm over the Shared Files rows: a shippable reference with no row, a row with no source, a maintainer-only command's reference in the manifest, a row installing anywhere but `.ductus/reference/<command>.md`, and a reference file with no command are each a finding.
- **Family 18** (`marker-list-parity.sh`) reads the adopter-facing restatement from `framework/reference/analyze.md`. It still cannot become a pointer: it ships to adopters, as `.ductus/reference/analyze.md`.
- **Family 28** (`audit-family-parity.sh`) reads the documented family list from `framework/reference/audit.md`. The generated copy it does not read is now `.ductus/reference/audit.md`.
- **Family 34** (`step-reference-integrity.sh`) is unchanged in behavior. Its subject was always the Instructions section alone, and the sub-procedures that restart at 1 are now in files outside the subject rather than in a section of the same file. Measured 2026-10-25 after the move: 83 `step N` mentions outside Instructions across 19 command and bootstrap files, 0 findings.
- **Family 39** (`shipped-anchor-resolution.sh`) reports `resolve-anchor`'s new `cross-document` count on stderr and turns an unresolved `cross-document` reference into a finding that names the shipped document, not the constitution. The judgment itself is 067's ([`pointers-into-shipped-files-are-judged`](../../067-analyze-and-audit-checks/scenarios/pointers-into-shipped-files-are-judged.md)).

Each changed family was proven against a seeded defect before 071 landed: a planted `{cli-config-dir}` and `.opencode/` in a reference file (4), a missing row, a stale row, a shipped maintainer-only reference, a wrong destination and an orphan reference file (16), the restatement sentence removed (18), a family entry removed and the whole list emptied (28), and a misspelled pointer (39).

## Edge Cases

- `audit`'s reference is maintainer-only and in no Shared Files row, so a `§` pointer into it would stay `qualified`, unjudged by Family 39. None exists: `audit.md` names the file as a whole. Family 4 skips it through the command's file-scope marker, and Family 28 reads it as its subject.
- A command with no reference (`amend`, `target`, `link`, `help`) has no reference file and is in no family's reference arm.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
