---
spec: 015-tarball-fetch
last-run: 2026-11-23T21:33:47Z
reviewed-against: 0db66fcc2811a609f2b2801a0b8d55ef6b271876
diff-base: 93ff4777b013994863228ab1a25fc35d188a523b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 11
scope: 14
unread:
  - .ductus/reference/audit.md
  - framework/bootstrap/govern.md
  - specs/083-commit-subjects-name-specs-in-a-form-the-project-declares/spec.md
excluded: 1
contract-breaks: 0
skipped-passes: []
fix-rounds: 2
re-reads: 2
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest: {}
criteria-judged: 14
criteria-met: 14
criteria-verdicts:
  - label: AC1
    digest: b1e79960a469129a4cb90cff3a7b6789c60d011090beeb575861325829c803ed
  - label: AC2
    digest: 9aa6ce65f4fb70cf4b91bbb77bea5138e7e6c031bf4faa478553d4b10f588dea
  - label: AC3
    digest: 87c0d6875ec5e21b789adf894073f4faed4f606208034c3d26c1923523c97d51
  - label: AC4
    digest: 650fb75074f1f3b082dcffbcdd1ca771d99a6214f8d0adf60579bd1cf8998c3d
  - label: AC5
    digest: c2ef6cc02689edef5a6dabe76df019908f431111e9bb25e8e496cc9702d8d20e
  - label: AC6
    digest: 8422bd284a4387ac561ebbb1e450c574bf8c67396983c9ba775b001282cd3a69
  - label: AC7
    digest: b6968d58863e1fb8506637e87be2dddcf0ba64a7a67997d9f47f564cf983bc18
  - label: AC8
    digest: 268ca11a4f4f32698a82dba41d958cfb518fc2c1f79710bb9aeb3b16e2a282d8
  - label: AC9
    digest: 7bff14ae322eb397dca8c5a15be02eba856082e0c1a6cbc1d824b21f8c509e17
  - label: AC10
    digest: 7868719e7f6c33ba0dba0fdd0d47e789a0b1d8482eb3100c071516c1385ecd9e
  - label: AC11
    digest: 1a58cf4be8cb4b510c63222344e1c9e3e6fb3b08b40c86c3ae49d04331197b37
  - label: AC12
    digest: 207a7c9fde87f2bd92306341fd136e67a35487dcefd1b9cb83c1249b38187a5d
  - label: AC13
    digest: 2e11b7e73cc32b7d98b1090b5b28e62f08b00d7a09dd26044129890626257d71
  - label: AC14
    digest: 361ffd177081df2745a0ffbb890c215c358d6f1e20055e7f399298fec0284551
blocking: false
dispositions:
  fixed: 6
  deferred: 0
  waived: 7
  undispositioned: 0
decisions:
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue
    decided-at: 2026-11-11T15:52:57Z
    decided-by: rule
  - key: "stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change"
    decided-at: 2026-11-11T15:52:57Z
    decided-by: rule
  - key: "convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the template has frontmatter, so the walk skips it with a report line and writes nothing
    decided-at: 2026-11-11T15:52:57Z
    decided-by: andy@stone.dev
  - key: "other: the trimmed archive, which is also GitHub's release-page 'Source code' download, now carries no LICENSE and no notice of the MIT terms anywhere (git archive HEAD holds 0 files with the copyright or permission text), although it carries substantial portions of the Software (rules, constitution, templates, hooks) and every /ductus run copies parts of it into adopter trees. Before the trim LICENSE rode in every archive. A fix is not mechanical: carrying /LICENSE requires Family 42 to exempt it, a durable requirement. — `.gitattributes`"
    outcome: waived
    reason: AC14 limits the archive to the files a /ductus run reads and no run reads LICENSE; no manifest row ever copied it into an adopter's tree, so what adopters receive is unchanged, and carrying it would add a requirement to AC14, which is the owner's to make
    decided-at: 2026-11-23T21:31:39Z
    decided-by: rule
---

# Review — 015-tarball-fetch

## Summary

Window 93ff4777..0db66fcc (base-source reopen), passes in one subagent over the four loaded rule files, each read whole (configuration-cross.md 29,932, quality-cross.md 9,060, security-backend.md 118,346, security-cross.md 24,283 bytes), AGENTS.md, and 11 of the 14 scope entries read whole; the five backend files whose headers give their rules to analysis were not loaded. Unread, by name: framework/bootstrap/govern.md, byte-identical to framework/bootstrap/ductus.md by cmp, which was read whole; .ductus/reference/audit.md, the generator's copy of framework/reference/audit.md, read whole, whose diff after the {project} substitution is empty and whose generators step reported in sync; and specs/083-commit-subjects-name-specs-in-a-form-the-project-declares/spec.md, another session's untracked draft spec that entered the derived scope after the passes ran and is no part of 015's change. No finding. Seven observations: six fixed in the run by the rule (stale size prose in ductus.md and govern.md, plan Overview, spec Problem, and three comments in scripts/audit/archive-contents.sh, one adding a coverage count), one waived by the rule (LICENSE no longer in the archive). Six stored decisions on ductus.md fired again and stand; one expired because ductus.md now names a dropped host's leftover tree; two on ductus-procedure.md are retained, that file being out of scope. Criteria: 14 judged, 14 met. AC7 was judged not met, since 022's task 18 dropped Bash(tar *) from Claude's seed, and its annotation was corrected in the run (the agent's recommendation, adopted by the operator, 2026-11-23), then judged met. Round 1's re-read, which read each path's round diff, found two SHOULDs on the {download-timeout} row's archive size; they were fixed with wording naming both archives rather than the prescribed edit, which would have misdescribed the runtime's release archives (7,288,539 to 8,132,268 bytes at ductus-v0.64.0), and round 2's re-read was clean. A scratch-adopter /ductus --ref=main at 0db66fcc completed: 78 files extracted, apply-manifest source-missing 0 on the shared and per-agent calls.

2 fix round(s): 2 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

14 of 14 acceptance criteria judged met.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Deferred findings

*None.*

## Acceptance criteria

- AC1 — met: framework/bootstrap/ductus.md ## File Fetching holds ### Archive fetch and extract (one codeload curl, extract-archive, {framework-root} derivation) and ### Per-file resolution; no per-file curl flow remains for ductus-repo files.
- AC2 — met: ductus.md ### Archive fetch and extract issues exactly one codeload.github.com archive curl; the pre-flight pin, self-update and latest-release/migration-registry fetches (### Source resolution, ### Self-update check) are the ones the annotation names, and the only per-language curls are the .gitignore ones in ## Shared Files.
- AC3 — met: apply-manifest tests all pass (cargo test apply_manifest: 66 passed): update_creates_when_dest_absent, update_updates_when_dest_differs, create_writes_only_when_dest_absent, skip_if_conflict_does_not_apply_substitutions, pinned_short_circuits_before_any_io_regardless_of_strategy; merge is merge-managed-block (ductus.md Instructions steps 5-6, ## Shared Files .gitignore/constitution imports), not an apply-manifest strategy, and I did not drive it here.
- AC4 — met: ductus.md ### Archive fetch and extract abort message fires before any manifest write (Instructions steps 2-4), ductus-procedure.md Edge Cases 'Archive fetch or extract fails' says clean abort with no ductus.md or wiring writes pending; Pre-flight abort (read whole) shows the only earlier writes are seed, recorded source and wiring/ductus.md, not scaffolding; the fetch failure itself is a documented procedure, not run.
- AC5 — met: runtime test source_missing_returns_action_not_error passes (absent.md gives action source-missing, source_missing=1, the present entry still created); ductus-procedure.md Edge Cases 'A required source file is absent' and Post-Scaffolding Output list the warning line 'Source not found in archive: {source-path}; skipping.'.
- AC6 — met: ductus-procedure.md ## Post-Scaffolding Output (read whole) lists 'temp directory: {tempdir} - left for the OS to sweep'; ductus.md ### Archive fetch and extract abort message names {tempdir}; ### Cleanup says /ductus does not delete it and no rm is seeded.
- AC7 — met: corrected in this run (the agent's recommendation, adopted by the operator, 2026-11-23): framework/bootstrap/ductus.md Agent Registry's claude settings_template carries Bash(mktemp -d -t ductus-XXXXXX), no Bash(tar *) and no rm allow, Permission Setup says curl and tar are not seeded (022 task 18), and pi's is {}
- AC8 — met: Claude settings_template in ductus.md ## Agent Registry carries Read(//private/var/folders/**/T/ductus-*/**), Read(//var/folders/**/T/ductus-*/**) and Read(//tmp/ductus-*/**), covering both macOS temp roots and Linux /tmp; the single-slash forms are in the retired list.
- AC9 — met: ductus.md ## Post-Write Integrity Check re-reads {tempdir}/ductus.md.upstream or {framework-root}/framework/bootstrap/ductus.md on failure, no curl.
- AC10 — met: ductus.md ### Self-update check (Per-agent comparison, Stale -> defer to pre-flight abort) still byte-compares the installed ductus.md against the resolved ref's copy and notices on stale; the copy now comes from a small raw fetch at the same ref as the archive, which the spec body records.
- AC11 — met: ductus.md ## Shared Files .gitignore step 5 still runs a separate curl to raw.githubusercontent.com/github/gitignore/main/{Language}.gitignore, outside the archive; ## File Fetching intro states it is not part of the archive.
- AC12 — met: runtime tests update_is_unchanged_and_preserves_mtime_when_matched and full_re_run_is_idempotent_with_unchanged_actions pass (second run: unchanged=1, skipped_exists=2, mtimes preserved); ductus-procedure.md Edge Cases 'ductus.md content already matches' and ## Idempotency state the same.
- AC13 — met: ductus.md ## Pre-flight Phase 'Create {tempdir} first' runs mktemp -d -t ductus-XXXXXX every run and says never reuse a prior directory; ### Cleanup repeats it; the Claude seed allows exactly that command.
- AC14 — met: Ran bash scripts/audit/archive-contents.sh (78 read, 78 archived, 78 in both, 0 findings, rc 0) and bash scripts/tests/test-archive-contents.sh (cases A-N all PASS: B omitted read file fails, C/D carried unread file fails, N real tree passes); git archive HEAD lists the same 78 files, only .markdownlint-cli2.jsonc outside framework/; .gitattributes allow-list; run-all.sh Family 42 and framework-checks.yml run it.

## Observations

- convention: framework/bootstrap/ductus.md still describes the framework archive by its pre-trim size in three places the AC14 change left unswept: Pre-flight Phase intro ('Neither pays the cost of the multi-megabyte archive'), Branch 2 step 1 ('it is the archive's multi-megabyte cost this phase avoids'), and the Cleanup subsection ('several megabytes of extracted files'). The archive is now 78 files, about 0.4 MB compressed and 1.3 MB extracted. framework/bootstrap/govern.md is its byte-identical copy and carries the same three. — `framework/bootstrap/ductus.md` — **fixed** (decided by the rule)
- convention: specs/015-tarball-fetch/plan.md Overview still says 'The change is localized to a single source file' and 'No new files; no template changes; no scaffolding-manifest changes', which task 7's own Affected Files rows contradict (a new audit family script, a new test, .gitattributes lines, a workflow step). The plan is the design as it stands. — `specs/015-tarball-fetch/plan.md` — **fixed** (decided by the rule)
- convention: specs/015-tarball-fetch/spec.md Problem still claims 'the entire framework directory is well under a megabyte'; tracked framework/ is 82 files and 1,541,517 bytes and the extracted archive is 1,312,777 bytes at 0db66fcc, which the spec's own Cleanup section records as about 1.3 MB. — `specs/015-tarball-fetch/spec.md` — **fixed** (decided by the rule)
- other: the trimmed archive, which is also GitHub's release-page 'Source code' download, now carries no LICENSE and no notice of the MIT terms anywhere (git archive HEAD holds 0 files with the copyright or permission text), although it carries substantial portions of the Software (rules, constitution, templates, hooks) and every /ductus run copies parts of it into adopter trees. Before the trim LICENSE rode in every archive. A fix is not mechanical: carrying /LICENSE requires Family 42 to exempt it, a durable requirement. — `.gitattributes` — **waived**: AC14 limits the archive to the files a /ductus run reads and no run reads LICENSE; no manifest row ever copied it into an adopter's tree, so what adopters receive is unchanged, and carrying it would add a requirement to AC14, which is the owner's to make (decided by the rule)
- convention: archive-contents.sh re-implements two parsers lib.sh already owns, the maintainer-only list (lib.sh maintainer_only_commands) and the Agent Registry table header (lib.sh agent_config_dirs, same regex), and says nowhere why. The reason exists (lib.sh reads the worktree, the family must read HEAD, since the archive is a commit's), so it belongs in a comment at the code. — `scripts/audit/archive-contents.sh` — **fixed** (decided by the rule)
- convention: archive-contents.sh's header claims the derived read set 'omits nothing a run reads', but it scans only ductus.md and ductus-procedure.md; the per-migration procedure files a run also reads (framework/migrations/{id}.md) are not scanned. None names a framework file to read today (checked all 15; audit-record-relocate.md and ductus-rename.md name two files only in messages or prose), and a future one would fail closed as 'carried, no run reads it', whose suggested fix (mark it export-ignore) would then break that migration. The header should state the bound. — `scripts/audit/archive-contents.sh` — **fixed** (decided by the rule)
- convention: archive-contents.sh parses markdown structure (code spans, manifest table rows, the Agent Registry table) in an embedded python3 heredoc, the shape scripts/audit/README.md Contract names as 'the signal that a check took the fallback without earning it' ('Several families here predate that standard ... the legacy shape, not the pattern to copy'). The reason it is not a primitive-backed entry point exists only in 015's plan.md (subject is this repository alone, no adopter runs it), and neither the script header nor its README or audit.md entry says so. — `scripts/audit/archive-contents.sh` — **fixed** (decided by the rule)
- convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md` — **waived**: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
- convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md` — **waived**: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
- convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md` — **waived**: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
- convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md` — **waived**: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
- other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md` — **waived**: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue (decided by the rule)
- stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md` — **waived**: a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `specs/073-decided-rule-departures-are-recorded/spec.md`
- `specs/073-decided-rule-departures-are-recorded/spec.md` — left out of fix round 1

## Unexamined governance

*None.*
