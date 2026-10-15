# 050 — Constitution Plan

Implements [050 — Constitution](spec.md).

## Overview

Three pieces of work over two files. Classify every rule-bearing `AGENTS.md`
entry, move the universal ones into `framework/constitution.md` leaving pointers
behind, and add two rules that were learned here but belong to every adopter —
criterion verification and the mechanical-edit test.

The work is almost entirely prose, and the risk is not that it breaks but that
it drifts: two copies of a rule, a promoted rule that still names this
repository's machinery, or a moved anchor another artifact cites. The plan is
shaped around making each of those checkable rather than careful.

## Technical Decisions

### The classification is written down before anything moves

The survey behind this spec was a judgment call recorded as three counts. AC1
asks for something stronger: every entry carries a classification *and* the
reason. So the first task produces the full table — one row per rule-bearing
entry, with its verdict and a one-line reason — and nothing is promoted until
that table exists.

The table lives in this plan rather than in `AGENTS.md`, per the clarify walk's
fourth resolution: a per-entry marker in `AGENTS.md` would add authored state to
every future entry, which §design-principles rejects as a design that depends on
author diligence.

Scope is the four rule-bearing sections — `Workflow`, `Gotchas`, `Boundaries`,
`Design Principles`. `Project Structure` and `Tech Stack` are descriptive prose
about this repository, not rules, and are out of scope. As of this plan those
sections hold 34, 16, 2 and 2 entries respectively; the pass runs against the
file as it stands at implementation time, not against those numbers.

### The reword test is applied, and its output recorded

The borderline group is decided by the test the clarify walk chose: an entry is
promoted if and only if it can be restated without naming machinery that exists
only here, *and* without losing what makes it actionable. Both halves matter —
dropping the second turns the test into "can it be made vague enough to ship".

The recorded reason is what makes it auditable: for a promoted borderline entry,
the reason names the machinery removed and asserts the rule still bites; for a
rejected one, it names what was lost. `scripts/audit/`, `runtime/`, the agent
registry, the `ductus-v*` release loop, the retired project name and the cargo
and rustup gotchas are all repo-only surfaces — an entry resting on one of them
fails the first half unless the rule survives its removal.

### Promotions land as bullets, in the section that already owns the subject

Per the clarify walk's second resolution. Each promoted rule is a bullet under
the existing section whose subject it shares — a spec-lifecycle rule under
§spec-lifecycle, a frontmatter rule under §text-first-artifacts, an
artifact-integrity rule under §drift-prevention — and a new section only when
the rule genuinely stands alone, as §recommendations did.

This is also what keeps AC10 cheap. The constitution carries 40-odd anchors of
the form `<!-- §name -->` that other artifacts cite by name; appending a bullet
inside a section moves none of them, while promoting a dozen rules to sections
would roughly double the top-level surface and put one-paragraph rules at
section rank.

### The mirror points and does not restate

A promoted entry's `AGENTS.md` line is rewritten to name the constitution
section and say nothing normative of its own — the shape §recommendations
already uses ("The rule is §recommendations … this entry is the contributor-side
mirror, not a second copy"). Two copies of a rule is precisely the drift
§drift-prevention exists to prevent, and a mirror that restates is two copies.

The check for this is mechanical and worth stating as one: grepping a promoted
rule's distinctive phrasing must find exactly one normative statement and one
pointer (AC3).

### Two rules are authored rather than promoted

**Criterion verification** (AC6) is not in `AGENTS.md` at all — it was learned
after the survey. It is universal for a structural reason: `check-artifacts`'
`criterion-path-existence` family examines specs at `done` only, so a spec that
sits in progress indefinitely never has its criteria examined, and the family
proves only that a path *resolves* — a criterion whose paths all exist while its
claim about their contents is false is invisible to it. Both halves were
observed here on 2026-08-17. Adopters run the same pipeline and inherit the same
blind spot.

**The mechanical-edit test** (AC13) settles what §spec-lifecycle's three
enumerated cases leave open: whether an edit that changes no claim is mechanical.
Today the list reads as closed, so a pure typo repair in a `done` spec's body is
argued to reopen it — which is the disproportion the held `045` chore in
`specs/inbox.md` is waiting on. The section gains a stated test rather than a
fourth enumerated case, so the next new case is decidable without another
constitution change.

### The version pin does not move

AC12, and it is a constraint rather than a deliverable. The repo-root `version`
file is the runtime acquisition pin: `/ductus` reads it to choose which release
to fetch, and `/ductus:audit` Family 20 requires it to equal
`runtime/Cargo.toml` and the newest `runtime/CHANGELOG.md` heading
(`scripts/audit/version-agreement.sh`). Bumping it alone fails that family;
bumping all three declares a runtime release that must be tagged `ductus-v*` or
every adopter fetches assets that do not exist. The constitution reaches
adopters by the **Shared Files** manifest row copying
`framework/constitution.md` to `.ductus/constitution.md`
(`framework/bootstrap/ductus.md:771`), which tracks `main` — so no bump, and no
tag. (Since 061 the archive is the latest release's by default, so such a change
reaches default-source adopters at the next release; see the spec's Resolved
Question.)

### Verification is the audit, not a reading

AC7 through AC10 are all checkable without judgment, and the plan leans on that
rather than on care: `/ductus:audit` Family 1 catches a promotion that
contradicts another document's claim, the manifest row proves adopters receive
the file, `markdownlint` covers AC9, and the anchor set is a grep. The
substantive risk the checks do *not* cover is AC5 — whether a promoted rule
truly holds for an adopter — which is why the reword test's recorded reason
carries that weight.

## Classification

Every rule-bearing entry, one verdict and one reason each (AC1). **23 to promote,
3 already promoted, 28 project-only — 54 total.**

**Entries are keyed by their `AGENTS.md` lead phrase, with the survey's positional
ID kept in parentheses as a historical marker.** The positional ID was the original
key and cannot stay one: it encodes where an entry sat on 2026-08-17, so inserting
a single entry anywhere above it renumbers every entry below, and `AGENTS.md` has
since grown from 54 rule-bearing entries to 119. Re-deriving the mapping therefore
needed `git show 9814d3ef^:AGENTS.md` rather than the file on disk. The lead phrase
is not a perfect key either, and the limit is stated rather than implied: **an entry
that is later reworded stops matching**, which is what five of these 54 had already
done (the entries now reading *never record anything durable in `review.md`*, *a
`.ductus/config.toml` change is cross-spec impact only when…*, *the generator rule
has no exceptions*, *the two frontmatter derivations are runtime primitives*, and
*never edit an installed command file directly*). Those five were re-matched by hand;
the other 49 matched mechanically. The key is chosen because it degrades from
*matches* to *needs one hand re-match*, where the positional ID degrades silently
to *matches the wrong entry*.

**Coverage, measured 2026-09-15 against `AGENTS.md` at 119 rule-bearing entries**
(Workflow 49, Gotchas 59, Boundaries 3, Design Principles 8): *unclassified =
rule-bearing − (table-keyed ∪ constitution-citing)* = 119 − 75 = **44** (Workflow 7,
Gotchas 34, Design Principles 3) — **discharged to 0 by the second round below, which
classifies exactly those 44.** Verified rather than asserted: the 98 keys both rounds
carry resolve to 98 distinct entries, and 98 ∪ 47 citing = 119, the whole rule-bearing
set. All 54 first-round entries still resolve to a live entry,
and 47 in-scope entries cite a `framework/constitution.md#` anchor. **Task 22 moved that
to 48 in-scope citing entries (and the whole-file line count from 51 to 52)** by reducing the
*recorded measurement can encode a constraint* entry to a pointer; rule-bearing is unchanged at
119 because no entry was added, and *unclassified* stays 0 because that entry was already
table-keyed, so the union did not move. Same method both times: top-level bullets tallied per
level-two heading, in-scope sections only. Count the citing
side as **rule-bearing bullets in the four in-scope sections** — a bare
`grep -c` over the file answers 51, because it counts anchor-bearing *lines*
including two outside those sections, and 51 is right about lines while 44 needs 47.
This is a **notice, never a gate**: promotion has to stay free, or the honest choice
between a growing backlog and a silent one pushes toward silence.

### Second round — classified 2026-09-15

The 44 entries the coverage check above reported as unclassified, judged by the
same reword test and recorded the same way (AC1). **26 to promote, 1 routes to a
rule file rather than here, 1 already promoted in substance, 16 project-only.**
**Amended 2026-09-15 by task 22: 25 to promote, 2 already promoted in substance.**
The *recorded measurement can encode a constraint* row moved between those two
tables when AC21 stated its disposition in the constitution directly — see the
note under **already promoted in substance** below. The totals still sum to 44
and the coverage check is unaffected, because the entry was table-keyed either way.
This is a second deliberate round, not a standing requirement — AC1 rejects the
latter and is unchanged. Promotion itself is *not* done by this task; this is the
worklist, exactly as the first round's classification preceded its promotions.

**Promoted 2026-09-15 by task 23.** All 25 rows in the promote table have their
canonical text as bullets in `framework/constitution.md`, each under the existing
section whose subject it shares, and each entry's `AGENTS.md` line is now a
pointer. The anchor set is byte-identical before and after, so AC10 holds; the
constitution went 767 → 792 lines and 163 → 188 top-level bullets, and `AGENTS.md`
went 231,882 → 198,966 bytes at an unchanged 181 lines and 120 rule-bearing
entries, with the constitution-citing side moving 48 → 73. The bolded lead phrase
of every rewritten entry is byte-identical, so both rounds' table keys still
resolve 1:1 — verified rather than assumed, all 98 keys and the second round's 26.
The one rule-file row is registered as `QUAL-DELEG-001` through `036`'s back-edge
under AC22, which is the fourth destination the three-tier classification cannot
express; this spec declares that obligation in `cross-spec-impact:` so it gates
`done` until `036` links back rather than resting on prose.

#### Second round — promote

| Entry | Reason |
| --- | --- |
| `examined` counts the in-scope files the passes *read*. A file you are confident about by other means is named in the Summary, never folded into the numerator | The field ships on every review record; the generated-mirror temptation is the adopter's too |
| The local gate is the whole CI surface, not the checks that look related to your change | **R** — stated as *pick the gate from the blast radius, not the file extension*, dropping this repo's script list |
| After pushing, read every workflow's run — not the one whose name matches what you changed | **R** — a green *chosen* workflow is evidence about itself; true of any multi-workflow project |
| Verify what a commit actually contains before reporting it landed — `git show --stat`, not the absence of an error | Git discipline behind every completion claim; `git show --stat` plus `git status --short` answer different questions |
| Read the clock and read the sha — never write either from recollection | Both values are adopter-written fields on the review and analyze records |
| A spec's review diff base moves forward on every reopen, so the window shrinks — measure it before reaching for `--since` | `compute-review-scope` ships; every adopter's base moves on every back-edge |
| `remove-inbox-item` takes the bullet text, so never drive it from line numbers — each removal shifts the lines under you | **R** — match by text and assert uniqueness; never edit by position over a list that shrinks as you walk it |
| A scenario's link to a sibling spec needs `../../`, and the depth error is the half the hook catches | Adopters have scenarios and sibling specs, and the dependency derivation ships with the cycle check |
| "Record" spans five destinations in this framework — name the artifact and section, never the verb alone | **R** — the five destinations are all adopter-facing commands with different gate semantics |
| Piping a gate command's output replaces its exit status — `cmd \| tail` reports `tail`'s success, not `cmd`'s | **R** — never pipe a command whose exit code you intend to read; the shown output is genuine and the status is manufactured |
| A `§` reference resolves by *line*, and intra-document by longest-first heading match — a line wrap alone breaks it | Adopters cite constitution anchors and run the anchor resolver |
| A review record written before `ductus-v0.49.0` carries no `examined` and no `reviewed-digest`, so a green `done` spec may be one nothing can judge | **R** — stated by capability rather than by version; adopters upgrade through the same boundary and hold the same records |
| An acceptance criterion's parenthetical enumeration is a claim of its own, and it drifts from what the artifact does | Adopters write criteria carrying enumerations, and the enumeration goes stale while the requirement holds |
| A spec with no recorded `in-progress` transition derives an *empty* diff base, and `write-review` then resolves `scope: 0` | Both primitives ship; the incoherent `examined`-over-zero record is reachable in any corpus |
| A sweep can make a quoted instruction point at itself, and the identical sentence may be correct one file over — classify by *who emits it*, never by the string | **R** — classify a swept user-facing string by *who emits it*; adopters rename things too |
| A spec body must not carry a point-in-time status section — it goes stale silently and then contradicts its own frontmatter | Pure spec-authoring rule; pipeline state is derived everywhere, not only here |
| Only a spec's durable contracts stale its review — `scenarios/*.md` and `data-model.md`, nothing else — and that is what prices a corpus sweep | The digest covers `scenarios/*.md` and `data-model.md` for every adopter; this is what prices any corpus sweep |
| `criterion-path-existence` only sees a path inside backticks — an unbackticked one is not a finding, not a skip, and not a candidate | **R** — stated as how to write a path claim so it is checkable (backticked, interior slash, repo-relative) |
| `unexamined` cannot be measured while the spec is `in-progress`, and the gate's own ordering is what hides that | The family is `done`-only and the gate ordering is the shipped one, so the trap is identical in any adopter repo |
| A spec's own completion commit can destroy content, and a `## Heading` absorbed into an unterminated code span is invisible to every check this project has — including the gate that reads it | **R** — drops the Family-25 scoping note; the blindness is in the shared markdown reader every adopter uses |
| `set-status` takes `--from` and `--to`, not `--status` — and the refusal is the useful half | **R** — the two-sided form's rationale (a transition computed from a stale read fails loudly) is the general half |
| `write-review` derives `scope` itself, so a count you measured earlier and typed into the Summary can contradict the record it is written into — and nothing compares the two | One call writes both, so nothing can compare them; true wherever the primitive runs |
| Removing an embedded copy is a corpus sweep, not a deletion — grep for what pointed at it, including the artifacts that *describe* it | The pointer-not-copy rule already ships; this is what *replacing* one costs, classified by tense |
| A `data-model.md` that claims "authoritative shapes" drifts from the runtime silently — check it against a record the runtime actually wrote, not against the code | **R** — check a schema table against an artifact the writer actually wrote, not against the code |
| A conformance test whose subject is drawn from corpus state can go vacuous, and its vacuity guard will then assert that an unhealthy corpus must exist | Extends §design-principles' check-that-cannot-run to the case where the vacuity guard inverts |

#### Second round — routes to a rule file, not here

| Entry | Reason |
| --- | --- |
| Replacing a hand-rolled predicate with a shared one widens everything it was quietly doing, not just the part you meant to fix | Universal, but it governs *code* rather than the pipeline, so §rules puts it in a `quality-cross` rule with an ID rather than in this document |

#### Second round — already promoted in substance

| Entry | Reason |
| --- | --- |
| A feature directory is not necessarily `NNN-slug` — ask `parse_feature_dir`, never the first three bytes | §numbering already states that the membership rule is defined in exactly one place and is called rather than restated; what remains here is the Rust function name |
| A recorded measurement can encode a constraint you have not found yet — when a fresh derivation disagrees with a recorded one, hunt the constraint before concluding the record is wrong | **Discharged 2026-09-15 by task 22, not by a promotion pass.** AC21 added *a recorded measurement states its method and its units* to §grounding, which states this entry's disposition outright — re-derive at the recorded figure's own commit, then diff the field across its own history — and its diagnosis, that the reflex to trust the newer figure is wrong precisely where the older one was better informed. The `AGENTS.md` entry now points at that bullet, so promoting it again would write a second copy |

#### Second round — project-only

| Entry | Reason |
| --- | --- |
| A pushed tag is not a published release, and `release not found` is the *normal* answer for the ~11 minutes in between | This project's release pipeline, its job order and its crates.io step |
| Discard an observation about the pipeline's own machinery made while reviewing framework work, with its reason — routing it is what feeds the loop | The premise — a review executed *by* the machinery it reviews — holds only for this framework's own repository |
| After changing runtime source, verify through the built binary — not the MCP tools | Adopters do not change runtime source |
| `framework/commands/help.md` is generated — hand-edits to it are reverted on the next commit | Framework build step |
| Registering a runtime primitive is five sites, and two of them are only found by tests — de-registering one is the same five | Framework build step |
| `check-artifacts`' `skipped` array names targets that could not be *reached*, never subjects that were empty | About *authoring* a check family; the interpretive half is already §design-principles |
| A prose-only edit to `framework/commands/*.md` can break a Rust test, so run `cargo test` before calling such a change verified | Framework build step |
| A new slash command wires into five places outside `framework/commands/`, and two of them are load-bearing for adopters | Framework build step |
| Never write a literal `ductus-vX.Y.Z` into a `framework/commands/*.md` prose line — `BLESS` will template it into a time bomb | Framework build step, and the trap is this repo's parity goldens |
| An apostrophe anywhere in an `/audit` family's embedded python closes the shell string holding it | The audit is maintainer-only |
| `check-orphaned-references` used to report `AGENTS.md` → `specs/rules/` on every run in this repo, and it was never a finding. Fixed in `ductus-v0.49.4` — the primitive now excludes it itself, and this repo reports `findings: 0` | Turns entirely on this repository keeping its rule files outside the adopter-side path |
| In a `set -e` script, `[ test ] && cmd` as the *last* command of a block exits the script when the test is false | Shell idiom in this project's own tooling rather than pipeline governance |
| clap expands `{n}` in a doc comment as a newline, so a `#[derive(clap::Args)]` field's docs cannot contain one | Runtime source detail |
| Editing `framework/commands/implement.md` stales the `implement-basic` parity golden, because the golden records the fixture repo's commit shas | This repo's parity fixture |
| `cargo` exits 101 from the repo root, and 101 is also what a failing test suite returns — so a mis-rooted probe reads exactly like the failure it was meant to prove | This repo's toolchain and the harness's working-directory behaviour |
| `grep` here is ugrep, and a bounded-context pattern exceeds its complexity limit — erroring once per file, which reads exactly like a scan that found nothing | This machine's tooling, not the framework's |

`R` marks a verdict reached through the reword test rather than directly.

### Third round — classified 2026-09-28

The 9 entries the promotion-coverage notice reported unclassified on 2026-09-28, judged by the same reword test and recorded the same way (AC1): **1 to promote, 1 routes to a rule file rather than here, 7 project-only.** The verdicts are the operator's, confirmed 2026-09-28. Like the second round, this is a deliberate round rather than a standing requirement, and AC1 is unchanged.

**Promoted the same day by task 28.** The promote row's canonical text is a paragraph in §scenarios, beside the scenario-creation paragraph its subject matches, and the entry's `AGENTS.md` line is now a pointer. Reading the primitive before promoting showed the entry had drifted. It named a phased `tasks.md` as one of its two no-write shapes, but `append_task.rs:94-118` has the scenario-slug dedup and the title dedup 058 added, while a phased file without a heading writes into a follow-on phase. The constitution states the code's two, and the pointer records the correction. The anchor set is unchanged. The rule-file row is `QUAL-TEST-001`, registered through `036`'s back-edge as the second round registered `QUAL-DELEG-001`. This spec's `cross-spec-impact:` already names `036`, and 036's signpost records this round.

#### Third round — promote

| Entry | Reason |
| --- | --- |
| `append-task` has two shapes that both return exit 0 while writing nothing, and neither is an error | **R** — stated as reading a primitive's returned fields rather than its exit status; both dedup shapes reach every adopter's task appends through groom, amend and implement |

#### Third round — routes to a rule file, not here

| Entry | Reason |
| --- | --- |
| A test must fail when the behavior it names is removed | Universal, but it governs *code* rather than the pipeline, so §rules puts it in a `quality-cross` rule with an ID, `QUAL-TEST-001`, as the second round routed `QUAL-DELEG-001` |

#### Third round — project-only

| Entry | Reason |
| --- | --- |
| No dead code — remove what nothing calls, reads, or reaches in the change that makes it dead | Recorded as this repository's rule at the operator's request (2026-09-25); promoted, it would govern code and route to a rule file |
| Two extension points fan out in the exec walker, so neither is a stand-in for "one request, one response" in a test | This runtime's exec walker and its rule fixtures |
| A word starting with `=` is a command lookup in zsh, so an unquoted `======` separator kills the command it sits in | A shell trap with no pipeline connection, like the zsh `path` entry |
| Mutating code to prove a test fails: run unit tests in the dev profile, and rebuild the release binary after any parity mutation | This repository's cargo profiles and parity harness |
| A boolean argument is a valueless flag on the CLI and a JSON boolean over MCP — `--checked true` is an error, `--checked` is the call | Tool-call hygiene for this runtime's CLI; generalized it stops biting, like *use repo-relative paths in tool calls* |
| The project root is not the git work tree's root, and every path that crosses between them must be converted | Runtime and command-source authoring; an adopter receives the behavior, not the rule |
| An `assessSpecQuality` step's prose chooses the tier of rules the exec walker asks about, so a `-tier` word added to that prose can change what the step covers | Concerns authoring this framework's command sources |

Two entries were reclassified after checking the constitution rather than the
survey: **W14 and W15 are already promoted.** They are cases (b) and (c) of
§spec-lifecycle's mechanical-edit rule, and both `AGENTS.md` entries already
cite the section by anchor — they were in the target shape before this spec
started. The survey counted them as pending because it read `AGENTS.md` alone.

That check also surfaced a defect this promotion fixes rather than causes.
§spec-lifecycle's back-edge paragraph cites "the **Design Principles** rule:
never depend on human diligence" — a rule that lives in `AGENTS.md`, which the
**Shared Files** manifest never ships. So the one artifact every adopter
receives cites a rule none of them can read. Promoting DP2 resolves the
dangling reference; DP1's substance appears at §recommendations as a supporting
clause rather than as a principle in its own right.

### Promoted — universal

| Entry | Reason |
| --- | --- |
| A spec does not reach `done` with an outstanding SHOULD — every review finding is resolved or waived first, not only the MUST ones (survey W4) | Every adopter runs the review command and inherits its blocking semantics; a spec sitting at `done` with an unaddressed finding is indistinguishable from unfinished work anywhere |
| Never record anything durable in `review.md` — it is regenerated wholesale (survey W6) | Specs, criteria and `review.md` are all adopter artifacts, and `write-review` regenerates the Summary wholesale for them too. First promoted inside the superseded-criterion rule, which `054-remove-supersession` withdrew from the constitution on 2026-08-31, leaving the residue as normative text in `AGENTS.md`. Task 26 re-promoted the residue on its own under §implement-phase, worded for the waiver and decision lists 058 made persistent |
| Record a review's `reviewed-against` as a commit that already contains the artifacts it reviewed — so commit the work first, then review, then commit the review (survey W7) | **R** — drops the Family 19 citation; the ordering rule (commit, review, commit the review) stands on its own and adopters' freshness check reads the same field |
| A `.ductus/config.toml` change is cross-spec impact only when it modifies a key another spec already documented (survey W10) | Adopters add config sections per spec; the anti-pattern of reopening the config's originating spec is the same |
| No dead references in live artifacts (survey W11) | **R** — the artifact list is restated generically (specs, commands, rules, docs, README) rather than naming this repo's directories |
| A behavior change needs a prose-claim sweep, not just an identifier sweep (survey W12) | Identifier greps miss stale claims in any project; the failure mode is the wording, not the paths |
| Never hand-write an `AC{n}:` label — add the criterion unlabelled and run `label-criteria` (survey W16) | The counter is adopter frontmatter and the collision risk is identical |
| Use the `Write` tool, not Bash redirects, for `.ductus/session.toml` (survey W18) | **R** — stated as the session file rather than a permission-entry anecdote; adopters carry the same per-path grants |
| Re-open a `done` spec via `set-status` when the only intent is to reflect on-disk edits, not via `/{project}:amend` (survey W21) | The back-edge and the refinement loop are both adopter-facing |
| Add a rule to its surface's home spec via the back-edge — do not spawn a new spec for it (survey W22) | Adopters own rule files and hit the same spec-proliferation pressure |
| A canonical table living on a `done` spec is synced as a mechanical edit — no back-edge (survey W24) | Follows from the canonical-sources map, which adopters receive |
| A CI check that reads git history needs `fetch-depth: 0` (survey W27) | **R** — the shipped adopter CI template runs exactly such a check, so this is adopter-facing already |
| A new test that reads git history or shells out to a repo script is a change to the workflow that runs it, not just to the suite (survey W28) | **R** — same family as W27; stated as history/inputs rather than this repo's workflow filenames |
| When an item routes to a chore, fix it — do not park it (survey W29) | The groom command and its five routes ship unchanged |
| `git checkout -- specs/{feature}/` silently reverts uncommitted pipeline state (survey W30) | Status flips and ticked criteria are tracked-file writes in every adopter repo |
| Renaming this repository orphans contributor-local state that no migration can reach (survey W31) | **R** — generalised from this rename to any; no migration can reach state keyed to a path outside the repo |
| Before reporting that a declarative entry misbehaved, read the entry (survey W32) | Adopters run the migration registry and read its gating fields; this is §grounding applied to a data row |
| Never `git add -A` / `git add .` in this repo (survey G5) | The untracked-draft hazard is created by the pipeline itself, which adopters run |
| A markdown link in a spec body creates a `dependencies:` edge — cite in prose when you mean a citation (survey G11) | The dependency generator ships; citing versus depending is an adopter distinction |
| `create-scenario` appends its own Open/Resolved Questions scaffolding — do not author those headings in the body you pass it (survey G12) | An adopter-facing primitive with adopter-facing output |
| Never edit an installed command file directly — edit the source the generator copies from (survey B2) | **R** — stated as "the installer overwrites it; pin it instead" rather than naming this repo's generator, which is what makes it matter to an adopter |
| A check that cannot run must never look like one that passed (survey DP1) | Already `QUAL-CLAIM-001` in a shipped rule file; the design-time statement belongs beside it |
| Never design framework features that depend on human diligence or discipline (survey DP2) | The hardest constraint on any pipeline artifact an adopter authors |

### Already promoted

| Entry | Reason |
| --- | --- |
| Work a recommendation out to its result *before* presenting it (survey W5) | Promoted 2026-08-17 as §recommendations; the entry is already the pointer-shaped mirror this spec generalises |
| Criterion-label assignment is mechanical-class (non-reopening) (survey W15) | Adopters receive the labelling pass and the same back-edge question |
| Cross-service reference edits are mechanical-class (non-reopening) (survey W14) | Cross-service references ship to adopters; the non-reopening rule is part of that contract |

### Project-only

| Entry | Reason |
| --- | --- |
| Commit directly to `main` — this repo uses trunk-based development (survey W1) | This repo's trunk-based flow; adopters choose their own branching |
| Never create a repository named `govern` under this account again, and never reuse a retired project name (survey W2) | Concerns this project's distribution redirect, not an adopter's pipeline |
| A change ships via a `ductus-v<version>` tag — the commit alone reaches no default-source adopter (survey W3) | Adopters have no `runtime/` and cut no release |
| Never record another project's name in this repository — describe the shape instead (survey W8) | Arises from using outside projects to test this framework |
| Read `framework/commands/{name}.md` before recommending, describing, or disambiguating a s (survey W9) | Loses nothing an adopter needs that §grounding does not already state |
| A change to an agent-facing surface must cover every agent in the registry, not just Claude (survey W13) | The registry is the framework's own enumeration |
| Run `/ductus` per its spec — no ad-hoc prompts (survey W17) | Canonical statement already ships inside the installer itself; this is its mirror |
| Use repo-relative paths in tool calls (survey W19) | Agent tool hygiene rather than pipeline governance; genericised it stops biting |
| Never call specs "frozen archaeology" or use frozen-archaeology phrasing (survey W20) | The substance (specs are living documents) is already §spec-lifecycle; what remains is a phrasing ban local to this repo |
| Route shared runtime work to spec 022 via the back-edge — 022 is the durable home for the runtime's shared surfaces; a feature-specific primitive lives with its feature (survey W23) | Names this repository's own spec |
| Backtick a primitive in an `## Instructions` step only when the walker can actually supply its arguments (survey W25) | Concerns authoring framework command sources |
| Do not invoke `cargo build` twice in one shell command and read only the second output (survey W26) | A cargo-specific shell trap in this repo's toolchain |
| A real adopter run is the only test of composition this project has (survey W33) | About testing this framework against adopters |
| A run summary is evidence about what that agent wrote, not about what the framework does — verify every other claim against the source (survey W34) | Restates §grounding's prefer-the-source rule for a narrow case; the general rule already ships |
| Use `npx markdownlint-cli2` to run markdown linting — do not suggest installing it globall (survey G1) | Tooling preference, not a rule |
| The command generator substitutes `{project}` → `ductus` and `{cli-config-dir}` → `.claude (survey G2) | Framework build step |
| An edit to `framework/commands/*.md` is invisible to that slash command until the generator re-runs — including to the command you are editing, in the same session (survey G3) | Framework build step |
| The generator rule has no exceptions — every file under `.claude/commands/ductus/` is generated output, and one with no source is drift for the prune loop rather than a file to hand-maintain (survey G4) | Framework build step |
| The two frontmatter derivations are runtime primitives, not shipped scripts (survey G6) | Framework authoring |
| `write_atomic_bytes` (the create-then-rename helper backing every primitive write) lands t (survey G7) | Runtime internals |
| `runtime/rust-toolchain.toml` pins `components = ["clippy", "rustfmt"]`, and every `cargo` (survey G8) | Runtime build |
| A new `ductus` runtime primitive wires into more than `primitives/mod.rs` + `mcp/server.rs (survey G9) | Runtime authoring |
| `<!-- audit:ignore-promotion -->` silences the framework self-audit's Family-9 (primitive- (survey G10) | Framework command authoring |
| A version bump needs one build without `--locked` (survey G13) | Runtime release |
| Gitignored adopter state survives a `git` reset, so a "reset" subject is not a fresh one (survey G14) | About testing adopters, not being one |
| This repo's dogfooded copy of an adopter file is not the copy adopters get — test the shipped one (survey G15) | Structurally impossible for an adopter, who has only the shipped copy |
| `path` is a reserved array in zsh and clobbering it destroys `PATH` (survey G16) | A shell trap with no pipeline connection |
| No host-level enforcement of the deterministic (`ductus`) path — it is a deliberate non-goal (survey B1) | A framework design non-goal |

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `framework/constitution.md` | Modify | Receives every promoted rule as a bullet under the section owning its subject, plus the criterion-verification rule, the mechanical-edit test in §spec-lifecycle, and a §drift-prevention canonical-sources row naming this spec |
| `AGENTS.md` | Modify | Each promoted entry rewritten to a pointer; project-only entries untouched |
| `specs/050-constitution/plan.md` | Modify | Carries the classification table — the audit trail AC1 requires |
| `specs/inbox.md` | Modify | The held `045` chore is resolved against the new mechanical-edit test and removed |
| `specs/045-decision-state-drift-detection/spec.md` | Modify | The one-word sweep-residue repair the chore describes, once the test licenses it |

## Trade-offs

**The classification table lives in a plan, which is a design record.** Plans are
not durable contracts, so nothing gates on this table staying accurate as
`AGENTS.md` grows. The alternative — a per-entry marker in `AGENTS.md` — was
rejected in the clarify walk because it depends on every future author
remembering to classify. Accepted limitation: the table is a snapshot of one
pass, and a later promotion round re-derives rather than amends it.

**Promoted rules lose their war stories.** `AGENTS.md` entries carry the failure
that produced them ("surfaced 2026-08-17 when…"), which is much of what makes
them stick. An adopter-neutral restatement cannot cite this repository's
incidents, so the constitution gets the rule and the mirror keeps the story.
Considered and rejected: shipping the incident text too, which would put
this project's history into every adopter's constitution.

**The reword test is a judgment, applied by one reader.** It is more auditable
than a vote — the reason is recorded per entry and can be disagreed with
specifically — but it is not mechanical, and two readers could classify a
borderline entry differently. Accepted: the alternative was deferring the whole
borderline group, which costs adopters ~10 substantively universal rules.

**Nothing verifies AC5 mechanically.** No check proves a promoted rule holds for
an adopter; a rule can pass every gate here and still read as advice about
somebody else's repository. The reword test's recorded reason is the only
defence, and it is a written argument rather than a check.

**Bullets are less citable than sections.** A rule promoted as a bullet has no
anchor of its own, so another artifact can cite only its containing section.
Accepted per the clarify walk: the alternative doubles the constitution's
section surface, and a rule that later proves worth citing directly can be
promoted to a section without moving any existing anchor.
