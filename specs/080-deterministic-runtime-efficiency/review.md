---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-11T14:41:07Z
reviewed-against: 3cf9955b3abfbf21338d629eeba187474f49ba7e
diff-base: de56c366b4874cbd80409262252acdb81cba5d9e
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 70
scope: 76
excluded: 157
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 0
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 08e290ce310f85f4ebea70052dbce6bcfbb9d85f666daf6e2dd73ee6db22cb14
  scenarios/configure-types-no-permission-entries.md: 6c764c4d44c20f0b7d88582dbd52de2d9c1efd915080813f41eb858e16ca7313
  scenarios/groom-derives-routing-candidates.md: 516690b45261a38cad315305e8b1ca19fa51a072137a7bfbb1a12c9217fe42da
  scenarios/help-answers-with-no-model-turn.md: 8962bafa8cbf7224dc43d5d73a0507d3bd1088492c5977c287eac52acff33189
  scenarios/one-search-covers-referenced-sections.md: 8f024532fe05b863b06fdb6facb8cae8ebe1e979dbfc8fd28035b6336fec97c3
  scenarios/reference-reads-follow-the-cited-sections.md: 66a004c23e55c8e1a4823a5fa13370220af42c42563194fc693c7067998d9c5f
  scenarios/rules-load-without-shell-reads.md: 052d22a0ed61649128bdada4bf6cef556aba1568aa64f6d6055eb5dbe84b92dd
  scenarios/the-search-goes-out-with-the-first-call.md: fb2cfb2453ca185c97d8fd9e81daad0c09fe92280d925c2ee2b52d6273dc000e
blocking: false
dispositions:
  fixed: 1
  deferred: 1
  waived: 68
  undispositioned: 0
decisions:
  - key: "other: with a pinned `target.md`, a tool can load from neither side when the session's `cli-config-dir` names another agent — `runtime/src/primitives/apply_manifest.rs`"
    outcome: waived
    reason: "Inside the two-agent-CLI bound the constitution's §concurrent-features names: the runtime reads command files from the agent `/ductus` last recorded. Within one agent's layout every case agrees."
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "convention: resolve-anchor's `heading_lines` and tool_loading's `atx_headings` read ATX headings by different rules — `runtime/src/primitives/resolve_anchor.rs`"
    outcome: waived
    reason: No shipped command, reference or configure source holds a heading the two read differently, and unifying them changes every reader of one, which is not a mechanical edit.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "other: no test holds each bounded member's top-level `-count` keys to the lists its file holds, which the walker's halt reads — `runtime/tests/mcp.rs`"
    outcome: waived
    reason: Every member's counts name a list its file holds today, checked struct by struct; a guard for a future member is new test design, not a chore.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "other: read-section's file form holds each section's text as one line, so a host reading by line offset cannot split a section — `runtime/src/primitives/read_section.rs`"
    outcome: waived
    reason: No shipped section reaches the default read size (the largest is 7,934 bytes), and a read size below one section is a host configuration this spec does not size for.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "other: past the bound the MCP response carries the result twice, as text (49,965 bytes for render-status on 700 specs) and as `structuredContent` (53,614) — `runtime/src/mcp/server.rs`"
    outcome: waived
    reason: Claude Code hands the model one copy, as every `mcp__ductus__` result in this run shows; the bound measures the result, and changing how the server's JSON wrapper serializes is 022's surface and not a mechanical edit.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "doc: 080's plan.md Affected Files omits files 080's commits changed, among them read_spec.rs, payload.rs, bounded_lists.rs and the bounded members — `specs/080-deterministic-runtime-efficiency/plan.md`"
    outcome: waived
    reason: The plan's bounded-results decision covers those files, which rows to add is a judgment, and check-artifact-size's advisory on this plan asks for finished rows to be trimmed rather than added.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "doc: 023's AC36 and AC38 count nine retired entries where there are 13, and AC37 says the revoke conflict is raised before any filesystem read — `specs/023-govern-refinement/spec.md`"
    outcome: waived
    reason: The counts were false at 150b4dc1 too, and AC37 was never strictly true; correcting a done spec's criteria reopens it, a reopen that is 023's to spend.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "other: no 080 artifact records that the next release must be a minor version, since 080 changes tool results — `specs/022-deterministic-runtime/data-model.md`"
    outcome: waived
    reason: The release step reads 022's versioning section when it chooses the version, so nothing of 080's holds it.
    decided-at: 2026-11-10T20:37:33Z
    decided-by: rule
  - key: "other: fix round 1's re-read found that `bounded()`'s watchdog can leave its `sleep` running when the call ends before the watchdog sets its trap — `scripts/gen-claude-commands.sh`"
    outcome: waived
    reason: "Nothing hangs: an orphaned sleep holds only /dev/null and exits at the bound, 3 in 200 calls in the re-read's count, and closing the window needs a second signal handshake, not a mechanical edit."
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: fix round 1's re-read found that the dashboard test drives only `session-unreadable` past the bound, so removing the `session-notices` move fails no test — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: Both lists move by the same branch, which the test drives for one of them; a fixture that puts a session notice past the bound needs co-targeting session records, which is new test design, not a chore.
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: fix round 2's re-read found the unrecognized-suffix notice and the `loading rule files:` line printing a rule file's name unflattened — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: waived
    reason: Both lines predate this spec and print the names of files the project put in its own rule directory; flattening every notice name is a sweep beyond 080's change, not its fix.
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: fix round 2's re-read found no test feeding the non-file notice a name holding a line hazard — `runtime/src/primitives/discover_rule_files.rs`"
    outcome: waived
    reason: The flattening is `flatten_line`'s, whose own tests and the pipeline view's cover it; a symlink named with a newline is a new fixture, not a chore.
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: fix round 2's re-read found no generator-test case for the all-zero timeout refusal — `scripts/tests/test-gen-claude-commands.sh`"
    outcome: waived
    reason: The refusal was driven by hand for 0, 00 and abc, each refused naming the variable, and 05 accepted; a case per refused value is new test design, not a chore.
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: fix round 2's re-read found that a timeout of 20 or more digits overflows `[`'s comparison and is let through, as it was before the round — `scripts/gen-claude-commands.sh`"
    outcome: waived
    reason: A contrived value a maintainer sets by hand; `sleep` then rejects it, so every runtime call is stopped at once and the generator fails, which is no silent pass.
    decided-at: 2026-11-10T21:33:12Z
    decided-by: rule
  - key: "other: for a namespace holding a tab or a line break, Pi's bridge respells the status view with render-help's unflattened namespace, so the view's flattened hints keep their colon form — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: Such a namespace names no command on any host, since Pi registers `{project}-status` from it and every installed command file is named by it, so no respelled hint could run; aligning the bridge would change 064's verbatim bridge, not a mechanical edit.
    decided-at: 2026-11-11T00:12:04Z
    decided-by: rule
  - key: "other: a `|` in a slug, status or tag adds a cell to the model path's markdown table, since view_text replaces only control characters and tabs — `runtime/src/primitives/pipeline_view.rs`"
    outcome: waived
    reason: No loaded rule covers a printable character, the pre-080 dashboard's table rendered the same cells raw (150b4dc1), and escaping each cell changes the table the model path re-types, which is not a mechanical edit.
    decided-at: 2026-11-11T00:12:04Z
    decided-by: rule
  - key: "other: the markdown-only reference-cleanup prose differs from enforce-manifest at the edges (links never removed, dot-prefixed .md matched) — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "contrived: nobody writes either into a ductus-owned directory (owner decision, 2026-10-26)"
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: check-corpus-links reads framework/reference/'s presence as a framework checkout, which an adopter's own directory of that name would match — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: waived
    reason: "contrived: it costs only the repository-scope check of shipped copies, which an adopter's spec-corpus hook never runs (owner decision, 2026-10-26)"
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: a greenfield --ref run whose installed ductus.md is stale writes the config before aborting, so its re-run is not greenfield and prompts — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: pre-existing and not made worse; the documented install path places a current bootstrap
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: step 4's per-agent apply-manifest loop sits in one walker step whose context carries one map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the real bootstrap is walked by the host, as step 8's per-directory loop already is
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: the greenfield rule's newest-entry id is undefined when the active registry is empty — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: "contrived: the registry is never empty while migrations exist, and 027's AC22 already writes nothing then"
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: the corpus scope matches `.md` case-sensitively where the repository scope does not — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: waived
    reason: Predates 059; which files each scope examines is `check-corpus-links`' own contract, and changing either direction changes what the check reports.
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: tests that expect no repository assume the temp directory has no enclosing work tree — `runtime/src/primitives/check_corpus_links.rs`"
    outcome: waived
    reason: Predates 059 and is shared with other test modules; a ceiling would need process-wide environment in a parallel suite, and CI's temp directory has no enclosing work tree.
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "bug: the criterion-path grammar accepts `../` and scenario finding paths hard-code a lowercase `.md` — `runtime/src/primitives/check_artifacts.rs`"
    outcome: waived
    reason: Predates 059; both are contracts of `check-artifacts`' criterion-path and scenario families, and neither is a path conversion.
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "security: the review payload ships an in-scope file whatever its name, where the writeCode payload refuses secret-named and ignored files — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: Predates 059; the reviewing host reads the same tree with its own tools, and whether review withholds a file it is reviewing is the review payload's contract.
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "perf: `read_repo_file` canonicalizes the project root once per rule file — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: N is the rule-file count, so the cost is negligible.
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "perf: build_assess_spec_quality_request re-reads the spec file for every rule request, up to 167 times per step — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: each read is a small local file the OS caches, beside an LLM round trip per request; reading it once would thread a second value through the walker context for a gain nobody has measured
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "reuse: runtime_binary, ensure_binary_built and the envelope loop are copied across test files — `runtime/tests/exec_subprocess.rs:18`"
    outcome: waived
    reason: predates the window; consolidating test helpers is a refactor outside 048
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "efficiency: the prune-plan walk runs inline on the async worker and reads HEAD per done spec, even for specs it then drops — `runtime/src/mcp/server.rs:710`"
    outcome: waived
    reason: one HEAD read per done spec is not the unbounded revwalk the blocking pool is scoped to, and a spec it drops still needs the answer to know it can be dropped
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "tests: the macOS integration test leaks its mktemp directory if building the archive or spawning the binary panics — `runtime/tests/exec_subprocess.rs`"
    outcome: waived
    reason: it leaks only on a panic before its assertions, one empty directory under the per-user temp directory, which macOS clears
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "perf: dashboard and the --all prune walks now walk every feature directory — `runtime/src/primitives/dashboard.rs`"
    outcome: waived
    reason: measured dashboard 0.23-0.25 s, prune-plan --all 0.04 s and prune-tasks --all 0.01 s on this corpus, the walks' share at most about 6 ms; refusing the whole command is the scenario's stated edge case
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: canonicalize-and-contain logic exists in both interpreter/payload.rs and primitives/mod.rs — `runtime/src/interpreter/payload.rs`"
    outcome: waived
    reason: "the two differ on purpose: the payload readers skip a missing or outside path and give None for a non-UTF-8 name, where the check refuses with an error and judges a missing path, so one helper would carry both behaviors"
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: RecordFreshness::Undeterminable's doc names only a missing digest, where an unparseable review.md also answers it — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: "Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window."
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: the tasks 43 and 44 changelog entry does not name BE-API-013's explicit bar on a credential or PII in a link sent to a person — `runtime/CHANGELOG.md`"
    outcome: waived
    reason: "Not mechanical, and not untrue: BE-API-013's earlier Not-flagged clause already kept a URL sent in an email or another message as its own, so the entry, which lists new obligations, omits a clarification rather than a change."
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "other: a security-frontend.md copied from main between afb8a865 and 0481c218^ hands over only a server the project serves, so it keeps an authorization-server library's redirect_uri case beside SEC-AUTHN-004 — `runtime/CHANGELOG.md`"
    outcome: waived
    reason: "A contrived input: both windows are unreleased main, the second about an hour long, and a project pinning such a copy re-syncs it as the changelog's pinned-copy sentence already says."
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "other: check-step-references' not-a-procedure field doc defines the set by files holding several numbered lists, though it also holds files with no step or too few — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: the primitive's schema doc, 067's surface and outside 026's code; the family's own stderr line now says not one procedure
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "other: under ductus exec a malformed shared session default halts as config-unreadable, since Host::load reads it first, so main.rs's session-unreadable branch is reached only by the process's own record — `runtime/src/main.rs`"
    outcome: waived
    reason: The branch is still reached by a malformed own record, the exec test pins the code the default halts with, and either code labels a halt that names the file.
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "test: the check_artifacts REGISTRY fixture carries a scope column the parser does not read — `runtime/src/primitives/check_artifacts.rs`"
    outcome: waived
    reason: the fixture mirrors the real file's table shapes, and the real MCP registration table carries the column
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "edge: a session or config naming a dropped host is not diagnosed: .augment still resolves through the claude-style candidate, and .opencode or .agents ends in command-not-found naming the paths tried — `runtime/src/host.rs`"
    outcome: waived
    reason: Host::load documents that a config-dir mismatch surfaces as the command-file-not-found error, which names every path tried; the resolver is generic in cli-config-dir by design, so a custom directory keeps working, and the CHANGELOG's Removed entry announces the removal
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "other: this repository's .pi/prompts/ductus-configure.md names .pi/prompts/ductus.md, which this repository does not track — `.pi/prompts/ductus-configure.md`"
    outcome: waived
    reason: the dogfooded .pi copies keep the generator honest; this repository is not a Pi adopter, and an adopter's copy has .pi/prompts/ductus.md beside it, which install.sh writes
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: pi's configure step restates /ductus's latest-release fetch without its grammar check, floor halts or DUCTUS_REPO notice — `framework/bootstrap/configure/pi.md`"
    outcome: waived
    reason: configure needs only the ref to fetch the bridge at; the halts guard /ductus's own writes, a bad ref fails the bridge fetch with curl's error, and the bounds are named rather than copied
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: the template has frontmatter, so the walk skips it with a report line and writes nothing
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: the pre-run status's claim that nothing earlier in the run writes under specs/ holds only if Pre-run Migrations run before walker step 4's Shared Files apply-manifest, which ductus.md §Instructions does not state — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: already recorded as 056's scenario the-walker-places-every-archive-half-section, whose task gates 056's done; the old fresh status refused the same way, so this is no regression
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "edge: Pre-run Migrations step 1's abort does not say whether the section or the run stops — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: "contrived: it needs a malformed archive, and the pre-run status is taken before step 1, so either reading is safe"
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "edge: a file the adopter had dirtied that a migration also rewrote is named only by the refusal, not as changed by a migration — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: "decided in the plan and in the scenario's first edge case: the file is the adopter's to commit, and the refusal names it"
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: the repeated refusal ends with The rest of this run continues after the run has finished — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: wording only; the sentence is still true of the run it describes
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: step 2's parenthetical in `.ductus/reference/analyze.md` §Rules (blocking and advisory) can read as describing the checks rather than naming the heading — `framework/bootstrap/ductus-procedure.md:198`"
    outcome: waived
    reason: "Style only: the checks applied are the same either way, and Family 39 needs the full heading (owner, 2026-11-03, option a)."
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: a Windows absolute document path already holds backslashes, and the space escaping may misread them — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: resolve-constitutions writes the document path through rel_path, which replaces every backslash with a forward slash, so the block line holds no backslash but the spaces the bootstrap escapes
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: framework/bootstrap/ductus.md §Project Configuration says the description renders wherever ductus names a source (fix rounds 1 and 3's NOTEs) — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: A comment the installer writes into adopters' config; correcting it is a bootstrap change, which this repository verifies only with a real adopter run, out of proportion to one clause; the data model states which surfaces carry the description.
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "other: main's bootstrap calls `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, so a `--ref=main` run errors at that step and falls back to the unflagged call until a release moves the pin — `framework/bootstrap/ductus.md:924`"
    outcome: waived
    reason: "An observation about the pipeline's own machinery made while verifying framework work (AGENTS.md): it concerns 055's step, not 077's, blocks nothing here, and ends when the next release tag moves the runtime pin to one carrying the flag."
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: an adopter with an installed .augment, .agents or .opencode tree gets no notice from /ductus, nothing removes it, and the managed .gitignore block no longer ignores it — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the owner dropped the three hosts as a breaking change; the CHANGELOG's Removed entry tells such an adopter to remove the tree by hand, the runtime refuses writes there by the owner's exception, and detecting a dropped host's tree would be new bootstrap behavior rather than a fix to this change
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "other: the swept migrations narrow three procedures to Claude, so a dropped-host adopter far enough behind loses their rewrites silently — `framework/migrations/runtime-store-path.md`"
    outcome: waived
    reason: no entry was removed or renumbered, so no last_applied names a missing id; claude and pi adopters run the same steps as before, and the hosts were dropped
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change"
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "other: §spec-lifecycle resolves the plan rule against the shared test only by sentence order — `framework/constitution.md`"
    outcome: waived
    reason: coherent as read, the rule sitting between the three cases and Anything else, and the section is held at its byte ceiling
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md`"
    outcome: waived
    reason: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "convention: docs/shared-constitution.md's authoring guideline 'Anything that loosens a framework rule' leaves out the exception (fix round 1's NOTE) — `docs/shared-constitution.md`"
    outcome: waived
    reason: An authoring guideline for shared sources; the exception is a framework rule's own opt-in, which a shared author meets in that rule's text.
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "security: `audit.md` step 1 and `analyze.md`'s drift check send the agent to `.ductus/bin/ductus run-generator`, which the seeded `Bash(.ductus/bin/ductus *)` allow runs with no prompt until 022 task 18 narrows it — `framework/commands/audit.md:31`"
    outcome: deferred
    target: specs/022-deterministic-runtime/tasks.md
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "reuse: Family 2's generated-block extraction repeats the marker literals that `scripts/tests/test-gen-configure-mcp.sh` and the generator use — `scripts/audit/manifest-parity.sh`"
    outcome: waived
    reason: scripts/tests has no lib.sh to share a helper through, and check-zero's gen-configure-mcp.sh --dry-run already fails when the markers drift
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "note: a ceiling over 19 digits passes the digit guard and makes `[ -gt ]` error — `scripts/lint-governing-size.sh`"
    outcome: waived
    reason: "contrived: a ceiling that large admits any file, so the silent pass it allows cannot hide an oversized one"
    decided-at: 2026-11-11T14:41:07Z
    decided-by: andy@stone.dev
  - key: "reuse: Family 15 adds another hand-rolled scan of the §Agent Registry table beside those of installer-registry-parity.sh, host-namespace-parity.sh, placeholder-roundtrip.sh and permission-entry-shape.sh, while lib.sh's agent_config_dirs reads one column of it — `scripts/audit/runtime-probe-parity.sh`"
    outcome: waived
    reason: Each family reads a different part of the table and lib.sh's helper returns one column, so a shared row reader is a refactor of four other families' code, not a mechanical fix.
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "perf: install.sh validates the agent key only after the latest-release lookup and the bootstrap download — `install.sh`"
    outcome: waived
    reason: the order predates the removal, the arm still halts before writing anything, and moving the check is a behavior change with its own test
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "test: ductus exec resolution of a claude-style commands/{project}/ copy lost its only subprocess fixture with exec-auggie, so both exec fixtures are Pi-shaped — `runtime/tests/exec_subprocess.rs`"
    outcome: waived
    reason: the exec call site shares Host::command_file_candidates with the unit-covered callers, the reviewer drove ductus exec on a claude-shaped fixture to completion, and a third fixture is new test design, not a chore
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "dead: ductus-procedure's edge case for a first run with only one supported agent cannot arise while the registry holds two — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: a conditional that holds again whenever the registry has one agent, and 012's spec states the same edge case, so removing it would sweep 012's contract for no behavior change
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "claim: Per-file resolution and the markdown-only fallback never mention tool-search-line, so a claude command copy made by that fallback lacks the ToolSearch line every claude entry carries — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: spec 080's surface and a rare fallback; stating the fallback for the preamble line is design work, not a mechanical edit
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "bug: install.sh writes the bootstrap with cp from a mktemp file, so .claude/commands/ductus.md and .pi/prompts/ductus.md land at mode 0600 while the settings seed lands at 0644 — `install.sh`"
    outcome: waived
    reason: pre-existing, and cosmetic since git records only the exec bit and the file holds no secret; changing the installer's write path is an acquisition change, not a chore
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "test: Family 17's pi branch, the flat prompts/<ns>-<name>.md scan, has no case in test-host-namespace-parity.sh, which drives only the claude layout — `scripts/audit/host-namespace-parity.sh`"
    outcome: waived
    reason: a new test case, not a chore; the branch was proven by hand on a renamed pi prompt set, and the family fails its other mutations
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "reuse: main.rs and the interpreter keep separate hand-maintained per-primitive lists — `runtime/src/main.rs`"
    outcome: waived
    reason: flat registries pinned by set-equality tests, a design docs/developing-ductus.md records
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "contract: the two flattened Option result halves allow neither or both; an untagged enum would keep exactly one — `runtime/src/schema/primitives.rs`"
    outcome: waived
    reason: rmcp refuses a tool output schema that is not an object, and an untagged enum's root is anyOf; docs/developing-ductus.md records the flattened-halves design
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "convention: Adding or removing a runtime primitive names seven sites but neither /configure step 4 retired list, and no check catches a removed tool missing from them — `docs/developing-ductus.md`"
    outcome: waived
    reason: "the pipeline's own maintainer procedure: a doc line would rest the next removal on the remover's memory, and a check deriving the retired entries is new design, not a chore"
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "contract: whether the per-agent configure commands carry the host-integration pointer is undecided — `framework/bootstrap/configure/claude.md`"
    outcome: waived
    reason: a design question the configure files leave open; no rule or 071 contract requires the pointer there, and lint-tool-coverage examines framework/commands/ alone
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "concurrency: this repository's pre-commit hook stages generator outputs regenerated from the working tree, sweeping another session's in-flight files into a commit — `.githooks/pre-commit`"
    outcome: waived
    reason: 017's task 45 and its scenario a-commit-stages-only-its-own-generated-files hold this as pending work, as AGENTS.md's gotcha says, and this window did not change the staging
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "other: v0.28.0 cannot be walked at its tag, and AGENTS.md placeholders an old release left literal are never repaired — `framework/bootstrap/ductus-procedure.md`"
    outcome: waived
    reason: observations about the bootstrap's own machinery, none caused by this window and none blocking it
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
  - key: "other: the bootstrap's archive-fetch curl carries no timeout and no --proto or --tlsv1.2, where its other curls now carry all three — `framework/bootstrap/ductus.md`"
    outcome: waived
    reason: "pre-existing: the bootstrap's curls are steps the host agent runs, and changing the archive fetch is an acquisition change, not a chore"
    decided-at: 2026-11-11T14:41:07Z
    decided-by: rule
---

# Review — 080-deterministic-runtime-efficiency

## Summary

One review of 080's reopening window, `de56c366..3cf9955b` (`base-source: reopen`, `review-owed: true`), the third over `b1d319f1`, the chore with no spec that dropped Auggie, Antigravity and OpenCode, after 064's (`2411dca9`) and 071's (`c9b2155c`). Of the 76 scope entries, 68 are shared with 071's window. This session's five passes for 071's review read `b1d319f1`'s and `0afc0c00`'s change on each of them in depth, at `e08ea5b7`, against the 12 rule files. So this review leans on that read, as AGENTS.md's rule for a shared window says, and reads only what changed on those paths since: `e2009d70`, 064's fixes, read hunk by hunk here, and `c9b2155c`, 071's fixes, re-read clean in its fix round.

080's own change was read in full against the same rules:

- `6b0f96fd`'s hunks on its plan's paths, the `unwritable` bound in `main.rs`, `schema/descriptions.rs`, `schema/primitives.rs` and `framework/commands/analyze.md`;
- the reopening's commits `7904cb19`, `7ee3986a` and `04e0e8bf`;
- the agent-removal sweep `636a0df6` and `f78e3004`;
- `3cf9955b`, the corrections the criteria verification found.

Before the runs, one read-only agent verified 080's 22 criteria, `data-model.md` and all seven scenarios at `deca3cff`, each checked whole against the code and driven on the release binary. All hold. Its corrections landed in `3cf9955b`: the Bounded-results bullets name `derive-dependencies`' `unwritable`, `research.md`'s task 11 account and six `§` citations are corrected, the plan's analyze-reference size is re-stated, and task 58 is ticked.

No MUST and no SHOULD. One observation is fixed by the rule: the sweep left `research.md`'s account of the 0.59.11 bootstrap showing the wrong command and copy count. No contract break.

Examined 70 of the 76 scope entries, each a file. The 62 shared files were read by this session's passes, and the eight of 080's own were read whole or by every window commit's change. Not opened: the six generated mirrors, two under `.claude/commands/ductus/`, two under `.pi/prompts/` and two under `.ductus/reference/`, which `scripts/gen-claude-commands.sh --check` reports in sync. The 32 files deleted since the base are named, not read.

Stored decisions:

- Of 080's own, six on in-scope files still stand and are matched. Two, on `framework/bootstrap/ductus.md`'s State A probe and step 10 for the dropped hosts, no longer hold and are dropped. The ten on files outside the scope are retained.
- 63 decisions 071's record holds on in-scope files are carried with their text and disposition, 071's record being the newest over the shared window. Among them is the deferral of `audit.md`'s run-generator step to 022's open task 18.

1 fix round(s): 0 re-read, 0 diff-checked, 1 exempt, 0 not re-read. Exempt: round 1.

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

- security: the review payload ships an in-scope file whatever its name, where the writeCode payload refuses secret-named and ignored files — `runtime/src/interpreter/payload.rs` — **waived**: Predates 059; the reviewing host reads the same tree with its own tools, and whether review withholds a file it is reviewing is the review payload's contract.
- security: `audit.md` step 1 and `analyze.md`'s drift check send the agent to `.ductus/bin/ductus run-generator`, which the seeded `Bash(.ductus/bin/ductus *)` allow runs with no prompt until 022 task 18 narrows it — `framework/commands/audit.md:31` — **deferred** to `specs/022-deterministic-runtime/tasks.md`
- other: with a pinned `target.md`, a tool can load from neither side when the session's `cli-config-dir` names another agent — `runtime/src/primitives/apply_manifest.rs` — **waived**: Inside the two-agent-CLI bound the constitution's §concurrent-features names: the runtime reads command files from the agent `/ductus` last recorded. Within one agent's layout every case agrees. (decided by the rule)
- other: no test holds each bounded member's top-level `-count` keys to the lists its file holds, which the walker's halt reads — `runtime/tests/mcp.rs` — **waived**: Every member's counts name a list its file holds today, checked struct by struct; a guard for a future member is new test design, not a chore. (decided by the rule)
- other: past the bound the MCP response carries the result twice, as text (49,965 bytes for render-status on 700 specs) and as `structuredContent` (53,614) — `runtime/src/mcp/server.rs` — **waived**: Claude Code hands the model one copy, as every `mcp__ductus__` result in this run shows; the bound measures the result, and changing how the server's JSON wrapper serializes is 022's surface and not a mechanical edit. (decided by the rule)
- doc: 080's plan.md Affected Files omits files 080's commits changed, among them read_spec.rs, payload.rs, bounded_lists.rs and the bounded members — `specs/080-deterministic-runtime-efficiency/plan.md` — **waived**: The plan's bounded-results decision covers those files, which rows to add is a judgment, and check-artifact-size's advisory on this plan asks for finished rows to be trimmed rather than added. (decided by the rule)
- other: fix round 1's re-read found that the dashboard test drives only `session-unreadable` past the bound, so removing the `session-notices` move fails no test — `runtime/src/primitives/dashboard.rs` — **waived**: Both lists move by the same branch, which the test drives for one of them; a fixture that puts a session notice past the bound needs co-targeting session records, which is new test design, not a chore. (decided by the rule)
- other: for a namespace holding a tab or a line break, Pi's bridge respells the status view with render-help's unflattened namespace, so the view's flattened hints keep their colon form — `runtime/src/primitives/dashboard.rs` — **waived**: Such a namespace names no command on any host, since Pi registers `{project}-status` from it and every installed command file is named by it, so no respelled hint could run; aligning the bridge would change 064's verbatim bridge, not a mechanical edit. (decided by the rule)
- other: the markdown-only reference-cleanup prose differs from enforce-manifest at the edges (links never removed, dot-prefixed .md matched) — `framework/bootstrap/ductus.md` — **waived**: contrived: nobody writes either into a ductus-owned directory (owner decision, 2026-10-26)
- other: check-corpus-links reads framework/reference/'s presence as a framework checkout, which an adopter's own directory of that name would match — `runtime/src/primitives/check_corpus_links.rs` — **waived**: contrived: it costs only the repository-scope check of shipped copies, which an adopter's spec-corpus hook never runs (owner decision, 2026-10-26)
- other: a greenfield --ref run whose installed ductus.md is stale writes the config before aborting, so its re-run is not greenfield and prompts — `framework/bootstrap/ductus.md` — **waived**: pre-existing and not made worse; the documented install path places a current bootstrap
- other: step 4's per-agent apply-manifest loop sits in one walker step whose context carries one map — `framework/bootstrap/ductus.md` — **waived**: the real bootstrap is walked by the host, as step 8's per-directory loop already is
- other: the greenfield rule's newest-entry id is undefined when the active registry is empty — `framework/bootstrap/ductus-procedure.md` — **waived**: contrived: the registry is never empty while migrations exist, and 027's AC22 already writes nothing then
- convention: the corpus scope matches `.md` case-sensitively where the repository scope does not — `runtime/src/primitives/check_corpus_links.rs` — **waived**: Predates 059; which files each scope examines is `check-corpus-links`' own contract, and changing either direction changes what the check reports.
- convention: tests that expect no repository assume the temp directory has no enclosing work tree — `runtime/src/primitives/check_corpus_links.rs` — **waived**: Predates 059 and is shared with other test modules; a ceiling would need process-wide environment in a parallel suite, and CI's temp directory has no enclosing work tree.
- bug: the criterion-path grammar accepts `../` and scenario finding paths hard-code a lowercase `.md` — `runtime/src/primitives/check_artifacts.rs` — **waived**: Predates 059; both are contracts of `check-artifacts`' criterion-path and scenario families, and neither is a path conversion.
- perf: `read_repo_file` canonicalizes the project root once per rule file — `runtime/src/interpreter/payload.rs` — **waived**: N is the rule-file count, so the cost is negligible.
- perf: build_assess_spec_quality_request re-reads the spec file for every rule request, up to 167 times per step — `runtime/src/interpreter/payload.rs` — **waived**: each read is a small local file the OS caches, beside an LLM round trip per request; reading it once would thread a second value through the walker context for a gain nobody has measured
- reuse: runtime_binary, ensure_binary_built and the envelope loop are copied across test files — `runtime/tests/exec_subprocess.rs:18` — **waived**: predates the window; consolidating test helpers is a refactor outside 048
- efficiency: the prune-plan walk runs inline on the async worker and reads HEAD per done spec, even for specs it then drops — `runtime/src/mcp/server.rs:710` — **waived**: one HEAD read per done spec is not the unbounded revwalk the blocking pool is scoped to, and a spec it drops still needs the answer to know it can be dropped
- tests: the macOS integration test leaks its mktemp directory if building the archive or spawning the binary panics — `runtime/tests/exec_subprocess.rs` — **waived**: it leaks only on a panic before its assertions, one empty directory under the per-user temp directory, which macOS clears
- perf: dashboard and the --all prune walks now walk every feature directory — `runtime/src/primitives/dashboard.rs` — **waived**: measured dashboard 0.23-0.25 s, prune-plan --all 0.04 s and prune-tasks --all 0.01 s on this corpus, the walks' share at most about 6 ms; refusing the whole command is the scenario's stated edge case
- other: canonicalize-and-contain logic exists in both interpreter/payload.rs and primitives/mod.rs — `runtime/src/interpreter/payload.rs` — **waived**: the two differ on purpose: the payload readers skip a missing or outside path and give None for a non-UTF-8 name, where the check refuses with an error and judges a missing path, so one helper would carry both behaviors
- convention: RecordFreshness::Undeterminable's doc names only a missing digest, where an unparseable review.md also answers it — `runtime/src/schema/primitives.rs` — **waived**: Option B (owner, 2026-11-04): an observation about 066's and 026's Family 19 machinery from 02bda11a, outside 078's code, waived per AGENTS.md's rule for such observations; 066's own review reads that commit in its window.
- other: the tasks 43 and 44 changelog entry does not name BE-API-013's explicit bar on a credential or PII in a link sent to a person — `runtime/CHANGELOG.md` — **waived**: Not mechanical, and not untrue: BE-API-013's earlier Not-flagged clause already kept a URL sent in an email or another message as its own, so the entry, which lists new obligations, omits a clarification rather than a change. (decided by the rule)
- other: a security-frontend.md copied from main between afb8a865 and 0481c218^ hands over only a server the project serves, so it keeps an authorization-server library's redirect_uri case beside SEC-AUTHN-004 — `runtime/CHANGELOG.md` — **waived**: A contrived input: both windows are unreleased main, the second about an hour long, and a project pinning such a copy re-syncs it as the changelog's pinned-copy sentence already says. (decided by the rule)
- other: check-step-references' not-a-procedure field doc defines the set by files holding several numbered lists, though it also holds files with no step or too few — `runtime/src/schema/primitives.rs` — **waived**: the primitive's schema doc, 067's surface and outside 026's code; the family's own stderr line now says not one procedure (decided by the rule)
- other: under ductus exec a malformed shared session default halts as config-unreadable, since Host::load reads it first, so main.rs's session-unreadable branch is reached only by the process's own record — `runtime/src/main.rs` — **waived**: The branch is still reached by a malformed own record, the exec test pins the code the default halts with, and either code labels a halt that names the file. (decided by the rule)
- test: the check_artifacts REGISTRY fixture carries a scope column the parser does not read — `runtime/src/primitives/check_artifacts.rs` — **waived**: the fixture mirrors the real file's table shapes, and the real MCP registration table carries the column (decided by the rule)
- edge: a session or config naming a dropped host is not diagnosed: .augment still resolves through the claude-style candidate, and .opencode or .agents ends in command-not-found naming the paths tried — `runtime/src/host.rs` — **waived**: Host::load documents that a config-dir mismatch surfaces as the command-file-not-found error, which names every path tried; the resolver is generic in cli-config-dir by design, so a custom directory keeps working, and the CHANGELOG's Removed entry announces the removal (decided by the rule)
- other: this repository's .pi/prompts/ductus-configure.md names .pi/prompts/ductus.md, which this repository does not track — `.pi/prompts/ductus-configure.md` — **waived**: the dogfooded .pi copies keep the generator honest; this repository is not a Pi adopter, and an adopter's copy has .pi/prompts/ductus.md beside it, which install.sh writes
- other: pi's configure step restates /ductus's latest-release fetch without its grammar check, floor halts or DUCTUS_REPO notice — `framework/bootstrap/configure/pi.md` — **waived**: configure needs only the ref to fetch the bridge at; the halts guard /ductus's own writes, a bad ref fails the bridge fetch with curl's error, and the bounds are named rather than copied
- convention: the Closing restart gives no single form for Pi wired in the run beside Claude already registered — `framework/bootstrap/ductus-procedure.md` — **waived**: the per-agent forms compose into an accurate notice, as the subject's run shows, so no host reads it wrong
- convention: the Frontmatter Migration's specs/**/spec.md walk reports specs/templates/spec.md as already frontmatter on every run — `framework/bootstrap/ductus-procedure.md` — **waived**: the template has frontmatter, so the walk skips it with a report line and writes nothing
- other: the pre-run status's claim that nothing earlier in the run writes under specs/ holds only if Pre-run Migrations run before walker step 4's Shared Files apply-manifest, which ductus.md §Instructions does not state — `framework/bootstrap/ductus-procedure.md` — **waived**: already recorded as 056's scenario the-walker-places-every-archive-half-section, whose task gates 056's done; the old fresh status refused the same way, so this is no regression
- edge: Pre-run Migrations step 1's abort does not say whether the section or the run stops — `framework/bootstrap/ductus-procedure.md` — **waived**: contrived: it needs a malformed archive, and the pre-run status is taken before step 1, so either reading is safe
- edge: a file the adopter had dirtied that a migration also rewrote is named only by the refusal, not as changed by a migration — `framework/bootstrap/ductus-procedure.md` — **waived**: decided in the plan and in the scenario's first edge case: the file is the adopter's to commit, and the refusal names it
- other: the repeated refusal ends with The rest of this run continues after the run has finished — `framework/bootstrap/ductus-procedure.md` — **waived**: wording only; the sentence is still true of the run it describes
- convention: step 2's parenthetical in `.ductus/reference/analyze.md` §Rules (blocking and advisory) can read as describing the checks rather than naming the heading — `framework/bootstrap/ductus-procedure.md:198` — **waived**: Style only: the checks applied are the same either way, and Family 39 needs the full heading (owner, 2026-11-03, option a).
- other: a Windows absolute document path already holds backslashes, and the space escaping may misread them — `framework/bootstrap/ductus.md` — **waived**: resolve-constitutions writes the document path through rel_path, which replaces every backslash with a forward slash, so the block line holds no backslash but the spaces the bootstrap escapes
- convention: no numbered step of the bootstrap names the two managed-block merges — `framework/bootstrap/ductus.md` — **waived**: Instructions step 4 lists what apply-manifest replaces, not every Shared Files step, and two real adopter bootstraps on this text wrote both blocks from Shared Files
- convention: a language with no github/gitignore template, such as markdown, prints a fetch failure on every run — `framework/bootstrap/ductus.md` — **waived**: the run names the failed URL and writes nothing for it, which is the procedure's stated outcome; the subject's markdown language is an input no project has a template for
- convention: the Pi Bridge step says verbatim while the per-agent apply-manifest call carries a substitutions map — `framework/bootstrap/ductus.md` — **waived**: the Bridge step's verbatim copy is its own call, and the subject's bridge matched the framework source byte for byte
- convention: the detection step counts deferred mcp__ductus__ tool names as State A — `framework/bootstrap/ductus.md` — **waived**: an artifact of the test harness, whose tool list belonged to another project's session; a real adopter session lists only its own
- convention: framework/bootstrap/ductus.md §Project Configuration says the description renders wherever ductus names a source (fix rounds 1 and 3's NOTEs) — `framework/bootstrap/ductus.md` — **waived**: A comment the installer writes into adopters' config; correcting it is a bootstrap change, which this repository verifies only with a real adopter run, out of proportion to one clause; the data model states which surfaces carry the description. (decided by the rule)
- other: main's bootstrap calls `resolve-constitutions --unmeasured`, which the pinned 0.59.11 runtime lacks, so a `--ref=main` run errors at that step and falls back to the unflagged call until a release moves the pin — `framework/bootstrap/ductus.md:924` — **waived**: An observation about the pipeline's own machinery made while verifying framework work (AGENTS.md): it concerns 055's step, not 077's, blocks nothing here, and ends when the next release tag moves the runtime pin to one carrying the flag.
- other: an adopter with an installed .augment, .agents or .opencode tree gets no notice from /ductus, nothing removes it, and the managed .gitignore block no longer ignores it — `framework/bootstrap/ductus.md` — **waived**: the owner dropped the three hosts as a breaking change; the CHANGELOG's Removed entry tells such an adopter to remove the tree by hand, the runtime refuses writes there by the owner's exception, and detecting a dropped host's tree would be new bootstrap behavior rather than a fix to this change (decided by the rule)
- other: the swept migrations narrow three procedures to Claude, so a dropped-host adopter far enough behind loses their rewrites silently — `framework/migrations/runtime-store-path.md` — **waived**: no entry was removed or renumbered, so no last_applied names a missing id; claude and pi adopters run the same steps as before, and the hosts were dropped (decided by the rule)
- other: with surface-instruction gone, the MCP registration table's mechanism column holds one value — `framework/bootstrap/ductus.md` — **waived**: the column is the extension point the Adding a new agent steps cite; dropping it is a design change, not removal residue (decided by the rule)
- stale: the Derived values row label Command/skill path still names the skill layout — `framework/bootstrap/ductus.md` — **waived**: a label, not a claim: it still names the command-file path both layouts use, no parser keys on it, and renaming it is a seven-spec sweep, two of them scenarios, for no behavior change (decided by the rule)
- other: §spec-lifecycle resolves the plan rule against the shared test only by sentence order — `framework/constitution.md` — **waived**: coherent as read, the rule sitting between the three cases and Anything else, and the section is held at its byte ceiling
- other: the constitution line naming plan.md carries § references resolve-anchor classes as qualified and does not judge — `framework/constitution.md` — **waived**: both targets, §brownfield-inbox and §drift-prevention, carry markers (checked by hand), and citing a subsection's marked section is the constitution's own anchor rule
- convention: docs/shared-constitution.md's authoring guideline 'Anything that loosens a framework rule' leaves out the exception (fix round 1's NOTE) — `docs/shared-constitution.md` — **waived**: An authoring guideline for shared sources; the exception is a framework rule's own opt-in, which a shared author meets in that rule's text. (decided by the rule)
- reuse: Family 2's generated-block extraction repeats the marker literals that `scripts/tests/test-gen-configure-mcp.sh` and the generator use — `scripts/audit/manifest-parity.sh` — **waived**: scripts/tests has no lib.sh to share a helper through, and check-zero's gen-configure-mcp.sh --dry-run already fails when the markers drift
- note: a ceiling over 19 digits passes the digit guard and makes `[ -gt ]` error — `scripts/lint-governing-size.sh` — **waived**: contrived: a ceiling that large admits any file, so the silent pass it allows cannot hide an oversized one
- reuse: Family 15 adds another hand-rolled scan of the §Agent Registry table beside those of installer-registry-parity.sh, host-namespace-parity.sh, placeholder-roundtrip.sh and permission-entry-shape.sh, while lib.sh's agent_config_dirs reads one column of it — `scripts/audit/runtime-probe-parity.sh` — **waived**: Each family reads a different part of the table and lib.sh's helper returns one column, so a shared row reader is a refactor of four other families' code, not a mechanical fix. (decided by the rule)
- perf: install.sh validates the agent key only after the latest-release lookup and the bootstrap download — `install.sh` — **waived**: the order predates the removal, the arm still halts before writing anything, and moving the check is a behavior change with its own test (decided by the rule)
- test: ductus exec resolution of a claude-style commands/{project}/ copy lost its only subprocess fixture with exec-auggie, so both exec fixtures are Pi-shaped — `runtime/tests/exec_subprocess.rs` — **waived**: the exec call site shares Host::command_file_candidates with the unit-covered callers, the reviewer drove ductus exec on a claude-shaped fixture to completion, and a third fixture is new test design, not a chore (decided by the rule)
- dead: ductus-procedure's edge case for a first run with only one supported agent cannot arise while the registry holds two — `framework/bootstrap/ductus-procedure.md` — **waived**: a conditional that holds again whenever the registry has one agent, and 012's spec states the same edge case, so removing it would sweep 012's contract for no behavior change (decided by the rule)
- claim: Per-file resolution and the markdown-only fallback never mention tool-search-line, so a claude command copy made by that fallback lacks the ToolSearch line every claude entry carries — `framework/bootstrap/ductus.md` — **waived**: spec 080's surface and a rare fallback; stating the fallback for the preamble line is design work, not a mechanical edit (decided by the rule)
- bug: install.sh writes the bootstrap with cp from a mktemp file, so .claude/commands/ductus.md and .pi/prompts/ductus.md land at mode 0600 while the settings seed lands at 0644 — `install.sh` — **waived**: pre-existing, and cosmetic since git records only the exec bit and the file holds no secret; changing the installer's write path is an acquisition change, not a chore (decided by the rule)
- test: Family 17's pi branch, the flat prompts/<ns>-<name>.md scan, has no case in test-host-namespace-parity.sh, which drives only the claude layout — `scripts/audit/host-namespace-parity.sh` — **waived**: a new test case, not a chore; the branch was proven by hand on a renamed pi prompt set, and the family fails its other mutations (decided by the rule)
- reuse: main.rs and the interpreter keep separate hand-maintained per-primitive lists — `runtime/src/main.rs` — **waived**: flat registries pinned by set-equality tests, a design docs/developing-ductus.md records (decided by the rule)
- contract: the two flattened Option result halves allow neither or both; an untagged enum would keep exactly one — `runtime/src/schema/primitives.rs` — **waived**: rmcp refuses a tool output schema that is not an object, and an untagged enum's root is anyOf; docs/developing-ductus.md records the flattened-halves design (decided by the rule)
- convention: Adding or removing a runtime primitive names seven sites but neither /configure step 4 retired list, and no check catches a removed tool missing from them — `docs/developing-ductus.md` — **waived**: the pipeline's own maintainer procedure: a doc line would rest the next removal on the remover's memory, and a check deriving the retired entries is new design, not a chore (decided by the rule)
- contract: whether the per-agent configure commands carry the host-integration pointer is undecided — `framework/bootstrap/configure/claude.md` — **waived**: a design question the configure files leave open; no rule or 071 contract requires the pointer there, and lint-tool-coverage examines framework/commands/ alone (decided by the rule)
- concurrency: this repository's pre-commit hook stages generator outputs regenerated from the working tree, sweeping another session's in-flight files into a commit — `.githooks/pre-commit` — **waived**: 017's task 45 and its scenario a-commit-stages-only-its-own-generated-files hold this as pending work, as AGENTS.md's gotcha says, and this window did not change the staging (decided by the rule)
- other: v0.28.0 cannot be walked at its tag, and AGENTS.md placeholders an old release left literal are never repaired — `framework/bootstrap/ductus-procedure.md` — **waived**: observations about the bootstrap's own machinery, none caused by this window and none blocking it (decided by the rule)
- other: the bootstrap's archive-fetch curl carries no timeout and no --proto or --tlsv1.2, where its other curls now carry all three — `framework/bootstrap/ductus.md` — **waived**: pre-existing: the bootstrap's curls are steps the host agent runs, and changing the archive fetch is an acquisition change, not a chore (decided by the rule)
- claim: research.md's account of the 0.59.11 bootstrap, as the agent-removal sweep left it, showed the run as `/ductus --ref=ductus-v0.59.11` with 17 command copies, though it adopted Claude and a second agent and installed 17 for each — `specs/080-deterministic-runtime-efficiency/research.md` — **fixed** (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/amend.md` — 067-analyze-and-audit-checks
- `.claude/commands/ductus/analyze.md` — 067-analyze-and-audit-checks
- `.claude/commands/ductus/clarify.md` — 067-analyze-and-audit-checks
- `.claude/commands/ductus/target.md` — 067-analyze-and-audit-checks
- `.ductus/reference/clarify.md` — 067-analyze-and-audit-checks
- `.ductus/reference/implement.md` — 067-analyze-and-audit-checks
- `.ductus/reference/plan.md` — 067-analyze-and-audit-checks
- `.github/workflows/framework-checks.yml` — 022-deterministic-runtime
- `.pi/prompts/ductus-amend.md` — 067-analyze-and-audit-checks
- `.pi/prompts/ductus-analyze.md` — 067-analyze-and-audit-checks
- `.pi/prompts/ductus-clarify.md` — 067-analyze-and-audit-checks
- `.pi/prompts/ductus-target.md` — 067-analyze-and-audit-checks
- `AGENTS.md` — 050-constitution, 067-analyze-and-audit-checks, 071-command-reference-split
- `framework/commands/amend.md` — 067-analyze-and-audit-checks
- `framework/commands/clarify.md` — 067-analyze-and-audit-checks
- `framework/commands/target.md` — 067-analyze-and-audit-checks
- `framework/reference/clarify.md` — 067-analyze-and-audit-checks
- `framework/reference/implement.md` — 067-analyze-and-audit-checks
- `framework/reference/plan.md` — 067-analyze-and-audit-checks
- `runtime/src/interpreter/analyze_tally.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/derive_dependencies.rs` — 067-analyze-and-audit-checks
- `runtime/src/primitives/derive_references.rs` — 067-analyze-and-audit-checks
- `scripts/audit/done-spec-criteria.sh` — 026-framework-self-audit
- `scripts/mutation/owner-reviews-067.py` — 067-analyze-and-audit-checks
- `scripts/tests/test-host-namespace-parity.sh` — 022-deterministic-runtime
- `specs/000-slash-commands/scenarios/analyze-reads-the-layout-derived-command-set.md` — 000-slash-commands
- `specs/000-slash-commands/tasks.md` — 000-slash-commands
- `specs/002-project-scaffolding/scenarios/claude-md-template-imports-load.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/spec.md` — 002-project-scaffolding
- `specs/002-project-scaffolding/tasks.md` — 002-project-scaffolding
- `specs/003-bootstrap-automation/scenarios/curl-sh-installer.md` — 003-bootstrap-automation
- `specs/003-bootstrap-automation/tasks.md` — 003-bootstrap-automation
- `specs/007-govern-workflow/plan.md` — 007-govern-workflow
- `specs/007-govern-workflow/scenarios/ductus-self-update-precheck.md` — 007-govern-workflow
- `specs/007-govern-workflow/spec.md` — 007-govern-workflow
- `specs/007-govern-workflow/tasks.md` — 007-govern-workflow
- `specs/009-scenario-targeting/spec.md` — 009-scenario-targeting
- `specs/010-agent-autonomy/tasks.md` — 010-agent-autonomy
- `specs/011-brownfield-process/tasks.md` — 011-brownfield-process
- `specs/012-multi-agent-govern/data-model.md` — 012-multi-agent-govern
- `specs/012-multi-agent-govern/plan.md` — 012-multi-agent-govern
- `specs/012-multi-agent-govern/scenarios/settings-template-bash-allowlist.md` — 012-multi-agent-govern
- `specs/012-multi-agent-govern/spec.md` — 012-multi-agent-govern
- `specs/012-multi-agent-govern/tasks.md` — 012-multi-agent-govern
- `specs/015-tarball-fetch/plan.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/spec.md` — 015-tarball-fetch
- `specs/015-tarball-fetch/tasks.md` — 015-tarball-fetch
- `specs/017-derive-dont-ask/plan.md` — 017-derive-dont-ask
- `specs/017-derive-dont-ask/scenarios/a-commit-stages-only-its-own-generated-files.md` — 017-derive-dont-ask
- `specs/017-derive-dont-ask/tasks.md` — 017-derive-dont-ask
- `specs/022-deterministic-runtime/data-model.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/plan.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/ask-consolidation.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/cli-config-dir-per-contributor.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/commands-dir-parameterization.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/constitution-excerpts-as-skill-resources.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/coverage-residue-cleanup.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/framework-list-dedup.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/govern-bootstrap.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/merge-managed-block-subsection-insertion.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/the-constitutions-registry-validates-its-values.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/the-pi-command-candidate.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/scenarios/writecode-payload-bundling.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/spec.md` — 022-deterministic-runtime
- `specs/022-deterministic-runtime/tasks.md` — 022-deterministic-runtime
- `specs/023-govern-refinement/plan.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/configure-dedup-permissions.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/configure-inert-write-path-entries.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/configure-permission-pattern-safety.md` — 023-govern-refinement
- `specs/023-govern-refinement/scenarios/configure-retires-formerly-canonical-entries.md` — 023-govern-refinement
- `specs/023-govern-refinement/spec.md` — 023-govern-refinement
- `specs/023-govern-refinement/tasks.md` — 023-govern-refinement
- `specs/026-framework-self-audit/plan.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/scenarios/host-namespace-parity.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/spec.md` — 026-framework-self-audit
- `specs/026-framework-self-audit/tasks.md` — 026-framework-self-audit
- `specs/029-bootstrap-runtime-autowire/plan.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/archive-fetch-direct-codeload.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/project-inputs-asked-once.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/runtime-probe-parity-audit.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/scenarios/state-a-deterministic-path-forcing.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/spec.md` — 029-bootstrap-runtime-autowire
- `specs/029-bootstrap-runtime-autowire/tasks.md` — 029-bootstrap-runtime-autowire
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/plan.md` — 042-consolidate-govern-per-project-files-under-govern-directory
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/spec.md` — 042-consolidate-govern-per-project-files-under-govern-directory
- `specs/042-consolidate-govern-per-project-files-under-govern-directory/tasks.md` — 042-consolidate-govern-per-project-files-under-govern-directory
- `specs/043-workflows-sunset/plan.md` — 043-workflows-sunset
- `specs/043-workflows-sunset/spec.md` — 043-workflows-sunset
- `specs/043-workflows-sunset/tasks.md` — 043-workflows-sunset
- `specs/045-decision-state-drift-detection/data-model.md` — 045-decision-state-drift-detection
- `specs/045-decision-state-drift-detection/spec.md` — 045-decision-state-drift-detection
- `specs/046-scenario-open-question-visibility/analysis.md` — 046-scenario-open-question-visibility
- `specs/046-scenario-open-question-visibility/plan.md` — 046-scenario-open-question-visibility
- `specs/046-scenario-open-question-visibility/review.md` — 046-scenario-open-question-visibility
- `specs/046-scenario-open-question-visibility/spec.md` — 046-scenario-open-question-visibility
- `specs/048-govern-acquired-runtime/data-model.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/plan.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/retired-namespace-tools-are-off-limits.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/scenarios/state-b-continues-in-session.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/spec.md` — 048-govern-acquired-runtime
- `specs/048-govern-acquired-runtime/tasks.md` — 048-govern-acquired-runtime
- `specs/049-rename-govern-to-ductus/plan.md` — 049-rename-govern-to-ductus
- `specs/049-rename-govern-to-ductus/spec.md` — 049-rename-govern-to-ductus
- `specs/049-rename-govern-to-ductus/tasks.md` — 049-rename-govern-to-ductus
- `specs/050-constitution/plan.md` — 050-constitution
- `specs/050-constitution/scenarios/memory-routing-lives-in-agents-md.md` — 050-constitution
- `specs/050-constitution/scenarios/the-constitution-fits-one-read.md` — 050-constitution
- `specs/050-constitution/tasks.md` — 050-constitution
- `specs/054-remove-supersession/plan.md` — 054-remove-supersession
- `specs/054-remove-supersession/spec.md` — 054-remove-supersession
- `specs/054-remove-supersession/tasks.md` — 054-remove-supersession
- `specs/055-shared-constitution/plan.md` — 055-shared-constitution
- `specs/055-shared-constitution/tasks.md` — 055-shared-constitution
- `specs/056-bootstrap-archive-boundary-split/spec.md` — 056-bootstrap-archive-boundary-split
- `specs/058-findings-route-at-discovery/plan.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/spec.md` — 058-findings-route-at-discovery
- `specs/058-findings-route-at-discovery/tasks.md` — 058-findings-route-at-discovery
- `specs/061-updates-track-the-latest-release-tag/plan.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/spec.md` — 061-updates-track-the-latest-release-tag
- `specs/061-updates-track-the-latest-release-tag/tasks.md` — 061-updates-track-the-latest-release-tag
- `specs/062-concurrent-session-targets/plan.md` — 062-concurrent-session-targets
- `specs/062-concurrent-session-targets/tasks.md` — 062-concurrent-session-targets
- `specs/063-oversized-artifacts-warn-with-a-fix/plan.md` — 063-oversized-artifacts-warn-with-a-fix
- `specs/063-oversized-artifacts-warn-with-a-fix/tasks.md` — 063-oversized-artifacts-warn-with-a-fix
- `specs/064-pi-host-support/analysis.md` — 064-pi-host-support
- `specs/064-pi-host-support/plan.md` — 064-pi-host-support
- `specs/064-pi-host-support/review.md` — 064-pi-host-support
- `specs/064-pi-host-support/spec.md` — 064-pi-host-support
- `specs/064-pi-host-support/tasks.md` — 064-pi-host-support
- `specs/066-audit-records-and-the-completion-gate/tasks.md` — 066-audit-records-and-the-completion-gate
- `specs/067-analyze-and-audit-checks/data-model.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/adopter-generator-promotion.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/body-path-existence-family.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/criterion-adopter-scope-destinations.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/derive-references-unstaged-drift-is-reported.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/every-agents-generated-copies-are-excluded.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/frontmatter-severity-tiers-on-both-sides.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/per-agent-scaffold-paths-ship-to-adopter.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/pointers-into-shipped-files-are-judged.md` — 067-analyze-and-audit-checks
- `specs/067-analyze-and-audit-checks/scenarios/traverse-deps-cycle-check.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/067-analyze-and-audit-checks/tasks.md` — 067-analyze-and-audit-checks
- `specs/069-accessibility-frontend-rules/plan.md` — 069-accessibility-frontend-rules
- `specs/071-command-reference-split/analysis.md` — 071-command-reference-split
- `specs/071-command-reference-split/plan.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/071-command-reference-split/review.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/071-command-reference-split/spec.md` — 071-command-reference-split
- `specs/071-command-reference-split/tasks.md` — 067-analyze-and-audit-checks, 071-command-reference-split
- `specs/072-governing-documents-stay-within-one-read/plan.md` — 072-governing-documents-stay-within-one-read
- `specs/075-agents-md-holds-what-every-session-needs/plan.md` — 075-agents-md-holds-what-every-session-needs
- `specs/075-agents-md-holds-what-every-session-needs/spec.md` — 075-agents-md-holds-what-every-session-needs
- `specs/075-agents-md-holds-what-every-session-needs/tasks.md` — 075-agents-md-holds-what-every-session-needs
- `specs/078-review-fix-rounds-are-re-read-within-a-bound/plan.md` — 078-review-fix-rounds-are-re-read-within-a-bound
- `specs/079-review-and-analysis-work-converges/plan.md` — 079-review-and-analysis-work-converges
- `specs/079-review-and-analysis-work-converges/tasks.md` — 079-review-and-analysis-work-converges

## Unexamined governance

*None.*
