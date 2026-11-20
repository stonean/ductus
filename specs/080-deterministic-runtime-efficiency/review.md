---
spec: 080-deterministic-runtime-efficiency
last-run: 2026-11-20T00:22:56Z
reviewed-against: d904dcdfe252762eabeaa5de040c5160311953d0
diff-base: dd17d2cdad1da41a78737c1037ba451b523df8eb
must-violations: 0
should-violations: 0
low-confidence: 0
examined: 20
scope: 41
unread:
  - .claude/commands/ductus/analyze.md
  - .claude/commands/ductus/clarify.md
  - .claude/commands/ductus/consolidate.md
  - .claude/commands/ductus/fold.md
  - .claude/commands/ductus/implement.md
  - .claude/commands/ductus/review.md
  - .claude/commands/ductus/target.md
  - .ductus/reference/clarify.md
  - .pi/prompts/ductus-analyze.md
  - .pi/prompts/ductus-clarify.md
  - .pi/prompts/ductus-consolidate.md
  - .pi/prompts/ductus-fold.md
  - .pi/prompts/ductus-implement.md
  - .pi/prompts/ductus-review.md
  - .pi/prompts/ductus-target.md
  - runtime/CHANGELOG.md
  - runtime/src/schema/descriptions.rs
  - runtime/src/schema/primitives.rs
  - runtime/tests/golden/implement-basic.jsonl
  - runtime/tests/golden/review-basic.jsonl
  - runtime/tests/mcp.rs
excluded: 112
contract-breaks: 0
skipped-passes: []
fix-rounds: 1
re-reads: 1
diff-checked: 0
not-re-read: 0
budget-stop: false
reviewed-digest:
  data-model.md: 8d6d914d57d77dc1dd020c6209abeb647abc79a4708a0e97cbb83a40b7329d08
  scenarios/a-step-gates-on-a-bounded-list-by-its-count.md: bdf77659a5e3c0b7dac784691889a9a0160fbe5859a072adb33f7a4134217e83
  scenarios/configure-types-no-permission-entries.md: 6c764c4d44c20f0b7d88582dbd52de2d9c1efd915080813f41eb858e16ca7313
  scenarios/groom-derives-routing-candidates.md: 516690b45261a38cad315305e8b1ca19fa51a072137a7bfbb1a12c9217fe42da
  scenarios/help-answers-with-no-model-turn.md: 8962bafa8cbf7224dc43d5d73a0507d3bd1088492c5977c287eac52acff33189
  scenarios/one-search-covers-referenced-sections.md: 8f024532fe05b863b06fdb6facb8cae8ebe1e979dbfc8fd28035b6336fec97c3
  scenarios/reference-reads-follow-the-cited-sections.md: 66a004c23e55c8e1a4823a5fa13370220af42c42563194fc693c7067998d9c5f
  scenarios/rules-load-without-shell-reads.md: 052d22a0ed61649128bdada4bf6cef556aba1568aa64f6d6055eb5dbe84b92dd
  scenarios/the-search-goes-out-with-the-first-call.md: fb2cfb2453ca185c97d8fd9e81daad0c09fe92280d925c2ee2b52d6273dc000e
criteria-judged: 22
criteria-met: 22
criteria-verdicts:
  - label: AC1
    digest: 3af472e783b942747e431d2c3ee85263b381696287f711c0ab8687d73e41b9b6
  - label: AC2
    digest: 1552ebc297093f9cf5c0cf65b09beae9f9c9cbe5d98af0fd39caa7a6138d1f79
  - label: AC20
    digest: 2e89f7966d96052857fecdbff0468f21386628ba4a206ae877c137c682a23e0b
  - label: AC16
    digest: b48e4c9de27f4d4f512deaf3221ac7718efc3aefb2bad3cc49f17c64fff93bdc
  - label: AC25
    digest: cbbec455e4eef636bfbda802de0a6ac8834d1ba7cf49923197153f6cabcc1a9d
  - label: AC26
    digest: ad64a9caa4503c923a4fea21ad38b3f6ba4d0e2ff85d01eee5622ced95d0006a
  - label: AC9
    digest: 18e0c96ba36accf939bfe4e0eec1b8ce46c17fe74b2b4f0ba9d8e0846040f00a
  - label: AC27
    digest: b0fb2439b3b5acbfb087990d6d074826308690fb59e4128b6df42cbbbdcfeca8
  - label: AC28
    digest: e9bf3da3e7152a570b4e7ea84b5e7539b54df1c93f7c52e11f5cf918c3438503
  - label: AC17
    digest: 2e907f5943fd5011b9bb2a48adf8d8968fa6281fe5725af0343f7020b21bda60
  - label: AC18
    digest: a257d006dbce830646a45b26db874e59a3ec266c9ed8ea1632947979b6c81a70
  - label: AC19
    digest: de8bca5564d154781aad259e8425a0057e9bff9483ed588d1e8569b3d0987a2c
  - label: AC6
    digest: 2a3305b0a6cca1b51ab77e131980388d0a1fb40eb2c8e9ce1091ff935d3f25c1
  - label: AC13
    digest: 0b72fc93b34d5d139a3ee1a4d3510331873f6ff660ceb6978c238762e6cf074c
  - label: AC21
    digest: 72d960f617aed7af193c11c7ff7b8d97555bc872f392a3b39cb7975be318f977
  - label: AC29
    digest: 780e661f2a0cf639598fe49036836f9a1b8632650a2762e4017d6d8819f70ce0
  - label: AC23
    digest: bee719d50c6cd9b95c53a01fd97c3c046e1fae6ded55f72c80fa8608377ec8ae
  - label: AC7
    digest: 55ce61c237cb43bf6d1621d5ac20e07f46e79b99c27eaba9ab78fcc56055b3b0
  - label: AC24
    digest: e4b885063193ba254f51b5dc6b94c716ad864bb82df549cf501a3072e88c880a
  - label: AC8
    digest: 7fd60619dd594f411fd5dc62882e83242b5989d40206afd7fb4e8159b971d6e8
  - label: AC14
    digest: 4c965d0d7c0061e5619421d1819d6a1aa5f99b3784f1c5e576977ad388902897
  - label: AC15
    digest: ddf1dbb2a4994d93e6be70ed102627f70581e812ccbc197e6e66c2d1fdcb9823
blocking: false
dispositions:
  fixed: 5
  deferred: 0
  waived: 5
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
  - key: other — the DASHBOARD description in runtime/src/schema/descriptions.rs says session-notices and session-unreadable are absent when non-empty past the read size but not that their counts exist only in the file form, and READ_SPEC's does not say scenario-files-unreadable is absent when empty; the data model's closing sentence names the description as the home of each exception. Both facts hold in the code (dashboard.rs lines 106-113, read_spec.rs line 45). The fix lands in 022's shared descriptions surface, not in 080's directory. — `runtime/src/schema/descriptions.rs:83`
    outcome: waived
    reason: "An observation about the pipeline's own machinery made while reviewing framework work: it is a description wording change in runtime/ that a runtime batch carries, and the data model states each exception beside its owning list."
    decided-at: 2026-11-16T14:29:02Z
    decided-by: rule
  - key: other — the case pattern at the top of the timeout check lists an empty string beside the non-digit pattern, but the value comes from a defaulted expansion that is never empty, so that alternative cannot match. — `scripts/gen-claude-commands.sh:99-101`
    outcome: waived
    reason: It keeps the case total over every string the next line compares, so a later change to the default expansion cannot send an empty string to the integer test, and it reads as the empty-counts-as-unset rule the plan states.
    decided-at: 2026-11-16T14:29:02Z
    decided-by: rule
  - key: "other: `framework/commands/review.md` step 7.1, `framework/commands/plan.md` step 1 and `framework/commands/clarify.md` steps 5 and 6 read `read-spec` and `traverse-deps` lists with no file-form clause, outside the steps the owner's 2026-11-19 decision names — `framework/commands/review.md`"
    outcome: waived
    reason: "`read-spec` without `include-body` returns a result bounded by one spec's criteria and section headings: the largest over this repository's 76 specs is 29,265 bytes against the 50,000-byte bound (measured with `read-spec` per spec directory, 2026-11-19), so the file form is not reachable in these steps; the plan's corpus-growing section now states the bound"
    decided-at: 2026-11-20T00:22:56Z
    decided-by: rule
  - key: "other: `specs/080-deterministic-runtime-efficiency/tasks.md` task 62 says seventeen steps and task 64 names `framework/commands/prune.md` among four blocks, though prune needed no edit and the plan states the set — `specs/080-deterministic-runtime-efficiency/tasks.md`"
    outcome: waived
    reason: tasks.md is spent once checked (constitution §tasks-phase); the plan and the spec's Resolved Question carry the set, and the plan now says why `framework/commands/prune.md` and `framework/commands/groom.md` do not change
    decided-at: 2026-11-20T00:22:56Z
    decided-by: rule
  - key: "convention: commits 2ff7f188, 4ac7a847 and 2a58fa0c do not note each layout's impact in their messages as AGENTS.md asks — `AGENTS.md`"
    outcome: waived
    reason: both registry layouts' generated copies are in those commits and `scripts/gen-claude-commands.sh --check` exits 0, so the impact is on the record; commit messages are not rewritten
    decided-at: 2026-11-20T00:22:56Z
    decided-by: rule
  - key: "other: four file-form descriptions in `runtime/src/schema/descriptions.rs` (`BRIEF_RE_READ`, `LABEL_CRITERIA`, `READ_RE_READ`, `WRITE_ANALYSIS`) dropped the backticks round list names, so they format list names unlike their siblings — `runtime/src/schema/descriptions.rs`"
    outcome: waived
    reason: the text is true and no larger; the backticks are the cost of a scan that cannot tell a file-form statement from a defect, and an allow-list entry per true statement would hide more
    decided-at: 2026-11-20T00:22:56Z
    decided-by: rule
  - key: "other: `specs/080-deterministic-runtime-efficiency/research.md` records the AC14 and AC23 result and schema sizes at ddc4703f on 2026-11-15, before the description rewordings of this reopening — `specs/080-deterministic-runtime-efficiency/research.md`"
    outcome: waived
    reason: each figure states its commit and date (grounding), so it is point-in-time, not false; AC23 requires only that sizes and a total be recorded, and `scripts/measure-result-sizes.sh` re-measures on demand
    decided-at: 2026-11-20T00:22:56Z
    decided-by: rule
---

# Review — 080-deterministic-runtime-efficiency

## Summary

Window: base dd17d2cd (reopen), 41 scope entries, 22 criteria owing (every criterion: the stored record predates the criteria step). Only 080's own commits belong to this review (the other specs' commits in the window are their reviews): `fix(080)` commits for the bounded-list gates, the descriptions and schema doc comments, the four more steps and the golden re-bless, plus the plan and spec edits. Four readers: security pass (three security rule files whole), reuse/quality/efficiency/simplicity (nine other rule files whole, AGENTS.md Gotchas through Design Principles, system.md), and the criteria step in two parts over all 22 criteria, each judged against code and the whole `cargo test` (exit 0, no failure). Findings: none under a loaded rule. Two observations showed 080's own code breaking the scenario (`framework/commands/clarify.md` step 13, `framework/commands/consolidate.md` step 4) and were fixed in the run with the guard scan widened; the fix round was re-read clean. Unread: 15 generated copies under `.claude/commands/ductus/`, `.pi/prompts/` and `.ductus/reference/clarify.md` (generated, `scripts/gen-claude-commands.sh --check` exits 0, sources read), `runtime/CHANGELOG.md`, `runtime/src/schema/descriptions.rs`, `runtime/src/schema/primitives.rs` and `runtime/tests/mcp.rs` (changed hunks and named constants read, not the files), and the two goldens (hunks, lines truncated). Readers read `research.md`, `plan.md`, `runtime/src/primitives/pipeline_view.rs`, `runtime/src/primitives/tool_loading.rs` and `framework/bootstrap/ductus.md` in part for the criteria. Of the stored decisions, eleven naming in-scope files still hold and carry unchanged.

1 fix round(s): 1 re-read, 0 diff-checked, 0 exempt, 0 not re-read.

22 of 22 acceptance criteria judged met.

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

- AC1 — met: runtime/src/hook.rs `status` + runtime/src/primitives/merge_status_hook.rs `hook_command` (command is the `.ductus/bin/ductus` pointer, `hook status`); runtime/tests/hook_status.rs `the_view_is_shown_as_a_block_that_suppresses_the_original_prompt` asserts decision block + reason (ok in cargotest.log); no-model-turn observed live in research.md 'Typed /ductus:status in Claude Code (task 8)'.
- AC2 — met: merge_status_hook.rs `merge` (matcher `{project}:status`, recognised by command); unit tests `no_file_is_created_holding_both_entries` and `a_second_run_is_unchanged_and_leaves_one_of_each_entry` (exactly one of each after a re-run, bytes unchanged), both ok in cargotest.log; framework/bootstrap/configure/claude.md step 6 invokes merge-status-hook.
- AC20 — met: merge_status_hook.rs `gate`/`MIN_CLAUDE_VERSION` (2,1,101) via `probe_version`; tests `an_older_version_is_skipped_and_writes_nothing`, `an_unreadable_version_is_skipped_with_its_reason`, `a_skip_leaves_the_merged_permissions_intact`, `the_version_gate_compares_numerically_and_admits_its_bound` and tests/merge_status_hook.rs (real probe on inherited PATH) all ok; configure/claude.md steps 6-7 say skipped hook and its `reason` are reported.
- AC16 — met: hook.rs `HookSpecificOutput.suppress_original_prompt` always true in `BlockResponse::showing`; hook_status.rs `the_view_is_shown_as_a_block_that_suppresses_the_original_prompt` asserts hookSpecificOutput equals {hookEventName, suppressOriginalPrompt:true}; absence of 'Original prompt:' observed live in research.md task 8 first/second run.
- AC25 — met: main.rs `Command::Hook` -> `emit_result`, whose Err arm prints stderr and returns ExitCode 1 (never 2); hook_status.rs `a_view_that_cannot_be_rendered_exits_1_with_the_error_on_stderr_alone` asserts code 1, empty stdout, error on stderr (ok); model-path fallback observed in research.md task 8 'Pointer moved aside' runs (fresh session reports the server did not start).
- AC26 — met: Claude: configure/claude.md step 6 and bootstrap/ductus.md step 10 + '### Status hook' run merge-status-hook idempotently by command (test `a_file_holding_only_status_s_entry_gains_help_s`); research.md 'An adopter bootstrap across the change (task 11)' shows 0.59.11 -> main upgrade gaining the entry. Pi: ductus.md Bridge strategy 'update' copies framework/bootstrap/pi/ductus-bridge.ts each run; apply_manifest.rs test `the_pi_bridge_survives_pis_substitutions` (ok) asserts zero substitutions; research.md 'A Pi adopter upgrade across the change' records the fix and the re-run listing the extension commands.
- AC9 — met: framework/bootstrap/pi/ductus-bridge.ts registers `{project}-status` and `{project}:status` via registerCommand, handler calls `render-status` on the bridge's `ductus mcp` stdio child (the ductus binary) and `sendMessage({display:true},{triggerTurn:false})`; scripts/tests/pi-bridge-harness.mjs check 'registers {project}-status, whose handler shows the view and triggers no turn' (helper `shownWithoutATurn` requires exactly one message, display true, triggerTurn false) - harness not run by me (read-only), not in cargotest.log; observed on real pi 1.0.4/1.5.0 in research.md 'Pi's registered /{project}-status (task 12)'.
- AC27 — met: research.md 'Pi's registered /{project}-status (task 12)' **Context** paragraph records the run could not execute a model turn (waived, expired pi login), so the answer is taken from pi's docs/message-types.md: the displayed message reaches context, and sendMessage with triggerTurn:false adds context without a turn; names the source as documentation.
- AC28 — met: Conditional criterion whose antecedent is false (research.md records the view DID reach context, so no compact form is owed); the bridge adds none, and the harness helper `shownWithoutATurn` (scripts/tests/pi-bridge-harness.mjs) fails if a status run sends other than one message; plan.md line on 'Context' states the condition. Met by the condition not firing, not by a compact-form test.
- AC17 — met: runtime/src/primitives/pipeline_view.rs text_table (two-space gaps, no pipes/emphasis); proved by pipeline_view::tests::the_text_table_aligns_its_columns_with_two_space_gaps_and_no_markdown (asserts no '**', no '|', no trailing space, 2-space gap before each column) - ok in cargotest.log
- AC18 — met: pipeline_view.rs summary()/summary_block()/compose(); the_summary_is_one_item_per_line_and_ends_with_the_one_target_hint asserts Counts/Tags/Reopenings/Inbox lines, one ' 004-delta · s1' line per question, exactly one ':target' and last line = hint, both formats - ok
- AC19 — met: pipeline_view.rs markdown_table on Format::Markdown (render_status.rs uses it); both_renderings_carry_the_same_facts_from_one_model asserts equal parsed rows and equal lines outside the table - ok
- AC6 — met: pipeline_view.rs build() next_actions (skips done and target) + next_actions_block + prompt as last block; pipeline_view::tests::the_specs_with_work_left_are_grouped_by_next_action_without_the_target and render_status::tests::the_result_is_the_view_alone (ends_with target prompt) - ok; framework/commands/status.md steps 1-2 only emit rendered-markdown and say not to recompute
- AC13 — met: runtime/src/primitives/render_status.rs + RenderStatusResult (schema/primitives.rs); the_result_is_the_view_alone asserts keys == [rendered-markdown, project, form]; framework/commands/review.md line ~121 still reads dashboard specs[].status (from lists-file when form is file)
- AC21 — met: runtime/src/mcp/server.rs flag_always_loaded/ALWAYS_LOAD_META + tool_loading.rs AlwaysLoaded::resolve; runtime/tests/mcp.rs the_tools_targets_steps_name_are_always_loaded_and_no_other_is derives expected set from framework/commands/target.md and asserts flagged == expected == tools with _meta - ok
- AC29 — met: tool_loading.rs add_tool_search_line via apply-manifest tool-search-line entries written by scripts/gen-claude-commands.sh (write_claude_manifest); apply_manifest::tests::a_flagged_copy_carries_the_line_and_parses_to_its_sources_procedure asserts line text and steps(written)==steps(plain) (exec dispatches nothing new); Pi copies use substitute() only, command sources have no 'select:' (grep 0); .claude copies carry the line - ok. Note: target.md's copy also gets a line for tools it cites that are not always-loaded (read-tasks, check-review-gate); the criterion says nothing against that
- AC23 — met: specs/080-deterministic-runtime-efficiency/research.md 'Loaded schema sizes (AC23)': 73 per-tool rows (description + input schema chars, I re-summed rows: 186,233) and the stated total 186,233, from runtime/tests/result_sizes.rs via scripts/measure-result-sizes.sh
- AC7 — met: runtime/src/primitives/bounded_lists.rs place(); runtime/tests/mcp.rs every_result_that_grows_with_the_corpus_moves_its_lists_past_the_read_size_via_mcp drives all 16 research.md members + derive-references (incl. read-tasks) inline vs read-size 1 and asserts form file, lists absent, counts match, file holds lists, entries>0 - ok. Bound is forced by shrinking read-size, not by a corpus that outgrows 50,000
- AC24 — met: pipeline_view.rs render()/compose()/CUT_ORDER (done rows -> row cut -> names -> lists to counts), render_status.rs places the whole view in lists-file; unit tests past_the_bound_the_done_rows_collapse..., still_over_the_bound..., with_no_row_left..., past_the_rows_and_names_each_list_collapses_in_its_order, each_line_standing_for_a_cut_names_the_file; runtime/tests/status_view_bound.rs both_surfaces_hold_the_whole_result_to_the_read_size_on_disk (120-spec generated corpus, render-status and hook status, bytes<=bound, file holds whole view, hook names no file) - all ok
- AC8 — met: research.md 'Command cost (AC8)': 17 commands (all of framework/commands except maintainer-only audit, which /ductus installs no copy of: ductus.md 'withheld with the command') with method, units, population, date 2026-11-09 at 14c88044; scripts/measure-command-cost.py
- AC14 — met: scripts/measure-result-sizes.sh runs ignored test runtime/tests/result_sizes.rs (every PRIMITIVE_REGISTRY name via MCP on generated corpora, no claude/model call found in either); research.md 'Result sizes (AC14)' records the output with commit and time
- AC15 — met: scripts/measure-command-cost.py drives 'claude -p --output-format stream-json'; no .github workflow references it (grep of .github finds only result_sizes comments in runtime.yml); research.md 'The claude -p harness's first run (task 16)' records that a custom slash command runs under -p

## Observations

- bug: `framework/commands/clarify.md` step 13 gated the scenario-question report on `scenario-open-questions` being non-empty, a list absent in file form; the step now reads `scenario-open-questions-count`, as step 2 does — `framework/commands/clarify.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- bug: `framework/commands/consolidate.md` step 7 reports `rewritten`, `skipped`, `kept-tails` and `mentions` with no file-form clause; step 4 now reads them from the file `lists-file` names when `form` is `file` — `framework/commands/consolidate.md` (breaks the spec's contract) — **fixed** (decided by the rule)
- other: `framework/commands/review.md` step 7.1, `framework/commands/plan.md` step 1 and `framework/commands/clarify.md` steps 5 and 6 read `read-spec` and `traverse-deps` lists with no file-form clause, outside the steps the owner's 2026-11-19 decision names — `framework/commands/review.md` — **waived**: `read-spec` without `include-body` returns a result bounded by one spec's criteria and section headings: the largest over this repository's 76 specs is 29,265 bytes against the 50,000-byte bound (measured with `read-spec` per spec directory, 2026-11-19), so the file form is not reachable in these steps; the plan's corpus-growing section now states the bound (decided by the rule)
- convention: `specs/080-deterministic-runtime-efficiency/plan.md`, the spec's Resolved Question and `runtime/CHANGELOG.md` stated the changed steps as a count (twelve) that task 64 moved, and the plan's Affected Files row omitted the description scan — `specs/080-deterministic-runtime-efficiency/plan.md` — **fixed** (decided by the rule)
- other: the phrase scan in `runtime/tests/bounded_list_gates.rs` missed a name followed by `field was non-empty` or `is not empty`, so it passed over clarify step 13 — `runtime/tests/bounded_list_gates.rs` — **fixed** (decided by the rule)
- convention: `runtime/tests/bounded_list_gates.rs` repeated the in-process server setup in two helpers and started two servers — `runtime/tests/bounded_list_gates.rs` — **fixed** (decided by the rule)
- other: `specs/080-deterministic-runtime-efficiency/tasks.md` task 62 says seventeen steps and task 64 names `framework/commands/prune.md` among four blocks, though prune needed no edit and the plan states the set — `specs/080-deterministic-runtime-efficiency/tasks.md` — **waived**: tasks.md is spent once checked (constitution §tasks-phase); the plan and the spec's Resolved Question carry the set, and the plan now says why `framework/commands/prune.md` and `framework/commands/groom.md` do not change (decided by the rule)
- convention: commits 2ff7f188, 4ac7a847 and 2a58fa0c do not note each layout's impact in their messages as AGENTS.md asks — `AGENTS.md` — **waived**: both registry layouts' generated copies are in those commits and `scripts/gen-claude-commands.sh --check` exits 0, so the impact is on the record; commit messages are not rewritten (decided by the rule)
- other: four file-form descriptions in `runtime/src/schema/descriptions.rs` (`BRIEF_RE_READ`, `LABEL_CRITERIA`, `READ_RE_READ`, `WRITE_ANALYSIS`) dropped the backticks round list names, so they format list names unlike their siblings — `runtime/src/schema/descriptions.rs` — **waived**: the text is true and no larger; the backticks are the cost of a scan that cannot tell a file-form statement from a defect, and an allow-list entry per true statement would hide more (decided by the rule)
- other: `specs/080-deterministic-runtime-efficiency/research.md` records the AC14 and AC23 result and schema sizes at ddc4703f on 2026-11-15, before the description rewordings of this reopening — `specs/080-deterministic-runtime-efficiency/research.md` — **waived**: each figure states its commit and date (grounding), so it is point-in-time, not false; AC23 requires only that sizes and a total be recorded, and `scripts/measure-result-sizes.sh` re-measures on demand (decided by the rule)

## Skipped passes

*None.*

## Excluded from scope

- `.claude/commands/ductus/help.md`
- `.claude/commands/ductus/prune.md`
- `.ductus/reference/analyze.md`
- `.ductus/reference/audit.md`
- `.ductus/reference/implement.md`
- `.ductus/reference/plan.md`
- `.ductus/reference/review.md`
- `.pi/prompts/ductus-help.md`
- `.pi/prompts/ductus-prune.md`
- `framework/commands/help.md`
- `framework/commands/prune.md`
- `framework/reference/analyze.md`
- `framework/reference/audit.md`
- `framework/reference/implement.md`
- `framework/reference/plan.md`
- `framework/reference/review.md`
- `runtime/Cargo.toml`
- `runtime/src/interpreter/mod.rs`
- `runtime/src/interpreter/payload.rs`
- `runtime/src/io.rs`
- `runtime/src/main.rs`
- `runtime/src/parser/mod.rs`
- `runtime/src/primitives/analyze_subjects.rs`
- `runtime/src/primitives/append_inbox.rs`
- `runtime/src/primitives/check_artifacts.rs`
- `runtime/src/primitives/check_review_gate.rs`
- `runtime/src/primitives/classify_spec_edit.rs`
- `runtime/src/primitives/commit_attribution.rs`
- `runtime/src/primitives/compute_review_scope.rs`
- `runtime/src/primitives/merge_managed_block.rs`
- `runtime/src/primitives/mod.rs`
- `runtime/src/primitives/prune_plan.rs`
- `runtime/src/primitives/prune_tasks.rs`
- `runtime/src/primitives/read_spec.rs`
- `runtime/src/primitives/relocate_audit_records.rs`
- `runtime/src/primitives/rule_sections.rs`
- `runtime/src/primitives/spec_links.rs`
- `runtime/src/primitives/tool_loading.rs`
- `runtime/src/primitives/write_review.rs`
- `runtime/tests/check_step_references_cli.rs`
- `runtime/tests/exec_stderr_gone.rs`
- `scripts/audit/README.md`
- `scripts/audit/step-reference-integrity.sh`
- `scripts/mutation/exec-arguments.py`
- `scripts/mutation/installed-file-modes.py`
- `scripts/mutation/prune-open-region.py`
- `scripts/mutation/prune-review-fixes-041.py`
- `scripts/mutation/relocate-merge-children-057.py`
- `scripts/mutation/reopen-measure.py`
- `scripts/mutation/review-022-task-44.py`
- `scripts/mutation/review-read-credit.py`
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
- `specs/026-framework-self-audit/scenarios/families-follow-the-command-reference-split.md`
- `specs/026-framework-self-audit/scenarios/family-34-reference-section-reachability.md`
- `specs/026-framework-self-audit/scenarios/family-34-step-reference-integrity.md`
- `specs/026-framework-self-audit/spec.md`
- `specs/026-framework-self-audit/tasks.md`
- `specs/041-task-pruning/data-model.md`
- `specs/041-task-pruning/scenarios/an-open-fenced-block-or-comment-is-not-examined.md`
- `specs/041-task-pruning/spec.md`
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
- `specs/067-analyze-and-audit-checks/scenarios/a-bare-port-span-names-its-remedy.md`
- `specs/067-analyze-and-audit-checks/scenarios/check-artifacts-skipped-targets.md`
- `specs/067-analyze-and-audit-checks/scenarios/derive-unparseable-frontmatter-is-reported.md`
- `specs/067-analyze-and-audit-checks/scenarios/orphaned-reference-check.md`
- `specs/067-analyze-and-audit-checks/spec.md`
- `specs/067-analyze-and-audit-checks/tasks.md`
- `specs/071-command-reference-split/scenarios/reference-sections-are-reached-by-a-step.md`
- `specs/071-command-reference-split/spec.md`
- `specs/071-command-reference-split/tasks.md`
- `specs/079-review-and-analysis-work-converges/data-model.md`
- `specs/079-review-and-analysis-work-converges/plan.md`
- `specs/079-review-and-analysis-work-converges/scenarios/the-excluded-section-lists-paths-only.md`
- `specs/079-review-and-analysis-work-converges/spec.md`
- `specs/079-review-and-analysis-work-converges/tasks.md`
- `specs/081-reviews-and-checks-follow-what-changed/analysis.md`
- `specs/081-reviews-and-checks-follow-what-changed/plan.md`
- `specs/081-reviews-and-checks-follow-what-changed/review.md`
- `specs/081-reviews-and-checks-follow-what-changed/spec.md`

## Unexamined governance

*None.*
