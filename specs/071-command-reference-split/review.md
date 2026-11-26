---
spec: 071-command-reference-split
last-run: 2026-11-20T01:08:00Z
reviewed-against: 35f42b3646fa33142b56656b355bad5e64e98b19
diff-base: dec1b0b67bd3d63b4230bb19980027c536f8d93b
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 16
scope: 33
unread:
  - .claude/commands/ductus/implement.md
  - .claude/commands/ductus/plan.md
  - .claude/commands/ductus/review.md
  - .claude/commands/ductus/specify.md
  - .ductus/reference/analyze.md
  - .ductus/reference/consolidate.md
  - .ductus/reference/fold.md
  - .ductus/reference/plan.md
  - .ductus/reference/review.md
  - .ductus/reference/specify.md
  - .ductus/reference/status.md
  - .pi/prompts/ductus-implement.md
  - .pi/prompts/ductus-plan.md
  - .pi/prompts/ductus-review.md
  - .pi/prompts/ductus-specify.md
  - framework/reference/analyze.md
  - runtime/CHANGELOG.md
excluded: 158
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  scenarios/reference-sections-are-reached-by-a-step.md: 36e3ed15f3cc1a502d2b6fb284da192027b185b80692faab1522a48cf48ab0c9
criteria-judged: 24
criteria-met: 24
criteria-verdicts:
  - label: AC1
    digest: 91d3a897fab2d174656dede3173860cdf35872370e568dba297e3b0ba39a7324
  - label: AC2
    digest: 04bef7427bb0a0ac2502bcf453bd06455061877fe873fbc63b3a1c44f96b3044
  - label: AC3
    digest: a329786dc7543047b004ddace9afe7fa84924462c8ec7fb85f760a8da9d87c78
  - label: AC4
    digest: 49a6b827386cce7bc3616310b2785a6abbe7dee5c12f084c5e04845f18b5bbe1
  - label: AC5
    digest: d3aae62fff213480a0a199df5843f0979e4ec493e2e314b855748ae23e095e10
  - label: AC6
    digest: 9728ee023f095e99dc90fc963e8d67ba0015625dbd35b3c3ad5714b0292356df
  - label: AC7
    digest: c02d321c16de5f0d68b6355f6ac1b8ae41ef2a7ae1215268084a6f67caf43a75
  - label: AC8
    digest: 26889d21d33100954a8ed41b1c3b257a6f01a49b28978e460e0e3a55528c36f0
  - label: AC9
    digest: 022ea5bdd3d0fb602a4b875cfdee0a7d1f67e29df463d216fb174d504fe27a20
  - label: AC10
    digest: a44d6e0a3d65cdf49e003b671f739782380332cd77eecbfa3d4d8672dcb55458
  - label: AC11
    digest: abe298e62c1555a52c38ba2fa03812f752d11801d476f87d616374ede800843e
  - label: AC12
    digest: abd18e68e274f436bfbb490a25dcbfd142d7e821dab7e7baafb5e847daad9d48
  - label: AC13
    digest: 545fb653a276e0c4225b424671a1923d0cfa2affbcd511d1ac7f8b70358af7c7
  - label: AC14
    digest: e450f9aa635d2b6e0b3be83b6b40ef06c43aa2efdc4d4fa1a01466979ca21394
  - label: AC15
    digest: 77002af7ed7fd994b1df95c162e97ac1f79fd9cd5bff01611ec72656a9f8a3a5
  - label: AC16
    digest: ff828f8f906a9ab248aaa7c81609c71271265154a1201e83ceb133ef81018b9a
  - label: AC17
    digest: 9546cc17642ade76dd9ec86eed424750f067b6226786007aa75578d6d37ff31e
  - label: AC18
    digest: d7f3802e8354a7ab3462e7636c0404b201b51a1613aa41dc0f16a0177427942d
  - label: AC19
    digest: e73ddf523ad785834b0fd66a8889671a69d85c9462f0aa3d9c8118978ffde88d
  - label: AC20
    digest: 0f0651591f9d13d1e7dfd24ca8b00e12fafa73e8d1f6396e02cf7c8717d5de06
  - label: AC21
    digest: 8f224196f442cfae2caa3bc0787134714b8b105f2881092ccee2417a939bf0fa
  - label: AC22
    digest: f23e1aa8cbdd3236dc6389469ec2766f684f9fad0f7ed6e55af9038dac7b9f01
  - label: AC23
    digest: 1421c97b8defd4ba00276a4c3da9693364457081e228d6dd33955272299b0009
  - label: AC24
    digest: f8c3ff3afc4b612bf9f9f2895a202e2326ad21164a09f88dfbfaa0fe1b7e5569
blocking: false
dispositions:
  fixed: 6
  deferred: 0
  waived: 6
  undispositioned: 0
decisions:
  - key: "other: the markdown-only reference-cleanup prose differs from enforce-manifest at the edges (links never removed, dot-prefixed .md matched) — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "contrived: nobody writes either into a ductus-owned directory (owner decision, 2026-10-26)"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: gen-claude-commands.sh writes and prunes through a symlink at .ductus/reference/ — `scripts/gen-claude-commands.sh`"
    outcome: waived
    reason: the maintainer's own checkout, the same shape as the generator's two existing passes, and no trust boundary crossed (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: check-corpus-links reads framework/reference/'s presence as a framework checkout, which an adopter's own directory of that name would match — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: waived
    reason: "contrived: it costs only the repository-scope check of shipped copies, which an adopter's spec-corpus hook never runs (owner decision, 2026-10-26)"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "convention: resolve-anchor and check-artifacts each parse the Shared Files table, and the bootstrap path literal appears three times — `runtime/src/primitives/resolve_anchor.rs`"
    outcome: waived
    reason: their scopes differ on purpose (the plan records why), and merging them would narrow check-artifacts' suppression set (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "convention: gen-claude-commands.sh's reference pass is a third inline copy of the generate, compare and prune blocks — `scripts/gen-claude-commands.sh`"
    outcome: waived
    reason: the file's existing style; bash 3.2 has no namerefs, and refactoring it is its own change (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "convention: Family 16's reference arm is not section-scoped and duplicates Family 35's row parser — `scripts/audit/installer-command-parity.sh`"
    outcome: waived
    reason: "contrived: no framework/reference row exists outside Shared Files (owner decision, 2026-10-26)"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "convention: Family 18 restates its file label nine times — `scripts/audit/marker-list-parity.sh`"
    outcome: waived
    reason: the file's existing style (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: a **§X** pair naming the markers file is judged against its headings, not its markers — `runtime/src/primitives/resolve_anchor.rs`"
    outcome: waived
    reason: no such pair exists anywhere, and it fails loudly as unresolved, never silently (owner decision, 2026-10-26)
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: create-strategy rows map specs/inbox.md and its siblings to seed templates — `runtime/src/primitives/resolve_anchor.rs`"
    outcome: waived
    reason: "contrived: this repository's inbox headings equal the template's (owner decision, 2026-10-26)"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: the word boundary rejects a heading ending in punctuation followed by a word character, a non-ASCII letter after an ASCII anchor, and underscore emphasis around a § — `runtime/src/primitives/resolve_anchor.rs`"
    outcome: waived
    reason: "contrived: none occurs in the corpus, and each fails loudly as unresolved"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: a greenfield --ref run whose installed ductus.md is stale writes the config before aborting, so its re-run is not greenfield and prompts — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: pre-existing and not made worse; the documented install path places a current bootstrap
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: step 4's per-agent apply-manifest loop sits in one walker step whose context carries one map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the real bootstrap is walked by the host, as step 8's per-directory loop already is
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "other: the greenfield rule's newest-entry id is undefined when the active registry is empty — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: "contrived: the registry is never empty while migrations exist, and 027's AC22 already writes nothing then"
    decided-at: 2026-10-26T15:06:31Z
    decided-by: andy@stone.dev
  - key: "convention: the corpus scope matches `.md` case-sensitively where the repository scope does not — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: waived
    reason: Predates 059; which files each scope examines is `check-corpus-links`' own contract, and changing either direction changes what the check reports.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "convention: tests that expect no repository assume the temp directory has no enclosing work tree — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: waived
    reason: Predates 059 and is shared with other test modules; a ceiling would need process-wide environment in a parallel suite, and CI's temp directory has no enclosing work tree.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "bug: the criterion-path grammar accepts `../` and scenario finding paths hard-code a lowercase `.md` — `runtime/src/primitives/check_artifacts.rs`"
    outcome: waived
    reason: Predates 059; both are contracts of `check-artifacts`' criterion-path and scenario families, and neither is a path conversion.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "security: the review payload ships an in-scope file whatever its name, where the writeCode payload refuses secret-named and ignored files — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: Predates 059; the reviewing host reads the same tree with its own tools, and whether review withholds a file it is reviewing is the review payload's contract.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "perf: `read_repo_file` canonicalizes the project root once per rule file — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: N is the rule-file count, so the cost is negligible.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "perf: build_assess_spec_quality_request re-reads the spec file for every rule request, up to 167 times per step — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: each read is a small local file the OS caches, beside an LLM round trip per request; reading it once would thread a second value through the walker context for a gain nobody has measured
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "reuse: runtime_binary, ensure_binary_built and the envelope loop are copied across test files — `runtime/tests/exec_subprocess.rs:18`"
    outcome: waived
    reason: predates the window; consolidating test helpers is a refactor outside 048
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "efficiency: the prune-plan walk runs inline on the async worker and reads HEAD per done spec, even for specs it then drops — `runtime/src/mcp/server.rs:710`"
    outcome: waived
    reason: one HEAD read per done spec is not the unbounded revwalk the blocking pool is scoped to, and a spec it drops still needs the answer to know it can be dropped
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "tests: the macOS integration test leaks its mktemp directory if building the archive or spawning the binary panics — `runtime/tests/exec_subprocess.rs`"
    outcome: waived
    reason: it leaks only on a panic before its assertions, one empty directory under the per-user temp directory, which macOS clears
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "perf: dashboard and the --all prune walks now walk every feature directory — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: measured dashboard 0.23-0.25 s, prune-plan --all 0.04 s and prune-tasks --all 0.01 s on this corpus, the walks' share at most about 6 ms; refusing the whole command is the scenario's stated edge case
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: canonicalize-and-contain logic exists in both interpreter/payload.rs and primitives/mod.rs — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: "the two differ on purpose: the payload readers skip a missing or outside path and give None for a non-UTF-8 name, where the check refuses with an error and judges a missing path, so one helper would carry both behaviors"
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "convention: RecordFreshness::Undeterminable's doc names only a missing digest, where an unparseable review.md also answers it — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: the tasks 43 and 44 changelog entry does not name BE-API-013's explicit bar on a credential or PII in a link sent to a person — `runtime/CHANGELOG.md`"
    outcome: waived
    reason: "Not mechanical, and not untrue: BE-API-013's earlier Not-flagged clause already kept a URL sent in an email or another message as its own, so the entry, which lists new obligations, omits a clarification rather than a change."
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: a security-frontend.md copied from main between afb8a865 and 0481c218^ hands over only a server the project serves, so it keeps an authorization-server library's redirect_uri case beside SEC-AUTHN-004 — `runtime/CHANGELOG.md`"
    outcome: waived
    reason: "A contrived input: both windows are unreleased main, the second about an hour long, and a project pinning such a copy re-syncs it as the changelog's pinned-copy sentence already says."
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: check-step-references' not-a-procedure field doc defines the set by files holding several numbered lists, though it also holds files with no step or too few — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: the primitive's schema doc, 067's surface and outside 026's code; the family's own stderr line now says not one procedure
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: under ductus exec a malformed shared session default halts as config-unreadable, since Host::load reads it first, so main.rs's session-unreadable branch is reached only by the process's own record — `runtime/src/main.rs`"
    outcome: waived
    reason: The branch is still reached by a malformed own record, the exec test pins the code the default halts with, and either code labels a halt that names the file.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: with a pinned `target.md`, a tool can load from neither side when the session's `cli-config-dir` names another agent — `runtime/src/primitives/apply_manifest.rs`"
    outcome: waived
    reason: "Inside the two-agent-CLI bound the constitution's §concurrent-features names: the runtime reads command files from the agent `/ductus` last recorded. Within one agent's layout every case agrees."
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: no test holds each bounded member's top-level `-count` keys to the lists its file holds, which the walker's halt reads — `runtime/tests/mcp.rs`"
    outcome: waived
    reason: Every member's counts name a list its file holds today, checked struct by struct; a guard for a future member is new test design, not a chore.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: past the bound the MCP response carries the result twice, as text (49,965 bytes for render-status on 700 specs) and as `structuredContent` (53,614) — `runtime/src/mcp/server.rs`"
    outcome: waived
    reason: Claude Code hands the model one copy, as every `mcp__ductus__` result in this run shows; the bound measures the result, and changing how the server's JSON wrapper serializes is 022's surface and not a mechanical edit.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: fix round 1's re-read found that the dashboard test drives only `session-unreadable` past the bound, so removing the `session-notices` move fails no test — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: Both lists move by the same branch, which the test drives for one of them; a fixture that puts a session notice past the bound needs co-targeting session records, which is new test design, not a chore.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: for a namespace holding a tab or a line break, Pi's bridge respells the status view with render-help's unflattened namespace, so the view's flattened hints keep their colon form — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: Such a namespace names no command on any host, since Pi registers `{project}-status` from it and every installed command file is named by it, so no respelled hint could run; aligning the bridge would change 064's verbatim bridge, not a mechanical edit.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "test: the check_artifacts REGISTRY fixture carries a scope column the parser does not read — `runtime/src/primitives/check_artifacts.rs`"
    outcome: waived
    reason: the fixture mirrors the real file's table shapes, and the real MCP registration table carries the column
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "edge: a session or config naming a dropped host is not diagnosed: .augment still resolves through the claude-style candidate, and .opencode or .agents ends in command-not-found naming the paths tried — `runtime/src/host.rs`"
    outcome: waived
    reason: Host::load documents that a config-dir mismatch surfaces as the command-file-not-found error, which names every path tried; the resolver is generic in cli-config-dir by design, so a custom directory keeps working, and the CHANGELOG's Removed entry announces the removal
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: this repository's .pi/prompts/ductus-configure.md names .pi/prompts/ductus.md, which this repository does not track — `.pi/prompts/ductus-configure.md`"
    outcome: waived
    reason: the dogfooded .pi copies keep the generator honest; this repository is not a Pi adopter, and an adopter's copy has .pi/prompts/ductus.md beside it, which install.sh writes
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: pi's configure step restates /ductus's latest-release fetch without its grammar check, floor halts or DUCTUS_REPO notice — `framework/bootstrap/configure/pi.md`"
    outcome: waived
    reason: configure needs only the ref to fetch the bridge at; the halts guard /ductus's own writes, a bad ref fails the bridge fetch with curl's error, and the bounds are named rather than copied
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the template has frontmatter, so the walk skips it with a report line and writes nothing
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: the pre-run status's claim that nothing earlier in the run writes under specs/ holds only if Pre-run Migrations run before walker step 4's Shared Files apply-manifest, which ductus.md §Instructions does not state — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: already recorded as 056's scenario the-walker-places-every-archive-half-section, whose task gates 056's done; the old fresh status refused the same way, so this is no regression
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "edge: Pre-run Migrations step 1's abort does not say whether the section or the run stops — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: "contrived: it needs a malformed archive, and the pre-run status is taken before step 1, so either reading is safe"
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "edge: a file the adopter had dirtied that a migration also rewrote is named only by the refusal, not as changed by a migration — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: "decided in the plan and in the scenario's first edge case: the file is the adopter's to commit, and the refusal names it"
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: the repeated refusal ends with The rest of this run continues after the run has finished — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: wording only; the sentence is still true of the run it describes
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "convention: step 2's parenthetical in `.ductus/reference/analyze.md` §Rules (blocking and advisory) can read as describing the checks rather than naming the heading — `framework/bootstrap/ductus-procedure.md:198`"
    outcome: waived
    reason: "Style only: the checks applied are the same either way, and Family 39 needs the full heading (owner, 2026-11-03, option a)."
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: a Windows absolute document path already holds backslashes, and the space escaping may misread them — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: resolve-constitutions writes the document path through rel_path, which replaces every backslash with a forward slash, so the block line holds no backslash but the spaces the bootstrap escapes
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "convention: framework/bootstrap/ductus.md §Project Configuration says the description renders wherever ductus names a source (fix rounds 1 and 3's NOTEs) — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: A comment the installer writes into adopters' config; correcting it is a bootstrap change, which this repository verifies only with a real adopter run, out of proportion to one clause; the data model states which surfaces carry the description.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: main's bootstrap calls `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, so a `--ref=main` run errors at that step and falls back to the unflagged call until a release moves the pin — `framework/bootstrap/ductus.md:924`"
    outcome: waived
    reason: "An observation about the pipeline's own machinery made while verifying framework work (AGENTS.md): it concerns 055's step, not 077's, blocks nothing here, and ends when the next release tag moves the runtime pin to one carrying the flag."
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: an adopter with an installed .augment, .agents or .opencode tree gets no notice from /ductus, nothing removes it, and the managed .gitignore block no longer ignores it — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the owner dropped the three hosts as a breaking change; the CHANGELOG's Removed entry tells such an adopter to remove the tree by hand, the runtime refuses writes there by the owner's exception, and detecting a dropped host's tree would be new bootstrap behavior rather than a fix to this change
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: the swept migrations narrow three procedures to Claude, so a dropped-host adopter far enough behind loses their rewrites silently — `framework/migrations/runtime-store-path.md`"
    outcome: waived
    reason: no entry was removed or renumbered, so no last_applied names a missing id; claude and pi adopters run the same steps as before, and the hosts were dropped
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change"
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "exec-path: fold's and consolidate's session step dispatches nothing under exec, and §concurrent-features states no exec carve-out — `framework/commands/fold.md:76`"
    outcome: waived
    reason: the reduction is documented in each step, the convention the two-paths guarantee sets for exec reductions, as clarify.md's scope note does
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "ux: /target X announces adopting the feature it is about to leave — `framework/commands/target.md:29`"
    outcome: waived
    reason: "truthful: the session did adopt that target before the write replaced it"
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "reuse: AGENTS.md entries restate 061's rationale and the recommendations rule, and carry dated measurements of 022's record and the file's size — `AGENTS.md:46`"
    outcome: waived
    reason: each records what a rule cost here at a dated moment, which is what the file is for; classification to the constitution is 050's backlog
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "prose: prune.md's opening and step 5 are long, nested sentences — `framework/commands/prune.md:8`"
    outcome: waived
    reason: readability of correct procedure text; the content is accurate after this run's fixes
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: §spec-lifecycle resolves the plan rule against the shared test only by sentence order — `framework/constitution.md`"
    outcome: waived
    reason: coherent as read, the rule sitting between the three cases and Anything else, and the section is held at its byte ceiling
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: the implement reference's \"non-blocking\" counts a SHOULD as blocking where review vocabulary does not — `framework/reference/implement.md`"
    outcome: waived
    reason: the inline definition names exactly which findings it covers, and non-blocking is the scenario's own term
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md`"
    outcome: waived
    reason: "contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true"
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md`"
    outcome: waived
    reason: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: fix rounds driven by mechanical NOTEs have no length bound — `framework/reference/review.md`"
    outcome: waived
    reason: The Behavior permits NOTE rounds and AC6's budget counts re-reads with a MUST or SHOULD; a bound on NOTE rounds is a requirement decision.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "convention: docs/shared-constitution.md's authoring guideline 'Anything that loosens a framework rule' leaves out the exception (fix round 1's NOTE) — `docs/shared-constitution.md`"
    outcome: waived
    reason: An authoring guideline for shared sources; the exception is a framework rule's own opt-in, which a shared author meets in that rule's text.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: BE-ERR-002's Verification reaches a request-response protocol a process serves over its stdio, while AGENTS.md's gotcha says BE-ERR-002's correlation ID never fires against the runtime and an MCP tool error carries no code — `AGENTS.md`"
    outcome: waived
    reason: "Not mechanical, and 077's review waived the same tension by the rule: AGENTS.md's gotcha is this repository's reading of the rule against its own local runtime, naming the parts that assume a deployed service, while the rule states what an adopter's API owes, and the two meet at the gotcha's 'Name the boundary first'."
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "security: `audit.md` step 1 and `analyze.md`'s drift check send the agent to `.ductus/bin/ductus run-generator`, which the seeded `Bash(.ductus/bin/ductus *)` allow runs with no prompt until 022 task 18 narrows it — `framework/commands/audit.md:31`"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "note: the check-artifacts skipped-worklist entry's pointer is approximate; no constitution section states its lead — `AGENTS.md`"
    outcome: waived
    reason: §spec-requirements is the nearest statement and the verbatim lead carries the rule; Family 38 matches by lead, not target
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "reuse: Family 2's generated-block extraction repeats the marker literals that `scripts/tests/test-gen-configure-mcp.sh` and the generator use — `scripts/audit/manifest-parity.sh`"
    outcome: waived
    reason: scripts/tests has no lib.sh to share a helper through, and check-zero's gen-configure-mcp.sh --dry-run already fails when the markers drift
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "note: a ceiling over 19 digits passes the digit guard and makes `[ -gt ]` error — `scripts/lint-governing-size.sh`"
    outcome: waived
    reason: "contrived: a ceiling that large admits any file, so the silent pass it allows cannot hide an oversized one"
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "other: verify-installer ignores a Retry-After of 2^63 or more and reads a redirect hop's Retry-After when the final hop sends none — `.github/workflows/runtime-release.yml`"
    outcome: waived
    reason: an out-of-range value falls back to the jittered wait, and GitHub's release-asset redirect sends no Retry-After, so neither changes a run
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "reuse: Family 15 adds another hand-rolled scan of the §Agent Registry table beside those of installer-registry-parity.sh, host-namespace-parity.sh, placeholder-roundtrip.sh and permission-entry-shape.sh, while lib.sh's agent_config_dirs reads one column of it — `scripts/audit/runtime-probe-parity.sh`"
    outcome: waived
    reason: Each family reads a different part of the table and lib.sh's helper returns one column, so a shared row reader is a refactor of four other families' code, not a mechanical fix.
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "perf: install.sh validates the agent key only after the latest-release lookup and the bootstrap download — `install.sh`"
    outcome: waived
    reason: the order predates the removal, the arm still halts before writing anything, and moving the check is a behavior change with its own test
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "edge: a ./ prefix in the hook or a root markdown file assumes the project root is the work-tree root, which a project in a repository subdirectory breaks — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: waived
    reason: a ./ resolves as the bare paths in those files already do, the assumption every bare reference carries, not one task 30 adds
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "edge: in a project below its repository's root the hook runs from the repository's root, so its ./ and bare paths resolve there — `specs/067-analyze-and-audit-checks/data-model.md`"
    outcome: waived
    reason: the same assumption every bare reference in the hook carries, which task 30 does not add; prefixed and bare agree
    decided-at: 2026-11-11T13:40:45Z
    decided-by: andy@stone.dev
  - key: "test: ductus exec resolution of a claude-style commands/{project}/ copy lost its only subprocess fixture with exec-auggie, so both exec fixtures are Pi-shaped — `runtime/tests/exec_subprocess.rs`"
    outcome: waived
    reason: the exec call site shares Host::command_file_candidates with the unit-covered callers, the reviewer drove ductus exec on a claude-shaped fixture to completion, and a third fixture is new test design, not a chore
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "dead: ductus-procedure's edge case for a first run with only one supported agent cannot arise while the registry holds two — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: a conditional that holds again whenever the registry has one agent, and 012's spec states the same edge case, so removing it would sweep 012's contract for no behavior change
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "claim: Per-file resolution and the markdown-only fallback never mention tool-search-line, so a claude command copy made by that fallback lacks the ToolSearch line every claude entry carries — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: spec 080's surface and a rare fallback; stating the fallback for the preamble line is design work, not a mechanical edit
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "bug: install.sh writes the bootstrap with cp from a mktemp file, so .claude/commands/ductus.md and .pi/prompts/ductus.md land at mode 0600 while the settings seed lands at 0644 — `install.sh`"
    outcome: waived
    reason: pre-existing, and cosmetic since git records only the exec bit and the file holds no secret; changing the installer's write path is an acquisition change, not a chore
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "test: Family 17's pi branch, the flat prompts/<ns>-<name>.md scan, has no case in test-host-namespace-parity.sh, which drives only the claude layout — `scripts/audit/host-namespace-parity.sh`"
    outcome: waived
    reason: a new test case, not a chore; the branch was proven by hand on a renamed pi prompt set, and the family fails its other mutations
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "claim: 071's spec and plan say nothing of the remove-legacy-constitutions-region migration task 23 shipped, nor of the template and procedure files tasks 20 to 24 changed — `specs/071-command-reference-split/plan.md`"
    outcome: waived
    reason: tasks 20 to 24 dispositioned findings outside the split, so the plan records the split's design without them; migrations.toml and the commits record what shipped
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: 071's analysis.md is stale, analyzed against cde884d0 with digests that no longer match — `specs/071-command-reference-split/analysis.md`"
    outcome: waived
    reason: "a regenerated record: this close's analysis replaces it whole, and nothing durable lives in it"
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "claim: adopter-generator-promotion counts four framework/bootstrap/configure/*.md permission blocks, and two configure files exist since the agent removal — `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md`"
    outcome: waived
    reason: an account of what that change touched when four configure files existed; rewriting it would falsify the record
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "claim: shared-constitution-references-are-judged's Behavior is unimplemented: markers_identifiers still pushes the basename, and a test locks it — `specs/067-analyze-and-audit-checks/scenarios/shared-constitution-references-are-judged.md`"
    outcome: waived
    reason: the scenario is pending, with three open questions and 067 in-progress; code that has not yet met a requirement under open questions breaks no contract, and 067's task and Open Questions hold that work
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "reuse: main.rs and the interpreter keep separate hand-maintained per-primitive lists — `runtime/src/main.rs`"
    outcome: waived
    reason: flat registries pinned by set-equality tests, a design .ductus/docs/agents/developing-ductus.md records
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "contract: the two flattened Option result halves allow neither or both; an untagged enum would keep exactly one — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: rmcp refuses a tool output schema that is not an object, and an untagged enum's root is anyOf; .ductus/docs/agents/developing-ductus.md records the flattened-halves design
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "convention: Adding or removing a runtime primitive names seven sites but neither /configure step 4 retired list, and no check catches a removed tool missing from them — `.ductus/docs/agents/developing-ductus.md`"
    outcome: waived
    reason: "the pipeline's own maintainer procedure: a doc line would rest the next removal on the remover's memory, and a check deriving the retired entries is new design, not a chore"
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "contract: whether the per-agent configure commands carry the host-integration pointer is undecided — `framework/bootstrap/configure/claude.md`"
    outcome: waived
    reason: a design question the configure files leave open; no rule or 071 contract requires the pointer there, and lint-tool-coverage examines framework/commands/ alone
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "concurrency: this repository's pre-commit hook stages generator outputs regenerated from the working tree, sweeping another session's in-flight files into a commit — `.githooks/pre-commit`"
    outcome: waived
    reason: 017's task 45 and its scenario a-commit-stages-only-its-own-generated-files hold this as pending work, as AGENTS.md's gotcha says, and this window did not change the staging
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: v0.28.0 cannot be walked at its tag, and AGENTS.md placeholders an old release left literal are never repaired — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: observations about the bootstrap's own machinery, none caused by this window and none blocking it
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "other: the bootstrap's archive-fetch curl carries no timeout and no --proto or --tlsv1.2, where its other curls now carry all three — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "pre-existing: the bootstrap's curls are steps the host agent runs, and changing the archive fetch is an acquisition change, not a chore"
    decided-at: 2026-11-11T13:40:45Z
    decided-by: rule
  - key: "convention: `framework/reference/analyze.md`'s Unexamined targets closed-set paragraph names two classes of reasons and does not place `constitution-unresolved`, which `write-analysis` derives — `framework/reference/analyze.md`"
    outcome: waived
    reason: the bullet beside the paragraph says write-analysis records that reason itself and a caller never passes it, the two classes are the reasons a caller passes, and write-analysis refuses it as a caller's reason (a_reason_outside_the_closed_set_is_refused)
    decided-at: 2026-11-20T01:07:54Z
    decided-by: rule
  - key: "other: the plan's Bytes loaded per invocation table was measured at c9b2155c and no longer equals the current byte counts after the 2026-11-19 section deletions — `specs/071-command-reference-split/plan.md`"
    outcome: waived
    reason: a dated measurement that states its sha and method, as AC20 asks, and not a claim about the current tree; re-measuring would only date a new one
    decided-at: 2026-11-20T01:07:54Z
    decided-by: rule
---

# Review — 071-command-reference-split

## Summary

Reviewed 071's window from `dec1b0b6` (the 2026-11-19 reopening; the base `compute-review-scope` derived, no override): 0 MUST and 0 SHOULD violations, 24 of 24 criteria judged met, 12 observations (6 fixed in the run, 6 waived: 2 new by the rule and 4 stored decisions carried). Three readers split the passes: the security pass over the three `security-*.md` files; the reuse, quality, efficiency and simplicity passes over the other nine rule files, `AGENTS.md` Gotchas through Design Principles and `specs/system.md`, with a verdict on each stored decision naming an in-scope file (all four still stand); and the criteria step, with AC18 and AC19 judged by running the whole local gate (audit, the three generators, derive-dependencies and derive-references, markdownlint, the lint and test scripts, shellcheck, cargo fmt, clippy and test). The readers found the specify command's `--branch-id` text false and its step-6 pointer stale, the changelog's 2-unreached figure stale, task 26's five-section count unfounded, and 071's AC2, AC12 and Cross-file pointers stale against 067's decision (j) and the 2026-11-19 deletions; all were fixed in the run (commit `fix(071): specify names the empty-identifier refusal`), one fix round was re-read clean, and AC2 and AC12 were re-judged met against the corrected text. Read whole: the four command sources, `framework/reference/review.md`, `plan.md`, `specify.md`, `consolidate.md`, `status.md` and `fold.md`, the three goldens, and 071's `spec.md`, `tasks.md` and scenario. Not counted in `examined` and named in `unread`: `framework/reference/analyze.md` (64 KB, read through every hunk of its window diff and the sections they sit in, not whole), `runtime/CHANGELOG.md` (601 KB, its whole window diff read, the file not), and the 15 installed copies under `.claude/commands/ductus/`, `.pi/prompts/` and `.ductus/reference/`, each generated from a source read as above, with `scripts/gen-claude-commands.sh --check` reporting them in sync. Nothing was waived that the owner's 2026-11-19 decisions settle, and no finding was left open.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

24 of 24 acceptance criteria judged met.

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

- AC1 — met: grep -rn '^## Markdown-only reference' framework/commands returns nothing, and no case-insensitive 'markdown-only' remains in any of the 17 framework/commands/*.md
- AC2 — met: after the parent's correction of its text: the 13 `framework/reference/<command>.md` files each open with a Where a named section lives paragraph, five hold that paragraph alone, and `scripts/audit/step-reference-integrity.sh` (Family 34) reports 74 sections, 0 unreached
- AC3 — met: python scan of framework/commands, framework/reference and `framework/bootstrap/ductus.md`: 120 '**§X** in `.ductus/reference/<cmd>.md`' pointers, 0 name a heading absent from that file; no 'reference below' wording remains in a command but link.md's own in-file Procedure
- AC4 — met: grep of framework/reference/*.md finds no {cli-config-dir}, no agent dir, and the only $ARGUMENTS (`analyze.md` Command frontmatter completeness bullet) documents the token under an audit:ignore-placeholders marker; `scripts/audit/placeholder-roundtrip.sh` exits 0 clean and exits 1 with a finding for each of {cli-config-dir}, .claude/ and $ARGUMENTS appended to a reference file (driven in a clone)
- AC5 — met: `framework/bootstrap/ductus.md` Shared Files update table has 12 `framework/reference/<cmd>.md` rows to `.ductus/reference/<cmd>.md` (no audit); `scripts/audit/installer-command-parity.sh` exits 0, and fails when a row is deleted (no Shared Files row) and when a row's source is renamed (no source file) -- both driven in a clone
- AC6 — met: `ductus.md` Instructions step 1 lists .ductus/reference/ in enforce-directories, step 8 runs enforce-manifest per directory, and the Reference cleanup paragraph under Shared Files deletes any *.md no row produces unless in pinned.files; prune behaviour itself is proved by AC15's fixture (not judged here)
- AC7 — met: cmp `framework/bootstrap/ductus.md` `framework/bootstrap/govern.md` exits 0; govern.md is tracked
- AC8 — met: ls .claude/commands/ductus and .pi/prompts hold only the 18 commands (and ductus-* prompts), no reference copy; gen-claude-commands.sh writes references only to REF_DEST=.ductus/reference (its temp copy of references for apply-manifest is in a scratch framework/reference tree) and the manifest rows' destinations are .ductus/reference only
- AC9 — met: `scripts/gen-claude-commands.sh` --check (DUCTUS_BIN set) exits 0 clean, exits 1 on a modified copy ('drift in log.md'), a deleted copy ('missing in .../.ductus/reference: log.md') and an orphan ('orphan ... zzz.md'); write mode removed the orphan; substitute_shared substitutes {project} (22 occurrences in the source, 0 in `.ductus/reference/analyze.md`); .githooks/pre-commit generated_outputs lists ^\.ductus/reference/ (no test of the hook line)
- AC10 — met: `scripts/audit/marker-list-parity.sh` reads ANALYZE_MD=`framework/reference/analyze.md` and exits 0; with the 'closed phrases is exempted whole' sentence reworded in a clone it exits 1 ('could not locate the non-assertion marker restatement')
- AC11 — met: `runtime/src/primitives/check_corpus_links.rs` exclusion is gated on generates_references (framework/reference is a real dir); tests generated_reference_copies_are_excluded_and_the_rest_of_ductus_dir_is_not and installed_reference_files_are_checked_where_there_are_no_sources pass (90 lib tests ok), and mutating the gate to always-exclude fails the installed test while never-exclude fails the generated test (both also fail a_reference_sources_directory_linked_outside_is_not_probed)
- AC12 — met: after the parent's correction of its last clause: `judge()` in `runtime/src/primitives/resolve_anchor.rs` and its tests a_line_naming_two_documents_is_judged_against_both and every_shipped_pointer_into_a_reference_file_resolves pass, and Family 39 (`scripts/audit/shipped-anchor-resolution.sh`) reports 0 unresolved
- AC13 — met: `framework/bootstrap/ductus.md` carries the pointer 'item 5 of **§Resolve target and scope** in `.ductus/reference/review.md`' (item 4 of Collect Project Inputs, config-key docs, comment near line 648); `framework/reference/review.md` has heading '## Resolve target and scope' whose item 5 is 'Discover rule files'
- AC14 — met: `.github/workflows/runtime.yml` lists 'framework/reference/*.md' directly after 'framework/commands/*.md' under both on.pull_request.paths and on.push.paths (the only two places commands/*.md appears)
- AC15 — met: `runtime/tests/parity.rs` assert_reference_installed_and_pruned reads `.ductus/reference/specify.md` (contains /acme:specify, no {project}) and runs enforce-manifest --directory project/.ductus/reference with expected [`specify.md`], asserting stale.md removed and `specify.md` kept; in a scratch copy with its own CARGO_TARGET_DIR the test passes, and fails when the fixture row is removed (read `specify.md`: No such file), when keep-literals suppresses substitution ('must install with {project} -> acme'), and when the enforce-manifest call is neutralised ('must remove a reference file no row produces')
- AC16 — met: `framework/constitution.md` canonical-sources row 'Pipeline command behavior' reads '`framework/commands/*.md` (...), with its reference in `framework/reference/*.md`'; the <!-- §runtime-host-integration --> section (Host integration) says a command's reference keeps what a step does with the result, host judgment and policy, installed at `.ductus/reference/<command>.md` and read only where a step points
- AC17 — met: `.ductus/docs/agents/developing-ductus.md` 'A command with a reference takes two more' paragraph names `framework/reference/<name>.md` and its row in `framework/bootstrap/ductus.md` Shared Files, `framework/reference/<name>.md` -> `.ductus/reference/<name>.md`, which Family 16 fails a reference without
- AC18 — met: `scripts/audit/run-all.sh` exits 0 at the worktree head, `scripts/gen-claude-commands.sh --check`, `scripts/gen-configure-mcp.sh` and `scripts/gen-help-tables.sh` leave the tree unchanged, and `derive-dependencies` and `derive-references` report no drift
- AC19 — met: `npx markdownlint-cli2` 0 issues over 884 files, every `scripts/lint-*.sh` and `scripts/tests/*.sh` exits 0, `shellcheck -S warning` over the tracked shell set 0, and under `runtime/` cargo fmt --check, clippy --release --all-targets --locked -D warnings and cargo test --release --locked all exit 0
- AC20 — met: `plan.md` 'Bytes loaded per invocation, measured after the split' states sha c9b2155c, date and method (git show c9b2155c:<path> | wc -c); re-measured all 13 command rows (after) and 13 reference rows with that method: every figure equal, totals 175,387 and 199,406, 255,509 fewer, percentages recomputed (76.3 62.9 70.5 86.9 38.6 45.4 42.4 43.0 10.1 69.0 55.1 0.3 14.8, total 59.3); before column matches the spec's 430,896 (see observation: the table is point-in-time)
- AC21 — met: `scripts/audit/audit-family-parity.sh` sets AUDIT_MD=`framework/reference/audit.md`; run in a scratch copy: clean (38 registered vs 38 documented), with the numbered entries removed it emits 'no documented families extracted' (rc 1), with the file removed 'not found' (rc 1)
- AC22 — met: each of the 13 `framework/commands/<cmd>.md` with a reference has exactly one line after the Instructions blockquote 'This command's reference is `.ductus/reference/<cmd>.md`. A section named here ... that has no heading in this file is there'; amend, target, link, help have no .ductus/reference/ mention (wording says 'its reference', not literally 'markdown-only reference'; substance met)
- AC23 — met: searched 590 tracked files of framework/, scripts/, runtime/ (src, tests, fixtures), .github/, docs/, README.md, AGENTS.md (`runtime/CHANGELOG.md` read for hits too, historical entries only) with python3 re for 'markdown-only', 'Markdown-only reference', '<cmd>.md ... reference/section', 'reference ... command file', 'reference below/above'; no live claim places a command's reference in its command file (sibling-coupling.sh, audit-family-parity.sh, marker-list-parity.sh, placeholder-roundtrip.sh, check_artifacts.rs docs and link.md's in-file Procedure section all read correctly; CHANGELOG 7187 is a past entry)
- AC24 — met: `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md` links `../../071-command-reference-split/spec.md` and has one bullet each for Families 4, 16, 18, 28, 39 (checked against placeholder-roundtrip.sh, installer-command-parity.sh, marker-list-parity.sh, audit-family-parity.sh, shipped-anchor-resolution.sh); 067 scenarios generated-reference-copies-are-excluded (check-corpus-links) and pointers-into-shipped-files-are-judged (resolve-anchor) each link `../../071-command-reference-split/spec.md`

## Observations

- convention: `framework/commands/specify.md` step 4 sent the reader to step 6 for the identifier actually used, where step 5 (`create-feature`) reports it — `framework/commands/specify.md` — **fixed** (decided by the rule)
- convention: the specify flags table and step 4 said a `--branch-id` is never refused, though `create-feature` refuses one that sanitizes to nothing, and the deleted specify reference section that said so moved into no step — `framework/commands/specify.md` — **fixed** (decided by the rule)
- convention: `runtime/CHANGELOG.md`'s 071 entry counted 2 reference sections unreached by design, where `check-step-references` reports 4 exempt and 0 unreached — `runtime/CHANGELOG.md` — **fixed** (decided by the rule)
- convention: task 26 counted five restating sections of the specify reference, where all nine of its sections were deleted and no basis for five is stated — `specs/071-command-reference-split/tasks.md` — **fixed** (decided by the rule)
- convention: 071's spec still said a line naming several documents stays unjudged, which 067's later decision (j) overturned, in its Cross-file pointers paragraph and in AC12 — `specs/071-command-reference-split/spec.md` — **fixed** (decided by the rule)
- convention: 071's AC2 and its What moves list still said every reference holds its moved sections and named Idempotency, after the 2026-11-19 deletion of unreached sections — `specs/071-command-reference-split/spec.md` — **fixed** (decided by the rule)
- convention: `framework/reference/analyze.md`'s Unexamined targets closed-set paragraph names two classes of reasons and does not place `constitution-unresolved`, which `write-analysis` derives — `framework/reference/analyze.md` — **waived**: the bullet beside the paragraph says write-analysis records that reason itself and a caller never passes it, the two classes are the reasons a caller passes, and write-analysis refuses it as a caller's reason (a_reason_outside_the_closed_set_is_refused) (decided by the rule)
- other: the plan's Bytes loaded per invocation table was measured at c9b2155c and no longer equals the current byte counts after the 2026-11-19 section deletions — `specs/071-command-reference-split/plan.md` — **waived**: a dated measurement that states its sha and method, as AC20 asks, and not a claim about the current tree; re-measuring would only date a new one (decided by the rule)
- other: the tasks 43 and 44 changelog entry does not name BE-API-013's explicit bar on a credential or PII in a link sent to a person — `runtime/CHANGELOG.md` — **waived**: Not mechanical, and not untrue: BE-API-013's earlier Not-flagged clause already kept a URL sent in an email or another message as its own, so the entry, which lists new obligations, omits a clarification rather than a change. (decided by the rule)
- other: a security-frontend.md copied from main between afb8a865 and 0481c218^ hands over only a server the project serves, so it keeps an authorization-server library's redirect_uri case beside SEC-AUTHN-004 — `runtime/CHANGELOG.md` — **waived**: A contrived input: both windows are unreleased main, the second about an hour long, and a project pinning such a copy re-syncs it as the changelog's pinned-copy sentence already says. (decided by the rule)
- edge: an expired waiver on a low-confidence finding re-counts toward low-confidence, not its severity's count — `framework/reference/review.md` — **waived**: contrived: waiving a finding that counts toward no gate has no effect worth documenting, and step 1's exclusion stays true
- other: fix rounds driven by mechanical NOTEs have no length bound — `framework/reference/review.md` — **waived**: The Behavior permits NOTE rounds and AC6's budget counts re-reads with a MUST or SHOULD; a bound on NOTE rounds is a requirement decision. (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/analyze.md`
- `.claude/commands/ductus/clarify.md`
- `.claude/commands/ductus/consolidate.md`
- `.claude/commands/ductus/fold.md`
- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/prune.md`
- `.claude/commands/ductus/target.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/clarify.md`
- `.pi/prompts/ductus-analyze.md`
- `.pi/prompts/ductus-clarify.md`
- `.pi/prompts/ductus-consolidate.md`
- `.pi/prompts/ductus-fold.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-prune.md`
- `.pi/prompts/ductus-target.md`
- `framework/commands/analyze.md`
- `framework/commands/clarify.md`
- `framework/commands/consolidate.md`
- `framework/commands/fold.md`
- `framework/commands/help.md`
- `framework/commands/prune.md`
- `framework/commands/target.md`
- `framework/reference/audit.md`
- `framework/reference/clarify.md`
- `runtime/Cargo.toml`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/io.rs`
- `runtime/src/main.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_inbox.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_command_flags.rs`
- `runtime/src/primitives/check_corpus_links.rs`
- `runtime/src/primitives/check_orphaned_references.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/check_step_references.rs`
- `runtime/src/primitives/classify_spec_edit.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/derive_routing_candidates.rs`
- `runtime/src/primitives/merge_managed_block.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/prune_plan.rs`
- `runtime/src/primitives/prune_tasks.rs`
- `runtime/src/primitives/read_spec.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/rewrite_spec_links.rs`
- `runtime/src/primitives/rule_sections.rs`
- `runtime/src/primitives/spec_links.rs`
- `runtime/src/primitives/tool_loading.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/src/schema/descriptions.rs`
- `runtime/src/schema/primitives.rs`
- `runtime/tests/bounded_list_gates.rs`
- `runtime/tests/check_step_references_cli.rs`
- `runtime/tests/derive_dependencies_golden.rs`
- `runtime/tests/derive_references_golden.rs`
- `runtime/tests/exec_stderr_gone.rs`
- `runtime/tests/mcp.rs`
- `scripts/audit/README.md`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/installed-file-modes.py`
- `scripts/mutation/owner-reviews-067.py`
- `scripts/mutation/prune-open-region.py`
- `scripts/mutation/prune-review-fixes-041.py`
- `scripts/mutation/relocate-merge-children-057.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/review-022-task-44.py`
- `scripts/mutation/review-read-credit.py`
- `scripts/mutation/reviews-follow-what-changed.py`
- `scripts/mutation/rewrite-spec-links.py`
- `scripts/mutation/scope-attribution.py`
- `scripts/mutation/stderr-warnings-022.py`
- `scripts/mutation/step-references-bounded.py`
- `scripts/mutation/step-references-reach.py`
- `scripts/tests/test-step-reference-integrity.sh`
- `specs/020-code-review/analysis.md`
- `specs/020-code-review/data-model.md`
- `specs/020-code-review/review.md`
- `specs/022-deterministic-runtime/data-model.md`
- `specs/022-deterministic-runtime/scenarios/a-closed-stdout-is-a-quiet-exit.md`
- `specs/022-deterministic-runtime/scenarios/append-inbox-comment-aware-write.md`
- `specs/022-deterministic-runtime/scenarios/dashboard-primitive.md`
- `specs/022-deterministic-runtime/scenarios/host-protocol-conformance.md`
- `specs/022-deterministic-runtime/scenarios/installed-files-are-not-group-writable.md`
- `specs/022-deterministic-runtime/scenarios/scenario-open-question-signal.md`
- `specs/022-deterministic-runtime/scenarios/skipscanner-inline-code-exemption.md`
- `specs/022-deterministic-runtime/scenarios/spec-side-parser-hardening.md`
- `specs/022-deterministic-runtime/spec.md`
- `specs/022-deterministic-runtime/tasks.md`
- `specs/026-framework-self-audit/analysis.md`
- `specs/026-framework-self-audit/plan.md`
- `specs/026-framework-self-audit/review.md`
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/scenarios/an-open-fenced-block-or-comment-is-not-examined.md`
- `specs/041-task-pruning/spec.md`
- `specs/041-task-pruning/tasks.md`
- `specs/051-branch-scoped-spec-numbering/analysis.md`
- `specs/051-branch-scoped-spec-numbering/plan.md`
- `specs/051-branch-scoped-spec-numbering/review.md`
- `specs/051-branch-scoped-spec-numbering/scenarios/rewrites-preserve-line-endings.md`
- `specs/051-branch-scoped-spec-numbering/spec.md`
- `specs/051-branch-scoped-spec-numbering/tasks.md`
- `specs/055-shared-constitution/plan.md`
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
- `specs/067-analyze-and-audit-checks/analysis.md`
- `specs/067-analyze-and-audit-checks/data-model.md`
- `specs/067-analyze-and-audit-checks/plan.md`
- `specs/067-analyze-and-audit-checks/review.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-none-list-item-is-no-open-question.md`
- `specs/067-analyze-and-audit-checks/scenarios/a-plan-heading-is-matched-without-regard-to-case.md`
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-results-are-bounded.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-unstaged-drift-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md`
- `specs/067-analyze-and-audit-checks/scenarios/shared-constitution-references-are-judged.md`
- `specs/067-analyze-and-audit-checks/scenarios/traverse-deps-cycle-check.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/079-review-and-analysis-work-converges/analysis.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/review.md`
- `specs/079-review-and-analysis-work-converges/scenarios/the-excluded-section-lists-paths-only.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/080-deterministic-runtime-efficiency/analysis.md`
- `specs/080-deterministic-runtime-efficiency/plan.md`
- `specs/080-deterministic-runtime-efficiency/review.md`
- `specs/080-deterministic-runtime-efficiency/scenarios/a-step-gates-on-a-bounded-list-by-its-count.md`
- `specs/080-deterministic-runtime-efficiency/spec.md`
- `specs/080-deterministic-runtime-efficiency/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`

## Unexamined governance

*None.*
