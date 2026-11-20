---
spec: 026-framework-self-audit
last-run: 2026-11-20T00:50:00Z
reviewed-against: 6e319bdf3d43af8d295d37241816da9d48fc731b
diff-base: 1cd775cfc88886f8c4fb401f01803dfb2b8a0aa7
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 21
scope: 25
unread:
  - runtime/CHANGELOG.md
  - runtime/src/main.rs
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
excluded: 68
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
  scenarios/families-follow-the-command-reference-split.md: 0d900d455decf8f126bdf8ef6da61284ca98641cfbfe89ac334e80f22df7e63c
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
  scenarios/family-34-reference-section-reachability.md: 2e27857310f2c77e7e87dd8028c537978b0cdbce8f0eec87a6287538b7b98e29
  scenarios/family-34-step-reference-integrity.md: 69728c803782a37219f79c1c6b8c53c35eb470fadebb8fcdbd523c0b86adbb0d
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
  fixed: 6
  deferred: 0
  waived: 7
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
  - key: "other - reuse: three ATX heading scanners (`parse_atx_heading`, `heading_lines`, `atx_headings`) serve this primitive's two halves and disagree on a closing-hash heading — `runtime/src/primitives/tool_loading.rs` — `runtime/src/primitives/tool_loading.rs`"
    outcome: waived
    reason: "Pipeline machinery: no shipped reference uses a closing-hash heading, the failure is loud (an unreached-section finding), and unifying the scanners is a refactor across 022's and 080's shared helpers, not a defect in 026's change."
    decided-at: 2026-11-20T00:35:00Z
    decided-by: rule
  - key: bug - sections are reached by heading text, so two `##` sections with the same text in one reference are judged as one — `runtime/src/primitives/check_step_references.rs` — `runtime/src/primitives/check_step_references.rs`
    outcome: waived
    reason: A citation names a section by its heading text, so two same-text sections cannot be cited apart by any step; no shipped reference holds a duplicate `##` heading (checked over `framework/reference/*.md`), so it is a latent false negative with no instance.
    decided-at: 2026-11-20T00:35:00Z
    decided-by: rule
---

# Review — 026-framework-self-audit

## Summary

Window `1cd775cf` to HEAD, base-source `reopen`: `b0fa2e98` reopened 026 for Family 34's judgment of the reference sections a command's numbered steps reach, and `1b6455d2`, `71401ef5`, `6a91019e`, `f10d8321` and `47437915` delivered it. Scope 25 entries; no criterion owed a verdict by `compute-review-scope`, and all 26 were judged anyway. Examined 21: the whole of `runtime/src/primitives/check_step_references.rs`, `runtime/src/primitives/tool_loading.rs`, `runtime/tests/check_step_references_cli.rs`, `scripts/audit/step-reference-integrity.sh`, `scripts/tests/test-step-reference-integrity.sh`, `scripts/mutation/step-references-bounded.py`, `scripts/mutation/step-references-reach.py`, the three scenarios, `specs/026-framework-self-audit/tasks.md`, the window's diff of each of `framework/reference/analyze.md`, `framework/reference/audit.md`, `framework/reference/plan.md`, `framework/reference/review.md` and `scripts/audit/README.md` with the section around each hunk, the `spec.md` diff, and the four installed copies under `.ductus/reference/`, each compared with its source after the generator's own substitution (`gen-claude-commands.sh --check` itself was not run). Unread, named as such: `runtime/CHANGELOG.md`, `runtime/src/main.rs`, `runtime/src/schema/descriptions.rs` and `runtime/src/schema/primitives.rs` were read only where the window changed check-step-references (its changelog entry whole, its description constant whole, its result structs whole, the command's arm and exit gate); the hunks other specs made there were not read. The security pass read all three `security-*.md` files whole and the other passes the nine remaining rule files whole, plus `AGENTS.md` from Gotchas on and `specs/system.md`, against all 12 loaded rule files; no rule finding. The passes ran the release binary, the Family 34 script and its test, and fixtures. Eight observations were made: six were chores fixed in the run (an exemption finding named the line before its marker, the stated rule said directly beneath where the code takes the first non-blank line, the description claimed a file with no steps is a finding and keyed reachability as path, a test doc said five lists, and the CLI's non-zero exit over a skip was pinned by no test), proven by mutants M19 and M20, and two were waived (three heading scanners disagreeing on a closing-hash heading, and two same-text sections judged as one). Five stored decisions on in-scope files still hold and are carried; one decision whose key lists several paths expired and is dropped. Criteria: 26 of 26 judged met; the criteria parts read the criteria whole and drove or mutated each family, but read some family scripts by header and targeted search rather than line by line, and judged annotated criteria as annotated. Fix rounds: see the count line below.

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

- AC1 — met: `framework/commands/audit.md` exists, its Instructions step 1 runs `scripts/audit/run-all.sh` and the file says it runs without a session target; the tracked `.claude/commands/ductus/audit.md` differs only by the host-added ToolSearch note.
- AC2 — met: Judged as annotated: `emit` in `scripts/audit/lib.sh` renders `FAMILY | LOCATION | MESSAGE | SUGGESTED-FIX` with no severity (same 4 columns at `1f92d4cc`); I drove Families 1, 2, 4, 5, 7, 8, 9 to findings of that shape, and `scripts/audit/ssot-invariants.sh` has no `emit` (Family 6 stub, `b2695ab3`).
- AC3 — met: Judged as annotated: `scripts/gen-readme-table.sh` is gone (deleted in `9847647b`; `1f92d4cc` invoked it); `scripts/audit/cross-doc-consistency.sh` carries 1b (README order swap driven to a finding) and 1c (derives transitions from the constitution's `Backward via` bullets, README is the consumer, `docs/introduction.md` gone) and exits 0 on the tree.
- AC4 — met: Judged as annotated: `scripts/audit/manifest-parity.sh` fails on a missing end marker, a stray `mcp__ductus__` grant and an empty block (all three driven, rc 1), prints the examined count (73) to stderr, header records the installer half retired (never built at `1f92d4cc`; `framework/commands/init.md` absent); the set comparison against `framework/runtime-tools.txt` is done by check-zero's `gen-configure-mcp.sh --dry-run`, not by this script, which the annotation's phrase 'checks the MCP tool set derived from runtime-tools.txt' glosses.
- AC5 — met: Retired record, judged as history: `framework/workflows/` is absent (spec 043, `531e3ea`) and `scripts/audit/run-all.sh` registers no Family 3.
- AC6 — met: Driven on a mutant: `scripts/audit/placeholder-roundtrip.sh` flags `.claude/` and `/ductus:` in a command file and `{cli-config-dir}`, `$ARGUMENTS`, `.pi/` in a reference file (rc 1); an empty `agent_config_dirs` derivation is a finding (lines 59-65); clean tree rc 0.
- AC7 — met: Judged as annotated: `scripts/audit/template-alignment.sh` holds a hardcoded invariant list (header 'v1 checks (hardcoded canonical invariants)' at `1f92d4cc`), covers Done-when (driven: renaming it in `framework/templates/spec/tasks.md` gave 3 findings) and `guard_judged_set` compares `judged_blocking_sections` to analyze.md's blocking headings; tree rc 0.
- AC8 — met: Judged as annotated: `scripts/audit/ssot-invariants.sh` has no `emit` and exits 0 (stub since `b2695ab3`, 'v1: no findings emitted' present at `1f92d4cc`), and writes `ssot-invariants: skipped - detection not implemented (026 AC8)` to stderr, which `run-all.sh` passes through (seen in the full run).
- AC9 — met: Driven: two draft specs linking each other with a shared plan.md Affected Files row gave one finding naming both specs and `x/shared.md` (rc 1) in `scripts/audit/sibling-coupling.sh`; a `Why split from ...:` entry in the second spec's Resolved Questions suppressed it (rc 0).
- AC10 — met: Judged as annotated: `scripts/audit/introducing-drift.sh` has a literal `RENAMED_TOKENS` list (header says not derived from git log, same at `1f92d4cc`), flags any backticked old token outside a blockquote regardless of tense (driven on `specs/017-derive-dont-ask/spec.md`: the plain line flagged at file:line with a rewrite hint, the `>` line skipped).
- AC11 — met: Driven: `scripts/audit/primitive-promotion-candidates.sh` flagged prose-only numbered steps and not the `<!-- llm:* -->` step (an `<!-- audit:ignore-promotion -->` placed inline on the step line still flagged, the marker goes on the line above); `runtime/legacy-prose-commands.txt` lists 4 files now, 6 at `1f92d4cc`, and the script skips listed files, as annotated.
- AC12 — met: `scripts/audit/run-all.sh` ends `exit "$drift"`: real tree rc 0 (full run), a stub aggregator with one failing family rc 1; `.github/workflows/framework-checks.yml` step (h) and `.github/workflows/runtime-release.yml` job `audit` run it.
- AC13 — met: Judged as annotated: `run_check` in `scripts/audit/run-all.sh` prints `=== label ===` only for a family with findings (same at `1f92d4cc`; a clean family printed nothing on the real run), rows are `FAMILY | LOCATION | MESSAGE | SUGGESTED-FIX` with no severity, output is stdout-only and no `audit.md` is written.
- AC14 — met: `framework/commands/audit.md` has `## Boundary with /ductus:analyze` (owner table plus rule of thumb, never-duplicate sentence) and says maintainer-only; no Notes section was ever committed (`git log -S'## Notes'` empty).
- AC15 — met: `scripts/audit/adopter-shell-behavior.sh` builds the fixture at `specs` and `features` with the runtime only at `.ductus/bin/ductus`; removing the pointer lookup or the re-stage in `framework/bootstrap/hooks/ductus-pre-commit` yields findings; `.ductus/scripts` is absent from `framework/bootstrap/ductus.md`, so the superseded annotation holds.
- AC16 — met: `scripts/audit/sweep-target-manifest-parity.sh` extracts both sides, emits on empty or unbalanced markers, reports direction and counts (13 entries, 38 paths); deleting `framework/` from the `AGENTS.md` list yields findings.
- AC17 — met: `scripts/audit/rename-sweep-residue.sh` has three closed classes (modal, demonstrative/wh, possessive) with the, to, that, my excluded; a planted `should ductus` in `framework/rules/quality-cross.md` is found and the 885-file count is reported.
- AC18 — met: `scripts/audit/unbalanced-inline-markup.sh` flagged a planted odd backtick at `AGENTS.md:217` and reports its two-file scope on stderr (wrapped-bullet report present at the script's wrap check).
- AC19 — met: `scripts/audit/broken-relative-links.sh` delegates to `check-corpus-links --scope repository`; a wrong-depth scenario link yields the corrected path, a deleted target yields the prose advice, a fenced link is skipped with the following line number correct, and an unusable git index is a finding.
- AC20 — met: `scripts/audit/done-spec-criteria.sh` flagged an unticked AC1 in a done spec at `spec.md:75` with both repairs, and reports 68 examined with untracked skipped counted.
- AC21 — met: `scripts/audit/audit-family-parity.sh` derives all three sets (38/38/38): deleting a Family 41 doc entry gave registered-but-undocumented and documented-but-unregistered findings, and a planted `ghost.sh` in `scripts/audit/README.md` §Scripts was reported.
- AC22 — met: As annotated (superseded by 057): no review-agreement script exists in `scripts/audit/` (removed by `031fd9ef`), `run-all.sh` and `scripts/audit/README.md` name no such family, and `specs/057-*/` exists.
- AC23 — met: `scripts/audit/manifest-destination-links.sh` builds a synthetic adopter tree for 35a (29 files) and scans slash-command sources lexically for 35b (16); a planted `../nonexist/zz.md` in a shipped reference was reported at its destination path; empty/unreachable/unparseable are emitted as findings in the script.
- AC24 — met: `scripts/audit/self-url-resolution.sh` derives the slug from the codeload URL in `framework/bootstrap/ductus.md`; planted missing path, `/blob/` on a directory and `/tree/` on a file each gave distinct findings and a v1.0.0 URL was counted as pinned.
- AC27 — met: As annotated (superseded by 081): `scripts/audit/review-freshness.sh` was deleted in `33688944` and is absent, `run-all.sh` registers no Family 19, `framework/runtime-tools.txt` has no `check-review-freshness` and only the retired-entry lists in `framework/bootstrap/configure/claude.md` and `runtime/src/primitives/merge_permissions.rs` name it.
- AC26 — met: `scripts/audit/shipped-anchor-resolution.sh` enumerates `git ls-files` over `framework/*.md` and every spec.md, plan.md and scenario, delegates to `resolve-anchor`, reports at file:line, and stderr names the counts (558 files, 118 not judgeable); a planted bad anchor was found; the annotations match the script's header.

## Observations

- other: check-step-references' not-a-procedure field doc defines the set by files holding several numbered lists, though it also holds files with no step or too few — `runtime/src/schema/primitives.rs` — `runtime/src/schema/primitives.rs` — **waived**: the primitive's schema doc, 067's surface and outside 026's code; the family's own stderr line now says not one procedure (decided by the rule)
- other — the family-36 scenario's 'Three findings, because three repairs differ' and AC24's enumeration list unresolved, blob-names-dir and tree-names-file, while the family now also reports `outside`, `file-outside` and `unreadable`; the two new kinds are stated in the next paragraph, not in that list. — `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md:27-34; specs/026-framework-self-audit/spec.md:148` — `specs/026-framework-self-audit/scenarios/family-36-self-url-resolution.md` — **waived**: The three-finding list is the repairs for a path inside the repository and still holds as written; the new kinds are specified in the Behavior paragraph that follows, and AC24 says those three are distinct, not that they are the only kinds. (decided by the rule)
- other — reuse: the file-form read of a `lists-file` is now carried by four embedded-python copies (`scripts/audit/lib.sh` spec_corpus, `broken-relative-links.sh`, `manifest-destination-links.sh`, and `step-reference-integrity.sh`), each with its own key set and precondition wording. — `scripts/audit/step-reference-integrity.sh:116` — `scripts/audit/step-reference-integrity.sh` — **waived**: Pipeline machinery. Each family's python is embedded in its own script by the README contract (families keep their own renderer, and an apostrophe-free `python3 -c` string cannot import a shared module); the copies differ in the keys they require and the message they name, and a shared helper would be a new feature, not a defect in 026's change. The four copies read the lists the same way (missing list halts, never reads as none), which is the property that matters. (decided by the rule)
- other — the lists file sits under `.ductus/results/check-step-references/all-specs/`; `all-specs` is the key 080's data model gives a corpus-wide spec result, while this primitive's subject is command sources and two bootstrap files, not the spec corpus. Mutant M10 and the unit test pin the segment. — `runtime/src/primitives/check_step_references.rs:145` — `runtime/src/primitives/check_step_references.rs` — **waived**: a naming choice with no behavioral effect: the key only separates one primitive's list files, a caller reads the path from lists-file, and the whole-corpus key is how the other corpus-wide primitives key theirs (decided by the rule)
- other — the family-34 scenario's Context says 'the 33 existing `/audit` families'; the framework has more families now. — `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md:19` — `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md` — **waived**: the sentence is the account of the 054 incident at the time Family 34 was introduced, when 33 families existed and Family 34 was the next; it is a dated historical account, not a claim about the present, as the scenario's other counts carry their dates (decided by the rule)
- bug - the line of an `empty-exemption` and a `stale-exemption` finding was the line before the marker whenever a blank line separates the marker from its heading; it names the marker's own line now, pinned by a test and mutant M19 — `runtime/src/primitives/check_step_references.rs` — `runtime/src/primitives/check_step_references.rs` — **fixed** (decided by the rule)
- other - 'directly beneath its heading' was the stated rule while `judge_sections` attaches a marker on the first non-blank line after the heading; the description, scenario, module doc, `FIX` text and changelog now say the first non-blank line — `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md` — `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md` — **fixed** (decided by the rule)
- bug - the `CHECK_STEP_REFERENCES` description claimed a file with no steps is a finding and the `line` doc listed a `no-steps-extracted` kind no code produces; both now say such a file is named in `not-a-procedure` with no finding — `runtime/src/schema/descriptions.rs` — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- other - the test doc said the bound moves five lists while it moves six — `runtime/src/primitives/check_step_references.rs` — `runtime/src/primitives/check_step_references.rs` — **fixed** (decided by the rule)
- other - `reachability` entries are keyed `reference`, while the description and changelog said `path` — `runtime/src/schema/descriptions.rs` — `runtime/src/schema/descriptions.rs` — **fixed** (decided by the rule)
- other - the CLI's non-zero exit over a skipped subject was pinned by no test; a test and mutant M20 now pin it — `runtime/tests/check_step_references_cli.rs` — `runtime/tests/check_step_references_cli.rs` — **fixed** (decided by the rule)
- other - reuse: three ATX heading scanners (`parse_atx_heading`, `heading_lines`, `atx_headings`) serve this primitive's two halves and disagree on a closing-hash heading — `runtime/src/primitives/tool_loading.rs` — `runtime/src/primitives/tool_loading.rs` — **waived**: Pipeline machinery: no shipped reference uses a closing-hash heading, the failure is loud (an unreached-section finding), and unifying the scanners is a refactor across 022's and 080's shared helpers, not a defect in 026's change. (decided by the rule)
- bug - sections are reached by heading text, so two `##` sections with the same text in one reference are judged as one — `runtime/src/primitives/check_step_references.rs` — `runtime/src/primitives/check_step_references.rs` — **waived**: A citation names a section by its heading text, so two same-text sections cannot be cited apart by any step; no shipped reference holds a duplicate `##` heading (checked over `framework/reference/*.md`), so it is a latent false negative with no instance. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/review.md`
- `.ductus/reference/clarify.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-review.md`
- `framework/commands/analyze.md`
- `framework/commands/review.md`
- `framework/reference/clarify.md`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_command_flags.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_orphaned_references.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/spec_links.rs`
- `runtime/tests/bounded_list_gates.rs`
- `runtime/tests/derive_dependencies_golden.rs`
- `runtime/tests/derive_references_golden.rs`
- `runtime/tests/golden/review-basic.jsonl`
- `runtime/tests/mcp.rs`
- `scripts/mutation/relocate-merge-children-057.py`
- `scripts/mutation/reopen-measure.py`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/review.md`
- `specs/020-code-review/spec.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/tasks.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/057-analyze-artifact-and-record-relocation/analysis.md`
- `specs/057-analyze-artifact-and-record-relocation/data-model.md`
- `specs/057-analyze-artifact-and-record-relocation/review.md`
- `specs/057-analyze-artifact-and-record-relocation/scenarios/a-merge-carries-each-keys-whole-value.md`
- `specs/057-analyze-artifact-and-record-relocation/spec.md`
- `specs/057-analyze-artifact-and-record-relocation/tasks.md`
- `specs/066-audit-records-and-the-completion-gate/analysis.md`
- `specs/066-audit-records-and-the-completion-gate/data-model.md`
- `specs/066-audit-records-and-the-completion-gate/plan.md`
- `specs/066-audit-records-and-the-completion-gate/review.md`
- `specs/066-audit-records-and-the-completion-gate/spec.md`
- `specs/066-audit-records-and-the-completion-gate/tasks.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`

## Unexamined governance

*None.*
