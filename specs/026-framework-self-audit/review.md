---
spec: 026-framework-self-audit
last-run: 2026-11-15T21:16:05Z
reviewed-against: a2de525eece1e614aaa29b42fd0ea1f155a05757
diff-base: 368249a9cd1942bc5344f4e73106cd1002e739c0
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 23
scope: 31
unread:
  - scripts/audit/README.md
  - scripts/audit/installer-command-parity.sh
  - scripts/audit/placeholder-roundtrip.sh
  - scripts/audit/primitive-promotion-candidates.sh
  - scripts/audit/readme-command-parity.sh
  - scripts/tests/test-deprecation-removal.sh
  - scripts/tests/test-migration-coverage.sh
  - specs/026-framework-self-audit/scenarios/readme-command-parity.md
excluded: 60
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/audit-ci-hard-gate.md: a7bad7a167532019112d79a746696ad32d963171598d2283072af0e9f3234be7
  scenarios/audit-script-refactors.md: 4d4a93fd968898006fa4780beb8088e48616ed1ceae54574ac38a3a02ad9583b
  scenarios/families-follow-the-command-reference-split.md: 2827003a57acd578cd14add059be8c7fcdee0ae80389acad64f282d8c9ffc1ea
  scenarios/family-10-migration-coverage.md: 6b90b5c3838db4d12ac521c90a4085a841ad8af6f47a7ad44d808fe47b71f3df
  scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
  scenarios/family-18-marker-list-parity.md: 296f55313954ec984b5260945b92639d15b8458f707f90e94e0bd1bbc0f052c0
  scenarios/family-22-adopter-shell-behavior.md: 995c7c45e1918732b886e1174e5b627161bdd3c42ea824a4cbce7b04b70e02ff
  scenarios/family-23-sweep-target-manifest-parity.md: fabc89386b8c36156f41c09b5ae28e5f0cd3130c08abc284fa90dcd37507ebb1
  scenarios/family-24-rename-sweep-residue.md: 3b70ff8b57ef5bbd117ce0bd29752dd15a481d07e28a3fc3505671a2e6507cea
  scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
  scenarios/family-26-broken-relative-links.md: 9e03cfe250dbffe6128229b1bb064c8b574b61ead93ed1ce5fd2fe12ec8e615d
  scenarios/family-27-done-spec-unchecked-criteria.md: c7c66e810ae27435c40bb372d7295b101f480e897f91a648db3d0902cacf57b1
  scenarios/family-28-audit-family-registry-parity.md: bf80f1d64718deb87a7dec79c69e9248e8c1ab4a41e65c638488fdd9fcd5e529
  scenarios/family-34-step-reference-integrity.md: 4494835543a7ebfa3fe429d732652a658faf6f0b0cc8376daa05cb4e09ad2ca0
  scenarios/family-35-manifest-destination-links.md: 28297923f8f5f4ec6833696028c4076e247cfadb3d945b0a2446acc18237cc11
  scenarios/family-36-self-url-resolution.md: 7840a2f17391f0644e2a7e5809cccdfc3e89039e05c1d94f2432e89461b716b3
  scenarios/family-39-shipped-anchor-resolution.md: c25191adf85b75351517d55c89a55a24a5d083e325f894f680c04ace54401e8f
  scenarios/family-40-deprecated-rules-are-removed-on-time.md: 689441c4d67ea3b73b147f0bf319db6ef7a50034ea46218ef67799dee0f5732e
  scenarios/host-namespace-parity.md: e8a67c95e2e596cf9a231bbfa26545c4beee96af1a043fd83608899e16087790
  scenarios/link-check-consolidation.md: edaa15335c49241c5565a0b4063a3f59a5233b5b4a0c6362cbe9f4356b82812f
  scenarios/readme-command-parity.md: 899fdc8ebb306541496892772ec4540b4be968aed55ee366c87185cdcc5304b6
blocking: false
dispositions:
  fixed: 7
  deferred: 0
  waived: 8
  undispositioned: 0
decisions:
  - key: "other: 066 task 19's item 0 is applied but unchecked, and its item 3 and note 5 name text that has moved — `specs/066-audit-records-and-the-completion-gate/tasks.md`"
    outcome: waived
    reason: 066's own task list, reconciled by its walker when it runs task 19; outside 026's code
    decided-at: 2026-11-10T14:03:43Z
    decided-by: rule
  - key: "other: 027 task 14's handoff counts 30 ticked criteria where 027 labels 27 — `specs/027-bootstrap-migration-registry/tasks.md`"
    outcome: waived
    reason: 027's handoff prose, which its own criteria verification re-counts before its runs; three of the 30 boxes are not criteria
    decided-at: 2026-11-10T14:03:43Z
    decided-by: rule
  - key: "other: 063's plan says if-requirement-changes takes any other trim, which edits spec.md, where tasks.md's last-resort trim takes it too — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`"
    outcome: waived
    reason: 063's own plan, which its close's analysis reads; the value is right and only the parenthetical is narrow
    decided-at: 2026-11-10T14:03:43Z
    decided-by: rule
  - key: "other: check-step-references' not-a-procedure field doc defines the set by files holding several numbered lists, though it also holds files with no step or too few — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: the primitive's schema doc, 067's surface and outside 026's code; the family's own stderr line now says not one procedure
    decided-at: 2026-11-10T14:03:43Z
    decided-by: rule
  - key: "other: BE-ERR-002's Verification reaches a request-response protocol a process serves over its stdio, while AGENTS.md's gotcha says BE-ERR-002's correlation ID never fires against the runtime and an MCP tool error carries no code — `AGENTS.md`"
    outcome: waived
    reason: "Not mechanical, and 077's review waived the same tension by the rule: AGENTS.md's gotcha is this repository's reading of the rule against its own local runtime, naming the parts that assume a deployed service, while the rule states what an adopter's API owes, and the two meet at the gotcha's 'Name the boundary first'."
    decided-at: 2026-11-10T14:03:43Z
    decided-by: andy@stone.dev
  - key: "other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml`"
    outcome: waived
    reason: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
    decided-at: 2026-11-10T14:03:43Z
    decided-by: andy@stone.dev
  - key: "other: runs-on: ubuntu-latest is mutable and now supplies framework-checks' Node too — `.github/workflows/framework-checks.yml`"
    outcome: waived
    reason: The owner chose the runner image's Node (022 task 3, option a), and a GitHub-hosted image has no immutable reference to pin.
    decided-at: 2026-11-10T14:03:43Z
    decided-by: andy@stone.dev
  - key: "other: verify-installer ignores a Retry-After of 2^63 or more and reads a redirect hop's Retry-After when the final hop sends none — `.github/workflows/runtime-release.yml`"
    outcome: waived
    reason: an out-of-range value falls back to the jittered wait, and GitHub's release-asset redirect sends no Retry-After, so neither changes a run
    decided-at: 2026-11-10T14:03:43Z
    decided-by: andy@stone.dev
  - key: "other: loose \"gets a disposition\" phrasing in 050, 047, 041, 058 and walker.rs — `specs/047-analyze-findings-durability/spec.md`"
    outcome: waived
    reason: none asserts that an undecided finding holds done, and each run still proposes a disposition for every finding
    decided-at: 2026-11-10T14:03:43Z
    decided-by: andy@stone.dev
  - key: "security: `audit.md` step 1 and `analyze.md`'s drift check send the agent to `.ductus/bin/ductus run-generator`, which the seeded `Bash(.ductus/bin/ductus *)` allow runs with no prompt until 022 task 18 narrows it — `framework/commands/audit.md:31`"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-11-10T14:03:43Z
    decided-by: andy@stone.dev
  - key: "convention: Family 16's reference arm is not section-scoped and duplicates Family 35's row parser — `scripts/audit/installer-command-parity.sh`"
    outcome: waived
    reason: "contrived: no framework/reference row exists outside Shared Files (owner decision, 2026-10-26)"
    decided-at: 2026-11-10T14:03:43Z
    decided-by: andy@stone.dev
  - key: "note: the check-artifacts skipped-worklist entry's pointer is approximate; no constitution section states its lead — `AGENTS.md`"
    outcome: waived
    reason: §spec-requirements is the nearest statement and the verbatim lead carries the rule; Family 38 matches by lead, not target
    decided-at: 2026-11-10T14:03:43Z
    decided-by: andy@stone.dev
  - key: "other: 066's task 19 items 2 and 4 prescribe old text that no longer occurs — `specs/066-audit-records-and-the-completion-gate/tasks.md`"
    outcome: waived
    reason: 066's own task list, outside 078's code; the stored deferrals that pointed there expired this run, their issues gone.
    decided-at: 2026-11-10T14:03:43Z
    decided-by: andy@stone.dev
  - key: "other: 063's plan justifies 058's step-number sync as mechanical because it changes no claim — `specs/063-oversized-artifacts-warn-with-a-fix/plan.md`"
    outcome: waived
    reason: It records a decision, and its conclusion holds under §spec-lifecycle's case (a).
    decided-at: 2026-11-10T14:03:43Z
    decided-by: andy@stone.dev
  - key: other — the family-22 scenario says the fixture is built twice (default and configured root) and lists four assertions; adopter-shell-behavior.sh also runs cases for a too-old runtime (1b), a hung runtime (1c), a staged spec deletion (3), a project in a repository subdirectory through the outer hook (4) and the size-limit refusal (5). The scenario understates the family; nothing it says is false, and AC15 states the same subset. — `specs/026-framework-self-audit/scenarios/family-22-adopter-shell-behavior.md:59`
    outcome: waived
    reason: "an understatement, not a false claim: every behavior the scenario states holds, the extra cases are documented in the script's own case headers, and listing them in the scenario would add requirements rather than correct one. It is also a claim about the pipeline's own machinery under review."
    decided-at: 2026-11-14T01:00:10Z
    decided-by: rule
  - key: other — done-spec-criteria.sh counts a done spec as examined when its Acceptance Criteria heading is absent or spelled differently (the match is exact and case-sensitive), so the stderr line says N specs examined while one may have contributed no criterion. The header states this is deliberate (section completeness is /ductus:analyze's), and all 40 done specs at HEAD carry the exact heading (checked with grep over the tracked spec.md files). — `scripts/audit/done-spec-criteria.sh:113`
    outcome: waived
    reason: the header documents the boundary with /ductus:analyze, framework/templates/spec/spec.md scaffolds the exact heading, and no done spec at HEAD lacks it; the stderr message quantifies specs examined rather than asserting every criterion was seen, which QUAL-CLAIM-001's Not flagged clause covers. It is also an observation about the pipeline's own audit machinery.
    decided-at: 2026-11-14T01:00:10Z
    decided-by: rule
  - key: other — the audit-script-refactors scenario's Context says flush_step 'reads and mutates seven caller-scope variables' and then lists five names plus `drift`, which is six. — `specs/026-framework-self-audit/scenarios/audit-script-refactors.md:12`
    outcome: waived
    reason: Context quoting a 2026-05-18 review finding about code that no longer exists (flush_step is gone, report_step is pure, which the Behavior records and the script confirms); the count cannot be re-derived against the pre-refactor source the finding read, and correcting it changes no requirement.
    decided-at: 2026-11-15T18:19:24Z
    decided-by: rule
  - key: other — the family-22 scenario's reworded sentence says the orchestration cases' stub 'records its invocation', while install_stub also simulates the dependency derivation (it rewrites the seeded spec on `derive-dependencies --write`), which assertions 3 and 4 depend on. The script's header states both. — `specs/026-framework-self-audit/scenarios/family-22-adopter-shell-behavior.md:66-69`
    outcome: waived
    reason: The sentence is accurate as far as it goes and its purpose is the contrast with the project-file cases, which drive the real runtime; the simulation is stated in adopter-shell-behavior.sh's header, and adding it to the scenario edits a requirement sentence for no behavioral difference.
    decided-at: 2026-11-15T18:19:24Z
    decided-by: rule
  - key: other — the family-36 scenario's 'Three findings, because three repairs differ' and AC24's enumeration list unresolved, blob-names-dir and tree-names-file, while the family now also reports `outside`, `file-outside` and `unreadable`; the two new kinds are stated in the next paragraph, not in that list. — `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md:27-34; specs/026-framework-self-audit/spec.md:148`
    outcome: waived
    reason: The three-finding list is the repairs for a path inside the repository and still holds as written; the new kinds are specified in the Behavior paragraph that follows, and AC24 says those three are distinct, not that they are the only kinds.
    decided-at: 2026-11-15T18:19:24Z
    decided-by: rule
  - key: other — three copies of the containment comparison now exist (lib.sh `contained`, 35b's inline python, 36's `inside`), and 35a starts one python3 process per Shared Files row (38 rows today, about a second). — `scripts/audit/lib.sh:78-93; scripts/audit/manifest-destination-links.sh:125,284; scripts/audit/self-url-resolution.sh:124-129`
    outcome: waived
    reason: "The copies are the documented trade-off (lib.sh's comment and the family scenarios): the python-computing families cannot call a shell function per path without a process each, and the comparison is two lines. The per-row cost is under two seconds in a maintainer-run script that also builds a runtime-backed tree."
    decided-at: 2026-11-15T18:19:24Z
    decided-by: rule
  - key: "other — in scripts/audit/adopter-shell-behavior.sh (outside 026's review scope, read as a check of the Family 22 claim) project_file_fixture links `\"$ROOT/$bin\"`, and `ductus_bin`'s third tier returns an absolute PATH result, giving `$ROOT//abs/path`, a broken link that surfaces as the 'no runtime reachable' finding. The claim edited by 55a96825 (a runtime they cannot reach is a finding) holds: the failure is loud, only its message is misleading. — `scripts/audit/adopter-shell-behavior.sh:503`"
    outcome: waived
    reason: Not an entry in 026's review scope, and the pipeline's own machinery observed while reviewing framework work, which AGENTS.md waives; the case needs a runtime found only on PATH, which this repository never has (the release build is tier 2).
    decided-at: 2026-11-15T18:19:24Z
    decided-by: rule
  - key: "other — reuse: the file-form read of a `lists-file` is now carried by four embedded-python copies (`scripts/audit/lib.sh` spec_corpus, `broken-relative-links.sh`, `manifest-destination-links.sh`, and `step-reference-integrity.sh`), each with its own key set and precondition wording. — `scripts/audit/step-reference-integrity.sh:116`"
    outcome: waived
    reason: Pipeline machinery. Each family's python is embedded in its own script by the README contract (families keep their own renderer, and an apostrophe-free `python3 -c` string cannot import a shared module); the copies differ in the keys they require and the message they name, and a shared helper would be a new feature, not a defect in 026's change. The four copies read the lists the same way (missing list halts, never reads as none), which is the property that matters.
    decided-at: 2026-11-15T19:09:11Z
    decided-by: rule
  - key: other — spec 080's data-model 'Bounded results' table says each member's row is added as it lands, and its research.md measurement table records check-step-references as Bounded 'no'; neither names the primitive now that it is bounded. prune-project (041/075) is equally absent from the table. — `specs/080-deterministic-runtime-efficiency/data-model.md:196`
    outcome: waived
    reason: "080's Bounded results table lists 080's own population, as the owner decided on 2026-11-14 when 067 task 32 bounded derive-references (that task's text: the table gains no row, since it lists 080's own population), so a bound another spec adds takes no row there; research.md's table is a measurement record at the commit it states, not a live claim"
    decided-at: 2026-11-15T19:09:11Z
    decided-by: rule
  - key: other — the lists file sits under `.ductus/results/check-step-references/all-specs/`; `all-specs` is the key 080's data model gives a corpus-wide spec result, while this primitive's subject is command sources and two bootstrap files, not the spec corpus. Mutant M10 and the unit test pin the segment. — `runtime/src/primitives/check_step_references.rs:145`
    outcome: waived
    reason: "a naming choice with no behavioral effect: the key only separates one primitive's list files, a caller reads the path from lists-file, and the whole-corpus key is how the other corpus-wide primitives key theirs"
    decided-at: 2026-11-15T19:09:11Z
    decided-by: rule
  - key: other — the family-34 scenario's Context says 'the 33 existing `/audit` families'; the framework has more families now. — `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md:19`
    outcome: waived
    reason: the sentence is the account of the 054 incident at the time Family 34 was introduced, when 33 families existed and Family 34 was the next; it is a dated historical account, not a claim about the present, as the scenario's other counts carry their dates
    decided-at: 2026-11-15T19:09:11Z
    decided-by: rule
  - key: other — specs_root in lib.sh reads only .ductus/config.toml and takes the first double-quoted specs-root line in the file whatever its table, while the runtime's Paths::load resolves the key from the newest existing tier of the three-tier CONFIG_CHAIN and parses TOML (a single-quoted literal string or a key outside [paths] would differ). Families 7, 8, 27 and 37 would then name a directory other than the runtime's; containment still holds because the built path is contained, but the comparison reads the wrong tree. — `scripts/audit/lib.sh:123`
    outcome: waived
    reason: the audit is a maintainer-only gate run in this repository, whose only config is .ductus/config.toml with no specs-root key (the default applies) and whose two retired tiers are absent; lib.sh's comment says the function reads and does not contain, and scripts/tests/test-audit-path-containment.sh exercises a configured root.
    decided-at: 2026-11-15T21:14:46Z
    decided-by: rule
  - key: "other — a glob is expanded before its matches are contained when the symbolic link sits inside the pattern: Family 10b with target_paths framework/*/x.md and framework/link pointing at an outside directory holding x.md reports 'matched framework/link/x.md, which resolves outside the repository and was not probed', yet the shell's expansion had already tested that outside path for existence; Family 17's '*/' glob over .claude/commands likewise tests an outward link for being a directory before admit reports it. Both reported the link and neither read or opened anything outside, as fixtures showed; the plan's wording is 'the pattern is contained before it is expanded and each match after', which both meet. — `scripts/audit/migration-coverage.sh:145`"
    outcome: waived
    reason: the plan decision contains the pattern before expansion and each match after, as both families do, and the expansion's existence test opens and reads nothing; the finding is reported either way. Closing it would replace the shell glob with a readdir walk, a new design the plan does not ask for.
    decided-at: 2026-11-15T21:14:46Z
    decided-by: rule
  - key: "other — reuse: the 'contained; case $? in 1) report and continue; 2) containment_unchecked and exit' block is written out nine times across the six scripts of this half (Family 10 four times, one each in 7, 8, 12 and 35a) and wrapped once as admit in Family 17, so a change to the three-way contract is nine edits plus admit; the simplicity pass found nothing else (no dead branch, no unneeded constant). — `scripts/audit/migration-coverage.sh:74`"
    outcome: waived
    reason: each site reports a different location, message and fix and continues a different loop, so a shared helper would take five arguments and exit the caller from inside a function, which admit does for one family; the plan decision does not require it, and the S case in scripts/tests/test-audit-path-containment.sh checks every site by itself.
    decided-at: 2026-11-15T21:14:46Z
    decided-by: rule
  - key: "other — Family 26 delegates to check-corpus-links, which tests a lexically inside link target with exists() on the unresolved path (check_corpus_links.rs, comment at 476: never canonicalized), so a link target that is a symbolic link out of the repository is probed by following it. The family hands the primitive no path, which the plan decision's primitive clause covers, and the lexical rule is deliberate so a corpus answers the same in two checkouts. Read in part (lines 470 to 500 only); not in this half's table. — `runtime/src/primitives/check_corpus_links.rs:484`"
    outcome: waived
    reason: the primitive is the runtime surface 067 owns, its lexical resolution is a stated design choice, and the plan decision scopes the rule to paths a family hands a primitive or receives from one
    decided-at: 2026-11-15T21:14:46Z
    decided-by: rule
---

# Review — 026-framework-self-audit

## Summary

026's third review of its 2026-11-15 reopening. The window runs from `368249a9`, 026's last review, to `a2de525e`; its one code commit is `a35b3d06` (task 7: every audit family contains the paths it did not write as a literal), with `33fc7340` and `87ee9f2e` to `tasks.md`. Two agents ran all five passes over halves of the scope, each reading all nine loaded backend and cross rule files whole, and checked each of the eight scenarios in scope claim by claim. They read 23 of the 31 scope entries whole: the plan, `tasks.md`, the eleven family scripts and `lib.sh` `a35b3d06` changed, the containment test and the eight scenarios. `scripts/audit/README.md` and `scripts/tests/test-deprecation-removal.sh` were read at the window's hunks and their sections; the other unread entries are the files only this run's fix round changed (Families 4, 9, 16 and 33, `scripts/tests/test-migration-coverage.sh` and the readme-command-parity scenario), which the round's re-read reads. Fixed in the run: both findings, `BE-INPUT-003` (Family 12 spliced a fixture path into `python3 -c` source; a crafted directory name ran code) and `QUAL-CLAIM-001` (Family 12 exited 0 in silence with its fixtures directory absent), each low confidence; two contract breaks, Families 4, 9, 16 and 33 opening glob-listed files uncontained, which task 7's ten-family inventory missed (an inventory of every registered family now finds no other), and Family 10's tab-delimited rows collapsing an empty `procedure_file`; and five chores: Family 7's awk probe emitting inside `$(...)`, Family 10b's empty array under `set -u`, the README's `lib.sh` bullet, the scenarios' pointer to the plan decision by its heading, and Family 24's per-file `python3`, which cost 78.7 s a run, now 5.35 s through `contained_each`. Four observations were waived by the rule, and four stored decisions still stand and are carried.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

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

## Observations

- security — Families 4 and 9 open files a shell glob lists, with no containment: placeholder-roundtrip.sh walks framework/commands/*.md and framework/reference/*.md and primitive-promotion-candidates.sh walks framework/commands/*.md, each testing the name with -f (which follows a symbolic link) and reading it. The plan decision says a path listed by git ls-files or a walk is contained before it is opened, and task 7's Done-when says no audit family opens a path it did not write as a literal without containing it first, yet its inventory names only ten families. Proven on a fixture under the scratchpad: a tracked symlink framework/commands/lnk.md resolving into a sibling directory made Family 4 report two findings at lnk.md:1 from the outside file's content, and Family 9 report the outside file's prose-only step at lnk.md:5; neither says the path resolves outside. installer-command-parity.sh (Family 16) greps the same glob's contents at line 200, read but not run; readme-command-parity.sh probes names only. BE-INPUT-004 maps to it, but these scripts are outside the review scope, so it is recorded here and not as a finding. — `scripts/audit/placeholder-roundtrip.sh:69; scripts/audit/primitive-promotion-candidates.sh:100; scripts/audit/installer-command-parity.sh:200` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug — known defect confirmed: Family 7's awk precondition emits inside extract_affected_files, which runs under $(...), so its finding is captured as data and its drift never reaches the script. Under /bin/bash 3.2 with an awk whose match() fails, in a fixture of two mutually linking non-done specs with one plan.md absent, the family printed nothing and exited 0; with both plans present it printed a bundling finding whose overlap text was the captured precondition line. This is QUAL-CLAIM-001's case (a clean result indistinguishable from could-not-examine), so the parent may promote it to a finding; it is filed as an observation because the brief asks for the two known defects as observations with their fix. — `scripts/audit/sibling-coupling.sh:121` — **fixed** (decided by the rule)
- bug — known defect confirmed: Family 10b expands an empty paths array under set -u, an unbound-variable error on bash 3.2 for a registry entry with no target_paths. Under /bin/bash 3.2.57 in a fixture registry holding one such entry the script aborts at line 171 with exit 1 and no finding line, so 10b for later entries and 10d never run. No entry in the real registry lacks the key today, so it is latent. — `scripts/audit/migration-coverage.sh:123` — **fixed** (decided by the rule)
- bug — Family 10 reads its python rows with IFS set to a tab, and a tab is IFS whitespace, so consecutive tabs collapse: a registry entry with no procedure_file but with target_paths has its pipe-joined target_paths read as procedure_file, tps is empty, and 10b never examines that entry. The finding then reads 'missing procedure_file: framework/gone.md|framework/there.md', where family-10-migration-coverage.md's edge case says the entry is treated as procedure_file = "". Confirmed under /bin/bash 3.2 in a fixture: an existing framework/there.md listed in target_paths was not reported as claimed-removed-but-present. — `scripts/audit/migration-coverage.sh:64` (breaks the spec's contract) — **fixed** (decided by the rule)
- other — scripts/audit/README.md's lib.sh bullet lists the helpers as 'repo root, drift, emit, contained, specs_root', which leaves out containment_unchecked, added by the same commit, and four older helpers (ductus_bin, spec_corpus, agent_config_dirs, maintainer_only_commands); the parenthetical is an enumeration that drifts. Each name checked against lib.sh. — `scripts/audit/README.md:40` — **fixed** (decided by the rule)
- other — specs_root in lib.sh reads only .ductus/config.toml and takes the first double-quoted specs-root line in the file whatever its table, while the runtime's Paths::load resolves the key from the newest existing tier of the three-tier CONFIG_CHAIN and parses TOML (a single-quoted literal string or a key outside [paths] would differ). Families 7, 8, 27 and 37 would then name a directory other than the runtime's; containment still holds because the built path is contained, but the comparison reads the wrong tree. — `scripts/audit/lib.sh:123` — **waived**: the audit is a maintainer-only gate run in this repository, whose only config is .ductus/config.toml with no specs-root key (the default applies) and whose two retired tiers are absent; lib.sh's comment says the function reads and does not contain, and scripts/tests/test-audit-path-containment.sh exercises a configured root. (decided by the rule)
- other — a glob is expanded before its matches are contained when the symbolic link sits inside the pattern: Family 10b with target_paths framework/*/x.md and framework/link pointing at an outside directory holding x.md reports 'matched framework/link/x.md, which resolves outside the repository and was not probed', yet the shell's expansion had already tested that outside path for existence; Family 17's '*/' glob over .claude/commands likewise tests an outward link for being a directory before admit reports it. Both reported the link and neither read or opened anything outside, as fixtures showed; the plan's wording is 'the pattern is contained before it is expanded and each match after', which both meet. — `scripts/audit/migration-coverage.sh:145` — **waived**: the plan decision contains the pattern before expansion and each match after, as both families do, and the expansion's existence test opens and reads nothing; the finding is reported either way. Closing it would replace the shell glob with a readdir walk, a new design the plan does not ask for. (decided by the rule)
- other — reuse: the 'contained; case $? in 1) report and continue; 2) containment_unchecked and exit' block is written out nine times across the six scripts of this half (Family 10 four times, one each in 7, 8, 12 and 35a) and wrapped once as admit in Family 17, so a change to the three-way contract is nine edits plus admit; the simplicity pass found nothing else (no dead branch, no unneeded constant). — `scripts/audit/migration-coverage.sh:74` — **waived**: each site reports a different location, message and fix and continues a different loop, so a shared helper would take five arguments and exit the caller from inside a function, which admit does for one family; the plan decision does not require it, and the S case in scripts/tests/test-audit-path-containment.sh checks every site by itself. (decided by the rule)
- perf — rename-sweep-residue.sh starts one python3 per tracked markdown file through contained: 868 files, 77 ms each measured over 200 files (15.4 s, wall), so about 67 s of added wall time here per run of the family, against the 35a figure of about a second that the stored decision on the copies weighed. The plan allows either form, contained or the inline comparison in one python over the whole list, as Families 27, 36, 37 and 39 do; no performance rule loaded here reaches a maintainer-run audit script, and the S case's stub fails the contained call alone, so a batched form would also change that test. — `scripts/audit/rename-sweep-residue.sh:128` — **fixed** (decided by the rule)
- other — Family 26 delegates to check-corpus-links, which tests a lexically inside link target with exists() on the unresolved path (check_corpus_links.rs, comment at 476: never canonicalized), so a link target that is a symbolic link out of the repository is probed by following it. The family hands the primitive no path, which the plan decision's primitive clause covers, and the lexical rule is deliberate so a corpus answers the same in two checkouts. Read in part (lines 470 to 500 only); not in this half's table. — `runtime/src/primitives/check_corpus_links.rs:484` — **waived**: the primitive is the runtime surface 067 owns, its lexical resolution is a stated design choice, and the plan decision scopes the rule to paths a family hands a primitive or receives from one (decided by the rule)
- convention — eight scenarios and the test header point at the plan's path-containment decision, but the plan has no heading of that name: its section is titled Every family contains the paths it did not write as a literal, so a reader following the pointer by name searches for words the plan does not carry. — `specs/026-framework-self-audit/scenarios/family-24-rename-sweep-residue.md:36` — **fixed** (decided by the rule)
- other — done-spec-criteria.sh counts a done spec as examined when its Acceptance Criteria heading is absent or spelled differently (the match is exact and case-sensitive), so the stderr line says N specs examined while one may have contributed no criterion. The header states this is deliberate (section completeness is /ductus:analyze's), and all 40 done specs at HEAD carry the exact heading (checked with grep over the tracked spec.md files). — `scripts/audit/done-spec-criteria.sh:113` — **waived**: the header documents the boundary with /ductus:analyze, framework/templates/spec/spec.md scaffolds the exact heading, and no done spec at HEAD lacks it; the stderr message quantifies specs examined rather than asserting every criterion was seen, which QUAL-CLAIM-001's Not flagged clause covers. It is also an observation about the pipeline's own audit machinery. (decided by the rule)
- other — the family-36 scenario's 'Three findings, because three repairs differ' and AC24's enumeration list unresolved, blob-names-dir and tree-names-file, while the family now also reports `outside`, `file-outside` and `unreadable`; the two new kinds are stated in the next paragraph, not in that list. — `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md:27-34; specs/026-framework-self-audit/spec.md:148` — **waived**: The three-finding list is the repairs for a path inside the repository and still holds as written; the new kinds are specified in the Behavior paragraph that follows, and AC24 says those three are distinct, not that they are the only kinds. (decided by the rule)
- other — three copies of the containment comparison now exist (lib.sh `contained`, 35b's inline python, 36's `inside`), and 35a starts one python3 process per Shared Files row (38 rows today, about a second). — `scripts/audit/lib.sh:78-93; scripts/audit/manifest-destination-links.sh:125,284; scripts/audit/self-url-resolution.sh:124-129` — **waived**: The copies are the documented trade-off (lib.sh's comment and the family scenarios): the python-computing families cannot call a shell function per path without a process each, and the comparison is two lines. The per-row cost is under two seconds in a maintainer-run script that also builds a runtime-backed tree. (decided by the rule)
- other — reuse: the file-form read of a `lists-file` is now carried by four embedded-python copies (`scripts/audit/lib.sh` spec_corpus, `broken-relative-links.sh`, `manifest-destination-links.sh`, and `step-reference-integrity.sh`), each with its own key set and precondition wording. — `scripts/audit/step-reference-integrity.sh:116` — **waived**: Pipeline machinery. Each family's python is embedded in its own script by the README contract (families keep their own renderer, and an apostrophe-free `python3 -c` string cannot import a shared module); the copies differ in the keys they require and the message they name, and a shared helper would be a new feature, not a defect in 026's change. The four copies read the lists the same way (missing list halts, never reads as none), which is the property that matters. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md` — 067-analyze-and-audit-checks
- `.claude/commands/ductus/prune.md` — 041-task-pruning
- `.ductus/reference/analyze.md` — 067-analyze-and-audit-checks
- `.ductus/reference/audit.md` — 067-analyze-and-audit-checks
- `.ductus/reference/prune.md` — 041-task-pruning
- `.githooks/pre-commit` — 067-analyze-and-audit-checks
- `.github/workflows/framework-checks.yml` — 067-analyze-and-audit-checks
- `.pi/prompts/ductus-analyze.md` — 067-analyze-and-audit-checks
- `.pi/prompts/ductus-prune.md` — 041-task-pruning
- `framework/commands/analyze.md` — 067-analyze-and-audit-checks
- `framework/commands/prune.md` — 041-task-pruning
- `framework/reference/analyze.md` — 067-analyze-and-audit-checks
- `framework/reference/audit.md` — 067-analyze-and-audit-checks
- `framework/reference/prune.md` — 041-task-pruning
- `runtime/CHANGELOG.md` — 041-task-pruning, 067-analyze-and-audit-checks
- `runtime/src/interpreter/payload.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/append_task.rs` — 041-task-pruning
- `runtime/src/primitives/bounded_lists.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/check_artifacts.rs` — 041-task-pruning, 067-analyze-and-audit-checks
- `runtime/src/primitives/check_corpus_links.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/check_departures.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/label_criteria.rs` — 041-task-pruning
- `runtime/src/primitives/mark_task.rs` — 041-task-pruning
- `runtime/src/primitives/mod.rs` — 041-task-pruning, 067-analyze-and-audit-checks
- `runtime/src/primitives/prune_plan.rs` — 041-task-pruning
- `runtime/src/primitives/prune_project.rs` — 041-task-pruning
- `runtime/src/primitives/prune_tasks.rs` — 041-task-pruning
- `runtime/src/primitives/read_deprecation_labels.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/read_section.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/resolve_anchor.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/tasks_counter.rs` — 041-task-pruning
- `runtime/src/schema/descriptions.rs` — 041-task-pruning, 067-analyze-and-audit-checks
- `runtime/src/schema/primitives.rs` — 041-task-pruning, 067-analyze-and-audit-checks
- `runtime/tests/mcp.rs` — 041-task-pruning
- `scripts/audit/feature-dir-grammar-parity.sh` — 067-analyze-and-audit-checks
- `scripts/audit/run-all.sh` — 067-analyze-and-audit-checks
- `scripts/lint-frontmatter.sh` — 067-analyze-and-audit-checks
- `scripts/mutation/anchor-and-label-results-bounded.py` — 067-analyze-and-audit-checks
- `scripts/mutation/corpus-link-containment.py` — 067-analyze-and-audit-checks
- `scripts/mutation/criterion-body-path-links.py` — 067-analyze-and-audit-checks
- `scripts/mutation/prune-plan-bounded.py` — 041-task-pruning
- `scripts/mutation/prune-review-fixes-041.py` — 041-task-pruning
- `scripts/mutation/pruned-task-number.py` — 041-task-pruning
- `scripts/mutation/suffixed-task-number.py` — 041-task-pruning
- `scripts/tests/test-anchor-and-label-families.sh` — 067-analyze-and-audit-checks
- `scripts/tests/test-feature-dir-grammar-parity.sh` — 067-analyze-and-audit-checks
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime
- `specs/041-task-pruning/data-model.md` — 041-task-pruning
- `specs/041-task-pruning/plan.md` — 041-task-pruning
- `specs/041-task-pruning/review.md` — 041-task-pruning
- `specs/041-task-pruning/scenarios/a-pruned-task-number-is-never-reissued.md` — 041-task-pruning
- `specs/041-task-pruning/scenarios/a-suffixed-task-number-is-a-task.md` — 041-task-pruning
- `specs/041-task-pruning/scenarios/prune-reduces-the-spec-directory.md` — 041-task-pruning
- `specs/041-task-pruning/scenarios/prune-walks-the-project-files.md` — 041-task-pruning
- `specs/041-task-pruning/spec.md` — 041-task-pruning
- `specs/041-task-pruning/tasks.md` — 041-task-pruning
- `specs/067-analyze-and-audit-checks/data-model.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/plan.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks

## Unexamined governance

*None.*
