---
spec: 026-framework-self-audit
last-run: 2026-11-15T19:09:11Z
reviewed-against: 5e00b87bd1c819d383df5480614edb9b7fc843d5
diff-base: 713c1ca7f9a8c8c59a840e4c1a43ad05b77081f4
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 11
unread:
  - .github/workflows/framework-checks.yml
  - runtime/CHANGELOG.md
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
excluded: 8
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
  scenarios/family-10-migration-coverage.md: d2071390e33f8598daed4a07d0e08a52d8569f25b69fa4bb260c85ae77b3d515
  scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
  scenarios/family-18-marker-list-parity.md: 296f55313954ec984b5260945b92639d15b8458f707f90e94e0bd1bbc0f052c0
  scenarios/family-22-adopter-shell-behavior.md: 995c7c45e1918732b886e1174e5b627161bdd3c42ea824a4cbce7b04b70e02ff
  scenarios/family-23-sweep-target-manifest-parity.md: fabc89386b8c36156f41c09b5ae28e5f0cd3130c08abc284fa90dcd37507ebb1
  scenarios/family-24-rename-sweep-residue.md: 41d6c829f0851a3caab11650d24b7100992667969ee3f21454141b482819e132
  scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
  scenarios/family-26-broken-relative-links.md: 9e03cfe250dbffe6128229b1bb064c8b574b61ead93ed1ce5fd2fe12ec8e615d
  scenarios/family-27-done-spec-unchecked-criteria.md: d164ba1fedaababcb88f5a4062b150ae8bd7e017699d7ac6d310292a25489bf9
  scenarios/family-28-audit-family-registry-parity.md: bf80f1d64718deb87a7dec79c69e9248e8c1ab4a41e65c638488fdd9fcd5e529
  scenarios/family-34-step-reference-integrity.md: 4494835543a7ebfa3fe429d732652a658faf6f0b0cc8376daa05cb4e09ad2ca0
  scenarios/family-35-manifest-destination-links.md: d51539ef0b2834b712658784442a2f90f574cf667510b5a771c65b6b60d646cc
  scenarios/family-36-self-url-resolution.md: cd99f03527a227ce027dff7026bea461c1d0dd5d054422687e3076f44cefa3fe
  scenarios/family-39-shipped-anchor-resolution.md: c00188dfd875ce2aeda794be6e023221173dd712c949b58f60ac34a80a5e17ca
  scenarios/family-40-deprecated-rules-are-removed-on-time.md: b0f97a19013851cdabf3172954b6c0a8da12ccc06a46f0183c93bbf50f0227bc
  scenarios/host-namespace-parity.md: 5d20df476bcd94b0f843e3c44919b19215f5dcc9ddba18af435f2b46e5b194ed
  scenarios/link-check-consolidation.md: edaa15335c49241c5565a0b4063a3f59a5233b5b4a0c6362cbe9f4356b82812f
  scenarios/readme-command-parity.md: a628c66cde7ce274281f77151f3a6a485c5fd3563b74074bed8d8ba25ff59b75
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 9
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
---

# Review — 026-framework-self-audit

## Summary

026's second review of its 2026-11-15 reopening. The window runs from `713c1ca7`, its first review, to `5e00b87b`. One agent ran all five passes, reading all nine loaded backend and cross rule files whole. It read 7 of the 11 scope entries whole: `check_step_references.rs`, Family 34's script, its mutation list and shell test, 026's plan and tasks, and the family-34 scenario, checked claim by claim. It reviewed `5e00b87b` (026 task 6, check-step-references' result bounded), `851d0f17` (the plan's analysis chores) and `4f093e4e` (task 6 recorded) hunk by hunk. Unread: `.github/workflows/framework-checks.yml`, `runtime/CHANGELOG.md`, `runtime/src/schema/descriptions.rs`, `runtime/src/schema/primitives.rs`, each read at the task-6 hunk and what it needed; `framework-checks.yml`'s lines 60-110 and 140-195 were not read. No finding. Three chores were fixed in the run: `step_references`' dead `bytes` binding, the plan's claim that 026 adds no primitive (Family 34's check is one, since it parses markdown structure), and the CHANGELOG entry now says an empty list is `[]` inline. Four observations were waived by the rule, and five stored waivers still stand and are carried.

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

- other — dead code in `step_references`: `let bytes = lower.as_bytes();` is never read, and `let _ = bytes;` exists only to silence the unused binding (from 8edc39c4, outside the window's hunks but inside the file in scope). AGENTS.md: no dead code, fixed in the run. — `runtime/src/primitives/check_step_references.rs:367` — **fixed** (decided by the rule)
- other — plan.md still says no primitive is added by 026 (Overview: 'v1 uses shell-first orchestration ... no new primitives are added in 026'; Trade-offs: add /audit-specific runtime primitives 'Rejected'), but Family 34's check is the `check-step-references` primitive, added by 026 in 8edc39c4 and the subject of task 6. The plan is meant to record the design as it stands; Family 9's reversal is recorded in the same file, this one is not. — `specs/026-framework-self-audit/plan.md:11` — **fixed** (decided by the rule)
- other — the CHANGELOG entry for task 6 states the file form but not that an inline empty list now serializes as `[]` where it was absent (`not-a-procedure`, `skipped`, `findings` on a clean tree), a shape change a consumer of the MCP or CLI result can see. The commit message records it; the CHANGELOG, which adopters read, does not. — `runtime/CHANGELOG.md:327` — **fixed** (decided by the rule)
- other — reuse: the file-form read of a `lists-file` is now carried by four embedded-python copies (`scripts/audit/lib.sh` spec_corpus, `broken-relative-links.sh`, `manifest-destination-links.sh`, and `step-reference-integrity.sh`), each with its own key set and precondition wording. — `scripts/audit/step-reference-integrity.sh:116` — **waived**: Pipeline machinery. Each family's python is embedded in its own script by the README contract (families keep their own renderer, and an apostrophe-free `python3 -c` string cannot import a shared module); the copies differ in the keys they require and the message they name, and a shared helper would be a new feature, not a defect in 026's change. The four copies read the lists the same way (missing list halts, never reads as none), which is the property that matters. (decided by the rule)
- other — spec 080's data-model 'Bounded results' table says each member's row is added as it lands, and its research.md measurement table records check-step-references as Bounded 'no'; neither names the primitive now that it is bounded. prune-project (041/075) is equally absent from the table. — `specs/080-deterministic-runtime-efficiency/data-model.md:196` — **waived**: 080's Bounded results table lists 080's own population, as the owner decided on 2026-11-14 when 067 task 32 bounded derive-references (that task's text: the table gains no row, since it lists 080's own population), so a bound another spec adds takes no row there; research.md's table is a measurement record at the commit it states, not a live claim (decided by the rule)
- other — the lists file sits under `.ductus/results/check-step-references/all-specs/`; `all-specs` is the key 080's data model gives a corpus-wide spec result, while this primitive's subject is command sources and two bootstrap files, not the spec corpus. Mutant M10 and the unit test pin the segment. — `runtime/src/primitives/check_step_references.rs:145` — **waived**: a naming choice with no behavioral effect: the key only separates one primitive's list files, a caller reads the path from lists-file, and the whole-corpus key is how the other corpus-wide primitives key theirs (decided by the rule)
- other — the family-34 scenario's Context says 'the 33 existing `/audit` families'; the framework has more families now. — `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md:19` — **waived**: the sentence is the account of the 054 incident at the time Family 34 was introduced, when 33 families existed and Family 34 was the next; it is a dated historical account, not a claim about the present, as the scenario's other counts carry their dates (decided by the rule)
- other: check-step-references' not-a-procedure field doc defines the set by files holding several numbered lists, though it also holds files with no step or too few — `runtime/src/schema/primitives.rs` — **waived**: the primitive's schema doc, 067's surface and outside 026's code; the family's own stderr line now says not one procedure (decided by the rule)
- other: framework-checks step (m) tests the pins check on fixtures only, so a pull request adding a tag-pinned uses: merges green until the weekly run reports it — `.github/workflows/framework-checks.yml` — **waived**: The owner chose the weekly currency check as the pins' update process (022 task 3, option c); a per-pull-request pin lint is not part of it and would be its own decision.
- other: runs-on: ubuntu-latest is mutable and now supplies framework-checks' Node too — `.github/workflows/framework-checks.yml` — **waived**: The owner chose the runner image's Node (022 task 3, option a), and a GitHub-hosted image has no immutable reference to pin.
- other — the family-36 scenario's 'Three findings, because three repairs differ' and AC24's enumeration list unresolved, blob-names-dir and tree-names-file, while the family now also reports `outside`, `file-outside` and `unreadable`; the two new kinds are stated in the next paragraph, not in that list. — `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md:27-34; specs/026-framework-self-audit/spec.md:148` — **waived**: The three-finding list is the repairs for a path inside the repository and still holds as written; the new kinds are specified in the Behavior paragraph that follows, and AC24 says those three are distinct, not that they are the only kinds. (decided by the rule)
- other — three copies of the containment comparison now exist (lib.sh `contained`, 35b's inline python, 36's `inside`), and 35a starts one python3 process per Shared Files row (38 rows today, about a second). — `scripts/audit/lib.sh:78-93; scripts/audit/manifest-destination-links.sh:125,284; scripts/audit/self-url-resolution.sh:124-129` — **waived**: The copies are the documented trade-off (lib.sh's comment and the family scenarios): the python-computing families cannot call a shell function per path without a process each, and the comparison is two lines. The per-row cost is under two seconds in a maintainer-run script that also builds a runtime-backed tree. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/prune.md` — 041-task-pruning
- `.pi/prompts/ductus-prune.md` — 041-task-pruning
- `framework/commands/prune.md` — 041-task-pruning
- `runtime/src/primitives/prune_project.rs` — 041-task-pruning
- `scripts/mutation/prune-project-containment.py` — 041-task-pruning
- `specs/041-task-pruning/data-model.md` — 041-task-pruning
- `specs/041-task-pruning/tasks.md` — 041-task-pruning
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks

## Unexamined governance

*None.*
