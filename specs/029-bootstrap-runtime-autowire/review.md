---
spec: 029-bootstrap-runtime-autowire
last-run: 2026-11-19T00:18:57Z
reviewed-against: efd3d488116e0752e8e7fb94f5c712c42f6263c5
diff-base: 0adf54ba29b1abcda37b902c3b02012b11c9e5df
must-violations: 0
should-violations: 0
low-confidence: 1
examined: 4
scope: 12
unread:
  - framework/bootstrap/govern.md
  - .github/workflows/runtime-acquisition.yml
  - .github/workflows/runtime-release.yml
  - specs/048-govern-acquired-runtime/data-model.md
  - specs/048-govern-acquired-runtime/plan.md
  - specs/048-govern-acquired-runtime/spec.md
  - specs/029-bootstrap-runtime-autowire/plan.md
  - specs/029-bootstrap-runtime-autowire/analysis.md
excluded: 75
contract-breaks: 0
skipped-passes: []
fix-rounds: 3
re-reads: 3
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/archive-fetch-direct-codeload.md: 0c333420c198f9fd69c3deb351da92c157df213d27e4c0f9dbbe09e70d2456cd
  scenarios/project-inputs-asked-once.md: 8c36e81564e833083b10b6bdee4440d27fd556deabb7b35df9a5d3bc97c1f411
  scenarios/runtime-probe-parity-audit.md: 584a4570a0b5ea5b345fa1b9f9612759f2b6d847087210470a41029782c60d12
  scenarios/state-a-deterministic-path-forcing.md: 6826e091652444615f197ea687ff2e25abad353a806148408c143c4641a561a8
criteria-judged: 21
criteria-met: 21
criteria-verdicts:
  - label: AC1
    digest: 9885ca63d32e6de7e5072e3a624021e4ed19cde0c9d83f9137775797662f0523
  - label: AC2
    digest: fd184feb479812f1f5a6446831b6a50bab077aa519f570011fe34bf41b099b40
  - label: AC3
    digest: 10a3a2b8f00715ce3eadb4e23cdef64c5b0d3c1c27073c7b39f7feb7e79af696
  - label: AC4
    digest: f5f2e566a510ad7c67c51196c3e7f088d4b4b66a01b7e0b72d62d821691fc0cc
  - label: AC5
    digest: 7455e41f0a4b8a46d423b0d8e7d3412ce4ea5c68a25befe62c174d859d658d16
  - label: AC6
    digest: 77e5147497187002806f78cbc2e7211bd33af3fd30f6b5db8b5a4cd5bbed9918
  - label: AC7
    digest: 5423f2c1b1aa8bf6e6e5a2856d924d1ddfe24278595752d4ade0e25421d444b8
  - label: AC8
    digest: b0f9165179620d91c4aeb696fb7b4a510fb4cd08132c139b1e3afb118d7c22ff
  - label: AC9
    digest: 23e1f4c650a9427e3aef1f47b6c5ab78654cfc1139be48f06d8ca4f9e86d901f
  - label: AC10
    digest: 0e657c852ec1cf2741f8317b4fe63268c6f3e5db02ef36f3a98dbceb6e79eabd
  - label: AC11
    digest: c672cc2142ef7b8c942a05de7d8c23b438de864639b704403486c12f7759981c
  - label: AC12
    digest: 7d78cf88f12a6ebca89fee72bb8803e0c01120152e1163c7f57b1f0c60a23a61
  - label: AC13
    digest: 66116991725835a7630a39ce6a9b1d931c2cdf62b8c054d0828d3d22dc80934f
  - label: AC14
    digest: 5e2b4bc2a8d10ae42a2d444b31a4965906bc2028bdac177966fb82ed437913f8
  - label: AC15
    digest: 4c22b6470385c0381e4b7bad9804f9520590ae25182edaabde1570f547a74aad
  - label: AC16
    digest: 184411821aec59a2b17b3ff4bf22453d2059c835c4867d6d4587396d9534881d
  - label: AC17
    digest: 1ff9330af2fe499bbce8a1acae62ba17c732eb46c80dcc041fa240ab727ed73b
  - label: AC18
    digest: 2fe9bed3a0ea2455a2bc8611985cef4a8858bc589d955d62b363569cb301f433
  - label: AC19
    digest: 59d1f0d61061c6e9a0378001758390a49614c9f853c143f4afe8fd94805d0de9
  - label: AC20
    digest: fe965dc215644cccd2059db52a5600c40ec604c468d0d161ca16f8b601b129d8
  - label: AC21
    digest: 0cb672af06210e2312dc2516e1ee31d3039ac9cf62a48fb9c2200351bb5f8669
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 13
  undispositioned: 0
decisions:
  - key: "security: `audit.md` step 1 and `analyze.md`'s drift check send the agent to `.ductus/bin/ductus run-generator`, which the seeded `Bash(.ductus/bin/ductus *)` allow runs with no prompt until 022 task 18 narrows it — `framework/commands/audit.md:31`"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: the constitution's principle 5 and §runtime-host-integration mapping still say every capability is an MCP tool, and deviations require their own amendment — `framework/constitution.md:623`"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "reuse: Family 2's generated-block extraction repeats the marker literals that `scripts/tests/test-gen-configure-mcp.sh` and the generator use — `scripts/audit/manifest-parity.sh`"
    outcome: waived
    reason: scripts/tests has no lib.sh to share a helper through, and check-zero's gen-configure-mcp.sh --dry-run already fails when the markers drift
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: the analyze walker's seed derivation leaves paths unbound for a refused feature rather than raising InvalidPath — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: it opens nothing, and the walk's first step, read-spec on the same feature, raises the InvalidPath before any step reads paths
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "other: the interpreter test table's length assertion cannot catch a call site missing from the table — `runtime/src/interpreter/mod.rs`"
    outcome: waived
    reason: it pins the table against an accidental deletion; completeness against the call sites is the count the scenario records, and keying the table to the registry would need each primitive's arguments, which the registry does not hold
    decided-at: 2026-10-21T22:55:56Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "convention: framework/bootstrap/ductus.md §Project Configuration says the description renders wherever ductus names a source (fix rounds 1 and 3's NOTEs) — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: A comment the installer writes into adopters' config; correcting it is a bootstrap change, which this repository verifies only with a real adopter run, out of proportion to one clause; the data model states which surfaces carry the description.
    decided-at: 2026-11-10T15:30:33Z
    decided-by: rule
  - key: "other: the markdown-only reference-cleanup prose differs from enforce-manifest at the edges (links never removed, dot-prefixed .md matched) — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "contrived: nobody writes either into a ductus-owned directory (owner decision, 2026-10-26)"
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "other: step 4's per-agent apply-manifest loop sits in one walker step whose context carries one map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the real bootstrap is walked by the host, as step 8's per-directory loop already is
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "other: main's bootstrap calls `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, so a `--ref=main` run errors at that step and falls back to the unflagged call until a release moves the pin — `framework/bootstrap/ductus.md:924`"
    outcome: waived
    reason: "An observation about the pipeline's own machinery made while verifying framework work (AGENTS.md): it concerns 055's step, not 077's, blocks nothing here, and ends when the next release tag moves the runtime pin to one carrying the flag."
    decided-at: 2026-11-10T15:30:33Z
    decided-by: andy@stone.dev
  - key: "reuse: Family 15 adds another hand-rolled scan of the §Agent Registry table beside those of installer-registry-parity.sh, host-namespace-parity.sh, placeholder-roundtrip.sh and permission-entry-shape.sh, while lib.sh's agent_config_dirs reads one column of it — `scripts/audit/runtime-probe-parity.sh`"
    outcome: waived
    reason: Each family reads a different part of the table and lib.sh's helper returns one column, so a shared row reader is a refactor of four other families' code, not a mechanical fix.
    decided-at: 2026-11-10T15:30:33Z
    decided-by: rule
  - key: "convention: the Archive fetch and extract abort message advises checking network connectivity even for an archive extract-archive refused — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the message ends 'or report this if it persists', which covers a refused archive, and 015's spec quotes its wording, so rewording it changes no outcome and widens the change to another spec
    decided-at: 2026-11-19T00:18:57Z
    decided-by: rule
  - key: "other: spec.md's Resolved Questions entry Version compatibility still says a too-old wired ductus degrades to the markdown fallback — `specs/029-bootstrap-runtime-autowire/spec.md`"
    outcome: waived
    reason: a resolved decision recorded as it was taken, already marked superseded in part by 048; AC20 and the MCP Wiring section state the current rule
    decided-at: 2026-11-19T00:18:57Z
    decided-by: rule
  - key: "security: the Frontmatter Migration rewrites each globbed spec.md and scenario file in place with no symbolic-link screen, and walks the literal specs/ rather than the configured spec root — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: ductus-procedure.md is outside this window, and 017's and 018's open reviews hold it in scope; the migration serves pre-017 projects, which predate [paths] specs-root (040), so the literal specs/ is their root
    decided-at: 2026-11-19T00:18:57Z
    decided-by: rule
  - key: "security: Pre-run Migrations step 7.1 reads framework/migrations/{id}.md with an id taken from the archive's registry and no containment check — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: a read, not a write, of procedure text the framework's own registry selects and the agent already executes; ductus-procedure.md is outside this window, and 017's and 018's open reviews hold it in scope
    decided-at: 2026-11-19T00:18:57Z
    decided-by: rule
  - key: "security: step 6 and Failure halt naming the listing, so a hostile archive's whole entry list reaches the host's context and the halt reason — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the archive has already matched its published digest and its download is time-bounded, so the listing is the release's own; both workflows' errors name the listing the same way
    decided-at: 2026-11-19T00:18:57Z
    decided-by: rule
  - key: "other: the listing check is copied in runtime-acquisition.yml, runtime-release.yml's acquire job and Branch 2 step 6, and nothing checks the copies against each other — `.github/workflows/runtime-release.yml`"
    outcome: waived
    reason: this review drove the reject branches on nine crafted archives for step 6's check and for each workflow's snippet as written, runtime-acquisition.yml's header binds it to the procedure, and a drift check across the copies is new work, not a defect in 029's contract
    decided-at: 2026-11-19T00:18:57Z
    decided-by: rule
  - key: "other: on Windows a tar that prints CRLF line endings would leave a carriage return on the listing's last field, and the workflows' gate would reject ductus.exe — `.github/workflows/runtime-acquisition.yml`"
    outcome: waived
    reason: unverified here; the dispatched runtime-acquisition run and every release's acquire job run the snippet on windows-latest under Git Bash, and a false rejection fails loudly rather than passing
    decided-at: 2026-11-19T00:18:57Z
    decided-by: rule
---

# Review — 029-bootstrap-runtime-autowire

## Summary

Five passes and the criteria step over 029's window since its reopen (diff-base 0adf54ba, base-source reopen) to efd3d488. The window holds task 17's commits (0fbd9952, 0d054008, 6649ee18), the reopen daf13f9a, and task 17's ductus.md hunk that 66203361 carried from the shared index. Three reviewer parts:

- the security pass, over the three security-*.md files (194,749 bytes);
- the reuse, quality, efficiency and simplicity passes, over the other nine rule files (213,777 bytes), AGENTS.md's Gotchas, Boundaries and Design Principles, and specs/system.md;
- the criteria step, over the spec, the plan, the four scenarios, and the code each criterion names.

Each part read its rule files whole.

Coverage:

- examined counts 4 of the 12 scope entries, each read whole: framework/bootstrap/ductus.md (163,440 bytes, read by all three parts), the scenario state-a-deterministic-path-forcing, spec.md and tasks.md.
- unread names framework/bootstrap/govern.md, a byte-identical copy of ductus.md (same sha256, which the self-audit's Family 21 requires). This run's fixes re-copied it.
- unread also names the five entries this run's fixes added to the scope, which no pass read whole: .github/workflows/runtime-acquisition.yml, .github/workflows/runtime-release.yml, and 048's data-model.md, plan.md and spec.md. The fix rounds re-read their changes.
- unread also names 029's plan.md and analysis.md, which this run's analysis wrote after the passes read: a grounding fix and the analysis record. The passes' re-run read plan.md's change in the fix diff.
- For its criteria, the criteria step read README.md, configure/claude.md, the Pi bridge and three primitives' sources in part, and named each part.

Criteria: 21 of 21 judged met. AC21 was driven: scripts/audit/runtime-probe-parity.sh exits 0 on the tree and 1 on each of three mutations in a copy.

Fixed in the run:

- MUST BE-INPUT-004, which the owner decided to fix in 029. Runtime acquisition Branch 2 step 6 extracted the downloaded release archive with a bare tar -xzf, which no primitive can screen. The step now lists the archive and halts unless it holds exactly one regular file named ductus (ductus.exe on Windows), then extracts that member alone.
  - runtime-acquisition.yml runs the same check, and so does runtime-release.yml's acquire job, by a second owner decision.
  - 048's plan and data-model state the check. Four claims, in runtime-release.yml and 048's data-model and spec, said extraction prompts once; they now say tar runs twice.
  - A scratch-adopter bootstrap ran the new step 6 against the published ductus-v0.59.11 asset: the listing check passed on its one regular file ductus, only that member was extracted, and the re-probe reported 0.59.11. Its self-update check then aborted, since the installed copy differs from the tag's, so that run tested pre-flight only.
  - Proven against the real ductus-v0.59.11 aarch64-apple-darwin asset, which passes, and nine crafted archives, which each halt: traversal (two), absolute, symlink, hardlink, two members, a ./ prefix, a wrong name and a subdirectory. Both the procedure's check and each workflow's snippet, as written in its file, were run.
- After the fixes, the security pass and the other four passes ran again over the whole fix diff (38,847 bytes) and found no rule finding. Both reported that step 6 required the listing to end with the name ductus, looser than the workflows' exact last-field test, and that no text said a failed extraction halts. Step 6 now requires the entry's name to be exactly ductus with no directory part, and Failure names a failed extraction.
- Fix round 1's NOTE, a chore: step 6 created the store directory before its listing check, so a listing halt left an empty store directory though Failure says nothing is written into the store. The directory is now created after the extraction, and 048's command-set table lists the steps in that order.
- Three chores:
  - the halt set is named rather than given as a property wider than it, in ductus.md and, by owner decision, in spec.md's MCP Wiring;
  - Archive fetch and extract's failure trigger no longer names a tar exit;
  - State B states the outcome of a CLI call that cannot run or refuses its input.

Low-confidence: BE-INPUT-004 on the [runtime] path value. 048's plan states that the value names a binary the project chose to execute.

Stored decisions: six matched and stand; two expired, since the managed-block merges and the greenfield --ref re-run issues no longer stand; six are retained, naming files this window does not hold.

Not examined here: the two workflows' checks under Linux GNU tar and Windows tar, which only their CI runs show.

3 fix round(s): 3 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

21 of 21 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

### LOW-CONFIDENCE: BE-INPUT-004 — A [runtime] path config value is used as a path in ln -sf, cp and an exec probe with no validation, quoting, canonicalization or containment

- **File**: `framework/bootstrap/ductus.md:382-394,476-478`
- **Rule**: User-supplied values MUST NOT be used directly in filesystem paths. Filesystem operations MUST resolve the canonical path and verify it falls within the expected base directory before opening the file.
- **Finding**: Branch 1 and Pointer materialization steps 3 and 4 interpolate the committed `[runtime] path` value into `<path> --version`, `ln -sf {path} .ductus/bin/ductus` and `cp {path} .ductus/bin/ductus`. The rule counts a configuration value and repository-committed text as user input, and the text names no character screen, quoting rule or containment check (the project-name, specs-root and language inputs, by contrast, each have an allowlist in Collect Project Inputs). Low confidence because the design deliberately lets the project point anywhere (absolute or repo-relative), a repository that controls this key already controls the binary the run executes and the one .mcp.json registers, so a screen would not move the trust boundary; the same defect read as shell-string construction is BE-INPUT-003's and is reported here once.
- **Auto-fixable**: no
- **Suggested fix**: Validate the value before use (no shell metacharacters or whitespace, or pass it as an argument vector with quoting stated), resolve it and state it names a regular executable file; or waive with the reason that the key's author already controls the executed binary.

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: ductus.md State A (Detection mechanism, 'Live is not current' version check, Pointer check) is silent only on a pin match and warns/re-acquires per the superseding clause; 048 scenario state-a-version-checks-the-pin exists and commit 171d5024 (fix(048): version-check the runtime in State A) exists; no detection message emitted on a current runtime (Closing restart: 'Nothing is emitted when a State A run leaves the set empty').
- AC2 — met: ductus.md Detection mechanism: 'counting deferred or lazily-loaded tool names as present ... Any match => State A'; Instructions preamble: 'When the server is registered for your session, call the corresponding tool'; the spec's own advice to fetch the schema through the host mechanism is not restated in ductus.md.
- AC3 — met: ductus.md Detection mechanism: introspection of own tool inventory (`mcp__ductus__*` on Claude, `ductus__*` bridge tools on Pi) 'needs no shell and no permission'; the store probe runs 'Only when introspection finds no ductus tool' (read whole file; grep for 'command -v'/'which ductus' finds only the seed entry and the 'no step runs' sentence).
- AC4 — met: Superseded clause holds: ductus.md Store probe says 'A probe that cannot run classifies the run as State B, which acquires'; whole-file grep -c 'State C' = 0 in ductus.md and ductus-procedure.md, so no step classifies State C; Edge Cases (ductus-procedure.md) 'store probe cannot run or is denied' restates State B.
- AC5 — met: Superseded clause holds: Claude settings_template in ductus.md Agent Registry carries "Bash(~/.ductus/bin/ductus --version)" and its .exe twin plus "Bash(command -v *)"; Pi seed is {}; Permission Setup states 'command -v, which no step runs' and grep 'command -v' ductus.md hits only line 74 (seed) and 173 (that sentence), ductus-procedure.md none; scripts/audit/runtime-probe-parity.sh run: exit 0.
- AC6 — met: ductus.md MCP registration (per-agent) table: claude .mcp.json write-file, pi .pi/extensions/ductus.ts write-file project-local; State B step 3 and MCP wiring write the file, Pi's via the Bridge step of Pi layout after extraction; framework/bootstrap/pi/ductus-bridge.ts exists (19819 bytes), spawns .ductus/bin/ductus mcp and registers ductus__<name>; spec 064 AC4 states the same mechanism and scope.
- AC7 — met: ductus.md MCP wiring cases: 'Has mcpServers, no ductus' preserves every other server and top-level key; 'Already has a ductus entry - no-op; leave the file byte-unchanged'; wiring is host-side with no primitive (stated in the same section).
- AC8 — met: Superseded-in-part clause holds: ductus.md Runtime acquisition Branch 2 step 2 (idempotent store probe vs {pin}, 048 AC6/AC7 confirmed in 048 spec.md), State A version check, Branch 1 step 3 warning for [runtime] path (048 AC18 confirmed); constitution Versioning subsection pins the runtime in lockstep and 022 spec.md carries the 'Versioning enforcement' decision, as the criterion's last sentence says.
- AC9 — met: ductus.md MCP wiring: 'No mcpServers key - add the key with just the ductus entry; preserve all other top-level keys'.
- AC10 — met: ductus.md MCP wiring 'Not valid JSON - do not touch the file. Skip wiring and warn ... The runtime is still acquired and the pointer still materialized'; ductus-procedure.md Edge Cases 'wiring file is malformed JSON' and Closing restart variant 'The wiring was skipped' say the run continues via CLI (superseded-on-path clause).
- AC11 — met: ductus.md State B step 5 and ductus-procedure.md Closing restart name the store path and every file written, say it reduces tokens ('uses far fewer tokens') and ask for one restart for the MCP tool surface; the stale-ductus.md abort carries the same list (Pre-flight abort first bullet); 048 scenario state-b-continues-in-session read, matches.
- AC12 — met: ductus.md State B: 'issues no separate consent prompt'; whole-file grep AskUserQuestion finds only Agent Selection (line 152) and Collect Project Inputs (line 595); ductus-procedure.md prompts only for migrations ('Apply now? (Y/n)'); disclosure is the Closing restart or Pre-flight abort file list.
- AC13 — met: ductus.md State B step 4 and 'ductus runtime auto-wiring' add mcp__ductus__* to Claude's settings file; Permission Setup step 3 and the auto-wiring section say additive, idempotent, 'no existing entry is removed, reordered, or overwritten'; Pi: 'no permission entries exist to add'. (Retired-entries removal in Permission Setup step 2 is the seed's, not this wiring write.)
- AC14 — met: Superseded clause holds: grep -c 'State C' = 0 in ductus.md and ductus-procedure.md; State B acquires a missing binary (Runtime acquisition Branch 2); ductus-procedure.md Post-Scaffolding tips carry no single README-Runtime tip (grep -i 'tip' shows only the two PKM tips); the first-run next step 7 says the runtime is already acquired.
- AC15 — met: ductus.md Pre-flight Phase runs source resolution, runtime detection and self-update before Pre-run Migrations/archive fetch; Pre-flight abort inspects the two sets separately, the stale abort carries State B's file list and notice, State B alone does not abort; Pi exception (bridge written after extraction, a never-wired Pi adopter restarts once more) is stated in Pre-flight Phase and Edge Cases; spec 064 Verified Pi Layout read.
- AC16 — met: ductus.md Agent Registry has exactly two rows (claude/claude-style, pi/pi) and MCP registration (per-agent) has a row for each with mechanism write-file; Pi's settings_template is {} and Permission Setup skips Pi entirely; spec 064 lines 23/58 confirm Pi has no permission-gating surface.
- AC17 — met: README.md Quick start line 33 ('acquires and wires the runtime'), Installing (per agent) line 137 ('/ductus acquires the runtime and wires it in the same run, automatically for both') and Claude row ('MCP wiring is fully automatic'); The runtime section (line 160) describes the store and pointer; commit cb5b330c ('update the docs') touches README.md, docs/runtime.md; read README lines 19-46 via grep and 120-176, not the whole 34555-byte file.
- AC18 — met: ductus.md Inputs says 'Do not prompt for any missing project input here'; Collect Project Inputs sits after the Pre-flight abort, reads [project] name/description/languages (then [host] project fallback) and persists them to the active config's [project] table, 'prompts for nothing' on update runs, host.project written from project.name (Instructions step 6); the abort-skip list in Pre-flight abort names Collect Project Inputs first; scenario project-inputs-asked-once read whole and matches; AskUserQuestion grep shows no other project-input prompt.
- AC19 — met: ductus.md Archive fetch and extract uses `https://codeload.github.com/${DUCTUS_REPO:-stonean/ductus}/tar.gz/{archive-ref}` and says Claude's seed grants no curl (one prompt in Manual mode); the Claude settings_template contains no curl entry and the retired-seed list holds "Bash(curl *)"; scenario archive-fetch-direct-codeload read whole and matches (its Context is a dated record of the pre-fix prompt).
- AC20 — met: Current text holds on every route: halt set {extract-archive, apply-manifest, enforce-manifest} stated in ductus.md Instructions preamble (route-neutral), step 3 ('A call that cannot run halts the run the same way ... so that tar -xzf never runs in its place'), and State A cannot-run bullet; scenario state-a-deterministic-path-forcing and 007 plan 'Paths built from the inputs are contained' (commit fba832e8) agree; runtime validate_contained is called in apply_manifest.rs and enforce_manifest.rs and extract_archive.rs screens ../absolute/non-UTF-8/link entries. Class enumeration in class_check: every primitive-backed step whose shell spec writes, removes or extracts a path built from input is in the halt set; State B's CLI route is covered by the preamble and by the 'refusal stands' bullet (CLI --*-json refusal), though State B's own section does not restate the halt.
- AC21 — met: Drove bash scripts/audit/runtime-probe-parity.sh: it only reads ductus.md and writes to stdout/stderr (git status clean), exit 0 with 'skipped pi' on stderr; wired as Family 15 in scripts/audit/run-all.sh line 75 and framework/reference/audit.md item 15; fixtures in fixtures-C copies: dropping the .exe twin from the claude row exits 1 naming the literal, dropping the Unix form exits 1, adding an unnamed 'zed' agent row exits 1, unmutated control exits 0 (no dedicated test under scripts/tests exists; proof is by these mutations in a copy).

## Observations

- security: the Frontmatter Migration rewrites each globbed spec.md and scenario file in place with no symbolic-link screen, and walks the literal specs/ rather than the configured spec root — `framework/bootstrap/ductus-procedure.md` — **waived**: ductus-procedure.md is outside this window, and 017's and 018's open reviews hold it in scope; the migration serves pre-017 projects, which predate [paths] specs-root (040), so the literal specs/ is their root (decided by the rule)
- security: Pre-run Migrations step 7.1 reads framework/migrations/{id}.md with an id taken from the archive's registry and no containment check — `framework/bootstrap/ductus-procedure.md` — **waived**: a read, not a write, of procedure text the framework's own registry selects and the agent already executes; ductus-procedure.md is outside this window, and 017's and 018's open reviews hold it in scope (decided by the rule)
- security: Branch 2 step 6 required the listing to end with the name ductus, looser than the workflows' exact last-field test, so an entry such as x/ductus or 'x ductus' passed the prose and only the member-name extraction stopped it, and no text said a failed extraction halts (the security and quality re-checks) — `framework/bootstrap/ductus.md` — **fixed** (decided by the rule)
- security: step 6 and Failure halt naming the listing, so a hostile archive's whole entry list reaches the host's context and the halt reason — `framework/bootstrap/ductus.md` — **waived**: the archive has already matched its published digest and its download is time-bounded, so the listing is the release's own; both workflows' errors name the listing the same way (decided by the rule)
- convention: the halt exception for a call that cannot run is stated as a property (a primitive that screens the paths it extracts, writes or removes) in ductus.md's Instructions preamble and State A bullet and in spec.md's MCP Wiring section, but as three names in AC20 and the scenario, while merge-managed-block, merge-permissions, merge-status-hook and write-session also screen the paths they write — `framework/bootstrap/ductus.md` — **fixed** (decided by the rule)
- convention: Archive fetch and extract's failure trigger still names a non-zero tar exit, although an extract-archive call that cannot run now halts so tar never runs in its place — `framework/bootstrap/ductus.md` — **fixed** (decided by the rule)
- convention: the Archive fetch and extract abort message advises checking network connectivity even for an archive extract-archive refused — `framework/bootstrap/ductus.md` — **waived**: the message ends 'or report this if it persists', which covers a refused archive, and 015's spec quotes its wording, so rewording it changes no outcome and widens the change to another spec (decided by the rule)
- convention: the halt for a call that cannot run is cited as (State A) in the Instructions preamble and step 3, and State B's section names no outcome for a CLI call that cannot run or refuses its input — `framework/bootstrap/ductus.md` — **fixed** (decided by the rule)
- convention: Branch 2 step 6 created the store directory before its listing check, so a listing halt left an empty store directory though Failure says nothing is written into the store (fix round 1's NOTE) — `framework/bootstrap/ductus.md` — **fixed** (decided by the rule)
- other: spec.md's Resolved Questions entry Version compatibility still says a too-old wired ductus degrades to the markdown fallback — `specs/029-bootstrap-runtime-autowire/spec.md` — **waived**: a resolved decision recorded as it was taken, already marked superseded in part by 048; AC20 and the MCP Wiring section state the current rule (decided by the rule)
- convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md` — **waived**: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
- convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md` — **waived**: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
- convention: framework/bootstrap/ductus.md §Project Configuration says the description renders wherever ductus names a source (fix rounds 1 and 3's NOTEs) — `framework/bootstrap/ductus.md` — **waived**: A comment the installer writes into adopters' config; correcting it is a bootstrap change, which this repository verifies only with a real adopter run, out of proportion to one clause; the data model states which surfaces carry the description. (decided by the rule)
- other: the markdown-only reference-cleanup prose differs from enforce-manifest at the edges (links never removed, dot-prefixed .md matched) — `framework/bootstrap/ductus.md` — **waived**: contrived: nobody writes either into a ductus-owned directory (owner decision, 2026-10-26)
- other: step 4's per-agent apply-manifest loop sits in one walker step whose context carries one map — `framework/bootstrap/ductus.md` — **waived**: the real bootstrap is walked by the host, as step 8's per-directory loop already is
- other: main's bootstrap calls `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, so a `--ref=main` run errors at that step and falls back to the unflagged call until a release moves the pin — `framework/bootstrap/ductus.md` — **waived**: An observation about the pipeline's own machinery made while verifying framework work (AGENTS.md): it concerns 055's step, not 077's, blocks nothing here, and ends when the next release tag moves the runtime pin to one carrying the flag.
- other: the listing check is copied in runtime-acquisition.yml, runtime-release.yml's acquire job and Branch 2 step 6, and nothing checks the copies against each other — `.github/workflows/runtime-release.yml` — **waived**: this review drove the reject branches on nine crafted archives for step 6's check and for each workflow's snippet as written, runtime-acquisition.yml's header binds it to the procedure, and a drift check across the copies is new work, not a defect in 029's contract (decided by the rule)
- other: on Windows a tar that prints CRLF line endings would leave a carriage return on the listing's last field, and the workflows' gate would reject ductus.exe — `.github/workflows/runtime-acquisition.yml` — **waived**: unverified here; the dispatched runtime-acquisition run and every release's acquire job run the snippet on windows-latest under Git Bash, and a false rejection fails loudly rather than passing (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/implement.md` — 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/review.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `.ductus/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/review.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-implement.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-review.md` — 066-audit-records-and-the-completion-gate
- `AGENTS.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `.ductus/docs/agents/developing-ductus.md` — 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate
- `docs/slash-commands.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/bootstrap/ductus-procedure.md` — 027-bootstrap-migration-registry
- `framework/commands/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/commands/review.md` — 066-audit-records-and-the-completion-gate
- `framework/constitution.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/reference/review.md` — 066-audit-records-and-the-completion-gate
- `runtime/src/interpreter/mod.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/interpreter/payload.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/main.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/check_review_gate.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/classify_spec_edit.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/compute_review_scope.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/criterion_verdicts.rs` — 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/invalidate_review.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/mod.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/write_review.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/schema/descriptions.rs` — 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/schema/extensions.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/schema/primitives.rs` — 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/fixtures/implement-basic/.govern.session.toml` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/implement-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/review-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/implement-basic.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/review-basic.jsonl` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/mcp.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/review_command.rs` — 066-audit-records-and-the-completion-gate
- `scripts/audit/migration-coverage.sh` — 027-bootstrap-migration-registry
- `scripts/mutation/criteria-in-review-066.py` — 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `scripts/mutation/fix-round-record.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/reopen-measure.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/reviews-follow-what-changed.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/scope-attribution.py` — 066-audit-records-and-the-completion-gate
- `scripts/tests/test-migration-coverage.sh` — 027-bootstrap-migration-registry
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/000-slash-commands/scenarios/validation-gates.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/000-slash-commands/spec.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/007-govern-workflow/analysis.md` — 007-govern-workflow
- `specs/007-govern-workflow/plan.md` — 007-govern-workflow
- `specs/007-govern-workflow/spec.md` — 007-govern-workflow
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/020-code-review/data-model.md` — 020-code-review, 066-audit-records-and-the-completion-gate
- `specs/020-code-review/spec.md` — 020-code-review, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/data-model.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/scenarios/implement-completion-gate.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/026-framework-self-audit/analysis.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/plan.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/family-10-migration-coverage.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/spec.md` — 026-framework-self-audit
- `specs/027-bootstrap-migration-registry/plan.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migrations-apply-once-per-repo.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/spec.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/tasks.md` — 027-bootstrap-migration-registry
- `specs/056-bootstrap-archive-boundary-split/analysis.md` — 056-bootstrap-archive-boundary-split
- `specs/056-bootstrap-archive-boundary-split/plan.md` — 056-bootstrap-archive-boundary-split
- `specs/056-bootstrap-archive-boundary-split/spec.md` — 056-bootstrap-archive-boundary-split
- `specs/056-bootstrap-archive-boundary-split/tasks.md` — 056-bootstrap-archive-boundary-split
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/079-review-and-analysis-work-converges/spec.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/spec.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed

## Unexamined governance

*None.*
