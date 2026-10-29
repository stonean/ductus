# review — markdown-only reference

The numbered Instructions in the command file, `review.md`, are the deterministic path — the runtime's primitives own the rule-file selection, waiver arithmetic, scope resolution, and report scaffolding, and the five passes cross the boundary at the `performReview` extension point. When no runtime is available, walk the detailed procedure below by hand, for each targeted feature, in order.

**Where a named section lives.** Read from `/{project}:review`, this file has its command, `review.md`, already loaded; a reader arriving from another file — the installer reads **§Resolve target and scope** — opens `review.md` as well when a section it needs is named there. A section named below that has no heading in this file is in the command file, along with the numbered Instructions steps this file cites. A bare `§name` is a heading in this file or a marker in the constitution, `.ductus/constitution.md`; a `§name` with another file's path on the same line is a heading in that file, unless it is a constitution marker.

## Resolve target and scope

1. Resolve the working feature from `--all` or the current `/{project}:target`.
   If neither yields a target, halt with `no target — run /{project}:target first`.
2. Read the spec frontmatter. If `status` is not in `{in-progress, done}`,
   halt with `review only runs against in-progress or done specs`.
3. Build the file scope per **Inputs** (in the command file). If the resolved scope is
   empty (no implementation files yet), write a `review.md` recording 0
   findings across all five passes, `blocking: false`, and exit `0` — there
   is nothing to review yet. Skip steps 4–5 and the rest of this run, except
   that an observation the reviewer supplies is still dispositioned, as
   **Observations** under [Write `review.md`](#write-reviewmd) describes,
   before the record is written: the reviewer's judgment is the input, not
   the diff.
   **Distinguish an empty scope from a scope you could not read.** A tool
   that returns an error, a size-cap notice, or a saved-output pointer has
   told you nothing about the scope; resolve it by another route and read
   it in full. Recording the 0-findings report there would state that the
   passes examined an empty scope when they examined nothing at all.
4. **Tech-stack alignment check.**
   - Read the active config file (resolution rule in Instructions step 1;
     spec 042). If `[review] tech-stack-verified = true`, skip to
     step 5.
   - Otherwise, read `AGENTS.md`'s `Tech Stack` section and inspect the file
     scope (extensions, imports, runtime/manifest markers). Confirm the
     documented stack appears consistent with the implementation. A
     missing or empty `Tech Stack` section, or an inconsistency between
     documentation and code, halts the run with the
     [tech-stack-misalignment](#blocking-message) message and exits `1`.
   - On a successful check, prompt the operator once (routing the prompt
     through `gate-confirm` on the runtime path, as the other
     confirm-before-write pipeline steps do): _"Tech-stack alignment
     confirmed. Persist this so future runs skip the check? (Y/n)"_. On
     `Y`, write `[review] tech-stack-verified = true` to the active config
     file (same resolution as Instructions step 1). On `n` or skip, the check runs again on the next
     invocation. To re-run the check after a stack change, the operator
     removes the line manually — `/{project}:review` does not auto-reset.
5. Discover rule files by suffix. List `framework/rules/*.md` in ductus's
   own repository, or `specs/rules/*.md` in adopter projects. For each
   file, classify by basename suffix:
   - `*-backend.md` → backend surface
   - `*-frontend.md` → frontend surface
   - `*-cross.md` → cross-cutting (applies to every stack)
   - anything else → unrecognized — load for every stack and emit one
     stdout line per file:

     ```text
     rule file <name> has unrecognized suffix — loading for all stacks; rename to -backend.md, -frontend.md, or -cross.md
     ```

   Name every **unknown key** in the two sections this step reads, ahead
   of every other line it emits (spec 019, `CFG-ENV-012`). `[rules]`
   recognizes `surfaces`; `[review]` recognizes `tech-stack-verified` and
   `disabled-rule-files`, each of whose entries recognizes `file` and
   `reason`. Any other key — a typo such as `surface`, which would
   otherwise be dropped while its setting silently took its default — is
   reported and the run continues, one stdout line per key:

   ```text
   unknown config key: [rules] surface (<config-file>) — this version of ductus does not read it
   unknown config key: [[review.disabled-rule-files]] entry [N] approved-by (<config-file>) — this version of ductus does not read it
   ```

   An entry carrying an unknown field still applies. A key that needs
   quoting is shown quoted and escaped, as TOML writes it.

   Determine the **surface selection** for this run. Read `.ductus/config.toml`
   `[rules] surfaces` (see **Inputs** in the command file):

   - **Set to a valid list** (every member in `{backend, frontend}`) —
     keep the rule files whose surface is listed in `surfaces`, plus
     every `*-cross.md`. This explicit operator-set selection _replaces_
     the detected-stack filter; the stack from step 4 is not consulted
     for rule-file selection. The **empty list** (`surfaces = []`) is a
     valid value of this case and means **cross-only**: no
     `*-backend.md`/`*-frontend.md` file is kept, only `*-cross.md`. The
     empty list is distinct from the key being unset (below) — it is the
     operator explicitly declaring "no surface rules, only cross-cutting
     ones," not a request to derive.
   - **Set to a degenerate value** — fail fast (do not silently ignore,
     do not warn-and-continue), consistent with `CFG-ENV-008`'s
     fail-fast-on-malformed-configuration posture:
     - **Unrecognized member.** A member outside `{backend, frontend}` —
       a typo like `"fullstack"`, or `"cross"` (cross-cutting files are
       unconditional, not a selectable surface) — halts the run with
       `/{project}:review: invalid [rules] surfaces member "<value>" — accepted members are "backend" and "frontend" (use [] for cross-only; -cross.md files always apply)`.
       A list mixing valid and invalid members (`["backend", "fullstack"]`)
       fails on the invalid member; a valid member does not rescue it.
     - **Type mismatch.** A non-list value (`surfaces = "backend"`, a
       bare string) halts the run with
       `/{project}:review: [rules] surfaces must be a list of strings, got <type>`.
   - **Unset** — fall back to the detected stack from step 4: keep the
     matching surface, keep every `*-cross.md` (pre-033 behavior).

   In every non-error case, keep every unrecognized-suffix file
   unconditionally.

   Then apply the **disabled-rule-files filter**. Read `.ductus/config.toml`
   `[[review.disabled-rule-files]]` (see **Inputs** in the command file). First
   validate **every** entry, before any rule file is selected — a
   malformed one halts the run (below). Then, for each entry, in list
   order:

   - **Drop + notice (selected match).** `file` matches the
     basename of a file currently in the post-selection set. Remove
     it from the set and emit one line:

     ```text
     disabled-rule-file: <filename> — <reason> (<config-file>)
     ```

     `<config-file>` is the repo-relative resolved config file the
     disable came from — `.ductus/config.toml`, or `.govern/config.toml`
     / the legacy root `.govern.toml` on a pre-migration layout. The
     ladder is three tiers, not two (`schema/paths.rs`'s `CONFIG_CHAIN`);
     `status.md` states the same set in the same sentence shape. Collapse internal
     whitespace in `reason` (including newlines from TOML multi-line
     strings) to single spaces before emitting — the notice is
     single-line by contract.

   - **No-op notice (non-selected match).** `file` matches a
     basename in the rule-file directory but the file was NOT in the
     post-selection set (different surface). Emit one line and
     change nothing:

     ```text
     disabled-rule-file (no-op): <filename> not selected by stack detection
     ```

     This is honest about state — the entry is currently a no-op,
     becomes load-bearing if the project's stack changes later.

   - **Unknown warning.** `file` does not match any basename in the
     rule-file directory. Emit one line and change nothing:

     ```text
     unknown disabled-rule-file: <filename> (no such file in the rule-file directory)
     ```

     This covers renamed/moved files; not a fatal error.

   - **Malformed entry — fail fast.** An entry that is not a table,
     whose `file` or `reason` is missing or not a string, or whose
     `reason`'s trimmed length is < 16 Unicode codepoints, is a
     configuration value that does not parse, so — consistent with
     `CFG-ENV-008`, as for a degenerate `[rules] surfaces` above — it
     halts the run before any rule file is selected, naming the entry's
     index and the field that failed:

     ```text
     /{project}:review: invalid [[review.disabled-rule-files]] entry [N] in <config-path>: <problem> — each entry is a table with a `file` naming a rule file and a `reason` of at least 16 characters
     ```

     `<config-path>` is the resolved config file's full path.
     `<problem>` names the field and the offending value as TOML renders
     it, on one line — for example `` `reason` is missing ``,
     `` `file` is an integer (5), not a string `` or
     `` `reason` "PoC only" is 8 characters once trimmed, under the
     minimum of 16 ``. The entry is not skipped and NOT auto-removed;
     the operator fixes it. The halt is a configuration error that stops
     the run with a non-zero exit (`CFG-ENV-008`), never a review
     finding, so it does not set `blocking`.

   - **Duplicate warning.** Same `file` listed twice. Only the first
     entry applies; each subsequent duplicate emits one line and is
     not auto-pruned:

     ```text
     duplicate disabled-rule-file: <filename> — entry [N] ignored
     ```

   The no-op, unknown and duplicate lines emit to stdout and **do not
   affect the exit code**. A run that completes takes its exit status
   from MUST violations alone (see [Output](#output)); a malformed entry
   halts the run before it reaches any finding, exiting non-zero without
   setting `blocking`. Every `file` and `reason` a notice or the halt
   renders is put on one line first, so committed config cannot forge a
   notice line.

   Finally, emit a single stdout line naming what was selected:

   ```text
   loading rule files: <comma-separated basenames>
   ```

   Disabled files are excluded from this list. The notice fires AFTER
   all disabled-rule-file lines, so a normal run reads top-down as:
   any `disabled-rule-file: …` notices, then `loading rule files: …`.
   This is the discoverability surface — adopters can confirm which
   files were considered without parsing the report.

## Load rules

Load these inputs inline as the authoritative review criteria:

- Every rule file selected by the suffix-based discovery in step 5 of
  [Resolve target and scope](#resolve-target-and-scope)
- Any rule file outside the rule-file directory (e.g., `docs/rules/internal-api.md`)
  referenced from `AGENTS.md` — see [Notes for adopters](#notes-for-adopters)
- `AGENTS.md` `Code Style`, `Testing`, `Gotchas`, and `Boundaries` sections
- The target spec's acceptance criteria and any `scenarios/*.md` files

These are the **only** sources of normative rules for the review. Do not
introduce review criteria from outside the project.

## Run review passes

Run the five passes below. When a flag restricts dimensions (`--security`,
`--simplicity`, `--quality`), skip the unselected passes and record them as
`skipped` in the report.

When the same finding (matching rule ID, file, and overlapping line range)
is produced by more than one pass, retain only the highest-severity instance
in `must-violations` and `should-violations`; lower-severity duplicates are
dropped from the counts and the report. Pass-of-record for the surviving
finding is the highest-severity pass that flagged it.

### Security pass

Walk every file in scope against the loaded security rules. For each finding,
record: rule ID, severity (MUST or SHOULD), file path, line range, the rule
text, and a one-sentence explanation of how the code violates it. **Do not
flag patterns that are not in the loaded rules** — the project's rule set is
authoritative.

That rule holds for every pass below, and it is not a reason to drop what you
noticed: anything real that matches no loaded rule is an **observation**, and
every pass may return them alongside its findings — see the Observations
rules under [Write `review.md`](#write-reviewmd). Recording one puts it
in front of the fix-and-route step, so there is nothing separate to remember.

### Reuse pass

Identify logic that duplicates existing utilities or that should be extracted
into shared code. Cross-reference with `specs/system.md` for established
patterns and shared infrastructure. Severity is SHOULD unless the duplication
contradicts an explicit MUST in `AGENTS.md` `Boundaries`.

### Quality pass

Detect bugs, missing error handling, unhandled edge cases, off-by-one errors,
and contract violations against `specs/errors.md`. Each finding carries a
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

## Write `review.md`

Write the report to `specs/NNN-feature/review.md`. A scenario-targeted run still writes to the same spec-level path; the `scenario:` frontmatter field records which scenario was reviewed and `reviewed-against` records the commit. Re-running review (scenario- or feature-targeted) supersedes the prior `review.md` wholesale.

```markdown
---
spec: 042-example-feature
last-run: 2026-05-10T14:32:00Z
reviewed-against: <sha-of-HEAD>
diff-base: <sha of the parent of the in-progress transition commit>
must-violations: 0
should-violations: 3
low-confidence: 2
examined: 38
scope: 46
skipped-passes: []
reviewed-digest:
  scenarios/retry.md: 3f2a…
blocking: false
dispositions:
  fixed: 1
  routed: 1
  discarded: 1
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

## Observations

<one bullet per observation this run recorded, each with its disposition; `*None.*` when empty>

## Skipped passes

<`*None.*` when none>

## Unexamined governance

<one bullet per registered shared constitution this run could not read; `*None.*` when none>
```

**Unexamined governance** names any `[constitutions.*]` entry (spec 055) the run could not read, with its reason — `not checked out`, or `no constitution.md in checkout`. `write-review` resolves the registry itself rather than taking it as an argument, so a report cannot omit the section: a review that ran under fewer rules than the project's config declares says so, instead of letting the finding counts stand in for a clean result. That is `QUAL-CLAIM-001` applied to the review's own inputs. `*None.*` covers both "none registered" and "all registered sources read" — from the report's side those are the same claim, because in neither case did anything go unexamined.

Each bullet carries the entry's `description` when it has one, between the path and the reason: `` - `acme` (../governance) — Acme platform engineering rules — not checked out — its rules were NOT loaded for this review ``. An alias alone is a config key someone chose, so the description is what tells an operator _which_ checkout they are missing rather than sending them to look the alias up. It renders here precisely because the document could not be read — the value comes from the config, not the checkout, so it is available when the document is not. An entry with no description renders exactly as it did before. The line is single-line by contract, so an over-long description is truncated with `…` rather than wrapped, and internal whitespace is collapsed before it arrives (a TOML multi-line string makes an embedded newline reachable).

**`examined` and `scope` are what make a clean report mean something.** The counts alone cannot distinguish _the passes read the scope and found nothing_ from _the passes never ran_ — both write `0/0/0`, the same `reviewed-digest`, and `blocking: false`. `write-analysis` has required `unexamined` since spec 047 for exactly this reason, and the review half carried no equivalent until a review was recorded over a scope nothing had read. `scope` is derived by the primitive, so the denominator cannot be shrunk to match whatever was read; `examined` is the reviewer's claim about the numerator, and an **unstated** one is recorded as absent rather than as zero — a claim never made and a claim that came back empty are different facts. No check reads the pair: Family 31 did, and was retired with the record's second home (spec 057), so these are read by a person at the completion gate. Nothing could prove the passes ran anyway — an overstated numerator is as available as an omitted one — so what this buys is that the claim is explicit and re-derivable instead of invisible.

Every empty section renders the literal `*None.*` line — the `write-review` primitive emits it, and the markdown-only path writes the same so the two paths produce byte-identical reports. The **Observations** heading carries no suffix.

The **Observations** section is the home for something the reviewer judged
real that maps to **no loaded rule**. Inventing a rule for it is not the answer
— the rule set is authoritative — and the free-text Summary is not either,
because `write-review` regenerates the Summary wholesale on the next run and it
is per-spec, so a cross-cutting note filed there is both erased and misfiled.
An observation is **not** a finding: it never enters the
`must-violations` / `should-violations` / `low-confidence` counts, never
affects `review.blocking`, and never changes the exit code. Each entry carries
its own one-line text and, optionally, the path it anchors to. Leading the text
with a category (`security` / `leak` / `convention` / `bug` / `perf` / `other`)
helps a reader route it, but nothing parses it.

**Every observation is proposed a disposition before the record is written**
(§brownfield-inbox, Finding dispositions), and the section renders it beside
the observation, the path clause omitted when the observation has none:

```text
- {text} — `{path}` — **fixed**
- {text} — `{path}` — **routed** to `{target}`
- {text} — `{path}` — **discarded**: {reason}
- {text} — `{path}` — **undispositioned**
```

- **Fixed** — a chore: mechanical, and adding no durable requirement
  (§bug-handling's durability test). It is confirmed before it writes, and
  the affected passes re-run before the record is written, as `--fix` does,
  so the record describes the fixed tree. A fix that fails, or turns out not
  to be mechanical, is reverted and is not a chore; a chore is never counted
  fixed while its fix is not on disk.
- **Routed** — written to the home the **§Groom decision tree** in `.ductus/reference/groom.md`
  chooses; that is its single canonical statement, and this reference does
  not restate it. The homes are a task, a scenario with its task, or a body
  edit on this spec; a scenario or body edit on another existing spec; a new
  spec, created through `/{project}:specify`'s procedure in the same run; or
  an amendment to a rule file the project owns. A route to a `done` spec names
  the `done → in-progress` reopen in its confirmation and performs it with
  `from: done`, so a status that changed since the proposal surfaces rather
  than being overwritten. A declined spec creation leaves the observation
  discarded or undispositioned, never routed to a spec that does not exist,
  and a confirmed route whose write fails is undispositioned, with the failure
  named beside it.
- **Discarded** — with the reason it is out of scope. It is the disposition
  that ends a loop: an observation about the pipeline's own machinery rather
  than the code under review, routed to a `done` spec, reopens that spec, and
  that spec's next review produces the next observation.
- **Undispositioned** — nobody decided: the operator declined without a
  reason, a route's write failed, or nobody could be asked. `ductus exec`'s
  walker no-ops steps 9 and 10 of the command's Instructions, so every observation an exec run threads
  through is recorded undispositioned. It stays recorded and visible for a
  later review to decide, and it does not hold `done`: only a gating finding
  does, and an observation is never one.

**Nothing reaches the inbox.** Observations were once written through to
`specs/inbox.md`, the one destination no gate reads, so a spec could reach
`done` while they waited there unseen. An observation supplied to a run whose
scope is empty is still dispositioned — the reviewer's judgment is the input,
not the diff.

**Decisions persist.** Each routed or discarded observation is stored in
`review.md`'s `decisions:` list, beside `waivers:`, keyed on its rendered line
— the text, then an em dash and the path in backticks — with its outcome, its
target or reason, `decided-at`, and `decided-by`. Observation text is the
reviewer's own wording and does not reproduce byte for byte, so the next run
matches each new observation to the stored decision describing the same issue
and passes that decision's key as the observation's `decision-key`; a matched
observation counts under its stored outcome and is not asked about again. A
stored decision nothing matched is pruned on an unrestricted run and retained
when a pass did not run, exactly as a waiver is. A missed match costs one
repeated question, never a silent waiver, because an unmatched decision is
pruned rather than applied to something else. A malformed entry is reported,
applies to nothing, and is kept; an entry with no fields at all holds no
decision, so a re-render drops it. A duplicate key is reported and ignored,
and goes when its key expires or is given a new decision, because a new
decision replaces every stored entry for its key. Waivers have their own
rules, under [Malformed and duplicate waivers](#malformed-and-duplicate-waivers). A
`decisions:` list that does not parse is reported by `validate-frontmatter`,
and `write-review` refuses to write over it rather than read it as empty.

An observation whose subject later becomes a real rule finding needs nothing
special — the finding counts, and the observation is the reviewer's to drop on
the next run.

The finding sections reconcile the same way. A SHOULD kept as-is is waived
with `--waive` and its rationale ([Waivers](#waivers)), so it renders under
**Waived findings** rather than its original heading — an accepted trade-off
left filed as a violation is indistinguishable from unfinished work; a
low-confidence finding blocks nothing and needs no waiver. A fix is recorded
by re-running `/{project}:review`, never by editing the report:
`write-review` regenerates its body and counts whole, so they state what is
_outstanding_.
Observation dispositions are recorded in `dispositions:`, which `write-review`
derives from the observations — there is no need, and no room, for a field of
your own: `write-review` emits a fixed field set and would drop it on the next
run.

Each finding follows this shape:

```markdown
### MUST: <rule-id> — <one-line summary>

- **File**: `path/to/file.ts:42-55`
- **Rule**: <verbatim rule text from the rule file (framework/rules/... or specs/rules/...)>
- **Finding**: <one to three sentences>
- **Auto-fixable**: yes | no
- **Suggested fix**: <code block or prose>
```

The report is regenerated on every `/{project}:review` run — never appended.
Findings the user has explicitly waived (see [Waivers](#waivers)) carry across
runs as long as their anchor (rule + file) is still valid.

## Apply `--fix` (optional)

When `--fix` is set, after writing the report:

1. Apply every finding marked `auto-fixable: yes` whose severity is SHOULD,
   plus MUST findings whose suggested fix is purely mechanical (e.g. removing
   a hardcoded secret, adding a missing CSRF token attribute).
2. **Never** auto-apply fixes that alter externally observable behavior, change
   error messages or status codes, or modify schema. These require a manual pass.
3. Re-run only the affected passes against the modified files. Update
   `review.md` with the post-fix counts.
4. Stage the modified files but do not commit. The user owns the commit.

## Record the run in `review.md`

The record is `review.md`'s own frontmatter, written in the same pass as the
report body below it. `spec.md` is not touched — one fact, one home
(spec 057):

```yaml
---
spec: 020-code-review
last-run: 2026-05-10T14:32:00Z
reviewed-against: <sha>
diff-base: <sha>
must-violations: 0
should-violations: 3
low-confidence: 2
examined: 38
scope: 46
skipped-passes: []
reviewed-digest:
  scenarios/retry.md: 3f2a…
  data-model.md: 9c1b…
blocking: false
dispositions:
  fixed: 0
  routed: 1
  discarded: 0
  undispositioned: 0
waivers: []
decisions:
  - key: "convention: retry backoff is hard-coded — `src/retry.ts`"
    outcome: routed
    target: specs/020-code-review/scenarios/retry-config.md
    decided-at: 2026-05-10T14:32:00Z
    decided-by: dev@example.com
---
```

The timestamp is spelled `last-run`, matching the analyze record. It was
`reviewed-at` here and `last-run` in the spec's `review:` block — one instant
under two names, back when the record had two homes, which the reconciliation
check had to key _by meaning rather than by name_ to compare at all. With one home the second spelling had nothing left to
justify it, and the relocation migration folds the pair.

`reviewed-digest` is the record's description of **what this review read** — a
per-path digest of the spec's durable contracts (`scenarios/*.md`,
`data-model.md`) taken from disk. The completion gate's staleness check
compares it against those files as they are now, rather than diffing
`reviewed-against` against `HEAD`: this command reviews the _working tree_
while that field records a _commit_, so a scenario written during the session
came back as a contract that had changed since the review when it had changed
only since the commit the review was labelled with. `reviewed-against` stays as
provenance, read for the mechanical-sweep rename exemption alone.

`review.md` and `spec.md` are deliberately outside the digest. `review.md` is
this command's own output, so counting it would stale every review the instant
it was recorded. `spec.md` is outside for the same reason historically —
`write-review` used to stamp a block into it — and that rationale lapsed when
the record moved; the set is left unchanged regardless, because adding
`spec.md` would stale every review on any spec-body edit, which is a behavior
change rather than a relocation. That is the inverse of the analyze record's
subject set, which _includes_ them, because they are this command's outputs
and that command's inputs. A spec with no scenarios and no data model records
`reviewed-digest: {}` — taken and empty, which reads as current, and is
distinct from an absent digest, which cannot be judged at all.

`blocking: true` when `must-violations > 0`. This is the field other commands
read. (`write-review` writes `last-run`, `reviewed-against`, `must-violations`,
`should-violations`, `low-confidence`, `scope`, `reviewed-digest`, `blocking`,
and `dispositions` on every run; `examined` when the run stated it, and
`reviewed-unreadable` when a contract could not be read; plus the `waivers`
and `decisions` lists when present.)

`dispositions:` counts the run's **observations** — `fixed`, `routed`,
`discarded`, `undispositioned` — never its MUST and SHOULD violations, which
keep their own counts. The map is always written, all four counts, because its
absence has a meaning: a record without it predates dispositions, and the
pre-done gate blocks an `in-progress` spec on it until this command re-runs.
Absence is not zero — reading it as zero would pass exactly the old review
whose observations went to the inbox unseen.

## Window and coverage

The window a review reads and the coverage counts it records, as `/{project}:review` applies them across runs. The record's fields are declared in the constitution's §text-first-artifacts.

- **`examined` counts the in-scope files the review actually read — a file you are confident about by other means is named, never counted.** Believing a file correct — a generated mirror of one you read, say — is not having read it. Count what you opened, and name the rest in the Summary with why you did not open it and what you relied on instead: _generated from a source read in full, generator re-run and reported in sync_ is a complete answer, _it is generated_ is not. Likewise say which parts of a too-large file you read, and name an in-scope path that no longer exists as absent.
- **A spec's review window is a property of its last reopen, not of the spec.** The diff base is the parent of the commit that last moved the spec into `in-progress`, or `HEAD` while that move is uncommitted, so every back-edge moves it forward, and every change the pass makes afterwards — its own review record included — joins the window. Measure the window rather than quoting an earlier count. Do not narrow a wide window by overriding the base unless it is genuinely wider than the change under review, and record the base used and why in the Summary. The opposite gap does call for an override: a fix committed before the reopen that records it lies outside the window, so record the reopen before making the fix, or, when the fix has already landed, set the base to the commit before it.
- **A review record written before the record gained its coverage and digest fields is unexamined, not clean.** Without `examined` and `scope`, a review that read nothing is byte-identical to one that read everything; without the durable-contract digest, the pre-`done` gate reports freshness as _undeterminable_, which does not block. Nothing backfills these fields: treat such a record as unread and re-run the review, which writes them truthfully only if the passes read the scope. The gap comes in several shapes — all fields absent, a numerator without a digest, a digest without a numerator — so read the record before pricing the work.
- **A spec with no recorded `in-progress` transition derives an empty diff base, and the review's denominator collapses to zero.** Scope resolution still returns the plan's affected files, so the record reads as files examined over a scope of zero. Pass the current commit as the base: the window is then empty by construction, and numerator and denominator agree. The same answer fits a recorded transition whose base is simply old; measure both before choosing, and say in the Summary which base was passed and why.
- **Only a spec's durable contracts stale its review — its scenarios and its data model, nothing else — and that is what prices a corpus-wide sweep.** A sweep confined to spec bodies, plans and tasks costs no re-review however many specs it touches, while one changed line in a scenario or data model obliges a full re-review of that spec. Measure a sweep's durable-contract hits first, then decide each: sweep it and accept the re-review, leave it, or defer it into a pass that re-reviews the spec anyway.
- **The review's denominator is derived at write time, so a count measured earlier and typed into the Summary can contradict the record it is written into.** The primitive computes the scope while the reviewer supplies the Summary, and no check compares the two. Read the returned counts against every number the Summary states, and re-run with corrected prose when they differ; the call is idempotent.
- **`scope` counts entries, not files.** It is the resolved scope: the plan's Affected Files entries as written, united with the paths modified since `diff-base`. A plan Affected Files pattern such as `src/handlers/*.ts` is one entry however many files it matches, so a Summary that counts files says how its count reconciles with `scope`; `examined` equal to `scope` means fully examined only when no entry is a pattern.

## Blocking semantics

A spec MUST NOT advance from `in-progress` to `done` while its `review.md`
records `blocking: true`. This is enforced as follows:

1. **`/{project}:implement`** — before marking `status: done`, its `check-review-gate`
   runs every check in a fixed order, first failure wins; the full order and
   its message texts are canonical in the pre-done gate, step 5 of
   **§Completion gate (after all tasks)** in `.ductus/reference/implement.md`, and only the two review checks are
   restated here. Ahead of them run whether the spec is already `done`, the
   feature directory's markdown lint, unresolved scenario open questions, and
   an undischarged cross-spec impact — any of which halts before the review
   record is consulted. Behind them run the review staleness check, the
   analyze checks, and — last — the two disposition checks: a `review.md`
   or `analysis.md` with no `dispositions:` map, then an outstanding gating
   finding, which for a record the runtime writes is an unwaived SHOULD. The two review checks read the
   record from `review.md`: a missing/null `last-run` — or **no `review.md` at
   all**, which is the never-run state — halts with

   ```text
   blocked: spec has not been reviewed — run /{project}:review before completing
   ```

   and only `blocking: true` halts with the MUST-violations message plus
   waive guidance:

   ```text
   blocked: spec has N MUST violation(s) — see specs/NNN-feature/review.md
   resolve the violations and re-run /{project}:review, or waive with /{project}:review --waive
   ```

2. **`/{project}:analyze`** — adds a check to its existing audit: if the spec's
   status is `done` but its `review.md` records `blocking: true` or a missing
   `last-run`, this is a validation failure; a `done` spec whose `review.md`
   records undispositioned observations is reported as advisory disposition
   drift, which `--fix` does not revert. Composable with `--fix`:
   `/{project}:analyze --fix` reverts `done` → `in-progress` and emits a notice
   (it never silently downgrades; the notice is the point).

3. **CI hook** — the shipped GHA template at
   `framework/templates/ci/adopter-generators.yml` fails when any spec at
   `status: done` has a record with `blocking: true` or a missing `last-run`,
   in `review.md` or in `analysis.md`. A `done` spec with **no** `review.md`
   (or no `analysis.md`) has never been reviewed (or analyzed) and is exempt —
   matching `/{project}:analyze`'s own grandfather rule — but that exemption is
   **bounded** by a committed high-water mark, because the set cannot
   legitimately grow. Keying it on the absence of a `spec.md` block instead is
   what the template used to do, and the relocation made that predicate true of
   every spec.

This implements the constitution's quality gate via three mutually reinforcing
mechanisms rather than relying on any single one — consistent with the
**Design Principles** rule: never depend on human diligence.

## Blocking message

Emitted by `/{project}:review` when tech-stack alignment fails (missing/empty
`AGENTS.md` `Tech Stack` section, or documented stack inconsistent with
implementation):

```text
blocked: tech-stack alignment failed — AGENTS.md Tech Stack {missing | inconsistent with code in scope}

  expected: <stack inferred from scope, e.g., "TypeScript + React frontend">
  documented: <AGENTS.md Tech Stack contents, or "(empty)">

reconcile AGENTS.md Tech Stack with the implementation, then re-run /{project}:review.
to skip this check on future runs after manual reconciliation, add
[review] tech-stack-verified = true to .ductus/config.toml.
```

## Waivers

A MUST or SHOULD violation can be waived only with explicit, recorded justification:

```text
/{project}:review --waive <rule-id> --reason "<text>"
```

This appends one entry to `review.md`'s frontmatter. Its `file` lists every
in-scope file the rule fires at in this run — a single path when there is one:

```yaml
waivers:
  - rule: SEC-BE-014
    file: src/api/internal.ts
    reason: "Endpoint is internal-only behind mTLS; rule applies to public APIs"
    waived-at: 2026-05-10T14:40:00Z
    waived-by: <git config user.email>
  - rule: SEC-BE-021
    file:
      - src/api/admin.ts
      - src/api/metrics.ts
    reason: "Both routes are bound to the loopback interface"
    waived-at: 2026-05-11T09:15:00Z
    waived-by: <git config user.email>
```

A list is one judgment — one `reason`, `waived-at` and `waived-by` — over
each path it names. A later `--waive` for the same rule records a new entry
rather than appending to an existing one, because an entry's `waived-at` and
`waived-by` attest to one judgment made once. An operator may still merge
entries that share a rule, reason and author by hand; the framework never
merges them itself.

Waived findings drop out of the `must-violations` and `should-violations` counts (there is no separate
`waived-violations` field; `write-review` reports the waived count
only in its transient result). They appear in `review.md` under the **Waived
findings** section. They survive across `/{project}:review` runs as long as the
rule ID and file location still match; if either changes, the waiver expires
and the finding re-blocks. Line numbers are not part of the waiver anchor —
the contract is `(rule, file)`, so code moving within the file does not
expire the waiver. An entry listing several paths is that many anchors
sharing their other fields: each path applies, expires and is retained on its
own, and is matched literally — a pattern such as `src/**/*.ts` is not
expanded, so it names a path that does not exist and expires as one.

## Per-run waiver processing

On every `/{project}:review` run, after the review passes have produced their
findings (see **Run review passes**) and before counting them into `must-violations`
and `should-violations` or writing the record, walk the recorded `waivers` and classify each anchor — each
path an entry lists, under its rule — against those findings. A waiver can only be judged against findings that exist — when
an empty scope skips the passes entirely, leave the waivers untouched; and on
a dimension-restricted run, waivers anchored to skipped dimensions apply
unchanged rather than expiring:

1. **Apply** when the file exists at the anchored path AND the rule still
   fires there. The finding appears under **Waived findings** in
   `review.md` with the waiver's `reason`; it is excluded from its
   severity's count, `must-violations` or `should-violations`.
2. **Expire** when either the file no longer exists at the anchored path
   (renamed, deleted, moved) or the rule no longer fires there (offending
   code fixed, rule removed, rule renamed — IDs are permanent per
   `specs/008-security-rules/data-model.md`, so a renamed rule is a
   different rule). On expiry, drop the path from the entry on the next
   frontmatter write — and the entry, when it was the last path — AND emit
   one line on stdout:

   ```text
   waiver expired: rule {rule-id} at {file} ({reason})
   ```

   The notice is the point of the action; expiry MUST NOT be silent. When
   the same rule still fires anywhere in scope after a drop, the finding
   re-counts toward its severity's count: a MUST toward `must-violations`,
   flipping `review.blocking` to `true` if it was `false`; a SHOULD toward
   `should-violations`, which holds `done` at the pre-done gate.
3. **Do not extend** a waiver to a file it does not list. If the same rule
   fires at a path other than the waiver's anchors, that is a separate
   finding; the operator records a fresh `--waive` if it is also intentional.

The record writes `file` as a single path when an entry holds one and as a
list when it holds more, so a list pruned to one path reads like any
one-path entry.

## Malformed and duplicate waivers

- A waiver entry missing any of `rule`, `file`, `reason`, `waived-at`, or
  `waived-by` is **skipped** with a one-line warning naming the offending
  entry (e.g. `malformed waiver at review.waivers[2]: missing 'reason'`).
  The entry is kept on the write; the operator must clean it up to silence
  the warning. Malformed entries are operator-authored state, not garbage
  for the framework to collect. A `file` that is an empty list, or a list
  holding a blank path, is missing too. Two exceptions: an entry with no
  fields at all holds nothing to keep, so a re-render drops it; and pruning
  matches on `(rule, file)` alone, so a malformed entry that lists an
  expired waiver's rule and file loses that path, and is dropped when no
  path remains. A `file` holding anything but a path or a list of paths
  fails the `waivers:` parse, as any mistyped field does.
- Two or more claims on the same `(rule, file)` pair, in different entries
  or twice in one list: **only the first applies**. Each later claim emits a
  one-line warning (`duplicate waiver: rule {rule-id} at {file} — entry [N]
  ignored`); only that pair of entry N is ignored, and its other paths are
  classified as usual. Duplicates are kept until the pair expires, when every
  claim on it is pruned together.

The `waivers` list follows the §text-first-artifacts open-schema
rule. Adopters MAY add fields (e.g., `co-waived-by`, `approved-by-team`,
`ticket`) to enforce org-specific waiver policy in their own CI; `/{project}:review`
and `/{project}:analyze` will not error on unknown fields, and `write-review`
preserves them verbatim on a surviving waiver when it re-renders the block, so
an org policy field is never dropped by a later review run.

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
/{project}:review — 042-example-feature

  security    ✓ 0 MUST   2 SHOULD
  reuse       ✓ 0 MUST   1 SHOULD
  quality     ✓ 0 MUST   0 SHOULD   (2 low-confidence)
  efficiency  ✓ 0 MUST   0 SHOULD
  simplicity  ✓ 0 MUST   0 SHOULD

  observations 3 — 1 fixed, 1 routed, 1 discarded, 0 undispositioned
  analyze     ✗ last run 2026-09-06 against 683a1e0 — this review supersedes it
  blocking: no
  report:   specs/042-example-feature/review.md

  next: fix or waive the 3 SHOULD violations, then /{project}:analyze — the done gate holds on an unwaived SHOULD
```

## The `observations` row

The row renders on **every** run from the `dispositions` field `write-review`
returns, as the observation total followed by all four counts — including a
run with no observations, which renders `observations 0`, because
examined-and-empty and not-computed must not be the same output. It is
informational and never affects the exit code, and an `undispositioned` count
above zero does not hold `done`, so the row names it for a later review to
decide rather than leaving it unseen:

```text
  observations 2 — 0 fixed, 1 routed, 0 discarded, 1 undispositioned — recorded; it does not hold done
```

The standing inbox count is not a review row. It is a count of the todos a
person has logged, which has no bearing on the spec under review, and
`/{project}:status` renders it on every run.

## The `analyze` row

`/{project}:review` is the command that most reliably _invalidates_ an analyze
record — it rewrites `review.md`, `--fix`
rewrites code, and an operator resolving MUST violations rewrites more — and it
used to say nothing about it. The pipeline mandates
`/{project}:review → /{project}:analyze → done` and the pre-done gate enforces
both records, so the only thing carrying an operator from a passing review to
the second gate was memory: the diligence dependency §design-principles rejects
(spec 047, `analyze-record-freshness`).

The row renders on **every** run, in one of four states, from the
`analyze-freshness` field `write-review` returns:

```text
  analyze     ✗ never analyzed — run /{project}:analyze before done
  analyze     ✗ last run 2026-09-06 against 683a1e0 — this review supersedes it
  analyze     ✓ last run 2026-09-06 against 683a1e0 — current
  analyze     ? freshness undeterminable — the record carries no analyzed-digest
```

`current` is why this is a computed row rather than a fixed reminder: a
`/{project}:review` that changed nothing leaves a genuinely current record, and
reporting it superseded would be the false alarm that teaches operators to skip
the row.

The state is a **content** comparison: the digest the analysis recorded of what
it read, against the spec's analyze subjects as they are now. It is not a
commit comparison, which is why this row and the pre-done gate cannot disagree
— there is no reference point left for them to differ on, and neither reports a
record as stale merely because content the analysis already examined has since
been committed. Writing this review supersedes the record because `review.md`
is itself an analyze subject, not because `HEAD` moved. On a `done` spec the
comparison ignores an in-place edit to `plan.md` or `tasks.md` (a deleted one
still counts), since such an edit adding no unchecked task takes no back-edge
there (§spec-lifecycle), so a plan correction alone does not supersede the
record.

The review record works the same way, over its own narrower subject set —
one comparison, two subject sets. See `reviewed-digest` under
[Record the run in `review.md`](#record-the-run-in-reviewmd).

A record carrying no `analyzed-digest` — every record written before this
existed — renders the fourth state. It is not current and not stale: nothing on
disk says what that run examined.

**The row is a notice, not a gate.** `/{project}:review` has no authority over
the `done` transition and does not acquire one here: `blocking`, the exit code,
and the review record's own `blocking` are all unchanged by it. The `next:` line
follows for the same reason — it names the command the operator owes, it does
not withhold anything.

When MUST violations are present:

```text
/{project}:review — 042-example-feature

  security    ✗ 2 MUST   1 SHOULD
  reuse       ✓ 0 MUST   0 SHOULD
  quality     ✗ 1 MUST   0 SHOULD
  efficiency  ✓ 0 MUST   0 SHOULD
  simplicity  ✓ 0 MUST   0 SHOULD

  blocking: yes — 3 MUST violations
  report:   specs/042-example-feature/review.md

  spec cannot advance to done. Resolve violations and re-run /{project}:review,
  or run /{project}:review --waive <rule-id> --reason "..." for each waivable finding.
  then /{project}:analyze, which the done gate also requires.
```

The `analyze` row renders on the blocking path too. A blocked review does not
make the second gate go away, and the fixes that clear the violations are
exactly what supersedes the recorded analysis.

Exit code: `0` when not blocking, `1` when blocking. Allows CI to gate on the
exit code without parsing the report.

## Idempotency

Re-running `/{project}:review` against an unchanged target reproduces an identical
`review.md` (modulo `reviewed-at` and `reviewed-against`). This is a
derive-don't-ask invariant: review output is a function of code + rules,
never of session state.

## Notes for adopters

- Projects that customize shipped rule files (e.g.,
  `specs/rules/security-backend.md`) pin them in `.ductus/config.toml`
  `[pinned] files` to prevent `/ductus` from overwriting their additions.
  `/{project}:review` reads whatever is on disk — pinned or not.
- Files inside the rule-file directory (`specs/rules/` in adopter
  projects; `framework/rules/` in ductus's own repo) are auto-discovered
  by directory walk (see step 2 of the command's Instructions, `discover-rule-files`). No `AGENTS.md` reference is
  required. Adding a new file at `specs/rules/<domain>-{backend,frontend,cross}.md`
  with a recognized suffix is the only step needed; the suffix selects
  which stacks load it.
- The `AGENTS.md` rule-file reference survives strictly for adopter-local
  rule files placed **outside** `specs/rules/` — e.g.,
  `docs/rules/internal-api.md`. The framework cannot directory-walk
  arbitrary adopter paths, so an explicit `AGENTS.md` reference is the
  discovery signal for these files.
- A rule file with an unrecognized suffix loads for every stack and
  emits a one-line stdout warning (see step 2 of the command's Instructions, `discover-rule-files`). The default
  is "load + warn," never "silent skip." Rename to one of the closed
  suffixes — `-backend.md`, `-frontend.md`, `-cross.md` — to silence
  the warning.
- A rule file can be explicitly excluded from a given project's review
  via `.ductus/config.toml` `[[review.disabled-rule-files]]` (see
  **Inputs** in the command file for the schema and step 2 of the command's
  Instructions (`discover-rule-files`) for the
  filter behavior). The override is project-wide and requires a
  mandatory `reason` — the reason is the audit trail. Use this when
  the stack-derived selection is correct (the rule file applies) but
  the team is not yet ready to enforce that file's rules (e.g., an
  internal admin UI that has not adopted full WCAG AA). Waivers
  remain the right tool for individual `(rule, file)` exceptions; the
  opt-out is for whole-file deferrals.
- The five-dimension model is fixed. Domain-specific concerns (accessibility,
  i18n, licensing) belong in additional rule files, not new passes.
