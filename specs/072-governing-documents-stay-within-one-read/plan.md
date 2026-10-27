# 072 — Governing documents stay within one read Plan

Implements [072 — Governing documents stay within one read](spec.md).

## Overview

Three changes land in order, and only the last two are enforcement:

1. **The cut.** Every `AGENTS.md` entry is classified once, by the rubric under §Entry dispositions, and the guide is rewritten from that classification to fit the ceiling. Lessons a check already reports leave for the check. Where a check's message does not name the fix, the fix is added to the message or as a comment at the enforcement site.
2. **The ceiling.** `scripts/lint-governing-size.sh` measures each governed file as it sits in the git index and fails when one is over its ceiling, missing or unreadable. `.githooks/pre-commit` and CI run it, and a test proves it fails.
3. **The import.** `CLAUDE.md` imports `AGENTS.md` as `@AGENTS.md` and drops what it copied from the guide. A tool-less `claude -p` run proves the guide loads.

No constitution content or shipped file changes, and the two `runtime/src/` changes are comments only, so nothing here needs a release.

## Technical Decisions

### Entry dispositions

Each entry under `AGENTS.md`'s `##` sections gets exactly one disposition:

- **pointer**: the rule's canonical text is in `framework/constitution.md`. The entry becomes one line: its bold lead, kept verbatim, then the `§anchor` link and nothing else. Grepping the rule's distinctive phrasing then finds one statement and one pointer, which is the shape `050-constitution` AC3 describes.
- **trimmed**: a project-only rule stays. It keeps the rule, plus how this repository applies it where that is not obvious. The incident account goes ("Reason: surfaced …", the session, the task, what went wrong). An owner decision keeps one clause, `(owner, YYYY-MM-DD)`.
- **moved**: a test, hook, audit family or CI check in this repository reports the failure the entry warns about. The entry is removed, and the row names the enforcement site. Where that site's message does not name the fix, it is added in the same change.
- **deleted**: the entry is obsolete (it names a file, flag or behavior the repository no longer has), or it duplicates another entry and is merged into it. The row gives the reason, or the entry it merged into.

Four constraints bind every disposition, because existing checks read this file:

- **Bold leads stay verbatim** on every surviving entry. `/ductus:audit` Family 38 (`scripts/audit/promotion-coverage.sh`) matches each row of `050-constitution`'s plan §Classification to an entry by its lead phrase, and reports an unmatched key as a finding. Trimming applies to the body; a lead that is itself a story is re-keyed in 050's table in the same change.
- **A removed entry's 050 row goes with it.** When a **moved** or **deleted** entry has a row in 050's §Classification table, that row is removed or re-keyed to the entry it merged into, in the same change. 050 is `in-progress` and the table is its record of the guide, so the edit opens nothing.
- **Content a family reads survives.** The `<!-- audit:sweep-targets:begin -->` … `end` list is Family 23's subject; it moves out of the dead-references pointer into its own *live-artifact set* entry, and Family 23's fix messages name that entry and is kept whole. `unbalanced-inline-markup` scans the file, so markup stays balanced.
- **The prose outside entries is held to the same rules.** That is about 9.4 KB of plain bullets, mostly §Gotchas and §Project Structure, and about 2.5 KB of section prose. It counts toward the ceiling, and its stories go too.

The table below is filled while the cut is made. The lead column is abridged to about 40 characters, so the table stays near 25 KB. It has one row per entry, in file order, cited by section and position as the file stood at `5cb01fdb`. Entries added after that commit get rows too, under the spec's *Entries added meanwhile* edge case.

| § | # | Entry (abridged lead) | Disposition | Destination or reason |
| --- | --- | --- | --- | --- |
| Header | 3 | The ductus framework — a pipeline-driven… | trimmed | project-only; unchanged |
| Header | 5 | Agents: this file is the committed home… | trimmed | project-only; routing restated as pointer to §drift-prevention, section rule kept |
| Constitution | 9 | See [constitution.md](framework/constitu… | trimmed | project-only; unchanged |
| Project Structure | 13 | The framework/ directory is ductus's sou… | trimmed | project-only; unchanged |
| Project Structure | 15 | framework/constitution.md — the constitu… | trimmed | project-only; unchanged |
| Project Structure | 16 | framework/rules/ — domain rule sets adop… | trimmed | project-only; unchanged |
| Project Structure | 17 | framework/templates/spec/ — templates co… | trimmed | project-only; unchanged |
| Project Structure | 18 | framework/templates/project/ — project d… | trimmed | project-only; unchanged |
| Project Structure | 19 | framework/commands/ — operational slash… | trimmed | project-only; unchanged |
| Project Structure | 20 | framework/reference/ — each command's ma… | trimmed | project-only; 071 provenance link and reference-reading detail cut |
| Project Structure | 21 | framework/bootstrap/ — the ductus.md ins… | trimmed | project-only; unchanged |
| Project Structure | 23 | When adding files under framework/, plac… | trimmed | project-only; unchanged |
| Tech Stack | 27 | ductus is a text-first framework. The im… | trimmed | project-only; unchanged |
| Tech Stack | 29 | Markdown | trimmed | project-only; durability restatement cut (constitution §tasks-phase states it) |
| Tech Stack | 30 | Bash scripts | trimmed | project-only; unchanged |
| Tech Stack | 31 | GitHub Actions YAML | trimmed | project-only; unchanged |
| Tech Stack | 32 | Node.js / markdownlint-cli2 | trimmed | project-only; pinning rationale and failure list cut, Node-on-PATH rule kept |
| Tech Stack | 33 | Rust | trimmed | project-only; examples and stale-prone run-generator list cut (runtime-tools.txt) |
| Tech Stack | 35 | The runtime is required, and acquired by… | trimmed | project-only; §runtime-boundary restatement and five-targets count cut |
| Tech Stack | 37 | When further tooling-language decisions… | trimmed | project-only; CLI-UX rationale cut |
| Workflow | 41 | Commit directly to main — this repo uses… | trimmed | project-only; owner decision; incident account and §grounding copy cut |
| Workflow | 42 | Review an external pull request for corr… | trimmed | project-only; PR #5 incident account cut |
| Workflow | 43 | Never create a repository named govern u… | trimmed | project-only; reason paragraph cut |
| Workflow | 44 | A change ships via a ductus-v<version> t… | trimmed | project-only; owner decision; version sites left to Family 20, golden notes obsolete |
| Workflow | 45 | A pushed tag is not a published release,… | trimmed | project-only; 0.49.1 incident account cut |
| Workflow | 46 | done is not the same as discharged — the… | trimmed | project-only; gate enumeration and 055 history cut |
| Workflow | 47 | A spec does not reach done with an outst… | pointer | §implement-phase |
| Workflow | 48 | Work a recommendation out to its result… | pointer | §recommendations |
| Workflow | 49 | Never record anything durable in review.… | pointer | §implement-phase |
| Workflow | 50 | Run scripts/audit/run-all.sh after commi… | pointer | §grounding |
| Workflow | 51 | When a scenario is recorded after the fi… | pointer | §implement-phase |
| Workflow | 52 | A review is the five passes, not the wri… | trimmed | project-only; stale no-denominator premise and incident account cut |
| Workflow | 53 | examined counts the in-scope files the p… | pointer | §implement-phase |
| Workflow | 54 | When one commit reopens several specs, r… | trimmed | project-only; incident account and review-record chain cut |
| Workflow | 54+ | (split) fix-round re-read rule is distin… | trimmed | new entry split from #54 |
| Workflow | 55 | Never record another project's name in t… | trimmed | project-only; owner decision; story cut, dead tests/ path fixed |
| Workflow | 56 | Read framework/commands/{name}.md before… | trimmed | project-only; unchanged |
| Workflow | 57 | A .ductus/config.toml change is cross-sp… | pointer | §cross-spec-impact |
| Workflow | 58 | A retired feature's spec is deleted, not… | pointer | §spec-lifecycle |
| Workflow | 59 | No dead references in live artifacts. | pointer | §drift-prevention |
| Workflow | 59+ | (split) Family 23 sweep-target list is r… | trimmed | new entry split from #59 |
| Workflow | 60 | No dead code — remove what nothing calls… | trimmed | project-only; owner decision; incident account cut |
| Workflow | 61 | A test must fail when the behavior it na… | pointer | QUAL-TEST-001 |
| Workflow | 62 | A behavior change needs a prose-claim sw… | pointer | §drift-prevention |
| Workflow | 62+ | (split) sweep-completeness sub-rules not… | trimmed | new entry split from #62 |
| Workflow | 63 | A change to an agent-facing surface must… | trimmed | project-only; backfill incident accounts cut |
| Workflow | 64 | Cross-service reference edits are mechan… | pointer | §spec-lifecycle |
| Workflow | 65 | Criterion-label assignment is mechanical… | pointer | §spec-lifecycle |
| Workflow | 66 | Never hand-write an AC{n}: label — add t… | pointer | §spec-requirements |
| Workflow | 67 | Run /ductus per its spec — no ad-hoc pro… | pointer | installer preamble (framework/bootstrap/ductus.md Procedural fidelity) |
| Workflow | 68 | Use the Write tool, not Bash redirects,… | pointer | §text-first-artifacts |
| Workflow | 69 | Use repo-relative paths in tool calls. | trimmed | project-only; incident account cut |
| Workflow | 70 | Never call specs "frozen archaeology" or… | trimmed | project-only; done-sweep history and file list cut |
| Workflow | 71 | Re-open a done spec via set-status when… | pointer | §spec-lifecycle |
| Workflow | 72 | Add a rule to its surface's home spec vi… | pointer | §rules (second Rules heading, #rules-1) |
| Workflow | 73 | Route shared runtime work to its owning… | trimmed | project-only; owner decision; per-spec primitive lists cut (owner specs name them) |
| Workflow | 74 | A canonical table living on a done spec… | pointer | §spec-lifecycle (release-gate cost reported by review-freshness.sh:497) |
| Workflow | 75 | Backtick a primitive in an ## Instructio… | trimmed | project-only; parity goldens cover only some commands; incident account cut |
| Workflow | 76 | Do not invoke cargo build twice in one s… | trimmed | project-only; incident account cut |
| Workflow | 77 | The local gate is the whole CI surface,… | pointer | §grounding |
| Workflow | 77+ | (split) repo gate surface (push workflow… | trimmed | new entry split from #77 |
| Workflow | 78 | After pushing, read every workflow's run… | pointer | §grounding |
| Workflow | 79 | The runtime stays current — scripts/chec… | trimmed | project-only; owner decision; drift story and script-reported mechanics cut |
| Workflow | 80 | A CI check that reads git history needs… | pointer | §design-principles |
| Workflow | 81 | A new test that reads git history or she… | pointer | §design-principles |
| Workflow | 82 | When an item routes to a chore, fix it —… | pointer | §brownfield-inbox |
| Workflow | 83 | git checkout -- specs/{feature}/ silentl… | pointer | §spec-lifecycle |
| Workflow | 84 | Renaming this repository orphans contrib… | pointer | §drift-prevention |
| Workflow | 86 | Before reporting that a declarative entr… | pointer | §grounding |
| Workflow | 87 | A real adopter run is the only test of c… | trimmed | project-only; six-defect incident account cut |
| Workflow | 88 | A run summary is evidence about what tha… | pointer | §grounding (prefer the source to the recollection) |
| Workflow | 90 | Verify what a commit actually contains b… | pointer | §grounding |
| Workflow | 91 | Read what a push carries before pushing… | pointer | §concurrent-features |
| Workflow | 92 | Stop your own background tasks before wr… | trimmed | project-only; release-poll handoff incident account cut |
| Workflow | 93 | A CI verdict comes from pushing main — n… | trimmed | project-only; owner decision; draft PR #4 incident and quote cut |
| Workflow | 94 | Read the clock and read the sha — never… | pointer | §grounding (substitute, never transcribe, a timestamp or commit) |
| Workflow | 95 | Discard an observation about the pipelin… | trimmed | project-only (050 row); 026 review incident cut |
| Workflow | 96 | The inbox is a queue to drain, not a led… | trimmed | project-only; owner decision; 2026-09-13/14 measurements and quote cut |
| Workflow | 97 | A finding whose fix lands inside a spec… | trimmed | project-only; owner decision; core is §brownfield-inbox; 022 pricing figures cut |
| Workflow | 98 | A finding made during implementation bec… | pointer | §brownfield-inbox (scope decides the destination; disposition task) |
| Workflow | 99 | An inbox item blocked on an external eve… | pointer | §brownfield-inbox (blocked on an external event is discarded) |
| Workflow | 100 | check-artifacts reports clean: true and… | pointer | §spec-requirements (a short skip list is a fact about backticks; read the criteria yourself) |
| Workflow | 100+ | (split) never-true arm not in constituti… | trimmed | new entry split from #100 |
| Workflow | 102 | This repository keeps no per-user agent… | trimmed | project-only; owner decision; quote cut; overlaps planned CLAUDE.md routing (AC8) |
| Workflow | 103 | Land a correction in a git-tracked artif… | trimmed | project-only; 2026-05-17 heredoc incident cut |
| Workflow | 104 | "Confirm every modification" means verif… | trimmed | project-only; owner decision; spec 058 incident cut |
| Workflow | 105 | "Continue" while decisions are open is n… | trimmed | project-only; README rewrite incident cut |
| Workflow | 106 | An interrupt followed by a command is th… | trimmed | project-only; re-target incident cut |
| Workflow | 107 | Trace a surprising state change to your… | trimmed | project-only; release-binary incident and example cut |
| Workflow | 108 | A requested prompt is one copy-pasteable… | trimmed | project-only; owner decision |
| Workflow | 109 | Do not relay claude.ai connector-authori… | trimmed | project-only; owner decision |
| Workflow | 110 | Fan independent work out in parallel; ke… | trimmed | project-only; owner decision; examples, quote and constitution restatements cut |
| Gotchas | 114 | Pi has no MCP client — its tool surface… | trimmed | project-only; harness header cites this entry; PR #5 incident account cut |
| Gotchas | 116 | After changing runtime source, verify th… | trimmed | project-only; spec 054/055 incident accounts cut |
| Gotchas | 117 | framework/commands/help.md is generated… | trimmed | project-only; hook regenerates silently, nothing reports it; incident cut |
| Gotchas | 118 | Registering a runtime primitive is five… | trimmed | project-only; absorbs L133; runtime/src/parser/mod.rs:993 cites AGENTS.md Gotchas |
| Gotchas | 119 | check-artifacts' skipped array names tar… | trimmed | project-only; path corrected to framework/reference/analyze.md; incident cut |
| Gotchas | 120 | Use npx markdownlint-cli2 to run markdow… | trimmed | project-only; unchanged |
| Gotchas | 121 | A prose-only edit to framework/commands/… | moved | runtime/tests/parity.rs:933 |
| Gotchas | 122 | An assessSpecQuality step's prose choose… | trimmed | project-only; nothing pins analyze.md step tiers; incident cut |
| Gotchas | 123 | Two extension points fan out in the exec… | trimmed | project-only; test-authoring rule, no check; incident cut |
| Gotchas | 124 | The command generator substitutes {proje… | trimmed | project-only; gov-rename history cut |
| Gotchas | 125 | An edit to framework/commands/.md is inv… | trimmed | project-only; runtime/MCP half duplicates L116; incident cut |
| Gotchas | 126 | The pre-commit hook regenerates the comm… | moved | .github/workflows/generators.yml:72 |
| Gotchas | 127 | The pre-commit hook lints and tests the… | trimmed | project-only; failure is clippy output, no repo line names it; incident cut |
| Gotchas | 128 | The generator rule has no exceptions — e… | moved | scripts/gen-claude-commands.sh:173 |
| Gotchas | 129 | Never git add -A / git add . in this rep… | pointer | §text-first-artifacts |
| Gotchas | 130 | The two frontmatter derivations are runt… | pointer | §runtime-boundary (frontmatter schema also names both primitives) |
| Gotchas | 131 | write_atomic_bytes (the create-then-rena… | trimmed | project-only; corrected: a rewrite now keeps its prior mode; incident cut |
| Gotchas | 132 | runtime/rust-toolchain.toml pins compone… | trimmed | project-only; nothing reports a workflow missing the input; incident cut |
| Gotchas | 133 | A new ductus runtime primitive wires int… | deleted | duplicate of L118 (primitive registration sites) |
| Gotchas | 134 | A new slash command wires into five plac… | trimmed | project-only; parser step-number test list is unreported; incident cut |
| Gotchas | 135 | <!-- audit:ignore-promotion --> silences… | trimmed | project-only; parser half of Workflow L75; incident cut |
| Gotchas | 137 | A markdown link in a spec body creates a… | pointer | §text-first-artifacts |
| Gotchas | 138 | create-scenario appends its own Open/Res… | pointer | §scenarios |
| Gotchas | 139 | A version bump needs one build without -… | moved | scripts/audit/check-zero.sh:43-44 (names fix); parity-rebuild tail obsolete |
| Gotchas | 140 | Never write a literal ductus-vX.Y.Z into… | trimmed | project-only; 022 inbox-row incident cut; BLESS caution from L139 kept here |
| Gotchas | 142 | Gitignored adopter state survives a git… | trimmed | project-only; 048 AC10 incident account cut |
| Gotchas | 143 | This repo's dogfooded copy of an adopter… | trimmed | project-only; 048/013 adopter-run incident cut |
| Gotchas | 144 | A spec's review diff base moves forward… | pointer | §implement-phase |
| Gotchas | 145 | An apostrophe anywhere in an /audit fami… | moved | .github/workflows/framework-checks.yml:196 (shellcheck SC1011); comment proposed |
| Gotchas | 146 | Replacing a hand-rolled predicate with a… | pointer | QUAL-DELEG-001 (framework/rules/quality-cross.md carries the trust-context point) |
| Gotchas | 147 | remove-inbox-item takes the bullet text,… | pointer | §brownfield-inbox |
| Gotchas | 148 | A scenario's link to a sibling spec need… | pointer | §text-first-artifacts (hook check-corpus-links also reports it, names fix) |
| Gotchas | 149 | path is a reserved array in zsh and clob… | trimmed | project-only; path+status kept, incidents cut; headSha split to L149+ |
| Gotchas | 149+ | (split) headSha poll filter is a distinc… | trimmed | new entry split from #149 |
| Gotchas | 150 | An unquoted parameter does not word-spli… | trimmed | project-only; fixed check-rule-ids bug story and routing lesson cut |
| Gotchas | 151 | A word starting with = is a command look… | trimmed | project-only; handoff-note history cut |
| Gotchas | 152 | A colon after a parameter is a zsh modif… | trimmed | project-only; 025 recovery incident cut |
| Gotchas | 153 | check-orphaned-references used to report… | deleted | obsolete: fixed in ductus-v0.49.4 (check_artifacts.rs:1560); findings: 0 |
| Gotchas | 154 | compute-review-scope and diff-cross-spec… | pointer | §grounding (review.md step 1 states the failed-call branch) |
| Gotchas | 155 | In a set -e script, [ test ] && cmd as t… | trimmed | project-only; hook and gen-configure-mcp incident history cut |
| Gotchas | 156 | A feature directory is not necessarily N… | trimmed | project-only (Rust half of §numbering-convention); 051 incident cut |
| Gotchas | 157 | clap expands {n} in a doc comment as a n… | trimmed | project-only; create-feature incident cut |
| Gotchas | 158 | Verify that a recorded decision still ho… | trimmed | project-only; 003/000/017 accounts cut (universal: promotion candidate) |
| Gotchas | 159 | "Record" spans five destinations in this… | pointer | §pipeline-boundaries; the count is dropped from the lead, since the section states none, and 050's row is re-keyed |
| Gotchas | 160 | Editing framework/commands/implement.md… | moved | runtime/tests/parity.rs:933 (CI runtime.yml); message fix proposed; dup of L121 |
| Gotchas | 161 | Mutating code to prove a test fails: run… | trimmed | project-only; dated timings and harness history cut |
| Gotchas | 162 | The .govern.toml residue, and what it me… | pointer | §drift-prevention; lead was a story, replaced by the constitution's own lead |
| Gotchas | 163 | Piping a gate command's output replaces… | pointer | §grounding |
| Gotchas | 165 | A § reference resolves by line, and intr… | pointer | §drift-prevention |
| Gotchas | 166 | A path:line citation in a spec body is p… | trimmed | project-only; measurements and 051 account cut (universal: promotion candidate) |
| Gotchas | 167 | A review record written before ductus-v0… | pointer | §implement-phase; the lead's version claim was false (`reviewed-digest` shipped in 0.46.0), so the lead is replaced by the constitution's own and 050's row is re-keyed |
| Gotchas | 168 | An acceptance criterion's parenthetical… | pointer | §grounding |
| Gotchas | 169 | A spec with no recorded in-progress tran… | pointer | §implement-phase |
| Gotchas | 170 | A cross-spec signpost is not rename resi… | trimmed | project-only; 006/011 incident cut (universal: promotion candidate) |
| Gotchas | 171 | A sweep must not rewrite the record of t… | trimmed | project-only; dated measurements and sweep incidents cut |
| Gotchas | 172 | A sweep can make a quoted instruction po… | pointer | §drift-prevention |
| Gotchas | 173 | A spec body must not carry a point-in-ti… | pointer | §spec-requirements |
| Gotchas | 174 | Only a spec's durable contracts stale it… | pointer | §implement-phase |
| Gotchas | 175 | criterion-path-existence only sees a pat… | pointer | §spec-requirements |
| Gotchas | 176 | unexamined cannot be measured while the… | pointer | §implement-phase |
| Gotchas | 177 | A spec's own completion commit can destr… | pointer | §text-first-artifacts |
| Gotchas | 179 | process-decisions with an empty fired ex… | trimmed | project-only; incident account cut (universal: promotion candidate) |
| Gotchas | 181 | append-task has two shapes that both ret… | pointer | §scenarios |
| Gotchas | 181+ | (split) CLI --body/--parent-heading spel… | trimmed | new entry split from #181 |
| Gotchas | 182 | A primitive whose result has two shapes… | trimmed | project-only; incident cut; no check for flattened halves sharing a key |
| Gotchas | 183 | set-status takes --from and --to, not --… | pointer | §spec-lifecycle |
| Gotchas | 184 | A boolean argument is a valueless flag o… | trimmed | project-only; incident cut; clap refusal is runtime/src and lacks the fix |
| Gotchas | 185 | A recorded measurement can encode a cons… | pointer | §grounding |
| Gotchas | 186 | Git's -E is POSIX ERE and does not honou… | trimmed | project-only; incident account cut |
| Gotchas | 187 | cargo exits 101 from the repo root, and… | trimmed | project-only; incident account cut |
| Gotchas | 189 | /{project}:consolidate migrates no conte… | trimmed | project-only; incident cut; keeps the hand-correct rule L58's pointer dropped |
| Gotchas | 190 | grep here is ugrep, and a bounded-contex… | trimmed | project-only; incident account cut |
| Gotchas | 191 | write-review derives scope itself, so a… | pointer | §implement-phase |
| Gotchas | 191+ | (split) scope counts entries, not files… | trimmed | new entry split from #191 |
| Gotchas | 193 | Adding a member to a closed set the chec… | trimmed | project-only; incident cut; Family 18 binds marker lists only |
| Gotchas | 195 | Relocating a frontmatter field: the read… | pointer | §grounding |
| Gotchas | 196 | The project root is not the git work tre… | trimmed | project-only; 059 account cut; hook case is Family 22; no Repository lint |
| Gotchas | 196+ | (split) command-source git output parsin… | trimmed | new entry split from #196 |
| Gotchas | 198 | Since spec 062 the runtime reads a sessi… | trimmed | project-only; incident cut; no check enforces ductus_command |
| Gotchas | 200 | A rule that assumes a deployed service d… | trimmed | project-only; 022 analysis account cut |
| Gotchas | 202 | A new rule category must be registered i… | trimmed | project-only; incident cut; path now framework/reference/analyze.md |
| Gotchas | 204 | eprintln! panics when stderr is a pipe w… | trimmed | project-only; incident cut; stale list of remaining eprintln sites cut |
| Gotchas | 205 | An interrupted turn silently kills the b… | trimmed | project-only; incident account cut |
| Gotchas | 206 | A background agent that launches its own… | trimmed | project-only; incident cut; unlocatable ask-the-owner rule reference cut |
| Gotchas | 207 | A background agent's hand-back can arriv… | trimmed | project-only; incident account cut |
| Gotchas | 208 | Invoke the runtime as runtime/target/rel… | trimmed | project-only; rebuilt-binary half left to L116 |
| Gotchas | 209 | Identify a request to an outside service… | trimmed | project-only; 2026-10-20 and 2026-10-26 incident accounts cut |
| Gotchas | 210 | A destination check judges where a write… | trimmed | project-only; nine re-read round accounts cut; over budget, several sub-rules |
| Gotchas | 211 | An exact allow rule bounds the command t… | trimmed | project-only; 022 task 18 account cut; docs quote dropped |
| Gotchas | 212 | Two traps in this repo's audit scripts a… | trimmed | project-only; incident account and constitution-restating clause cut |
| Gotchas | 213 | With anything under runtime/ staged, the… | trimmed | project-only; 2026-10-25 incident account cut |
| Gotchas | 214 | An installer change can be exercised wit… | trimmed | project-only; measurements and dates cut; two rules in one entry, over budget |
| Boundaries | 218 | No host-level enforcement of the determi… | trimmed | project-only (050 B1); reason paragraph cut |
| Boundaries | 219 | Never edit an installed command file dir… | pointer | §drift-prevention |
| Boundaries | 220 | Never end a response with a "worth flagg… | pointer | §design-principles |
| Design Principles | 224 | Reach for a runtime primitive first; a s… | pointer | §runtime-boundary |
| Design Principles | 226 | A check that cannot run must never look… | pointer | §design-principles |
| Design Principles | 228 | Never close work with a caveat attached… | pointer | §design-principles |
| Design Principles | 229 | Never design framework features that dep… | pointer | §design-principles |
| Design Principles | 231 | Removing an embedded copy is a corpus sw… | pointer | §drift-prevention |
| Design Principles | 233 | A data-model.md that claims "authoritati… | pointer | §drift-prevention |
| Design Principles | 235 | Point at the canonical source — never em… | pointer | §drift-prevention |
| Design Principles | 237 | A conformance test whose subject is draw… | pointer | §design-principles |

### Size target

The ceiling is 50,000 bytes, and the guide measures 46,731 bytes at `1ebc2a20`, leaving room for the next few lessons. A ceiling met with zero headroom would force a retirement on the very next commit, before the exit had been exercised once. The landing is above the 45,000 first aimed at because the two security-derived entries (*a destination check judges where a write lands* and *an exact allow rule bounds the command text*) keep their full text, about 650 bytes over the cap. Each enumerates refusals a later check must reproduce, and the cap would have dropped four of them.

### Fan-out for classification

The 164 entries are independent of one another, so classification fans out to parallel agents by section chunk. Each agent gets the rubric above, the four constraints, its chunk's text, and the byte arithmetic, not an adjective. Pointers keep their long leads, so 71 of them cost about 12 KB. The trimmed set therefore gets a body cap of 200 bytes after its lead, enforced in two passes. The first classified and trimmed. It landed at 57 KB, because a lead averages about 150 bytes on top of a 350-byte body. The second compressed only the bodies still over the cap, again fanned out, with every lead and the sweep-target list held byte-identical by the applying script. Each agent returns a proposed disposition, the replacement text and, for **moved**, the enforcement site and whether its message names the fix. The assembly, the table, every write to `AGENTS.md` and the commit are serial (`AGENTS.md` §Workflow, the fan-out entry). Every **moved** and **deleted** proposal is verified against the tree before it is accepted. An agent's claim that a check covers a failure is a claim, and it is read before it is trusted.

### Enforcement-site edits stop short of `runtime/src/`

A message or comment added for a **moved** entry lands in `scripts/`, `scripts/tests/`, `.githooks/`, `.github/workflows/` or `runtime/tests/`. When the only enforcement site is in `runtime/src/` and its message lacks the fix, the entry stays **trimmed** instead. Two `runtime/src/` comments that cited `AGENTS.md` text the cut removes (`parser/mod.rs`, `primitives/mechanical_sweep.rs`) are corrected in place, comment-only, because a prose-claim sweep covers doc comments (`AGENTS.md`, *a prose-claim sweep reaches past the changed tokens*). A change under `runtime/src/` reaches adopters only at a `ductus-v*` release (`AGENTS.md`, *a change ships via a `ductus-v<version>` tag*) and runs the cargo gate on every commit (`.githooks/pre-commit`, the `runtime/` block), and neither cost buys anything for a documentation move.

### The ceiling check is a script, not a runtime primitive

`scripts/lint-governing-size.sh` follows `scripts/lint-rule-filenames.sh`: a maintainer lint over this repository's own files, run by CI and never shipped. It parses no markdown, frontmatter or configuration. It reads two blob sizes with `git cat-file -s`, so §runtime-boundary principle 3's prohibition, which covers shell pipelines that parse frontmatter or markdown structure, does not reach it. It also fails eligibility criterion 2 (currently mechanical), because no LLM step or script performed this check: `git grep -n 'AGENTS.md\|constitution.md' 5cb01fdb -- scripts .githooks .github`, filtered for `wc -c`, `cat-file -s`, `size` and `bytes`, returns nothing. The nearest primitive, `check-artifact-size`, measures one feature's spec artifacts. Giving it a governing-document subject means new argument and result types, CLI, MCP, registry and exec wiring (063's plan lists about twenty files for the primitive), plus a release, all to replace two `git cat-file` calls.

- **Ceilings** live in one table at the top of the script: `AGENTS.md 50000` and `framework/constitution.md 50000`. A comment above the table names `DEFAULT_READ_SIZE_BYTES` in `runtime/src/primitives/check_artifact_size.rs` as the source of both.
- **What is measured.** `git -C "$ROOT" cat-file -s ":$path"` reads the index. In the hook that is the staged file, so an unstaged edit neither fails nor passes a commit. In CI, the index is the checkout. One code path serves both, as AC9 requires.
- **Four failures, four messages**, each line naming the file:
  - Over the ceiling: `{file}: {size} bytes, over its {ceiling}-byte ceiling — retire, merge, or move an entry to the check that enforces it (spec 072)`. The constitution's line names its own fix: cut the file, not the ceiling, by moving a rule only one command applies to its `framework/reference/` file or tightening one (spec 050).
  - Missing from the index: `{file}: not in the index, so its size cannot be measured`.
  - Unreadable (`cat-file` fails on a path the index holds): `{file}: could not be read from the index`.

  A ceiling that is not a whole number of bytes is a fourth failure, with its own message. Exit 1 on any failure and 0 only when every file was measured and is at or under its ceiling (AC11). A file exactly at its ceiling passes.
- **Raising a ceiling** means editing the script. The comment above the table says that is an owner decision, recorded in this spec through its back-edge.

### Wiring

- **`.githooks/pre-commit`** runs `bash scripts/lint-governing-size.sh` unconditionally, after `check-corpus-links`, and its header comment gains a paragraph for it. Four git calls cost nothing, so a path filter would buy nothing.
- **`.github/workflows/framework-checks.yml`** gains step `(o) Governing-document size ceilings`. It runs the lint and then `bash scripts/tests/test-lint-governing-size.sh`, following step `(k)`'s pairing of a guard with the test that proves it fails.
- **The wiring lands after the cut.** Wiring first would fail every commit until the guide fit.

### The test proves both directions

`scripts/tests/test-lint-governing-size.sh` follows `test-lint-release-ordering.sh`. Each case copies the lint into a throwaway tree, runs `git init`, stages sized files and runs the lint. The cases:

- (a) the committed `AGENTS.md` and constitution pass;
- (b) `AGENTS.md` one byte over fails, and the message names the file, the size, the ceiling and the fix;
- (c) exactly at the ceiling passes;
- (d) the constitution over its ceiling fails, naming its fix;
- (e) `AGENTS.md` absent from the index fails with the missing message, not a pass;
- (f) back under the ceiling after (b) passes;
- (g) a file the index holds but whose blob is gone fails as unreadable;
- (h) both ceilings equal `DEFAULT_READ_SIZE_BYTES` in `runtime/src/primitives/check_artifact_size.rs`, the constant they copy;
- (i) a ceiling that is not a whole number of bytes fails with its own message. Without that check, `[ -gt ]` errors to stderr, an `if` reads the error as false, and the oversized file passes.

### `CLAUDE.md`

The rewritten file has three parts:

- `# CLAUDE.md`.
- Two import lines: `@framework/constitution.md`, which `050-constitution`'s `the-constitution-fits-one-read` added once the constitution fit one read, then `@AGENTS.md`.
- The **Auto-Memory Routing** section `050-constitution` AC20 requires, reduced to a pointer at §drift-prevention's two questions plus the one fact this repository adds: it keeps no auto-memory at all.

The **Non-negotiables** section goes. Its trunk-based rule is an `AGENTS.md` entry and its truncation rule is §grounding, both now loaded through the import. The `@import framework/constitution.md` line goes rather than being converted, because at 119,262 bytes the constitution is more than one read (AC6). The pipeline commands keep reading it as they do today. Once `050-constitution`'s cut brings it to one read, that spec's `the-constitution-fits-one-read` scenario restores the import, so a session started from a handoff prompt has the constitution too; until then the `AGENTS.md` prompt entry asks a handoff to start with `/ductus:target`. No sentence states another file's size (AC8).

### Proving the import loads

AC7 is verified with `claude -p` at the repository root and the file tools disallowed (`--disallowedTools Read,Grep,Glob,Bash`), asking a question only `AGENTS.md` answers, with `--output-format stream-json --verbose` so the run's tool use is visible. Because the run has no file tools, a correct answer can only come from the loaded context. The same run against the pre-change `CLAUDE.md` is the negative control: it shows the probe fails when the guide is not loaded. Two things can make the control pass for the wrong reason, and both are checked before it counts:

- Claude Code's **Project instructions** setting, when it is `claude-md-and-agents-md`, loads `AGENTS.md` beside `CLAUDE.md` without any import ([memory documentation](https://code.claude.com/docs/en/memory), *Choose which instruction files load*).
- A nested `claude -p` that refuses to start says something about the environment, not about the import.

The probe question is picked after the cut, from an entry that survives it.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `AGENTS.md` | Modify | The cut: every entry reclassified and rewritten to fit the ceiling |
| `CLAUDE.md` | Modify | `@AGENTS.md` import; Non-negotiables and the constitution import removed; routing section reduced to a pointer |
| `scripts/lint-governing-size.sh` | Create | The ceiling check |
| `scripts/tests/test-lint-governing-size.sh` | Create | Proves the check fails and passes |
| `.githooks/pre-commit` | Modify | Runs the check on every commit |
| `.github/workflows/framework-checks.yml` | Modify | Step `(o)`: the check and its test; step `(j)`'s comment cites the constitution instead of removed `AGENTS.md` text |
| `runtime/tests/parity.rs` | Modify | Enforcement-site message or comment for a **moved** entry whose report did not name the fix |
| `.github/workflows/generators.yml` | Modify | Enforcement-site message or comment for a **moved** entry whose report did not name the fix |
| `scripts/gen-claude-commands.sh` | Modify | Enforcement-site message or comment for a **moved** entry whose report did not name the fix |
| `scripts/audit/lib.sh` | Modify | Enforcement-site message or comment for a **moved** entry whose report did not name the fix; also a comment that cited `AGENTS.md` text the cut removes now cites the constitution or the surviving entry |
| `scripts/audit/sweep-target-manifest-parity.sh` | Modify | Fix messages name the *live-artifact set* entry the sweep-target list moved to |
| `scripts/lint-release-ordering.sh` | Modify | A comment that cited `AGENTS.md` text the cut removes now cites the constitution or the surviving entry |
| `scripts/tests/test-lint-release-ordering.sh` | Modify | A comment that cited `AGENTS.md` text the cut removes now cites the constitution or the surviving entry |
| `.github/workflows/runtime-release.yml` | Modify | A comment that cited `AGENTS.md` text the cut removes now cites the constitution or the surviving entry |
| `.github/workflows/runtime.yml` | Modify | A comment that cited `AGENTS.md` text the cut removes now cites the constitution or the surviving entry |
| `scripts/audit/introducing-drift.sh` | Modify | A comment that cited `AGENTS.md` text the cut removes now cites the constitution or the surviving entry |
| `scripts/audit/sibling-coupling.sh` | Modify | A comment that cited `AGENTS.md` text the cut removes now cites the constitution or the surviving entry |
| `runtime/src/parser/mod.rs` | Modify | A comment that cited `AGENTS.md` text the cut removes now cites the constitution or the surviving entry |
| `runtime/src/primitives/mechanical_sweep.rs` | Modify | A comment that cited `AGENTS.md` text the cut removes now cites the constitution or the surviving entry |
| `specs/072-governing-documents-stay-within-one-read/plan.md` | Modify | The entry-disposition table (AC5) |
| `specs/050-constitution/plan.md` | Modify | §Classification rows for removed entries removed or re-keyed, so Family 38 still matches |

## Trade-offs

- **A runtime primitive for the ceiling.** Rejected for this spec: see *The ceiling check is a script*. If adopters later want a ceiling on their own guide, extending `check-artifact-size` is the route, and that is a spec of its own.
- **Deleting the 56 mirrors outright.** Rejected: `050-constitution` resolved that a promoted rule keeps a one-line pointer, so the index of how to work here does not silently lose a rule. A one-line pointer costs about 100 bytes.
- **Splitting `AGENTS.md` into topic files imported on demand.** Rejected: an import loads its file whole ([memory documentation](https://code.claude.com/docs/en/memory): imports organise a file but do not reduce its context cost), so a split saves nothing unless the agent chooses what to read, which is a diligence dependency. Claude Code's path-scoped rules load by file path, and most entries here are about workflow rather than a path.
- **Warning instead of failing at the ceiling.** Rejected: a warning is an exit that depends on someone choosing to take it.
- **Known limitation.** The ceiling bounds bytes, not quality. A guide can be within its ceiling and still carry a story, and only review catches that. The ceiling makes the cost of a story visible, because every byte competes with a rule.
- **Known limitation.** Hosts that read `AGENTS.md` natively (Antigravity and OpenCode, per `framework/bootstrap/ductus.md`) get the smaller guide too. That is the intent, and nothing here tests it on those hosts.

## Cross-spec impact

- **`050-constitution`** owed the constitution's cut to one read (`scenarios/the-constitution-fits-one-read.md`, task 35). That scenario's link back to this spec discharges the `cross-spec-impact` declaration. The cut brought the constitution to 49,980 bytes and lowered this spec's constitution ceiling in `scripts/lint-governing-size.sh` to the 50,000-byte read size.
- **`050-constitution` plan §Classification** loses the rows of entries this spec removes, or re-keys them to the entry they merged into, so Family 38 keeps matching. That is a sync of 050's own record while it is `in-progress`.
- **`050-constitution` AC3** (each promoted entry's `AGENTS.md` line points without restating) is false against the tree at `5cb01fdb`, and the **pointer** disposition makes it true again. No edit to 050 is owed for that, because the criterion already states the shape this spec restores.
