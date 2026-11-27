---
status: done
dependencies: []
next-criterion: 16
---

# 047 — Analyze Findings Durability

`/{project}:analyze` persists the findings it produces, so an audit's results
survive the session that ran it.

> **Signpost:** [057 — Analyze artifact and record relocation](../057-analyze-artifact-and-record-relocation/spec.md) makes the analyze record the frontmatter of `analysis.md`, and the review record `review.md`'s, so `spec.md` carries neither. [058 — Findings route at discovery](../058-findings-route-at-discovery/spec.md) gives every live finding a disposition in the run instead of an inbox entry. A criterion below that names the record reads against `analysis.md`.

## Motivation

`/{project}:review` had a durable sink: it wrote `review.md` and stamped the
spec's `review:` frontmatter, so its findings stayed readable long after the
session ended and `check-review-gate` could act on them later.
`/{project}:analyze` had none. It was read-only by design, and its findings
existed only in the invoking session's output.

That asymmetry made a full audit the only way to recover what a previous audit
had already found. On 2026-08-03 a comprehensive sweep of this repo produced 21
`criterion-path-existence` findings; they were acted on in-session and never
written down, so nothing in git recorded that they existed. The user asked the
question that exposed it: *if I lost the session, that information would be
hidden until I run another comprehensive audit.*

The constitution already required capture — [§brownfield-inbox Automatic issue
capture](../../framework/constitution.md#finding-dispositions), which 058
replaced with Finding dispositions, said issues an agent surfaces MUST be
captured, not dropped, and named `/{project}:implement` and `/{project}:review`
as the surfacing gates. Two things were missing.
`/{project}:analyze` was not among those gates, and the section addressed an
agent noticing something *incidental to other work* — not a command whose
**primary output is findings**. A command that exists to produce findings and
then discards them was the failure the **Design
Principles** rule names directly: it relied on the operator remembering.

## Behavior

`/{project}:analyze` gives each finding still live when detection ends a disposition in the run (fixed, deferred, or waived with its reason) and records the outcome in `analysis.md`: a `dispositions:` map of counts, and a `decisions:` list holding each deferred or waived decision.

**What is covered.** Every finding the run produces that is still live when the run ends — hard-fail, blocking, and advisory alike. A finding fixed in the same run is recorded `fixed`, in no tier. Informational entries (the unexamined-target `skipped` list, cross-service reference unknowns) are **not** findings and get no disposition; they report what could not be examined rather than a defect to route.

**A decision's identity includes the finding's message.** A stored decision is keyed `{family} — {message}`. `ArtifactFinding` carries `family`, `severity`, `message`, and `path` — and `path` is the *citing artifact*, not the missing subject the finding is about. Keying on family plus `path` therefore merges every finding a single `spec.md` produces: measured against this repo, that collapses 21 findings into 8 keys, silently discarding spec 012's `specs/errors.md` and `specs/events.md` because `specs/system.md` was keyed first, and one of spec 008's two rule files for the same reason. The subject that distinguishes them appears only inside `message`, so the message is what makes the key faithful. The cost is that re-wording a check's message asks about its findings once more; losing distinct findings is the worse failure, and this trade is measured rather than assumed. The key omits a category, which is a host's judgment that does not reproduce across runs.

**Idempotent across runs.** Analyze is an audit that operators re-run freely. A re-run over an unchanged tree counts each decided finding under its stored outcome without asking again, and no run touches `inbox.md`.

**The record precedes the report.** `analysis.md` is written before the report is rendered, so a run interrupted between the two still leaves the record.

## Constitution amendment

[§brownfield-inbox Finding dispositions](../../framework/constitution.md#finding-dispositions) names a `/{project}:analyze` finding live when detection ends, in any tier, among the findings that each get at most one disposition in the run that finds them, and its scope covers a command whose primary output is findings. A gating finding must have one before its spec can reach `done`.

## Acceptance Criteria

- [x] AC1: `framework/commands/analyze.md` carries, after detection and before the step that records the run, a fix-and-route step that gives each live finding a disposition and writes a gating finding's deferral only after `gate-confirm`, and the command's reference states the policy it applies.
- [x] AC2: `analysis.md`'s report lists each finding with its family, message, path, and disposition: the fixed file, the artifact or work item it was deferred to, or the waiver reason.
- [x] AC3: Re-running `/{project}:analyze` over an unchanged repo leaves `inbox.md` byte-identical, since no run touches it, and counts each decided finding under its stored outcome without asking again.
- [x] AC4: Two findings of the same family citing the same `spec.md` but naming different missing paths are two findings with two stored decisions: a stored decision's key is `{family} — {message}`, and the message names the missing path.
- [x] AC5: A finding resolved within the run no longer fires at the re-check, so the record, written after it, counts it in no tier and records it `fixed`, under Fixed in this run.
- [x] AC6: Informational entries — the `skipped` unexamined-target list and cross-service reference unknowns — are not findings: they carry no key and get no disposition.
- [x] AC7: §brownfield-inbox Finding dispositions names a `/{project}:analyze` finding among those that each get at most one disposition, and its scope covers a command whose primary output is findings.
- [x] AC8: The generated `{cli-config-dir}/commands/{project}/analyze.md` mirror matches its source, and the full markdown lint passes.
- [x] AC9: `/{project}:analyze` records every run in `analysis.md`'s frontmatter — `last-run`, `analyzed-against`, the three tier counts, `unexamined`, and a derived `blocking` — including on a clean run and an empty scope, so the record's absence is itself information: the file's absence is the never-analyzed state [§text-first-artifacts Frontmatter Schema](../../framework/constitution.md#frontmatter-schema) declares. A spec whose frontmatter does not parse receives no record. A run records once implementation is complete: `write-analysis` refuses while `tasks.md` holds an unchecked box outside the run's own deferral's task (`refuse_while_implementation_is_open`).
- [x] AC10: `/{project}:implement`'s pre-done gate blocks a spec whose `analysis.md` is absent, carries a null `last-run`, reports `blocking: true`, or carries a record gone stale (AC13), ordered after every check of `review.md`'s record and with no grandfather clause. An `analysis.md` that exists but carries no parseable record is **undeterminable**, never never-analyzed. Advisory findings and unexamined targets are reported in the gate's guidance only on an `analyze-findings` block, and never block.
- [x] AC11: The exempt population — `done` specs predating the record — is counted by `/{project}:audit` Family 37 against a committed high-water mark, which fails when the backlog grows (the gate has no grandfather clause, so growth means it was bypassed) and reports a baseline that has gone slack. Backfilling the record was rejected: it would assert a run that nothing on disk substantiates.
- [x] AC12: `/{project}:review` renders exactly one `analyze` row in every run's stdout summary — never-analyzed, superseded, current, or freshness-undeterminable — from the same subject-set digest comparison the gate uses, so a review that has just written `review.md` reports superseded rather than current. The row is advisory: the command's exit code and the review record's `blocking`, in `review.md`'s frontmatter, are unchanged by it. It renders on an empty scope, on a dimension-restricted run, and once per feature under `--all`.
- [x] AC13: `check-review-gate` blocks a spec whose analyze record is stale, ordered after `not-analyzed` and `analyze-findings`, with the blocked message naming the changed paths and directing the operator to re-run `/{project}:analyze`. A record carrying no `analyzed-digest` is reported as undeterminable rather than passed silently or diffed by sha.
- [x] AC14: Both surfaces derive staleness from one subject set — every `.md` under `specs/{feature}/`, `review.md` included — excluding `analysis.md` whole, because the run writes its body in the same call as its record, and excluding a mechanical sweep (§drift-prevention's uniform token substitution). `spec.md` is digested with its criterion checkboxes normalized, so a tick leaves the record current. One implementation and **one reference point** back both: the recorded digest of what the run read, compared to the same subject set now. The notice and the gate therefore return the same answer wherever they are asked. The reason for the exclusion is that a record written after its subjects are read can never digest itself. On a `done` spec the comparison also ignores an in-place edit to `plan.md` or `tasks.md`, which §spec-lifecycle gives no back-edge when it adds no unchecked task; the recorded digest still covers them (066's `a-done-specs-plan-edit-leaves-its-analysis-current`).
- [x] AC15: `/{project}:analyze` records `analyzed-digest` — a per-path digest of the subject set as the run read it from disk, `analysis.md` left out whole — and `analyzed-against` is recorded as provenance only, never as the staleness basis. A subject that could not be read is recorded as unreadable rather than digested as empty.

## Open Questions

*None — captured during specification.*

## Resolved Questions

- **Should analyze write a per-spec `analysis.md` artifact, mirroring `review.md`?** **Resolved: yes.** `review.md` exists because review is a **gate** whose artifact the pipeline reads back, and the analyze record became one when it gained its own gate (AC9-AC10). Two gate-bearing records in one file was the two-homes condition §drift-prevention rejects, so each record is the frontmatter of the artifact its own command writes, and analyze's is `analysis.md` (057). `analysis.md` is overwritten wholesale on every run, and 057's AC13 forbids a `- [ ]` item anywhere in its body **mechanically**, so it is never a second triage surface. The inbox is not where findings live: no gate reads it, so a finding parked there let its spec reach `done` looking finished. Each live finding gets a disposition in the run that finds it, and `analysis.md` records the outcome and stores each deferred or waived decision (058).
- **Should disposition be limited to advisory findings, since blocking ones already halt the pipeline?** **Resolved: no — every live finding, in every tier, gets one.** Blocking findings do get acted on, but they are equally lost when a session ends before anyone acts, and a partially-audited repo is precisely where the record matters most. The distinction also would not survive contact with `--all`, where a blocking finding on one spec sits beside advisory findings on forty-six others. Severity raises salience, not routing — the constitution's own words. A hard-fail or blocking finding cannot be waived, because it already gates `done`.
- **Does analyze's disposition step break its read-only contract?** **Resolved: no.** Detection never modifies an artifact it audits. After detection, the fix-and-route step writes a mechanical fix by rule and a gating finding's deferral only after `gate-confirm`, since 079 nobody is asked about a chore or a waiver. `--fix` remains the only flag that mutates a spec, and it only reverts a drifted `done` spec's status and edits no content.
