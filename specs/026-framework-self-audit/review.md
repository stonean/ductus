---
spec: 026-framework-self-audit
last-run: 2026-11-19T13:37:30Z
reviewed-against: 9b03441041e44897e412358a43ef1c5edf4feca1
diff-base: 6bb984b0e9cfb642138181fa11a45dfa638809ae
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 7
scope: 7
unread: []
excluded: 125
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
  scenarios/families-follow-the-command-reference-split.md: 12e2262fecdcf34d018c78f9dfe65fcebc769ba9eabfca3859c172163da4b073
  scenarios/family-10-migration-coverage.md: a535ac8c1e4dce26084c03066fc88978f0668f05e15f3600d87dbb6bf85ecf9a
  scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
  scenarios/family-18-marker-list-parity.md: 296f55313954ec984b5260945b92639d15b8458f707f90e94e0bd1bbc0f052c0
  scenarios/family-22-adopter-shell-behavior.md: 995c7c45e1918732b886e1174e5b627161bdd3c42ea824a4cbce7b04b70e02ff
  scenarios/family-23-sweep-target-manifest-parity.md: fabc89386b8c36156f41c09b5ae28e5f0cd3130c08abc284fa90dcd37507ebb1
  scenarios/family-24-rename-sweep-residue.md: 3b70ff8b57ef5bbd117ce0bd29752dd15a481d07e28a3fc3505671a2e6507cea
  scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
  scenarios/family-26-broken-relative-links.md: 9e03cfe250dbffe6128229b1bb064c8b574b61ead93ed1ce5fd2fe12ec8e615d
  scenarios/family-27-done-spec-unchecked-criteria.md: c7c66e810ae27435c40bb372d7295b101f480e897f91a648db3d0902cacf57b1
  scenarios/family-28-audit-family-registry-parity.md: bf80f1d64718deb87a7dec79c69e9248e8c1ab4a41e65c638488fdd9fcd5e529
  scenarios/family-34-step-reference-integrity.md: 8598c2bd4072742f934254d0ea46094ec1388a6d0206defa6a123c0553e29015
  scenarios/family-35-manifest-destination-links.md: 28297923f8f5f4ec6833696028c4076e247cfadb3d945b0a2446acc18237cc11
  scenarios/family-36-self-url-resolution.md: e4c14882f66a90cee309b539ffe60ecfeef564eb7ad024ab39a537314c0e051f
  scenarios/family-39-shipped-anchor-resolution.md: c25191adf85b75351517d55c89a55a24a5d083e325f894f680c04ace54401e8f
  scenarios/family-40-deprecated-rules-are-removed-on-time.md: 689441c4d67ea3b73b147f0bf319db6ef7a50034ea46218ef67799dee0f5732e
  scenarios/host-namespace-parity.md: e8a67c95e2e596cf9a231bbfa26545c4beee96af1a043fd83608899e16087790
  scenarios/link-check-consolidation.md: edaa15335c49241c5565a0b4063a3f59a5233b5b4a0c6362cbe9f4356b82812f
  scenarios/readme-command-parity.md: 899fdc8ebb306541496892772ec4540b4be968aed55ee366c87185cdcc5304b6
criteria-judged: 26
criteria-met: 26
criteria-verdicts:
  - label: AC1
    digest: 23a70438cc888f9419c64cb161b525b850a3763738b717de3b69ae2e86c4901a
  - label: AC2
    digest: f2cefc7df4e5169479fe13dc59595d48375c12ef4b3bfdc9f21aba64689b3866
  - label: AC3
    digest: 6bbf992e08f8eed22c946070c1375499cd972f0a336eb40f88efa3a9dd555860
  - label: AC4
    digest: e54b44506e4d304c9e115dec1f71bd201bf56628be1e199d92178bee72152c33
  - label: AC5
    digest: fe8ed07e8baf4de6a888952c8312d6c949894c00103d718415c983ee6f925014
  - label: AC6
    digest: 935396562e7cd2b13d16b9af7c3ebc4a70daadf612524275ed8047cf8741704b
  - label: AC7
    digest: 2b40506bf4c7a1e61caeac28936a56d3ff1170740513d311649d6d5138789691
  - label: AC8
    digest: 070aa01103320afd4caef4265b4a425941e21ae723655b0d2b2aa962c8322a75
  - label: AC9
    digest: ee333e2c4000bd6acde70f8b67f9c9a43452c6cdaaed01fde8b1c0ed63817fb3
  - label: AC10
    digest: fdc3543bbee248df59cd9c2286973fa3ad7c0dc02f585062fa291c391f1f08ea
  - label: AC11
    digest: 0624640d054c2a6f7276e2a1ed8d2fd1664748eb6faca271646adeb1b071a69f
  - label: AC12
    digest: 97222287dedd4b94f0a0a54a644ebe2a773011381f78e2b2ec052728bdeef518
  - label: AC13
    digest: 3996dbc9e9be66f7390e2bb30cc906be83c35bafc222c9463e49f12b73d84a08
  - label: AC14
    digest: 07341bd8d9c3ca90ead816b7b021b03553a11a143cc26dd8c9c8406a95be05d3
  - label: AC15
    digest: a2d808cb3fb63f2e23e74db34f0a858830b0ee1cc1d26472113600af2292196a
  - label: AC16
    digest: 8a53ed82a33f4a2b122185d941a00fe2d3fb42efd2d60104675c91915c7e75a5
  - label: AC17
    digest: e26530fb0ac1a180ced886c71804f108552532ab62dd1e067e9632abbefee687
  - label: AC18
    digest: 3fa7b999f4ed147a8297a1817fd2a0cad9d2dd6e3313197a4b9e825762946a13
  - label: AC19
    digest: 1f6a8af802e79ef7599ba13e961737637aa8798c6c63a06f40404a4f312e30d8
  - label: AC20
    digest: 668e33f01b24da0cbe5ce6efbeccbabfe334b446f1492cacaad8eb11687a648d
  - label: AC21
    digest: c5dd76b3a932dbd1c76a03932a1199e3a58b4c723435ba231a1bae8e53ebe174
  - label: AC22
    digest: dd7672c7cb7cb4cc783533664a262f9738e9eb712e8ab2417631da9cd84d38de
  - label: AC23
    digest: 6ff370c560786a715104db58e38f1f3caadab9215cf0c51ed7fb5d963895e8d2
  - label: AC24
    digest: 9c227418a4f54100f44ec7f2420c1fdc2af7e00dfcfeff26b60e9966b366357a
  - label: AC27
    digest: 36d75f0dc844e60a17590fd35479299a7d7d62a78ed2638164afda6f66e456f6
  - label: AC26
    digest: ca43c4c6993e68af79d14e1794554c8bf76e2ff79e53d390f5cbeabd0dfa7d7d
blocking: false
dispositions:
  fixed: 3
  deferred: 0
  waived: 4
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
  - key: quality — `scripts/audit/primitive-promotion-candidates.sh` expands `${legacy_files[@]}` under `set -u`, an unbound-variable error on bash 3.2 when `runtime/legacy-prose-commands.txt` lists no file, so an empty allowlist makes Family 9 exit 1 with no finding — `scripts/audit/primitive-promotion-candidates.sh`
    outcome: waived
    reason: "An observation about the pipeline's own machinery, waived with its reason (AGENTS.md): it is latent, since the shipped allowlist holds four entries, and an empty one fails loudly with exit 1 rather than passing, so no check reads clean that did not run."
    decided-at: 2026-11-19T13:36:29Z
    decided-by: rule
---

# Review — 026-framework-self-audit

## Summary

Window `6bb984b0` to `9b034410`, base-source `reopen`: `26b65754` reopened 026 (owner decision: the Family 10 scenario points at 027's list of its checks), and its analysis then edited the spec and plan. A review is owed because the last record predates the criteria step (066 task 27), so every one of the 26 criteria was judged. Scope: 7 entries, all spec artifacts and no code. Examined 7: each was read whole by the security pass and by the reuse, quality, efficiency and simplicity passes, against all 12 loaded rule files, read whole. Both pass parts first read some rule files in part, were asked to finish them, and changed no verdict. The passes found no rule finding. Observations: three stale scenario claims were fixed by the rule (the reference-split Edge Case on Family 39, the family-36 scenario's undated 32, and its missing ellipsis exclusion). One about Family 9's empty-allowlist expansion was waived as the pipeline's own machinery. Three stored waivers whose issues still stand are carried. Criteria step: 26 of 26 met, judged against `scripts/audit/`, `framework/commands/audit.md`, `framework/reference/audit.md` and the workflows. AC1 to AC13 were each driven on a fixture or the tree. AC14 to AC27 were judged by reading each family's code and running each family on the tree, since that part's fixture commands were refused. Several were judged on their Superseded or Never-held annotations. Unread: the AC14 to AC27 part read the spec's criteria whole but not its Behavior prose, `plan.md` or the scenarios, and the criteria parts read some family scripts in part, as their notes say. Fix rounds: one, the two scenario edits, re-read clean by an independent agent; the next round was empty.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

26 of 26 acceptance criteria judged met.

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

- AC1 — met: `framework/commands/audit.md` exists and its Instructions step 1 runs `scripts/audit/run-all.sh` with no session target; the mirror `.claude/commands/ductus/audit.md` is tracked and differs from the source only by the host-added ToolSearch note.
- AC2 — met: Judged on its annotation: `lib.sh` `emit` renders `FAMILY | LOCATION | MESSAGE | SUGGESTED-FIX` with no severity (4-column `emit` already at `1f92d4cc`, never true at delivery), and I drove Families 1, 2, 4, 5, 7, 8, 9 to a finding of that shape; Family 6 `ssot-invariants.sh` has no `emit`, as the annotation records (never true at delivery, `1f92d4cc`).
- AC3 — met: Judged on its annotation: `scripts/gen-readme-table.sh` is gone (deleted in `9847647b`) and `scripts/audit/cross-doc-consistency.sh` carries only 1b (state sequence, guarded against the constitution's diagram) and 1c (back-edge presence, guarded against its `Backward via` bullets); it exits 0 on this tree.
- AC4 — met: Judged on its annotation: `scripts/audit/manifest-parity.sh` checks the markers, stray grants and the empty extraction of the generated block in `framework/bootstrap/configure/claude.md`, prints the examined count (73) to stderr, and its header records the installer half as retired (never built at `1f92d4cc`; `framework/commands/init.md` has no source); `b1d319f1` is the later narrowing.
- AC5 — met: Retired record, judged as history: `framework/workflows/` is absent (spec 043, `531e3eab`) and `scripts/audit/run-all.sh` registers no Family 3.
- AC6 — met: `scripts/audit/placeholder-roundtrip.sh` driven on a fixture: it flags `.claude/` and `/ductus:` in a command file, and in a reference file `{cli-config-dir}`, `$ARGUMENTS`, `.pi/` and `/ductus:`; an empty `agent_config_dirs` derivation is a finding; the real tree exits 0.
- AC7 — met: Judged on its annotation: `scripts/audit/template-alignment.sh` holds a fixed invariant list (hardcoded since `1f92d4cc`), now covering plan-to-spec, tasks-to-plan and Done-when, with `guard_judged_set` comparing `judged_blocking_sections` to the blocking headings of `framework/reference/analyze.md` both ways; exits 0.
- AC8 — met: Judged on its annotation: `scripts/audit/ssot-invariants.sh` is a stub (stub commit `b2695ab3`, never detected anything, so never true at delivery) and I confirmed it writes `ssot-invariants: skipped - detection not implemented (026 AC8)` to stderr, which a `run-all.sh` run passes through, then exits 0.
- AC9 — met: `scripts/audit/sibling-coupling.sh` driven on a fixture: two non-done specs linking each other with a shared Affected Files row gave one finding naming both specs and the row (exit 1); the `Why split from a:` entry in the second spec suppressed it (exit 0).
- AC10 — met: Judged on its annotation: `scripts/audit/introducing-drift.sh` has a hardcoded `RENAMED_TOKENS` list (hardcoded at `1f92d4cc`) and flags a backticked old token outside a blockquote in a done spec, naming `file:line` and a rewrite; driven on a fixture, the blockquote line was skipped.
- AC11 — met: `scripts/audit/primitive-promotion-candidates.sh` driven on a fixture: a prose-only step was flagged, while steps carrying a primitive, an `<!-- llm:* -->` marker or `<!-- audit:ignore-promotion -->` were not; a file listed in `runtime/legacy-prose-commands.txt` is skipped (six entries at `1f92d4cc`, four now).
- AC12 — met: `scripts/audit/run-all.sh` ends `exit "$drift"`; on a stubbed copy it exited 0 clean and 1 with one family finding, and the real tree exits 0; `.github/workflows/framework-checks.yml` and `.github/workflows/runtime-release.yml` gate on it.
- AC13 — met: Judged on its annotation: `run-all.sh` prints a `=== label ===` header and the `FAMILY | LOCATION | MESSAGE | SUGGESTED-FIX` rows only for a family with findings (same at `1f92d4cc`), writes only stdout and no `audit.md`; confirmed on a stubbed copy.
- AC14 — met: `framework/commands/audit.md` §Boundary with `/ductus:analyze` holds the owner table and the rule of thumb, and the file says maintainer-only, never invoked by adopters; documentation claim read directly, no test applies.
- AC15 — met: `scripts/audit/adopter-shell-behavior.sh` builds fixtures at `specs` and `features` roots via `build_and_run`, config only at `.ductus/config.toml`, runtime only via `.ductus/bin/ductus` under `PATH=/usr/bin:/bin`, fixture failures go through `fixture_failed` to `emit`; the script ran clean (rc 0, 65s) and the superseded `.ductus/scripts/**` half is annotated and matches its header.
- AC16 — met: `scripts/audit/sweep-target-manifest-parity.sh` extracts between the `audit:sweep-targets` markers, compares manifest to list only, emits on missing markers and on empty extraction of either side, and reports counts and direction on stderr (ran: 13 entries, 38 paths).
- AC17 — met: `scripts/audit/rename-sweep-residue.sh` holds `MODALS`, `FOLLOWERS` and `POSSESSIVES` (three closed classes, `my` and `the`/`to`/`that` excluded), emits on zero tracked files and reports the examined count (ran: 871 files, rc 0).
- AC18 — met: `scripts/audit/unbalanced-inline-markup.sh` awk-counts odd backticks and `**` per line outside fences in `AGENTS.md` and `framework/templates/project/agents.md`, reports wrapped bullets, missing targets, and the two-file scope on stderr (ran: 2 of 2, rc 0).
- AC19 — met: `scripts/audit/broken-relative-links.sh` delegates to `check-corpus-links --scope repository` and emits `file:line` findings and skips; `runtime/src/primitives/check_corpus_links.rs` `classify` states the `../` fix for a depth error and the name-it-in-prose guidance for a deleted target, with unit tests for code spans, fences and exclusions (ran: 816 examined, 137 excluded, rc 0).
- AC20 — met: `scripts/audit/done-spec-criteria.sh` reads status from frontmatter only, scans only the Acceptance Criteria section outside fences, emits `file:line` with the label and both repairs, skips untracked specs with a count, and fails on zero `done` specs (ran: 69 examined, rc 0).
- AC21 — met: `scripts/audit/audit-family-parity.sh` derives all three sets (`run_check` lines, `(Family N` entries in `framework/reference/audit.md`, §Scripts entries of `scripts/audit/README.md`), reports each direction separately and fails on an empty derivation (ran: 38/38/38, rc 0).
- AC22 — met: Annotated Superseded by 057 and judged on that record: `scripts/audit/run-all.sh` registers no Family 31, no review-agreement script exists under `scripts/audit/`, and `scenarios/family-31-review-block-agreement.md` is gone.
- AC23 — met: `scripts/audit/manifest-destination-links.sh` copies Shared Files sources into a synthetic tree for `check-corpus-links --scope repository` (35a) and lexically checks slash-command sources (35b), emits on empty extraction, missing runtime and unparseable result, and reports both counts (ran: 29 files from 38 entries, 16 sources from 17, rc 0).
- AC24 — met: `scripts/audit/self-url-resolution.sh` derives the slug from the `codeload.github.com` archive URL in `framework/bootstrap/ductus.md`, resolves only `main` refs, counts pinned ones, and emits `unresolved`, `blob-names-dir` and `tree-names-file` separately (ran: 935 files, 56 URLs, rc 0).
- AC27 — met: Annotated Superseded by 081 and judged on that record: `scripts/audit/run-all.sh` registers no Family 19, `scripts/audit/review-freshness.sh` is deleted (33688944), and no `check-review-freshness` primitive remains registered in `runtime/src`.
- AC26 — met: `scripts/audit/shipped-anchor-resolution.sh` enumerates `git ls-files` over `framework/*.md` plus every spec's `spec.md`, `plan.md` and scenarios, calls `resolve-anchor` per file, emits at `file:line` and on unreachable runtime, empty enumeration and unreadable result, with coverage on stderr; `scripts/tests/test-anchor-and-label-families.sh` passes (ran: 544 of 544, rc 0); the Never-held and Holds annotations match the code.

## Observations

- convention — the reference-split scenario's first Edge Case said a `§` pointer into `audit`'s reference would stay `qualified`, unjudged by Family 39, though Family 39 has judged `qualified` references since 067's `shared-constitution-references-are-judged` — `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md` — **fixed** (decided by the rule)
- convention — the family-36 scenario stated its 32 URLs and 32 findings in the present tense with no date or method, while the family now counts 56 self-referencing URLs; the 32 is the shipped citations `96823ec0` counted on 2026-09-05 — `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md` — **fixed** (decided by the rule)
- convention — the family-36 scenario's `{placeholder}` Edge Case omitted the exclusion `scripts/audit/self-url-resolution.sh` applies to a URL whose path holds an ellipsis (`…` or `...`) — `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md` — **fixed** (decided by the rule)
- quality — `scripts/audit/primitive-promotion-candidates.sh` expands `${legacy_files[@]}` under `set -u`, an unbound-variable error on bash 3.2 when `runtime/legacy-prose-commands.txt` lists no file, so an empty allowlist makes Family 9 exit 1 with no finding — `scripts/audit/primitive-promotion-candidates.sh` — **waived**: An observation about the pipeline's own machinery, waived with its reason (AGENTS.md): it is latent, since the shipped allowlist holds four entries, and an empty one fails loudly with exit 1 rather than passing, so no check reads clean that did not run. (decided by the rule)
- other — the family-36 scenario's 'Three findings, because three repairs differ' and AC24's enumeration list unresolved, blob-names-dir and tree-names-file, while the family now also reports `outside`, `file-outside` and `unreadable`; the two new kinds are stated in the next paragraph, not in that list. — `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md` — **waived**: The three-finding list is the repairs for a path inside the repository and still holds as written; the new kinds are specified in the Behavior paragraph that follows, and AC24 says those three are distinct, not that they are the only kinds. (decided by the rule)
- other — the family-34 scenario's Context says 'the 33 existing `/audit` families'; the framework has more families now. — `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md` — **waived**: the sentence is the account of the 054 incident at the time Family 34 was introduced, when 33 families existed and Family 34 was the next; it is a dated historical account, not a claim about the present, as the scenario's other counts carry their dates (decided by the rule)
- other — three copies of the containment comparison now exist (lib.sh `contained`, 35b's inline python, 36's `inside`), and 35a starts one python3 process per Shared Files row (38 rows today, about a second). — `scripts/audit/lib.sh` — **waived**: The copies are the documented trade-off (lib.sh's comment and the family scenarios): the python-computing families cannot call a shell function per path without a process each, and the comparison is two lines. The per-row cost is under two seconds in a maintainer-run script that also builds a runtime-backed tree. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/implement.md` — 066-audit-records-and-the-completion-gate
- `.claude/commands/ductus/review.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `.ductus/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `.ductus/reference/review.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `.github/workflows/runtime-acquisition.yml` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime-release.yml` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `.github/workflows/runtime.yml` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `.pi/prompts/ductus-amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-implement.md` — 066-audit-records-and-the-completion-gate
- `.pi/prompts/ductus-review.md` — 066-audit-records-and-the-completion-gate
- `AGENTS.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `README.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `docs/developing-ductus.md` — 062-concurrent-session-targets, 066-audit-records-and-the-completion-gate
- `docs/runtime.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `docs/slash-commands.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/bootstrap/ductus-procedure.md` — 027-bootstrap-migration-registry
- `framework/bootstrap/ductus.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `framework/bootstrap/govern.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `framework/commands/amend.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `framework/commands/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/commands/review.md` — 066-audit-records-and-the-completion-gate
- `framework/constitution.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/migrations.toml` — 027-bootstrap-migration-registry
- `framework/reference/audit.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `framework/reference/implement.md` — 066-audit-records-and-the-completion-gate
- `framework/reference/review.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `runtime/CHANGELOG.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 073-decided-rule-departures-are-recorded
- `runtime/README.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `runtime/src/interpreter/mod.rs` — 066-audit-records-and-the-completion-gate
- `runtime/src/interpreter/payload.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `runtime/src/main.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/check_review_gate.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/classify_spec_edit.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/compute_review_scope.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/criterion_verdicts.rs` — 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/primitives/derive_routing_candidates.rs` — 079-review-and-analysis-work-converges
- `runtime/src/primitives/invalidate_review.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/line_citations.rs` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/mod.rs` — 020-code-review, 066-audit-records-and-the-completion-gate
- `runtime/src/primitives/write_review.rs` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `runtime/src/schema/descriptions.rs` — 000-slash-commands, 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/src/schema/extensions.rs` — 022-deterministic-runtime, 048-govern-acquired-runtime, 066-audit-records-and-the-completion-gate
- `runtime/src/schema/primitives.rs` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/fixtures/implement-basic/.govern.session.toml` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/implement-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/fixtures/review-basic/stdin.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/implement-basic.jsonl` — 066-audit-records-and-the-completion-gate
- `runtime/tests/golden/review-basic.jsonl` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/mcp.rs` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `runtime/tests/review_command.rs` — 066-audit-records-and-the-completion-gate
- `scripts/audit/README.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/audit/done-spec-criteria.sh` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/audit/migration-coverage.sh` — 027-bootstrap-migration-registry
- `scripts/audit/promotion-coverage.sh` — 067-analyze-and-audit-checks
- `scripts/mutation/criteria-in-review-066.py` — 000-slash-commands, 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `scripts/mutation/fix-round-record.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/line-citation-strip-066.py` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `scripts/mutation/reopen-measure.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/reviews-follow-what-changed.py` — 066-audit-records-and-the-completion-gate
- `scripts/mutation/scope-attribution.py` — 066-audit-records-and-the-completion-gate
- `scripts/tests/test-migration-coverage.sh` — 027-bootstrap-migration-registry
- `specs/000-slash-commands/analysis.md` — 000-slash-commands
- `specs/000-slash-commands/plan.md` — 000-slash-commands
- `specs/000-slash-commands/review.md` — 000-slash-commands
- `specs/000-slash-commands/scenarios/criterion-route-after-draft.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/000-slash-commands/scenarios/validation-gates.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/000-slash-commands/spec.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/007-govern-workflow/plan.md` — 007-govern-workflow
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/020-code-review/analysis.md` — 020-code-review
- `specs/020-code-review/data-model.md` — 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `specs/020-code-review/plan.md` — 020-code-review
- `specs/020-code-review/review.md` — 020-code-review, 022-deterministic-runtime
- `specs/020-code-review/spec.md` — 020-code-review, 022-deterministic-runtime, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/data-model.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/022-deterministic-runtime/scenarios/implement-completion-gate.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/022-deterministic-runtime/scenarios/review-runtime-acceleration.md` — 020-code-review, 022-deterministic-runtime
- `specs/022-deterministic-runtime/spec.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/027-bootstrap-migration-registry/analysis.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/plan.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/review.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/a-projects-own-constitution-is-not-relocated.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migration-chain-reference-integrity.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/scenarios/migrations-apply-once-per-repo.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/spec.md` — 027-bootstrap-migration-registry
- `specs/027-bootstrap-migration-registry/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/analysis.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/plan.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/review.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/spec.md` — 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 027-bootstrap-migration-registry, 029-bootstrap-runtime-autowire
- `specs/048-govern-acquired-runtime/data-model.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/state-b-continues-in-session.md` — 022-deterministic-runtime, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 022-deterministic-runtime, 029-bootstrap-runtime-autowire, 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/tasks.md` — 022-deterministic-runtime, 048-govern-acquired-runtime, 062-concurrent-session-targets, 067-analyze-and-audit-checks
- `specs/060-exec-analyze-assesses-each-loaded-rule/plan.md` — 000-slash-commands, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate
- `specs/062-concurrent-session-targets/data-model.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/review.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/scenarios/first-writers-create-the-ignore-file-once.md` — 048-govern-acquired-runtime, 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/spec.md` — 048-govern-acquired-runtime, 062-concurrent-session-targets
- `specs/066-audit-records-and-the-completion-gate/analysis.md` — 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/data-model.md` — 020-code-review, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/plan.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/scenarios/criteria-are-verified-in-review.md` — 000-slash-commands, 066-audit-records-and-the-completion-gate
- `specs/066-audit-records-and-the-completion-gate/spec.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 000-slash-commands, 020-code-review, 022-deterministic-runtime, 060-exec-analyze-assesses-each-loaded-rule, 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/067-analyze-and-audit-checks/review.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md` — 048-govern-acquired-runtime, 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/spec.md` — 048-govern-acquired-runtime, 067-analyze-and-audit-checks
- `specs/073-decided-rule-departures-are-recorded/plan.md` — 073-decided-rule-departures-are-recorded
- `specs/079-review-and-analysis-work-converges/analysis.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/plan.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/review.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/spec.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/plan.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/review.md` — 081-reviews-and-checks-follow-what-changed
- `specs/081-reviews-and-checks-follow-what-changed/spec.md` — 066-audit-records-and-the-completion-gate, 079-review-and-analysis-work-converges, 081-reviews-and-checks-follow-what-changed

## Unexamined governance

*None.*
