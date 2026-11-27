# Developing `ductus`

Procedures for changing this repository, each read when doing the job it names. A rule an agent needs before acting in any session stays in [`AGENTS.md`](../../../AGENTS.md).

## Adding or removing a runtime primitive

Registering one is seven sites, and three of them are found only by tests; removing one is the same seven:

- `PRIMITIVE_REGISTRY` in `runtime/src/schema/registry.rs`
- its one description, a constant in `runtime/src/schema/descriptions.rs`, which the `#[tool]` and the `Command` below name
- a `#[tool]` in `runtime/src/mcp/server.rs`, or a `CLI_ONLY_PRIMITIVES` entry
- a `dispatch_primitive` arm in `runtime/src/interpreter/mod.rs`
- a `Command` in `runtime/src/main.rs`
- a tool or a `# cli-only:` line in `framework/runtime-tools.txt`
- a case in `runtime/tests/result_sizes.rs`' `case`, which measures its result, or a not-measured row with its reason; `every_registry_name_has_a_case` fails without one

**A result with two shapes** returns one struct with two flattened `Option` halves, and the halves must never share a key. rmcp requires an object output schema, which rules out an untagged enum (`prune-tasks`, `prune-plan`), and serde's flatten consumes a shared key even when that half fails. Keep a round-trip test per shape.

## Adding a slash command

A new command wires into five places outside `framework/commands/`, and two of them are load-bearing for adopters. The hook, Families 16 and 21 and `scripts/gen-help-tables.sh` cover four. Nothing flags the fifth: the list in `runtime/src/parser/mod.rs`'s `rewritten_command_files_parse_with_document_step_numbers`.

A command with a reference takes two more ([071](../../../specs/071-command-reference-split/spec.md)): the reference itself at `framework/reference/<name>.md`, never under `framework/commands/`, where every file is a command, and its row in `framework/bootstrap/ductus.md` §Shared Files, `framework/reference/<name>.md` → `.ductus/reference/<name>.md`, which Family 16 fails a reference without. The command's steps then name its sections in the pair form **§Heading** in `.ductus/reference/<name>.md`.

## Adding a member to a closed set

A family, a skip reason or a marker that `check-artifacts`' families use is a corpus sweep, and Family 18 binds only the marker lists. Before the release commit, grep an existing sibling (`root-absent`) across `framework/`, `docs/`, `runtime/src/` and `specs/`, then update every list and count naming it, or drop a needless count.

## Adding a rule category

Register a new category in its file's introducing-spec data-model in the same change, or `/ductus:analyze` rejects the whole rule file: `framework/reference/analyze.md` (*Which ID format governs a file*) reads it first, while `scripts/lint-rule-ids.sh` accepts any category. Update the data-model and the file's header format line together.

## Proving a test fails

Run unit tests in the dev profile, and rebuild the release binary after any parity mutation. `scripts/mutation/run.py` runs unit tests only; add a mutation with its killing test, and run `--check` after editing anchored code. A filter is the test's path in the crate, a child module's included (`primitives::prune_project::moves::tests::…`); one matching no test runs none and reads SURVIVED, so read a survivor's filter before its code (087, 2026-11-26). A list with every mutant killed can still leave tests unproven: to judge that each test a spec added fails without its behaviour, take every `#[test]` function its commits added under `runtime/` that is still present, and subtract the killing tests `run.py`'s `KILLED` lines name. 087's review found 15 of its tests named by no mutant that way while all 82 mutants were killed (2026-11-26). A parity mutation is a separate `cargo test --release` run that leaves `runtime/target/release/ductus`, which `.ductus/bin/ductus` and the MCP server run, a mutant, so run `cargo build --release` after it.

**Changing what a shared predicate or a fixture's record means can hollow a test elsewhere, so run the whole list over the code it reaches.** Holding a record with no `dispositions:` map left M25's test in `scripts/mutation/scope-attribution.py` passing for that reason alone, and the mutant surviving was the only sign.

**Run a large list from `git worktree`s of the commit, not this checkout.** Its `runtime/target` grows without bound and slows every build: each mutant took 34–111 s in a checkout whose `target/debug` held 255 GB, against 3–9 s after one fresh build in a worktree (`run.py`'s per-mutant times, tasks 1 and 5 of spec 079). `run.py` finds its tree from its own path, so `git worktree add --detach <dir> HEAD` per worker and `python3 scripts/mutation/run.py <list> <ids>` from each `<dir>` split a list across workers, while this checkout stays free to edit. Commit the list and the code first, since a worktree holds only what is committed, and `git worktree remove --force` each when done. For work not yet committed, point `CARGO_TARGET_DIR` at a fresh directory instead: mutants took about 4 s each that way, and the suites follow it. Give each tree its own: a test binary built from a worktree since removed reads that worktree's files through `CARGO_MANIFEST_DIR`, and cargo reuses it from a shared directory, so the suite fails by the hundred on missing files. The hook and the audit still run `runtime/target/release/ductus`, so build that plainly before committing, and never edit under `runtime/` while `run.py` runs.

## Landing a runtime change

Gate the commit on every check's exit status, mutants included, in one script: a follow-up that assumes a landing passed records a commit that was never made. A landing takes about 8 minutes, so batch fixes rather than land each.

1. From a worktree of `HEAD` with this checkout's diff applied and its untracked files copied, since a patch carries none, and its own `CARGO_TARGET_DIR`: `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, the whole `cargo test --no-fail-fast`, and each new or re-anchored mutant by id. Without `--no-fail-fast` cargo stops at the first failing test binary: a worktree with no `npm ci` fails one lib test on markdownlint and then runs no integration test, which reads like a full run (083, 2026-11-23). An uncommitted edit there joins no other session's scope.
2. Here: `cargo build --release --locked`, since the hook and the audit run that binary, then `framework-checks.yml`'s and `generators.yml`'s steps: `python3 -I scripts/run-workflow-steps.py .github/workflows/framework-checks.yml .github/workflows/generators.yml`. Its generator-diff step reads this uncommitted tree, so before a commit it fails by construction. So does its self-audit step (h) when the change edits a `framework/commands/*.md` source: the audit's check-zero runs `gen-claude-commands.sh --check` before `generators.yml`'s steps regenerate the installed copies, so run `scripts/audit/run-all.sh` again after the run, which then passes. Run the steps alone: their `npm ci` reinstalls `node_modules`, and a `cargo test` running beside them fails its markdownlint test (087, 2026-11-26).
3. Sweep what the change leaves stale, with it staged here, before committing it and before a fix round's `brief-re-read`. A change edits one statement and leaves the ones that depend on it. 022's review fix rounds of 2026-11-17 hit four kinds:
   - a header still saying "two kinds" after the change added a third;
   - a byte count another landing had already moved;
   - a test fixture copying a contract string the change narrowed;
   - a reader still dropping what the change's rule says to report.

   A re-read finds such slips one round at a time, and each costs the round's budget, so do these three things first, for an artifact-only change too:
   - **Every literal the diff removes or replaces** is listed with each remaining hit in the live-artifact set AGENTS.md lists by `scripts/stale-literals.sh BASE`.
     - BASE is the commit the change starts from: `HEAD` before a commit, a fix round's base before its brief.
     - It covers backticked spans and multi-digit figures. Search by hand for any other literal the diff replaces, passing the paths as an array, since zsh does not split an unquoted one.
     - Update each remaining hit, or name why it stays.
   - **Every enumeration, count and header** in a file the diff touches is read against the file as changed.
   - **Every figure the change states** (a byte size, a count, a line) is measured again on this tree. A change written from an older base, by a parallel worker among them, measured it before other landings moved it.
   - **A cost keyed on rule IDs** is measured with each surface selection `/ductus` installs (backend and cross, frontend and cross, cross alone), not only the 12 files this repository loads: 088's first AC2 fix judged no rule needlessly here and 13, 20 or 8 in a single-surface adopter (2026-11-26).
4. Check the index is empty, stage by name, and commit with `DUCTUS_SKIP_RUNTIME_CHECKS=1`.
5. `git show --stat`, `git status --short`, then `scripts/audit/run-all.sh`, reading its exit status.

A mutant that survives is a finding: read why before re-landing. After an edit to anchored code, `run.py --check` names what moved. A fixture record the parser refuses makes a test pass for the wrong reason, so a run must be visible in what it asserts. An edit to `framework/commands/implement.md` or `framework/commands/review.md` moves its `-basic` golden, whose fixture repo's commit shas it records: re-bless that one alone, as its message says, and read the diff.

## Measuring a run's cost

A figure names the commit it was taken at, so take it at a commit already pushed: a rebase rewrites every local sha, and a figure naming one names a commit no clone holds (086's task 1, 2026-11-26). Push the commits the change sits on first, or after the rebase re-measure in a detached `git worktree` at the pushed parent with the change's files checked out and its own `CARGO_TARGET_DIR`, then amend the unpushed commit. Time with nothing else running: a concurrent build and test run stretched one walk from 2.4 s to 3.9 s. A timing harness the change does not keep is an ignored test added and removed before staging, and the commit message says so.

## Comparing two builds across the corpus

Copy each binary out of its target directory first, and check the copy holds what it should with `strings <copy> | grep '<a new literal>'`. `cargo test` runs `cargo build --release` in `CARGO_TARGET_DIR` (`runtime/tests/exec_subprocess.rs`, `concurrent_sessions.rs`), so a binary left there is overwritten by the next landing, and a comparison of it against the new one reads 0 differences, which looks like a clean result. `scripts/audit/run-all.sh` builds it too, in its check-zero, so a binary copied into a `git clone --local` probe is replaced by the clone's own build at the first audit run there: apply the change to the clone, or run the audit last and check `strings` again after it (084 task 8). Build the old one from a worktree of its commit into its own target directory. Run from a clean worktree of `HEAD`, since an uncommitted edit joins every spec's scope and the gate lints. Give it a real `node_modules` directory holding a link to the checkout's `node_modules/.bin`: `.gitignore`'s `node_modules/` matches no symbolic link, so a linked `node_modules` is itself an untracked path in every spec's scope.

## Sizing a review's parts

A subagent reviewer finishes a part only when the part fits the reading it will actually do. Measured on 022's review on 2026-11-16 and 2026-11-17, a part is the nine backend rule files (285,666 bytes) plus the spec's acceptance criteria and the scenarios and plan sections its files bear on, plus its own slice:

- Six reviewers given about 1.0 to 1.3 MB each all stopped on their own, at 157K to 482K tokens, with most of the part unread.
- Twenty-three given about 550 KB each (slice at most about 160 KB of hunks, or one large file whole) all finished, at 275K to 525K tokens, mean 387K.

So size a part by bytes read, not by entry count. Name the contract each part reads instead of the whole spec, plan and data model: the command's normative sources are the rule files, `AGENTS.md`'s sections, the acceptance criteria and the scenarios (`framework/reference/review.md` §Load rules). A reviewer that stops short with a named gap can be asked to finish it with `SendMessage`, keeping its context; the two asked to finish took 34K and 57K more tokens. A fix-round re-read's brief carries the spec, data model and scenarios whole, so keep each round's diff small enough to fit beside them. A re-read can write `"complete": true` over a path its brief names and it never opened, another session's uncommitted file among them: ask it which paths it read before taking the result (066's review, 2026-11-18).

Since 020's AC17 a review loads only the rule files whose header names it. Here that is the three `security-*.md` files, `quality-cross.md` and `configuration-cross.md`, 232,805 bytes (`discover-rule-files`, 2026-11-21), so a part now carries less rule text than the measurement above. On 2026-11-21 one passes part over two or nine small windows, and one criteria part per spec or group of related specs, each finished at 120K to 400K tokens.

On 2026-11-26 087's review, 79 scope entries and about 1.1 MB to read, took four passes parts sliced by behaviour, each running all five passes over all five rule files (231,962 bytes, `wc -c`, read whole) beside a 234 to 275 KB slice, about 500 KB a part, and two criteria parts (the agent's recommendation, adopted by the operator). The passes parts finished at 389K to 408K tokens, and 403K to 489K after re-running over the fix diff; the criteria parts at 171K to 289K.

On 2026-11-26 088's review, 39 scope entries and about 1.02 MB read with every file up to 95 KB whole (the agent's recommendation, adopted by the operator), took four passes parts by behaviour, each beside a 230 to 290 KB slice, finishing at 278K to 401K tokens with their re-runs over the fix diff, and one criteria part at 202K to 215K. Its review after a reopen, 21 entries of which 8 changed, took one passes part at 318K to 326K and one criteria part at 183K to 189K. Call `discover-rule-files` with no `detected-surfaces`: `tech-stack-verified` skips detection here, and passing `backend` selects four files, leaving out `security-frontend.md`.

A criterion saying a rule file's claims match the current text of the documents its Sources cite is judged only by fetching each document, never from recall (constitution §rules). 068's, 069's and 070's took 42, 62 and 54 fetches (2026-11-21). Brief that part to fetch, since a criteria part judged two such criteria from recall before it was told to.

In one shared tree:

- Write and commit each spec's review record before the next spec's first `write-review`. An uncommitted `review.md` is in every later spec's scope: 034's first write on 2026-11-21 counted 024's record in its scope.
- Take `start-fix-rounds`' base before the run's first fix, as step 10 says. A fix made first is inside the base, so no round re-reads it.

## Sizing an analysis's judges

`/{project}:analyze`'s rule judges read the design subjects whole beside their rule part, so size a cell by bytes read too. Measured on 022's analysis of 2026-11-17: 310 rules in 368 KB of rule text, a spec, plan and data model of 208 KB, and 78 scenarios of 397 KB. Nine cells, three rule parts by three scenario groups balanced by bytes, each about 450 to 480 KB, all finished with whole reads, at 264K to 306K tokens. 088's analysis of 2026-11-26, three cells of 176 to 180 KB with no scenario, finished at 157K to 163K tokens a judge and 105K for step 15's reader; each later re-judge of a diff by `SendMessage` took 10 to 22 seconds, and a run against 8 changed sections about 35K tokens more a judge.

Run an analysis here with `scripts/analyze/` (spec 060), from the repository root with `python3 -I`, each script taking the feature. Where a script falls short, fix it; never rebuild it in a temp directory. Each one's docstring holds its options and exit statuses.

1. `detect.py` runs the deterministic steps and prints what each found.
2. `split.py` writes the cells manifest: shares sized as above, and scenario groups only when a cell would overflow.
3. `briefs.py` writes every brief and prints each subagent's prompt.
4. Start step 15's reader, then the judges.
5. `check.py` checks each hand-back. Ask a failing judge to finish with `SendMessage`, then run it again.
6. `combine.py` merges the cells.
7. Write the plan file `record.py` takes. Run it once as a dry run, then with `--write`.

Steps 6 and 7 apply these rules:

- **Merge each rule's answers across its cells.**
  - All not fired is not fired.
  - Addressed, with no finding, is addressed, citing the core subject.
  - Every cell a finding is a finding.
  - A finding beside a cell that found none is unsettled. Settle it with one fresh reader, given the rule, the answers and the cited sections, and enter the outcome in the plan file's `settled` map.
- **Read each finding against its rule before recording it.** `settled` also overrides a verdict the host finds wrong.
- **Run the deterministic steps and step 15's reader before the judges.** A fix either one prompts edits a design subject.
- **Close step 15's class, not its instance.** Its reader names stale present-tense claims a few per round: 056's analysis took four rounds and 026's three (2026-11-18). After its first such finding, read the spec and plan whole for the class yourself and fix every one before the judges read. In a tree another session edits, run `detect.py` and `record.py` back to back, since an edit between them makes the record refuse.
- **`check.py` refuses a read account keyed by anything but the exact path.** Judges have keyed the share as a label around it (067's and 048's analyses, 2026-11-19); ask that judge to rewrite the one key with its value unchanged, then run `check.py` again.
- **Re-judge a design-subject edit made after the judges read by sending each judge the diff** (`SendMessage`, which keeps its context), with the changed lines to read whole, not with a fresh run. Re-run `split.py` and `briefs.py` first, which `check.py` requires, and give each judge its new output path and its share's new path, under which its `bytes-read` must key the share: `check.py` looks for this round's share file, which is byte-identical to the last when no rule file changed, and refuses an account keyed to the old one (017's and 018's analyses, 2026-11-19). 022's re-checks took about ten seconds a cell. `split.py` also regroups the scenarios whenever a subject's size moves, so compare each cell's new subjects with its judge's read account and name the newly assigned scenarios for it to read whole: two scenario edits in 022's analysis of 2026-11-19 moved 0 to 6 scenarios into each of its 12 cells.
- **Key a carried finding to its stored decision in the plan file.** `record.py` matches a step-15 item to a plan finding by its exact `message`, and a stored decision by `decision-key`, so a finding a stored waiver covers takes the reader's message and the stored key. A rule finding has no `decision-key`: `record.py` keys it `rule-assessment — {rule}: {message}` from the judges' wording, so one worded unlike its stored decision is keyed to it through the plan's `settled` map, with the judges' verdict and location and the stored message (073's analysis, 2026-11-24). A step-15 re-check after an edit rewords the items it keeps, so take each live finding's message from the newest output and each fixed one's from the pass that raised it, pairing the two by what they claim, never by text (020's re-analysis, 2026-11-22). `detect.py`'s own findings (`check-artifact-size`'s, say) are recorded only when the plan lists them, and a re-measured size changes the key, so re-enter it with the stored reason (079's analyses, 2026-11-21).
- **Audit a clean tree.** `record.py` refuses once the working tree changes after `detect.py`, and `compute-review-scope` counts an uncommitted path whoever's it is, unless another open spec's plan holds it (spec 084, AC23), so an agent editing the tree blocks every analysis record and widens every review's scope. Run the implementation lane to its commits first.
- **A run against changed sections can reuse the last run's judges.** When their shares are byte-identical and their subjects changed only by a diff, send each judge the diff and the new brief rather than starting fresh: 079's four re-judges took about 13 seconds each (2026-11-21).

## Running a second session

Each session needs its own checkout: on a fixture, one session's uncommitted file in a shared tree joined the other's review scope and took its gate from passing to `review-owed` until it was gone (079's Resolved Questions), and the pre-commit hook and `.ductus/bin/ductus` are shared there too. So is the index: a bare `git commit` takes everything staged, and in one shared tree another session's staged `framework/bootstrap/` hunk landed in 66203361 under the other session's message (2026-11-18), where `git commit -m … -- <paths>` would have committed only its own paths. Such a pathspec commit that includes a `framework/commands/` or `framework/reference/` source commits the hook's regenerated copies, and a named file the hook's `--fix` or a derivation rewrote commits rewritten, and the hook carries each into the shared index too, leaving every other staged entry as it was (017's scenario `a-path-list-commit-leaves-the-shared-index-current`). Two clones, each on `main`, rebase past each other cleanly, and a record's `reviewed-against` no longer being an ancestor afterwards changes neither the gate nor the scope.

```bash
git clone https://github.com/stonean/ductus ../ductus-2 && cd ../ductus-2
npm ci
(cd runtime && cargo build --release --locked)
mkdir -p .ductus/bin && ln -s ../../runtime/target/release/ductus .ductus/bin/ductus
bash scripts/install-hooks.sh
cp ../ductus/.claude/settings.local.json .claude/
```

Keep each session on its own specs. `git pull --rebase` before each push, and after pulling runtime changes rebuild and restart the session, since the MCP server keeps the binary it started with.

## Running a review or an analysis from a worktree

A review or analysis run in a shared tree reads every other session's uncommitted file into its window and its fix-round briefs, so run each in a clean worktree of `origin/main`, one per spec (027's, 048's, 062's and 067's on 2026-11-19):

```bash
git worktree add --detach <scratch>/wt-<NNN> origin/main && cd <scratch>/wt-<NNN>
npm ci && (cd runtime && cargo build --release --locked)
mkdir -p .ductus/bin && ln -s ../../runtime/target/release/ductus .ductus/bin/ductus
runtime/target/release/ductus write-session --cli-config-dir .claude
```

Without the last two lines `check-orphaned-references` reports false findings. In the worktree use only the CLI, since the `mcp__ductus__*` tools act on the session's own checkout, and commit there.

- **Review.** Follow `framework/commands/review.md` through its primitives' CLI forms, with steps 3 to 7.2 split among three subagents that share one brief in `$TMPDIR`:
  - A, the security pass over the three `security-*.md` files;
  - B, the other four passes over `quality-cross.md` and `configuration-cross.md`, `AGENTS.md`'s Gotchas to Design Principles and `specs/system.md`, plus a verdict on each stored decision whose key names an in-scope file;
  - C, the criteria step over every criterion, briefed to read the code each criterion's claim names: one judged 28 of 30 from grep hits and passing tests until sent back (079's review, 2026-11-23).

  Give them the review reference's sections, `write-review`'s shapes from the HEAD binary (`tools/list`, sent after `initialize` and `notifications/initialized` to `env -u CLAUDE_CODE_SESSION_ID -u DUCTUS_SESSION runtime/target/release/ductus mcp`), a table of 20 KB `sed -n` read ranges, and a fresh hand-back path each. Take `start-fix-rounds`' base before any write, send A and B the whole fix diff to re-run their passes, and hand each `brief-re-read` round's brief path alone to a fresh agent, with the worktree's path: the brief names paths from a project root it does not name, and a fresh agent starts in the session's own checkout (079's review, 2026-11-23). Write the record at the worktree's base, then rebase: a commit on `origin/main` after `reviewed-against` that changed a scope path refuses the write. Pass each structured argument as a `--{name}-json` file in `$TMPDIR`, and backtick every path in a note or observation, since `review.md`'s lint gates `done`. An observation matched to a stored decision carries the stored key as `decision-key` and its own wording as `text`: a stored key holding nested code spans, rendered as the text, fails that lint (MD038; 073's review, 2026-11-24). A re-read's prescribed edit is a claim: read it against the tree before applying it, since 048's first round (2026-11-19) prescribed two that would have written a false claim, and fixing that round otherwise sent its next one to a full re-read.
- **Pricing a reopen.** Read `compute-review-scope`'s `criteria-owing` after the reopen, never infer it: a `done` spec whose review record predates 066 task 27's criteria step owes every criterion a review when reopened (067 and 062, 2026-11-19).
- **Analysis.** Run `scripts/analyze/` from the worktree with `ANALYZE_WORK` set to a fresh directory under `$TMPDIR` and `DUCTUS_BIN` to the worktree's binary, in the order **Sizing an analysis's judges** gives. Commit each design-subject fix before the next `detect.py`, then re-run `split.py` and `briefs.py` and send each judge and step 15's reader the diff with its new output path, and each judge its new share path too.
- **Status commit.** Family 8 (`scripts/audit/introducing-drift.sh`) reads only `done` specs, so a retired name in a spec body passes every audit until the commit that sets the spec `done`: run `scripts/audit/run-all.sh` after that commit too, before pushing it. 087's `done` commit failed `framework-checks` on a `.govern/` span its audits had passed (2026-11-26).
- **Push gate.** After committing, `git fetch && { git merge-base --is-ancestor origin/main HEAD || git rebase origin/main; }`, then `scripts/audit/run-all.sh`, then in one chain the fetch, the ancestor check, the outgoing subjects counted with `wc -l` and each matched with `grep -qF`, and `git push origin HEAD:main`. Run it again when another push lands meanwhile. The rebase gives each unpushed commit a new sha, so a record's Summary, a commit message or an artifact written before it cites an unpushed commit of the run by its subject, never its sha: 017's and 018's review Summaries named `892c1b07` and `b2bd79ef`, which landed as `2780cf84` and `ce62b96d` (2026-11-19).
- **Scratch-adopter bootstrap.** After the push, one subagent that calls no `mcp__ductus__*` tool installs the bootstrap with `install.sh claude --ref=main` and follows it under a scratch `HOME` with an empty store, through a zsh function that unsets both session identities and sets `HOME`. A change to a guard's text also gets a run of that text alone on a tampered copy of the staged registry.

## Writing a destination check

A destination check judges where a write lands, not only where its path leads, and compares names as the file system folds them.

- Judge the parent resolved with the final name as given, and the whole path resolved, since `write_atomic_bytes` replaces a link and never follows it.
- Compare names after `host_config::fold`.
- Write the bytes the check judged.
- Find the root among the path's resolved ancestors.
- Judge an exception's own parameter, and list every caller before granting a shared check an exception.
- Prove each refusal from the base where it alone refuses, by its own mutation, with the whole cell table beside the tests, as `runtime/src/primitives/host_config.rs` keeps it.

## Narrowing a permission rule

An exact allow rule bounds the command text, not every way to run it, so probe a narrowed rule against the commands the steps spell and against hostile forms before claiming it bounds anything.

- Claude Code strips a bare `xargs` before matching, so `echo x | xargs cp a b` runs `Bash(cp a b)` with operands appended.
- zsh glob qualifiers (`*(e:'cmd':)`) run code.

Test what each rule admits and refuses (separators, substitution, redirects, `xargs`, zsh qualifiers, `run-generator`, `exec`), and disclose what it still admits.

## Exercising an installer change without a push

Drive `apply-manifest` and `enforce-manifest` from a staging copy in `/ductus`'s own layout: `cp -R framework` into a `mktemp -d` directory, pass that directory as `--source-root`, and build `--entries-json` from its `framework/bootstrap/ductus.md` with sources spelled `framework/…` as the bootstrap's tables spell them. Staging `framework/` itself as the root is the generator's layout, not an adopter's, and passed a check an adopter's run then failed (spec 080). On `main`, a subagent (no nested CLI) runs `install.sh` in scratch, with no `mcp__ductus__*`.

**A bootstrap run executes the installed `ductus.md`, not the one it fetches, so a prose edit is tested by installing it.** Copy the edited file to the fixture's `.claude/commands/ductus.md` and run `/ductus --ref=<latest release tag>`: the release's copy then reads as stale, and the new text drives the run's pre-flight (029 task 15). Seed `.claude/settings.local.json` with the registry's `settings_template`, and unset `CLAUDE_CODE_SESSION_ID` and `DUCTUS_SESSION`. In `claude -p`, Claude Code refuses writes to `.mcp.json` and `.claude/` even under path allow rules, so such a run reports those writes as not made.

**A first adoption can carry `[runtime] path`** in a `.ductus/config.toml` with no `[host]` table, which is no earlier-adoption signal (`framework/bootstrap/ductus.md` §Pre-flight Checks). Without one, run its shell commands with `HOME` pointed at a scratch home whose `~/.ductus/bin/ductus` is a copy of this tree's release build, which reports the pin, so Branch 2 takes it as current and no real store is touched. Start an older release's subject with an empty scratch store, so its own bootstrap acquires its release for real, and set `[runtime] path` before the upgrade. **Drive a real pi in RPC mode** (`pi --mode rpc --approve --no-session`, with `CLAUDE_CODE_SESSION_ID` and `DUCTUS_SESSION` unset): `get_commands`, a no-turn command, then a prompt template. Since 064 task 17, `pi -p` exits after its turn as well. This harness's classifier refuses a nested `claude -p --permission-mode bypassPermissions`.
