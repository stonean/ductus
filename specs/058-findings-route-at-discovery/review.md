---
spec: 058-findings-route-at-discovery
last-run: 2026-10-31T01:16:01Z
reviewed-against: afb8a865b3d62ef7a02ff52fa64b3234bb7cefe6
diff-base: 797799ba1b61a055d662fab06ef5a7d51965d65c
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 13
scope: 56
skipped-passes: []
reviewed-digest:
  data-model.md: c7e234cb3bb24c79adb55d9b627fe41f2f346388a8644637c34cacdfa20cdb2c
  scenarios/analysis-drift-judges-the-record-it-writes.md: 9181d8ef6a4ad54cf2847a1fdac96d42ab509b598079c4901a17471f5b8d6b56
  scenarios/analyze-findings-match-decisions-by-host-judgment.md: d14bea60a4dfdba81edc5180b77367629154699b6808fda0739ab8a902776660
  scenarios/analyze-state-drift-judges-the-record-it-writes.md: 97833e416068dabe61138bc0e644535f9c2bbfdb065a7cce89d0b7a22180a607
  scenarios/auto-records-disposition-tasks-without-pausing.md: 7d09684e1a17de3c9aee551cada7245b9e1f7b640f643a8c35b7d96166190da0
  scenarios/findings-cost-in-proportion-to-risk.md: 7ff3dc556498c6147e5e27d651bb181b04d2263e7ddad7ed983dec039329cbba
  scenarios/only-unreadable-targets-retain-decisions.md: 76a12d565f6e1056f780906fed41de7d27e4a23c314232eddca0563026bcc3e1
blocking: false
dispositions:
  fixed: 0
  routed: 0
  discarded: 15
  undispositioned: 0
decisions:
  - key: "perf: validate-frontmatter reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs`"
    outcome: discarded
    reason: bounded to two small files per spec; the decisions list is read apart from the record by design
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "reuse: write_analysis::plain re-implements the shared bullet-marker stripping — `runtime/src/primitives/write_analysis.rs`"
    outcome: discarded
    reason: "predates 058: moved from render_captured_plain, not introduced by it"
    decided-at: 2026-09-25T20:05:21Z
    decided-by: andy@stone.dev
  - key: "simplicity: render_list_entry takes a base indent every caller passes empty — `runtime/src/primitives/write_review.rs`"
    outcome: discarded
    reason: predates 058 on render_waivers_at; no behavior depends on it
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "simplicity: describe and disposition_suffix fall back on a companion already validated — `runtime/src/primitives/process_decisions.rs`"
    outcome: discarded
    reason: DecisionRef is a wire type; carrying the companion inside the outcome variant would change its schema
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "reuse: append_task.rs parses a task heading two ways; the slug guard's grammar predates 058 — `runtime/src/primitives/append_task.rs`"
    outcome: discarded
    reason: predates 058, not introduced by it
    decided-at: 2026-09-26T13:34:14Z
    decided-by: andy@stone.dev
  - key: "convention: check-review-gate, check-unfolded-specs, dashboard, and prune-tasks return MissingFrontmatter and UnclosedFrontmatter without naming either under # Errors (predates 058) — `runtime/src/primitives/check_review_gate.rs`"
    outcome: discarded
    reason: predates 058; a doc-only omission, and each variant's own message names the file and the defect
    decided-at: 2026-09-26T18:25:59Z
    decided-by: andy@stone.dev
  - key: "other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md`"
    outcome: discarded
    reason: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: loose \"gets a disposition\" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md`"
    outcome: discarded
    reason: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md`"
    outcome: discarded
    reason: "pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion"
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md`"
    outcome: discarded
    reason: the Context records the state when the scenario was found, and the close is loose, not false
    decided-at: 2026-10-27T06:47:44Z
    decided-by: andy@stone.dev
  - key: "reuse: the blank-line removal rule is written out four times, in inbox-guidance-refresh step 4, remove-legacy-constitutions-region step 7 and strip-inbox-guidance steps 5 and 6 — `framework/migrations/strip-inbox-guidance.md`"
    outcome: discarded
    reason: each procedure is applied once per repository from one archive, and editing an applied procedure reaches no adopter, so the copies cannot drift in effect
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "edge: a `~~~` fence would hide a placeholder from the removal while iter_bullets, which knows only backtick fences, still counts it — `framework/migrations/strip-inbox-guidance.md`"
    outcome: discarded
    reason: "contrived: no template ever put the placeholder in a fence"
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "edge: step 8's item count may treat nested or tab-separated bullets differently from iter_bullets — `framework/migrations/strip-inbox-guidance.md`"
    outcome: discarded
    reason: it feeds only the report line, and nothing gates on it
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "quality: a deterministic, byte-exact rewrite stays prose for the host to walk although it meets §runtime-boundary's eligibility test — `framework/migrations/strip-inbox-guidance.md`"
    outcome: discarded
    reason: "pipeline machinery: 027's registry contract defines prose procedures, every sibling entry is prose, and a real adopter bootstrap verified this one on three subjects"
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
  - key: "other: AC35 names the former placeholder item in the singular while the procedure removes two forms — `specs/058-findings-route-at-discovery/spec.md`"
    outcome: discarded
    reason: "the criterion stays true: each form is still removed only as its exact text"
    decided-at: 2026-10-31T01:16:01Z
    decided-by: andy@stone.dev
---

# Review — 058-findings-route-at-discovery

## Summary

Five passes over 058's window since `797799ba`, passed as `--since`: `bca9a2fd` reopened 006 and 058 together, and `specs/006-bug-workflow/review.md` reviewed that shared window in depth at `797799ba`, so this run reads only what changed since, plus 058's own artifacts and deliverable (AGENTS.md). An independent agent did the passes and re-read the fix round; its claims were re-checked here.

Examined 13 of 56 scope entries: `framework/migrations/strip-inbox-guidance.md`, `framework/migrations/inbox-guidance-refresh.md`, `framework/bootstrap/ductus-procedure.md` and 058's `spec.md`, `plan.md` and `tasks.md` in full; `framework/migrations.toml` (lines 1-41 and 225 onward as text, every entry's id, `introduced_in`, `sunset_after` and `target_paths` parsed); `specs/006-bug-workflow/spec.md` (`4bf2e236`'s status line only); and 013's `plan.md`, `spec.md`, `data-model.md`, `tasks.md` and scenario `the-precheck-ignores-what-the-same-run-wrote`, whose passes are 013's record. 058's `data-model.md` and six scenarios were read too and sit outside the scope. Not read, 43 entries: 26 unchanged since the base (`git diff --quiet 797799ba` per entry), among them all 18 `runtime/` entries, `.claude/commands/ductus/*.md`, `AGENTS.md`, `docs/analyze.md`, the `framework/commands/` glob, `framework/constitution.md`, `framework/runtime-tools.txt`, `framework/templates/project/inbox.md` and `version`; 7 rule files, loaded as this run's rules rather than reviewed (accessibility-frontend, api-backend, configuration-cross, performance-backend, performance-frontend, security-backend, security-frontend); 7 entries of other specs' work in `9b067399`, `224d63f5` and `afb8a865` (008's `spec.md` and `tasks.md`, 017's `data-model.md`, 034's, 068's and 070's `spec.md`, and the `specs/{008,020,022,047,050,054,057}-*/` glob, changed only under 008); the two review records this pass writes, 058's own and 013's; and `resume.txt`, an untracked file another session left at the root.

Rules: every rule in the eleven files was loaded by heading, Statement and trigger; `quality-cross.md`, BE-INPUT-003 and BE-INPUT-004 were read in full as the ones whose triggers reach a host-walked migration procedure, and none fires.

Fixed in the run: `strip-inbox-guidance` removed the former placeholder only in its checkbox form, while the bootstrap installed `- {Brief description of the issue and any relevant context}` from `main` between `7b22162c` and `ef2b0cf2`, before any release, and every inbox primitive counts that line as an item; step 2 now removes both exact forms (a correctness defect, no rule). The re-read of that fix found the Background's "Since then" pointing at 2026-05-02 rather than 0.60.0, also fixed; that round was prose-only and is not re-read. A model of both inbox migrations over all ten template shapes the repository shipped, with LF and CRLF endings, with and without a logged item and a final newline, converges and is idempotent; the real adopter bootstrap at `89e6f02b` (`09b2a408`) verified the released shapes on three subjects.

Observations: five new, each discarded with the operator's confirmation; ten stored decisions carried with their text and disposition, each re-checked against the current tree, the analyze-state-drift scenario's among them because 058's artifacts were read; five expired because their paths are outside this run's scope (`decisions.rs` twice, `server.rs`, `framework/reference/implement.md`, `framework/reference/review.md`).

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Observations

- perf: validate-frontmatter reads each record file twice — `runtime/src/primitives/validate_frontmatter.rs` — **discarded**: bounded to two small files per spec; the decisions list is read apart from the record by design
- reuse: write_analysis::plain re-implements the shared bullet-marker stripping — `runtime/src/primitives/write_analysis.rs` — **discarded**: predates 058: moved from render_captured_plain, not introduced by it
- simplicity: render_list_entry takes a base indent every caller passes empty — `runtime/src/primitives/write_review.rs` — **discarded**: predates 058 on render_waivers_at; no behavior depends on it
- simplicity: describe and disposition_suffix fall back on a companion already validated — `runtime/src/primitives/process_decisions.rs` — **discarded**: DecisionRef is a wire type; carrying the companion inside the outcome variant would change its schema
- reuse: append_task.rs parses a task heading two ways; the slug guard's grammar predates 058 — `runtime/src/primitives/append_task.rs` — **discarded**: predates 058, not introduced by it
- convention: check-review-gate, check-unfolded-specs, dashboard, and prune-tasks return MissingFrontmatter and UnclosedFrontmatter without naming either under # Errors (predates 058) — `runtime/src/primitives/check_review_gate.rs` — **discarded**: predates 058; a doc-only omission, and each variant's own message names the file and the defect
- other: review's description and help row say it blocks done on MUST violations, while the gate also blocks an unwaived SHOULD — `framework/commands/review.md` — **discarded**: true of the review record, whose blocking flag is MUST-only, and 023's AC25 pins the exact text
- other: loose "gets a disposition" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md` — **discarded**: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
- other: 020's data model calls should-violations an advisory severity count rather than a post-waiver one — `specs/020-code-review/data-model.md` — **discarded**: pre-existing and not false: the row names the SHOULD count, and §Waivers and the §Expiry rule state the post-waiver exclusion
- other: the analyze-state-drift scenario's Context says only --fix ends the loop, and its close omits the reopen-by-route exit — `specs/058-findings-route-at-discovery/scenarios/analyze-state-drift-judges-the-record-it-writes.md` — **discarded**: the Context records the state when the scenario was found, and the close is loose, not false
- reuse: the blank-line removal rule is written out four times, in inbox-guidance-refresh step 4, remove-legacy-constitutions-region step 7 and strip-inbox-guidance steps 5 and 6 — `framework/migrations/strip-inbox-guidance.md` — **discarded**: each procedure is applied once per repository from one archive, and editing an applied procedure reaches no adopter, so the copies cannot drift in effect
- edge: a `~~~` fence would hide a placeholder from the removal while iter_bullets, which knows only backtick fences, still counts it — `framework/migrations/strip-inbox-guidance.md` — **discarded**: contrived: no template ever put the placeholder in a fence
- edge: step 8's item count may treat nested or tab-separated bullets differently from iter_bullets — `framework/migrations/strip-inbox-guidance.md` — **discarded**: it feeds only the report line, and nothing gates on it
- quality: a deterministic, byte-exact rewrite stays prose for the host to walk although it meets §runtime-boundary's eligibility test — `framework/migrations/strip-inbox-guidance.md` — **discarded**: pipeline machinery: 027's registry contract defines prose procedures, every sibling entry is prose, and a real adopter bootstrap verified this one on three subjects
- other: AC35 names the former placeholder item in the singular while the procedure removes two forms — `specs/058-findings-route-at-discovery/spec.md` — **discarded**: the criterion stays true: each form is still removed only as its exact text

## Skipped passes

*None.*

## Unexamined governance

*None.*
