---
spec: 076-waived-and-deferred-findings-are-distinct-dispositions
last-run: 2026-11-03T14:02:54Z
reviewed-against: 104cdf695d51f9bc9fd56e5f634a4b7455ad5c73
diff-base: a76c99e8b513fbc402644743ccc6a8ee7d9cd4f0
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 35
scope: 148
skipped-passes: []
reviewed-digest:
  data-model.md: c36e56e63ab29d0b9217eadf38f2ae9f976fdc6124d193e95e15dcdf82eb7909
blocking: false
dispositions:
  fixed: 25
  deferred: 1
  waived: 16
  undispositioned: 0
decisions:
  - key: "bug: the dedup keeps the higher-ranked duplicate's disposition, so a deferral only a dropped duplicate carried would be lost — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: "--defer names a rule ID and the dedup collapses only findings sharing that rule and file, so duplicates carry the same deferral; no caller supplies differing ones"
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "perf: each finding's existence check re-parses the stored list and re-reads a named tasks.md — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: bounded by one run's findings; the lists and task files are a few entries
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "simplicity: Buckets is a borrowed copy of Sorted, and the two writers' stored_decisions loops are alike — `runtime/src/primitives/write_review.rs`"
    outcome: waived
    reason: a refactor with no behavior change; each loop names its own argument paths
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "bug: releases compares a target's feature to the feature argument as raw strings, so 001-x/ would release its own task — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: "contrived: every caller passes the canonical name resolve-feature returns"
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: a deferral's target kind is not checked, so README.md counts as pipeline work — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: 076's data model names the target kinds and no refusal for another; a README.md target lies under no spec, releases nothing, and lapses the next run its finding fires
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "bug: a task with no checkboxes counts as complete, so a deferral to it lapses at once — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: the data model's completion row read literally; 0 of 957 tasks read-tasks parses across this corpus's 70 tasks.md files on 2026-11-03 have neither a subtask nor a done-when checkbox, and the failure re-asks or blocks rather than releasing
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "simplicity: Work::External is handled in the same arm as Pending — `runtime/src/primitives/process_decisions.rs`"
    outcome: waived
    reason: the variant documents why a work-item-only deferral never lapses; no behavior hangs on merging them
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: no gate-level test seeds a pre-076 routed/discarded map — `runtime/src/primitives/check_review_gate.rs`"
    outcome: waived
    reason: the gate reads the map through Dispositions, whose alias test covers parsing
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: the gate names a stored gating deferral whose task has since been checked without saying it may have lapsed — `runtime/src/primitives/check_review_gate.rs`"
    outcome: waived
    reason: the gate reports what the record marks gating; lapse is judged by process-decisions when the next review or analyze run reads the record, and the gate's guidance sends the operator to re-run
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: the gate reads a record's release as last written, so another spec's work completed since the last run leaves its finding released until a run re-judges it — `runtime/src/primitives/check_review_gate.rs`"
    outcome: waived
    reason: "the gate reports what the record states; release and lapse are judged when a review or analyze run writes the record, by design (076's data model: derived on every run, never stored), and the next run re-judges it"
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: a work-item ID accepts vertical tab and form feed — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: "contrived: no tracker issues an ID holding either; the record escapes both and every report flattens them, so neither can break the record or a report line"
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: a hand-edited deferral naming a task in a nested tasks.md is judged by its spec's status — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: only a hand-edited record can reach it; the writers refuse that target
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: a stored target with .. lapses as complete rather than reading as unreadable — `runtime/src/primitives/decisions.rs`"
    outcome: waived
    reason: it names no feature directory under the spec root, so it is judged as lying outside every spec, as a rule file is, and nothing is opened; the writers refuse a traversal
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: the gate's outstanding-findings guidance lists this spec's own task and an external work item alone as deferrals that release nothing, omitting a rule file and complete or unreadable work — `runtime/src/primitives/check_review_gate.rs`"
    outcome: waived
    reason: the guidance is true as written and names the cases an operator reaches for; the full release condition is stated once, in the review reference's §Deferrals, which the gate's references point at
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: 076's plan Affected Files omits docs/rules.md and the 058, 066, 020 and 050 scenarios the sweep edited — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md`"
    outcome: waived
    reason: the review scope unites Affected Files with every file modified since the diff base, which holds each of them, and the plan's Overview names the by-meaning sweep
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "other: 076's plan does not state the same-work match rule or the expired-decisions refusal — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md`"
    outcome: waived
    reason: the plan's Data Model section points at the data model for the writer refusals, and the data model states both
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
  - key: "security: process-waivers' duplicate, retained and expired notices print a stored waiver's rule, file and reason as written — `runtime/src/primitives/process_waivers.rs`"
    outcome: deferred
    target: specs/066-audit-records-and-the-completion-gate/tasks.md
    task: "13"
    decided-at: 2026-11-03T14:02:54Z
    decided-by: andy@stone.dev
---

# Review — 076-waived-and-deferred-findings-are-distinct-dispositions

## Summary

0 MUST violations, 0 SHOULD violations, 1 low-confidence finding: BE-COMPAT-001, the 0.x rename of the `dispositions:` keys and outcome names, deliberate per `plan.md`'s Trade-offs and the CHANGELOG entry, with the old names still read from disk. The five passes (security, reuse, quality, efficiency, simplicity) ran on 2026-11-03 over base `a76c99e8` at `53a87a87`, in four read-only agent slices: `decisions.rs` with `process_decisions.rs`; the two writers; `check_review_gate.rs` with 076's smaller hunks; and the command, reference and CHANGELOG prose. The host re-checked each slice's claims, and two slices reproduced the main defect (a re-deferral to other work kept under its old entry) by driving the built binary. `examined: 35` counts the scope entries the passes read: 076's changes in nineteen runtime files, six command and five reference sources, the constitution, the CHANGELOG entry, and 076's `spec.md`, `plan.md` and `tasks.md`, of the 130 the base then resolved. The scope now resolves 148: the 18 added since are twelve from 077's commit `62814405` and another session's uncommitted edits (not 076's work, not examined), the 057 and 058 `tasks.md` handoff notes, 066's new scenario and task, 076's `data-model.md` as the fix rounds changed it, and 078's draft spec. Not read by the passes: the window's 077 work, 076's sweep edits to other specs' bodies, scenarios and data models, the generated mirrors, and `runtime/tests/`. Rule findings QUAL-CLAIM-001 (twice), QUAL-GROUND-001, QUAL-TEST-001 and BE-INPUT-004 were fixed. Fix rounds followed in `cc746fe6`, `251fef5e`, `03b0c9f5`, `57ec6a26`, `a0427653` and `104cdf69`. Each round that changed code or a durable contract was re-read by an independent agent, which read 076's and 058's data models and drove the built binary (those reads are not counted in `examined`), except a round that only applied a re-read's prescribed edits, which was checked by diff instead. A loop ended at a re-read with no MUST or SHOULD in its round, or whose remaining SHOULDs were prose only (fixed, not re-read) or outside the round (deferred); task 14's ended at the owner's two-re-read budget. 42 observations: 25 fixed, 16 waived with the owner's decisions of 2026-11-03, and 1 deferred to 066 task 13.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: BE-COMPAT-001 — the dispositions: keys and outcome names are renamed within 0.x

- **File**: `runtime/src/schema/primitives.rs:146-178`
- **Rule**: Within a major version, schema changes MUST be additive only. Removing or renaming a field, changing a field's type (including narrowing, e.g. `string` → `enum`), changing a default value, adding a required request field or making an optional request field required, ceasing to populate a response field the server previously populated, or adding a value to a response enum the schema does not document as open to new values MUST NOT happen without a major version bump.
- **Finding**: `Dispositions` renames `routed`/`discarded` to `deferred`/`waived` and `DispositionOutcome` renames its variants, which the Statement lists as breaking within a major version. Low confidence: the runtime is 0.x, the old names are still read from disk, and plan.md's Trade-offs and the CHANGELOG entry record the rename as deliberate.
- **Auto-fixable**: no

## Waived findings

*None.*

## Deferred findings

*None.*

## Observations

- bug: a waiver over a standing deferral for the same key was accepted, so the record held both dispositions — `runtime/src/primitives/write_review.rs` — **fixed**
- bug: a deferral to complete work, a done spec or a checked task, released its finding at once, and --defer named no reopen of a done target — `runtime/src/primitives/decisions.rs` — **fixed**
- bug: the gate's, implement's and analyze's prose stated the release condition and the exits loosely or wrongly — `framework/reference/implement.md` — **fixed**
- bug: review step 9 said a lapsed violation is re-proposed in step 10, which proposes observations only — `framework/commands/review.md` — **fixed**
- bug: a --waive carrying a --work-item was accepted rather than refused as an operator error — `framework/commands/review.md` — **fixed**
- bug: implement's disposition task could be checked on a blocking finding deferred to an external work item alone — `framework/reference/implement.md` — **fixed**
- other: §brownfield-inbox's release sentence read as the only way a gating finding leaves the counts — `framework/constitution.md` — **fixed**
- other: the groom reference said a finding's route is its deferral — `framework/reference/groom.md` — **fixed**
- other: the CHANGELOG entry's enumerations were narrower than the code: lapse, gating, --task, outcome: discarded — `runtime/CHANGELOG.md` — **fixed**
- other: review's Purpose called waivers the sanctioned escape, leaving deferral out — `framework/commands/review.md` — **fixed**
- convention: RawWaiverFull's doc named a ticket field no waiver carries — `runtime/src/primitives/write_review.rs` — **fixed**
- convention: the write-review MCP description said a waived violation is stored under decisions:, where only a deferred one is — `runtime/src/mcp/server.rs` — **fixed**
- other: the gate's unreadable-decisions branch was untested and its message omitted the path — `runtime/src/primitives/check_review_gate.rs` — **fixed**
- convention: the gate's disposition checks spelled the record filenames as literals, rebuilt the feature path and took a tuple parameter — `runtime/src/primitives/check_review_gate.rs` — **fixed**
- convention: DecisionRef.reason's doc said a reason belongs to a waiver only — `runtime/src/schema/primitives.rs` — **fixed**
- other: the generated review Summary omitted violations deferred to another spec, reading like a clean run — `runtime/src/primitives/write_review.rs` — **fixed**
- security: a work-item ID was screened for LF and CR only, so U+0085, U+2028 and U+2029 reached the report raw — `runtime/src/primitives/decisions.rs` — **fixed**
- convention: the data model's notice templates lacked the code span the code prints — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/data-model.md` — **fixed**
- reuse: process-decisions restated names_a_tasks_file, and refuse_missing_work derived a target's feature unlike target_feature — `runtime/src/primitives/process_decisions.rs` — **fixed**
- simplicity: decision_for's deferred filter was dead, since require_companion refuses first — `runtime/src/primitives/decisions.rs` — **fixed**
- convention: ProcessDecisionsResult.lapsed carried a lone serde default — `runtime/src/schema/primitives.rs` — **fixed**
- convention: two doc lines in decisions.rs ran to 119 columns — `runtime/src/primitives/decisions.rs` — **fixed**
- security: process-decisions' notices printed a stored key, target and waiver reason, and the gate's held-by-deferral suffix a stored key, as written — `runtime/src/primitives/process_decisions.rs` — **fixed**
- other: the analyze reference worded §implement-phase's deferral exit with the full release condition, which the constitution states only as a necessary one — `framework/reference/analyze.md` — **fixed**
- bug: a deferral to an unchecked task on a done spec released its gating finding, though a closed spec's gate never runs again — `runtime/src/primitives/decisions.rs` — **fixed**
- bug: the dedup keeps the higher-ranked duplicate's disposition, so a deferral only a dropped duplicate carried would be lost — `runtime/src/primitives/write_review.rs` — **waived**: --defer names a rule ID and the dedup collapses only findings sharing that rule and file, so duplicates carry the same deferral; no caller supplies differing ones
- perf: each finding's existence check re-parses the stored list and re-reads a named tasks.md — `runtime/src/primitives/decisions.rs` — **waived**: bounded by one run's findings; the lists and task files are a few entries
- simplicity: Buckets is a borrowed copy of Sorted, and the two writers' stored_decisions loops are alike — `runtime/src/primitives/write_review.rs` — **waived**: a refactor with no behavior change; each loop names its own argument paths
- bug: releases compares a target's feature to the feature argument as raw strings, so 001-x/ would release its own task — `runtime/src/primitives/decisions.rs` — **waived**: contrived: every caller passes the canonical name resolve-feature returns
- other: a deferral's target kind is not checked, so README.md counts as pipeline work — `runtime/src/primitives/decisions.rs` — **waived**: 076's data model names the target kinds and no refusal for another; a README.md target lies under no spec, releases nothing, and lapses the next run its finding fires
- bug: a task with no checkboxes counts as complete, so a deferral to it lapses at once — `runtime/src/primitives/decisions.rs` — **waived**: the data model's completion row read literally; 0 of 957 tasks read-tasks parses across this corpus's 70 tasks.md files on 2026-11-03 have neither a subtask nor a done-when checkbox, and the failure re-asks or blocks rather than releasing
- simplicity: Work::External is handled in the same arm as Pending — `runtime/src/primitives/process_decisions.rs` — **waived**: the variant documents why a work-item-only deferral never lapses; no behavior hangs on merging them
- other: no gate-level test seeds a pre-076 routed/discarded map — `runtime/src/primitives/check_review_gate.rs` — **waived**: the gate reads the map through Dispositions, whose alias test covers parsing
- other: the gate names a stored gating deferral whose task has since been checked without saying it may have lapsed — `runtime/src/primitives/check_review_gate.rs` — **waived**: the gate reports what the record marks gating; lapse is judged by process-decisions when the next review or analyze run reads the record, and the gate's guidance sends the operator to re-run
- other: the gate reads a record's release as last written, so another spec's work completed since the last run leaves its finding released until a run re-judges it — `runtime/src/primitives/check_review_gate.rs` — **waived**: the gate reports what the record states; release and lapse are judged when a review or analyze run writes the record, by design (076's data model: derived on every run, never stored), and the next run re-judges it
- other: a work-item ID accepts vertical tab and form feed — `runtime/src/primitives/decisions.rs` — **waived**: contrived: no tracker issues an ID holding either; the record escapes both and every report flattens them, so neither can break the record or a report line
- other: a hand-edited deferral naming a task in a nested tasks.md is judged by its spec's status — `runtime/src/primitives/decisions.rs` — **waived**: only a hand-edited record can reach it; the writers refuse that target
- other: a stored target with .. lapses as complete rather than reading as unreadable — `runtime/src/primitives/decisions.rs` — **waived**: it names no feature directory under the spec root, so it is judged as lying outside every spec, as a rule file is, and nothing is opened; the writers refuse a traversal
- other: the gate's outstanding-findings guidance lists this spec's own task and an external work item alone as deferrals that release nothing, omitting a rule file and complete or unreadable work — `runtime/src/primitives/check_review_gate.rs` — **waived**: the guidance is true as written and names the cases an operator reaches for; the full release condition is stated once, in the review reference's §Deferrals, which the gate's references point at
- other: 076's plan Affected Files omits docs/rules.md and the 058, 066, 020 and 050 scenarios the sweep edited — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md` — **waived**: the review scope unites Affected Files with every file modified since the diff base, which holds each of them, and the plan's Overview names the by-meaning sweep
- other: 076's plan does not state the same-work match rule or the expired-decisions refusal — `specs/076-waived-and-deferred-findings-are-distinct-dispositions/plan.md` — **waived**: the plan's Data Model section points at the data model for the writer refusals, and the data model states both
- security: process-waivers' duplicate, retained and expired notices print a stored waiver's rule, file and reason as written — `runtime/src/primitives/process_waivers.rs` — **deferred** to `specs/066-audit-records-and-the-completion-gate/tasks.md` task 13

## Skipped passes

*None.*

## Unexamined governance

*None.*
