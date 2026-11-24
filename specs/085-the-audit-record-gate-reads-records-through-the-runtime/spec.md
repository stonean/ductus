---
status: draft
dependencies: [057-analyze-artifact-and-record-relocation, 084-reopen-for-owed-code-review-every-code-change]
cross-spec-impact: [057-analyze-artifact-and-record-relocation, 066-audit-records-and-the-completion-gate]
next-criterion: 6
---

# 085 — The audit-record gate reads records through the runtime

The adopter CI template's audit-record gate reads each `done` spec's review and analyze records through the runtime, never through a frontmatter parse of its own.

## Motivation

- `framework/templates/ci/adopter-generators.yml`'s step "Audit-record gate on done specs", which [057](../057-analyze-artifact-and-record-relocation/spec.md)'s AC17 states, read every spec's `spec.md` frontmatter and each `done` spec's `review.md` and `analysis.md` frontmatter through an inline python `frontmatter` function: top-level scalars, split at the first colon (read at `ea691221`). §runtime-boundary principle 3 rules out such a parse as a substitute for the runtime, and the template already acquired the runtime for its derive step.
- That parse was a second reader of the records `check-review-gate` reads on the way to `done`, so the two could disagree with nothing to report it.
- [084](../084-reopen-for-owed-code-review-every-code-change/spec.md) rejected the same parse for the CI steps it added to the template (its plan's Trade-offs), and left this step as it stood.

## Behavior

- The step calls one runtime primitive once over the corpus and parses no frontmatter or markdown itself. The primitive reads each `done` spec's two records by the reader `check-review-gate` uses.
- It keeps every property the step enforced: each record in one of three states, absent (never run), present, or present and unparseable (undeterminable, a finding); a present record with a null `last-run`, with no `blocking` field (absent is not `false`), or with `blocking: true` is a finding; a spec whose `spec.md` frontmatter cannot be read is a finding; the never-run set is bounded by the committed high-water marks (`audit-record-baseline.txt` in the project's `.ductus/`), zero when the file is absent and a finding when it is malformed; and the examined, `done` and exempt counts are reported on every run, an empty enumeration failing.
- It honours the job's commented `working-directory` and the configured spec root, and annotates each finding on its file from the repository root, as the step does (spec 059).
- The template's pinned runtime serves the primitive, so the pin moves in the release that ships it.

## Acceptance Criteria

- [ ] AC1: `framework/templates/ci/adopter-generators.yml`'s audit-record gate calls one runtime primitive over the corpus and parses no frontmatter or markdown of its own.
- [ ] AC2: The primitive reports each `done` spec's review and analyze record as absent, present or unparseable, and reports as a finding naming its file a present record with a null `last-run`, with no `blocking` field or with `blocking: true`, and a spec whose `spec.md` frontmatter cannot be read.
- [ ] AC3: The never-run set is held against the committed high-water marks, zero when the baseline file is absent, a malformed file is a finding, and the step fails when either exempt set exceeds its mark.
- [ ] AC4: The step reports the specs it examined, those at `done` and each exempt count with its mark on every run, and fails when it examined none.
- [ ] AC5: A test proves each finding fails the step with its condition present and passes without it, from a project at the repository root and from one in a subdirectory.

## Applicable Rules

- `QUAL-CLAIM-001` — an unparseable record, an absent `blocking` and an empty enumeration are findings, never a clean pass.
- `QUAL-DELEG-001` — replacing the step's parse with the runtime's reader keeps every property the step enforced.

## Open Questions

- Is the primitive new, or a corpus form of an existing one: `check-review-gate`'s record checks, or `check-owed-work`'s corpus form, which 084 adds to the same template?
- Does the runtime read the baseline file, or does the step keep reading it? It is plain text, not frontmatter or markdown, but a count the runtime compares against a mark it read itself cannot disagree with the mark.

## Resolved Questions

- **A new spec, built after 084's bar is read** (the agent's recommendation, adopted by the operator, 2026-11-24). The step predates 084, which the owner chose not to widen to cover it. A task on 057 would reopen it, and one on 066 would hold its close, one analysis away at `ea691221`, behind the build. Building now would run this spec's review and analysis inside the two weeks over which 084's bar (b) counts analysis runs per 100 commits, so this spec is clarified, planned and built only once that bar is read, from the commit that sets 084 `done` (084's Resolved Questions). Rejected: an inbox item, which §design-principles names no disposition; and a waiver, since no requirement puts the step out of scope.
