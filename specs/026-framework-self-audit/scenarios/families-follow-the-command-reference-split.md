---
section: "Check Families"
---

# Families-follow-the-command-reference-split

## Context

[071 — Command reference split](../../071-command-reference-split/spec.md) moved each pipeline command's `## Markdown-only reference` out of `framework/commands/<command>.md` into `framework/reference/<command>.md`, which `/ductus` installs at `.ductus/reference/<command>.md`, one copy shared by every agent. Several families read their subject out of that section. Left alone, two would have failed (Family 18 could not find its restatement, Family 28 its family list). Two more would have kept passing while no longer covering the moved text: Family 4 scanned only `framework/commands/`, and Family 39 counted a command's pointer into its reference file without judging it.

## Behavior

- **Family 4** (`placeholder-roundtrip.sh`) scans `framework/reference/*.md` beside the command sources, under a stricter rule: a reference file is shared by every agent, so `{cli-config-dir}`, every agent's config directory, and `$ARGUMENTS` are findings there; `$ARGUMENTS` because a host substitutes it only into the command body it loads, so in a reference file it would stay literal. The agent directories are derived from the Agent Registry by `agent_config_dirs` in `scripts/audit/lib.sh`, the helper Family 17 now reads its own set through, so a newly registered agent is checked without an edit here, and an empty derivation is a finding rather than a pass. A reference file inherits its command file's file-scope ignore marker, so the maintainer-only `audit` command states it once.
- **Family 16** (`installer-command-parity.sh`) gains a reference arm over the Shared Files rows: a shippable reference with no row, a row with no source, a maintainer-only command's reference in the manifest, a row installing anywhere but `.ductus/reference/<command>.md`, and a reference file with no command are each a finding. Only a reference whose command exists is expected to have a row, so an orphan reference file yields one finding, the orphan, and not also a missing-row finding whose remedy would contradict it.
- **Family 18** (`marker-list-parity.sh`) reads the adopter-facing restatement from `framework/reference/analyze.md`. It still cannot become a pointer: it ships to adopters, as `.ductus/reference/analyze.md`.
- **Family 28** (`audit-family-parity.sh`) reads the documented family list from `framework/reference/audit.md`. The generated copy it does not read is now `.ductus/reference/audit.md`.
- **Family 34** (`step-reference-integrity.sh`) keeps its subject, the Instructions section alone, and the sub-procedures that restart at 1 are now in files outside the subject rather than in a section of the same file. Measured 2026-10-25 after the move: 83 `step N` mentions outside Instructions across 19 command and bootstrap files, 0 findings. The pointer rewrite still changed what it reads: three of `implement.md`'s Instructions references became pointers of the form `step 5 of **§…**`, each numbering a step of a section in implement's reference file, and `check-step-references` resolved them against implement's own step 5. Since 071's review a `step N of §X` reference is qualified, like `X's step N`, so it is neither resolved nor counted, and the out-of-subject count is 80 (scenario [`family-34-step-reference-integrity`](family-34-step-reference-integrity.md)).
- **Family 39** (`shipped-anchor-resolution.sh`) reports `resolve-anchor`'s new `cross-document` count on stderr and turns an unresolved `cross-document` reference into a finding saying it names no heading of the shipped document it points into: an explicit pair is judged against that document alone, and otherwise a heading of the citing file or a constitution marker would also have resolved it. The judgment itself is 067's ([`pointers-into-shipped-files-are-judged`](../../067-analyze-and-audit-checks/scenarios/pointers-into-shipped-files-are-judged.md)).

Each changed family was proven against a seeded defect before 071 landed: a planted `{cli-config-dir}` and `.opencode/` in a reference file (4), a missing row, a stale row, a shipped maintainer-only reference, a wrong destination and an orphan reference file (16), the restatement sentence removed (18), a family entry removed and the whole list emptied (28), and a misspelled pointer (39).

## Edge Cases

- `audit`'s reference is maintainer-only and in no Shared Files row, so a `§` pointer into it would stay `qualified`, unjudged by Family 39. None exists: `audit.md` names the file as a whole. Family 4 skips it through the command's file-scope marker, and Family 28 reads it as its subject.
- A command with no reference (`amend`, `target`, `link`, `help`) has no reference file and is in no family's reference arm.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
