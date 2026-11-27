# review — reference

The numbered Instructions in the command file, `review.md`, are the procedure: the runtime's primitives own rule-file selection, waiver arithmetic, scope resolution and report scaffolding, and the five passes cross the boundary at the `performReview` extension point. The detail they apply is below.

**Where a named section lives.** A section named below with no heading here is in the command file, `review.md`, with the numbered steps this file cites. A bare `§name` is a heading here or a constitution marker; one with another file's path on its line is a heading there, unless it is a constitution marker. A reader arriving from another file, as the installer does for **§Resolve target and scope**, opens `review.md` as well when a section it needs is there.

## Resolve target and scope

1. Resolve the working feature from `--all` or the current `/ductus:target`.
   If neither yields a target, halt with `no target — run /ductus:target first`.
   `--project` needs no target: its subject is the code no spec claims, and
   **A project review** in the command file says what replaces items 2 and 3.
2. Read the spec frontmatter. If `status` is not in `{in-progress, done}`,
   halt with `review only runs against in-progress or done specs`.
3. Build the file scope per **Inputs** (in the command file). An empty scope
   (no implementation files yet, or none changed since a finished review)
   records 0 findings across all five passes. It skips items 4–5
   and the passes but not the criteria step (**§Criteria step**), and records
   `blocking: false` and exits `0` unless that step judges a criterion not met.
4. **Tech-stack alignment check**, unless the active config file (step 1)
   sets `[review] tech-stack-verified = true`.
   - Read `AGENTS.md`'s `Tech Stack` section and inspect the file scope
     (extensions, imports, runtime/manifest markers). A missing or empty
     section, or an inconsistency between documentation and code, halts the
     run with the [tech-stack-misalignment](#blocking-message) message and
     exits `1`.
   - On a successful check, prompt the operator once, through `gate-confirm`:
     _"Tech-stack alignment confirmed. Persist this so future runs skip the
     check? (Y/n)"_. `Y` writes the flag to the active config file; `n` or
     skip runs the check again on the next invocation. After a stack change
     the operator removes the line by hand: `/ductus:review` does not
     auto-reset.
5. Discover rule files: step 2's `discover-rule-files` applies what follows. It lists
   `framework/rules/*.md` in ductus's own repository, or `specs/rules/*.md` in
   adopter projects, and classifies each by basename suffix:
   - `*-backend.md` → backend surface
   - `*-frontend.md` → frontend surface
   - `*-cross.md` → cross-cutting (applies to every stack)
   - anything else → unrecognized: loads for every stack, with a notice, in
     every non-error case

   **Surface selection** reads `.ductus/config.toml` `[rules] surfaces` (see
   **Inputs** in the command file):

   - **A valid list** keeps the listed surfaces' files plus every `*-cross.md`,
     and _replaces_ the detected stack. The **empty list** means **cross-only**,
     distinct from unset: the operator declares "no surface rules, only
     cross-cutting ones," rather than asking to derive.
   - **A degenerate value** — a member outside `{backend, frontend}`, `"cross"`
     included, or a non-list — fails fast, per `CFG-ENV-008`. A valid member
     does not rescue an invalid one.
   - **Unset** falls back to the stack item 4 detected, which the host passes to
     the primitive, keeping the matching surface and every `*-cross.md`.

   The **disabled-rule-files filter** then reads
   `[[review.disabled-rule-files]]` (see **Inputs** in the command file). An
   entry's `file` naming a selected file removes it, with a notice carrying its
   `reason`. One naming a file the selection left out or no file at all, or
   repeating an earlier entry, changes nothing and gets a notice: a no-op entry
   becomes load-bearing if the project's stack changes, and a renamed file is
   not fatal. None is auto-pruned. A malformed entry halts the run before any
   rule file is selected, and is not skipped: the operator fixes it.

   The closing `loading rule files:` line names what was selected, so adopters
   can confirm it without parsing the report.

   A degenerate `surfaces` or a malformed entry is a configuration error that
   exits non-zero (`CFG-ENV-008`), never a review finding, so it does not set `blocking`.

## Load rules

Load these inputs inline as the authoritative review criteria:

- The rules of every rule file selected by the suffix-based discovery in item 5 of
  [Resolve target and scope](#resolve-target-and-scope), as step 2's
  `discover-rule-files` returns them under `rules`
- The rules of any rule file outside the rule-file directory (e.g., `docs/rules/internal-api.md`)
  referenced from `AGENTS.md`, passed as step 2's `extra-files` — see [Notes for adopters](#notes-for-adopters)
- `AGENTS.md` `Code Style`, `Testing`, `Gotchas`, and `Boundaries` sections
- The target spec's acceptance criteria, each judged by the criteria step (**§Criteria step**), and any `scenarios/*.md` files

These are the **only** sources of normative rules for the review. Do not
introduce review criteria from outside the project.

**A rule file its header gives to analysis is not loaded** (spec 020, AC17): `discover-rule-files`' description states the header test and the notice.

**Every loaded rule file has a pass that applies it to the code in scope.** The
runtime carries the mapping, which this list states
(`review_pass_for_rule_file`, a test holds the two together), and the request
each pass receives names its own files, every loaded file with its pass, and
the passes this run performs:

- `security-*.md` — the security pass.
- `performance-*.md` — the efficiency pass.
- `quality-cross.md` — the quality pass.
- every other loaded file, a design-time file whose header names review or a project-authored file with an unrecognized suffix among them — the quality pass.

A Statement whose Verification is a design-time commitment is applied by its pass only where the code in scope shows what it governs; a Statement the code does not reach records nothing. The reuse and simplicity passes apply no rule file. On a dimension-restricted run (`--security`), a loaded file whose pass did not run is named in the Summary as unapplied, never folded into a clean result.

## Run review passes

Run the five passes below. Dividing a wide scope among agents, follow each behavior across its files and drive it on fixtures, rather than giving each agent one kind of file: the worst defects lie between files that read fine alone. A scenario or data model in scope is checked whole, every concrete claim in it against the code, not only its changed lines, as the criteria step judges every criterion: a stale claim the window left alone otherwise waits for a review that reads its file. When a flag restricts dimensions (`--security`,
`--simplicity`, `--quality`), skip the unselected passes and record them as
`skipped` in the report. A pass's finding, an observation or a reviewer's claim is recorded only when its trigger is shown possible in the project, from its code, configuration, history or deployment shape, and its mechanism is checked against the tree; break things only in a clone while reviewers read. A case shown only by building it, an input or a layout the project cannot produce, is a false positive: recorded nowhere and fixed in no round (spec 020, AC18). Each finding names its **site**: the narrowest named unit enclosing it that a later run can name again — a function or method, a type, a configuration key path, a markdown heading — spelled as the file spells it, or `(top level)` outside any; a line number is never a site.

### Passes in a subagent

The passes read every loaded rule file and every in-scope file, which in the host's own context every later call of the session would read again. So a host that can start a subagent (Claude Code's Agent tool) runs steps 3 to 7.2 in one fresh subagent, and one that cannot (Pi) runs them itself. Brief it with the feature, the run's scope (`compute-review-scope`'s `scope`, or the file its `lists-file` names), the rule files step 2 loaded, any dimension-restricting flag, this section, **§Criteria step**, §Load rules (which maps each loaded rule file to its pass), the five pass sections below, the **Observations** rules under §Write `review.md` (what an observation is and how it is dispositioned), and the shapes `write-review` takes for `findings` and `observations`. It never tells the passes to leave out an issue `review.md`'s `decisions:` settles: a stored decision applies only to an observation that fires (step 9), and `process-decisions` expires one that does not fire on a file the run read. The subagent reads each rule file and each in-scope file whole, the remainder of any read a tool cut short included, and returns as its last message:

- the findings and the observations as JSON in `write-review`'s shapes, each with the evidence that its trigger can occur in the project (the code path, configuration, history or deployment shape that reaches it), and the criterion verdicts in its `criteria` shape, with each criterion it could not judge and why;
- apart from those, each finding and observation on a file the run's scope does not hold, marked as such: the passes read related files, so they can find an issue in another spec's code, and the write-review primitive refuses one (spec 086);
- each in-scope file it read, and each it did not, with why;
- for each rule file, the bytes it read against the file's size, and each pass that did not run.

The host checks the hand-back before using it, as it checks any reviewer's claim. A file read short, a rule file read below its size, or a pass that did not run is named in the Summary and kept out of `examined`, or the subagent is asked to finish; it never reads as a clean result (§design-principles). A finding or observation whose hand-back shows no trigger the project can reach is dropped, recorded nowhere. One on a file the scope does not hold, marked or not, goes to step 10, never to step 11's write. The host then continues at step 7.3 with what it was handed, opening a file itself only to check or fix a finding. Under `ductus exec` the walker asks for each pass in turn.

### Security pass

Walk every file in scope against the loaded security rules (the files **§Load rules** maps to this pass). For each finding,
record: rule ID, severity (MUST or SHOULD), file path, line range, site, the
rule text, and a one-sentence explanation of how the code violates it. **Do not
flag patterns that are not in the loaded rules** — the project's rule set is
authoritative.

**Report a defect under its owner.** Where a loaded rule's Verification yields
the case the code shows to another rule with a **Not flagged:** clause, as
`/ductus:analyze` applies it, the finding is reported under the owner, at
the owner's tier, and not under the yielding rule: one defect is one finding,
counted once toward blocking, needing one waiver. A load condition in the
clause ("where `security-backend.md` is also loaded") is judged against the
files this review loaded (the request's `loaded-rule-files`), so a
frontend-only review keeps the frontend rule's finding. The finding stays with
the yielding rule when the owner's file is not loaded (a file the project
excludes from review included) or the owner's pass is not among the request's
`passes-run`, so routing never leaves a defect reported by no rule. A clause
that only narrows the rule's scope and names no owner sets the case aside, as
at analyze; a case only a spec can show routes nothing. A waiver recorded under
the yielding rule's ID expires once the finding routes
(**§Per-run waiver processing**), and one already recorded under the owner's
ID applies.

That rule holds for every pass below, and it is not a reason to drop what you
noticed: anything real that matches no loaded rule is an **observation**, which
every pass may return alongside its findings — see the Observations rules under
[Write `review.md`](#write-reviewmd).

### Reuse pass

Identify logic that duplicates existing utilities or that should be extracted
into shared code. Cross-reference with `specs/system.md`, and the parts its
pointers name for the code under review (spec 087), for established patterns
and shared infrastructure, and name the parts read in the Summary. Severity is
SHOULD unless the duplication contradicts an explicit MUST in `AGENTS.md`
`Boundaries`.

### Quality pass

Detect bugs, missing error handling, unhandled edge cases, off-by-one errors,
and contract violations against `specs/errors.md` and the parts its pointers
name for the code under review (spec 087). Each finding carries a
`confidence` tier — `high` or `low` (the string the `write-review` contract
consumes, compared case-insensitively). A `low`-confidence finding is recorded
in the Low-confidence section regardless of severity and is excluded from the
blocking count; use it when the finding is plausible but unconfirmed.

### Efficiency pass

Flag N+1 queries, repeated work, unbounded loops over user-controlled input,
and other performance issues. Severity is SHOULD by default; promote to MUST
when the inefficiency is also a security concern (e.g. unbounded input is a
DoS vector covered by the security rules).

### Simplicity pass

Identify overengineering: premature abstraction, unnecessary indirection,
configuration that could be a constant, branches that are dead under the
current spec. Severity is SHOULD. If a simpler form is mechanically derivable,
mark the finding `auto-fixable`.

## Criteria step

Steps 7.1 to 7.3 judge every acceptance criterion of the spec, whatever the window holds (066's scenario `criteria-are-verified-in-review`): the criteria are the spec's contract, and the window only says what changed, so a criterion whose code no reopening touched is judged too.

- **A verdict per criterion, read against the code.** Read the code a criterion's claim names, in scope or not, and drive it where it can be driven: a test's name proves only that the test exists. A met verdict's `note` is its evidence, the path and symbol and the test that proves it; a not-met verdict's says what fails. A note is one line: `write-review` refuses one that spans lines, before it writes anything. A criterion that cannot be judged, for want of code, a test or a fetched source, gets no verdict and is named in the Summary: it is never ticked, and a review stays owed while it is not judged met.
- **On a `done` spec the owner decides first** (spec 084). A ticked criterion judged not met changes only once the owner, or an invocation told to decide, says which side is correct: when the code is, its text is corrected, it stays ticked and is recorded judged met with its new text, and the spec stays `done`; when the criterion is, it is unticked and the run reopens the spec. With nobody to ask, neither changes and the record blocks: under `ductus exec` the walker marks no not-met verdict on a spec step 7.1 read as `done`.
- **A not-met criterion is fixed or corrected, never waived or deferred.** It clears when the code is fixed and a review judges it met, or when the criterion is corrected or moved to the spec that owns the work (§implement-phase). The gate holds `done` while it stands unticked, so a waiver or a deferral would release nothing.

## Write `review.md`

Write the report to `specs/NNN-feature/review.md`. A scenario-targeted run writes the same spec-level path, with the scenario in the `scenario:` frontmatter field. A re-run supersedes the prior `review.md` wholesale.

```markdown
---
spec: 042-example-feature
last-run: 2026-05-10T14:32:00Z
reviewed-against: <head compute-review-scope returned>
diff-base: <sha compute-review-scope resolved>
must-violations: 0
should-violations: 3
low-confidence: 2
examined: 38
scope: 46
excluded: 4
skipped-passes: []
reviewed-digest:
  scenarios/retry.md: 3f2a…
blocking: false
dispositions:
  fixed: 1
  deferred: 1
  waived: 1
  undispositioned: 0
---

# Review — 042-example-feature

## Summary

<one paragraph: overall posture, count by severity, blocking status>

## MUST violations (blocking)

<one heading per finding; `*None.*` when the section is empty>

## SHOULD violations (advisory)

## Low-confidence findings

## Waived findings

## Deferred findings

<violations their deferral releases (§Deferrals), which leave the counts; `*None.*` when none>

## Acceptance criteria

<one line per criterion the criteria step judged, met or not met, with its note; `*No criterion was judged.*` for an empty `criteria` list, and no section when the run passed none>

## Observations

<one bullet per observation this run recorded, each with its disposition; `*None.*` when empty>

## Skipped passes

<`*None.*` when none>

## Excluded from scope

<one bullet per path the scope left out, and per path a fix round left out, with that round; `*None.*` when none>

## Unexamined governance

<one bullet per registered shared constitution this run could not read; `*None.*` when none>
```

**Unexamined governance** names any `[constitutions.*]` entry (spec 055) the run could not read, with its reason and, when it has one, its `description`, which tells an operator _which_ checkout they are missing. `write-review` resolves the registry itself, so a report cannot omit the section: a review that ran under fewer rules than the project's config declares says so (`QUAL-CLAIM-001`). `*None.*` covers both "none registered" and "all registered sources read", since in neither did anything go unexamined.

**`examined` and `scope` are what make a clean report mean something.** The counts alone cannot distinguish _the passes read the scope and found nothing_ from _the passes never ran_. `scope` is derived by the primitive, so the denominator cannot shrink to match what was read, and `examined` is the reviewer's claim. No check reads the pair: a person reads it at the completion gate. [Window and coverage](#window-and-coverage) says how each is counted.

The **Observations** section is the home for something the reviewer judged
real that maps to **no loaded rule**. Inventing a rule for it is not the answer
— the rule set is authoritative — and the free-text Summary is not either,
since the next run regenerates it. An observation is **not** a finding: it never
enters the violation counts, never affects `review.blocking`, and never changes
the exit code. Each entry carries its own one-line text and, optionally, the
path it anchors to. Lead the text with a category (`security` / `leak` /
`convention` / `bug` / `perf` / `other`): a `security` or `leak` lead renders
first, and the rest only help a reader route it.

**An observation that is design rationale is fixed in the run**, as a short why in a comment at the code, a chore.

**Two things a pass notices are no observation.** An edit that would change no
claim, rule or behavior, such as a stray comma or a hedge, is not reported. A
claim the passes could not re-check, because no fetched copy of its source was
at hand, is named with that source in the Summary's account of what went
unexamined, not under **Observations**: an unchecked claim is a gap in coverage,
which the Summary states, not a defect to disposition.

**Every observation takes a disposition before the record is written**
(§brownfield-inbox, Finding dispositions), and the section renders it beside
the observation.

**The rule decides, and nobody is asked, except for a broken contract** (spec
079): step 10 applies it. An observation that cannot be judged mechanical is
waived with its reason, unless it shows a broken contract, whose deferral to the
spec's own task holds `done` through that task's unchecked box. A person's
stored decision wins, and the rule does not re-decide it.
`/ductus:implement`'s disposition task is unchanged.

- **Fixed** — a chore: mechanical, and adding no durable requirement
  (§bug-handling's durability test). A chore is never counted fixed while its
  fix is not on disk.
- **Deferred** — a broken contract, with the operator's confirmation, to the
  spec's own task or the home the **§Groom decision tree** in `.ductus/reference/groom.md`
  chooses. An external work-item ID may stand beside that home, or in its place,
  as the operator gives it (spec 076). Only a deferral to another spec's work still
  to do releases a broken contract; any other holds `done` until a run finds it
  fixed. A declined spec creation leaves the observation
  waived or undispositioned, never deferred to a spec that does not exist.
- **Waived** — with the reason it is not addressed, stored with `decided-by:
  rule` when the rule decided it and the person's name when one did. It is the
  disposition that ends a loop: an observation about the pipeline's own machinery
  rather than the code under review, deferred to a `done` spec's work, reopens that
  spec while the work is owed, and that spec's next review produces the next observation.
- **Undispositioned** — nobody decided: the operator declined without a
  reason, a deferral's write failed, its stored deferral lapsed with nobody to
  ask, a broken contract had nobody to ask, or it came through a `ductus exec`
  run, whose walker no-ops steps 9 and 10. It stays recorded and visible for a
  later review to decide, and it does not hold `done` unless it is a broken contract.

**A person overrules the rule on an observation by deferring it or having it fixed.**
`--waive` and `--defer` take a rule ID, so they act on violations, and a waiver agreeing
with the rule's makes no new decision, so it stays the rule's.

**Decisions persist.** Each deferred or waived observation is stored in
`review.md`'s `decisions:` list, beside `waivers:`. Observation text is the
reviewer's own wording and does not reproduce byte for byte, so the next run
matches each new observation to the stored decision describing the same issue
and passes that decision's key as the observation's `decision-key` (step 9). A
missed match costs one repeated decision, never a silent waiver of a person's
call, because an unmatched decision is pruned rather than applied to something
else.

An observation whose subject later becomes a real rule finding needs nothing
special — the finding counts, and the observation is the reviewer's to drop on
the next run.

A violation is fixed, waived ([Waivers](#waivers)) or deferred ([Deferrals](#deferrals)); a fix is recorded by re-running `/ductus:review`, never by editing the report, which `write-review` regenerates whole.

Each finding follows this shape:

```markdown
### MUST: <rule-id> — <one-line summary>

- **File**: `path/to/file.ts:42-55`
- **Site**: <the narrowest named unit enclosing it, or (top level)>
- **Rule**: <verbatim rule text from the rule file (framework/rules/... or specs/rules/...)>
- **Finding**: <one to three sentences>
- **Auto-fixable**: yes | no
- **Suggested fix**: <code block or prose>
```

## Apply `--fix` (optional)

When `--fix` is set, after writing the report:

1. Apply every finding marked `auto-fixable: yes` whose severity is SHOULD,
   plus MUST findings whose suggested fix is purely mechanical (e.g. removing
   a hardcoded secret, adding a missing CSRF token attribute).
2. **Never** auto-apply what **§Auto-fix scope** rules out.
3. Re-run only the affected passes against the modified files. Update
   `review.md` with the post-fix counts.
4. Stage the modified files but do not commit. The user owns the commit.

## Fix-round loop

Step 12 re-reads the fixes this run made before the record is final (spec 078): step 10's chore fixes, `--fix`'s, and those a re-read prompts. It covers no other command's fixes: `/ductus:analyze`'s re-check reads its own, chores and the design edits it makes in the run (spec 067, AC6), and `/ductus:implement`'s land before the review whose passes read them.

- **A round is derived, never stated.** A deferral's write is a round's path like any other write. Another session's uncommitted edits cannot be told apart and stay in the round.
- **The loop ends when a round is `empty`**: after a clean re-read, or a matched check with `open` at zero, nothing has changed since the last base. A NOTE fixed afterwards is a change, so it is a round.
- **An exempt path is not re-read.** `/ductus:analyze` reads each such edit again before `done`. A round with no path left is recorded `exempt` and the Summary names it.
- **The brief is the runtime's.** Hand the independent agent the `brief` path alone, as step 12 says; a result file from the operator's fresh session is read as an agent's.
- **Only a finished re-read counts.** `read-re-read`'s `incomplete` and `unparseable` never read as clean: brief the round again from the same base and start another agent, or record it `not-re-read`.
- **A prescribed fix is checked by diff.** Call `check-prescriptions` with the re-read's base before anything else touches the tree, since a formatter's run is a change of its own; step 12 says what each verdict does. A MUST or SHOULD it gave no edit (`open`) still needs a fix, a round of its own.
- **At the budget the operator decides** (`start-fix-rounds`' `budget`; step 12 says what to report). A diff-checked round is not a re-read, but one after the budget's last finding still records `budget-stop: true`, which keeps the window's start, so only a clean re-read closes the loop without owing another full review. The operator lets the loop go on, or defers or waives each open finding, recorded as an observation with that disposition, since a re-read's finding maps to no loaded rule.
- **Declining is a waiver.** A round the operator declines to have re-read is `not-re-read` with the reason they give, which `write-review` records as that round's waiver; a round without a reason holds `done` (**§Completion gate (after all tasks)** in `.ductus/reference/implement.md`). With no operator to ask and no agent to start, every round that needed a re-read is `not-re-read` with no reason.
- **`[review] re-reads-off` switches the loop off.** Record the round `exempt`, or `not-re-read` with no reason when it needed a re-read, since `write-review` waives it with the switch's reason; step 12 says the rest.
- **The record.** Step 11's `write-review` passes `loop-open` and step 12's last omits it, so a run cut off between the two holds `done` rather than reading as predating the loop. That last write passes `fix-rounds`, one entry per round with a non-empty scope, in order: `{"check": "exempt"}`, `{"check": "re-read", "result": "clean"}` or `"findings"`, `{"check": "diff-checked"}`, or `{"check": "not-re-read"}` with a `reason` when the operator declined, each with its result's `excluded` list when that holds a path.

## Record the run in `review.md`

The record is `review.md`'s own frontmatter, written in the same pass as the report body below it; `spec.md` is not touched (spec 057). §text-first-artifacts declares its fields, and what they mean is below or under [Window and coverage](#window-and-coverage).

`reviewed-digest` is the record's description of **what this review read**: a per-path digest of the spec's durable contracts (`scenarios/*.md`, `data-model.md`) taken from disk. It decides no staleness: a review is owed for code, and a changed contract owes an analysis (spec 081). What it does is put each contract that changed since the review in the scope of the next review that runs, so a review owed for code reads the contracts that changed beside it. `reviewed-against` stays as provenance, read for the mechanical-sweep rename exemption inside that comparison. `reviewed-unreadable` names a contract that exists but could not be read when the digest was taken.

`review.md` and `spec.md` are outside the digest, which holds only the contracts a review reads code against. The analyze record's subject set _includes_ them, as this command's outputs and that command's inputs. A spec with no scenarios and no data model records `reviewed-digest: {}`, taken and empty.

`blocking: true` when `must-violations > 0` or the criteria step judged a criterion not met (**§Criteria step**). This is the field other commands read.

`dispositions:` counts the run's **observations** — `fixed`, `deferred`, `waived`, `undispositioned` — never its MUST and SHOULD violations, which keep their own counts. The map is always written, because its absence has a meaning: a record without it predates dispositions. Absence is not zero.

The fix-round fields (spec 078) are derived by `write-review` from the per-round list the run passes, never from a count: `fix-rounds` counts every round with a non-empty scope, `re-reads` those an independent agent re-read to completion, `diff-checked` those the diff check verified against a re-read's prescriptions, and `not-re-read` those that needed a re-read and have none; the rest were exempt. `budget-stop` is true when at least the budget's re-reads found a MUST or SHOULD and the last one did. `round-waivers` holds one entry per un-re-read round that is waived — the operator's reason with `decided-by`, or the `[review] re-reads-off` reason with the config file as `waived-by` —. A record without the fields does not say whether its fixes were re-read, which is not zero rounds.

## Window and coverage

The window a review reads and the coverage counts it records, as `/ductus:review` applies them across runs. The record's fields are declared in the constitution's §text-first-artifacts.

- **`examined` counts the in-scope files the review actually read — a file you are confident about by other means is named, never counted.** Believing a file correct — a generated mirror of one you read, say — is not having read it. Count what you opened, and name the rest in the Summary with why you did not open it and what you relied on instead: _generated from a source read in full, generator re-run and reported in sync_ is a complete answer, _it is generated_ is not. Likewise say which parts of a too-large file you read, and name an in-scope path that no longer exists as absent.
- **A spec's review window starts where its last review ended, whether or not the spec reopened since.** `compute-review-scope`'s description states each start, which `base-source` names. Do not narrow a wide window by overriding the base unless it is genuinely wider than the change under review, and record the base used and why in the Summary. On a spec with no review recorded, a fix committed before the reopen that records it lies outside the window and still joins the scope when a commit attributed to the spec made it, since a first review reads every file such a commit changed (spec 086, AC6); one in a commit no spec claims is the project record's to review (spec 084), so no override is needed for it.
- **A review reads its own spec's change, and the record names the rest.** A commit whose subject names a spec has its code reviewed by the specs it names, a spec it claims only by touching that spec's directory reads that directory's files and nothing else (spec 081), and a commit naming no spec and touching no spec directory is no spec's: its code is reviewed under the project record (spec 084). Code is every path outside the project's artifacts: the spec root, `.ductus/`, `AGENTS.md` and `CLAUDE.md` (spec 081), so a change confined to them owes an analysis and no review, unless a criterion owes one (**§Criteria step**). `compute-review-scope`'s description holds the whole rule, and on `ownership: not-examined` the Summary names `ownership-reason`.
- **`unread` names the scope entries the passes did not read.** Pass `[]` only when every entry was read: an entry opened in part, or one you are confident about by other means, is unread, as it is uncounted in `examined`.
- **A review record written before the record gained its coverage and digest fields is unexamined, not clean.** Without `examined` and `scope`, a review that read nothing is byte-identical to one that read everything; without the durable-contract digest, a review that runs cannot tell which contracts changed since that record. Nothing backfills these fields: treat such a record as unread and re-run the review, which writes them truthfully only if the passes read the scope. The gap comes in several shapes, so read the record before pricing the work.
- **A spec with no review recorded and no `in-progress` transition or `done` status in history derives an empty diff base, and the review's denominator collapses to zero.** On a first review scope resolution still returns the plan's affected files and the files the spec's commits changed, so the record reads as files examined over a scope of zero. Pass the current commit as the base: the window is then empty by construction, the scope holds what a first review reads beyond it, and numerator and denominator agree. The same answer fits a recorded transition whose base is simply old; measure both before choosing, and say in the Summary which base was passed and why.
- **The review's denominator is derived at write time, so a count measured earlier and typed into the Summary can contradict the record it is written into.** No check compares the two. Read the returned counts against every number the Summary states, and re-run with corrected prose when they differ; the call is idempotent.
- **`scope` counts entries, not files.** On a first review it holds the plan's Affected Files entries as written, beside the files the spec's commits changed. A pattern such as `src/handlers/*.ts` is one entry however many files it matches, so a Summary that counts files says how its count reconciles with `scope`; `examined` equal to `scope` means fully examined only when no entry is a pattern.

## Blocking semantics

<!-- audit:kept a rule no step acts on; the command's Pipeline position cites it -->

A spec MUST NOT advance from `in-progress` to `done` while its `review.md` records `blocking: true`: `framework/constitution.md` §spec-lifecycle states the gate, as `check-review-gate` evaluates it.

## Blocking message

Emitted by `/ductus:review` when tech-stack alignment fails (missing/empty
`AGENTS.md` `Tech Stack` section, or documented stack inconsistent with
implementation):

```text
blocked: tech-stack alignment failed — AGENTS.md Tech Stack {missing | inconsistent with code in scope}

  expected: <stack inferred from scope, e.g., "TypeScript + React frontend">
  documented: <AGENTS.md Tech Stack contents, or "(empty)">

reconcile AGENTS.md Tech Stack with the implementation, then re-run /ductus:review.
to skip this check on future runs after manual reconciliation, add
[review] tech-stack-verified = true to .ductus/config.toml.
```

## Waivers

A MUST or SHOULD violation can be waived only with explicit, recorded justification:

```text
/ductus:review --waive <rule-id> --reason "<text>"
```

This appends one entry to `review.md`'s frontmatter, through `write-review`'s
`new-waivers`:

```yaml
waivers:
  - rule: BE-AUTHN-007
    anchors:
      - file: src/api/internal.ts
        site: internalRouter
        covers: 1
    reason: "Endpoint is internal-only behind mTLS; rule applies to public APIs"
    waived-at: 2026-05-10T14:40:00Z
    waived-by: <git config user.email>
  - rule: BE-AUTHN-008
    anchors:
      - file: src/api/admin.ts
        site: adminRoutes
        covers: 1
      - file: src/api/metrics.ts
        site: metricsRoutes
        covers: 2
    reason: "Both routes are bound to the loopback interface"
    waived-at: 2026-05-11T09:15:00Z
    waived-by: <git config user.email>
```

A list is one judgment — one `reason`, `waived-at` and `waived-by` — over
each anchor it names, so a later `--waive` for the same rule records a new entry
rather than appending to an existing one. The framework never merges entries; an
operator may merge those sharing a rule, reason and author by hand.

Waived findings drop out of the `must-violations` and `should-violations` counts
and appear in `review.md` under the **Waived findings** section. `process-waivers`'
description states what one run applies, expires, retains and drops per `(rule, file, site)`
anchor (spec 020's scenario `waiver-expiry`). Line numbers are not part of the
anchor, so code moving within its site does not expire the waiver. An anchor's
path is matched literally: a pattern such as `src/**/*.ts` names a path that
does not exist and expires as one.

**A departure the spec states needs no `--waive`.** A well-formed departure
entry under the spec's `## Applicable Rules`, of any tier, a SHOULD's included,
is a decision the spec already holds: `write-review` stores one waiver for its
rule from it, listing the files in this review's scope, or, for an entry whose
`covers:` lists the sites it decides, an `anchors` list of those sites, each with
no count, and `process-waivers` expires it once the spec no longer holds the
entry. Step 11 reports the result's `malformed-departures` and
`departure-notices`. The entry decides over a deferral of its violations at the
sites it covers. The derivation, the waiver's fields and the entry's grammar are
`write-review`'s and `check-departures`' (`ductus write-review --help`, `ductus
check-departures --help`).

## Per-run waiver processing

On every `/ductus:review` run, step 8's `process-waivers` classifies each
anchor against the findings the passes produced (see **Run review passes**),
passed as `fired`, one entry per finding with its `site` and `line-range`. Its
description states what it applies, expires, retains and drops, and step 8 says
what the host passes and writes. A waiver applies while its rule still fires at
its site, at most `covers` times; it expires when the rule no longer fires there
in a file the run read, when its file is gone, or when more findings fire than it
covers; it is retained when the run's scope does not hold its file, and dropped,
on a spec's record, when no plan entry names its file and no commit attributed to
the spec changed it (spec 086). Expiry MUST NOT be silent: a `waiver expired:`
notice names the cause. When the same rule still fires anywhere in scope after a
drop, the finding re-counts toward its severity's count: a MUST flips
`review.blocking` to `true`, and a SHOULD holds `done` at the pre-done gate. A
waiver is never extended to a file or site it does not list: the rule firing
elsewhere is a separate finding, and the operator records a fresh `--waive` if it
is also intentional.

## Malformed and duplicate waivers

- A waiver entry missing any of `rule`, `anchors` (`file` on an unscoped
  departure waiver), `reason`, `waived-at`, or `waived-by`, or whose `file` is an empty list or holds a blank path, is
  **skipped** with a one-line warning and kept on the write: malformed entries
  are operator-authored state, not garbage for the framework to collect, so the
  operator cleans it up. Two exceptions: a re-render drops an entry with no
  fields at all, and a malformed entry listing an expired waiver's anchor loses
  that anchor.
- Two or more claims on the same `(rule, file, site)` anchor: **only the first
  applies**. Each later claim emits a one-line warning, and only that anchor of
  its entry is ignored. Duplicates are kept until the anchor expires, when every
  claim on it is pruned together.

The `waivers` list follows the §text-first-artifacts open-schema
rule. Adopters MAY add fields (e.g., `co-waived-by`, `approved-by-team`) to
enforce org-specific waiver policy in their own CI — but never a reference to
work that will fix the finding, which makes it a deferral ([Deferrals](#deferrals)).
`write-review` preserves them verbatim on a surviving waiver.

## Deferrals

A MUST or SHOULD violation that will be fixed later is **deferred**, never
waived: a waiver records that a finding will not be addressed
(§brownfield-inbox, Finding dispositions).

```text
/ductus:review --defer <rule-id> --target <path> [--task <n>] [--work-item <id>] [--reason "<text>"]
/ductus:review --defer <rule-id> --work-item <id> [--reason "<text>"]
```

Each site the rule fires at gets one decision in `review.md`'s `decisions:`,
keyed `` rule {rule-id} at {file} in `{site}` ``, carrying its `target`, `task`
and `work-item` as given and `covers`, the number of findings there
(`process-decisions`' and `write-review`'s descriptions state the key and the
count). A new deferral's target, and its task in that `tasks.md`, must exist.

**When a deferral releases a gating finding.** A gating finding — a review MUST
or SHOULD violation, or an analyze hard-fail or blocking finding — is
**released** from its spec's gate only when its deferral names pipeline work on
another spec that is still to do: a scenario, a body edit or an unchecked task on
a spec that is not `done`, or a new spec. That spec's gate then holds the work.
Any other deferral releases nothing, and the finding still holds `done`: one to
this spec's own work, to a rule file, to an external work item alone — no gate
reads an external tracker — or to work that is complete or cannot be read, whose
target the run names in a notice.

For a violation:

- **Released**, it leaves `must-violations` and `should-violations` and
  renders under **Deferred findings**. A `--defer` to a `done` spec's work
  names the `done → in-progress` reopen in a confirmation and performs it
  through the set-status primitive with `from: done` before `write-review`
  runs, once `check-owed-work` on that spec reports owed work: a `done` spec
  reopens only while its `tasks.md` holds an unchecked box or its `spec.md` an
  unticked criterion. Left closed, a `done` spec's work, its tasks included, is
  complete and releases nothing.
- **Not released**, it stays in its section with a `**Deferred**` line and
  still holds `done`. One to the spec's own work on a `done` spec reopens it,
  as a released one does, once its task is on disk: the one status a review
  writes on its own spec (§spec-lifecycle). Its decision carries the derived `gating: true`, and
  `check-review-gate` names it with its work where it blocks.

A deferral retires when its finding stops firing or its named work is complete
(`lapsed`, in `process-decisions`' description), and the violation then needs a
fix, a waiver, or a new deferral. A review also drops one that did not fire on a
file the spec does not own (spec 086). A new `--defer` naming other work than a
standing deferral replaces it as a new decision. A finding holds one disposition
across runs through step 11's drop lists, and `write-review` refuses one that
carries both. A departure entry decides over a deferral of its rule's violations
at the sites it covers (**§Waivers**).

## Auto-fix scope

`--fix` is conservative by design. It applies fixes when **all** of these hold:

- The finding is marked `auto-fixable: yes`
- The fix does not change function signatures, return types, or schema
- The fix does not change observable HTTP status codes, error messages, or
  log formats
- The fix does not delete tests
- The fix is contained to files already in the review scope

When in doubt, leave the finding unfixed and let the user apply the
`Suggested fix` manually.

## Output

Stdout summary (always), followed by the path to `review.md`:

```text
/ductus:review — 042-example-feature

  security    ✓ 0 MUST   2 SHOULD
  reuse       ✓ 0 MUST   1 SHOULD
  quality     ✓ 0 MUST   0 SHOULD   (2 low-confidence)
  efficiency  ✓ 0 MUST   0 SHOULD
  simplicity  ✓ 0 MUST   0 SHOULD

  observations 3 — 1 fixed, 1 deferred, 1 waived, 0 undispositioned
  analyze     ✗ last run 2026-09-06 against 683a1e0 — this review supersedes it
  blocking: no
  report:   specs/042-example-feature/review.md

  next: fix, waive or defer the 3 SHOULD violations, then /ductus:analyze — the done gate holds on an unwaived SHOULD
```

## The `observations` row

The row renders on **every** run from the `dispositions` field `write-review`
returns, as the observation total followed by all four counts — including a
run with no observations, which renders `observations 0`, because
examined-and-empty and not-computed must not be the same output. It never
affects the exit code, and an `undispositioned` count above zero does not hold
`done` unless a broken contract is among it, so the row names it for a later review to decide:

```text
  observations 2 — 0 fixed, 1 deferred, 0 waived, 1 undispositioned — recorded; it holds done only if a broken contract is among it
```

## The `analyze` row

`/ductus:review` is the command that most reliably _invalidates_ an analyze
record, and this row is what carries an operator from a passing review to the
second gate, which would otherwise take memory: the diligence dependency
§design-principles rejects (spec 047, `analyze-record-freshness`).

The row renders on **every** run, in one of four states, from the
`analyze-freshness` field `write-review` returns:

```text
  analyze     ✗ never analyzed — run /ductus:analyze before done
  analyze     ✗ last run 2026-09-06 against 683a1e0 — this review supersedes it
  analyze     ✓ last run 2026-09-06 against 683a1e0 — current
  analyze     ? freshness undeterminable — the record carries no analyzed-digest
```

The state is a **content** comparison: the digest the analysis recorded of what
it read, against the spec's analyze subjects as they are now, so this row and the
pre-done gate cannot disagree. Writing this review supersedes the record because
`review.md` is itself an analyze subject, not because `HEAD` moved; a run that
changed nothing leaves it `current`. On a `done` spec the comparison ignores an
in-place edit to `plan.md` or `tasks.md` (a deleted one still counts), since such
an edit adding no unchecked task takes no back-edge there (§spec-lifecycle). The
review record's `reviewed-digest` stales nothing; it only puts changed contracts
in a running review's scope (see [Record the run in `review.md`](#record-the-run-in-reviewmd)).

**The row is a notice, not a gate.** `blocking` and the exit code are unchanged by
it. The `next:` line follows for the same reason: it names the command the operator
owes and withholds nothing.

When MUST violations are present:

```text
/ductus:review — 042-example-feature

  security    ✗ 2 MUST   1 SHOULD
  reuse       ✓ 0 MUST   0 SHOULD
  quality     ✗ 1 MUST   0 SHOULD
  efficiency  ✓ 0 MUST   0 SHOULD
  simplicity  ✓ 0 MUST   0 SHOULD

  blocking: yes — 3 MUST violations
  report:   specs/042-example-feature/review.md

  spec cannot advance to done. Resolve violations and re-run /ductus:review,
  run /ductus:review --waive <rule-id> --reason "..." for each waivable finding,
  or defer each to pipeline work on another spec (/ductus:review --defer <rule-id> --target <path>),
  then /ductus:analyze, which the done gate also requires.
```

The `analyze` row renders on the blocking path too: the fixes that clear the
violations are exactly what supersedes the recorded analysis.

Exit code: `0` when not blocking, `1` when blocking, carried in `write-review`'s
result as `exit-code`. The CLI and `ductus exec` exit 0 once the record is written,
so CI gates on the committed record's `blocking` (the CI template's audit-record
gate), not on an exit status.

## Notes for adopters

- Projects that customize shipped rule files (e.g.,
  `specs/rules/security-backend.md`) pin them in `.ductus/config.toml`
  `[pinned] files` to prevent `/ductus` from overwriting their additions.
  `/ductus:review` reads whatever is on disk — pinned or not.
- Files inside the rule-file directory (`specs/rules/` in adopter
  projects; `framework/rules/` in ductus's own repo) are auto-discovered
  (see step 2 of the command's Instructions, `discover-rule-files`). Adding
  a file at `specs/rules/<domain>-{backend,frontend,cross}.md` is the only
  step needed; the suffix selects which stacks load it. Any other suffix loads
  for every stack and warns, never skips silently, until the file is renamed.
- The `AGENTS.md` rule-file reference survives strictly for adopter-local
  rule files placed **outside** `specs/rules/` — e.g.,
  `docs/rules/internal-api.md` — since the framework cannot directory-walk
  arbitrary adopter paths.
- A rule file can be excluded from a project's review via `.ductus/config.toml`
  `[[review.disabled-rule-files]]` (see **Inputs** in the command file). The
  override is project-wide and its mandatory `reason` is the audit trail. Use it
  when the stack-derived selection is correct but the team is not yet ready to
  enforce that file's rules (e.g., `security-frontend.md` on a project whose
  frontend is being retired). A file whose header gives it to analysis alone
  needs no entry, since a review does not load it. Waivers remain the right
  tool for individual `(rule, file, site)` exceptions.
- The five-dimension model is fixed. Domain-specific concerns (accessibility,
  i18n, licensing) belong in additional rule files, not new passes.
