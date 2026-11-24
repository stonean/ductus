# Changelog

All notable changes to the `ductus` deterministic runtime are recorded here. The runtime ships in lockstep with the framework per [§runtime-boundary](../framework/constitution.md#runtime-boundary); release tags use the `ductus-v<MAJOR>.<MINOR>.<PATCH>` scheme (was `gvrn-v*` before 0.28.0, and `runtime-v*` before 0.2.0 — see those entries below). Entries below 0.28.0 name the runtime `gvrn` because that is what was published under those tags.

## [Unreleased]

### Added

- **A departure entry can name the sites it covers (spec 073, AC9 to AC15;
  the agent's recommendations, adopted by the operator, 2026-11-23).** An
  entry's optional `covers:` lists `` `file` in `site` `` items, and
  `write-review` then waives its rule at those sites alone: the stored waiver
  holds `anchors:` with no count, a finding of the rule at another site or
  naming none counts, a deferral there stands, and `process-waivers` expires an
  anchor the entry no longer lists. The result's new `departure-notices` names
  each covered file not in the tree or leaving the project, which is never
  probed, and each finding of the rule in a covered file at a site the entry
  does not list or at none, bounded with the other departure lists.
  `check-departures` reports a malformed `covers:` as Blocking. An entry with
  no `covers:` is unchanged, and analyze waives a scoped entry's rule as an
  unscoped one's. A scoped waiver reads as malformed to a runtime that
  predates it, which then waives nothing there.
- **A review's window says how many of its commits no spec claimed (spec 083,
  AC13 and AC16; the agent's recommendations, adopted by the operator,
  2026-11-23).** `compute-review-scope` gains `commits-examined` and
  `commits-unclaimed`, inline in both forms: every commit its window's walk
  visited, and those no spec claimed although they changed a path in the
  project, counted before any credit, so a clean merge is examined and never
  unclaimed and a crediting review leaves the count as it was. Both are absent
  when there is no window, an empty diff-base. `write-review` states them in
  `review.md`'s `## Summary`, or that the scope had no window, and the record
  gains no field. A project whose commit subjects never name a spec now sees
  how much of its code no spec claimed.

### Changed

- **BREAKING: a review record the gate holds, whose passes read their scope,
  moves the next review's window to its `reviewed-against` (spec 079, AC5; the
  agent's recommendation, adopted by the operator, 2026-11-23).** Through 0.64.0
  a record carrying a gating count, an open finding, a contract break, a
  criterion not met, the open fix-round mark or an unwaived round left the
  window at the reopening, so each re-run read the whole reopening again.
  `compute-review-scope` now starts it at that record's `reviewed-against`, no
  earlier than the reopen, when the record states `unread: []`: the fixes its
  open loop or an unwaived round left unread lie after that commit, and what
  it leaves open joins the scope as before, as does what the held record the
  window starts at leaves open when a later run is passed over. A clean record
  keeps the commit finishing it. A held record stating no empty `unread` or no
  `reviewed-against`, every one written before 0.60.0's `unread` among them,
  still moves no start. `base-source` gains `held-review` and
  `reopen-past-partial-review`, the second when every run since the reopen was
  passed over for not reading its scope, which read `reopen` before. Breaking
  for a consumer matching `base-source` against its earlier values, and for a
  held record's next window, which narrows to what came after its passes.
- **What a review left unread joins the next review's scope (spec 079, AC30;
  the agent's recommendations, adopted by the operator, 2026-11-23).** Through
  0.64.0 a record the gate does not hold moved the window's start past its
  own `unread` entries, so nothing brought them back. `compute-review-scope`
  now holds in scope each entry the record at `HEAD`, or the record the window
  starts at, lists in `unread`, wherever the window starts, a plan entry that
  is a glob as the entry it is; like a changed contract, such an entry owes no review of
  its own, so `review-owed` reads as before. `write-review` derives its
  denominator from that same window when it starts at the run's own
  `diff-base`, so a carried entry the run lists in `unread` is accepted and
  counted.
- **BREAKING: routing candidates offer first the specs whose plans claim a
  path the work names (spec 022, task 62; the agent's recommendations, adopted
  by the operator, 2026-11-23).** `derive-routing-candidates` gains a
  `path-claim` source ahead of the other three: each path a description names
  in an inline code span, read by the acceptance-criterion path grammar once a
  `:N` citation is stripped, is offered to every spec whose plan's Affected
  Files claims it, an exact path, then a glob, then a directory, then by spec
  number, each reason saying that the plan changed the path and how many plans
  did, never that the spec owns it. The result gains `named-paths` and
  `unclaimed-paths`, each absent when empty, so a path no plan claims is
  reported rather than read as an empty list; a plan that cannot be read is a
  `path-claim` skip, and no path is reported unclaimed while one is.
  `routeInboxItem`'s request carries `unclaimed-paths`, and groom's decision
  tree holds that a spec's name, or a task filed in it, is no evidence that it
  owns the code. Breaking for a consumer matching `sources-examined` or a
  candidate's `source` against the three earlier values.

### Fixed

- **A `departs:` item's rule ID is a code span as `CommonMark` reads one
  (spec 073, review of 2026-11-24).** The reader split the line on backticks,
  so a lone unclosed backtick named a rule and the entry waived it, and a rule
  ID in a double-backtick span named none; it now finds the spans as a
  `covers:` item's are found, so the first is a malformed entry naming no rule
  ID and the second a well-formed one.

## [0.64.0] — 2026-11-23

A minor release with breaking changes: 3 of the 5 entries below are marked
BREAKING, under Changed. Four of `security-backend.md`'s and
`security-frontend.md`'s rules change by the owner's decisions of 2026-11-23:
`BE-DEPS-005` and `FE-STORAGE-006` hold a spec to more, `BE-API-015` flags a
case its Statement already refused, and `BE-DATA-012` sets aside a throwaway
local or test credential. Two `accessibility-frontend.md` Not-flagged clauses
are reworded with the same behavior, and the adopter CI template installs
0.63.0. No runtime code, lockfile or manifest change, and the MSRV stays 1.89.
The entry for the accessibility clauses was added with this release; the
others were written as their changes landed.

### Changed

- **BREAKING: `BE-DEPS-005` flags what its Statement requires (spec 008 task
  47; the agent's recommendation, adopted by the operator, 2026-11-23).** Its
  Verification flagged an omitted defence only in a project with private
  packages or a high dependency-churn rate. It now flags, in every project, a
  missing private-registry mapping where private packages exist, a missing
  integrity-verified install where the package manager supports one, and a
  plan that names no place where a new dependency's name is checked, and its
  Statement's (c) now requires the plan to name that place: an allowlist, a
  lockfile-diff review step or a tool, as `FE-DEPS-009` has since 0.60.0.
  Breaking for a spec or plan covering dependency management, in a project with
  `security-backend.md`, that omits any of the three: each is now a blocking
  `/analyze` finding. `BE-DEPS-004` is unchanged.
- **BREAKING: `FE-STORAGE-006` purges the per-user cache when the session ends
  (spec 008 task 48; the agent's recommendation, adopted by the operator,
  2026-11-23).** A service worker may cache user-specific authenticated content
  other than tokens and credentials only when the spec states the cache is
  partitioned per user and purged at sign-out and when the application learns
  the session has expired or been revoked, the moment `FE-AUTHN-002` already
  requires it to act on. Breaking for a service-worker spec that states the
  sign-out purge alone: it is now a blocking `/analyze` finding.
- **BREAKING: `BE-API-015` flags an absent `Origin` on a cookie-authenticated
  connection (spec 008 task 49; the agent's recommendation, adopted by the
  operator, 2026-11-23).** Its Statement already refused a WebSocket handshake
  with no `Origin` unless the connection is authenticated by a credential the
  browser does not attach automatically, but no Verification flag named the
  case. Breaking for a spec whose endpoint accepts such a handshake on a
  cookie-authenticated connection: it is now a blocking `/analyze` finding. The
  Statement is unchanged.
- **`BE-DATA-012` sets aside a throwaway credential (spec 008 task 46; the
  agent's recommendation, adopted by the operator, 2026-11-23).** A credential
  for a throwaway local or test resource that grants no access to a production
  or shared system, such as a `docker-compose.yml` `POSTGRES_PASSWORD: postgres`
  or a committed `.env.test` key for a local emulator, is not a secret under
  the rule, in its Statement and its Verification, as `CFG-ENV-013` has it, so
  a spec owes it no revocation path. It turns no passing `/analyze` into a
  finding.

### Fixed

- **`FE-A11YFORM-002`'s and `FE-KBD-001`'s Not-flagged clauses say what they
  mean (spec 069).** `FE-A11YFORM-002`'s ended in a garbled phrase: it now
  hands a timer that hides an error tooltip or popover, made visible by hover
  or focus, while its trigger still has hover or focus, to `FE-KBD-007`, and
  keeps a timer that hides an error message validation made visible as its
  own. `FE-KBD-001`'s held a flagged case, which now sits in its flags
  sentence: handwriting that enters text, and a signature that only
  acknowledges something or confirms identity. No case moves between flagged
  and not flagged.

## [0.63.0] — 2026-11-23

A minor release with a breaking change: 1 of the 3 entries below is marked
BREAKING, under Changed. A stored waiver or violation deferral covers only the
findings at its `(rule, file, site)` anchor, up to the number it covers, so an
adopter's existing waivers and violation deferrals expire once, on each spec's
next review, and are decided again. The constitution's §brownfield-inbox again
says a question is deferred with its trigger, in Resolved Questions. One
lockfile patch update, no manifest change, and the MSRV stays 1.89. The
entries for the lockfile and the constitution were added with this release;
the other was written as its change landed.

### Changed

- **BREAKING: a stored waiver or violation deferral covers only the findings at
  its `(rule, file, site)` anchor, up to the number it covers (spec 020).**
  Through 0.62.0 both matched on rule and file alone, so one waiver covered
  every finding of its rule in the file, unrelated code and code added later
  included. A finding now carries an optional `site`, the narrowest named unit
  enclosing it, and `fired` for `process-waivers` one entry per finding with its
  `site` and `line-range`. `write-review` records a `--waive` as one entry whose
  `anchors:` lists a `{file, site, covers}` per site its rule fires at,
  `covers` the findings there, two overlapping reports at one site counted once
  and findings at different sites apart, lowered by a run that applies the
  anchor to fewer and never raised; a violation's deferral is keyed
  `` rule {rule-id} at {file} in `{site}` `` and carries `covers`, counted the
  same way, and `process-decisions` takes a review's violations as
  `fired-violations` (`--fired-violations-json` on the CLI), one entry per
  finding with its `site` and `line-range`, keyed and counted as the deferral
  was stored, where step 9 fired one key per finding. More findings at
  a site than an anchor covers expire it, on a restricted run too, with
  `waiver expired: … — {n} findings fire where it covers {m}` or
  `decision expired: … — {n} findings fire where it covers {m}`. An entry
  recorded before sites, a hand waiver's `file` with no `anchors` or a violation
  key with no backticked site, covers nothing and expires on its spec's next
  review with a `recorded before sites` notice; a departure waiver keeps `file`
  and covers every site in it. `write-review` refuses a `--waive` or `--defer`
  of a finding that names no site, whatever `decision-key` it carries, and
  renders `- **Site**:` under each finding's file. A violation's `decision-key`,
  on `write-review`'s `findings` or `process-decisions`' `fired-violations`, is
  its own key or the call is refused naming it, so no host match moves a finding
  to another site's deferral. The cross-pass dedup matches on the site too, so a
  finding a waiver or deferral covers at one site no longer drops an overlapping
  finding of its rule at another from the report and the counts. A departure
  waiver is read by its `file` whatever else its entry holds, and a violation's
  key whose site is a heading holding an em dash and a backtick still names its
  file, so its deferral on a file the run did not read is retained. An adopter's
  existing waivers and violation deferrals expire once and are decided again.
- **Lockfile, net from 0.62.0 to the release head: 1 dependency changes version, none is added or removed, and no advisory is open.**
  Counted as the `[[package]]` name and version pairs of `runtime/Cargo.lock`
  at `ductus-v0.62.0` against the release commit (`git diff ductus-v0.62.0
  HEAD -- runtime/Cargo.lock`), the crate's own version aside; the lockfile's
  268 package entries stay 268. `zip` 9.8.1 to 9.8.2, the highest compatible
  with Rust 1.89. `scripts/check-runtime-currency.sh` scanned 268 crates and
  found no advisory, and the manifest's dependencies are unchanged.

### Fixed

- **The constitution again says a question is deferred with its trigger (spec 076).**
  §brownfield-inbox's Deferred disposition says a question is deferred with
  its trigger, in Resolved Questions, which 0.60.0 dropped from it, leaving
  §spec-requirements' pointer for a deferred decision on a definition that
  named findings alone. No gate changes: a deferred question stays in Resolved
  Questions and holds nothing.

## [0.62.0] — 2026-11-22

A minor release with breaking changes: 2 of the 6 entries below are marked
BREAKING, under Changed and Removed. `write-review` refuses a write whose
scope a later commit moved, in a refusal whose size is bounded, so a review's
record credits only what its passes read, and the two audit-record writers'
results state the counts their records hold. `BE-AUTHN-009` and
`BE-AUTHN-013` leave `security-backend.md` at the 0.62.0 target their 0.60.0
deprecation set. Two constitution rules attribute a decision to whoever made
it and keep a primitive's probe out of the project's tree, and `/ductus`'s
Claude seed grants `certutil` for the pointer alone, so each adopter's
self-update replaces the installed bootstrap, one restart, on the first
`/ductus` after this release. No lockfile or manifest change, and the MSRV
stays 1.89.

### Added

- **The audit-record writers' results state what they wrote (spec 066).**
  `write-analysis` returns `hard-fail`, `blocking-findings` and `advisory` as
  the record holds them, the first two net of the live findings a deferral
  releases, and `write-review` returns `excluded`, `unread-count` and
  `decisions`, the counts its record holds, so a caller sees a decision the
  write dropped without reading `review.md` back.
- **Two constitution rules (spec 050).** §recommendations attributes a
  decision to whoever made it in every record a command writes, the clarify
  reference pointing at it, and §grounding probes a primitive on an artifact
  not yet written in a `git clone --local` copy, never in the project's tree.

### Changed

- **BREAKING: `write-review` refuses a write whose scope moved under the review
  (spec 066).** `compute-review-scope` returns `head`, the commit its scope was
  resolved against, and `/ductus:review` passes it as `reviewed-against`.
  `write-review` refuses, writing nothing, while a commit after that sha changed
  a path its scope holds, since another spec's window took such a path as read.
  The refusal's size is bounded whatever moved: it counts the moved paths,
  names the first three, each with up to three of its commits, and gives the
  `git log --relative --name-only` command that lists every one. It says to
  read each moved path and each path a re-run of `compute-review-scope` adds
  before writing with the new `head`. A `reviewed-against` naming no commit,
  or one `HEAD` does not descend from, is refused too. None of the three is
  checked with an empty `diff-base`, whose scope is empty.

### Removed

- **BREAKING: `BE-AUTHN-009` and `BE-AUTHN-013` leave `security-backend.md`
  at their 0.62.0 target (spec 077).** Both were deprecated in 0.60.0, and
  `security-cross.md`'s `SEC-AUTHN-001` and `SEC-AUTHN-002` hold every case
  they governed. A spec citing either now draws the Blocking unknown-rule
  finding where it drew an Advisory one, so re-point it to the successor, or
  list the ID under the spec's `retired-rules:` when the spec records the
  retirement. A project that pinned `security-backend.md` keeps both in its
  copy until it re-syncs the file.

### Fixed

- **Three references and a description state what the runtime already did
  (spec 066).** The analyze reference says a criterion's tick leaves the
  analysis current, `compute-review-scope`'s description says every backticked
  path in an Affected Files cell is a plan entry, and the review reference says
  a pass is never told to leave out an issue a stored decision settles.
- **`/ductus`'s Claude seed grants `certutil` for the pointer alone (specs 023 and 048).**
  The seed's `Bash(certutil -hashfile * SHA256)` let its `*` span inserted
  options, and Claude Code warns on it in every adopter ("allows any option at
  its * · use an exact value"). It is now
  `Bash(certutil -hashfile .ductus/bin/ductus.exe SHA256)`, and the old entry is
  on Claude's retired seed list, so the next `/ductus` run removes it from an
  existing settings file. On Windows the configured binary's digest, whose path
  differs per project, now prompts on Claude, once a run and twice on a run that
  re-makes the pointer. `/ductus:audit` Family 29 reads the seed as well as
  `/configure`'s canonical set.

## [0.61.0] — 2026-11-21

A minor release with breaking changes: 4 of the 11 entries below are marked
BREAKING, under Added and Changed. An analysis fixes a gating finding at its
own design in the run once the operator confirms the edit, judges the plan
against the spec, and judges each scenario's `§` references; a stored rule
verdict stands while the sections it concerns are unchanged; a review leaves
the seven rule files whose headers give them to analysis out of its
selection; and a review pass or a fix round's re-read raises a finding only
on a case its trigger shows possible in the project. `/ductus` now writes
the shared-constitution links into `AGENTS.md` for every agent, so each
adopter's self-update replaces the installed bootstrap, one restart, on the
first `/ductus` after this release. Six lockfile patch updates, no manifest
change, and the MSRV stays 1.89. The entries for spec 020's AC18, spec 055's
AC13 and the lockfile were added with this release; the others were written
as their changes landed.

### Added

- **`resolve-anchor` counts a single-quoted heading and a double quote its line leaves open as `quoted-heading` references (spec 067, AC9).**
  `§'Cache invalidation'` (straight or curly quotes) and `§"Cache` ending its
  line with the heading continuing on the next were in no count; each is now a
  reference of kind `quoted-heading`, unjudged `anchor-target-unjudgeable`,
  counted as a closed double-quoted one is. A quote not directly after the `§`
  is no reference. A scan of a file holding either form reports one more
  reference and one more `quoted-heading` and `anchor-target-unjudgeable`.
- **BREAKING: `write-analysis` refuses a gating finding at a design subject deferred to its own `tasks.md`, and one left undispositioned at a subject a fix of the run edited (spec 067, AC6).**
  A live hard-fail or blocking finding whose `path` is a design subject of the
  spec being recorded (`spec.md`, `plan.md`, `data-model.md` or a scenario,
  repo-relative or relative to the feature directory) is refused when its
  deferral `target` is that spec's own `tasks.md`, and refused when recorded
  undispositioned while a finding recorded `live: false` cites the same
  subject, which is how the arguments show that a fix of the run edited it.
  Each refusal names the finding and writes no record. A caller that deferred
  such a finding to its own tasks, or recorded a new one undispositioned after
  fixing its subject, is refused where it was accepted; a deferral to another
  spec's work, a finding at `tasks.md` or any other file, an advisory finding,
  and every run that itemizes no fixed finding, `ductus exec` included, are
  recorded as before. `/ductus:analyze` steps 19 and 20 say the design edit is
  made in the run once the operator confirms it, and step 15 judges the plan
  against the spec's Behavior and acceptance criteria.
- **BREAKING: `resolve-anchor` reads a scenario's bare `spec.md` as the parent's and reports the forms it does not read (spec 067, AC8, AC9).**
  In a file under a `scenarios` directory a bare `spec.md` names the `spec.md`
  of the spec that directory sits in, so a reference beside it is judged
  instead of unjudged, and may now come back unresolved. A `§` followed by a
  quoted heading is a reference of the new kind `quoted-heading`, `unjudged`
  `anchor-target-unjudgeable`, counted in the new `quoted-heading` and in
  `anchor-target-unjudgeable`; an unresolved reference its next line completes
  carries the new `wrapped: true` and is counted in the new `wrapped`. A
  consumer that matches `kind` exhaustively gains a value to handle, and one
  that rejects unknown fields gains three to accept. `--help` states that a
  path leaving the project is unjudged even inside a registered `[services]`
  checkout. `/ductus:analyze` step 4 and `scripts/analyze/detect.py` run it on
  each scenario and count the files examined. An adopter counted 64 citations
  of a bare `spec.md` in scenarios across two repositories, each read as
  unjudged.
- **`ductus exec` judges each scenario's `§` references at `/ductus:analyze` step 4 (spec 067, AC8).**
  The walker scans the spec's `spec.md` and each `.md` under its `scenarios/`
  in one dispatch, merged before the result is bounded: a scenario's
  references carry `path`, its unresolved anchors read `<path>: <anchor>` and
  count as advisory, and the result carries `scenarios-examined`, 0 for a spec
  with none. A scenario that cannot be read, and a `scenarios/` that exists
  and cannot be listed or leaves the project, count in
  `anchor-target-unreadable`, so neither reads as a spec with no scenarios.
  Before this an exec analysis judged `spec.md` alone.
- **`brief-re-read` returns the loop's figures, counted from the round directory (spec 078, AC17).**
  Every result gains `rounds` (round directories holding a `round.toml`, this
  call's included), `re-reads-with-findings` (complete results holding a
  MUST or SHOULD on a path of their round) and `elapsed-seconds`, whole
  seconds since `start-fix-rounds` recorded `start.toml`, with
  `elapsed-absent-reason` when it is null: rounds started before the start
  was recorded, or a clock earlier than it, never read as zero seconds. The
  command and reference quote them at the budget stop instead of the host's
  count. An existing consumer that reads fields by name is unaffected, and
  one that rejects unknown fields gains four to accept. An adopter's review
  on 0.60.0 had run four re-reads over about four hours, each extra one put
  to the operator as "one more" with no count or time.

### Changed

- **The re-read brief's severity bar requires a trigger shown possible in the project (spec 078, AC16).**
  A finding of any severity, MUST, SHOULD or NOTE, is raised only on a case
  whose trigger the project's code, configuration, history or deployment shape
  shows possible; a case shown only by building it is no finding, as spec 020's
  AC18 has it for the passes, and "a contrived input" is no longer a NOTE. The
  same adopter's re-reads each opened on a contrived case in the previous
  round's fix.
- **BREAKING: a stored rule verdict stands while the sections it concerns are unchanged (spec 082, AC7).**
  `write-analysis` records `section-digest`, a 16-hex digest of each heading's
  own text in the design subjects (`<path>#<Heading>`, and `<path>` for the
  text before a file's first heading). After an edit, a rule that did not
  fire, was addressed or was waived is judged again against the changed
  sections alone, and `discover-rule-files`' `verdicts` gains `rejudge`,
  `changed-sections` and their counts. A stored finding stands while its own
  section is unchanged, and its rule is judged in full once that section
  changes. A finding at a section unchanged since its rule's not-fired or
  addressed verdict is no longer stored or, under `ductus exec`, counted: it
  is a contradiction alone, and `contradictions` entries gain `at`, `message`
  and `counted`. A record written before this falls back to the whole files,
  so `subjects-changed` now arises only for one without `section-digest`, or
  when a design subject cannot be read. A pure reordering of sections leaves
  every verdict current; a heading holding a control character, a Unicode
  line break or a noncharacter no longer reads as changed on every run; and a
  verdict's citation that reaches a file outside the feature through a
  linked directory is refused at any component of its path, not only at the
  cited file (`BE-INPUT-004`). An adopter's third analysis had judged
  a sentence its plan held unchanged through two earlier analyses a MUST,
  because every task between them edited the plan elsewhere.
- **BREAKING: a review no longer loads a rule file its header gives to analysis (spec 020, AC17).**
  `discover-rule-files`' review selection leaves out each file whose header,
  its text before the first `##` heading, names `/{project}:analyze` and not
  `/{project}:review` (as shipped, or with the project's name substituted),
  and names it in a `rule file … is judged by analysis alone` notice. Of the
  shipped files, seven name analysis alone and so are judged by analysis
  only: `accessibility-frontend.md`, `api-backend.md`,
  `concurrency-backend.md`, `observability-backend.md`,
  `performance-backend.md`, `performance-frontend.md` and
  `reliability-backend.md`. A header naming neither command keeps its file,
  and `all`, the analysis selection, is unchanged. An adopter's review on
  0.60.0 had judged concurrency rules out of reach of its code while the
  analysis of the design that code implements judged them violated.
- **A review records a finding only when its trigger can occur in the project (spec 020, AC18).**
  A pass's finding, an observation or a reviewer's claim is recorded only
  when its trigger is shown possible in the project's code, configuration,
  history or deployment shape, as well as its mechanism checked against the
  tree; a case shown only by building it is a false positive, recorded
  nowhere and fixed in no round. A review subagent returns that evidence with
  each finding and observation, and the host drops one with none
  (`framework/reference/review.md` §Run review passes and §Passes in a
  subagent). A fix round's re-read is held to the same bar (spec 078, AC16).
- **`/ductus` writes the shared-constitution links into `AGENTS.md` for every agent layout (spec 055, AC13).**
  With a `[constitutions.*]` entry registered, a Claude Code project now gets
  `- See [{alias}](<{document-path}>)` links in `AGENTS.md` beside its
  `CLAUDE.md` `@` imports, so an agent reading `AGENTS.md` alone is pointed
  at the shared constitutions too; Pi, whose native rules file is
  `AGENTS.md`, is unchanged. The block is appended with its markers where
  `AGENTS.md` has none, and the append is reported. The path sits in angle
  brackets, so a space in a registered path keeps the link whole.
- **Lockfile, net from 0.60.0 to the release head: 6 dependencies change version, none is added or removed, and no advisory is open.**
  Counted as the `[[package]]` name and version pairs of `runtime/Cargo.lock`
  at `ductus-v0.60.0` against the release commit (`git diff ductus-v0.60.0
  HEAD -- runtime/Cargo.lock`), the crate's own version aside; the lockfile's
  268 package entries stay 268. `form_urlencoded` 1.2.2 to 1.2.3, `hashbrown`
  0.17.1 to 0.17.2, `toml` 1.2.1 to 1.2.2, `toml_parser` 1.1.5 to 1.1.6, `url`
  2.5.8 to 2.5.9 and `walkdir` 2.5.0 to 2.5.1, each the highest compatible
  with Rust 1.89. `scripts/check-runtime-currency.sh` scanned 268 crates and
  found no advisory, and the manifest's dependencies are unchanged.

## [0.60.0] — 2026-11-20

A minor release with breaking changes: 109 of the 287 entries below are
marked BREAKING, under Added, Changed, Removed and Fixed. Auggie, Antigravity
and OpenCode are no longer supported agents and Windows is no longer a
published platform; Claude Code and Pi remain, and on Pi a command now runs
under the colon spelling too. A twelfth rule file, `security-cross.md`,
ships, the eleven others were re-reviewed against their sources, and
`BE-AUTHN-020` and `BE-DEPS-006`, added after 0.59.11, are removed before a
release shipped them. A review judges every acceptance criterion and is owed
for code, an analysis for the project's artifacts. Six migrations are new
(`claude-md-import-syntax`, `constitution-import`,
`memory-routing-leaves-claude-md`, `remove-legacy-constitutions-region`,
`strip-inbox-guidance` and `substitute-seed-placeholders`), and the MSRV
stays 1.89. Entries from 2026-11-02 on were written as their changes
landed; those for the 419 commits before were drafted from history.

### Added

- **`check-step-references` judges the `##` sections of each reference a command's numbered steps reach (spec 026 task 8, from 071's scenario `reference-sections-are-reached-by-a-step`).**
  An agent reads a reference section only where a step names it, so a section no
  step names is never read; 23 of 93 had gone unreached when this was found.
  Each `framework/reference/<command>.md` is judged against
  `framework/commands/<command>.md` by the citation walk spec 080's tool loading
  makes, extracted as `tool_loading::cited_sections` so there is one walker, but
  started only from the numbered steps under `## Instructions` (each with its
  continuation lines, sub-steps and bullets): a citation in the command's
  purpose, scope boundaries, a flags table, the Instructions preamble, a note
  after the steps or a pipeline-position paragraph reaches nothing. From a step
  the walk follows a `§` and its heading, a bold span equal to a heading, or a
  markdown anchor link `(#slug)` to a heading of the document holding it,
  through reached sections in turn. A `##` section reached by none is an
  `unreached-section` finding. A section kept for a reader outside the command
  carries `<!-- audit:kept <reason> -->` on the first non-blank line beneath
  its heading and is listed exempt with the reason; the marker is an `empty-exemption` finding
  with no reason, a `stale-exemption` finding on a section a step reaches
  anyway, and a `misplaced-exemption` finding anywhere else, so an exemption
  cannot outlive its cause. Four sections carry it: `Severity tiers` and
  `Record fields` in `framework/reference/analyze.md`, `Cross-spec context
  checklist` in `framework/reference/plan.md` and `Blocking semantics` in
  `framework/reference/review.md` (+158, +83 and +86 bytes). The result gains
  `reachability` (one entry per reference: `reference`, section count, exempt
  sections), a bounded list that moves to `lists-file` past `[artifacts]
  read-size-bytes` with `reachability-count` inline, and the counts
  `sections-examined`, `sections-exempt`, `sections-unreached` and
  `references-without-sections`. A reference with no `##` section is counted
  apart, never a pass, and a reference or command that cannot be read is a
  skip. Family 34 (`scripts/audit/step-reference-integrity.sh`) renders the new
  findings, so `scripts/audit/run-all.sh` fails on an unreached section. Over
  this repository: 74 sections in 13 references, 4 exempt, 0 unreached, 6
  references with no section. With `/plan` step 9's citation of `Validation
  gate` reverted and the exec-path note left, the check reports it and exits 1.
  Nothing runs in an adopter's project, where the primitive still reports that
  it examined nothing. The result is 2,886 bytes inline here (1,404 before) and
  the `CHECK_STEP_REFERENCES` description grows by 1,276 bytes (1,560 to 2,836).
- **`tests/bounded_list_gates.rs` fails when a command step states a bounded list's emptiness by the list rather than its `-count` (spec 080, scenario `a-step-gates-on-a-bounded-list-by-its-count`).**
  Past `[artifacts] read-size-bytes` a bounded result is `form: "file"`: each
  moved list is absent and only its `-count` stays inline, so a step that read
  "an empty `unfolded`" there read an absent list as an empty one. The test
  derives the bounded list names from the served tools' output schemas (a list
  property beside its `X-count`, in a tool that also has `lists-file`), scans
  `framework/commands/*.md` for `empty`, `non-empty`, `no` or `zero` before such a
  name, or the name followed by `is`, `are`, `was` or `were` and `empty`,
  `non-empty` or `absent` (with `not`, and a `field` or `list` between), and fails on one not in
  a short allow-list that states a reason for each entry. It also fails on an
  allow-list entry that no longer appears. It fails on the command text before
  this change ("non-empty `updated`", "empty `rewritten`", "empty `unfolded`") and
  passes after it. A phrase that names no list (a bare "when the field is
  empty") is outside what a phrase scan can see.
- **BREAKING: `append-question`, `append-task`, `label-criteria`, `mark-criterion`, `mark-task`, `set-status`, `append-inbox`, `remove-inbox-item`, `merge-managed-block` and `merge-permissions` write a file only when it is unchanged since they read it (spec 022 task 38; the owner's BE-TXN-002 class decision, 2026-11-16).**
  Each read its file, computed the replacement and renamed it over the file
  with no guard, so a change another writer landed in between was dropped.
  Each now reads through the shared `ReadSnapshot` guard (041 task 27) and
  renames only when a re-read under the project-wide lock matches; otherwise
  it returns a changed-since-read error naming the file and writes nothing,
  and the caller runs it again. `append-inbox`, `merge-managed-block` and
  `merge-permissions` also refuse a file that appeared after they read it
  absent. `append-task` keeps its advisory lock, which excludes only other
  `append-task` calls, and the guard covers the `mark-task` or `prune-tasks`
  that takes no lock. Any writer that does not go through the guard still
  takes no lock, so the window narrows to one re-read and one rename and is not
  closed. `merge-status-hook` stays unguarded under spec 080's departure. A new
  unit test fails when a non-test `write_atomic` or `write_atomic_bytes` call
  sits outside the guard and a named allowlist, and `line_ending_discipline`'s
  subject now counts the guarded writes, so a primitive does not leave the lint
  by taking the guard.
- **BREAKING: `derive-dependencies` and `derive-references` write a spec only when it is unchanged since they read it (spec 067 task 36; the owner's BE-TXN-002 class decision, 2026-11-16).**
  Each read a `spec.md`, spliced its frontmatter and renamed the result over
  it with no guard, so a writer landing between the read and the rename was
  dropped. Each now reads every spec through the shared `ReadSnapshot` guard
  (041 task 27) and, under `write`, renames only when a re-read under the
  project-wide lock matches. A spec that changed is left as the other writer
  made it and named in the new `refused` list (with `refused-count`, both
  absent at zero; the list is absent in the file form, where the lists file
  holds it whole), never counted in `updated`; the walk goes on, so the other drifted
  specs are written. The CLI exits non-zero on a refused spec and names each on
  stderr, so the pre-commit hook stops and the next run reads it as it now is.
  A writer that does not go through the guard takes no lock, so the window narrows to one
  re-read and one rename and is not closed. Read-only runs are unchanged.

- **BREAKING: `invalidate-review` and `rewrite-spec-links` write a file only when it is unchanged since they read it (spec 051 task 37; the owner's BE-TXN-002 class decision, 2026-11-16).**
  Both read a project file and renamed the result over it with no guard, so a
  `write-review` or an editor landing in between was dropped; for
  `rewrite-spec-links` the window spanned the whole sweep, since every file is
  read before the first is written. Each now reads through the shared
  `ReadSnapshot` (spec 041 task 27) and writes only when the file still holds
  what it read. `invalidate-review` fails with the `changed-since-read` error
  naming `review.md`, writing nothing, so the fold runs the step again.
  `rewrite-spec-links` goes on past a refused file and reports it in the new
  `refused` result field (`refused-count` inline, moved to the lists file past
  `[artifacts] read-size-bytes` with the other lists), never counting it
  `rewritten`; a non-empty `refused` is an unfinished sweep, and
  `/ductus:fold` and `/ductus:consolidate` run it again before
  `retire-feature`. Breaking under `BE-COMPAT-001`: two new result fields, and
  a new error where `invalidate-review` used to write. A writer that does not go through
  the guard takes no lock and, landing between the re-read and the rename, is
  still replaced.

- **BREAKING: a prune apply writes a file only when it is unchanged since it read it (spec 041 task 27, from 041's analysis; the owner's BE-TXN-002 class decision, 2026-11-16).**
  `prune-tasks`, `prune-plan` and `prune-project` each read a file, compute and
  renamed the result over it with no guard, so an `append-task`, `mark-task` or
  other writer landing between the read and the rename was dropped. Each now
  reads through the new shared guard, `ReadSnapshot` in
  `runtime/src/primitives/mod.rs`, which keeps what it read (or that the file
  was absent) and, on write, takes one project-wide lock
  (`.ductus/locks/compare-and-set.lock`, taken after `append-task`'s when a
  caller holds both, waiting at most `SESSION_LOCK_TIMEOUT`), reads the file
  again through the same containment check and renames only when the two reads
  match; the other writer's change stays. A refusal is the new operational
  error `changed-since-read` naming the path. A single-feature `prune-tasks` or
  a `prune-plan` apply fails with it; an `all` `prune-tasks` apply writes every
  other spec and lists the refused one in `skipped` with the new reason
  `changed-since-read`; `prune-project` goes on past a refused file and reports
  `written` and `refused` (two new result fields), with `applied` `false` once
  any is refused. A home it creates is written create-only, so one that
  appeared since the apply looked is `kept`. A writer that does not go through
  the guard takes no lock, and one landing between the re-read and the rename
  is still replaced. Breaking under `BE-COMPAT-001`: a new value of `skipped`'s
  response enum, and a new error where an apply used to write. The helper is
  reused by the other primitives that read a project file and replace it
  (spec 022 task 38, 066 task 24, 067 task 36, 051 task 37, 057 task 27).

- **BREAKING: `relocate-audit-records` writes a file only when it is unchanged since it read it (spec 057 task 27; the owner's BE-TXN-002 class decision, 2026-11-16).**
  The sweep read `spec.md` and each artifact, then renamed the result over
  each with no guard, so a writer landing between a read and its rename was
  dropped. It now reads through `ReadSnapshot` (`runtime/src/primitives/mod.rs`,
  spec 041 task 27) and writes each artifact, in the order its block appears in `spec.md`, and then `spec.md`
  only when each is unchanged, under the project-wide compare-and-set lock;
  the other writer's change stays. A refusal before anything is written is the
  `changed-since-read` error. A refusal after an earlier file was written is
  the new error `relocation-incomplete` (`PrimitiveError::RelocationIncomplete`),
  naming the files written, the file refused and the disagreements found for the files
  written (a re-run finds those copies equal and reports none); `spec.md` still carries its
  blocks, so re-running finishes the relocation. Breaking under
  `BE-COMPAT-001`: two new operational errors where the sweep used to write.
  A writer that does not go through the guard takes no lock and can still land between the
  re-read and the rename.

- **BREAKING: a review judges every acceptance criterion, and the pre-`done` gate judges none (spec 066 task 27, scenario `criteria-are-verified-in-review`).**
  `write-review` takes `criteria`, one verdict per criterion by label or index
  (`--criteria-json FILE` on the CLI), and records `criteria-judged`,
  `criteria-met`, `criteria-not-met` and `criteria-verdicts`, each met
  criterion's digest; a not-met verdict sets `blocking`, and a malformed verdict
  is refused before anything is written. `compute-review-scope` reads
  `review-owed: true`, whatever the window holds, while `spec.md` holds an
  unticked criterion or a ticked one the record's `criteria-verdicts` does not
  hold, naming each in the new `criteria-owing` with `criteria-owing-count`.
  `check-review-gate` blocks the new `unticked-criterion` after `unchecked-task`,
  and its `review-owed` and `must-violations` messages name the criteria.
  `ductus exec review` sends `verifyCriteria` and ticks each verdict, and
  `ductus exec implement` no longer sends it. `invalidate-review` drops the four
  criteria fields. Breaking under `BE-COMPAT-001`: a new value of a response enum
  the schema does not document as open to new values, an extension request moved
  from one walk to another, and `review-owed` reading true where it read false;
  the argument and the fields are additive.

- **BREAKING: `write-analysis` refuses an `unexamined-by-reason` reason outside the closed set (spec 066 task 26).**
  The reasons are the data model's table, held in one runtime list
  (`UNEXAMINED_REASONS`) that a test keeps equal to it; a call recording any
  other fails naming the argument, on the CLI and MCP paths alike, before it
  writes. Every reason the exec walker and the analyze command's steps supply is
  in the set. Breaking under `BE-COMPAT-001`: a call that recorded an unknown
  reason now fails.

- **BREAKING: `write-review` and `write-analysis` write their record only when it is unchanged since they read it (spec 066 task 24; the owner's BE-TXN-002 class decision, 2026-11-16).**
  Each read `review.md` or `analysis.md` through several helpers (the waivers,
  decisions and rule verdicts it carries forward) and renamed the regenerated
  record over it with no guard, so a record written in between lost them. Each
  now takes a `ReadSnapshot` (041 task 27) before its first read and writes
  through `write_if_unchanged`; a record changed, created or removed since is
  kept and the call fails with `changed-since-read` naming the path, so the
  command re-runs the write from its argument files. A writer that does not go through
  the guard takes no lock and can still land between the re-read and the rename.
  Breaking under `BE-COMPAT-001`: a new error where a write used to succeed.

- **BREAKING: the pre-`done` gate reads an affected spec only through paths contained in the project (spec 066 task 23, from 067's review; `BE-INPUT-004`).**
  `check-review-gate` contains each affected spec's `spec.md`,
  `scenarios/` directory and scenarios before it probes, lists or opens them:
  one that is a symbolic link leaving the project, or a scenario whose name is
  not UTF-8 and so has no spelling to contain, is never read, and its
  `cross-spec-impact` entry is the new state `outside-project`, with the link
  or the reason in a new optional `detail`, holding `done` as `undischarged`
  does. Before,
  an outside file decided the verdict: the same file passed the gate with a
  back-link and blocked it without one. `check-artifacts`'
  `cross-spec-impact-drift` family inherits the state and names it in its
  finding. Breaking under `BE-COMPAT-001`: a new value of a response enum
  the schema does not document as open to new values.

- **A review states what it read, and a read is credited once (spec 081).**
  `write-review` records `unread`, the scope entries the passes did not
  read, whenever the run states it (`[]` for a whole read), and refuses an
  entry outside the run's scope; on the CLI, `--unread-json FILE`. A commit
  naming no spec and touching no spec directory, in every open spec's window,
  no longer owes each of them a review: `compute-review-scope` credits each
  of its paths once a review record committed in the window states it read
  the path (its window holds the commit; no skipped pass, budget stop or open
  fix-round loop), listing it in `excluded`. A record
  with no `unread`, every one written before this change, credits nothing.

- **A spec's decided rule departures are recorded, not lost (spec 073).** A
  `departs:` entry under `## Applicable Rules` states once that the spec
  deliberately does not satisfy a rule of any tier. Analyze records the rule
  `waived` with the entry's reason and attribution and raises no finding for
  it, and review derives a departure waiver from the same entry, so neither
  command reads the decision twice. The entries below carry the pieces:
  `departures.rs`, `check-departures`, the review waivers and `process-waivers`
  expiry, and the `waived` verdict with its `## Waived rules` section. The
  shapes are in `specs/073-decided-rule-departures-are-recorded/data-model.md`.
- **`ductus exec` analyze counts a departure entry's findings (spec 073).**
  Step 5.1 of `/analyze` dispatches `check-departures`, and `AnalyzeTally`
  counts its findings in the tier each carries (its `tier` field), and none of
  its `unexamined` entries, which steps 4 and 5 already record once for the
  whole spec, so a malformed departure entry
  is a Blocking finding in `blocking-findings`, and so in the record's
  `blocking`, instead of being dropped. The walker no longer binds the
  context's `unexamined` list to `write-analysis`' `unexamined` count, which
  `check-departures`' result would have made fail to bind.
- **`write-analysis` lists the rules a record holds as `waived` apart from
  the rest (spec 073).** The body gains a `## Waived rules` section, absent
  when no rule is waived, with one line per rule (``- RULE (MUST) — reason — decided by `EMAIL` at
  `TIME` — cited `§anchor` ``; the email, time and citation are in code spans so
  the committed record passes `MD034`, the time is absent when unattributed and
  the citation when the entry has none), MUST-tier rules
  first and then every other, each in rule-ID order. The result gains `waived`
  (rule, tier, reason, decided-by, decided-at, cited, in the same order) and
  `waived-count`, bounded as `contradictions` is: inline within
  `[artifacts] read-size-bytes`, and past it in the one file `lists-file`
  names, with `waived-count` inline in both forms. The record gains no field;
  `rule-verdicts` already holds each verdict.
- **A spec's departure entries are read and attributed from history by one
  shared module (spec 073).** `runtime/src/primitives/departures.rs` parses
  the `departs:` entries under `## Applicable Rules` (HTML comments and fenced
  code skipped, each structural problem reported with its line, a malformed
  entry waiving nothing, a wrapped reason ending at a thematic break as
  CommonMark renders it) and attributes each rule to the author email and time
  of the first commit of the unbroken run ending at `HEAD` in which the spec
  holds an entry for it, following a renamed spec directory. A shallow or
  absent repository, an entry not in `HEAD`, or an unreadable revision is
  `unattributed`. `check-departures`, `write-review` and `process-waivers`
  call it.
- **`check-departures` lists a spec's decided rule departures and checks them
  (spec 073).** Given a `feature`, it returns each well-formed entry (`rule`,
  its `tier`, `reason`, `cited`, `decided-by`, `decided-at`, `line`),
  `findings` of family `departure-entries`, and `unexamined`. Blocking: an
  entry missing a required field or malformed (a second rule ID, another or a
  repeated child key), a rule no loaded rule file defines (unknown, or retired when the spec's
  `retired-rules:` lists it too), and a rule two entries name, which leaves
  both entries listed, each attributed, the second reported. Advisory: a deprecated rule
  and a `cited:` that is not exactly one `§anchor` reference (an unresolved
  reference is step 4's finding, reported once). A rule the project never installed
  (`rule-not-installed`) and a citation that cannot be judged
  (`anchor-target-unjudgeable`, `anchor-target-unreadable`) are `unexamined`,
  never a pass. The rule's state is `check-rule-ids`' reading of every rule
  file in the rules directory. With none loaded (`check-rule-ids` examining 0
  files) it records each entry whose rule token is, whole, shaped like a rule ID
  `rule-citations-not-checked` and raises no rule-state finding, as step 5 does
  for its citations; a token shaped like no rule ID stays an unknown-rule
  finding. Step 14 skips a well-formed departure entry's rule, and the analyze
  reference and the review reference's waiver paragraph point at the compiled
  descriptions instead of restating them. The tool needs an allow entry
  (`mcp__ductus__check-departures`), which `/{project}:configure` writes. Its
  three lists are bounded (BE-PAYLOAD-001): past `[artifacts] read-size-bytes`
  `form` is `file`, the lists move whole to the file `lists-file` names, and
  each list's count stays inline in both forms.
- **`write-review` waives the rule of each departure entry, and
  `process-waivers` expires the waiver when the entry is gone (spec 073).**
  Each well-formed entry under `## Applicable Rules`, of any tier, becomes a
  `waivers:` entry for its rule at every file the rule fires at that no waiver
  holds, carrying `departure: true`, the entry's reason, and `waived-by` and
  `waived-at` from the commit that introduced it (`unattributed` and the
  run's own time when history cannot say). A rule firing nowhere stores
  nothing, a hand `--waive` on the same rule and file stays the first claim,
  and a SHOULD departure leaves `should-violations` at 0. A malformed entry,
  or a rule a second entry names, derives no waiver and is listed in the
  result's `malformed-departures`, which with `superseded-deferrals` moves
  past `[artifacts] read-size-bytes` to the result's `lists-file`, beside
  `analyze-stale-paths`, each with a count inline. `process-waivers` expires
  a marked waiver whose rule has no well-formed entry, on any run, restricted included, with
  its `waiver expired:` notice, and bounds its result's lists, a breaking
  change entered under Changed.
- **A rule verdict can be `waived` (spec 073).** `write-analysis` accepts the
  bare `{verdict: waived}` in `rule-verdicts` for a rule whose trigger fires
  and whose spec holds a well-formed departure entry, and fills `reason`,
  `decided-by` (the author email of the commit that introduced the entry, or
  `unattributed`), `decided-at` (absent when unattributed) and `cited` (absent
  when the entry has none) from the entry on every write, a carried verdict
  included, so attribution follows history rather than a digest. It refuses
  `waived` for a rule with no well-formed entry and `finding` or `addressed`
  for a rule with one, each leaving no record. A `waived` rule is no finding:
  it counts into no tier, so `blocking-findings` stays 0 and
  `dispositions.waived` is untouched. A carried `waived` whose entry is gone is
  recorded unjudged. The exec walker records `waived` for any fired answer on
  a rule the spec departs from, a pass citing `addressed-by` included, and
  nothing for a plain pass. `discover-rule-files`' `reusable` list can carry a
  `waived` verdict.
- **`/{project}:analyze` records each loaded rule's verdict and judges a rule
  again only when its subjects changed (spec 082).** `analysis.md` gains
  `rule-verdicts` (each rule that fired: `addressed` with a `cite`, or
  `finding` with the `tier` it was counted in, `at` and `message`), `unjudged-rules` and `rule-digest` (a
  digest of each loaded rule file's rule IDs, Statements and Verifications).
  A stored verdict is current while the spec's design subjects (`spec.md`,
  `plan.md`, `data-model.md`, each scenario, never `tasks.md` or `review.md`)
  and its rule file are unchanged, and a rule is then not judged again.
  `discover-rule-files` takes `feature`, under which, with `all` and `rules`,
  its result gains `verdicts` (`judge`, `reusable` and `reason`, inline or past
  the read size in its lists file, with `reusable-count` and `judge-count`
  inline). `write-analysis` takes `rule-verdicts` and `judged-rules` (CLI
  `--rule-verdicts-json` and `--judged-rule`), reads the rule files itself,
  carries each current verdict forward, records every assessable rule the run
  neither judged nor reused under `unjudged-rules`, records an `addressed`
  verdict whose citation does not resolve there too, refuses a verdict naming
  no loaded rule and a tier count below its `finding` verdicts, and returns
  `contradictions` (a finding over unchanged subjects after a not-fired or
  `addressed` verdict, advisory) beside a new `## Contradictions` body section.
  `assessSpecQuality`'s response takes an optional `addressed-by`, and the exec
  walker sends no request for a rule whose verdict is current.
  `rule-assessments-not-checked` now also counts each rule left unjudged, on
  the interactive path as under `ductus exec`.
- **BREAKING: `check-review-gate` blocks `done` while the analysis names an unjudged rule
  (spec 082).** After `analyze-stale`, a current, passing `analysis.md` that
  lists any rule in `unjudged-rules` blocks with `blocked-by: unjudged-rules`,
  naming the rules and sending the operator to `/{project}:analyze`, which
  judges only them. A record with no `unjudged-rules` field blocks on nothing
  new.
  Breaking under `BE-COMPAT-001`: `blocked-by` gains `unjudged-rules`, a new
  value of a response enum the schema does not document as open to new values.
- **`check-artifacts` runs `/{project}:analyze`'s Spec integrity and Plan
  consistency checks.** Two new blocking families, `spec-integrity` (no
  Acceptance Criteria section, no criterion from `clarified` on, an empty or
  placeholder criterion, an open question in the body from `clarified` on) and
  `plan-consistency` (when `plan.md` exists: no link to or naming of its spec,
  no decision, an Affected Files section listing nothing outside comments). The three judged items stay with the host in
  step 15, and an exec walk records them unexamined under the new reason
  `integrity-not-checked` (spec 067).
- **BREAKING: `check-artifacts` reports a path cited by line number, in a
  new blocking family, `line-number-citations`, that examines a spec at every
  status (spec
  067, AC5).** A citation of a path by its line number in a spec's `spec.md`,
  `plan.md`, `tasks.md`, `data-model.md`, `research.md` or a scenario (a line
  drifts as the code moves; spec 050's AC24 forbids one) is a Blocking
  finding, counted in `blocking-findings`, in a `draft` spec as in a `done`
  one, with no history read and nothing grandfathered. A line inside a fenced
  block and a blockquote line are skipped; an HTML comment is read. The
  finding of a bare `:N` code span says to write a port with its host. An
  artifact that cannot be read is recorded as the other families record one.
  0.59.11 had no such family, so a citation it passed now blocks the spec's
  next `/analyze`. Removing a citation the family flags is a mechanical edit,
  by the definition spec 066's mechanical-edit rule shares.
- **`check-rule-ids` reports a citation of a shipped rule whose file is not
  installed as `not-installed`, not missing.** A project with
  `surfaces = ["backend"]` never receives `security-frontend.md`, so a
  citation of `FE-XSS-001` was a blocking unknown rule. The runtime now holds
  an index of the shipped rule IDs (`shipped_rule_ids.txt`, generated from
  `framework/rules/*.md` and held to it by a test), and a citation no given
  file defines but the index does is reported in the citation's
  `not-installed` flag and the result's `not-installed` list, each omitted
  when empty. `/{project}:analyze`'s exec walk records each under the new
  unexamined reason `rule-not-installed` (spec 033, spec 067).
- **`check-rule-ids` passes a spec's record of a removed rule.** A `spec.md`'s
  optional `retired-rules:` frontmatter list names the removed rules it records
  the retirement of; a citation of a listed ID that no loaded rule file defines
  is reported `retired` (the citation's `retired` flag and the result's
  `retired` list, each omitted when empty), neither found nor missing. A
  citation in any other file, or of an unlisted ID, stays missing, and the list
  line is no citation (spec 067).
- **`performReview` names its pass and carries the rule-file mapping.** The
  walker takes each request's `pass` from its step's `**pass**` prose, so it
  is no longer empty; `rule-files` holds the loaded files that pass applies
  (`security-*.md` the security pass, `performance-*.md` efficiency, every
  other file quality; reuse and simplicity none), and two fields trail it:
  `loaded-rule-files`, every loaded file with its pass, and `passes-run`, the
  passes the procedure performs. The security pass applies a rule's
  **Not flagged:** routing from them. Spec 020.
- **Every MCP argument has a CLI form (spec 022).** The 14 structured
  arguments with none gain a `--{name}-json FILE` flag, read from a staging
  directory like `apply-manifest`'s (`BE-INPUT-004`) and refused naming the
  flag when outside one, unreadable, or not valid JSON for the field:
  `write-review` `--findings-json`, `--applied-waivers-json`,
  `--expired-waivers-json`, `--new-waivers-json`, `--observations-json`,
  `--expired-decisions-json`, `--decided-by-json` and `--fix-rounds-json` (a
  file holding `[]` is the empty list; no flag leaves it absent);
  `write-analysis` `--findings-json`, `--expired-decisions-json` and
  `--decided-by-json`; `process-waivers` and `process-decisions`
  `--fired-json`; `prune-project` `--replace-json`. Additive: no scalar flag
  and no MCP schema changes. A test in `src/main.rs` asserts each served
  tool's every argument has a flag or its `-json` form.
- **An exec `/analyze` assessment request carries the rule's Statement and its
  whole Verification.** `assessSpecQuality`'s `rule` object gains `statement`,
  the rule's first block quote, beside `id`, `verification` and `severity`; a
  host that resolved a Verification's "the Statement's allowlist" or "all four
  requirements" only from the rule file can now do so from the request. The
  Verification now runs from its `**Verification:**` marker to the rule's next
  `**Field:**` line or heading, every block between included, instead of
  stopping at its first blank line unless a pipe table followed: a wrapped
  paragraph still joins with single spaces, while a table, list or fenced block
  keeps its line breaks. Among the shipped rules only `BE-API-001` gains text
  (its header table already arrived since spec 080). The same reading serves
  `discover-rule-files`' `verification`, so a project rule with a paragraph or
  list after the first blank line is no longer cut. Every such request also
  carries `loaded-rule-files`, the basenames of the rule files the walk read,
  in its `rule-files` order (a listed file it could not read is absent), so a
  host applies a Verification's "where `{file}` is also loaded" yield from it,
  and a request carrying none makes the condition false (spec 060).
- **A pruned task number is never reissued.** `prune-tasks` writes a
  `next-task:` frontmatter counter in `tasks.md`, set past the highest task
  number the file held before it removed any section (never lowering one
  already higher), and `append-task` assigns the larger of the counter and the
  highest number present plus one, then advances it. A `--reset` keeps the
  file's frontmatter, counter included. A file with no counter numbers from the
  highest present plus one as before, and `append-task` never creates one; a
  counter at or below the highest present never causes a duplicate. A block
  that never closes, or a `next-task:` that is not a positive integer, is
  `malformed-tasks` and writes nothing. Reset and keep-pending prunes that
  remove a section now write the frontmatter block, so such a prune's output
  begins with `---`.
- **`read-deprecation-labels` returns the deprecation labels of rule files.**
  One entry for each line containing `**DEPRECATED`: the rules it labels, the
  rules in whose section it follows an earlier label, and its deprecating
  version and removal target or the reason it is malformed; a file that cannot
  be read is `unreadable`, not counted in `examined`. It shares
  `check-rule-ids`' label reader, so the two cannot disagree, and
  `scripts/audit/deprecation-removal.sh` (Family 40) calls it in place of its
  own parse (spec 067).

- **BREAKING: `check-artifact-size` measures the four project files.** `--project-files`
  (`project-files` over MCP) adds `AGENTS.md` and the spec root's `system.md`,
  `events.md` and `errors.md` to the check, against the same
  `[artifacts] read-size-bytes`; `feature` becomes optional, and a call naming
  neither is an argument error. The result keeps its shared keys and adds a
  `project-files` half beside the feature half, sharing no key with it. A
  file over the limit joins `oversized` as kind `project-file`, whose one fix
  is `/prune --project`; a missing one is reported as `absent`, an unreadable
  one goes in `skipped`.
  `--staged` measures each file's index copy against its `HEAD` copy, adding
  `head-bytes` and `refused` (over the limit, with no `HEAD` copy or larger
  than it), and exits 1 when any file is refused or unreadable, naming it on
  stderr. An exec `/analyze` seeds `project-files`, so step 17 records an
  oversized project file as an advisory finding. Its `oversized`, `skipped`
  and `notices` grow with a spec's scenarios, so they are bounded
  (BE-PAYLOAD-001) as `check-artifacts`' lists are: past `[artifacts]
  read-size-bytes` `form` is `file`, the three move whole to the file
  `lists-file` names, under `.ductus/results/check-artifact-size/` keyed by the
  feature or `project-files`, and `oversized-count`, `skipped-count` and
  `notices-count` stay inline in both forms. A consumer reading the three
  inline past the read size finds none. The staged exit reads
  `skipped-count`, and under `ductus exec` the walker restores the lists
  before the tally reads them.
  Breaking under `BE-COMPAT-001`: `oversized`, `skipped` and `notices`, inline
  in 0.59.11, are absent inline past the read size.
- **BREAKING: `/review` re-reads the fixes it makes.** Before the record is final, an
  independent agent re-reads each round of fixes the review made, unless it
  changed only markdown in the spec's own directory other than `analysis.md`,
  its scenarios and its data model, from a brief the runtime writes: the
  round's paths, derived from the repository, and a fixed severity bar. The
  reviewed spec's own `review.md` is never a round's path, and a re-read's
  finding on a path outside its round counts as a NOTE. A round that only
  applies the edits a re-read prescribed is checked by diff instead, a file's
  NOTE edits being the operator's to decline, and after the reported
  `budget` of re-reads that find a MUST or SHOULD the loop waits for the
  operator. A host that cannot start an agent prints the brief for the
  operator to run in a fresh session. Four primitives carry it —
  `start-fix-rounds`, `brief-re-read`, `read-re-read` and
  `check-prescriptions` — each refusing a `feature` that is not a feature
  directory name and writing only under the self-ignoring
  `.ductus/review-rounds/` and `.ductus/results/` and as git objects no ref
  points at, so a review still commits nothing; past `[artifacts]
  read-size-bytes` a result's lists move to the file `lists-file` names,
  with every count inline. `review.md` records `fix-rounds`, `re-reads`,
  `diff-checked`, `not-re-read`, `budget-stop` and `round-waivers`, its
  Summary naming each exempt and each waived round whoever wrote it, and
  `/implement`'s completion gate holds `done` while a round that needed a
  re-read has neither one nor the operator's waiver: `check-review-gate`'s
  `blocked-by` gains `fix-round-not-re-read`, which a script matching its
  values must accept. A record written before this passes with a notice that
  it does not say. `[review] re-reads-off = "<reason>"` switches the loop
  off, waiving each such round with that reason; an empty one counts as
  unset, and a malformed one halts the run before its first fix. Spec 078;
  spec 066, tasks 15 and 17.
  Breaking under `BE-COMPAT-001`: `blocked-by` gains `fix-round-not-re-read`,
  a new value of a response enum the schema does not document as open to new
  values.
- **`classify-spec-edit`: whether an edit reopens a `done` spec, from the
  diff.** A new primitive, an MCP tool and CLI subcommand, comparing `base`
  (`HEAD` by default) with the working tree, untracked files included. It
  answers `correction` when an edit to `spec.md`, `plan.md` or `tasks.md`
  changes no criterion, open question or unchecked task, and `reopens` with its
  reasons otherwise; a label or a tick alone is a correction, and a run's record
  is no edit. `/amend` calls it before offering to reopen a spec, and stays
  quiet on a correction. Spec 079, task 6.
- **`measure-reopenings`: whether review and analysis converged, from history.**
  A new primitive, an MCP tool and CLI subcommand. A reopening is one pass from
  `in-progress` to `done`, and it holds at most one review run and one analysis
  run, or a second of either only after the earlier run left a gating finding, was
  cut off with its fix-round loop still open, or recorded a skipped pass or a budget
  stop. It reports each reopening closed since a date or a ref, the last 30 days by
  default, each one it could not count and why, how many stored decisions and waivers
  each of its runs recorded from a person, and the `in-progress` specs with no unchecked
  task, and says so when the history is shallow or nothing closed. `/status`
  shows the counts on every run through `dashboard`'s new `reopenings` block.
  `/configure` allows it with the other tools. Spec 079, task 7.
- **`render-status`: the pipeline view `/{project}:status` shows, without
  `dashboard`'s structured payload (spec 080).** A new primitive, an MCP tool
  and CLI subcommand, taking no arguments and read-only but for the file named
  below. Its `rendered-markdown` is the view as one markdown fragment (the
  target block, the table of every spec, the summary, the cross-service
  references readout, the specs with work left grouped by next action and the
  `/{project}:target` prompt), rendered from the same read `dashboard` makes,
  and `project` is the slash-command namespace the view's hints name. The
  result is held to `[artifacts] read-size-bytes`: past it `form` is `file`,
  `lists-file` names the file under `.ductus/results/` whose `view` holds every
  line of the whole view, and the view is cut (done rows collapse to a count
  first). A read size that is not a positive whole number is an error.
  `/{project}:status` calls it in place of `dashboard`, and Pi's bridge calls
  it for its `/{project}:status` handler. `/configure` allows it with the other
  tools.
- **`read-section`: named sections of a markdown document, in one call (spec
  080).** A new primitive, an MCP tool and CLI subcommand taking `path` and one
  or more `heading` values (`--path` and `--heading` on the CLI). Each section
  comes back in the order asked, from its heading line, matched by exact text,
  to the next heading of the same or a higher level, outside fenced blocks and
  HTML comments; `not-found` lists the headings the document lacks, and a path
  leaving the project is refused. Past `[artifacts] read-size-bytes` `form` is
  `file`, `sections` is absent, `sections-count` stays inline and `lists-file`
  holds the sections. Each command that sends a step to a section of its
  reference carries a line saying to call it with the path and every section
  the step names, in place of reading the whole reference. `/configure` allows
  it with the other tools.
- **`render-help`: the guide `/{project}:help` prints, with no model turn (spec
  080).** A new primitive, an MCP tool and CLI subcommand, taking no arguments.
  It reads the installed help copy, the first of
  `{cli-config-dir}/commands/{project}/help.md` and
  `{cli-config-dir}/prompts/{project}-help.md` that exists, and returns `guide`
  (the markdown between the copy's first `---` line after its `## Instructions`
  heading and the last such line, each line wholly an HTML comment removed)
  and `project`, the slash-command namespace, read without any spec. No
  installed copy, or a copy with no such text, is an error naming what it
  looked for. Pi's bridge calls it for its `/{project}:help` handler.
  `/configure` allows it with the other tools.
- **`merge-status-hook` installs Claude Code's status and help hooks, and a new
  `ductus hook` subcommand answers them (spec 080).** `merge-status-hook` (MCP
  tool and CLI subcommand, `path` the settings file, repo-relative) runs
  `claude --version` with a timeout first. On 2.1.101 or later it adds, under
  `hooks.UserPromptExpansion`, one entry matched to `{project}:status` running
  `"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus" hook status` and one matched to
  `{project}:help` running `"$CLAUDE_PROJECT_DIR/.ductus/bin/ductus" hook help`
  (`ductus.exe` on Windows), each recognized by its command alone so one
  already present is not added again, and every other key and entry stays. The
  result's `action` is `created`, `updated` or `unchanged`; on an older
  version, or one it cannot read, it writes nothing and returns `skipped` with
  a `reason` and `claude-version` (null when none was read). `ductus hook
  status` and `ductus hook help` read the hook's input on stdin and print a
  block response whose reason is the pipeline view as fixed-width text, or the
  guide, so a typed `/{project}:status` or `/{project}:help` is answered with
  no model turn; each exits 1, with the error on stderr, when it cannot render.
  `/ductus` (step 10, for the `claude` agent) and `/configure` call
  `merge-status-hook` with `{cli-config-dir}/settings.local.json`, and
  `/configure` allows it with the other tools. The write is not guarded by the
  changed-since-read check (spec 080's departure).
- **On Pi, a command runs under the colon spelling every ductus text uses.**
  The commands, references, constitution and rules name a command
  `/{project}:<name>`, and Pi names it after its prompt template's file,
  `/{project}-<name>`, so on Pi those references named no command and reached
  the model as plain text. Pi's bridge now registers `/{project}:<name>` for
  each installed `.pi/prompts/{project}-<name>.md`. An alias submits the
  hyphen form with its arguments as typed input, through `sendUserMessage`
  with `expandPromptTemplates`, so Pi's own expansion applies; run while Pi
  compacts, it submits once Pi is idle, as Pi queues typed input until then.
  `/{project}:status` and `/{project}:help` run the same handlers as the
  hyphen commands, with no model turn. The hyphen form keeps working, and no
  installed file is renamed. A Pi project takes it on its next `/ductus`,
  which replaces `.pi/extensions/ductus.ts`. Spec 064, task 18, scenario
  `colon-form-commands-run-on-pi`.

- **`dashboard`, `discover-rule-files`, `check-artifact-size` and the config
  readers name every unknown key in a ductus-owned `.ductus/config.toml`
  section (spec 019; `CFG-ENV-012`).**
  One registry, `runtime/src/schema/config_keys.rs`, declares the keys each
  ductus-owned section recognizes: the plain keys, the fields of named
  `[constitutions]` and `[services]` entries and of
  `[[review.disabled-rule-files]]` entries, `[host]`'s legacy
  `cli-config-dir`, and the retired `[workflows]`. A key outside it is
  reported by name, never dropped and never an error, and only the key name is
  judged: a wrong-typed value stays its reader's to report.
  `discover-rule-files` (`[rules]`, `[review]`) and `check-artifact-size`
  (`[artifacts]`) add a notice, `dashboard` returns `config.unknown-keys`
  (later moved to the lists file past `[artifacts] read-size-bytes`, with
  `unknown-keys-count` inline) and `/status` renders a callout, and
  `Host::load`, `Paths::load`, `resolve-constitutions`, `resolve-references`,
  `derive-references` and `check-orphaned-references` warn once per process on
  stderr. `/ductus` names them in its summary, and the markdown-only paths of
  `/review`, `/status`, `/clarify`, `/plan` and `/analyze` compare by hand. A
  key that needs quotes is shown escaped, a tab or bidirectional control in a
  key or in the config path is escaped (`BE-INPUT-011`), and the warning drops
  a failed stderr write, so a broken stderr pipe no longer panics a primitive
  that read the config (exit 101). The installer's `§Project Configuration`
  now lists `[runtime]`, `[review]` `tech-stack-verified` and `[services]`,
  and a test holds it to the registry. Spec 019 (reviewed under 063), commits
  `e7044738`, `b482e029`, `9d4d8c76`.

- **`resolve-anchor` judges a pointer into a shipped command reference, and
  its result gains a `cross-document` count (spec 067, scenario
  `pointers-into-shipped-files-are-judged`; spec 071).** A reference on a line
  naming exactly one shipped document, and an explicit `**§X** in <shipped
  doc>` pair, resolves against that document's headings, the citing file's, or
  the constitution's markers, and is counted `cross-document`; it is zero
  wherever the installer's Shared Files manifest is absent, as in every
  adopter. A `**§X** in` pair resolves only when `X` equals a heading of its
  document, so a pointer that outlived a shortened heading is no longer
  prefix-matched. `check-step-references` treats `step N of §X` and `step N of
  **§X**` as qualified, like `X's step N`, so a pointer into a reference's own
  numbered list is no longer resolved against the command's steps. Commits
  `b772a6ac`, `36b3ddb6` and `083c4b08`. Later widened: a reference on a line
  naming any readable document is judged against it (`8651d466`), not only
  one naming a shipped document.
- **A new migration, `claude-md-import-syntax`, rewrites `CLAUDE.md`'s
  `@import <path>` lines to the `@<path>` form Claude Code loads, and
  `constitution-import` adds the constitution import (spec 002,
  `claude-md-template-imports-load`, task 5; spec 050,
  `the-constitution-fits-one-read`).** Claude Code imports a file only as
  `@path`, and a path ends at the first unescaped space, so the template's
  `@import AGENTS.md` named a file called `import` and loaded nothing. The
  `CLAUDE.md` template now imports `@.ductus/constitution.md` above
  `@AGENTS.md`. `claude-md-import-syntax` (`introduced_in` 0.60.0, no
  `sunset_after`) rewrites each `@import <path>` line outside a code fence
  (quotes dropped, spaces escaped, each line keeping its own ending), the
  `@import .ductus/constitution.md` line and a hand-added shared-constitution
  `@import` line included, and converts the constitution line instead of
  removing it; a line already in `@path` form stays as written.
  `constitution-import`, ordered after it on the `id` tie-break, adds
  `@.ductus/constitution.md` where nothing imports the constitution. It is
  a second entry because Pre-run Migrations runs only entries ordered after
  `last_applied`, so it is the only way to reach a `--ref=main` adopter whose
  `last_applied` is already `claude-md-import-syntax`. It inserts the line
  with the file's own line ending, and a second run reports converged and
  writes nothing. `/ductus` renders a shared constitution as
  `@{document-path}` under a claude-style agent, so the block loads at session
  start; `/target` step 4 skips a document an import already loaded. Pi reads
  `AGENTS.md`, keeps its plain link and `/target`'s read.
  `check-orphaned-references` reports a dangling `@path` import (an `@` that
  starts a token begins a reference; `x@.ductus/y` does not). Commits
  `96934872`, `d1e3ec96`, `e48df859`, `7ce72592`, `d5de121c`, `da52bc4c` and
  `1503ca58`.

- **`prune-project` segments, searches and rewrites the project files (spec
  041 task 23, items 1-5; spec 075).**
  The deterministic half of `/{project}:prune --project`. It segments
  `AGENTS.md` and the spec root's `system.md`, `events.md` and `errors.md`
  into entries (heading, item, row, block), each reported by locator, size
  and digest, never text; an HTML comment and a `/ductus`-managed BEGIN/END
  region are never entries. A catalog entry carries its identifier, and a
  preview searches the tracked files outside the spec root for it literally;
  with no repository the search is reported unavailable, never as zero hits.
  An apply removes or replaces entries by digest, refuses the whole write
  when any entry changed, collapses blank lines only at the seams it changed,
  keeps line endings, and creates a missing home from its compiled-in
  template at mode 0644. It is wired as an MCP tool, a CLI subcommand and an
  `ductus exec` dispatch, listed in `framework/runtime-tools.txt`, and
  allowed in the `claude` and `auggie` configure blocks when it landed. This
  release drops Auggie (the entry "Auggie, Antigravity and OpenCode are no
  longer supported agents"), so the `auggie` block no longer exists and only
  `claude`'s allows it; Pi reads the tool from its bridge. Later entries bound
  its result, contain its reads (spec 041, task 25) and write a file only
  when it is unchanged since it was read (task 27). Spec 041, task 23.
  `2f61d67d`
- **`/{project}:prune --project` walks the project files (spec 041 task 23,
  item 6; specs 075 and 050).**
  The command gains `--project`. Step 1 refuses it with `--all`, `--reset` or
  `--force` before reading anything, and routes to a new section, Project
  files, which previews with `prune-project`, judges each entry by the
  constitution's §drift-prevention test and §spec-phase definitions,
  confirms through `gate-confirm`, writes the home before applying, and
  reports what moved, what was removed and why, and each drafted upstream
  report. Step 8 routes a plan section's project-only knowledge by the same
  definitions, to `AGENTS.md` only when every session needs it. The prune
  reference adds the markdown-only walk, in which a missing home is created
  with its `#` heading only. The scope names its two widenings: reading
  source for the catalog search, and writing a confirmed entry into its home.
  Spec 041, task 23. `ab5eaa49`, `314e7571`
- **Two rules: `CFG-ENV-013` owns a secret at rest, and `BE-DATA-013` keeps
  a credential header to its origin (spec 008, tasks 21 and 26).**
  `CFG-ENV-013` (MUST, `configuration-cross.md`) covers a secret written in
  source or a container-build manifest, committed to version control, passed
  to a container build, or placed into an image or other build artifact, in
  every project shape. A secret may be committed encrypted only when each
  service has its own copy in each environment, with its own value, key and
  decryptor, and no developer can decrypt any copy, among other conditions.
  `BE-DATA-003` keeps only runtime sourcing and `BE-DATA-008` only container
  runtime injection. `BE-DATA-013` (MUST, `security-backend.md`) requires an
  outbound client that sends a credential header to refuse redirects, follow
  only same-origin ones, or rely on a library shown by its documentation or
  source never to send the header cross-origin. Spec 008, tasks 21 and 26.
  `554d0ee3`
- **Migration `remove-legacy-constitutions-region` (0.60.0) removes the
  unmaintained `ductus:constitutions` comment region (spec 071, tasks 23
  and 24).**
  The templates shipped a `<!-- ductus:constitutions -->` comment region from
  v0.47.0 to v0.59.11 that no run ever maintained. The migration removes it
  from `CLAUDE.md` and `AGENTS.md` when it is the shipped text, and leaves an
  edited, unclosed or pinned one with a report. Its id sorts after
  `memory-routing-leaves-claude-md`, the 0.60.0 entry pushed before it, so
  an adopter who ran `/{project}` between the two pushes still runs it. Spec
  071, tasks 23 and 24. `f056b209`, `47e68233`, `77fff147`, `0f31a124`
- **Migration `substitute-seed-placeholders` (0.60.0) fills the placeholders a
  bootstrap through 0.59.11 left literal in `AGENTS.md` and `CLAUDE.md` (spec
  027, scenario `seed-files-keep-no-literal-placeholder`, task 17).**
  Through 0.59.11 the bootstrap mapped the `skip` rows for both files to
  `apply-manifest`'s `skip-if-conflict` strategy, which substitutes nothing, so
  an adoption on the runtime path wrote them with `{project-name}`,
  `{One-line project description.}` and `{project}` as they stood. Both rows
  map to `create` since spec 071 task 20, which substitutes, but that reaches
  new adoptions only: the files are installed once and never rewritten. The
  migration replaces every occurrence of each of the three tokens, matched
  exactly, with the value this run's substitutions map holds for it, in each
  of the two files that exists, keeping every other byte, the line endings and
  the mode, and writing atomically. Every other brace token (`{name}`,
  `{slug}`, `{NNN-feature-name}`) is left alone. A file that holds none of the
  three is converged and reports nothing; a pinned file is reported with one
  line and not written; a token with no value in the map is left and reported.
  It carries no `sunset_after`, since nothing else would ever fill the tokens
  in an adopter who skips this window, and its id sorts after
  `strip-inbox-guidance`, the 0.60.0 entry pushed before it, so an adopter who
  ran `/{project}` between the two pushes still runs it. Spec 027, task 17.
  `98d9e331`
- **The Pi trust reminder joins the First run and Update mode messages (spec
  064, task 16).**
  When Pi is among the adopted or updated agents, `/{project}` now says that
  Pi loads the project's `.pi/` resources, the bridge included, only after
  the project is trusted: an interactive start prompts, and a non-interactive
  run needs `--approve` or `defaultProjectTrust: "always"`. Spec 064, task
  16. `3b359742`

### Changed

- **BREAKING: `ductus check-step-references` exits non-zero on a finding, a skip or a
  subject it could not establish (spec 026 task 8).** It exited 0 whatever it
  found, and nothing read its exit status: the audit reads the JSON. The result
  is printed either way.
  Breaking for a script that read a zero exit as clean: `ductus check-step-
  references` now exits non-zero on a finding, a skip or a subject it could
  not establish.
- **The citation walk follows a markdown anchor link `[text](#slug)` to the
  heading it names in the document holding the link (spec 026 task 8).**
  `cited_tools` shares it, and still starts from the whole command file, so no
  installed command's tool-search line or always-loaded set changes; a tool a
  section invokes now loads when only a link reaches the section.
- **BREAKING: `dashboard` no longer returns `rendered-markdown`; the pipeline
  view is `render-status`'s (spec 080, task 4).** 0.59.11's `dashboard`
  returned the view as `rendered-markdown` beside its structured payload. It
  now returns the payload alone, and `render-status` (see its entry) returns
  `rendered-markdown` and `project` for the same read. `/{project}:status`
  calls `render-status`; `/{project}:review --all` and `/{project}:target`
  still read `dashboard`.
  Breaking under `BE-COMPAT-001`: a result field removed. A caller that read
  `rendered-markdown` from `dashboard` reads it from `render-status`.
- **BREAKING: `dashboard` is held to `[artifacts] read-size-bytes` (spec 080,
  AC7).** Its `specs` and `tags-union` grow with the corpus and 0.59.11 returned
  them whole. They stay inline while the result fits; past it `form` is `file`,
  both are absent, `specs-count` and `tags-union-count` stay inline, and
  `lists-file` names the file under `.ductus/results/` holding them whole. A
  malformed read size halts the call, as it halts every reader of the setting.
  `/{project}:review --all` reads `specs[].status` from that file when `form` is
  `file`. Breaking under `BE-COMPAT-001`: `specs` and `tags-union`, inline in
  0.59.11, are absent inline past the read size.
- **`/analyze` step 16 reports a `prune-plan` result with an `unexamined-tail`
  as examined in part (spec 041 task 29).** A result carrying `unexamined-tail`
  examined only part of `plan.md`, yet the step said nothing of it, so a plan
  with a section outside the design record after the tail read as having none.
  The step now says to report it so, naming `from-line` and `cause`, never as a
  plan with no section outside the design record. No primitive changes.
- **`prune-project` ends a heading entry before a comment that never closes (spec 041; found reviewing task 28).** A `###` heading entry open when a `<!--` with no `-->` began kept the unexamined lines in its span, so an apply replacing or removing that entry rewrote text the result said was not examined. The entry now ends before the comment and the tail is covered by no entry, as the data model states. Not breaking: a result's entry for such a file ends earlier.
- **A first adoption runs no migration, and names each spec it cannot read (spec
  027 task 16, scenario `a-first-adoption-runs-no-migration`; owner, 2026-11-16).**
  Pre-run Migrations skipped only a greenfield run, so a first adoption into a
  project that keeps its own spec root ran every active entry though no
  earlier run had left anything to convert; a scratch adoption into a project
  with two specs without frontmatter ran all 14 and two migration primitives
  exited 1 on both specs. A first adoption, which Pre-flight Checks decides
  from finding no signal an earlier `ductus` run leaves, now applies nothing,
  asks nothing and seeds `[migrations] last_applied` with the newest active
  entry's id, as a greenfield run does; an update, a legacy adopter with a spec
  root and no config file included, runs its pending entries as before.
  Post-Scaffolding Output names each feature `spec.md` with no frontmatter
  (`no frontmatter: {path} — ...`), which `/analyze` reads as a hard fail.
  `98d9e331`
- **Command steps gate on a bounded list's `-count`, and read its contents from `lists-file` in the form's file case (spec 080).**
  `/fold` steps 1 and 11, `/consolidate` step 4, `/analyze` step 6 and `/clarify`
  steps 2 and 13 read `unfolded-count`, `rewritten-count`, `updated-count`,
  `open-questions-count` and `scenario-open-questions-count`, which both forms
  carry. `/fold` steps 1 and 11, `/consolidate` step 4, `/implement` steps 1
  and 9, `/target`'s `read-tasks` step and step 3, `/review`'s `--all`
  enumeration and step 9, `/analyze` steps 1, 8 and 18 and `/clarify`'s
  reference paragraph deriving the scenario question list say to read the list
  from the file `lists-file` names when `form` is `file`, in the phrase the
  existing steps use.
- **Every `##` section of a command reference is reached by a step of its command, or is gone (spec 071, scenario `reference-sections-are-reached-by-a-step`).**
  Measured at `b7b07500` over the 12 shipped references that have a command, 23 of
  93 sections were named by no command text, no reached section and no anchor
  link; 74 sections remain, 0 unreached and 4 exempt by design (`Severity tiers` and
  `Record fields` in `framework/reference/analyze.md`, `Cross-spec context
  checklist` in `framework/reference/plan.md` and `Blocking semantics` in
  `framework/reference/review.md`, each read by a document outside the
  command's steps). `/plan` step 9 names `Validation gate` and step 10 carries the
  next-step line, `/implement` step 5 names `Walk through tasks`, `/review` step
  11 names `Output` and step 1 names the base-override rules of `Window and
  coverage`, and `/specify` steps 6 and 10 carry the one-question-per-list-item
  rule and the next-step line. The sections that only restated a command or a
  primitive's contract are deleted, and `status`, `consolidate` and `specify`
  references stay as their opening paragraph. The parity goldens `plan-basic`,
  `implement-basic` and `review-basic` are re-blessed (the plan step 9 prompt
  and the fixture commit shas).
- **`review.md`'s `Excluded from scope` section lists each excluded path without the specs that claimed it (spec 079, scenario `the-excluded-section-lists-paths-only`, task 39; owner decision, option b).** Those claimant lists were 88.6% of the section's bytes (the text after each path, 8,386,781 of 9,469,301 bytes, across the 76 `specs/*/review.md` records at `b7b07500`, method: section split on level-2 headings, UTF-8 byte length) and no primitive, command or check reads them. A fix round's left-out path keeps its round. Projected over those 76 records the corpus total falls from 10,982,629 to 2,595,848 bytes and the records over 50,000 bytes from 32 to 13 (`measure.py` projection, not a rewrite: a record written before keeps its claimant lists until its spec's next review). Nothing the primitives report changes: `compute-review-scope`'s `excluded` result already listed paths only, and `excluded` stays a count in the record's frontmatter. The claimant lists, read nowhere else, are removed from `compute_review_scope`'s internal `excluded` map and window claims, and `commit_attribution`'s `name_of` goes with them. `write-review`'s description is 29 bytes shorter (7,961 to 7,932); `compute-review-scope`'s, 36 bytes shorter (4,359 to 4,323), no longer says a credited path is listed "under the spec whose review read it".
- **BREAKING: Six shipped rules change tier under the same ID (specs 070 and 068).**
  `performance-frontend.md`'s `FE-BUNDLE-002`, `FE-IMAGE-001`,
  `FE-IMAGE-002`, `FE-LOAD-001` and `FE-FONT-002` move from MUST to SHOULD,
  so a finding under one is an advisory warning where 0.59.11's blocked
  `done`, and a waiver or `departs:` entry written only for one of them is no
  longer needed. `api-backend.md`'s `BE-PAGE-003` moves from SHOULD to MUST:
  a spec or plan that introduces a list endpoint over a collection accepting
  concurrent inserts or deletes and proposes offset pagination is now a
  blocking `/analyze` finding where 0.59.11 warned, until it commits to
  cursor-based pagination or records a departure. Offset pagination over a
  static or append-only-at-tail collection stays permitted.
  Breaking for a spec or plan whose list endpoint over a collection accepting
  concurrent inserts or deletes proposes offset pagination: `BE-PAGE-003` is
  now a blocking `/analyze` finding where 0.59.11 warned.
- **Each `--{name}-json` flag's `--help` states the JSON its file holds, and no
  subcommand's help prints a developer note (spec 022 task 53; scenario
  `every-mcp-argument-has-a-cli-form`, deferred from 003 task 8).** The help
  of each such flag (`write-review`'s `--observations-json`, `apply-manifest`'s
  `--entries-json` and the rest) named the argument and not its shape, so a host
  on the CLI learned a nested entry's fields one `missing field` error at a time.
  It now ends with one line per field, giving its name, type and whether it is
  required, with each nested type's fields indented below the field that holds
  it, rendered from the input schema the MCP tool publishes
  (`ductus::mcp::flag_help`, `ductus::mcp::server::tool_input_schemas`), so the
  help and the tool read one schema. `write-session`'s `--cli-config-dir` and
  `append-task`'s `--parent-heading` named a Rust path (`crate::host::Host`,
  `PrimitiveError::ParentHeadingNotFound`) in their help; the two sentences now
  say it in words; four more help texts (`merge-permissions`' `--revoke`,
  `retire-feature`'s `--feature`, `derive-references`' `--write` and
  `--staged`) name what a rustdoc link pointed at, and all six texts are also
  MCP argument descriptions. Two new unit tests in `runtime/src/main.rs` fail when a flag's help omits its
  shape or any nested field, and when any subcommand's help holds a Rust path,
  a rustdoc link or a serde attribute; `scripts/mutation/json-flag-help-022.py`
  proves them.
- **BREAKING: `discover-rule-files`' `selected`, `dashboard`'s
  `config.disabled-rule-files` and `label-criteria`'s `assigned` move to the
  list file past `[artifacts] read-size-bytes` (spec 022 task 42;
  `BE-PAYLOAD-001`, from the review of 2026-11-17).** Each list grows with its
  input (a rule directory, a config, a spec's unlabelled criteria) and was
  returned inline in both forms. Each is now absent in the file form, with
  `selected-count`, `disabled-rule-files-count` (under `config`) and
  `assigned-count` inline in both, and `label-criteria`'s result gains `form`
  and `lists-file`. `label-criteria` bounds its result before it writes the
  spec, so a malformed read size refuses the call with the spec untouched. A
  caller that reads `selected` itself takes `discover_rule_files::compute`, as
  `check-departures` now does.

- **BREAKING: 067's primitives contain every path drawn from committed text, and five results are bounded (spec 067's review, `BE-INPUT-004`, `BE-PAYLOAD-001`).** A path a primitive builds from text the repository holds is refused before it is probed, listed or opened, as a path argument is, and reported as the primitive reports an unreadable one, never read:
  - `derive-dependencies` and `derive-references` name a tracked spec that is or runs through a link leaving the project in `unparseable`, and `--write` never replaces it with a regular file holding the outside content. `derive-references` still reads a registered service's `.ductus/config.toml` outside the project, by contract (specs 030 and 040).
  - `validate-frontmatter` reports a `scenarios/`, scenario, `review.md` or `analysis.md` that leaves the project as a hard-fail finding, and a `scenarios/` that exists and cannot be listed as one saying no scenario was checked, where it read as `clean: true`.
  - `traverse-deps` treats a `dependencies:` name that is absolute, holds a `..` segment or runs through a link leaving the project as a missing dependency and a cycle-walk sink.
  - `check-corpus-links` lists no spec root that leaves the project, and probes no linked directory's target type before judging the link; `check-unfolded-specs` halts as an invalid path on a spec root or branch-scoped `spec.md` that leaves it; `check-departures` contains each listed rule file; `check-orphaned-references` skips a referrer, or a managed path one names, that leaves the project; `check-command-flags` skips a command directory or source that does.
  - A spec that links siblings and has no `dependencies:` key is `unwritable` in `derive-dependencies`, not in sync.
  - `validate-frontmatter`'s `findings`, `check-rule-ids`' five lists, `check-orphaned-references`' three, `check-command-flags`' five and `check-promotion-coverage`' three move to the file `lists-file` names past `[artifacts] read-size-bytes`, each list absent and its `-count` inline (`findings-count`, `citations-count`, `missing-count`, `deprecated-count`, `retired-count` and `not-installed-count` absent at zero, `examined-count`, `skipped-count`, `with-flags-table-count`, `with-argument-hint-count`, `unclassified-entries-count`, `unmatched-keys-count`, `missing-sections-count`), and `form` says which. `/analyze` steps 2, 5 and 9 and Families 30 and 38 read the lists from that file, as the bootstrap's Pre-run Migrations step 8 now reads `check-orphaned-references`' and `/analyze` step 3 reads `traverse-deps`', and `ductus exec` restores them from it.
  - `DERIVE_REFERENCES`' description states the report-only run and the CLI's exit on drift.

  Breaking under `BE-COMPAT-001`: a spec, scenario or listed path behind a
  link leaving the project, read in 0.59.11, is now refused, reported unread
  or halts the call.
- **`check-artifacts` reads what its placeholder, plan-reference and completeness checks name (spec 067's review).** A criterion is a placeholder only when it is wholly one bracketed span, so `<b>bold</b> output … the <i>` is a criterion; a branch-scoped spec's plan is matched by its `{identifier}.{n}`; a `+` or `N)` list item lists a file; a `plan.md` or `tasks.md` that is a link out of the project is no missing artifact, and a `scenarios/` link out is never listed. `check-corpus-links`' stderr line for a skipped file says `skipped:`.

- **BREAKING: a `ductus exec` extension request is bounded whole (spec 022 tasks 36
  and 37; spec 060's and spec 020's analyses, `BE-PAYLOAD-001`).**
  `writeCode`, `assessSpecQuality` (one request per loaded rule of the step's
  tier; the count grows with the rule files), `askClarifyQuestion`, `routeFold`, `verifyCriteria` and
  `performReview` are inline while the request fits `[artifacts]
  read-size-bytes`, byte for byte as before. Past it the request carries no
  body: `spec-content`, `routeFold`'s `target-content`, and each
  `performReview` `scope-files` and `rule-files` `content` are absent, the
  request names the file the host reads (`spec-path`, `target-path`, each
  entry's `path`, which `rule-files` entries gain only in this form), and a
  new `content-form: "path"` says so. A body the runtime could not read is
  never bounded, so a missing spec is still `""` and never a path. A
  malformed read size halts the walk at the request. A request still past
  the read size once its bodies are named moves the lists that grow with
  entries (`criteria`, `scenarios`, `target-sections`, `loaded-rule-files`,
  `scope-files`, `rule-files`, `passes-run`, `plan-relevant-files`,
  `constitution-excerpts`, `write-boundary`) and its whole legacy context tail
  into one file under `.ductus/results/exec-request/{extension point}/`, named
  by `request-lists-file`, leaving `{list}-count` and `legacy-context-count`
  inline; a file that cannot be written halts the walk with
  `request-lists-unwritable`. Spec 022's data model states the shapes,
  including a new `performReview` section.
  Breaking under `BE-COMPAT-001`: an extension request over the read size,
  carried whole in 0.59.11, now carries no body and moves its lists to
  `request-lists-file`.

- **BREAKING: `writeSpecBody` and `routeInboxItem` are bounded whole, and `payload.rs` contains its reads (spec 022 task 41; 022's review of 2026-11-17, `BE-PAYLOAD-001`, `BE-INPUT-004`).**
  Both were the two extension requests the bound missed. `writeSpecBody` past
  `[artifacts] read-size-bytes` drops `template-content` and `existing-content`,
  names the file the host reads in `template-path` and a new `existing-path`,
  sets `content-form: path` and moves its legacy tail to the file
  `request-lists-file` names; `routeInboxItem` moves `available-specs` and
  `candidates` there, each with its `-count` inline. A request under the size is
  byte for byte as before. `writeCode` gains `task-unexaminable`, beside `task`
  and omitted when the task was found, naming why the task was not read
  (`no-feature`, `tasks-unreadable`, `task-not-found`). The constitution, a
  feature's `plan.md`, a spec root, a spec's `spec.md` and a fold's `scenarios/`
  are read only when they stay in the project: a constitution that is a link
  out is the unexaminable cause `constitution-outside-project`, and the others
  halt the walk with the new `path-outside-project` code (a `spec.md` or
  scenario file link out reads as the empty status or `section` an unreadable
  one has). The rule load propagates a project config it cannot parse and a
  feature that leaves the project, so the first assessment step of an `/analyze`
  walk halts instead of judging every rule against a feature with no record.
  `performReview` gains `unexaminable`, after `passes-run` and omitted when
  empty, naming each scope path (`scope-outside-project: {path}`) and rule file
  (`rule-file-outside-project: {path}`) that leaves the project and so was not
  read, where the request used to drop them silently; past the read size it
  moves to the lists file with `unexaminable-count` inline. A scope path missing
  from disk is still skipped, as before.
  Breaking under `BE-COMPAT-001`: a spec root, `spec.md` or scenario behind a
  link leaving the project, read in 0.59.11, now halts the walk with `path-
  outside-project`, and a config that does not parse halts the first
  assessment step.

- **BREAKING: Claude's canonical allow set retires four entries no framework step runs
  (spec 022 task 43; 022's review of 2026-11-17, owner decision).**
  `Bash(git push *)`, `Bash(gh api *)`, `Bash(git commit *)` and
  `Bash(git config --unset core.hooksPath)` leave `CANONICAL_ALLOW_STATIC`,
  which now holds 12, and join `RETIRED_ALLOW`, which now holds 31, so
  `/configure` revokes them from an existing settings file, and a canonical
  `merge-permissions` call passing one in `allow` is refused as a conflicting
  revoke.
  Breaking under `BE-COMPAT-001`: a canonical `merge-permissions` call passing
  one of the four in `allow`, accepted in 0.59.11, is now refused as a
  conflicting revoke.

- **BREAKING: a `prune-tasks` or `prune-plan` result carries `form` and `lists-file`
  once, and a file-form walk deserializes back (spec 041's review).** The two
  keys moved from the one-feature summary and the walk to the result, so the
  halves share no key and serde's flatten no longer consumes them for the
  half that fails; the bound is judged on the result as it is sent, its own
  `form` included. Key order is unchanged. `prune-plan`'s walk entries no
  longer carry `form: inline`, which task 26 added. `PRUNE_PLAN` and
  `PRUNE_TASKS` state that a malformed `[artifacts] read-size-bytes` halts
  the call before any write; the parity suite's recorded descriptions
  follow.
  Breaking under `BE-COMPAT-001`: `form` and `lists-file` moved from the one-
  feature summary and the walk to the result, and `prune-plan`'s walk entries
  no longer carry `form: inline`.

- **BREAKING: `prune-plan`'s single-feature `sections` is bounded (spec 041's
  analysis, `BE-PAYLOAD-001`).** It is inline while the result fits
  `[artifacts] read-size-bytes`; past it `form` is `file`, `sections` is
  absent, `sections-count` stays inline in both forms, and `lists-file` names
  the file under `.ductus/results/prune-plan/{feature}/` holding it whole, as
  `prune-tasks` already did for one feature. A malformed read size now halts
  an applying call before its removal. A walk's entries carry `sections-count`
  and `form: inline`. `/{project}:prune` reads `sections` from `lists-file`.
  Breaking under `BE-COMPAT-001`: `sections`, inline in 0.59.11, is absent
  inline past the read size, and a malformed read size now halts an applying
  call.

- **BREAKING: `resolve-anchor`'s and `read-deprecation-labels`' results are bounded
  (spec 067 task 35, `BE-PAYLOAD-001`).** `resolve-anchor`'s `references` and
  `unresolved`, and `read-deprecation-labels`' `labels` and `unreadable`, grow
  with the references or labels in the files they read. They stay inline
  while the result fits `[artifacts] read-size-bytes`; past it `form` is
  `file`, the lists are absent and each has its `-count` inline
  (`references-count`, `unresolved-count`, `labels-count`,
  `unreadable-count`), and `lists-file` names the file under
  `.ductus/results/` holding them whole. Families 39 and 40, `/analyze` step 4
  and the `ductus exec` walker read the lists from that file, and a file
  `/audit` cannot read, or one lacking a list, is a `(precondition)` finding.
  Breaking under `BE-COMPAT-001`: `resolve-anchor`'s `references` and
  `unresolved`, inline in 0.59.11, are absent inline past the read size.

- **BREAKING: `check-corpus-links` refuses a link out of the repository (spec 067 task
  33, `BE-INPUT-004`).** Links are contained to the repository's work tree
  when the project sits in a git repository (a project in a subdirectory links
  up into its repository), else to the project root. A target that leaves it
  lexically, or whose path runs through a symbolic link resolving outside it,
  is reported in `broken`, never probed. A tracked markdown file, or a
  spec-root directory, that is or lies under such a link is never read or
  listed and is recorded in `skipped` as `outward-link`. A target inside the
  base resolves as before.
  Breaking under `BE-COMPAT-001`: a link out of the repository, probed in
  0.59.11, is now `broken` and never read.

- **BREAKING: `check-artifacts`' path probes refuse a link out of the project (spec 067
  task 33, `BE-INPUT-004`).** `criterion-path-existence` and
  `body-path-existence` no longer stat a path that runs through a symbolic
  link resolving outside the project, as the sibling-link family never
  follows one; such a candidate is recorded `outside-project`.
  Breaking under `BE-COMPAT-001`: a candidate behind a link leaving the
  project, statted in 0.59.11, is now an `outside-project` skip.

- **BREAKING: `check-step-references`' result is bounded (spec 026 task 6,
  `BE-PAYLOAD-001`).** Its lists, `findings`, `examined`, `with-steps`,
  `not-a-procedure` and `skipped`, grow with the command files it reads. They
  stay inline while the result fits `[artifacts] read-size-bytes`; past it
  `form` is `file`, the lists are absent and each has its `-count` inline
  (`findings-count`, `examined-count`, `with-steps-count`,
  `not-a-procedure-count`, `skipped-count`), and `lists-file` names the file
  under `.ductus/results/` holding them whole. `references-out-of-subject` and
  `guidance` stay inline, and an empty list is `[]` inline where it was
  absent, as `check-corpus-links`' is. Family 34 reads the lists from that file, and a file
  it cannot read, or one lacking a list, is a `(precondition)` finding.
  Breaking under `BE-COMPAT-001`: `findings`, `examined`, `with-steps`, `not-
  a-procedure` and `skipped`, inline in 0.59.11, are absent inline past the
  read size, and an empty list is `[]` inline where it was absent.

- **BREAKING: `prune-project` bounds its result and names an unexamined tail (spec
  075's review).** `entries`, each with its `hits`, is inline while the result
  fits `[artifacts] read-size-bytes`; past it `form` is `file`, `entries` is
  absent, `entries-count` stays inline, and `lists-file` names the file under
  `.ductus/results/prune-project/project-files/` holding them whole
  (BE-PAYLOAD-001). A project file that ends inside an open HTML comment, or
  a `<!-- BEGIN … -->` with no END, carries `unexamined-tail`
  (`from-line`, `cause`), so the lines from there on read as not examined
  rather than walked (QUAL-CLAIM-001). `/{project}:prune --project`'s preview
  reads both.
  `prune-project` is new in this release, so no 0.59.11 caller reads
  `entries`; a caller on an earlier build of this release finds it absent
  inline past the read size.

- **BREAKING: `discover-rule-files`' notices and `dashboard`'s unknown config keys are
  bounded (spec 022, task 32).** Each is a line per key in `.ductus/config.toml`
  that this version does not read, so it grows with the config. Every
  `discover-rule-files` call is now held to `[artifacts] read-size-bytes`,
  with `rules` or without: `form` is always present, `notices-count` stays
  inline, and past the bound `notices` moves to the lists file with the rules,
  where review and analyze emit the notice lines from. A malformed read size
  now halts a call without `rules` too. A non-empty `unreadable`, the rule
  files a call with `rules` could not read, moves there as well, with
  `unreadable-count` inline (from 080's analysis). `dashboard`'s `config` gains
  `unknown-keys-count`, and past the bound `unknown-keys` moves to the lists
  file under `config`. `/{project}:status` renders from the model and is
  unaffected.
  Breaking under `BE-COMPAT-001`: `notices` and a non-empty `unreadable`,
  inline in 0.59.11, are absent inline past the read size, and a malformed
  read size now halts a call without `rules`.

- **BREAKING: `derive-dependencies` and `derive-references` name a rewrite they refuse, and exit non-zero on it (spec 067, tasks 14 and 25).** A rewrite whose result would not parse as YAML is never written. Each primitive lists such a spec in the new `unwritable`, absent when empty, with `unwritable-count` beside it, absent at zero, whether or not `staged` holds the spec. Both CLIs exit non-zero on a non-empty `unwritable`, report-only and with `--write` alike, and name each spec on stderr, so the shipped pre-commit hook stops the commit, an unstaged spec included, where before both exited 0 over a stale index. `derive-dependencies` moves `unwritable` to the file `lists-file` names past the read size, its count inline, and the exec `/analyze` tally counts a non-empty `unwritable` and a non-empty `unparseable` as one advisory finding each.
  Breaking for a script or hook that read exit 0 over a stale index: both CLIs
  now exit non-zero on an unwritable spec.

- **BREAKING: `validate-frontmatter` chooses its schema row by the path it is given (spec 067 task 18).** A `.md` file under a `scenarios/` directory takes the scenario row (`section`, or the legacy `spec-ref`, a non-empty string; no `status` or `dependencies`), a `spec.md` the spec row and its run also checks every scenario beside it, each finding keyed by its `scenarios/{file}` path and an unreadable or fenceless scenario a hard fail, and a `review.md` or `analysis.md` that record's checks. Any other path (a `plan.md`, a `tasks.md`) is now refused with an error naming the four kinds, where it was checked against the spec row, and a `spec.md` whose frontmatter is not valid YAML no longer ends the run, so its records and scenarios are still checked.
  Breaking under `BE-COMPAT-001`: a call naming a `plan.md`, a `tasks.md` or any other path, accepted in 0.59.11 against the spec row, is now refused.

- **`plan-consistency` names a `plan.md` it cannot read (spec 067 task 31).** A plan the family could not read was passed over silently; it is now an `artifact-unreadable` skip below `done` and a blocking finding at `done`, as every family records an artifact it could not read.

- **BREAKING: `derive-references` bounds its result (spec 067, task 32).** Its six
  spec lists grow with the corpus, since a service rename drifts every
  referencing spec at once, yet they came back inline whatever their length.
  Each now carries its `-count` in both forms, with `unwritable-count` absent
  at zero as on `derive-dependencies`; past `[artifacts] read-size-bytes`
  the lists move whole to the file `lists-file` names, under
  `.ductus/results/derive-references/all-specs/`, and `form` says which. The
  CLI still exits non-zero on an unwritable spec and still names it on
  stderr, gating and reporting on the whole derivation. A malformed read
  size halts a writing call before it rewrites any spec.
  Breaking under `BE-COMPAT-001`: the six spec lists, inline in 0.59.11, are
  absent inline past the read size, and a malformed read size now halts a
  writing call.

- **Rule judging runs in a fresh subagent where the host can start one
  (specs 020 and 060).** Review's five passes and analyze's steps 11 and 12
  read every loaded rule file, and review every in-scope file, into the
  host's own context, where every later call of the session read them again.
  A host that can start a subagent (Claude Code's Agent tool) now runs them
  in one fresh subagent, briefed as `.ductus/reference/review.md` §Passes in
  a subagent and `.ductus/reference/analyze.md` §Rule judging in a subagent
  state, and continues with only the findings, verdicts and coverage it
  returns, checked before use. A host that cannot (Pi) runs them itself, and
  `ductus exec` still asks per pass and per rule.

- **BREAKING: `process-decisions` keeps retained decisions out of its inline result
  (spec 066, task 22).** A retained decision asks nothing of the host, yet
  each came back inline with a notice of its own, and in one session they
  were 131K of the 136K characters the primitive returned, which every later
  call re-read. Whenever a decision is retained, the `retained` list and each
  one's own notice (`retained-notices`) are in the file `lists-file` names,
  in either form, `retained-count` stays inline, and `notices` carries one
  line per reason instead, a scope that could not be read on its own line.
  Since task 14, past `[artifacts] read-size-bytes` no list is inline, and
  `lists-file` names the file holding them all. Breaking under
  `BE-COMPAT-001`: `matched`, `expired`, `retained` and `notices`, inline in
  0.59.11, are absent inline past the read size, and `retained` whenever a
  decision is retained.

- **BREAKING: `process-waivers` bounds its result's lists (spec 073, task 14;
  `BE-PAYLOAD-001`).** Its `applied`, `expired`, `retained` and `notices` grow
  with the departure waivers, so they are bounded as `process-decisions`'
  lists are: past `[artifacts] read-size-bytes` `form` is `file`, the four
  lists move whole to the file `lists-file` names, and each list's count
  (`applied-count`, `expired-count`, `retained-count`, `notices-count`) stays
  inline in both forms. A result with all four empty reads no setting. Under
  `ductus exec` the walker restores the lists before `write-review` reads
  `applied` and `expired`. Breaking under `BE-COMPAT-001`: the four lists,
  inline in 0.59.11, are absent inline past the read size.

- **BREAKING: `write-review`'s departure waivers are bounded by the review scope,
  follow their entry, and yield no deferral (spec 073, review round 1).** An
  entry waives its rule only at files the run's resolved scope holds (a scope
  entry, or a file the plan names), so a finding outside it stays counted, and
  a run with an empty `diff-base` derives none. Every write refreshes a stored
  departure waiver's `reason`, `waived-by` and `waived-at` from its entry, the
  stored `waived-at` kept while the entry is still `unattributed`. The entry
  decides over a deferral of its violations: a new `--defer` of one is refused
  naming the rule and the entry's `spec.md` line, and a stored one is
  superseded, pruned from `decisions:` and named in the result's new
  `superseded-deferrals` (bounded as `malformed-departures` is). A parent commit or rename diff that cannot be read
  makes the attribution `unattributed`, where it was the newest commit that
  held the entry.
  Breaking for a caller that defers a violation of a rule a spec's `departs:`
  entry names: the call is now refused.

- **`check-review-gate`'s and `check-artifacts`' descriptions restore what
  triggers a block and two family rules (spec 079, task 36).** The gate says
  what makes each of `not-reviewed`, `must-violations`, `not-analyzed` and
  `analyze-findings` block, what the scenario-question and `unjudged-rules`
  messages carry, and that an analysis with no `unjudged-rules` field blocks on
  nothing there; `check-artifacts` says scenario consistency runs at every
  status and plan consistency needs a Technical Decisions section that holds
  something outside comments. Descriptions only: no result changes.
- **`check-review-gate`'s description states each check's rule (spec 079, task
  36).** Only the exact status `done` blocks as `already-done`; the lint is
  recursive and a lint that linted nothing or exited unattributed blocks; an
  unchecked box counts outside fences and comments; `not-re-read` is compared
  with the round-waivers that carry a reason; a change confined to the project's
  artifacts owes no review; the gate never blocks on `reviewed-digest` or an
  advisory analyze finding; and an unparseable record blocks as
  `record-unreadable` or `analyze-record-unreadable`. Description only: no
  result changes.
- **`append-task`'s and `prune-tasks`' descriptions state the `next-task:`
  counter's rules (spec 079, task 36).** How the highest number is read (both
  task levels, by digits), the counter `append-task` advances to, that a
  reduction removing nothing writes none, where the block is created, that an
  existing block gains or updates only its own line, and that an unclosed or
  non-integer counter is an error that writes nothing. Description only: no
  result changes.
- **`check-orphaned-references`' description states what counts as a reference
  (spec 079, task 36).** The token-start rule and the `@` import form, which
  prefixes name the project root in which referrer and which do not, and the
  patterns, parent-directory paths, git object names and shipped paths that are
  never reported. Description only: no result changes.
- **`resolve-anchor`'s description states how a reference is classified and
  named (spec 079, task 36).** The kinds are decided in order, first match
  winning; a line naming one shipped document resolves against that source's
  and the citing file's headings or a marker; `## Shared Files` of
  `framework/bootstrap/ductus.md` decides what is shipped, and where it is
  absent nothing is judged cross-document; how a document and a heading are
  named; and that prose naming a document without its file stays reported, so
  an empty `unresolved` with both counts at zero means every judged `§`
  reference resolved.
  Description only: no result changes.
- **`check-artifacts`' description states its families' behavior rules (spec
  079, task 36).** `data-model.md` is never required; a spec with no `tasks.md`
  has no scenario mapping to evaluate, and an unreadable `tasks.md` history is
  recorded `no-readable-state`; a `done` spec with no `review.md` is exempt
  from the record checks but not from the unticked-criterion and unchecked-box
  ones; the body path family's `never-existed` and `not-a-live-claim` skips;
  how a plan references its spec and lists a file; and that the label,
  integrity and plan families run at every status and add nothing to
  `skipped` but a `plan.md` that exists and could not be read. Description
  only: no result changes.
- **`read-re-read` says a demoted finding stays in `findings` as a NOTE, and
  `write-review` says a fix round's `excluded` is `brief-re-read`'s (spec 079,
  task 36).** Descriptions only; no result changes.
- **Built with Rust 1.100.0**, up from 1.99.1. The pinned toolchain and every
  workflow's `dtolnay/rust-toolchain` input moved together; MSRV stays 1.89.
- **Compiled descriptions state what the specs that point at them used to carry
  (spec 079, task 36).** `write-session`'s peer `last-used` is the record's
  used-at verbatim; `process-waivers` counts a file a plan entry in the scope
  names as read, and, under an unreadable scope, retains a non-firing waiver on a
  deleted file; `rewrite-spec-links`
  lists no scenario tail in `kept-tails` once the retiring directory is gone;
  `resolve-anchor` ranks `anchor-target-unreadable` before
  `anchor-target-unjudgeable`, for qualified references; `brief-re-read`,
  `read-re-read` and `check-prescriptions` name their list file's directory and
  `empty` returning no snapshot, `unprescribed`, the `failed` entry's fields and
  how a text-less file and a line-ending conversion compare; `write-review` says
  `ductus exec` never passes `loop-open`, that `by-rule` records `decided-by:
  rule` and that an exec review pass carries `contract-break`;
  `derive-dependencies` and `derive-references` say what `unwritten` and an
  empty `updated` mean, `derive-dependencies` how `cycles` sort;
  `check-orphaned-references` how `registry` attribution resolves and what
  `watermark` covers; `check-command-flags` when `guidance` is set. The
  `check_artifact_size` `run` doc comment no longer calls an unreadable index
  an error. Descriptions only: no result changes.
- **BREAKING: `write-analysis`'s `contradictions` is bounded to the read size, and a
  caller's bare `unexamined` total is never lost (spec 082).** The result gains
  `form`, `contradictions-count` (inline in both forms) and `lists-file`: past
  `[artifacts] read-size-bytes` `contradictions` is absent and the file holds
  it, and an empty list stays absent inline. A read size that does not parse
  now fails the call, after the record is written. An `unexamined` total with
  no `unexamined-by-reason` is recorded under the new reason `unclassified`
  before the derived reasons join it, so `unexamined` is always the sum of the
  breakdown (a bare 5 with two unjudged rules records 7), and the body no
  longer renders a total with no breakdown.
  Breaking for a call under a malformed `[artifacts] read-size-bytes`, which
  `write-analysis` ignored in 0.59.11: it now fails, after the record is
  written.
- **`ductus exec` counts a single-tier rule's finding in the rule's own tier
  (spec 082).** The walker honored the host's `finding.severity` over the
  asked tier, so a host labelling a MUST-only rule's failure `should` (or a
  SHOULD-only rule's `must`) was refused by `write-analysis` and the walk
  aborted with no `analysis.md`. The host's label is now honored only when the
  rule's Statement carries both MUST and SHOULD; otherwise the finding counts
  and is stored in the rule's tier, so a label cannot move a MUST failure to
  advisory.
- **A stored verdict is current only while the loaded rule-file set is the one
  it was judged under (spec 082).** When a rule file is added or removed since
  the prior record's `rule-digest`, no verdict is reusable, `judge` holds every
  assessable rule and `discover-rule-files`' `verdicts.reason` is the new value
  `rule-files-changed`; a file whose content alone changed keeps the per-file
  rule. The gate's `unjudged-rules` block now says the record's
  `unjudged-rules` holds every name, the message showing three.

- **The label-assignment rule is stated as the runtime applies it, and
  `prune-tasks`' description no longer says a reset leaves no counter.**
  `framework/reference/clarify.md`, `specify.md` and the criterion-label backfill
  migration gave the next label as `max(highest label in body, next-criterion)`,
  which would reissue the highest label when the counter is missing or behind
  the body; `label-criteria` takes `max(highest label in body + 1,
  next-criterion)`. A reset that removes a task section writes the `next-task:`
  counter as any prune does. Text only.

- **Primitive descriptions gain the detail specs used to restate, and two stale
  ones are corrected (spec 079 task 36).** `relocate-audit-records`' description
  said it never overwrites an existing artifact; it merges the block into one,
  the spec block winning, and reports each disagreeing key.
  `create-plan-artifacts`' said a kept artifact reports its `template`; a kept
  one carries none. `resolve-constitutions` names its `examined` count.
  `write-session` states `clear`, the peer and result shapes and which
  unreadable files its sweep leaves in place, `resolve-session` the null and
  notice rules, `retarget-sessions` its arguments, `dashboard` each spec's
  scenario question and fold fields, `append-task` what its dedup keys on,
  `write-review` that a run's own earlier write never refuses its empty scope,
  `rewrite-spec-links` what `examined` leaves out, `start-fix-rounds` what
  `files` counts, `brief-re-read` the brief's contents and the inline counts,
  `check-prescriptions` the missing-`round.toml` case, `check-artifact-size` what
  `--staged` names on stderr, `read-deprecation-labels` its section-run rule and
  `classify-spec-edit` the absent `spec.md`. Text only.

- **`measure-reopenings` counts a decision as a person's only in the record that
  first holds its key.** A decision a review carried from another spec's record,
  with the same key and outcome, stood in that record at the parents of the run's
  first write and was counted again as the carrying run's, overstating
  `decisions-by-person` (22 of the 30 person-stamped decisions in five measured
  runs were carried). The compiled description says so (spec 079, AC17).
- **`/{project}:analyze` step 15 flags a decision rejected for the reopen or
  review run it would start.** A Resolved Question or plan Trade-off whose
  stated reason is that process cost is an advisory finding to re-examine,
  never gating. An exec walk records the host judgment unexamined under the
  new reason `decision-reasons-not-checked`, in the *could not be read*
  class, once per run like step 15's other two (spec 067).
- **BREAKING: `check-artifacts`' two path families examine a spec at
  `in-progress`, and say so when they do not.** `criterion-path-existence` and
  `body-path-existence` run at `in-progress` as at `done`, so the analysis the
  pre-`done` gate reads has run them; at any other status each records one
  `status-gated` skip naming the spec's `spec.md`. A criterion path the
  repository's ignore rules ignore is recorded `gitignored`, no longer a
  finding. The closed reason set gains `status-gated` and `gitignored`, both
  excluded by construction (spec 067 task 17).
- **BREAKING: a path citation that leaves the project is skipped as `outside-project`,
  no longer `root-absent`.** The two path families resolve a candidate
  lexically and record one that leaves the project without probing it, as
  before; the reason is new and sits with the targets that could not be read,
  where `root-absent` says an excluded-by-construction top-level segment is
  missing here (spec 067, spec 045).
  Breaking under `BE-COMPAT-001`: a skip reason `root-absent`, recorded in
  0.59.11 for a path leaving the project, is now `outside-project`.
- **`check-artifacts` opens nothing through a link that leaves the project.**
  It walks the feature directory as every other feature-argument primitive
  does, but records each link in it that resolves outside the project as a
  `target-unparseable` skip (family `outward-link`) and keeps running, instead
  of reading the file behind it. A linked `spec.md` ends the run with the skip
  alone and the status `unreadable` (spec 067, spec 022).
- **BREAKING: `write-review` and `check-review-gate` bound a stale analysis's path list
  (066, task 20).** `write-review`'s `analyze-freshness.paths` and `check-review-gate`'s
  new `analyze-stale-paths` (the artifacts the message shows three of) are
  inline while the result fits `[artifacts] read-size-bytes`, and past it move
  to the file `lists-file` names, under the key `analyze-stale-paths`. Each
  result gains `analyze-stale-paths-count`, in both forms, and `write-review`
  gains `form` and `lists-file`; a `stale` freshness whose list moved carries
  no `paths` key, so it never reads as a short complete list
  (`BE-PAYLOAD-001`). Breaking under `BE-COMPAT-001` for `write-review` past
  the read size, where a stale freshness no longer carries the `paths` it
  carried in 0.59.11; `check-review-gate`'s list is new.
- **BREAKING: every destination-judging writer refuses the runtime pointer's directory
  (spec 022).** `apply-manifest`, `enforce-manifest`, `merge-managed-block`,
  `append-task`, `create-scenario` and `merge-permissions` refuse a path under
  `.ductus/bin`, at any depth, as they refuse Claude Code's protected
  directories: the pointer there runs at every session start and on Claude's
  status and help hooks, and `apply-manifest` and `merge-managed-block` could
  replace it. No primitive wrote it; `/ductus` places it with the shell.
  Breaking under `BE-COMPAT-001`: a call writing under `.ductus/bin`, accepted
  in 0.59.11, is now refused.
- **BREAKING: `/configure` no longer grants thirteen Claude allow entries that can run or
  write anything, and retires them from existing settings (spec 022).**
  `curl *`, `make *`, `./.githooks/pre-commit`, `scripts/install-hooks.sh`,
  `./scripts/install-hooks.sh`, `git diff *`, `git log *`, `git show *`,
  `markdownlint *`, `markdownlint-cli2 *`, `mkdir -p *`, `chmod +x *` and
  `git config core.hooksPath *` leave `merge-permissions`' canonical set
  (`CANONICAL_ALLOW_STATIC`, 29 entries to 16) for its retired list, so the
  canonical call revokes each and the ceiling refuses each as an `allow`
  entry. `/ductus`'s Claude seed no longer grants `Bash(curl *)` or
  `Bash(tar *)` (`curl -o` writes anywhere, `tar --to-command` runs
  anything), and its retired list gains both and `Bash(git diff *)`, so a
  first run in Manual mode prompts for its downloads and extraction. A
  command an adopter ran through one of them now prompts.
  Breaking under `BE-COMPAT-001`: a `merge-permissions` call passing one of
  the thirteen in `allow`, accepted in 0.59.11, is now refused.
- **BREAKING: `apply-manifest` refuses an entry that would be written with a
  known placeholder unsubstituted (spec 022).** With a non-empty
  substitutions map, an `update` or `create` entry whose substituted text
  still carries `{project}`, `{cli-config-dir}`, `{project-name}` or
  `{One-line project description.}`, where the map has no such key and the
  entry does not keep it literal, halts the call before any write, an earlier
  entry's included, with an error naming the entry and the placeholder. It
  narrows what the primitive accepts, so it is a breaking change under
  `BE-COMPAT-001`, made within major version zero: a call that wrote the
  placeholder literally while reporting a clean run now fails. An empty map
  (the `ductus` self-install) is never checked; other brace tokens are not
  refused. The set is compiled in and a test pins it to
  `framework/bootstrap/ductus.md` §Placeholder Substitution.
- **`check-artifacts`' two path families never probe outside the project.**
  A criterion or body-line candidate is resolved lexically before any stat;
  one whose `..` leaves the project is recorded as an `outside-project` skip,
  as is one whose path runs through a link that resolves outside it (the body
  family used to drop a leading-`..` candidate silently and probe an interior
  one).

- **BREAKING: `append-task`'s `task-number` is a string, and a suffixed task
  number is a task.** The result field was an integer and is now a string,
  since a dedup guard may match a task a person hand-inserted as `11a`, and
  reporting `11` would name another task. That is a breaking change to the
  wire under `BE-COMPAT-001`, made within major version zero; a caller that
  read the field as an integer reads it as a string, which `mark-task`'s
  `task-number` already is. A task number is now decimal digits and at most
  one lowercase letter before the dot (`11`, `11a`; `11ab.` and `11A.` are
  not), read by one grammar, `split_numbered_heading`, and ordered by digits,
  then suffix (`11` before `11a` before `12`). `read-tasks` lists a suffixed
  task, `mark-task` addresses it as `11a`, `prune-tasks` removes a spent one
  with its phase instead of stranding the `###` heading under the title
  (MD001) and keeps a pending one, `check-artifacts` compares numbering in
  that order rather than skipping a suffixed number, and `append-task` numbers
  past the highest digits and never writes a suffix.

- **BREAKING: a review is owed for code, and an analysis for the project's artifacts.**
  `compute-review-scope` owes a review only for a path outside the spec root,
  `.ductus/`, `AGENTS.md` and `CLAUDE.md`, so an edit to a spec's scenarios
  or data model, a rule file or the inbox owes an analysis and no review, and
  `check-review-gate` no longer blocks with `review is stale`. A changed
  contract still joins the scope of a review that runs. A commit whose
  subject names a spec has its code reviewed by the specs it names; a spec
  that claims it only by touching that spec's directory reviews that
  directory's files. A commit naming no spec is attributed as before (spec
  081).
- **BREAKING: a review or an analysis runs once implementation is complete.** `write-review` and
  `write-analysis` refuse while the spec's `tasks.md` holds an unchecked box under any
  heading, and `/review` and `/analyze` stop at their first step naming the boxes, since
  nothing can say whether a task still open would change what a run read. A task list
  schedules no run: what a spec owes is prose on its pending task, and the tasks template
  and `/plan` say so. A box under a task the run's own deferral names does not refuse, since
  it is the work the record holds. A spec with no `tasks.md` is unaffected. Spec 079.
  Breaking under `BE-COMPAT-001`: a `write-review` or `write-analysis` call on
  a spec whose `tasks.md` holds an unchecked box, accepted in 0.59.11, now
  fails.
- **A command's reference holds what a step does with a result, and no more.**
  `framework/commands/` and `framework/reference/` together total 466,360
  bytes at `ffb3db9b` (`cat framework/commands/*.md framework/reference/*.md | wc -c`:
  254,809 over the 17 command files and 211,551 over the 13 references), where the
  17 command files of `ductus-v0.59.11` totalled 486,072 and no reference existed
  (the same `wc -c` over `git show <rev>:<path>`). The cuts reached 421,717 at
  `101a6e8e`, mid-release, from 553,352 before them, and later changes in the
  release grew the files again. `framework/reference/review.md` is 55,547 bytes
  at `ffb3db9b` and `framework/reference/analyze.md` 63,997, so each is over
  one 50,000-byte read; they were 45,232 and 48,691 at `101a6e8e`, from 67,168
  and 78,632 before the cuts. Each primitive's rules are in `ductus <name> --help`. `write-review`,
  `write-analysis` and `process-decisions` now state when a deferral releases a
  finding and when its work is complete, which their descriptions used to cite
  from the review reference (spec 079).
- **BREAKING: a finding a review leaves open stays in reach, and a broken contract left open
  holds `done`.** `review.md` gains `open-findings`, listing each counted violation's
  file and each broken contract's that the run did not fix, a person did not waive
  and no deferral releases. The next review's scope holds each wherever its window
  starts, so a re-run cannot clear a finding with its file unread, and
  `check-review-gate` blocks on a listed break, as `review-owed`, naming it. A break a
  person waived passes, and `write-review --empty-scope` refuses over an earlier
  record counting a finding (spec 079).
  Breaking under `BE-COMPAT-001`: `check-review-gate` now blocks a spec whose
  record lists a broken contract left open, as `review-owed`, and `write-
  review --empty-scope` refuses over a record counting a finding.
- **Each primitive's description says what it does without the prose.**
  `check-artifacts` states its severity tiers rather than pointing at the analyze
  reference, `resolve-constitutions` states that a malformed entry halts, and
  `brief-re-read` names the three lists that move to the file. `derive-boundary`,
  `mark-task` and `check-stuck` state their rules in `--help`, `dashboard` named
  `rendered-markdown` (spec 080 later moved that field to `render-status`, which
  the entry on `dashboard`'s result records), and
  `framework/reference/status.md` no longer restates that view line by line.
  Commands and references no longer describe a hand-walk (spec 079).
- **A report's body says what its record holds.** A person's waiver agreeing with the
  rule's makes no new decision, so `write-review` now renders the rule's reason and
  label rather than the person's words beside a record still naming the rule. Deferring
  the observation or fixing it is how a person changes the rule's decision (spec 079).
- **A signpost on a `done` spec is a correction.** `check-review-gate`'s
  cross-spec guidance says it leaves the spec `done`, and that a criterion or
  scenario there reopens it (spec 079).
- **The constitution makes the binary the source of a primitive's rules, and the MCP
  server required.** Without the server, a command stops and says to restart or re-run
  `/ductus`, since the CLI lacks arguments its steps need, and a command's reference
  keeps what a step does with the result, host judgment and policy. A correction to a `done` spec's `spec.md` or
  `plan.md` leaves it `done`, and a review or analysis run creates work only for a
  gating finding. `create-scenario`, `append-task` and `set-status` state in their
  descriptions what the constitution used to. Spec 079, task 9.
- **Each primitive has one description.** Its MCP description and its `--help`
  are now one compiled text, so `ductus <name> --help` prints everything the tool
  description says, and `ductus --help` and `-h` print its first sentence. The MCP
  text is unchanged but for `gate-confirm`, which now says it blocks on the CLI and
  not over MCP, and `label-criteria` and `check-artifacts`, which write `AC<n>` for
  `AC{n}`. Spec 079, task 8.
- **What a fix for an oversized artifact does to a `done` spec follows that
  rule.** `check-artifact-size`'s `on-done` for trimming `spec.md` is now
  `if-requirement-changes`, replacing `if-claim-changes`, and trimming
  `data-model.md` is `always`. Spec 079, task 6.

- **A review or analysis run asks a person only about a gating finding's waiver
  or deferral.** A finding is fixed in the run when mechanical and otherwise
  waived with its reason, by a rule, and the host marks what the rule decided
  `by-rule`, which `write-review` and `write-analysis` record as `decided-by:
  rule`, needing no `decided-by`, and refuse on anything but a fix or a waiver. A
  person's stored decision wins, and only a person's waiver keeps a plan section
  from `/prune`. An observation showing the spec's own code breaks
  its contract is marked `contract-break`: the rule never waives it, it renders
  with that mark, and unless fixed it counts in `review.md`'s new `contract-breaks`.
  A security or leak observation now renders first. `ductus exec` still writes no
  disposition at all.
- **BREAKING: `diff-cross-spec` reports this spec's impact, not the corpus's.** Its window
  now starts at the spec's last reopen rather than its first spec-directory
  commit, and it keeps a sibling path only when a commit belonging to the spec
  changed it or it carries an uncommitted change. A spec never `in-progress` still
  starts at its first commit. The result's `first-commit` is renamed
  `window-base`.
  Breaking under `BE-COMPAT-001`: `first-commit` is renamed `window-base`.
- **A fix round reads only the review's own fixes.** `brief-re-read` now drops
  a path only commits another spec claims changed, found through each snapshot's
  parent with nothing reported by the agent, and returns it as `excluded`, unless
  an uncommitted edit sits on top at either snapshot; a commit this spec or no
  spec claims stays. An exempt path leaves the brief and `round.toml` even in a
  round that also needs a re-read, and is returned as `exempt`, so a mixed round's
  re-read reads only its non-exempt paths. `write-review` names each path a round
  left out, from `fix-rounds[].excluded`.
- **A review cut off between its writes holds `done`.** `write-review`'s new
  `loop-open` writes `fix-rounds-open: true`, which the command's first write
  passes and its last omits, and `check-review-gate` blocks while it is present,
  as `fix-round-loop-open`. A record with no mark, an older one or one from `ductus
  exec`, is unaffected.
- **A trip back through `draft` stays in its reopening.** A spec's reopening now
  starts where it last left `done` or, never `done`, where it first entered
  `in-progress`, for the review window, `diff-cross-spec`, `check-stuck` and
  `measure-reopenings`. A return to `in-progress` used to start a new one, so the
  earlier code left the window and reached `done` unread; runs on both sides of the
  trip now count together, so a second clean review is over the bound.
- **BREAKING: the pre-`done` gate blocks on a review a spec owes, ahead of every analyze
  check.** `check-review-gate`'s `blocked-by` gains `review-owed`, which names what
  changed since the last review and clears once a review's record is committed.
  `review.md`'s missing-map, outstanding-finding and fix-round checks move ahead of
  the analyze checks, since a review stales an analysis run before it, so a
  reopening that changed code takes one of each; a message no longer names both
  records together. `compute-review-scope`'s `review-owed` no longer counts a path
  in another spec's directory, which that spec's review reads. A window that cannot
  be read passes, with guidance saying so.
  Breaking under `BE-COMPAT-001`: `blocked-by` gains `review-owed`, a new
  value of a response enum the schema does not document as open to new values.
- **A window no longer goes wrong on a rename, a revert or a shallow clone.** A spec
  is found through history by its number, so a slug changed mid-reopening keeps its
  window and `diff-cross-spec` no longer reads its old directory as another spec's.
  A commit restoring an older review record starts the window at the run it restores,
  not at itself. A shallow clone whose history stops inside the reopening makes
  `compute-review-scope` an error, and gives `diff-cross-spec` and `check-stuck`
  guidance, where each answered as if history were whole.
- **A `done` spec cannot hold an unticked criterion unreported.** `check-artifacts`
  reports one as blocking `review-state-drift`, which `/analyze --fix` reverts, and
  `classify-spec-edit` reads taking a tick back as `reopens`. Ticking one stays a
  `correction`.
- **`classify-spec-edit` recognizes a mechanical edit.** A scenario or data-model
  edit that only adds, removes or rewrites a cross-service reference, or is a sweep's
  by the rule the review gate's staleness uses, now reads `mechanical`, which leaves
  a `done` spec `done`, instead of `reopens`. A real change made with the same token
  in two files reads as a sweep, as it does for staleness.
- **A review no longer expires a person's decision on a file it did not read.**
  `process-waivers` and `process-decisions` retain a stored waiver or decision that
  did not fire when its file is still on disk and the run's scope does not hold it,
  or when that scope cannot be read. As windows narrowed, a run's silence on such a
  file had come to read as the finding having gone. A file gone from disk, or a run
  that read the file, still expires it, and both primitives take `since`.
- **A fold below `done` brings its code to the next review.** A branch-scoped spec
  removed below `done` adds the paths of every commit it claimed to its target's
  `compute-review-scope` scope, wherever the window starts. A fold into a `done`
  spec reopens it at the fold, so that code lay before the window, in no list, and
  one review and one analysis passed the gate with it unread. A spec that reached
  `done` had its code read by its own review, and brings nothing.
- **A review reads its own spec's change, and a later run starts where the
  last one finished.** `compute-review-scope`'s default window now starts at the
  later of the spec's last reopen and its last finished review, reported as
  `base-source`; a run that recorded a skipped pass or a budget stop, or whose
  record the gate holds until a review runs, does not move it, so a re-run reads
  again what a finding names rather than clearing it on an empty scope. A commit belongs to each spec whose directory
  it touches or whose number its subject's leading scope names, and the scope
  holds the paths of a commit that belongs to the spec alone or to no spec; of a
  commit several specs claim, the paths in the spec's directory or its plan's
  Affected Files, and a path in none of their plans when its subject names this
  spec first or, outside the other specs' directories, names none of them; and
  uncommitted work. A plan entry unchanged
  in the window stays only on the spec's first review. What the scope leaves out
  is the result's `excluded` list, and `review.md` gains an `excluded` count and
  an `Excluded from scope` section listing each path.
  A new `review-owed` is false when a review record names a run and the scope
  holds nothing outside the spec's directory and no scenario or data-model edit,
  and `/review` then reports that and stops, since such a reopening owes an
  analysis and no review. It is true while the record holds `done` until a review
  runs or its digest is stale, and when the reopening deleted code or a contract. A
  commit moving a run's record out of `spec.md`, and a merge carrying one in, do not
  move the window's start.
- **The error envelope's home is `specs/errors.md`.** `api-backend.md`'s
  `BE-ERRENV-001`, `BE-ERRENV-002` and `BE-ERRENV-003` read the project's error
  envelope from `specs/errors.md`, where the shipped `errors.md` template
  already puts it, instead of `specs/system.md`, and `security-backend.md`'s
  `BE-ERR-004` reads a non-HTTP API's documented error format there too. A
  project that documented its envelope in `specs/system.md`, as the rules said
  since `gvrn-v0.5.1`, meets `BE-ERRENV-001` on its endpoint specs until it
  moves the section to `specs/errors.md`, and a non-HTTP format left there
  draws `BE-ERR-004`'s warning. Spec 008, task 34; spec 068.
- **`security-cross.md`: security rules for every project shape.** A new rule
  file, installed whatever `[rules] surfaces` says. On a project with feature
  specs, `/ductus`'s one-time brownfield security audit now also runs when it
  newly installs `security-cross.md`, as the upgrade to 0.60.0 does, and
  reports each spec one of its rules finds a gap in. It now applies each rule
  to a spec as `/analyze` does — its trigger, commitment and flags judged
  against the spec's `spec.md`, `plan.md`, `data-model.md` and scenarios
  together, with a cross-spec file the spec or the rule refers to, such as
  `specs/system.md`, one finding per spec and rule — sets aside what the rule
  sets aside, and leaves a case a rule names as another rule's to that rule,
  judging the yield's load condition against every rule file installed, as
  `/analyze` does, so the case is reported under the owner's ID when the run
  newly installs the owner's file too, and not at all when that file was
  already installed. An upgrade of a backend-only or full-stack project
  therefore previews none of `SEC-LOG-001`'s gaps, which `BE-LOG-002` and
  `BE-LOG-008` own there, and `/analyze` reports them. An owner outside the
  security rule files, such as `configuration-cross.md` `CFG-ENV-006` for a
  spec that introduces a sensitive configuration value and logs it, is loaded
  and applied to that case alone, its Verification applied as `/analyze`
  applies it, so a first adoption previews the case under its owner's ID — a
  `configuration-cross.md` rule such as `CFG-ENV-006` or `CFG-ENV-013`, an
  `api-backend.md` rule or an `observability-backend.md` rule — where the
  yielding rule's trigger fires too and `/analyze` reports it, and an owner's
  file that fails to load is named in a `failed to load` line. `SEC-LOG-001` keeps
  passwords, tokens, session IDs, keys, one-time and recovery codes, PII and
  other sensitive field values out of logs, error reports and telemetry in
  every shape; where `security-backend.md` is also loaded, `/analyze` reports
  those cases under `BE-LOG-002` and under `BE-LOG-008`, which first ships in
  this release, and `configuration-cross.md` `CFG-ENV-006` names `SEC-LOG-001`
  where it is not. Two rules 0.59.11 shipped in `security-backend.md` move to
  `security-cross.md`, deprecated in 0.60.0 and removed in 0.62.0:
  `BE-AUTHN-009` (constant-time comparison) to `SEC-AUTHN-001`, and
  `BE-AUTHN-013` (OAuth and OIDC client `state`, PKCE, `nonce` and exact
  `redirect_uri`) to `SEC-AUTHN-002`. Two more were added to
  `security-backend.md` after 0.59.11 and moved before a release shipped them:
  `BE-DEPS-006` (build-input pinning) to `SEC-DEPS-001`, and `BE-AUTHN-020`
  (OAuth grant and response types, the PKCE method and public-client refresh
  tokens) to `SEC-AUTHN-003`; this release removes both (see Removed). A
  project upgrading from 0.59.11 meets their successors, `SEC-DEPS-001` and
  `SEC-AUTHN-003`, as new
  MUST rules: `SEC-DEPS-001` flags an action referenced by tag or branch
  outside a workflow onboarded to a GitHub Actions workflow dependency
  lockfile, a reusable workflow called by `{owner}/{repo}` and a tag or
  branch, a container or base image by tag alone, and a download run without
  checksum verification. `SEC-AUTHN-004` takes `BE-API-009`'s
  authorization-server half: an OAuth authorization server the project
  serves or provides, a library in a cross-only project included, compares
  each `redirect_uri` it receives with a registered one by exact string
  matching, and `/analyze` reports the case under `SEC-AUTHN-004` rather than
  `BE-API-009` or `FE-AUTHN-001`. A frontend-only or cross-only project is
  now held to all of them: a frontend-only OAuth client needs an exact registered
  `redirect_uri` where `FE-AUTHN-001` asked only for an allowlist, and
  `/analyze` reports a client's `redirect_uri` under `SEC-AUTHN-002` rather
  than `FE-AUTHN-001`, and an access token an authorization response returns
  in a URL fragment under `SEC-AUTHN-003` rather than `FE-STORAGE-003`. In
  every shape, `/analyze` reports a package a build step's command runs by a
  mutable reference, such as `npx tool@latest`, under `SEC-DEPS-001` rather
  than `BE-DEPS-002` or `FE-DEPS-004`. A spec citing `BE-AUTHN-009` or
  `BE-AUTHN-013` draws an Advisory finding until 0.62.0 and a Blocking one
  after, so re-point it to the successor. Until spec 020's owner routing lands, `/review`, which
  applies Statements and no routing, reports one defect under two IDs: a
  moved case under the old ID and its successor; where `security-backend.md`
  is loaded, a sensitive value in a log line, an error report or telemetry
  under `SEC-LOG-001` beside `BE-LOG-002` or `BE-LOG-008`; where
  `security-frontend.md` is loaded, a client's `redirect_uri` under
  `SEC-AUTHN-002` beside `FE-AUTHN-001`, and a token in an authorization
  response's URL fragment under `SEC-AUTHN-003` beside `FE-STORAGE-003`; and a
  package a build step's command runs by a mutable reference under
  `SEC-DEPS-001` beside `BE-DEPS-002` or `FE-DEPS-004`; and an authorization
  server's `redirect_uri` under `SEC-AUTHN-004` beside `BE-API-009` or
  `FE-AUTHN-001`. A sensitive configuration value in a log line, an error
  report, a span, a metric label, a product-analytics event or a
  session-replay recording stays under both `SEC-LOG-001` and `BE-LOG-002` or
  `BE-LOG-008` after that routing lands, and one in a URL under both
  `SEC-DATA-001` and `BE-API-013` or `FE-STORAGE-003`, since each yields it to
  `configuration-cross.md`, which no review pass applies yet. A waiver
  recorded under one ID of such a pair needs a counterpart under the other;
  one under the yielding ID (the old ID, `BE-API-009`, `SEC-LOG-001`,
  `FE-AUTHN-001`, `FE-STORAGE-003`, `BE-DEPS-002` or `FE-DEPS-004`) expires once that routing moves the finding to its owner,
  and an old ID's also when it is removed. A `specs/rules/security-backend.md`
  pinned from 0.59.11 keeps `BE-AUTHN-009` and `BE-AUTHN-013` undeprecated
  beside their successors, so `/analyze` reports each such case under both
  IDs, and has no `BE-LOG-008`, so `/analyze` reports under no rule a
  sensitive value in propagated trace context, one other than configuration
  in an error report, a span, a metric label, a product-analytics event or
  user property or a session-replay recording, and a missing redaction
  commitment for those
  carriers. A copy pinned from `main` after 0.59.11 keeps
  whichever moved rules it had not yet deprecated, all four from `1598b87a`
  until `ad8a36f9`, and a `BE-DEPS-006` deprecated there may still name
  removal in 0.61.0. Any pinned `security-backend.md` keeps `BE-DEPS-002`'s `npx` case,
  and any pinned `security-frontend.md` `FE-DEPS-004`'s, beside
  `SEC-DEPS-001`; and a pinned `specs/rules/security-frontend.md`'s
  `FE-AUTHN-001` and `FE-STORAGE-003` keep their OAuth cases beside
  `SEC-AUTHN-002` and `SEC-AUTHN-003` in a project without
  `security-backend.md`, or in any project when the copy predates
  `9b067399`, 0.59.11's among them, and one that predates `86f46805` keeps
  `FE-AUTHN-001`'s case of a `redirect_uri` an authorization server receives
  beside `SEC-AUTHN-004` in a project without `security-backend.md`, or in
  any project when it predates `afb8a865`. Each lasts until the file is
  re-synced. A
  `security-backend.md` or `security-frontend.md` left from a surface the
  project no longer configures stops receiving updates, with the same results
  until the file is deleted or its surface configured again. A project that
  wrote its own `specs/rules/security-cross.md` should rename it before
  upgrading: the upgrade replaces it with the shipped file, and no audit
  runs for it, since the file is not newly created. Pinning it instead
  keeps the shipped file out, so `/analyze` reports under no rule every case
  another rule file yields to a `SEC-` rule. A project upgrading from 0.59.11 meets
  `SEC-AUTHN-004` as a new MUST rule, since 0.59.11's `BE-API-009` had no
  authorization-server sentences; a `security-backend.md` pinned from `main`
  between `9b067399` and `86f46805` keeps them, so `/analyze` reports such a
  server's `redirect_uri` matching under both IDs until the file is
  re-synced. Spec 077; spec 008, tasks 32, 40, 42 and
  43.
- **BREAKING: a finding is fixed, deferred or waived: `routed` is now `deferred`, and
  `discarded` is now `waived`.** `review.md`'s and `analysis.md`'s
  `dispositions:` map counts `deferred` and `waived` where it counted `routed`
  and `discarded`, and a stored decision's `outcome` is `deferred` or
  `waived`. Both writers read a record 0.59.11 wrote, old keys and outcomes
  included, and write only the new names on their next run, so a CI step or
  script of your own that reads `dispositions.routed`,
  `dispositions.discarded`, `outcome: routed` or `outcome: discarded` must
  read the new names. A map
  carrying a key beside its old name (`routed` and `deferred`) does not parse.
  A deferral names its work — a `target`, a `task` in a `tasks.md` target, an
  external `work-item` ID, or pipeline work and a work item both — and is
  refused when it names none, or a target or task that does not exist; a
  work-item ID is any one-line text, stored as given and never read. A waiver
  states its reason and names no work. `/review` defers a MUST or SHOULD
  violation with `--defer <rule-id>` and `--target` (with `--task` for a
  `tasks.md` target), `--work-item`, or both, stored under `decisions:` keyed
  `rule {rule-id} at {file}`; a deferral naming other work than a standing one
  replaces it, and a waiver over a standing deferral is refused unless that
  deferral is passed in `expired-decisions`. A deferral of a gating finding — a
  review MUST or SHOULD, an analyze hard-fail or blocking finding — releases
  `done` only when its work is still to do on another spec (an unchecked task,
  a scenario or a body edit on a spec not `done`), and then leaves that
  record's counts; any other — to
  the spec's own work, a rule file, complete work, or a work item alone — is
  stored with `gating: true`, and `check-review-gate` names it where it
  blocks. `process-decisions` returns a deferral whose named work is complete —
  its task checked or gone, its spec `done` or gone, or a rule file — while
  its finding still fires, under `lapsed`, and the finding needs a new
  disposition. Spec 076.
  Breaking under `BE-COMPAT-001`: `dispositions.routed` and
  `dispositions.discarded`, and the outcomes `routed` and `discarded`, are
  renamed `deferred` and `waived`, and a deferral naming no work is refused.
- **BREAKING: `compute-review-scope` and `diff-cross-spec` results stay within one
  read.** Each result now carries `form` and each list's count
  (`scope-count`, `modified-since-count` and `plan-affected-count`;
  `cross-spec-count`). While a result fits `[artifacts] read-size-bytes`,
  50,000 bytes by default, its lists are inline as before; past it, `form` is
  `file`, no list is inline, and `lists-file` names a JSON file under the
  self-ignoring `.ductus/results/` holding them whole, so a review base with
  months of history no longer exceeds a host's tool-output cap. `/review`
  takes an empty scope from a zero `scope-count` and its in-scope paths from
  that file, and no longer re-runs the call through the runtime binary;
  `ductus exec`'s review request reads its scope from the file, halting with
  `lists-file-unreadable` when it cannot. A script of your own that reads
  `scope` or `cross-spec-paths` from either result must read `lists-file`
  when `form` is `file`. Breaking under `BE-COMPAT-001` past the read size,
  where `scope`, `modified-since`, `plan-affected` and `cross-spec-paths`,
  inline in 0.59.11, are not. Spec 066, task 7.
- **Ticking an acceptance criterion no longer stales the analysis.**
  `analysis.md`'s `analyzed-digest` takes `spec.md` with each acceptance
  criterion's checkbox as unticked, so `/implement`'s completion run, which
  ticks the criteria it verifies, passes the gate on the analysis it began
  with instead of stopping at `analyze-stale`. Any other change to
  `spec.md`, a criterion's wording included, still stales it. A record
  written before this release over a spec with a ticked criterion reads stale
  once, until the spec's next `/analyze`. Spec 066, task 9.
- **BREAKING: `compute-review-scope` reads every path a plan cell names, and sets
  deleted files apart.** An Affected Files cell naming several backticked
  paths (`` `a.md` / `b.md` ``) puts each in `plan-affected`, where only the
  first counted; a later span that is a command, a flag or an anchor stays
  out. A file deleted since the diff base stays in `modified-since`, is named
  in a new `deleted` list beside its `deleted-count`, and leaves `scope`, so a
  review that reads every readable file is no longer counted short in
  `examined`. A script reading `scope` no longer finds deleted paths there.
  Spec 066, task 10.
  Breaking under `BE-COMPAT-001`: `scope` no longer lists a path deleted since
  the diff base, which 0.59.11 listed there; read `deleted`.

- **The security, configuration and performance rules close the gaps a
  re-read round found.** Each new obligation is an `/analyze` finding for a
  spec that meets its rule's trigger. In `security-cross.md`, installed in
  every shape: `SEC-AUTHN-002` asks an OAuth or OIDC client for a one-time
  `state` bound to the user agent, an ID token's `iss`, `aud` and `exp`, and
  its signature, with a key the issuer provides, unless it came directly from
  the token endpoint, `state`, `code_verifier` and `nonce` from a
  cryptographically secure generator, a mix-up defence where it can talk to
  more than one authorization server, and a native app's external
  user-agent, and asks a change to an existing client for the requirements
  the change reaches, a redirection endpoint `state` and exact matching;
  `SEC-AUTHN-003` asks an authorization server to enforce PKCE at its token
  endpoint and never to redirect a credential-carrying request with 307, a
  change to an existing server's token endpoint or sign-in step included,
  and a change to the refresh tokens it issues to public clients for their
  protection; a
  new `SEC-DATA-001`, in a new `DATA` category, keeps a credential or PII out
  of a URL the project's code requests, sends or renders, a server or
  command-line tool calling another party's API with its key in the query
  included; `SEC-DEPS-001` reaches a container action's own image, a
  SHA-pinned action or reusable workflow whose own references take tags
  outside a lockfile, a `./` local action whose path a checkout of a branch
  or tag holds, a larger runner's custom image set to `Latest`, and
  `specs/system.md`, and sets aside a GitHub-hosted runner label that selects
  a GitHub-owned or partner image and a `$/` local action, with a `./` one
  checked out at the running commit or a full SHA, so a waiver of `runs-on: ubuntu-latest`
  can go; and `SEC-LOG-001` adds
  product-analytics events and session-replay recordings. In
  `security-backend.md`: `BE-LOG-008` adds the same two carriers;
  `BE-API-008` reaches search engines and coordination services;
  `BE-API-011` reaches a form or client code sending the token; `BE-ERR-002`
  asks a spec that introduces an API returning errors to callers outside its
  process, gRPC, JSON-RPC and MCP included, for its error format;
  `BE-AUTHN-012` and `BE-AUTHN-019` read an OIDC client's verification of an
  ID token's signature as JWT verification, the token's claims staying
  `SEC-AUTHN-002`'s, `BE-AUTHN-012` (e) applying where a spec
  issues tokens; and `BE-API-013` sets aside OpenID Connect Front-Channel
  Logout's `iss` and `sid`. In `security-frontend.md`: new `FE-DEPS-007`
  (SBOM), `FE-DEPS-008` (registry signatures and provenance) and
  `FE-DEPS-009` (dependency confusion) hold a frontend-only project to what
  `BE-DEPS-003` to `-005` hold a backend-only or full-stack one, a cross-only
  project being held to neither; `FE-STORAGE-003` sets aside an
  RFC 8058 `List-Unsubscribe` identifier and Front-Channel Logout's `iss` and
  `sid`, and its commitment no longer fails a callback that receives an
  authorization code; and `FE-CSRF-003` sets aside an OAuth redirection
  endpoint's `GET`. In `configuration-cross.md`: `CFG-ENV-006` reaches a
  sensitive value in trace context outside a backend, a message to a
  person, a configuration service client code fetches, such as Firebase
  Remote Config, product analytics and session replay, and a value newly
  written to a log line, and `CFG-ENV-010`'s trigger is the build's; an
  analytics ID is no longer named as public. `api-backend.md`'s
  `BE-ERRENV-003` judges `specs/errors.md`'s envelope, and
  `performance-frontend.md`'s `FE-LOAD-007` asks a spec that introduces a
  server-rendered page referencing build-emitted assets for its cache
  policy. At `/analyze`, a case a changed
  rule now names as another's is reported once, under that rule; `/review`
  applies Statements and no routing until spec 020's lands, so it reports
  this batch's new pairs under both IDs: `SEC-DATA-001` beside `BE-API-013`
  or `FE-STORAGE-003`, `SEC-AUTHN-002` beside `BE-AUTHN-012`, `BE-AUTHN-019`
  and `BE-DATA-009`, `BE-AUTHN-012` beside `SEC-AUTHN-003` for a public
  client's refresh token, and `FE-DEPS-007` to `-009` beside `BE-DEPS-003` to
  `-005`, a waiver under one ID of such a pair needing a counterpart under
  the other, and one under the ID that yields the case expiring once that
  routing moves the finding to its owner. A newly created
  `specs/errors.md` has its example envelope carry a `request_id`, as
  `BE-ERR-002` requires; an existing one is not rewritten. A pinned copy of
  a changed file keeps its old text until re-synced. A `security-backend.md`
  pinned from 0.59.11, or left from a surface the project no longer
  configures, has no `BE-LOG-008`, so beside it `configuration-cross.md`'s
  yield of a configuration value in trace context and `SEC-LOG-001`'s yields,
  product-analytics events and user properties and session-replay recordings
  now among them,
  report the case under no rule until the file is re-synced or deleted, as
  does `SEC-AUTHN-002`'s yield of an ID token's key source, 0.59.11 having no
  `BE-AUTHN-019`, while the cases `SEC-DATA-001` yields to `BE-API-013` and
  `BE-API-015`, absent there too, stay `SEC-DATA-001`'s; beside a pinned one,
  so do the current `observability-backend.md`'s
  `BE-METRIC-003` and `BE-TRACE-002`, which yield a sensitive metric label or
  span attribute to `BE-LOG-008`, where 0.59.11's `BE-METRIC-003` flagged an
  email label itself. Spec 008, tasks 43 and 44.
- **Three security and configuration rules agree with their neighbours.**
  `BE-AUTHN-008`'s Statement now has a plain-HTTP request to a login
  or credential-change endpoint refused, not redirected to HTTPS, which its
  Verification already flagged, so `/review`, which applies Statements, flags
  the redirect too. `configuration-cross.md`'s `CFG-ENV-005` has an
  out-of-range value's error redact a sensitive setting per `CFG-ENV-006`.
  `security-frontend.md`'s `FE-CSP-008` leaves a missing `X-Frame-Options` to
  `security-backend.md` `BE-API-012` where that file is also loaded, so
  `/analyze` reports it once; `/review` reports it under both until spec
  020's routing lands. Spec 008, its review.
- **`QUAL-TEST-001` flags a bound test that does not pin its figure.**
  `quality-cross.md`'s `QUAL-TEST-001` also flags a test of a limit on a
  size, a duration or a count whose figure a spec or contract states when it
  checks the code's own constant, or the figure on one side only, so the
  constant can move without the test failing. Asserting the figure itself, or
  the behavior at it and just past it, complies. Spec 036, its scenario
  `a-bound-test-pins-the-stated-figure`.
- **`BE-LOCK-002` has a set rewrite lock its parent first.**
  `concurrency-backend.md`'s `BE-LOCK-002` adds, at its SHOULD tier, that a
  transaction deleting or rewriting a set of child rows as a whole first
  locks the parent row that owns the set, or an equivalent serializer such as
  an advisory lock keyed on the parent, and `/analyze` flags a plan's set
  rewrite that does not. Its Source quotes MySQL's current wording. Spec 038,
  its scenario `lock-the-parent-before-rewriting-its-children`.

- **The eleven shipped rule files are re-reviewed against their sources: 208
  rules become 297, and the review renumbers, removes or deprecates no
  existing ID (specs 008, 017, 034, 036, 037, 038, 039).**
  The counts are `### {ID}` headings across the eleven files at the review's
  commit; later entries in this release add rules and move some, so the
  release ships 310 across twelve files at `79cd59a3` (`security-cross.md` is
  new), against 208 across eleven at `ductus-v0.59.11` (`### {ID}` headings,
  `grep -h '^### ' framework/rules/*.md | wc -l`, the line count of
  `runtime/src/primitives/shipped_rule_ids.txt`, which a test holds to the
  files). Three passes
  fetched every external claim from its source, and a fourth re-checked each
  rule changed after them. New obligations are `/analyze` findings for a spec
  that meets a rule's trigger: `BE-AUTHN-022` (re-authentication for a
  sensitive account change), `BE-API-015` (WebSocket handshake and origin),
  `BE-TIMEOUT-003` (a server-side statement timeout), and the further rules
  each file gains in the accessibility, API, concurrency, configuration,
  observability, performance, reliability and security sets. A MUST no longer
  requires continuous integration: the outcome is the MUST, and running the
  check on every change is a SHOULD of its own (`BE-SCHEMA-003` and `-004`,
  `FE-BUNDLE-001` and `-003`, `BE-DEPS-001` and `-007`, `FE-DEPS-001` and
  `-006`, `FE-A11YTEST-001` and `-003`). Service-level objectives and alerting
  stay out of rule files, bcrypt is legacy-only, `FE-BUNDLE-001` carries no
  default budget figures, and `QUAL-GROUND-001` covers in-repo contracts and
  leaves configuration to the `CFG-ENV` rules. `configuration-cross.md` gains
  a `FLAG` category and renames `ENV` to "Environment and configuration
  values". Every rule file header states its enforcement mode and the same
  opt-out paragraph, which now says a project may exclude an unconditional
  `-cross.md` file through `[[review.disabled-rule-files]]` as well as a
  stack-selected one. The constitution's `§rules` states the two authoring
  disciplines, and its configuration summaries match the files. `/analyze`
  accepts `CFG-FLAG` as an ID category. Citations of malformed-configuration
  handling move from `CFG-ENV-003` to `CFG-ENV-008` in `/review`, the
  installer and the registry error comment, and the spec template's
  `BE-AUTHN-001` example reads "deliberately slow password hashing". Commits
  `1598b87a`, `7577068c`, `59ba4cc0`.
- **The owner's answers on the security rules tighten and narrow eight
  obligations (spec 008, tasks 16 to 18).**
  `BE-AUTHN-001` accepts any minimum configuration the OWASP Password Storage
  Cheat Sheet lists as equivalent (five Argon2id, five scrypt), and flags
  PBKDF2 chosen without the FIPS 140 requirement that makes it the choice.
  `BE-AUTHN-022` requires a fresh verification with an enrolled second factor
  or passkey to replace or remove a second factor, enrol another beside an
  enrolled one, or turn MFA off; the current password alone no longer
  authorizes it, though it still authorizes the other changes and a first
  enrolment. A user who lost every enrolled second factor may bind a
  replacement through account recovery that meets NIST SP 800-63B-4 4.2's AAL2
  bar and notifies the user independently, and the Verification checks how
  recovery codes are made, stored, invalidated and expired. `BE-AUTHN-018`'s
  expiry, newer-token invalidation and reset sweep apply to the tokens a flow
  issues, and a recovery code saved in advance is exempt until used, then
  replaced. `BE-API-015` says a WebSocket endpoint MUST NOT accept a token
  carried in the handshake URL, not even the single-use ticket `BE-API-013`
  allows elsewhere. `BE-API-011` and `FE-CSRF-001` keep rejecting, or checking
  `Origin` on, a request without `Sec-Fetch-Site`, a design that falls back to
  a CSRF token complies through the token, and `SameSite` stays barred as the
  sole defense. `BE-API-013` and `FE-STORAGE-003` gain a scoped, expiring MAY
  for a pre-signed object-storage URL (one object, one method, a stated
  expiry) and allow a single-use token in a path or a query. `FE-XSS-003` now
  flags `eval`, `document.write`, `Function(...)` and `outerHTML` outright,
  which is stricter for a spec that sanitizes before `outerHTML`. Commits
  `3740a899`, `2423de2a`, `c950b949`, `4ce35aab`, `c4de83ce`, `cfda1404`,
  `9028bb9d`, `273492df`, `24c65619`, `343bf2c8`, `647153ce`, `fc876871`,
  `c732fcdf`, `88d934d4`.
- **The configuration rules cover a value from any source (spec 017, tasks 39
  to 41).**
  `CFG-ENV-001`'s required-or-optional declaration and `CFG-ENV-003`'s startup
  validation cover environment variables, configuration-file keys and
  command-line flags, so a required key missing from a file fails fast naming
  the setting and the file; a value read from several sources is absent only
  when every source in `CFG-ENV-007`'s order leaves it absent. `CFG-ENV-005`'s
  range check covers the same sources and its error names the setting, the
  file for a file key, the value and the violated bound. `CFG-ENV-002`
  requires every environment variable and every configuration-file key to be
  listed in one canonical inventory per kind of source or one for both, never
  two for one source; a commented example of the file is accepted, and flags
  are left to help output. A command-line flag is configuration only when it
  supplies a setting an environment variable, file or default could supply,
  and an argument naming what one invocation acts on stays with the
  interface's help output. `CFG-CONST-003` names a configuration value
  declared per `CFG-ENV-001` as an operator-tunable value's backing. The
  installer's `§Project Configuration` now states the defaults it left
  implicit: `[project]` keys have none and are asked for (`name` falling back
  to the command argument and then `[host] project`), an absent `[pinned]`
  pins nothing, an absent `[[review.disabled-rule-files]]` excludes nothing,
  and an absent `[review]` `tech-stack-verified` means false. Commits
  `6b0f52da`, `a5e18ad7`, `68b908a6`, `45aa399e`, `8dd72c61`, `24c65619`,
  `fc876871`.
- **Constitution: decisions go to the owner one at a time, as a default a
  project may replace (spec 050).**
  A grouped list of decisions gives no clean way to clarify one question or
  one answer, and a question put while background work is still reporting
  scrolls out of view. `§recommendations` now states the one-per-message rule
  and names it a default a project's `AGENTS.md` may replace, which the
  governance-precedence floor allows. Commit `be893cd0`.
- **`/plan` told an author to name one file per Affected Files row, a rule
  since replaced.**
  `/review` built its scope from the first path in each row's File cell, so a
  second path in the same cell was dropped from it. `compute-review-scope` now
  reads every backticked path a cell names (see its entry in this release),
  and `/plan`'s Affected Files step says so instead. Commit `8d426a72`.
- **The adopter CI template installs the pinned release binary and pins its
  action (spec 017, task 42; spec 022, task 3; `BE-DEPS-006`, since moved to
  `SEC-DEPS-001` by the `security-cross.md` entry).**
  `framework/templates/ci/adopter-generators.yml` could not run: a checkout
  carries no runtime, since `/ductus` keeps it in a per-machine store linked
  from the gitignored `.ductus/bin/`. Its first step now downloads
  `ductus-x86_64-unknown-linux-gnu.tar.gz` at `DUCTUS_VERSION` (0.59.11),
  checks it against `DUCTUS_SHA256` before anything runs, refuses a binary
  reporting another version, and puts it on `PATH` from `$RUNNER_TEMP`,
  outside the work tree, so the sync check cannot see it. The download is
  bounded by `DOWNLOAD_CONNECT_TIMEOUT_SECONDS`, `DOWNLOAD_MAX_TIME_SECONDS`
  and a step `timeout-minutes`, and is not retried (`BE-TIMEOUT-001`,
  `BE-RETRY-001`); a failed one fails the job. No Rust toolchain is needed; on
  an arm runner the x86_64 binary fails when run. The derive step calls
  `ductus` from `PATH`, and `actions/checkout` is pinned to a full commit SHA
  with its release tag in a comment. A release cannot carry its own archive's
  hash, so the template names a published release and its pin moves after each
  one: re-copy the template, or set both values from that release's `.sha256`
  asset. Neither `/ductus` nor any scaffolding pass writes the template, which
  an adopter copies by hand, and the header and the `generator-primitives`
  migration's CI step now say so; a copy still invoking
  `.ductus/scripts/gen-spec-deps.sh` fails on its first run. Commits
  `458fa3e3`, `d235e722`, `b85e4907`, `02a5ac13`, `fa489f6a`.
- **BREAKING: a malformed `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` entry fails
  every `fetch-archive` call (spec 022, analysis of 2026-10-20; `CFG-ENV-002`,
  `CFG-ENV-008`).**
  The variable is now parsed once at startup into bare host names, trimmed,
  with empty entries skipped and each lower-cased as a URL's host. An entry
  with a scheme, port, path or credentials could never match a host, so it
  silently exempted nothing; every fetch now fails with an `invalid
  DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` error naming the entry, which must be a
  host name or a bracketed IP literal. An adopter whose value carries a scheme
  or port, and relied on its being ignored, now fails until it is corrected. A
  later entry narrows the variable further: plain `http` passes only from a
  named loopback host. Commit `307b920c`.
- **BREAKING: `check-artifact-size` halts on an invalid `[artifacts]
  read-size-bytes` instead of falling back to 50,000 (spec 063, task 12;
  `CFG-ENV-008`, `CFG-ENV-005`).**
  A present value that is not a positive whole number of bytes, or an
  `artifacts` that is not a table, fails with `InvalidReadSize`, naming the
  setting, the value (flattened to one line, so a newline in committed config
  cannot forge a second line), the file and the accepted form. An unset key
  still means 50,000. The result's `threshold.source` can no longer be
  `invalid`, and `threshold.rejected` is removed, since nothing produces
  either; an adopter's caller that read them must handle the error instead.
  `/analyze`, `/clarify` and `/plan` (runtime and markdown-only paths), the
  MCP tool description and the installer's `[artifacts]` comment state the
  halt. Spec 063, scenario `an-invalid-threshold-fails-fast`, commits
  `a226ba3d`, `2bfdfa4e`.
- **BREAKING: `discover-rule-files` halts on a malformed
  `[[review.disabled-rule-files]]` entry, and `/review` stops with it (spec
  025, task 10; `CFG-ENV-008`).**
  Every entry is validated before any rule file is selected. One that is not a
  table, has a `file` or `reason` missing or not a string, or has a `reason`
  under 16 trimmed codepoints fails with `InvalidDisabledRuleFile`, naming the
  entry's index and the failing field; the entry used to be skipped with a
  warning. Duplicates keep their warning, and fields an adopter adds to an
  entry are still accepted. The halt is a configuration error, not a finding,
  so `blocking` still means a MUST violation, and the run exits non-zero.
  `file` and `reason` in the remaining notices, and each disabled file name
  `/status` shows, are flattened to one line so a newline cannot forge a
  `loading rule files:` or dashboard line (`BE-INPUT-011`). `/review`'s source
  and the installer's schema comment state the halt. Spec 025, scenario
  `a-malformed-disabled-entry-fails-fast`, commits `a32af25b`, `036f03cb`,
  `bee36927`.

- **BREAKING: a `.ductus/config.toml` or session file that does not parse, or an invalid
  `[paths] specs-root`, halts the call, naming the file (spec 022, task 12;
  `CFG-ENV-008`).** In 0.59.11 the readers were best-effort: `Host::load` (the
  `[host]` block and the session file's `cli-config-dir`) and `Paths::load`
  (`[paths] specs-root`, behind every spec-root lookup) logged a one-line
  warning to stderr for a file that did not parse, or a `specs-root` that
  failed validation, and used the default (`.claude`, the repository's
  directory name, `specs`), so the call went on against a layout nobody had
  configured. Each reader now returns an error naming the file, with the parse
  error or, for `specs-root`, the rejected value and the reason, and a
  primitive that resolves the spec root or the host halts on it: a CLI call
  exits 1 with the error on stderr, an MCP call returns it as the tool's
  error, and a `ductus exec` walk halts. `check-orphaned-references`' read of
  `[migrations] last_applied` halts the same way, `write-review` and
  `write-analysis` too (the spec root their record goes under cannot be read),
  and the exec seed reads the shared default session file strictly. A missing
  file, an absent table or key and an empty or blank `specs-root` still mean
  the default. The MCP server still starts over a host config that does not
  parse, flags no tool as always loaded and names the file, and every
  primitive that reads it halts; `write-session` still replaces a shared
  default that does not parse and names it in `unreadable`, so `/target`
  repairs it. A script or CI step that ran past the warning fails until the
  file is corrected. Commit `92120f45`.
  Breaking for a caller that ran past the warning: a `.ductus/config.toml` or
  session file that does not parse, or an invalid `[paths] specs-root`, now
  fails the call.
- **BREAKING: every path argument is resolved through its symbolic links
  before the containment check (spec 022, task 4; `BE-INPUT-004`, scenario
  `a-path-argument-resolves-inside-its-base`).** The lexical check read a
  path's text alone, so a link inside the project that pointed out of it
  passed: `set-status` rewrote an outside `spec.md`, `create-scenario` wrote
  outside through a linked `scenarios/`, and `mark-task` read an outside
  `tasks.md`. A new `validate_contained` refuses a path when a link on it
  resolves outside the canonicalized project root, compared by whole
  components; a path not created yet is judged by its deepest existing
  ancestor, a dangling link by where its target would resolve, and a link loop
  is refused. Every feature argument also walks the feature directory and
  refuses the call when any link in it resolves outside the project, so the
  fixed-name files a primitive opens beneath it are covered. The check runs at
  the 32 feature-argument call sites and the 7 file-path sites;
  `check-artifacts` checks its argument but skips the walk, so a linked
  sibling stays a skip (spec 067). The three session-file sites first kept the
  lexical screen, since replaced by the entry on session files opening through
  canonical containment, and later entries extend the check to the fixed-name
  project files, the spec-root listing and five more reads. A corpus walk
  reaching a checked call site, `prune-plan` and `prune-tasks` with `all` and
  `dashboard` through `resolve-references`, is refused as a whole when one
  spec directory holds an outward link. One hop budget, 40 links as the kernel
  counts them, is shared across a whole resolution, so a chain of links that
  each name the next twice is refused in well under a second rather than
  taking exponential time, and a name appended after a `..` past a missing
  directory is resolved when it exists. A malformed config is no longer warned
  about once per feature: it halts the call (the entry "A `.ductus/config.toml`
  or session file that does not parse, or an invalid `[paths] specs-root`,
  halts the call"). The `writeCode` boundary matches
  where each edit path lands, and its `OutOfBoundary` reason names a symlink
  taking it outside the boundary or the project, or one that cannot be
  resolved. A project that links a spec directory, or a file in one, to a
  target outside the project now sees `InvalidPath`. Spec 022, task 4;
  `fd7e1d4e`, `fd42d4e9`, `404db2ac`, `a680b2c1`.
- **BREAKING: eight more primitives check their path arguments, and the
  staging paths and `lint-markdown` globs narrow (spec 022, task 9;
  `BE-INPUT-004`).** `validate-frontmatter`, `resolve-anchor`,
  `check-rule-ids` and `check-promotion-coverage` take each path as
  project-relative through the containment check and refuse an absolute one,
  checking every path before reading any; `resolve-feature` checks the feature
  it matches, with the directory walk, before reading its status or a
  scenario. `fetch-archive`'s `destination` and `extract-archive`'s `archive`
  and `destination` must be absolute paths inside the system temp directory,
  the one `/ductus` makes with `mktemp -d -t`, with a link on them that leaves
  it refused; a later entry also refuses a `dest` that contains the working
  directory. The roots accepted are the temp directory as
  `std::env::temp_dir()` returns it or canonically, the directory `getconf
  DARWIN_USER_TEMP_DIR` names on macOS (read once, bounded by
  `GETCONF_TIMEOUT`, 5 s), and `/tmp` where `TMPDIR` is empty off macOS; the
  temp directory itself, and a root inside another, is refused. A staging path
  is matched to a root by its text before anything resolves it, so a path
  outside every root is never opened. `lint-markdown`'s `paths` must be
  project-relative, may hold no link leaving the project under a glob's
  literal prefix or a named directory, and refuse a leading `:`, `!` or `#`
  and any brace, parenthesis, `|` or backslash, which markdownlint-cli2 reads
  as syntax this check does not model. Every shipped caller already passes
  what these admit. Spec 022, task 9; `61ec6512`, `4ed8340d`, `fe4b949b`,
  `cdb679c9`, `478190a9`, `7767773f`, `9e7b7943`, `54068808`.
- **BREAKING: the runtime refuses to write an agent host's configuration (spec
  022, task 11).** The refusal holds on every surface (MCP, CLI and `ductus
  exec`) and for every agent, with no prompt. `merge-permissions` refuses an
  `allow` entry outside Claude's canonical set, which later entries shrink and
  extend with a link check. `apply-manifest`, `enforce-manifest` and
  `merge-managed-block` refuse a destination under `.claude/`, `.augment/`,
  `.agents/`, `.opencode/` or `.pi/`, or at `.mcp.json` or `opencode.json(c)`,
  matched at any depth; a later entry adds `.ductus/bin` to the refused
  directories. The exceptions are the rows a `/ductus` step writes: command
  and skill files, the `ductus` install file, the Antigravity rules mirror and
  the Pi bridge; `merge-managed-block` writes no row at all. `apply-manifest`
  and `enforce-manifest` judge every entry before the first write or removal,
  and `apply-manifest` writes the bytes it judged. A command, skill or prompt
  file it writes may carry only `description`, `argument-hint`, `parity` and
  `name` in its frontmatter. `migrate-session-file` takes only
  `<config-dir>/<name>-session.json`. A destination is judged where the write
  lands, so a link there is replaced and not followed through, and by the path
  as given; the names are Unicode case-folded, and Windows' trailing dots,
  spaces and stream suffix are dropped. The destination is judged from the
  root of the outermost git work tree, found by its `.git` entry, as well as
  from the project root, so running from inside `.claude/` or with
  `--target-root ..` reaches no host configuration; the names below
  `.claude/worktrees/<name>/` are judged afresh as a project's own. The
  worktree's own name is judged with the names below it, so a config-directory
  name there is refused. A link inside a config directory that points at a
  project file is left, and so is a destination outside the project, which
  `apply-manifest`'s roots still reach (spec 022, task 17); the plan states
  both residuals. Rows are matched at any depth, because a project may root a
  target in a subdirectory. Spec 022, task 11; `23710352`, `76836eac`,
  `bc4d9b06`, `03c84ab3`, `64290393`, `fc7dde88`, `6711de58`, `0ff220b7`,
  `9df128b6`.
- **Principle 5 of §runtime-boundary lets a primitive stay off the MCP surface
  (spec 022, task 19).** The constitution now says a primitive that, called
  without a prompt, would run a command the project supplies, write outside
  the project and its staging directory, or write a host's settings or command
  files MAY be left off MCP; it stays a `ductus <name>` subcommand and a
  `ductus exec` step. §runtime-host-integration names the exception: a step
  that names such a primitive runs it through the CLI. The amendment applies
  to every host and names no runtime identifier. Spec 022, task 19;
  `80a1ca97`.
- **BREAKING: the bootstrap bounds and verifies every fetch it makes (spec 061, task 24;
  spec 022, task 8).** The latest-release redirect read, the version pin, both
  migration-registry reads, the self-update fetch and the per-language
  `.gitignore` fetch pass `--proto '=https' --tlsv1.2` and `--connect-timeout
  {connect-timeout}` (10 s) and `--max-time {request-timeout}` (60 s),
  constants the bootstrap owns and defines in its Derived paths table. The
  runtime archive and its sidecar pass the same TLS flags and their own
  `{download-connect-timeout}` (30 s) and `{download-timeout}` (300 s), since
  the archives are 5.8 to 6.7 MB; a download that times out is a network
  failure. Pi's configure step fetches its bridge like `/ductus` and names the
  same two bounds. A halt for a tag below the floor, a tag below an applied
  migration, or a named tag that does not exist now names where the tag came
  from (`from --ref`, recorded as `[source] ref` in the active config file, or
  the default), and the halt for a failed latest-release read names curl's
  error when the request failed or timed out. A tag resolved from `latest` is
  "the latest release" however `latest` was given. Spec 061, task 24; spec
  022, tasks 6 and 8; `cc3d18aa`, `fdcdb129`, `db8fb327`, `17424d39`,
  `4ed8340d`.
  Breaking for a run that relied on a bootstrap fetch over plain `http`, a TLS
  version below 1.2 or no time bound: each now fails.
- **BREAKING: `install.sh` bounds both of its requests (spec 022, task 6; spec 061).**
  The latest-release header read and the bootstrap download take
  `--connect-timeout 10` (`CONNECT_TIMEOUT_SECONDS`) and `--max-time 60`
  (`REQUEST_TIMEOUT_SECONDS`), so a hung connection fails and the installer
  halts naming the URL (`BE-TIMEOUT-001`). The halt for a named tag below the
  floor says "from --ref". Spec 022, task 6; `a016b9d4`, `cc3d18aa`.
  Breaking for an `install.sh` run whose request hangs: it now fails after 10
  s to connect or 60 s in all, halting naming the URL.
- **The shipped CI template's `actions/checkout` pin moves a major release,
  v4.4.0 to v7.0.1 (spec 022, task 7).**
  `framework/templates/ci/adopter-generators.yml` pins commit
  `3d3c42e5aac5ba805825da76410c181273ba90b1`. None of the breaking changes in
  the release notes touches how the template uses the action. Spec 022, task
  7; `b58db0b8`.
- **Lockfile refresh**: `rustls-webpki` 0.103.15 to 0.103.16
  (RUSTSEC-2026-0382, certificate verification could accept distrusted
  intermediates) and `webpki-root-certs` 1.0.9 to 1.0.10 (RUSTSEC-2026-0383,
  bundled roots trusted distrusted emSign intermediates). `d58fd275`.

- **BREAKING: `apply-manifest` bounds `target-root` and `source-root` by the
  working directory (spec 022, task 17 (a), `ff35f4ca`).** The roots were
  absolute-capable bases, so an auto-approved tool could write outside the
  project with no host prompt. `target-root` must lie inside the working
  directory; `source-root` inside it or strictly inside a staging directory.
  An absolute root outside them, a `..` root and a root reached through a link
  that leaves them are refused before any entry is examined. It narrows what
  the primitive accepts, so it is breaking under `BE-COMPAT-001`. It bounds the
  primitive, not the agent: `/ductus` calls it with `target-root .`, and the
  agent's own file tools and the read-only cross-service primitives reach other
  projects as before. `/ductus` State B passes `--source-root
  {framework-root}`.
- **BREAKING: `fetch-archive` and `extract-archive` write only to a staging
  path outside the working directory (spec 022, task 17 question 2 (a);
  `b2719007`, `86635000`).** Both bounded their write destination to a staging
  root alone, ignoring the project, so a project lying inside a staging root (a
  checkout under `/tmp` on Linux, a `TMPDIR` override, or Windows with `TMP`
  and `TEMP` both unset) had its files written with no prompt; `extract-archive`
  wrote a `.claude/settings.local.json` granting every shell command into such
  a project. `extract-archive`'s `dest` and `fetch-archive`'s output archive must
  now lie strictly inside a staging root and outside the working directory;
  `extract-archive`'s input archive keeps the staging bound alone. `/ductus`'s
  `{tempdir}` is a fresh directory outside the project and passes. When
  `TMPDIR` points inside the working tree, `{tempdir}` lands inside the project
  and extraction is refused, so `/ductus` cannot bootstrap there. Later
  extended: both also refuse a `dest` that contains the working directory (the
  entry for spec 022 task 43, on `merge-permissions` and an archive's staging
  path).
- **BREAKING: every writer that takes a caller-supplied destination refuses
  Claude Code's protected set (spec 022, task 20 (a); `e8208aa2`,
  `e0262086`).** The destination check that judged the agent config
  directories now also refuses the protected directories (`.git`,
  `.config/git`, `.vscode`, `.idea`, `.husky`, `.cargo`, `.devcontainer`,
  `.yarn`, `.mvn`) and the protected files (shell rc files, `.gitconfig`,
  `.npmrc`, `.pre-commit-config.yaml`, `lefthook.*`, `.claude.json`, and the
  rest of the host's list), outright, since no `/ductus` row writes any of
  them. `apply-manifest`, `enforce-manifest` and `merge-managed-block` judge it
  through their destination names; `append-task` and `create-scenario` judge
  it on their feature path, which also closes the `.claude/rules` route; and
  `merge-permissions` judges the protected set and the MCP and settings files,
  not the config-directory rows, since it writes `settings.local.json` under a
  config directory. A protected component above the working directory, as when
  the working directory is itself `.cargo`, is refused, and so is a write
  through a root `.mcp.json` link. `.githooks` is not in the set. Later
  extended: the same writers refuse `.ductus/bin` (the entry "Every
  destination-judging writer refuses the runtime pointer's directory").
- **`/ductus` seeds only the commands each bootstrap step runs (spec 022, task
  18 question 1 (a) and 1a (a); `df474a1f`, `4a0e047d`, `f24c55c6`,
  `cd06a227`, `047c6e9b`).** Each seed entry whose command can run another
  command or write outside the project becomes the exact command a step spells,
  with `*` only for an operand that differs per run, a `.exe` twin for each
  Windows path, and one CLI rule per subcommand a step runs (the eleven of the
  bootstrap and its migrations, and `compute-review-scope`), so
  `run-generator`, `exec` and the rest of the CLI prompt. `git diff` is
  dropped from the seed (no step runs it) and Claude's three single-slash
  `Read` rules go. Claude goes from 20 shell entries to 46 and from 6 `Read`
  rules to 3, with 17 entries retired; the exact `cp` and `ln` rules for the
  runtime pointer are dropped again, because Claude Code strips a bare `xargs`
  before matching and an exact rule then runs with appended operands (any
  readable file copied into any directory, or `.ductus/bin/ductus` relinked to
  any file). The pointer's link therefore prompts on a run that acquires or
  repairs it. `/ductus`'s §Permission Setup gains a step that removes each
  agent's retired seed entries (allow grants only), so a project on
  `--ref=main` that ran `/ductus` since `df474a1f` is cleaned. Pi has no seed
  and is unchanged; Auggie, Antigravity and OpenCode, supported at this point,
  were narrowed the same way (support since dropped, see "Auggie, Antigravity
  and OpenCode are no longer supported agents") and `install.sh`'s four copies
  were regenerated from the one list. Later narrowed: the Claude seed no
  longer grants `Bash(curl *)` or `Bash(tar *)` (the entry "`/configure` no
  longer grants thirteen Claude allow entries").
- **A command's markdown-only reference moves into its own file, and the
  thirteen command files shrink from 430,896 to 191,489 bytes (spec 071, tasks
  2-16).** Each of the thirteen commands' reference half is now
  `framework/reference/<command>.md`, installed once at
  `.ductus/reference/<command>.md` and shared by every agent, and read when a
  step names a section there; the command keeps its Instructions and gains a
  location line, and each reference opens with a paragraph saying where a named
  section lives and that a bare `§name` is a heading of the file or a
  constitution marker. The maintainer-only `audit` command's reference is
  withheld with the command. `/ductus` installs twelve new Shared Files rows
  and enforces `.ductus/reference/` like a command directory, removing a stale
  reference file; `analyze` drops from 111,742 bytes to 38,862. The sizes sum
  the thirteen files under `framework/commands/` that gained a reference
  (`audit` included), measured with `git show <sha>:<path> | wc -c` at
  `b772a6ac~1` and `b772a6ac`; the twelve an adopter receives go from 393,629
  to 186,282. The constitution's canonical-sources row names
  `framework/reference/*.md` beside the command source.
  `check-corpus-links --scope repository` excludes the generated
  `.ductus/reference/` copies only in a tree that also carries
  `framework/reference/`, where the sources are checked; in an adopter they are
  checked as the installed files they are. Commits `c91391ad`, `b772a6ac`,
  `39c7c9e2`, `85b2e1a5`. The references were cut again later (the entry "A
  command's reference holds what a step does with a result, and no more").
- **The bootstrap's installer defects from a scratch adoption are fixed (spec
  071, tasks 20 and 21; `b0b1f9d0`, `083c4b08`, `117413d6`, `d9ac40a3`).**
  The constitutions region in the `AGENTS.md` and `CLAUDE.md` templates uses
  `BEGIN` and `END` markers, as `merge-managed-block` does; both files map to
  `create`, so their placeholders are substituted; `/ductus` makes one
  `apply-manifest` call per selected agent, each with that agent's own
  `cli-config-dir`; `enforce-manifest`'s `expected` and `pinned` lists are
  paths relative to the enforced directory; Pi's bridge is written after the
  framework archive is extracted; a greenfield run (no config file at any tier
  and no spec-root directory at its start) seeds `[migrations] last_applied`
  with the newest active entry instead of prompting for every no-op migration;
  the managed `.gitignore` block is a fixed point on its first run, and the
  adopter-area copies of its patterns are dropped; the closing restart notice
  says only what the run did (four variants, "Files written" listing only what
  it wrote); and the self-install's integrity check reads the first non-blank
  line. Step 7 records `cli-config-dir` as the config directory of the agent
  running the bootstrap; step 9's empty substitutions map replaces a
  keep-literals list it never passed. The `inbox-guidance-refresh` migration
  now stops silently when no inbox exists. The Antigravity rules-mirror call
  this round added went with that agent's removal (see "Auggie, Antigravity and
  OpenCode are no longer supported agents").
- **BREAKING: `derive-dependencies` and `derive-references` keep a key's whole block when
  they splice it, and never write a splice that does not parse (spec 067,
  review round 1; `2be18da6`).** A key's continuation counted only indented
  lines, so an indentless block sequence (`- item` at column 0, the default of
  PyYAML and Psych) survived the splice behind the new value as invalid
  frontmatter, reported `updated`, exit 0; a blank line or a column-0 comment
  inside the list ended the skip the same way. A continuation now also holds
  dash-led (`-`) entries and blank lines and comments that an entry follows.
  `derive-references` inserted `references:` between a block-form
  `dependencies:` key and its items, turning a dependency into a reference; it
  now inserts after the key's last continuation line. Both primitives re-parse
  the frontmatter they are about to write; one that fails is named in
  `unparseable` and left byte-identical, on a dry run and a write alike. Later
  changed: such a refused rewrite is named in `unwritable`, and the CLIs exit
  non-zero on it (the entry "`derive-dependencies` and `derive-references`
  name a rewrite they refuse").
  Breaking under `BE-COMPAT-001`: a splice whose result does not parse,
  written in 0.59.11, is now refused, the spec named in `unwritable`, and the CLIs exit non-zero.
- **BREAKING: `check-promotion-coverage` refuses an empty `sections` (spec 067,
  review round 1, `2be18da6`).** With none, every count was zero by
  construction, not a measurement. The CLI, the MCP tool and `ductus exec` now
  return a missing-argument error asking for at least one level-2 section
  (`--section` on the CLI).
- **BREAKING: `resolve-anchor` judges a reference on a line naming another
  document, which can now be unresolved (spec 067, task 19, scenario
  `shared-constitution-references-are-judged`; `8651d466`).** A reference on a
  line naming a readable document is judged against the documents the line
  names, a registered shared constitution's markers, the citing file's
  headings and the constitution's markers. One it cannot judge carries
  `unjudged`, `anchor-target-unjudgeable` or `anchor-target-unreadable`,
  counted beside the kinds. The markers file is recognized by its resolved
  path, never a basename, a malformed `[constitutions.*]` registry is an
  error, and a code span holding a path and one reference is a reference. The
  result's shape change is additive; the meaning change is breaking under
  `BE-COMPAT-001`. Family 39 now covers every spec's `spec.md`, `plan.md` and
  scenarios beside `framework/`, and fails on an unresolved judged reference
  or one naming an unreadable document.
- **BREAKING: `resolve-anchor` resolves a heading only where a word ends, and
  not inside a fenced block or an HTML comment (spec 067, scenario
  `pointers-into-shipped-files-are-judged`; spec 071, `36b3ddb6`,
  `083c4b08`).** A bare heading reference must end where a word ends
  (`§Groundingx` names no `Grounding`, `§Hooks` names no `Hook`), and headings
  are collected outside ` ``` ` fences and HTML comments, so a `## Summary`
  inside a fenced example names no section. This applies to a reference
  against the citing file's own headings as well as to a pointer into another
  document, so a reference that resolved by prefix before is now reported
  unresolved. It narrows what resolves, so it is breaking under
  `BE-COMPAT-001`. Measured over all 823 tracked `.md` files at the time, no
  reference changed kind or resolution.
- **The constitution is cut from 119,262 to 49,980 bytes and is imported at
  startup (spec 050, tasks 35-36, scenario `the-constitution-fits-one-read`;
  `05359e82`, `7ce72592`, `d5de121c`, `da52bc4c`).** Sizes are `git show
  <sha>:framework/constitution.md | wc -c` of the shipped source, before
  `{project}` is substituted: 119,262 at `05359e82~1`, 53,813 at `05359e82`,
  49,989 at `7ce72592`, and 49,980 at `1503ca58` after the review rounds (at
  `ductus-v0.59.11` it was 116,564, the result the 0.59.11 entry "The
  constitution is a fifth shorter" records; it grew to 119,262 before this cut,
  so the two do not overlap). The single-command rules move to the command
  references that apply them: the review window and coverage rules, with the
  definition of `scope`, to `reference/review.md` §Window and coverage, the
  analyze record's field meanings and the in-progress criteria-path
  measurement to `reference/analyze.md`, and removal by text to
  `reference/groom.md` §Inbox upkeep; the disposition task's shape is left to
  `reference/implement.md`, which the constitution now points at. Every other
  section is compressed to its rule and one reason. All 31 anchor markers, 68
  headings and the RFC 2119 keyword counts of the 53,811-byte file are kept.
  The cut fits one read on a host whose read tool caps one read at 51,200 bytes
  (OpenCode's and Pi's stopped after line 500 at 53,811; OpenCode support is
  since dropped); an installed copy stays within 50,000 bytes for a project name of up to 42 characters (29 `{project}` placeholders at 49,021 bytes, so 29 bytes per name character beyond nine). `/target` step 4's read of it now fits one call on those
  hosts. The file measures 49,021 bytes at `a7a7f898` (`git show a7a7f898:framework/constitution.md | wc -c`), 48,763 at `8e4dc677`, and 49,021 at `ffb3db9b`.
- **A plan or task-list edit that adds no unchecked task does not reopen a
  `done` spec, and does not stale its analysis (spec 041, scenario
  `a-plan-edit-does-not-reopen-a-done-spec`, task 21; specs 047, 063, 066;
  `e3e54b07`, `fbf2737c`).** `prune-plan`'s `reopen-required` is decided by
  the `tasks.md` trigger alone (an added unchecked box), so a changed plan
  design-record section no longer returns `true`; analyze freshness on a
  `done` spec ignores an in-place edit to `plan.md` or `tasks.md`, while a
  deleted one still stales the record, and an in-progress spec's plan edit
  still stales its record; and `check-artifact-size`'s warning for an
  oversized `plan.md` on a `done` spec says a trim does not reopen it. The
  constitution's §spec-lifecycle states it beside the three mechanical cases,
  since an edit adding an unchecked task still reopens. It loosens what
  reopens and stales, so it is not breaking. Generalized later by
  `classify-spec-edit`, which answers `correction` for an edit to `spec.md`,
  `plan.md` or `tasks.md` that changes no criterion, open question or
  unchecked task.
- **BREAKING: only gating findings hold `done`, and disposition drift is
  advisory (spec 058, `findings-cost-in-proportion-to-risk`, task 1; spec 066,
  `only-gating-findings-hold-done`, task 6; spec 067,
  `disposition-drift-is-advisory`, task 16).** `check-review-gate`'s last check
  sums the review record's unwaived MUST and SHOULD violations and the
  analysis's hard-fail and blocking findings, never `dispositions`'
  `undispositioned`, so an undecided advisory finding or review observation
  stays recorded and no longer holds `done`; an unwaived SHOULD now does,
  which the gate did not read before. Its message reads "outstanding gating
  finding(s)" where it read "undispositioned finding(s)" and asks to fix the
  finding or waive a review SHOULD with `/review --waive <rule-id> --reason`.
  The `blocked-by` wire value stays `undispositioned-findings`.
  `check-artifacts`' disposition-drift family is Advisory and `/analyze --fix`
  no longer reverts a `done` spec on it (its triggers are review-state drift
  and unresolved scenario questions). A non-blocking finding met while working
  a disposition task is recorded on that task, not appended as a new one. A
  waiver that expires re-counts toward `should-violations` as well as
  `must-violations`. The constitution's §implement-phase says the gate blocks
  on an unwaived SHOULD, and §brownfield-inbox says a gating finding must have
  a disposition. It changes which records block, so it is breaking under
  `BE-COMPAT-001`. Commits `e38f5c1f`, `84bb4aa1`, `6cf2bb6e`, `98a2eae0`.
  Later reworked: the `routed` and `discarded` dispositions became `deferred`
  and `waived`, and a deferral of a gating finding releases `done` only when
  its work is still to do on another spec (the entry "A finding is fixed,
  deferred or waived").
- **Lockfile refresh**: `rmcp` 3.5.1 to 3.7.0, `tokio` 1.53.3 to 1.54.1, `zip`
  9.0.4 to 9.1.2, `typed-path` 0.12.3 to 0.13.0, `cc` 1.8.0 to 1.10.0,
  `pulldown-cmark` 0.13.4 to 0.13.5 and `uuid` 1.28.0 to 1.29.0, with 16
  other packages moved within their declared ranges (23 in all, resolved to
  the highest Rust 1.89-compatible versions). No manifest change.

- **BREAKING: `rewrite-spec-links` re-points the path references that
  `check-orphaned-references` reads, and reports what it did not rewrite
  (specs 051 tasks 33-35 and 052 tasks 2 and 3).**
  Besides the body links across the spec root, a fold or a consolidation now
  re-points, with the same referrer list and token scanner as
  `check-orphaned-references`, each path reference to the retiring feature in
  the spec root's `system.md` and in `CLAUDE.md`, `AGENTS.md`, `README.md`
  and `.githooks/pre-commit`, so none is left dangling in those files. The
  result gains three lists: `skipped`, naming a referrer it did not read (a
  symbolic link, a parent resolving outside the project, or text that is not
  UTF-8) and a symbolic link under the spec root named `*.md` or pointing to
  a directory, each left as written; `kept-tails`, naming a re-pointed
  reference whose kept tail names a file the fold did not write, or a
  `scenarios/{name}` tail the retiring directory does not hold; and
  `mentions`, naming each line of an examined file that still names the
  retiring directory by name or number once the rewrite ran. The whole spec
  root is read before the first write: a markdown file there that is not
  UTF-8 text, cannot be read or cannot be listed stops the run before
  anything is written, and a spec root that is itself a link is followed. On
  a scenario target, a reference to one of the retiring spec's scenarios
  lands on the scenario the fold carried under that slug, fragment kept. A
  link inside the target that climbs to the spec root and into the retiring
  directory is written relative to its file, so the consolidation commit no
  longer records a self-link as a dependency cycle. Overlapping spans no
  longer panic. `/{project}:fold` step 7 now stops before step 8 when a
  carried scenario's slug is taken by a file that is not that scenario, where
  it used to report it and go on, and `/{project}:consolidate` states that
  the content move and the consolidation commit together. Breaking under
  `BE-COMPAT-001`: new files written and new refusals; the three result
  fields are additive. Later entries add the `refused` list and the write
  only when a file is unchanged since it was read (spec 051, task 37), walk a
  scenario target's feature directory and hold `from` to the feature-name
  rule, and list no scenario tail in `kept-tails` once the retiring directory
  is gone. Specs 051, tasks 33, 34 and 35, and 052, tasks 2 and 3.
  `3b359742`, `f056b209`, `47e68233`, `77fff147`, `c1ca45b5`, `2741fb12`,
  `f395900f`, `ced8c7ca`, `edd08387`, `ea859f7c`, `3297fbd6`
- **BREAKING: `rewrite-spec-links` holds its `to` to the name grammar (spec
  051, task 32).**
  `rewrite-spec-links` writes `to` into every referrer, the executable
  `.githooks/pre-commit` among them, but held it only to no traversal,
  containment and two non-empty halves. Both halves now pass
  `validate_session_feature` and `validate_session_scenario`, the checks
  `retarget-sessions` applies to the same `{feature}/{scenario}`, before any
  file is read or written. A feature half naming `templates`, and a scenario
  half with a space, `;` or `$(...)`, are refused with the corpus unchanged;
  a branch-scoped feature and a slug are admitted. A later entry holds `from`
  to the feature-name rule too, and `retire-feature` applies the same screen
  to its `feature`. Spec 051, task 32. `2f79a9ee`
- **`check-orphaned-references` and `rewrite-spec-links` read a path behind
  a prefix naming the project root (spec 051, task 30; spec 067).**
  A path spelled with a prefix that names the project root from its referrer
  (`./` and a leading `/` in a root markdown file, `./` in
  `.githooks/pre-commit`, `../` and a leading `/` in the spec root's
  `system.md`) is a reference wherever the same path written bare is one,
  and the existence check sees it as if written bare while the rewrite keeps
  the prefix as written. `./` and `../` take the bare rule exactly. The one
  exception is a leading `/` that ends another directory's name: after `~`,
  `}`, `%` or `)` ending a home or variable directory, or after a quote or
  backtick closing a variable, so `~/.ductus/bin/ductus` and
  `"$HOME"/.ductus/x` stay outside it. A leading `/` is read from the
  project root even in a project below its repository's root, as
  `check-corpus-links` reads a root-absolute link. Spec 051, task 30.
  `38fc39f6`, `54715126`, `3fc5755a`, `d1853444`, `c1cf51ff`, `1f2e7b98`
- **BREAKING: installed files lose group and other write, and
  `apply-manifest` names a mode it could not set (spec 022, task 23).**
  GitHub's tarballs record 0664 and 0775, and `apply-manifest` mirrors the
  extracted mode, so an adopter's installed files, the pre-commit hook among
  them, were group-writable. `extract-archive` now masks every extracted
  mode with 0o755, which drops group and other write where the 0o777 mask
  kept them. `apply-manifest` also re-applies the source mode to an
  `update`-strategy file whose bytes already match when only its mode
  differs (a chmod alone: bytes, mtime and the `unchanged` action are kept,
  and a link is not chmodded through), so an installed file loses group write
  on the next run. A chmod the file system refuses no longer stops the walk:
  the file keeps its mode and is named in the result's new `modes-kept`
  (omitted when empty), and the bootstrap's completion message lists it.
  Breaking under `BE-COMPAT-001`: the modes of extracted and installed files
  change, and an unchanged file's mode changes on a run that reports it
  `unchanged`; `modes-kept` is additive. Spec 022, task 23. `f056b209`,
  `47e68233`, `77fff147`, `c399913e`
- **The session-lock poll backs off with jitter and stops at an attempt
  bound (spec 062, task 28; `BE-RETRY-001`, `BE-RETRY-003`).**
  A process waiting for the session lock polled every 10 ms. It now waits
  from 1 ms, doubling to a 50 ms cap, each wait drawn uniformly up to its
  bound, still stopped by `SESSION_LOCK_TIMEOUT` and now also by
  `LOCK_MAX_ATTEMPTS` (1,000 attempts), and retries only `WouldBlock`.
  Spec 062, task 28. `47e68233`, `77fff147`
- **`fetch-archive` identifies itself by the repository URL (spec 048, task
  19).**
  Every request, the archive, its sidecar and each redirect hop, sends
  `ductus <version> (https://github.com/stonean/ductus)` as its
  `User-Agent`, where `reqwest` sent none. Spec 048, task 19. `4bbb86f7`
- **BREAKING: `fetch-archive` refuses a proxy value it cannot use, and reads
  its certificate paths once at startup (spec 048; `CFG-ENV-001`,
  `CFG-ENV-008`).**
  Each proxy variable is parsed when the process starts. A value that does
  not parse, or names a scheme other than `http` or `https` (a `socks5` URL,
  which `reqwest` accepted at build and refused only at connect, naming the
  scheme rather than the variable), now refuses every fetch, naming the
  variable with its credentials shown as `***`, before anything is resolved
  or sent; skipping it would have sent that scheme's fetches direct, past the
  operator's proxy. On Linux and other non-Apple, non-Android Unix,
  `SSL_CERT_FILE` and `SSL_CERT_DIR` are captured at startup too, and the
  first fetch loads the roots from those paths, or from the platform store
  when neither is set. A later entry adds a test that fails when the capture
  is removed, and another redacts a proxy value whose credentials hold `://`
  whole. Spec 048. `d4b75e66`
- **BREAKING: `/{project}` bounds every `--version` probe, accepts only a well-formed
  pin, and the shipped pre-commit hook echoes a sanitized version (spec
  048).**
  The bootstrap runs each `--version` probe, of the store or of a `[runtime]
  path`, with the host shell tool's timeout set to `{probe-timeout}` (10 s);
  a probe still running at the limit did not execute and takes that branch's
  failure path. It halts on any `{pin}` but one `MAJOR.MINOR.PATCH` line in
  ASCII digits, since later steps interpolate the pin into URLs and file
  names. The shipped pre-commit hook echoes the binary's first `--version`
  line cut to 80 printable characters. Spec 048. `d4b75e66`
  Breaking for a run whose pin is not one `MAJOR.MINOR.PATCH` line of ASCII
  digits, or whose `--version` probe of a `[runtime] path` binary, or a re-probe, outlasts 10 s: it now halts (a store probe that outlasts it goes to State B or re-acquisition).
- **BREAKING: `apply-manifest`'s and `enforce-manifest`'s `--*-json` files
  must lie inside a staging directory (spec 048; `BE-INPUT-004`).**
  The CLI read a `--entries-json`, `--pinned-json`, `--substitutions-json` or
  `--expected-json` file from any path. It now refuses a path outside the
  system temp directory, naming the flag, before opening it, as
  `extract-archive` does for an archive; `/{project}` writes the files into
  its `{tempdir}`. A later entry holds the `--*-json` flags it adds to the
  same staging directory (spec 022, "Every MCP argument has a CLI form").
  Spec 048. `d4b75e66`
- **`/{project}:plan` and `/{project}:implement` run `derive-dependencies` on
  entry, and `/{project}:plan` checks cross-spec impact before its gate (spec
  017).**
  Each command's sub-step 1.1 invokes `derive-dependencies` as the report-only
  safety net, as the commands did before spec 022's rewrites lost it, so
  drift from uncommitted body edits surfaces before work starts. `/plan`'s
  step 9 now runs the reference's Cross-spec impact check and presents its
  result in the `gate-confirm` prompt. Spec 017. `3b359742`
- **`/{project}` writes a `ductus:memory-routing` block into `AGENTS.md`, and
  the `CLAUDE.md` template no longer carries Auto-Memory Routing (spec 050,
  scenario `memory-routing-lives-in-agents-md`; spec 075, task 2).**
  On every run, under every layout, `merge-managed-block` installs or updates
  an `html-comment`-marked block in `AGENTS.md` saying where a learning goes,
  per the constitution's §drift-prevention; a pinned `AGENTS.md` still gets
  it. The `AGENTS.md` template ships the empty markers beneath its header,
  and the `CLAUDE.md` template drops its Auto-Memory Routing section. The
  block files a learning true for the project alone in `AGENTS.md` only when
  every session needs it before acting, and otherwise where its work is read:
  architecture in `system.md`, `events.md` or `errors.md`, a feature's
  decision in its spec, a procedure in `docs/` or a skill, a one-file trap in
  a comment or test there, how a lesson was learned in its commit. New
  migration `memory-routing-leaves-claude-md` (0.60.0) removes a shipped,
  unchanged Auto-Memory Routing section from an existing `CLAUDE.md` and
  leaves an edited one with a report; it ends the section at the next heading
  or framework `<!-- BEGIN ductus:` block, so a block appended after the
  section is kept. Specs 050 and 075. `3b359742`, `fb26d602`, `cb30b333`
- **The inbox template carries no guidance comment and no placeholder item,
  and migration `strip-inbox-guidance` (0.60.0) removes both from existing
  inboxes (spec 058, task 2; spec 006).**
  The template's `<!-- Rules: -->` comment restated constitution
  §brownfield-inbox, §finding-dispositions and §bug-handling, and its
  `- [ ] {Brief description ...}` placeholder sat outside it, so every inbox
  primitive counted it as an outstanding item (a scratch inbox reported
  `item-count` 2 after one logged item). The template is now its heading and
  introduction; the one rule stated nowhere else, that an inbox item is a
  todo and never a status or handoff note, moves into §brownfield-inbox.
  `inbox-guidance-refresh` (0.53.0) removes the comment when the template
  carries none, where it replaced it. `strip-inbox-guidance` removes the
  first `<!-- Rules:` comment and each line outside every fence and HTML
  comment that is the placeholder, with or without its checkbox, skips a
  pinned inbox with a warning, keeps line endings and the file's mode, and
  leaves every other item byte-identical; its id sorts after
  `remove-legacy-constitutions-region`. Spec 058, task 2; spec 006.
  `bca9a2fd`, `797799ba`, `89e6f02b`, `09b2a408`, `3f8cc104`
- **The Frontmatter Migration's precheck judges the tree as the run found
  it, and its refusal stops only that section (spec 013, task 25).**
  `/{project}`'s Pre-run Migrations take `git status --porcelain -- specs/`
  before step 1 writes anything, and the precheck judges that pre-run status,
  so a file `inbox-guidance-refresh` or `criterion-label-backfill` wrote no
  longer refuses the run, while a file the adopter had changed still does. A
  refusal names the dirty paths, writes nothing and skips to the next
  section; the rest of the run continues, so Shared Files still replaces a
  pre-upgrade constitution copy and a pre-0.30 hook. Post-Scaffolding Output
  lists each path a migration made dirty under `specs/` and repeats a
  refusal. Spec 013, task 25. `9d79988e`
- **The constitution states succinctness, routes a project-only learning by
  population, and applies an answer as chosen (specs 050 and 075).**
  §text-first-artifacts gains the bullet that each artifact is written as
  short as its required content allows, defining required content by
  §grounding, §design-principles and §drift-prevention. The Guiding
  Principles' Technology list gained "Succinct: artifacts hold all their
  readers need, and no more", and a later change (spec 050, task 50) removed
  that list with the Technology and Business lists, so the shipped
  constitution carries the bullet alone. §drift-prevention's *Shared
  knowledge stays in git* now sends a learning true for the project alone to
  its own artifacts (`AGENTS.md` only for what an agent needs before acting
  in any session, that nothing its work reads would say and no check, test
  or hook reports), and §spec-phase gives `system.md`, `events.md` and
  `errors.md` one-line definitions. §recommendations adds that an answer is
  applied as the option reads, a refinement the sources suggest being asked
  as its own question first. To stay within the 50,000-byte governing size,
  restatements were cut: the opening tagline, Grounding's lead slogan, the
  default spec-root clause, Bug Handling's and Brownfield Process's lead
  sentences, the acquisition clause and the eligibility restatement in the
  runtime boundary, the recorded-backfill sentence, the frontmatter list of
  other artifacts, the Readiness Check's closing instruction, and the
  disposition-task bullet down to its pointer. The sentence naming the
  checks the completion gate composes with was cut, then restored because
  spec 046 quotes it. Specs 050 (tasks 41, 43 and 44) and 075. `4e82686d`,
  `f9ea53dd`, `73f21fc9`, `3b359742`, `54715126`
- **Rules are as short as their required content (spec 008, tasks 19-21; spec
  050).**
  The rule schema drops Rationale: a rule is an ID, a Statement and a
  Verification, with an optional Source and `DEPRECATED` label, and a
  Rationale paragraph in a pinned or project-authored file is ignored, never
  malformed. `/{project}:analyze`'s integrity check, the bootstrap's adoption
  audit, constitution §rules and `docs/rules.md` say so. All eleven files
  under `framework/rules/` were cut from 509,421 to 324,390 bytes, then to
  319,306 in a second pass (`wc -c`, summed): Rationale removed, Source cut
  to citations, headers cut to scope, ID format, categories and a pointer to
  §rules. A boundary a Rationale stated moved into the Verification, and a
  case only a Rationale stated became a Statement clause of the rule's tier,
  or a MAY for an exemption (`BE-LRO-001`, `BE-RETRY-003`, `BE-BULK-002`,
  `BE-CACHE-002`, `BE-QUERY-004`, `BE-COORD-002`, `FE-LOAD-008`,
  `FE-ARIA-004`, `FE-PII-004`). Overlaps now have one owner each: a secret
  that reaches a client artifact is `CFG-ENV-010`'s wherever it was written;
  a secret at rest in source, a build manifest, version control or an image
  is `CFG-ENV-013`'s (the entry above), where the first pass had left an
  image-baked one with `BE-DATA-008`; `CFG-ENV-006` owns a sensitive
  configuration value in any error message or operator-facing output;
  `BE-DATA-001` owns a database connection's encryption and `BE-API-007`
  every public exposure, database ports included; five backend/frontend
  pairs (a redirect, a sensitive response's `Cache-Control`, a frontend
  build's dependencies, a CSRF defense, and a credential's carrier) name
  which rule owns which case, with a load-conditioned yield where the other
  file is also loaded; and the OAuth `redirect_uri` rules, the error-envelope
  rules (`BE-ERRENV-001` to `-003`, `BE-ERR-002`, `BE-ERR-004`) and the
  credential-carrier rules (`BE-API-013`, `BE-API-015`) do the same. The
  rule files measured 332,393 bytes at `554d0ee3` (`wc -c`, summed), after
  the later rule additions. Spec 008, tasks 19-21, 23-25, 28 and 30-33; specs
  034, 037, 038, 039, 068, 069 and 070. `70bb4af5`, `734b427c`, `28167f78`,
  `74f6361c`, `194e43f7`, `9b067399`, `afb8a865`
- **Project templates point at the constitution (spec 075, task 3).**
  The `AGENTS.md` template's header points at the test in §drift-prevention
  without restating it and no longer says to append a project-only rule to
  its matching section, and its Project Structure replaces the `specs/`
  subtree with a pointer to §spec-phase. The `system.md`, `events.md` and
  `errors.md` templates say the §spec-phase directory tree defines what the
  file holds. New adoptions only: `AGENTS.md` installs with the skip strategy
  and the other three with `create`. Spec 075, task 3. `b763977c`
- **BREAKING: `resolve-constitutions` measures each loaded shared
  constitution, halts on a malformed read size, and takes `unmeasured` for
  `/{project}` (spec 055, tasks 14 and 15; spec 075).**
  Each loaded record gains `bytes` and `over-limit` against `[artifacts]
  read-size-bytes`, plus a `warning` naming the alias, the size, the limit
  and a cut in its own repository as the fix; a document at the limit is not
  over it. A document that cannot be opened gets no size, and a skipped
  record is never measured. The result carries `threshold` only when an entry
  is registered, so a project with no registry reads no new key.
  `/{project}:target` step 4 shows each loaded alias's size and warning,
  never blocks on one, reports a skipped entry as not measured, and halts on
  a malformed read size once an entry is registered, as the primitive now
  does. The new `unmeasured` argument (`--unmeasured` on the CLI) resolves
  without reading that setting and carries no size or threshold;
  `/{project}`'s Shared constitution imports step passes it, so a malformed
  read size cannot halt the install and recovery path over a setting it does
  not use. `write-review` and `write-analysis` stay unmeasured. Breaking
  under `BE-COMPAT-001`: a call that returned a result with a malformed read
  size and a registered entry now fails; the new fields and argument are
  additive. Spec 055, tasks 14 and 15. `c6a0dca2`, `e8ed5d62`
- **Lockfile refresh: `aws-lc-rs` 1.18.2 to 1.19.0, `aws-lc-sys` 0.46.0 to
  0.47.0, `rmcp` and `rmcp-macros` 3.7.0 to 3.8.0.**
  Eleven lockfile entries change, within their declared ranges and resolved
  to the highest Rust 1.89-compatible versions: ten packages move, among
  them `openssl-probe`, `rustix`, `unicode-width`, `utf8_iter`, `xattr` and
  `zip`, and `utf_types` 1.0.0 is added. No manifest change. `99c704fd`
- **Lockfile, net from 0.59.11 to the release head: 66 packages change
  version, 11 are added and none is removed (the lockfile's 257 package
  entries become 268), and the four advisories it took are named.**
  Counted as the `[[package]]` name and version pairs of `runtime/Cargo.lock`
  at `ductus-v0.59.11` against `3441e4cb` (`git diff ductus-v0.59.11 HEAD --
  runtime/Cargo.lock`); the lockfile entries above record the steps and their
  intermediate versions, and these are the net ones. The crate's own
  dependencies move: `rmcp` 3.5.1 to 3.11.0, `zip` 9.0.4 to 9.8.1, `tokio`
  1.53.3 to 1.54.1, `toml` 1.1.8 to 1.2.1, `clap` 4.6.7 to 4.6.9, `flate2`
  1.1.10 to 1.1.11, `regex` 1.13.1 to 1.13.2 and `pulldown-cmark` 0.13.4 to
  0.13.6. Among the rest: `rustls-webpki` 0.103.15 to 0.103.17,
  `webpki-root-certs` 1.0.9 to 1.0.12, `aws-lc-rs` 1.18.2 to 1.19.0,
  `aws-lc-sys` 0.46.0 to 0.47.0, `libz-sys` 1.1.30 to 1.1.33, `cc` 1.8.0 to
  1.11.3, `nix` 0.31.3 to 0.32.0, `typed-path` 0.12.3 to 0.13.0, `uuid` 1.28.0
  to 1.29.3, `rmcp-macros` 3.5.1 to 3.11.0 and the `icu_*` crates 2.3 to 2.4.
  The 11 added crates are `constant_time_eq`, `lock_api`, `parking_lot`,
  `parking_lot_core`, `redox_syscall`, `scopeguard`, `temp-env`,
  `unicode-general-category`, `unicode-normalization`, `utf_types` and
  `zeroize_derive`. The one dependency change in the manifest is the dev-dependency `temp-env`
  0.3.6 (the `fetch-archive` startup-capture test's entry). Four advisories are
  addressed: RUSTSEC-2026-0382 and RUSTSEC-2026-0383 (`rustls-webpki` and
  `webpki-root-certs`, the first entry above), RUSTSEC-2026-0418 (`libz-sys`,
  under Fixed) and RUSTSEC-2026-0433, published 2026-10-28: `pulldown-cmark`
  0.13.5 parses nested image wikilinks in quadratic time when
  `ENABLE_WIKILINKS` is set, and 0.13.6 fixes it. The runtime enables only
  `ENABLE_TABLES`, so the path was not reachable here, but `cargo audit`
  fails on 0.13.5, which the scheduled runtime-currency run found.
  `750fde09`
- **`security-backend.md` and `security-frontend.md` flag what their sources
  support (spec 008 task 45; owner, 2026-11-19).** `BE-API-013` bars PII from
  a URL's query only, as CWE-598 has it, in what the server accepts and in
  what it sends; credentials stay barred from the query and the path, and
  from the fragment of a URL the server sends. The opaque identifier an
  RFC 8058 one-click `List-Unsubscribe` URI carries may appear in that URI,
  as `FE-STORAGE-003` already allowed, so a backend-only project is no longer
  flagged for the link. `FE-STORAGE-003` keeps its bar on PII in a query, a
  fragment and a path segment, and its hand-off now gives `BE-API-013` only
  PII in a query, so a project with both files keeps today's result for PII
  in a path, and a backend-only project no longer flags it. `BE-AUTHN-022`
  accepts, for an account with no local authenticator bound, a fresh sign-in
  round trip through its external identity provider as the re-authentication
  a security-settings change requires. `FE-STORAGE-006` keeps tokens and
  credentials out of the Cache API and lets a service worker cache other
  user-specific authenticated content, PII included, when the spec states the
  cache is partitioned per user and purged at sign-out. `FE-DEPS-008` takes
  the registry-signature check at any point before the dependencies are built
  into a production artifact, by the package manager's own mechanism, and a
  spec may state that its package manager has none; where
  `security-backend.md` is loaded, its `BE-DEPS-004` still owns the
  frontend's verification, as part of installation. `BE-API-015` (a) and
  `BE-DATA-012`'s Verification are reworded with the same behavior. None of
  these turns a passing `/analyze` into a finding.
- **BREAKING: `FE-DEPS-009` flags what its Statement requires (spec 008 task
  45; owner, 2026-11-19).** Its Verification flagged an omitted defence only
  in a project with private packages or a high dependency-churn rate. It now
  flags a missing private-registry scope where private packages exist and a
  missing lockfile integrity check in every project, and (c) requires the
  plan to name where a new dependency's name is checked: an allowlist, a
  lockfile-diff review step or a tool. Breaking for a spec or plan covering
  frontend dependency management, in a project without `security-backend.md`,
  that omits the integrity check or names no such place: each is now a
  blocking `/analyze` finding.
- **`security-cross.md`'s `SEC-DATA-001` bars PII from a URL's query only
  (spec 077 task 13; owner, 2026-11-19).** As `BE-API-013` now does,
  credentials stay barred from a URL's query, path and fragment and must
  travel in a request header or body, and PII is barred from its query, as
  CWE-598 has it. The opaque identifier an RFC 8058 one-click
  `List-Unsubscribe` URI carries may appear in that URI. `SEC-AUTHN-002`'s
  Source now says its `state` on every authorization request and PKCE for
  every authorization-code flow go beyond RFC 9700 on purpose, since RFC 9700
  makes PKCE RECOMMENDED for a confidential client and lets PKCE or the
  OpenID Connect `nonce` stand in for `state`; its Statement and Verification
  are unchanged. Neither turns a passing `/analyze` into a finding.
- **`configuration-cross.md`'s `CFG-ENV-013` sets aside a throwaway
  credential (spec 017 task 47; owner, 2026-11-19).** A credential for a
  throwaway local or test resource that grants no access to a production or
  shared system, such as a `docker-compose.yml` `POSTGRES_PASSWORD: postgres`
  or a committed `.env.test` key for a local emulator, is not a secret under
  the rule, in its Statement and its Verification. The rule stays a MUST, and
  its permission to commit an encrypted copy is as strict as before: the
  Verification says a secret committed encrypted still owes every condition.
  It turns no passing `/analyze` into a finding.
- **`reliability-backend.md`'s `BE-RETRY-005` lets a declared-expendable
  message be dropped (spec 039 task 8; owner, 2026-11-19).** A message the
  spec declares expendable may be dropped once its redelivery bound is
  exhausted, when the drop is recorded where the responsible team can see
  it, and it needs no dead-letter destination. `/analyze` still flags a
  discard the spec does not declare or does not record, and the MUST for
  every other message is unchanged. It turns no passing `/analyze` into a
  finding.
- **`api-backend.md`: `BE-REPR-003` exempts a bounded integer field, and
  `BE-PATCH-001` accepts a format its endpoint's spec documents (spec 068
  task 8; owner, 2026-11-19).** `BE-REPR-003` stays a MUST, and its Source
  says the MUST goes beyond RFC 7493 §2.2 on purpose, which advises against
  such a number and recommends a string. Its Verification no longer flags an
  integer field whose spec bounds its values within −(2^53 − 1) to
  2^53 − 1; a high-precision decimal stays flagged, since no magnitude bound
  keeps its precision. `BE-PATCH-001`'s project patch format may be
  documented in the spec that introduces the endpoint as well as in
  `specs/system.md`; its Verification already passed any format with stated
  absent-versus-`null` semantics, wherever documented. Neither turns a
  passing `/analyze` into a finding.
- **`concurrency-backend.md`'s `BE-COORD-003` flags a dual write only for a
  message the spec does not declare loss-tolerant (spec 038 task 8; owner,
  2026-11-19).** Its Verification asks for an outbox, change-data capture or
  a single source of truth, and flags a commit and a publish made as separate
  steps, only for a message or event the spec does not declare
  loss-tolerant, as its Statement binds an operation that must both commit
  and publish. The Statement is unchanged. It turns no passing `/analyze`
  into a finding.
- **`accessibility-frontend.md`: five Verifications flag only what their
  Statements cover (spec 069 task 6; owner, 2026-11-19).** Each gains a
  Not-flagged case: `FE-KBD-004` a region the user opened, such as a modal
  dialog and its backdrop, and a fixed or sticky element the spec states
  overlaps no focusable content; `FE-A11YFORM-003` a field about someone
  other than the user, such as a recipient's name or address;
  `FE-A11YFORM-006` a `change` or `input` handler that only updates the
  page's content or its URL query, since WCAG 2.2's definition of a change
  of context says a change of content is not always one; `FE-LAYOUT-002` a
  fixed-height `overflow: hidden` container whose full text is revealable;
  and `FE-MOTION-002` a cycle the spec bounds below the general and red flash
  thresholds. `FE-LAYOUT-002`'s clause gives its reason as a way to reveal the
  text answering the Statement's truncation bar. No Statement changes, and none
  turns a passing `/analyze` into a finding.

### Removed

- **BREAKING: Windows is no longer a published platform (spec 048, AC25; owner
  decision, 2026-11-19).** A release now publishes `ductus-{target}.tar.gz`
  and its `.sha256` for `aarch64-apple-darwin`, `x86_64-apple-darwin`,
  `x86_64-unknown-linux-gnu` and `aarch64-unknown-linux-gnu` only, with no
  `x86_64-pc-windows-msvc` asset, and no workflow starts a Windows runner.
  `/ductus` on a Windows host halts at the Runtime acquisition triple table as
  on any unpublished platform, naming the store path and the `[runtime]` key.
  An adopter on Windows builds the runtime from source and sets `[runtime]
  path` in `.ductus/config.toml` to the binary: `/ductus` then downloads
  nothing and keeps the pointer's copy current. The Windows asset 0.59.11 and
  earlier published stays on those releases, and the runtime's own Windows
  code stays.
  Breaking for an adopter on Windows: `/ductus` halts at acquisition until the
  runtime is built from source and `[runtime] path` names it.
- **The `parity:` frontmatter key is gone from every shipped command and from
  `framework/bootstrap/ductus.md`, with the TODO captures under
  `runtime/tests/parity/` (spec 022's AC7; owner decision, 2026-11-19).** It
  declared how a command's `ductus exec` output would be compared with an
  LLM-driven capture, but no capture was ever taken and no comparison ran:
  `runtime/tests/parity.rs` printed SKIPPED for each, and 048 retired the
  prose-walked path a capture would record. The golden-stream tests that
  compare each command's envelope stream byte for byte are unchanged.
  `apply-manifest` still accepts the key in a command file, so a newer binary
  at a project's `[runtime] path` can apply an older release's archive.
- **Five overdue migrations leave the registry: `gitignore-marker-rename`,
  `governance-config-rename`, `spec-and-plan-sunset`, `rule-files-relocate` and
  `workflows-sunset` (spec 027; owner decisions, 2026-11-17).** Their
  `sunset_after` (0.10.0, and 0.25.0 for `workflows-sunset`) was at or below
  0.59.11, so Pre-run Migrations already skipped each at 0.59.11, which skips
  an entry once the release reaches its `sunset_after`; they stayed registered
  because nothing prompted their archival. `framework/migrations.toml` now
  holds 15 entries (14 at `ductus-v0.59.11`), none with a `sunset_after`; the
  procedure files are deleted, and each recipe, with its **Target paths:**
  line, is appended to the repository's `CHANGELOG.md` under Archived
  migrations for an adopter far enough behind to apply one by hand. Family 10's
  new check 10e fails an active entry at or past its `sunset_after`. Not
  breaking: no adopter on 0.59.11 or later could run one. `c061c1f0`
- **BREAKING: Auggie, Antigravity and OpenCode are no longer supported agents.**
  `install.sh` and `/ductus` accept `claude` and `pi` only:
  `install.sh auggie`, `antigravity`, `agy` or `opencode`, and `/ductus
  --agents=` naming one of them, halt as an unknown agent and list the valid
  keys. A routine `/ductus` run skips a project's `.augment/`, `.agents/` or
  `.opencode/` tree and its root `opencode.json`, leaving them untouched and
  no longer updated: it seeds no permissions there, writes no MCP
  registration, and surfaces no registration step. The managed `.gitignore`
  block no longer lists those directories. A directory you keep is no longer
  ignored: a `.augment/`, `.agents/` or `.opencode/` tree is untracked and visible to
  `git add` until you ignore it. To keep one, add the lines 0.59.11's block
  carried to your own part of `.gitignore`, outside the managed block:
  `.augment/*` with `!.augment/commands/`, `.agents/*` with
  `!.agents/skills/`, and `.opencode/`. `framework/bootstrap/configure/`
  keeps `claude.md` and `pi.md`, and the runtime no longer resolves a command
  under `.opencode/command/`. It still refuses a primitive's write into
  those directories and files, as it refuses one into `.vscode/` or `.idea/`.
  An adopter on one of those hosts removes its tree, and any `ductus` MCP
  registration that host holds, by hand. `check-corpus-links` with `scope:
  repository` no longer excludes those directories as generated command
  copies, so a copy left tracked there has its links checked.
  Breaking for an adopter on one of those agents: `install.sh` and `/ductus
  --agents=` naming one halt as an unknown agent, and `/ductus` no longer
  updates its tree.

- **BREAKING: `run-generator` is no longer an MCP tool (spec 022, task 14
  (b)).** It runs any project file with `bash`, so a host that auto-approves
  the server's tools would let any ductus tool that writes a project file also
  run any command. The server stops serving it, and `runtime-tools.txt` and
  the generated configure lists drop it. It stays a `ductus run-generator`
  subcommand and a `ductus exec` step. The registry names such primitives in
  `CLI_ONLY_PRIMITIVES`, `runtime-tools.txt` lists them on a `# cli-only:`
  comment line every tool consumer skips, and the parser still recognizes
  them. Claude's retired list gains `mcp__ductus__run-generator` so an
  existing grant is revoked. `/ductus`'s State-A rule stops listing it, and
  `/analyze`'s generator-drift check and `/audit` step 1 run it through the
  CLI, `.ductus/bin/ductus run-generator --script <script>`. An agent that
  called the MCP tool must use the CLI. Spec 022, task 14; `cc4b3bd9`,
  `1a762901`, `48b0d744`.
- **BREAKING: `BE-AUTHN-020` and `BE-DEPS-006` are removed from
  `security-backend.md` (spec 008 task 45; owner, 2026-11-19).** Both were
  added after 0.59.11 and deprecated on `main` when spec 077 moved them to
  `security-cross.md`, so no tagged release shipped them. `SEC-AUTHN-003`
  holds the OAuth grant and response types, the PKCE method and public-client
  refresh tokens, and `SEC-DEPS-001` build-input pinning, both unchanged. The
  two IDs leave the shipped rule index, so `/analyze` reports a spec that
  cites either as a blocking unknown-rule finding unless the spec lists the ID
  under `retired-rules:`, as 008, 017 and 077 now do for their records of the
  move. Breaking for a project tracking `main` whose spec cites either ID:
  re-point it to the successor. The deprecations left for 0.62.0 are
  `BE-AUTHN-009` and `BE-AUTHN-013`.

### Fixed

- **`read-spec` names a `scenarios/` directory it cannot list (spec 022).** The shared
  scenario listing read a directory that exists and refuses a listing as one with no
  scenario, so `scenario-files-unreadable-count` was zero over a subject never read. The
  scenario-question scan behind `read-spec` and `check-review-gate` now lists it
  as `scenarios/` in `scenario-files-unreadable`; an absent directory is still empty.
- **A code fence closes only on a run as long as its opener (spec 022).**
  `SkipScanner` (read by about 25 primitives' structural walkers, among them
  `check-artifacts`' line-citation check) and `check-corpus-links` now open and
  close a fence as `CommonMark` does: a four-backtick block holding a
  triple-backtick example is one block, a line such as ```` ```sh ```` or a
  shorter run inside a fence is its content, and a line that begins an inline
  code span opens no fence. A fence indented to any depth still opens, since
  tasks and criteria nest fences in lists. `check-corpus-links` still reads
  links inside HTML comments. No file in this repository's spec corpus reads
  differently (the corpus run over every spec found no difference).
- **The remaining hand-rolled fence toggles close a fence as `SkipScanner` does
  (spec 022 task 61).** `extract_section_body` (the section reader of the `ductus exec` fill-step payload), `section_headings` (the fold destinations `routeFold` offers) and the link harvester behind
  `derive-dependencies`, `derive-references` and `check-review-gate`'s pointer
  reading toggled a fence on any line starting with three backticks. Each now
  closes a fence only on a run of its own character at least as long as its
  opener with nothing after it, as `SkipScanner` and `check-corpus-links` read
  it: a `~~~` fence is a fence, a shorter run or an info-string line inside a
  longer fence no longer ends it, and a backtick line whose info string holds a
  backtick opens none. So a link inside a `~~~` fence, or after a shorter run
  inside a longer fence, no longer creates a `dependencies:` edge or a
  `references:` entry, and one on a line a mis-read backtick line had opened a
  fence over now does. The command parser's test walker reads a fence the same
  way, from column 0 as before.
- **The descriptions and schema doc comments of thirteen primitives state
  examined-and-clean by a bounded list's `-count` (spec 080 task 61).**
  `check-artifacts`, `check-command-flags`, `check-corpus-links`,
  `check-orphaned-references`, `check-step-references`, `check-unfolded-specs`,
  `derive-dependencies`, `derive-references`, `derive-routing-candidates`,
  `prune-plan`, `read-spec`, `resolve-anchor` and `rewrite-spec-links` said it
  by an empty list, which is absent in the file form past `[artifacts]
  read-size-bytes`, so a caller read an absent list as an empty one. They now
  say a zero `-count`, which both forms carry (`derive-dependencies` and
  `derive-references`: a zero `updated-count` with zero `unparseable-count`,
  `unwritable-count` and `refused-count`). Four more descriptions
  (`brief-re-read`, `label-criteria`, `read-re-read` and `write-analysis`) were
  true in the file form and only lose backticks. A test in
  `runtime/tests/bounded_list_gates.rs` scans every served primitive's
  description against its own bounded lists, with a stale-checked allowlist of five non-defects, and fails on the old wording. Text only: no result changes.
- **A line directly under a criterion belongs to it (spec 022).** `read-spec`'s
  acceptance-criteria walk folds a line that follows a criterion with no blank
  line between, and starts no other block (heading, list item, thematic break,
  block quote, table row), into that criterion, as it already folded an indented
  one, so an edit to a wrapped criterion's second line changes its text and
  `classify-spec-edit` reopens a `done` spec for it. The criteria's indexes are
  unchanged. A stored criterion verdict for a lazily wrapped criterion re-judges
  once, since its key is now the whole text.
- **No subcommand's exit path panics when stderr's reader has gone (spec 022).**
  Every diagnostic `main.rs` wrote with `eprintln!` (35 sites) goes through
  `ductus::io::warn`, so a subcommand keeps its exit status and the result it
  wrote to stdout (`check-corpus-links` and `derive-dependencies` exit 1 with
  their JSON, `ductus exec` exits 1 with its terminal `error` envelope) instead
  of exiting 101. Clippy now denies `print_stderr`.
- **`ductus exec` skips a stdin line that is not UTF-8 (spec 022).** The walk
  logged and skipped a blank or unparseable line but ended in an `io-error`
  envelope and exit 74 on one that was not valid UTF-8. It now skips that line
  whole, never decoding it lossily.
- **BREAKING: `append-inbox` refuses text that would open an HTML comment (spec 022).**
  Text holding a `<!--` outside backticks with no `-->` after it made the new
  bullet's own line a comment opener, so the reader skipped the bullet and the
  rest of the file. It is refused with an error naming the fix; a closed inline
  comment or a backticked delimiter is accepted.
  Breaking under `BE-COMPAT-001`: text holding an unclosed `<!--`, accepted in
  0.59.11, is now refused.
- **A file `merge-managed-block` creates is mode 0644 (spec 022).** It landed at
  the tempfile's 0600; a `.gitignore` or `CLAUDE.md` it creates is committed and
  not secret. A rewrite keeps the file's mode, and `merge-permissions`' personal
  settings file stays owner-only.
- **`write-review` and `write-analysis` quote a path that would not read back as itself (spec 066, scenario `record-paths-are-quoted-only-when-they-need-it`).**
  The digest keys (`reviewed-digest`, `analyzed-digest`) and the unreadable
  items (`reviewed-unreadable`, `analyzed-unreadable`) were written bare, so a
  scenario or data-model file named with a colon then a space, or a space then
  `#`, broke the record:
  `write-review` wrote a `review.md` the next run could not read, and
  `write-analysis` recorded a key that read back as a shorter path, so the
  analysis read stale on every run. Each path now goes through `yaml_string`,
  which quotes exactly when the plain form would not read back, and never
  flattens the path. A path that was bare stays bare byte for byte (999 digest keys, every indented line under a `reviewed-digest:` or `analyzed-digest:` key, over the 76 `review.md` and 76 `analysis.md` records of this repository at the fix's base commit, 0 changed), so no stored record changes
  and no analysis goes stale on that account.
- **`rewrite-spec-links` re-points a reference-style definition and keeps a link's title out of the path (spec 051, scenario `a-definition-and-a-title-follow-the-link-grammar`).**
  A definition line (`[label]: ../{from}/spec.md`) in a spec-root file was left
  naming the retiring directory, and no check saw it dangle. It is now
  re-pointed as an inline link is, its title and spacing kept; a footnote
  definition (`[^1]: ...`) and a bare `<path>`, which is no link, are left as
  written and listed in `mentions`. A link's destination is split from its
  title (and from `<...>`), so a titled link no longer reports a kept tail of
  `spec.md 'x'` and a path quoted in a title is no longer re-pointed. A link
  without a title is rewritten exactly as before. The primitive's description
  grew by 144 bytes at the fix (the constant's string body: 3,838 to 3,982); a later
  wording change moved it to 3,964.
- **A list item that says none is no open question (spec 067).** `read-spec`
  and every reader of its parser (`dashboard`, `check-review-gate`,
  `append-question`, `classify-spec-edit`, `check-artifacts`) no longer count
  `- None`, `- N/A` or `- None — …` (emphasis, case and a trailing period
  ignored) as an open question, in a spec or a scenario. An item that merely
  begins with the word, or a `none — …` item that ends in a question mark,
  still counts. `spec-integrity` no longer blocks a clarified spec over one.
- **`plan-consistency` matches its two headings without regard to case (spec
  067).** `check-artifacts` finds `Technical Decisions` and `Affected Files` in
  any case and reports a missing heading as missing, not as an empty section.
  Every other reader of a section by heading keeps its exact match.
- **The finding for a bare `:N` code span names its remedy (spec 067).** A port
  is written with its host (`localhost:8080`). Detection is unchanged.
- **`check-orphaned-references` skips git-ignored local state (spec 067).** A
  managed path the repository's ignore rules ignore (`.ductus/session.toml`,
  `.ductus/sessions/`, `.ductus/results/`, `.ductus/bin/`), which a fresh clone
  lacks, is no orphan. The exemption is silent, not a `skipped` entry.
- **`check-promotion-coverage` keeps a table's keys when a later header has an
  empty first cell (spec 067).** An entry the earlier table covers is no longer
  reported unclassified.
- **BREAKING: `prune-tasks` and `prune-plan` no longer delete past an unclosed fenced
  block or HTML comment (spec 041).** A file that ends inside one is reported
  with `unexamined-tail` (`from-line`, and `cause` `open-fence` or
  `open-comment`), an apply of it writes nothing (`applied: false`), and a walk
  lists the spec and counts it in `unexamined-count`. Before, every later `##`
  heading was swallowed into the open block, so a prune removed a pending task
  or a design-record section. A result for a file with no open region carries
  no new key.
  Breaking under `BE-COMPAT-001`: `prune-tasks` and `prune-plan` over a file
  ending inside an open fence or comment, which applied in 0.59.11, now write
  nothing (`applied: false`).
- **`relocate-audit-records` carries each key's whole value (spec 057).** It
  keeps the indented lines of a key it carries from an existing `review.md` or
  `analysis.md` (`waivers:`, `decisions:`, `reviewed-digest:`,
  `criteria-verdicts:`, a block scalar, a sequence at the key's own indent),
  compares a key's whole value for `disagreements`, and no longer ends a
  `review:` or `analyze:` block at a blank line. Before, a multi-line key the
  block did not override was emptied while the call reported success. Input
  with no multi-line key merges as before. An existing artifact holding both
  `reviewed-at` and `last-run` keeps `last-run` alone (it was written a second
  time, a duplicate key `validate-frontmatter` reports as a hard fail), the
  dropped spelling is named in `disagreements` when its value differs, and a
  renamed key with an empty value is written without a trailing space.
- **`ductus exec` no longer fails a walk when a warning cannot reach stderr
  (spec 022; its review of 2026-11-19).** The walker warns on stderr for each
  stdin line or envelope it ignores (a blank or unparseable line, a response
  for another request, an envelope of another kind) and carries on, but it
  wrote each warning with `eprintln!`, which panics when stderr's reader has
  gone, so a walk that would have completed exited 101. The six warnings now go
  through `ductus::io::warn`, which drops a failed write, as
  `warn_unknown_keys` already did. A test runs a walk with stderr's reader
  closed, and `scripts/mutation/stderr-warnings-022.py` proves it.
- **BREAKING: `fetch-archive` fetches over plain `http` only from a named loopback
  host (spec 022 task 50; 022's analysis of 2026-11-17, `BE-DATA-001`).** A host
  named in `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` skipped both the https-only and
  the internal-address screens, so plain `http` reached an internal mirror, a
  leg that crosses hosts. A named host still skips the internal-address screen
  over `https`. Plain `http` now passes only when the named host is a loopback
  literal (`127.0.0.0/8` or `::1`). Any other named host, the name `localhost`
  among them, is refused over `http` with an error that names loopback, on the
  fetch URL and every redirect hop alike. A loopback address is never fetched
  through a configured proxy: the loopback ranges join every proxy's
  `NO_PROXY` list, so that plaintext stays on this host (022's review of
  2026-11-17). `specs/system.md` documents the loopback plaintext. An adopter
  fetching from an `http` mirror now fails and must serve it over `https`.
- **BREAKING: `ductus exec` refuses a forwarded argument it cannot bind (spec
  022 task 49; found by 022's pre-done verification, 2026-11-17).** `run_exec`
  bound each `key=value` argument into the walker context and dropped any other
  without a word. So `ductus exec analyze 022` analyzed the session's target
  and still reported `complete`. An argument with no `=`, or with an empty key,
  now halts before the command file is read, with a terminal `error` envelope
  coded `invalid-argument` that names it, exit 1. `--help` says each argument
  is `key=value`. An invocation that passed a bare argument and relied on its
  being ignored now fails, a narrowing of accepted input (`BE-COMPAT-001`). No
  framework file or test passed one.
- **BREAKING: `merge-permissions` refuses a settings path whose link under a config
  directory leads into a protected one, and an archive's staging path may not
  contain the working directory (spec 022 task 43; 022's review of 2026-11-17,
  `BE-INPUT-004`).** `check_no_protected` let a config-directory path pass by
  ending the whole judgment there, so the landing and links-lead forms were
  never judged: `.claude/sub/pre-commit`, with `sub` a link to `.git/hooks`,
  was written. Every judgment now runs, and only the config-directory verdict
  is skipped. `extract-archive` and `fetch-archive` refuse a `dest` that
  contains the working directory, as they already refused one inside it.
  Breaking under `BE-COMPAT-001`: a settings path through such a link, and a
  `dest` containing the working directory, accepted in 0.59.11, are now
  refused.
- **`apply-manifest`'s `create` and `skip-if-conflict` strategies,
  `create-plan-artifacts` and `migrate-session-file` create their destination
  exclusively (spec 022 task 43; 022's review of 2026-11-17, `BE-RACE-003`).**
  Each checked that the file was absent and then replaced it with a rename, so
  a file that appeared in between was overwritten. A file that appears after
  the check is now kept, and reported `skipped-exists`, `kept` or
  `kept-existing`. The raw-write allowlist check sees production code that
  follows a mid-file test module, which it used to skip.
- **BREAKING: a project file the runtime reads under a fixed name, or through a path a
  config key or committed text builds, is refused or reported unread when a
  symbolic link leads it out of the project (spec 022 task 42; `BE-INPUT-004`,
  the owner's class decision of 2026-11-17).** Contained now:
  `.ductus/config.toml` in every reader (`Host::load`, `Paths::load` and seven
  primitives), each feature directory `dashboard` lists and the scenario path
  its session names, the rule directory (`discover-rule-files` and
  `write-analysis` refuse it, the exec analyze walk halts naming it, and
  `derive-routing-candidates` records a skip), the spec root's `inbox.md`, a
  `references:` entry's `spec` under a service checkout (reported `broken`,
  the outcome of a target that does not resolve),
  `check-orphaned-references`' migration registry,
  `check-artifact-size`'s project files, and the fixed-name sources of
  `check-step-references`, `check-corpus-links`, `check-artifacts`,
  `append-inbox`, `compute-review-scope` and `derive-routing-candidates`.
  `rewrite-spec-links` walks a scenario target's feature directory, lists a
  link below the spec root that leaves the project without testing what it
  points at, and holds `from` to the feature-directory name rule before it
  reads anything (`BE-INPUT-002`): `--from .` re-pointed every `./` link in a
  fixture corpus, and now exits with an invalid-argument error.
  Breaking under `BE-COMPAT-001`: a project file behind a link leaving the
  project, read in 0.59.11, is now refused or reported unread, and `rewrite-spec-links --from .` is an invalid argument.
- **BREAKING: the spec-root listing is contained before it is read (spec 022 task 48;
  `BE-INPUT-004`, the owner's class decision of 2026-11-17).**
  `list_feature_dirs` refuses a spec root that is, or runs through, a symbolic
  link leaving the project, instead of listing the outside directory.
  `resolve-feature` (whose `ambiguous` and `not-found` outcomes could name or
  hide outside entries), `create-feature`, `prune-plan`, `prune-tasks`,
  `dashboard` and the commit attribution behind `compute-review-scope`,
  `diff-cross-spec` and the fix-round primitives return the invalid-path error;
  `derive-routing-candidates` skips its spec-corpus source as unreadable, and
  the derivation reads as incomplete.
  Breaking under `BE-COMPAT-001`: a spec root that is or runs through a link
  leaving the project, listed in 0.59.11, now returns `invalid-path`.
- **BREAKING: five more project-file reads and probes are contained, and the containment
  test holds every feature-argument site (spec 022 task 46; 022's review of
  2026-11-17, `BE-INPUT-004`).** `resolve-anchor` no longer reads
  `framework/bootstrap/ductus.md` through a link that leaves the project (it
  reads as an absent installer, so nothing is judged across documents);
  `derive-dependencies` and `derive-references`, outside a git repository,
  refuse a spec root that is such a link (`invalid-path`) and name a feature
  directory that is one as `unparseable` without probing it; the exec walker's
  spec-file binding no longer probes a context `path` that leaves the project;
  and the session peek, and the `ductus exec` seed, refuse a link at the
  process's own session file whether or not its target exists, where they read
  a dangling one as no record. The interpreter tests' feature-argument table
  gains the eight sites it lacked (`brief-re-read`, `read-re-read`,
  `check-prescriptions`, `start-fix-rounds`, `classify-spec-edit`,
  `discover-rule-files`, `create-feature` and `dashboard`) and
  `rewrite-spec-links`' scenario target, so removing any one site's check fails
  a named case.
  Breaking under `BE-COMPAT-001`: a spec root or feature directory that is a
  link leaving the project, read in 0.59.11, now returns `invalid-path` or is
  `unparseable`.
- **BREAKING: `append-inbox` refuses text that is only a list marker (spec 022 task 44,
  from 022's review).** A bare checkbox (`- [ ]`, `- [x]`) passed the empty-text
  check before the marker was stripped, so it was written as an empty bullet
  that `remove-inbox-item` could not name. The text is now checked again after
  the strip and refused as an invalid argument, as scenario
  `append-primitive-marker-normalization` requires.
  Breaking under `BE-COMPAT-001`: text that is only a list marker, accepted in
  0.59.11, is now refused.
- **BREAKING: `ductus exec` labels only a command file that leaves the project
  `command-file-outside-project` (spec 022 task 44, from 022's review).** A
  project root that cannot be resolved was reported under the same code; it now
  halts as `config-unreadable`, with the underlying error.
  Breaking for a caller that matched `command-file-outside-project` for a
  project root that cannot be resolved: it now gets `config-unreadable`.
- **BREAKING: an installed command file is read only from inside the project
  (spec 022 task 40, from 064's analysis; BE-INPUT-004).** `ductus exec`, the
  `writeCode` payload's constitution excerpts, `render-help` (and `ductus hook
  help`) and the MCP server's always-loaded `target` copy built each candidate
  path from `cli-config-dir` and `[host] project` unscreened, and opened the
  file with no containment, so a symbolic link at an installed copy, or a `..`
  in either value, served a file from outside the project. `Host::load` now
  refuses a value that is not one path component (not empty, `.` or `..`, and
  no `/`, `\` or NUL) with an invalid-path error naming the file it came from,
  and each reader contains a candidate before it probes it: `ductus exec`
  halts with the new error code `command-file-outside-project`, `render-help`
  returns an invalid-path error naming the candidate, the excerpt load records
  `command-file-outside-project: {path}` among its unexaminable causes, and the
  always-loaded copy is reported unread, so the server still starts. A config
  whose `project` or `cli-config-dir` held a separator, which never named an
  installed copy a release writes, now halts.
- **`merge-managed-block` replaces an upgraded `.gitignore` block whole when a
  release retired three of its subsections (spec 022 task 39).** Its
  `line-prefix` walk stepped over at most two consecutive unmatched groups, and
  the agent removal retired the Auggie, Antigravity and OpenCode subsections
  together, so an upgrade wrote the new block above the old one's tail and left
  those sections and the tail's emptied comment headers below it. Replayed into
  the shipped template, 7 of the 11 distinct templates releases have shipped,
  every one from `gvrn-v0.13.0` on, did so. The walk now steps over three, and a
  test merges every released template into the shipped one and fails when a
  template change is not added to its fixtures.
- **`measure-reopenings` never takes a keyless decision for a carried one, and
  says how many records its carry check could not read (spec 079's review).** A
  person's decision naming no key matched any keyless entry with its outcome in
  another spec's record, so it was dropped as carried. Another spec's record the
  check cannot read still holds no decision, so a decision it may have held counts
  as new, and each run's `run-decisions` entry now names how many such records it
  met as `records-unread`, absent at zero (`QUAL-CLAIM-001`).
- **Seven descriptions say what the code does (spec 079's review).**
  `process-waivers` counts a plan-named file as read only through a plan entry
  the scope holds; `derive-dependencies` and `derive-references` report a spec
  behind a link leaving the project as unparseable; `resolve-anchor`'s empty
  result covers the references its scan judges; `read-deprecation-labels` calls
  a digit-led target that is not `MAJOR.MINOR.PATCH` malformed; `write-review`
  names an empty `diff-base` among the empty-scope routes; and `write-session`
  says `own-path` is absent on a host-config write. Text only.
- **BREAKING: `prune-tasks` and `append-task` no longer write a counter their own reader
  refuses, and a `#` comment in `tasks.md`'s frontmatter is no heading (spec
  041's review).** A task numbered `4294967295` made `prune-tasks --apply`
  write `next-task: 0`, and `append-task` advanced past one the same way;
  both now refuse with `malformed-tasks`, naming the number, and write
  nothing. `prune-tasks` read the first top-level heading line anywhere as the file's H1, so
  a YAML comment in the frontmatter replaced the title on `--reset`, and a
  keep-pending prune inserted a blank line after the block's opening `---`;
  both now find the heading below the frontmatter and keep the block as it was.
  Breaking under `BE-COMPAT-001`: a task numbered `4294967295`, accepted in
  0.59.11, is now refused with `malformed-tasks`.

- **BREAKING: `prune-project` opens nothing through a symbolic link leaving the project
  (spec 041 task 25, BE-INPUT-004).** Each project file is checked with
  `validate_contained` before it is read, and one that is, or lies under, such
  a link reads the new file state `outside-project`: neither read nor written,
  no entries. A rewrite re-checks its path before `write_atomic`; every listed
  `create` home is checked before an apply writes anything, and a refused one
  fails the whole call with `invalid-path`. The search leaves a tracked file
  behind such a link unread and names it in the search's `outside-project`,
  with `outside-project-count`; the list moves to the `lists-file` under the
  key `outside-project` past the read size, the count staying inline.
  Breaking under `BE-COMPAT-001`: a project file behind a link leaving the
  project, read in 0.59.11, now reads `outside-project`, and a `create` home
  behind one fails the call with `invalid-path`.

- **BREAKING: the session files open through canonical containment (spec 022, task
  34).** Every read of a session file, the lock and the sessions directory
  goes through the project's containment check: another session's file that
  is a symbolic link leaving the project is left unread and listed in
  `unreadable`, and this process's own file, the shared default
  `.ductus/session.toml` or the sessions directory as such a link is refused
  as an invalid path, never read or replaced. The `Host` config read, the
  `ductus exec` seed and `migrate-session-file`'s destination take the same
  check (BE-INPUT-004, from 062's review).
  Breaking under `BE-COMPAT-001`: a session file, lock or sessions directory
  behind a link leaving the project, read in 0.59.11, is now left unread or
  refused as an invalid path.
- **`append-task` no longer loses a concurrent append (spec 022, task 35).**
  Its read of `tasks.md`, its numbering and its write are held under a lock,
  `.ductus/locks/append-task.lock`, with the session lock's backoff and
  ten-second timeout, so two calls on one `tasks.md` each land their task
  (BE-TXN-002, from 023's re-judge).
- **`process-waivers` flattens every notice it prints (spec 066's review).**
  The notice for a waiver retained because the run did not read its file
  interpolated the stored path raw, so a control or line-separator character
  in it reached the operator; it now goes through the same flattening as the
  other notices, as 066's scenario `process-waivers-notices-flatten-stored-text`
  requires.
- **`compute-review-scope` calls a path deleted only when it is gone (spec
  066's review).** Any error reading a window path's metadata used to count
  it as deleted, so a present file under a directory the review cannot search
  left the scope and was named deleted. Only not-found, or a parent that is
  now a file, is a deletion; a path whose metadata cannot be read for another
  reason stays in the scope.

- **A fenced block inside a rule no longer cuts its Verification (spec 060's
  review).** The rule-section reader took a `#`-led line inside a fence (a
  shell or YAML comment) for a heading, which closed the rule and listed the
  rest of the fence under `discover-rule-files`' `other-headings`, and took a
  `**Field:**`-shaped line there for the field that ends the Verification. A
  fenced block's lines, its delimiters included, are now its content: neither
  heading, field, Statement quote nor Verification marker, as `CommonMark`
  renders them. A fence opens and closes as `CommonMark`'s do: a run of three
  or more backticks or tildes opens one, and only a bare run of its character
  at least as long closes it, so a markdown example nesting a fence keeps its
  lines. A fence left open runs to the end of the file. No shipped rule file
  holds a fence, so only a project's own rule files read differently.

- **`create-scenario` never replaces a scenario another call wrote (spec 022,
  task 33).** It checked that the slug's file was absent and then wrote with a
  tempfile and a replacing rename, so two concurrent calls with one slug both
  passed the check and the second silently overwrote the first's scenario
  (BE-RACE-003). The tempfile is now persisted with no-clobber, so of two
  racing calls exactly one writes and the other gets `scenario already
  exists`.

- **BREAKING: the bootstrap's acquisition names both digests on a mismatch, and its
  framework archive fetch is HTTPS-only and bounded (spec 048).** A checksum
  mismatch halts with a reason naming the expected digest (the sidecar's) and
  the computed one. The framework archive `curl` takes the same
  `--proto '=https' --tlsv1.2` and connect and total time bounds as the
  runtime downloads. The digest comparisons, against the sidecar and of the
  pointer with the resolved binary, ignore case.
  Breaking for a run whose framework archive fetch relied on plain `http`: it
  now fails.
- **A test now proves `fetch-archive` reads `SSL_CERT_FILE` and `SSL_CERT_DIR`
  at startup (spec 048 task 30).** Removing `init_proxy_env`'s capture passed
  every test, since the platform store's fallback reads the same variables at
  the first fetch. `tests/fetch_archive_cert_env_startup.rs`, compiled only
  where the verifier reads them, changes `SSL_CERT_FILE` between startup and
  the fetch and fails without the capture. It adds the `temp-env`
  dev-dependency, as the crate forbids `unsafe_code`.
- **A `--*-json` path with a line break in it no longer reaches stderr as
  several lines, and a proxy value with `://` in its credentials is redacted
  whole.** A path refused for lying outside a staging directory, and one whose
  file is not valid JSON, are now named on one line, as an unreadable one
  already was. `fetch-archive`'s refusal of a proxy value took the first `://`
  as the scheme, so a value with no scheme and `://` in its credentials showed
  part of them; a scheme now ends only after scheme characters. The five
  `--*-json` flags' `--help` text says the file is read from a staging
  directory (spec 048 task 30).
- **BREAKING: `retire-feature --allow-sequential` no longer deletes every spec on a
  name that is no feature directory (spec 052).** `--feature .` resolved to
  the spec root and removed it. The flag now widens the accepted forms to the
  sequential one only: a `feature` that `parse_feature_dir` does not accept
  is refused before anything touches the filesystem, with or without it.
  Breaking under `BE-COMPAT-001`: `retire-feature` with `--feature .` or any
  name `parse_feature_dir` does not accept, accepted in 0.59.11, is now
  refused.
- **BREAKING: `retire-feature` refuses a name holding a path separator, and a spec
  consolidated into itself (spec 052).** A sequential name leaves its slug
  ungrammared, so `--feature 012-x/scenarios --allow-sequential` parsed as
  sequential and deleted the spec's scenarios: both arguments now pass the
  feature-name screen `rewrite-spec-links` applies to its `to`. A `feature`
  that is its own `fold-target` was removed too, its spec.md satisfying the
  guard meant to prove the content had a home; it is now refused.
  Breaking under `BE-COMPAT-001`: a name holding a path separator, or a spec
  consolidated into itself, accepted in 0.59.11, is now refused.
- **`lint-markdown` lints only the paths it is given.** It passes
  `--no-globs`, so the config's `globs` (`**/*.md` in the shipped config) are no
  longer appended: a violation elsewhere in the repository is not reported, and
  no longer blocks `check-review-gate` for a feature. Paths that match no file
  report `clean: false` with no violations, the gate naming that nothing was
  examined. Given no paths, the config's globs still apply.

- **BREAKING: `ductus exec` screens its command name.** A name outside the slug grammar
  (`^[a-z0-9]+(?:-[a-z0-9]+)*$`), such as one holding `/` or `..`, is refused
  with an `invalid-command-name` error naming it before any candidate path is
  built, in `exec` and in the `writeCode` payload's command-file lookup.
  Breaking under `BE-COMPAT-001`: a command name outside the slug grammar,
  accepted in 0.59.11, is now refused with `invalid-command-name`.

- **A closed stdout is a quiet exit, not a panic.** A subcommand whose stdout
  reader has gone (`ductus dashboard | true`) exits 141 (`128 + SIGPIPE`),
  distinct from a failure (1) and a panic (101), with no panic message; any
  other stdout write error is reported and exits 1. The `mcp` server's
  transport is unchanged.

- **`read-tasks` returns a wrapped `Done when` clause whole.** The clause is
  its first line joined, by single spaces, with each following more-indented
  line that is not a list item, up to a blank line, a heading, a bullet or a
  less-indented line; it was truncated at the first line. `mark-task`'s
  subtask indices are unchanged, since a continuation line is no checkbox.

- **The latest-release lookup quotes its URL.** `/ductus`'s Source
  resolution and Pi's `/{project}:configure` put
  `${DUCTUS_REPO:-stonean/ductus}` inside a quoted URL, so a `DUCTUS_REPO`
  holding whitespace or a glob character no longer changes the `curl`
  command's arguments.

- **`/analyze` goes on for a `done` spec holding an open box, so `--fix` can revert it.**
  Its first step stopped on any open box, a `done` spec's too, so the review-state drift
  such a spec has never reached `--fix`'s revert. Both commands' first step now names the
  exception for a box under a task the run's own deferral names on the stop itself, which
  `/analyze`'s re-check after such a deferral meets, and stops only that feature's run
  under `--all`. `§spec-lifecycle` now says a typo fixed in one scenario or data-model file
  reopens the spec, as `classify-spec-edit` has always read it. Spec 079.
- **A deferral's task is matched at the task level.** `write-review` and `write-analysis`
  left out the section of any numbered heading matching the deferred task, so in a phased
  task list a phase numbered like it exempted the open boxes under it. Spec 079.
- **BREAKING: a `[services]` entry with no `repo` or `path` halts, as does a config that
  does not parse.** `resolve-references`, `derive-references`,
  `classify-spec-edit` and `render-status` read the registry through one reader (`dashboard` did until spec 080 moved the references readout),
  which names the file, and the entry's alias and field. `derive-references`
  read a config that did not parse as no registry, so `--write` gave every
  reference `service: null`, and an empty `path` resolved a service against
  the project itself. `resolve-references`' result is bounded by `[artifacts]
  read-size-bytes`, past which its two lists move to the file `lists-file`
  names. Spec 030, task 15.
  Breaking for a caller that read a config that does not parse as no registry,
  or a `[services]` entry with no `repo` or `path` as a service: both now
  halt.

- **A repo registered under two aliases is reported.** `resolve-references`
  returns `duplicate-repos`, each repo the `[services]` registry holds under two
  or more aliases, compared with a trailing `/` and `.git` removed, with those
  aliases, and an empty list when there is none; `/analyze` reports each as an
  Advisory finding. A link to such a repo resolved through one alias with
  nothing saying so, though the data model and `/link` called it a finding.
  Spec 030, task 14.

- **BREAKING: a review whose `diff-base` names no commit records nothing.** `write-review`
  refuses a `diff-base` its scope cannot be derived from, naming it, and leaves
  `review.md` as it was, where it recorded `scope: 0` beside whatever `examined`
  the run claimed, a window no check could tell from an empty one. An empty
  `diff-base`, a spec that never entered `in-progress`, still records `scope: 0`,
  and now counts as an empty scope, so it refuses over an earlier record counting
  a finding. Spec 079, task 20.
  Breaking under `BE-COMPAT-001`: a `diff-base` naming no commit, which
  recorded `scope: 0` in 0.59.11, now fails and records nothing.

- **A waiver notice prints a stored waiver's text on one line.**
  `process-waivers`' expired, retained and duplicate notices, which `/review`
  prints verbatim, print the waiver's `rule`, `file` and `reason` flattened,
  so a control character a hand edit left in `review.md`'s `waivers:` list no
  longer reaches the terminal raw. Which waivers apply, expire or are
  retained is unchanged. Spec 066, task 13.

- **BREAKING: What a review finds on the exec path, and what the rule waived earlier, holds
  `done`.** A `performReview` pass can mark an observation `contract-break`, so an
  exec run records and lists a broken contract, and the gate's hold fires for it.
  `write-review` refuses a waived break while the rule's own stored waiver for its
  key stands, so an observation the rule waived and a later run judged a broken
  contract is fixed, deferred or waived by a person. Spec 079.
  Breaking under `BE-COMPAT-001`: a run waiving a break while the rule's
  stored waiver for its key stands, accepted in 0.59.11, is now refused.
- **`--waive` writes what it says.** `write-review` takes `new-waivers`, each a rule
  and a reason, and records the `waivers:` entry from the run's findings, so a waived
  finding stays waived on the next run. It used to clear only the run it was given
  in. Spec 079.
- **The gate reads a damaged `decisions:` list.** An `analysis.md` whose list does not
  parse blocks, as `validate-frontmatter` says it should. Spec 079.
- **A contract edited before the review's window is read.** `compute-review-scope`
  adds each contract the record's digest says changed, so a reopening's review reads
  a scenario edited while the spec was `done` before refreshing its digest. Spec 079.
- **BREAKING: `/analyze` reports two states on a `done` spec nothing read.** A review cut off
  with its fix-round loop open is blocking review-state drift, and a
  `cross-spec-impact:` entry no back-link discharges is the advisory
  `cross-spec-impact-drift`. Disposition drift's message names the `--since` that reads
  the findings again, since a plain re-run stops or reads nothing. Spec 079.
  Breaking for a `done` spec a 0.59.11 `/analyze` passed: one whose review was
  cut off with its fix-round loop open now reads as blocking review-state
  drift.
- **A record says one thing.** A person's stored decision now decides the count and the
  body as well as the `decisions:` list when the rule waives the observation again,
  and a run's own `review.md` leaves its `scope`, so its two writes state one
  denominator. Spec 079.
- **A command name a project can run.** Primitive descriptions name
  `/{project}:<command>` rather than `/ductus:<command>`, and `check-artifact-size`'s
  fixes name the command rather than print a placeholder. Spec 079.

- **BREAKING: a `done` spec owes nothing in `tasks.md`.** `check-review-gate` blocks `done` over
  an unchecked box, as `unchecked-task`, and `/analyze` reports a `done` spec holding
  one as blocking review-state drift. It used to pass, with `/implement`'s count the only
  guard, which sees no box outside a numbered task. A release step left as a box is
  ticked or removed before `done`. Spec 079.
  Breaking under `BE-COMPAT-001`: a `done` spec holding an unchecked box,
  which the gate passed in 0.59.11, now blocks as `unchecked-task`.
- **One test of what `tasks.md` adds.** `classify-spec-edit` and `prune-plan` disagreed
  in both directions. Both now compare an unchecked line by its text under any heading:
  a box added, reworded or unticked reopens a `done` spec, including one in a `tasks.md`
  new since `HEAD`, and a box moved, or a task above it renamed, does not. `/status`'s
  count of specs with no unchecked task reads the same test. Spec 079.
- **BREAKING: an empty scope clears no finding it did not read.** `write-review` refuses over a
  record counting a finding whenever the window holds nothing but the run's own record,
  as a `ductus exec` run's does without the flag, and over a record it cannot read. An
  empty run's summary names a break it holds. Spec 079.
  Breaking under `BE-COMPAT-001`: a `write-review` over a record counting a
  finding, with a window holding only the run's own record, or over a record
  it cannot read, accepted in 0.59.11, now fails.
- **A run's two writes store one waiver, and a stored break stays one.** A `new-waivers`
  anchor an earlier write of the run stored applies and is not stored again, and a fix
  clearing its finding lets the last write finish. A decision on a broken contract is
  stored with `contract-break: true`, so a later run matching it holds `done` with no
  host marking it again. A person's decision on `src/b.rs:3:9` now names `src/b.rs`, so
  it no longer reads as expired, and a plan's pattern in scope holds the file it matches.
  Spec 079.
- **The window survives a revert, a damaged status and a rename.** A record reverted away
  or a restored run the reopening carried in starts the window at the reopen, not at the
  revert, a commit with no readable status keeps the reopening's start, and a staging
  spec renamed before its fold brings its earlier commits. A path left open stays in
  scope under an uncommitted rename, and a rename alone owes no review. `check-stuck`
  follows a rename, whose count and first box read the old path as absent. Spec 079.
- **`classify-spec-edit` reads what a line says.** A sibling link, an anchor or an image
  ahead of a reference no longer borrows its target and cuts the prose between, which
  read a real change as `mechanical` and panicked on multibyte text there. A sweep
  through any other file under a spec is `mechanical`, and a real change there still
  reopens. Spec 079.
- **A box in a `~~~` fence is no box.** The shared line scanner reads a tilde fence, only
  its own kind closing it, so the gate, drift and `waiting` no longer count one. Spec 079.
- **One old commit no longer ends a measurement.** A commit dated before the window, as
  a date-preserving rebase's is, stopped `measure-reopenings`' walk, so a closing behind
  it read `nothing-to-compare`. Spec 079.
- **`/groom` asks before a spec edit reopens a `done` spec.** The route had no step for
  it, and read a classification that failed as staying `done`. It now classifies the
  written edit and asks, as `/amend` does, naming the reasons. Spec 079.
- **`invalidate-review` clears the rest of the run.** The record no longer keeps
  `excluded`, `contract-breaks` and `fix-rounds-open` beside `last-run: null`, and
  `open-findings` stays for the next scope. Spec 079.

- **BREAKING: `fetch-archive` screens every connect-time lookup, so a redirect hop
  cannot rebind to an internal address (spec 022, task 2; `BE-INPUT-016`).**
  Only the initial host was pinned to the addresses the SSRF screen resolved.
  A redirect target was screened, then resolved again, unscreened, when the
  client connected, so a host that answered the screen with a public address
  and the connect with an internal one slipped past on every hop. The client
  now carries a resolver that applies the same internal-range screen to every
  lookup; the initial host is still resolved once and reused, on port 0 so a
  same-host redirect keeps its own port. A host named in
  `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS` stays unscreened (later narrowed to
  plain `http` from a named loopback host only; over `https` it still skips
  the internal-address screen), and so does the named host of a configured
  proxy that the client accepts, only for connect-time lookups: a fetch URL or
  redirect target naming the proxy host, or an IP-literal or rejected proxy
  value, is screened like any other host. The NAT64 prefix `64:ff9b::/96` is
  screened as its embedded IPv4 address, a refusal at connect or of a redirect
  target is named in the error the CLI and MCP print, and every error renders
  its URL, the sidecar's remote leading token and the archive path on one line
  (`BE-INPUT-011`). Commits `00e4bd3f`, `d21aaf0c`, `307b920c`.
  Breaking under `BE-COMPAT-001`: a redirect hop that resolved to an internal
  address at connect time, fetched in 0.59.11, is now refused.
- **`/analyze` states the two citation messages it had dropped (spec 008).**
  `Spec at {path} references unknown rule {ID}` (Blocking) and `Spec at {path}
  references deprecated rule {ID}; targeted for removal in {version}`
  (Advisory) are 008's messages; the parseable-procedure rewrite dropped them
  without a decision to, and `§Rules (blocking and advisory)` restores them,
  in `framework/commands/analyze.md` when the commit landed and since moved to
  `framework/reference/analyze.md`. Commit `9ab0ba18`.

- **BREAKING: `extract-archive` refuses an archive entry that would write
  through a link or whose name is not UTF-8 (spec 022; `BE-INPUT-004`).**
  `safe_join` now checks each entry name with the canonical containment check
  against the destination, so a link already under the destination cannot
  carry an entry outside it; the call fails with `InvalidPath` naming the
  link. On Unix, where a tar entry's name is raw bytes, a name that is not
  UTF-8 is refused (`UnsafeArchivePath`) rather than checked by its lossy text
  and written by its raw bytes; on Windows such a name fails as `Io`. Spec
  022; `2aec1962`, `22d34767`, `93fcc6b9`.
- **BREAKING: `prune-tasks --all --apply` refuses as a whole when one feature fails the
  path check (spec 022; scenario
  `a-path-argument-resolves-inside-its-base`).** The walk wrote earlier
  features before a later one failed the check; it now computes every
  reduction and status first. Spec 022; `e2f4e1b2`.
  Breaking under `BE-COMPAT-001`: a `prune-tasks --all --apply` that wrote
  earlier features before a later one failed the path check now writes none.

- **BREAKING: `create-feature` claims its directory atomically and refuses
  one outside the project (spec 051; `BE-RACE-003`).**
  It checked that the directory was absent and later made it with
  `create_dir_all`, so two processes in one tree computing the same branch
  number both passed and the second `spec.md` landed over the first. The
  directory is now made with `create_dir`, which the OS refuses with
  `AlreadyExists` for every caller but one, and the loser returns `created:
  false`. It also validates containment before the directory is tested or
  created: with the spec root linked outside the project it wrote
  `001-escape/` there, and now refuses with nothing written. Breaking under
  `BE-COMPAT-001`: a spec root that leaves the project is refused where it
  was accepted. A later entry extends the containment to every listing of
  the spec root. Spec 051. `f33096a5`, `54715126`
- **`label-criteria`'s `--help` summary prints on one line, and
  `criterion-label-backfill` states its idempotency (spec 008, task 29).**
  clap replaces a brace-wrapped `n` in a doc comment with a newline, so the
  summary `Assign stable AC{n} labels` split in two in both `ductus --help`
  and `ductus label-criteria --help`; the comment now reads `AC<n>`, and a
  test fails when any subcommand's summary or argument help spans lines.
  `criterion-label-backfill` step 1 no longer tells the host to run a
  reporting form `label-criteria` does not have: the pass is idempotent by
  construction. Spec 008, task 29. `74f6361c`
- **A by-hand `derive-dependencies` runs the runtime through its pointer
  (spec 022, task 21).**
  `amend`, `clarify`, `target` and the clarify, implement and plan references
  advised a bare `ductus derive-dependencies`, which is whatever `PATH`
  resolves and may not be the runtime; all eight occurrences now spell
  `.ductus/bin/ductus derive-dependencies`. Spec 022, task 21. `75da4745`
- **The review reference no longer has a reviewer hand-edit a record, and its
  sample output names the SHOULDs the gate holds on (spec 020).**
  `framework/reference/review.md` said an entry fixed after the report was
  written gains a Status line and drops out of the count, a hand edit to a
  record `write-review` regenerates whole. A SHOULD kept as-is is now waived
  with `--waive` and its rationale, a low-confidence finding needs no waiver,
  and a fix is recorded by re-running `/{project}:review`. The sample stdout's
  next line now says to fix or waive the 3 SHOULD violations before
  `/{project}:analyze`, since the done gate holds on an unwaived SHOULD. Spec
  020. `b82c1c86`, `8815e460`
- **The bootstrap states what its seeds grant, and its dead runtime tip is
  gone (spec 048, tasks 24-27).**
  §Permission Setup said each seed entry grants only what a step runs, while
  every seed but Pi's kept `ls` and `command -v`; it now says an entry that
  can run another command or write outside the project is narrowed, and
  names the two it keeps. State A's version check now names both probes (a
  `[runtime] path` prompts; only the store form is seeded). The `ductus
  runtime tip` section and its Post-Scaffolding Output bullet are removed:
  the tip rendered only on a failed acquisition, which halts in pre-flight,
  so a run that reaches it never read that file. The Pi configure command
  calls an adopter's checkout "Pi adopter", not "pip-installed". The
  Antigravity and OpenCode wording these corrections also touched went with
  those hosts, which a later entry retires. Spec 048, tasks 24-27; spec 064.
  `2ce9e62d`, `86503cfb`, `31d064e6`, `40358729`
- **Lockfile: `libz-sys` 1.1.30 to 1.1.31 (RUSTSEC-2026-0418).**
  The advisory, published 2026-10-28, is a build script that leaks
  environment variables into the build directory; `libz-sys` reaches the
  runtime through `git2` and `libgit2-sys`. `c55d81a2`

## [0.59.11] — 2026-10-17

No runtime code change. A framework release: `/{project}:implement` regains
its stop below `planned`, the constitution gains a reporting rule and is
tightened, and the lockfile takes three patch updates. A patch release: no
primitive gains an argument or a result field, and the MSRV stays 1.89.

### Fixed

- **`/{project}:implement` stops a spec below `planned` again.** Step 4 stops
  a `draft`, `clarified` or `done` spec before any task is walked, changing
  nothing, and names the command to run instead (`clarify`, `plan`, `amend`).
  The command shipped with this stop and lost it when it was rewritten for the
  runtime (`feaf7899`). Recording a question reverts an in-progress spec to
  `draft` and leaves its tasks in place, so the command could walk tasks on a
  spec with open questions. Spec 000, scenario `implement-stops-below-planned`.
- **The bootstrap's add-an-agent recipe and re-run behavior.** `ductus.md`
  §Adding a new agent names the `install.sh` arm and the README install-table
  row rather than a README curl snippet, and §Re-Run Behavior no longer names
  a per-agent session JSON.

### Changed

- **The constitution is a fifth shorter.** Every prose section states the
  rule, one reason, and its boundary; anchors, headings, tables and rule leads
  are unchanged. It goes from 148,528 to 116,564 bytes, which `/{project}:target`
  loads in every session. Spec 050, scenario
  `the-constitution-states-rules-not-stories`.
- **A report ends with the remaining work, or says there is none.**
  §pipeline-boundaries: a reply reporting on finished work is short and ends
  with *No further work identified* or a list of outstanding work tagged must,
  should or advisory. Spec 050, scenario `reports-end-with-the-remaining-work`.
- **Lockfile refresh**: `libc` 0.2.191, `schannel` 0.1.30, `zip` 9.0.4.

The `ductus.md` change means each adopter's self-update replaces the installed
bootstrap on the first `/ductus` after this release, which costs one restart.

## [0.59.10] — 2026-10-16

`check-artifacts` gains a tenth family, `body-path-existence`: a path named in
a `done` spec's body, plan, data model or scenarios that existed in this
repository and no longer resolves is an advisory finding quoting its line.
New findings can therefore fire on `done` specs, and `skipped` gains one
reason, `never-existed`. A patch release: no primitive gains an argument or a
result field, and the MSRV stays 1.89.

### Added

- **Body path existence (advisory).** The criterion family stops at
  `## Acceptance Criteria` because body prose may name a dead path correctly
  while describing history. Two discriminators make the rest of the spec
  checkable without flagging those true statements. *History*: one walk from
  HEAD per call answers every candidate at once, and a path no revision ever
  held — an example, a never-built alternative — is recorded `never-existed`
  rather than flagged; a history that cannot be read (no repository, a
  shallow clone) records `no-readable-state` and flags nothing. *Markers*: a
  line carrying a criterion marker, a negated-creation clause, or one of seven
  body markers (`delete`, `remove`, `retire`, `replaced`, `superseded`,
  `rename to`, `` ` → ` ``) describes a transition and is recorded
  `not-a-live-claim`. Lines are markdown's logical lines — a paragraph, a list
  item, a blockquote paragraph, a table row or a heading, hard wraps joined —
  so a wrapped sentence keeps its marker. Candidates follow the criterion
  grammar with a `#fragment` stripped, and `../` resolves from the citing
  file. Spec 067, scenario `body-path-existence-family`; `/ductus:analyze`
  documents it under Body path existence.
- **`never-existed`** joins the closed `skipped` reason set, in the
  excluded-by-construction class: correct, and nothing is owed.

### Changed

- **Two criterion markers widen, and identifiers are one word.** `no longer
  exists` becomes `no longer exist`, which subsumes it and catches the
  plural; `target_paths` joins `target paths` as the TOML key's own
  spelling. The negated-creation predicate no longer splits a word at `_`,
  so the field name `introduced_in` is not the creation verb `introduced`.
  Measured over this repository's specs, the criterion family's results are
  unchanged.

## [0.59.9] — 2026-10-16

`check-artifacts`' scenario-consistency family reads `tasks.md` history at
every status instead of the file's shape. One finding that used to be hidden
can now fire: on a `done` spec whose `tasks.md` was reset, a scenario no
revision of `tasks.md` ever named. A patch release: no primitive gains an
argument or a result field, and the MSRV stays 1.89.

### Changed

- **Scenario consistency asks whether a scenario was ever tasked, at every
  status.** A scenario with no task in `tasks.md` is a finding only when no
  revision of `tasks.md` in the history from HEAD ever named its slug, and a
  history that cannot be read (no repository, a shallow clone) flags nothing.
  The family used to decide from the file's shape first: no task sections, or
  numbering that starts above 1 or skips, vouched for every unmapped scenario.
  That failed both ways. After a reset `append-task` numbers from 1, so a
  reopened spec lost the fingerprint and every implemented scenario was
  flagged; and on a `done` spec with a reset `tasks.md` the fingerprint
  answered before history was asked, hiding a scenario that never had a task.
  The `done` message is unchanged, so a stored decision still matches it;
  below `done` the message now names the remedy. A history that cannot be
  read is no longer silent: each unmapped scenario is listed in `skipped` as
  `no-readable-state`, at every status, where below `done` a project outside
  a git repository used to get a finding. Spec 067, scenario
  `scenario-consistency-reads-tasks-history`.
- **`/ductus:analyze` states the same rule.** Step 8 and the Scenario
  consistency section said the family skips `done` specs, which had not been
  true since the history probe shipped; they now describe the one rule both
  paths apply.

## [0.59.8] — 2026-10-16

`fetch-archive` bounds the whole download, `ductus exec`'s diagnostics name the
binary they come from, and the runtime is built with Rust 1.99.1. One fetch that
used to finish can now fail: a download slower than about 11 KB/s on the largest
release asset stops at the new ten-minute deadline. A patch release: no
primitive gains an argument or a result field, and the MSRV stays 1.89.

### Changed

- **`fetch-archive` bounds the whole download.** `FETCH_TIMEOUT` (30 s) bounds
  any one stall, so a body arriving just inside it on every read was bounded
  only by the 256 MiB cap. `FETCH_DEADLINE` (600 s) now bounds each fetch end
  to end, the archive and its sidecar separately. It is checked before every
  read of the body, so the worst case is the deadline plus one `FETCH_TIMEOUT`.
  A fetch past it fails with an error naming the deadline, and nothing is
  written. Ten minutes fails only a download slower than about 11 KB/s on the
  largest release binary (6.7 MB). Spec 022, scenario
  `fetch-archive-bounds-the-whole-download`.
- **`ductus exec`'s diagnostics name the binary.** Four stderr lines began
  `runtime exec:`; they now begin `ductus exec:`. The same stale name is gone
  from 022's spec, plan and data model, `AGENTS.md`, and the runtime's doc
  comments and test messages, which invoked `runtime mcp`, `runtime exec`,
  `runtime parse` and `runtime --version` for a binary the tree ships as
  `ductus`.
- **Built with Rust 1.99.1**, up from 1.99.0. The pinned toolchain and every
  workflow's `dtolnay/rust-toolchain` pin moved together.

## [0.59.7] — 2026-10-16

Spec 022 is split into three, the constitution corrects what the dependency
derivation reads, and the crate's readme describes the runtime that ships.
The runtime behaves as before: this release changes doc comments, the
readme, an audit-script label and one dependency patch. A patch release: no
primitive gains an argument or a result field, and the MSRV stays 1.89.

### Changed

- **022 is split into three specs.** Its data model had outgrown one read:
  121,695 bytes at 0.59.6 and 99,953 after a trim in place, against the
  50,000-byte read size. The audit-record and completion-gate primitives —
  `compute-review-scope`, `process-waivers`, `process-decisions`,
  `write-review`, `write-analysis`, `check-review-gate` and
  `diff-cross-spec` — are now specified in
  `066-audit-records-and-the-completion-gate`, and the analyze and audit
  checks — `check-artifacts` and its families, `validate-frontmatter`,
  `traverse-deps`, `resolve-anchor`, `check-rule-ids`,
  `derive-dependencies`, `derive-references`, `check-corpus-links`,
  `check-orphaned-references`, `check-command-flags` and
  `check-promotion-coverage` — in `067-analyze-and-audit-checks`, each with
  its scenarios, data-model sections and criteria. The three data models
  measure 44,823, 19,775 and 20,201 bytes. Five superseded scenarios are
  retired. `schema/primitives.rs`, the primitives' doc comments, the
  bootstrap procedure and the `generator-primitives` migration name the spec
  that now owns each contract.
- **The constitution's canonical-sources row for the scenario-to-task
  referencing rule names 067's data model**, where `scenario-consistency` is
  now specified.
- **§text-first-artifacts' scenario-link bullet says what the dependency
  derivation reads.** It said the derivation harvests sibling links from
  scenario files, so a correct link in a scenario creates a `dependencies:`
  edge that can close a cycle, and advised citing a sibling spec from a
  scenario by backticked slug. `derive-dependencies` reads spec bodies only
  and never read a scenario. The bullet keeps the one-level-short path the
  corpus link check reports, and puts the cycle warning on spec-body links.
- **`pointer-citing`, the term `check-promotion-coverage` reports, replaces
  `constitution-citing`** in `scripts/audit/promotion-coverage.sh` and in the
  doc comment on the primitive's result.
- **The crate's readme describes the `ductus` runtime.** `runtime/README.md`,
  which crates.io publishes, still described the govern-era optional `gvrn`
  binary: the old repository, tags and asset names, a first-class
  markdown-only path, and a `parse --check` that exits 0 on the legacy
  allowlist, where the binary exits 2 for legacy prose and the allowlist is
  the CI lint's. It now describes the required runtime `/ductus` acquires
  and points at the README's runtime section and `docs/runtime.md`.
- **`combine` 4.6.8 → 4.6.9** in `Cargo.lock`.

## [0.59.6] — 2026-10-15

`fetch-archive` names its timeout, and two constitution rules are promoted
from this repository's notes. The runtime behaves as before: the timeout
keeps the value it already had. A patch release: no primitive gains an argument or a
result field, and the MSRV stays 1.89.

### Changed

- **`fetch-archive` names its timeout.** Its request ran under `reqwest`'s
  blocking-client default of 30 seconds, which the runtime never stated, so
  a `reqwest` upgrade could have moved the bound silently. `FETCH_TIMEOUT`
  (30 s) now sits beside `MAX_FETCH_BYTES` and `MAX_FETCH_REDIRECTS` and is
  set on the client. It bounds the wait for the response and each read of
  the body, not the whole download, whose size the 256 MiB cap bounds. A
  test drives the client with a short timeout against a proxy that never
  answers. Spec 022, scenario `fetch-archive-names-its-timeout`.
- **§implement-phase: a fix committed before its reopen is outside the
  review window.** The review-window bullet already warned against
  narrowing a wide window. It now covers the converse: when a fix is
  committed before the reopen that records it, the window holds only the
  amendment. Record the reopen before making the fix, or set the base to
  the commit before the fix and say so in the Summary.
- **§concurrent-features: list what a push carries before making it.**
  Sessions sharing a working tree share its commits, so a push publishes
  another session's work and the checks that run on it report that work
  under the push. When the range holds commits that are not yours, confirm
  that their session has finished and that publishing them is intended, and
  name them when you report the push.

Spec 050 records both promotions in a fourth classification round, which
leaves no rule-bearing `AGENTS.md` entry unclassified.

## [0.59.5] — 2026-10-15

A constitution change from spec 050: a review may run before its work is
committed. The runtime's code and dependencies are unchanged. A patch
release: no primitive gains an argument or a result field, and the MSRV stays
1.89.

### Changed

- **§implement-phase: a review may precede its commit.** The constitution no
  longer requires recording a review against a commit that already contains
  what it reviewed. That rule rested on review staleness comparing commits,
  and staleness is now judged by a digest of the spec artifacts the review
  read, with `reviewed-against` kept as provenance. The natural order (edit,
  review, analyze, then commit) is supported, and so is committing first.
  Two sentences that assumed the old order now read correctly. §grounding's
  rule on verifying a commit's contents names claims that a commit holds a
  change, such as a completion report or a release, rather than review and
  analysis records. §implement-phase's review-window bullet states the base
  0.59.4 introduced: `HEAD` while a reopen is uncommitted. A check that reads
  git history still runs after committing, as §grounding's Rules list already
  required. Spec 050, scenario `a-review-may-precede-its-commit`.

## [0.59.4] — 2026-10-15

Two fixes. The MCP server's handshake now names ductus and the runtime's
version rather than the SDK's. `compute-review-scope` now scopes
`/ductus:review` to the working tree rather than the last commit, so a review
run before its work is committed reads the files that work changed. A patch
release: no primitive gains an argument or a result field, and the MSRV stays
1.89.

### Fixed

- **The MCP server names itself.** Its handshake reported
  `{"name":"rmcp","version":"3.5.1"}`, the SDK's name and version, because
  `get_info` never set `serverInfo` and rmcp fills it from its own build
  environment. A host now sees `{"name":"ductus","version":"<runtime
  version>"}` through `initialize` and through rmcp 3's discovery path
  alike. The tools a host is served, their schemas and their results are
  unchanged. Latent since the server was written; a new test holds it.
- **`compute-review-scope` now scopes the review to the working tree, not the
  last commit.** `/ductus:review` reads the working tree, but the files
  modified since the diff base came from a commit-to-`HEAD` diff, and the
  default base came from committed history alone. In the pipeline's natural
  order, `/ductus:amend` → `/ductus:implement` → `/ductus:review` with one
  commit at the end, the review ran before anything was committed. Its scope
  then left out every file the change touched, while the record still read as
  a complete examination. An adopter reported it on 0.58.0. They had reopened a
  `done` spec and changed two files without committing, and got a 475-file
  window that contained neither file; `--since=HEAD` gave zero files. Two
  fixes:
  - **Uncommitted files are included.** Modified-since is now one diff from the
    base to the working tree: committed, staged, unstaged and untracked files,
    with ignored files left out. `--since=HEAD` now yields exactly the
    uncommitted work.
  - **An uncommitted transition sets the base to `HEAD`.** This applies when
    `spec.md` on disk reads `in-progress` and the copy at `HEAD` does not. It
    covers both an `/ductus:amend` reopen and `/ductus:implement` starting a
    `planned` spec. An uncommitted `in-progress → done` flip leaves the base
    alone.

  `write-review` recomputes the scope the same way, so the recorded `scope`
  count matches. Spec 022, scenario `review-scope-includes-uncommitted-work`.

## [0.59.3] — 2026-10-15

The runtime's two dependencies a major version behind, `zip` and `rmcp`, move
to their latest releases, and it is built with Rust 1.99.0. A patch release:
no primitive gains an argument or a result field, and the MSRV stays 1.89.
Unlike 0.59.2, the runtime's own code changes, in three source files, to meet
the new APIs and clippy 1.99. This repository also gains a weekly check that
the runtime stays current (`scripts/check-runtime-currency.sh`, run by
`runtime-currency.yml`), which reaches no adopter.

### Changed

- **`zip` advanced 8.6.0 → 9.0.3, one major version.** `extract-archive`'s
  zip path behaves as before. An entry whose path escapes the destination is
  still rejected as an unsafe path and named in the error, and a new test now
  holds that, since nothing did before. zip 9 decodes an entry's name on
  demand, so the error reads the name through that decode, which cannot fail
  in practice. Upstream's read-side hardening comes with it: a read that runs
  past an entry's declared size fails, a ZIP64 size that overflows is
  rejected, and so is an empty archive whose central directory offset lies
  past its end record. Its symlink and permission fixes are in zip's own
  extraction, which `extract-archive` does not use. On macOS and the BSDs,
  zip 9 adds `nix` to the dependency graph.
- **`rmcp`, the MCP SDK, advanced 2.2.0 → 3.5.1, one major version.** A host
  sees the same server. Driven over stdio against 0.59.1 with each client
  protocol version from 2024-11-05 through 2025-11-25, it negotiates the
  same version and serves the same 61 tools, with the same tool-list digest,
  the same `read-spec` result and the same unknown-field rejection. The one
  difference is a correction: a client asking for 2026-07-28 used to have
  that version echoed back through `initialize`, which the server did not
  implement. rmcp 3 serves 2026-07-28 through its discovery path instead, so
  `initialize` now answers 2025-11-25. The code change is the tool-call
  wrapper that rejects unknown fields, whose call now returns rmcp 3's
  `CallToolResponse`, and `ServerInfo` renamed to `ServerConfig`.
- **Built with Rust 1.99.0, up from 1.97.0.** The pinned toolchain and every
  workflow that builds the runtime move together. Clippy 1.99's new lints
  are met rather than silenced, except one: `unused_async_trait_impl` fires
  on the handlers rmcp's `#[tool_handler]` macro generates, so it is allowed
  on that impl alone. The MSRV stays 1.89.

## [0.59.2] — 2026-10-14

A new §grounding rule from spec 050, the three `§` references 0.59.1 left
unresolved, a `rustls` security update, and every dependency moved to its
latest compatible version. The runtime's own code is unchanged; its binary
changes through its dependencies. A patch release: no primitive gains an
argument or a result field.

### Added

- **§grounding: run the project's own command, never a hand-rolled equivalent
  of one.** When a project declares how to build, test, lint, migrate or
  package something — a `package.json` script, a Makefile or justfile target,
  a compose file, a CI workflow step, a `Commands` section in its `AGENTS.md`
  — that declaration is the command. A substitute that succeeds proves only
  that the substitute works, and leaves the project's own command, with the
  flags, environment and build arguments it carries, unexercised. A narrower
  invocation while iterating is fine, but the result reported at the end is
  earned by the project's command. A failing project command is a finding
  that takes one of §design-principles' three dispositions, never something
  to route around. A deviation names the command it bypassed and why; a
  repeatable operation with no declared command is proposed for the project's
  own command runner as a task; and a declared command is not an
  authorization to run a deploy, a publish or a destructive migration unasked.
  The bullet states that nothing enforces it, since the substitute runs in a
  shell and never reaches a diff. Spec 050.

### Fixed

- **The last three `§` references the installer ships now resolve.** 0.59.1
  said every `§` reference the installer ships resolves under
  `resolve-anchor`; three did not. `configure/pi.md` cited a section of spec
  064 without naming the document, and `api-backend.md`'s two Sources cited
  sections of the Google Cloud API Design Guide, which is not a markdown
  document, as `§` anchors. The first now links spec 064, and the second
  names its sections in prose. This repository's self-audit now holds the
  whole shipped set at zero (spec 026). Framework only.

### Changed

- **Every dependency is at its latest version compatible with the declared
  requirements and `rust-version` 1.89.** The lockfile had drifted 121
  packages behind. `cargo update` locks 125 (121 updated, 4 added, 2
  removed, none downgraded), thirteen of them direct dependencies, and none
  was held back by the MSRV. `rmcp` and `zip` stay one major version behind
  (3.5.1 and 9.0.3 are available), because moving them is a code change and
  gets its own release.

### Security

- **`rustls` advanced 0.23.45 → 0.23.46 for RUSTSEC-2026-0346,
  RUSTSEC-2026-0347, RUSTSEC-2026-0348 and RUSTSEC-2026-0349.** Unlike
  0.59.1's `quinn-proto` bump, this one reaches the binary: `rustls` is a
  direct dependency and the TLS stack under `reqwest`, which `fetch-archive`
  uses. The advisories cover `Debug` impls that expose client session
  secrets, application data sent to the outer server after an ECH rejection,
  brotli certificate decompression that can allocate up to 1 GiB before
  authentication, and an unauthenticated ServerHello that can downgrade later
  connections to classical key exchange. They were published on 2026-10-14,
  and `cargo audit` gates both the push workflow and the release.

## [0.59.1] — 2026-10-13

Two false findings an adopter reported against 0.58.0, both reproducible on
0.59.0, and a lockfile security bump. A patch release: no primitive gains an
argument or a result field.

### Fixed

- **`check-orphaned-references` no longer reports a revision-qualified git
  path as a missing file.** A referrer citing `git show abc1234:specs/x/spec.md`
  — a retired spec's text at the commit that last held it — was matched on its
  `specs/` root, read as a working-tree path, and reported orphaned. A path
  after a colon is now skipped when the token before the colon is empty (the
  index) or shaped like a revision. A variable or a path before the colon
  marks a colon-joined list such as `PATH="$PATH:.ductus/bin"`, and a `]`
  marks a markdown link label; their paths are still checked, as is the same
  path written bare. Spec 022.
- **Every `§` reference the installer ships resolves under `resolve-anchor`.**
  An adopter running `resolve-anchor` over its installed `ductus.md` against
  `.ductus/constitution.md` got unresolved anchors it could not fix: headings
  cited short (`§MCP registration` for *MCP registration (per-agent)*),
  archive-half sections no installed copy holds, a bold paragraph cited as a
  section, a heading split across two TOML comment lines, and other documents'
  sections cited without naming the document. Archive-half sections are now
  named in bold, `§` is kept for a local heading or a constitution marker, and
  a section of another document names that document on the same line. The
  archive half, the migration bodies and `/analyze`'s command source carried
  the same shapes and are fixed with it. Framework only.

### Security

- **`quinn-proto` advanced 0.11.16 → 0.11.18 for RUSTSEC-2026-0338,
  RUSTSEC-2026-0339, RUSTSEC-2026-0340, RUSTSEC-2026-0341 and
  RUSTSEC-2026-0342.** Lockfile only: `quinn-proto` is compiled into no target
  (`cargo tree -e all --target all -i quinn-proto` prints nothing), so no
  shipped binary changes. The advisories reached the database after 0.59.0's
  run went green; `cargo audit` gates both the push workflow and the release.

## [0.59.0] — 2026-10-06

Pi becomes the fifth agent ductus scaffolds into (spec 064), with a
dependency-free extension bridge as its tool surface, and a fork can now be
adopted or tested by naming it in `DUCTUS_REPO` (spec 065). A new agent and a
new configuration variable — hence a minor bump. The fixes below sharpen the
mechanical-sweep exemption and `/analyze`'s checks around the new agent.

### Added

- **Pi host (spec 064): `Host::command_file_candidates` gains the pi prompt-template shape `.pi/prompts/{project}-{name}.md`, appended last.** The runtime now resolves installed slash-command files for Pi — the fifth supported agent — whose commands are flat project-hyphenated prompt templates rather than the two directory-shaped layouts (`commands/{project}/` for claude-style, `command/{project}/` for opencode). The two pre-existing candidates keep their relative order, so every pre-pi adopter resolves identically; the `cli-config-dir` in the gitignored session file remains the real selector. The pi layout's tool surface is a zero-dependency extension bridge (`.pi/extensions/ductus.ts`) wrapping this runtime's MCP server over stdio; the MCP server itself is byte-identical — no new primitive, no schema change.
- **`DUCTUS_REPO` adopts or tests ductus from a fork (spec 065).** `install.sh` and `/ductus` take the source repository's `owner/repo` from it, default `stonean/ductus`, for every fetch: the bootstrap, the version pin, the framework archive and the runtime binary. Any other value is announced before the first fetch, and the installer's next step says to export it to the agent too, because a value set for the installer alone would leave every later fetch on `stonean/ductus`. It is listed in the environment-variable inventory in `docs/runtime.md`.

### Fixed

- **A renumber that shifts several numbers in one sweep no longer stales
  the reviews it touches.** The mechanical-sweep exemption applied its
  repo-wide rewrites one after another, so a sweep carrying `063` → `064`
  and `064` → `065` rewrote `063` through to `065`, and a spelling-only
  link change read as a contract change. Both halves — the transition
  gate's `changed_beyond_spelling` and audit Family 19 — now apply the
  rewrites in one simultaneous pass. Spec 022.
- **`check-corpus-links --scope repository` skips every agent's generated
  command copies, not only the session agent's.** A repository can commit
  more than one agent's copies — this one commits Claude's and Pi's — and a
  session of one agent reported the other's links, which are broken by
  construction, as defects. The exclusion is now every `config_dir` in the
  bootstrap's Agent Registry, held to the registry by a test. Spec 022.
- **A renamed file now contributes its rewrites to the mechanical-sweep
  exemption, and both halves agree on it.** The transition gate's
  `SweepIndex::build` diffed without rename detection, so a renamed contract
  was a delete plus an add and contributed nothing, while audit Family 19's
  `git diff` detected renames — the halves built different repo-wide sets.
  Both now detect renames at a stated 50% threshold and 10,000-path limit
  instead of inheriting git's, libgit2's or a contributor's config; a window
  over the limit grants no exemption in either, and a rename with its content
  unchanged reads alike in both. Family 19's diffs also pin colour, external
  diff drivers, textconv and prefixes, and its pre-digest arm lists both sides
  of a move. `mechanical_sweep_parity` holds the halves to it with a renumbered
  directory fixture. Spec 022.
- **`criterion-path-existence` no longer flags a registered agent's scaffold
  paths in the framework repo.** The ships-to-adopter set was derived from the
  Shared Files tables alone, so 064's criteria naming `.pi/prompts/ductus.md`
  and `.pi/settings.json` read as stale here. It now also derives each Agent
  Registry row's `ductus` install path, settings file and project-scoped MCP
  target, for every registered agent, dogfooded or not. Spec 022.
- **`/analyze`'s command-frontmatter checks read every layout's installed
  command set.** They spelled the `claude-style` path, so for OpenCode,
  Antigravity and Pi adopters they read a directory that does not exist and
  passed. They now name the §Derived values rows, scope membership by the
  layout's cleanup glob, check `argument-hint:` only where the layout copies
  the source frontmatter, and report an absent path as unexamined. Spec 000.

## [0.58.0] — 2026-10-04

The pipeline now warns when a spec artifact is larger than an agent may read
in one call, and names the fix for it (spec 063). A new primitive and a new
`[artifacts]` configuration key carry it, and three commands gain a step, which
renumbers the steps after it and two `ductus exec` gate names — hence a minor
bump.

### Added

- **Oversized spec artifacts warn with a fix.** A new primitive,
  `check-artifact-size`, measures a feature's `spec.md`, `plan.md`,
  `tasks.md`, `data-model.md` and scenarios by their length on disk against
  `.ductus/config.toml` `[artifacts] read-size-bytes`, 50,000 bytes when
  unset; any value but a positive whole number is reported and the default
  used, and no value switches the check off. Each artifact over it gets a
  warning that an agent may not read it in one call, with the fixes for its
  kind — prune, split, trim, promote — and, on a `done` spec, which of them
  reopen it. `/{project}:clarify` and `/{project}:plan` print the warnings and
  never block on them. `/{project}:analyze` records each as an advisory
  finding in the new `artifact-size` family, and a stored discard holds until
  the file grows into another read page. A `done` spec with an artifact over
  the threshold gets that finding on its next `/{project}:analyze`, where it
  needs a decision like any other advisory finding. Spec 063.

### Changed

- **New steps renumber three commands.** `/{project}:analyze` steps 17–21 are
  now 18–22, and its detection range is steps 1–17; `/{project}:clarify`
  steps 10–13 are now 11–14; `/{project}:plan` steps 8–10 are now 9–11. Under
  `ductus exec` the gate names follow the step: `/{project}:clarify`'s gate is
  `step-12` and `/{project}:plan`'s is `step-9`. Spec 063.
- **`/{project}:plan` no longer puts code in plans.** Its Technical Decisions
  guidance has a decision name its code by `path:line` rather than reproduce
  it, so a plan trimmed to stay readable does not regrow. Spec 063.

## [0.57.0] — 2026-10-02

A branch-scoped spec now reaches `done` through the same checks as any other,
whatever its pending fold, so `check-review-gate`'s `blocked-by` no longer
takes the value `pending-fold` (spec 051). `ductus exec prune` writes the
reduction the operator confirmed rather than previewing it a second time
(spec 022). Three smaller fixes: `check-stuck` counts an unchecked box by the
grammar every tasks parser shares (spec 022), `write-analysis` no longer
writes a duplicate `unexamined-by-reason` key (spec 041), and `prune-tasks
--reset` takes the feature's identity only from a heading outside fences and
comments (spec 041).

### Changed

- **A pending fold never holds `done`.** `check-review-gate` no longer reads
  `folds-into`: a branch-scoped spec reaches `done` through the same checks as
  any other, wherever its fold target lives, and `/{project}:fold` runs after
  it. The order is done first, then fold — `done` says the staging spec's own
  work is finished, and fold-back is the upstream consolidation of its durable
  content that follows. `blocked-by` no longer takes the value
  `pending-fold`; host tooling matching it never matches, and nothing else in
  the result changes. The owed fold stays visible: `dashboard` still
  qualifies the row `(fold pending)` at every status and lists it in the
  outstanding-fold callout, and `check-unfolded-specs` still reports it.
  Below `done` the row's Next Action is now the ordinary one for its status,
  and at `done` it is the fold. A fold into a `done` upstream spec still
  reopens that spec and invalidates its review. The cross-spec-impact check
  is unchanged. Spec 051, scenario `a-pending-fold-never-holds-done`.

### Fixed

- **`ductus exec prune` writes what the operator confirmed.** A confirmed
  gate now binds `apply: true` into the arguments of every primitive
  dispatched after it, so `/{project}:prune`'s step 7 applies the reduction
  step 6 confirmed. Extension payloads do not carry it. Before, it previewed a
  second time and the walk completed with exit 0 having written nothing. A
  denied gate still ends the walk. Spec 022, scenario
  `a-confirmed-gate-authorizes-the-writes-after-it`.
- **`check-stuck` reads an unchecked box by the grammar every tasks parser
  shares**, so a tab after the dash counts as it does in `read-tasks`, and a
  `]` run into text is not a box. Spec 022.
- **`write-analysis` keys `unexamined-by-reason` by the reason as written.**
  Two reasons that flatten to one line are one entry with their counts
  summed, rather than a duplicate key the next reader refuses. Spec 041.
- **`prune-tasks --reset` lifts the feature's identity from a heading outside
  fences and comments**, so a file whose only top-level heading is an example
  in a fence is refused as malformed rather than reset under that line. Spec 041.

## [0.56.0] — 2026-10-01

Each identified agent process — one with `DUCTUS_SESSION`, or Claude Code's
`CLAUDE_CODE_SESSION_ID` — now holds its own session target on the
`.ductus/` layout (spec 062), `/{project}:prune` reduces a feature's
`tasks.md` and `plan.md`, or every spec's with `--all` (spec 041), and
`fetch-archive` reads its proxy and certificate configuration once (spec
048). Three tools are new: `resolve-session`, `retarget-sessions` and
`prune-plan`. `read-spec` counts an open question written as any list item
(spec 022). The minimum supported Rust version rises from 1.88 to 1.89, for
`std::fs::File::try_lock`.

### Added

- **Per-process session targets.** A process's identity is the
  operator-set `DUCTUS_SESSION`, else the identifier its agent passes to the
  processes it spawns (Claude Code's `CLAUDE_CODE_SESSION_ID`), else none. An
  identified process reads and writes its own target in
  `.ductus/sessions/{identity}.toml`, and every target write in any process
  also sets the shared default, `.ductus/session.toml`, which an unidentified
  process uses exactly as before. An identified process with no target of its
  own adopts the default on its first resolution and says so. A target
  neither resolved nor written for seven days is removed by the next target
  or clear write. A write naming a feature another session targets tells
  both sessions. A cleared target never adopts the default. A session file
  that does not parse is reported by name. Session writes are serialized by
  an advisory lock on `.ductus/sessions/.lock`, waited on for at most ten
  seconds. The directory ignores itself with a `.gitignore` of `*`, written
  by the process holding the lock, and the framework's gitignore template
  lists it. Spec 062.
- **`resolve-session`** resolves this process's target, reporting its
  identity, the source (own, adopted, default, cleared or none), and the
  notices to show once. Every command that resolved the target by hand now
  calls it. Spec 062.
- **`retarget-sessions`**: `/{project}:fold` re-targets, and
  `/{project}:consolidate` clears, every session that named the removed
  spec. Each affected per-process session is told at its next command; the
  shared default is changed without a notice, since every process without
  an identity reads it, and the command's report names it. Spec 062.
- **`prune-plan`** reports each `plan.md` section outside the design record,
  meaning the plan template's own `##` headings, compared case-insensitively.
  A heading in a fenced block or an HTML comment is not a section. Each
  section comes with its size, a digest, the `plan-record` advisory
  `/{project}:analyze` records for it, and whether `analysis.md` stores a
  discard for that advisory. An apply removes the sections the host lists by
  heading and digest, and refuses the whole write when any changed since the
  preview. On a `done` spec every call reports whether the spec reopens: a
  preview for the tree as it stands, so a reopen from edits made before the
  run is known before anything is written, and an apply for the tree it
  leaves, computed before it writes. A feature with no `plan.md` is reported
  as `missing`, not an error. Spec 041.
- **`/{project}:prune` covers the spec directory.** Beside `tasks.md`, each
  plan section outside the design record is proposed, and removed once its
  durable pieces have moved home, confirmed section by section. A section
  whose advisory is stored as discarded is kept. A `done` spec the run writes
  to reopens exactly when the diff changes the design record or adds an
  unchecked checkbox to `tasks.md` — a box moved or re-indented adds
  nothing. The reopen is named before anything is written, and decided on a
  fresh reading once the run has written to the spec, so a `--reset` that
  removes the unchecked boxes a named reopen came from leaves the spec
  `done`. Under `--all`, a `done` spec whose tree already requires a reopen
  is listed even with nothing else to reduce. Spec 041.
- **`--all` on both prune primitives** walks every feature in corpus order.
  It lists each spec with something to report and names each one skipped for
  a missing `tasks.md`, or for a missing `plan.md` with nothing else to
  report. `prune-tasks` refuses `force` with `all`, and `prune-plan` refuses
  `apply` with `all`. Spec 041.
- **`/{project}:analyze` reports the plan-record advisory** at a new step 16,
  on a spec at `planned` or later. It reports every section outside the
  record, decided or not, so a stored discard is matched rather than
  expired. Spec 041.

### Changed

- **`prune-tasks` takes `feature` or `all`.** A single-feature result is
  unchanged on the wire. A walk lists one line per spec — gate, status,
  counts, sizes, path — rather than its per-section records. Spec 041.
- **`/{project}:analyze`'s steps 16–20 are now 17–21.** Spec 041.
- **A plan records the design as it stands.** The constitution's
  §plan-phase says so, and says where evidence, contributor knowledge,
  findings and handoff state go instead; its canonical-sources table names
  the plan template as the source of the design record. The plan template
  gains Trade-offs and three optional sections. The tasks template says a
  task body may carry working notes. Spec 041.
- **`write-session` and `dashboard` go through the session core**, and
  `dashboard` peeks the session read-only. `write-session` refuses a
  `feature` that is not a feature directory name and a `scenario` that is
  not a scenario slug, and its result gains `identity`, `own-path`, `peers`,
  `expired`, `unreadable` and `notices`. `dashboard`'s result gains
  `session-identity`, `session-source`, `session-notices` and
  `session-unreadable`. `ductus exec` peeks the invoking agent's target, and
  shows as progress lines what the walked command's own `resolve-session`
  and `write-session` steps report: notices, peers, and removed or
  unreadable sessions. It halts with `session-identity-invalid` on a
  `DUCTUS_SESSION` that sanitizes to nothing, and with `session-unreadable`
  when the invoking agent's own session cannot be read. Spec 062.
- **`fetch-archive` reads its proxy and certificate configuration once.**
  The proxy variables (`ALL_PROXY`, `HTTP_PROXY`, `HTTPS_PROXY`, `NO_PROXY`,
  each uppercase first) and whether `REQUEST_METHOD` is set are captured at
  startup and applied as explicit proxies, keeping the convention's selection
  and its CGI guard. `fetch-archive` reads its allowlist variable once, at
  startup (spec 062). On Linux and other non-Apple Unix the trusted
  certificates load once per process, at the first fetch: the system roots,
  or, when `SSL_CERT_FILE` or `SSL_CERT_DIR` is set, only those it names in
  place of the system store. A store that yields no certificate is refused by
  name. Through a proxy, the operator's proxy is the documented trust
  boundary; the SSRF screen and a direct connection's address pin are
  unchanged. `docs/runtime.md` gains an inventory of the environment
  variables that configure the runtime, `DUCTUS_SESSION` and
  `CLAUDE_CODE_SESSION_ID` among them, saying when each is read; an invalid
  `DUCTUS_SESSION` is refused with an error naming it. `rustls` and
  `rustls-native-certs` become direct dependencies, at the versions already
  in the graph. Spec 048.

### Fixed

- **`read-spec` counts an open question written as any list item.** It read
  only `-` bullets under `## Open Questions`, so a numbered, `*` or `+` list
  of questions read as zero, and `/{project}:clarify` could advance a spec
  with them unresolved. A list item at the list's own level is an entry, and
  a deeper one folds into the question above it, so nested `-` sub-bullets
  now make one question where each used to count. A thematic break (`---`,
  `* * *`) opens nothing, and a marker with nothing after it opens an entry
  its continuation lines fill. Every reader of the section now agrees: a
  scenario with a numbered, `*` or `+` open question blocks the pre-`done`
  gate (`check-review-gate`) and is reported by `check-artifacts`'
  `scenario-open-questions` family, where 0.55.0 read it as clean;
  `append-question`'s duplicate check sees such entries, and it strips any
  list marker from the question it is given; and `dashboard`
  (`/{project}:status`) counts with the same parser, where it kept its own
  `-`-only count that read inside HTML comments. The spec template,
  `/{project}:specify` and `/{project}:clarify` say a question is one list
  item. Spec 022.
- **`lint-markdown` and `run-generator` stop a subprocess that hangs.**
  Each waited on its child with no bound, so an `npx` fetch that never
  answered, or a generator that never exited, held the primitive — and the
  review gate that lints through it — forever. Both now run the child under
  a five-minute timeout (`LINT_MARKDOWN_TIMEOUT`, `GENERATOR_TIMEOUT`), kill
  one that outlives it, and fail naming the program and the timeout. Spec
  022.
- **`prune-tasks` keep-pending keeps a phase heading it did not empty.** It
  dropped every phase container with no surviving task section, so a
  phase heading governing no task — one added to hold a note — was
  removed, and a phased file with nothing spent could still be rewritten. A
  phase is now dropped only when it governs a task section and every one is
  dropped. Spec 041.
- **`write-analysis` quotes what a plain YAML scalar would not read back.**
  An `unexamined-by-reason` key is flattened to one line, and it and the
  `spec`, `last-run` and `analyzed-against` values are quoted whenever a
  plain scalar would not read back — one holding a colon and a space, or
  starting with `#` — so a reason carrying a newline or a colon can no
  longer inject a frontmatter key or leave a record the next run refuses.
  Its `replaced` result reports whether an `analysis.md` already existed; it
  looked for an `analyze:` block in `spec.md`, which the primitive no longer
  writes. Spec 041.

## [0.55.0] — 2026-09-28

Updates now track the latest release rather than `main` (spec 061). The
release also carries rule-file and constitution changes from specs 008, 016,
036, 050 and 058. The runtime's code is unchanged apart from a doc comment, a
test and one lockfile entry. It is published at 0.55.0 anyway, because the
version pin must always name a release whose assets exist.

### Changed

- **`/ductus` and the installer fetch from the latest release by default.**
  Each run fetches from one ref: the latest `ductus-v*` release (read from the
  `Location` header of GitHub's `releases/latest` redirect), `main`, or a
  named tag. The bootstrap self-update, the runtime version pin and the
  framework archive all use it. `--ref=<value>` chooses it (`latest`, `main`,
  or `ductus-v<SemVer>`), in any position. A run given `main` or a tag records
  it as `.ductus/config.toml` `[source] ref`, so later runs keep it, and
  `--ref=latest` clears it. The run halts before anything is written on any
  of these:
  - a resolved tag below this release, the first whose bootstrap honors a ref;
  - a tag below the `introduced_in` of the project's last applied migration;
  - a latest release that cannot be resolved;
  - an unknown or repeated `--ref`;
  - a tag that does not exist.

  The run reports its source, and the self-update's pre-flight abort names the
  source it updated from. The framework root is now derived from the
  extracted archive rather than spelled `ductus-main/`. Spec 061.
- **The installer is a release asset.** Every release uploads `install.sh`,
  and a post-release job compares the published copy byte-for-byte with the
  tag's. The documented one-liner is
  `curl --proto '=https' --tlsv1.2 -sSfL https://github.com/stonean/ductus/releases/latest/download/install.sh | sh`.
  The installer takes the same `--ref` flag and applies the release floor.
  Given `--ref`, it names `/ductus --ref=<value>` as the next command.
  Spec 061.
- **A framework-only change reaches default-source adopters at the next
  release.** A project on `--ref=main` still sees it on commit. Spec 061.
- **Every rule Statement carries one obligation tier.** The 16 shipped
  Statements that mixed MUST and SHOULD are resolved. Fourteen are split: the
  MUST clause keeps its ID and the SHOULD clause becomes a new rule
  (`BE-ERRENV-003`, `BE-PAGE-003`, `FE-LOAD-003`, `FE-LOAD-004`,
  `FE-FONT-003`, `BE-AUTHN-015`, `BE-AUTHN-016`, `BE-AUTHN-017`, `BE-ERR-004`,
  `BE-LOG-007`, `FE-XSS-009`, `FE-STORAGE-004`, `FE-CSP-008`, `FE-PII-004`).
  `BE-DEPS-001` and `FE-CSP-001` drop a SHOULD clause their own MUST already
  implied. MAY may appear beside either tier as a permission, and a runtime
  corpus test holds the shipped rule files to one tier each. X-Frame-Options
  is now a SHOULD in both rule files: `BE-API-001` no longer lists it, and the
  new `BE-API-012` mirrors `FE-CSP-008`. Spec 008.
- **`QUAL-TEST-001`: a test fails when the behavior it names is removed.** A
  new SHOULD rule in `quality-cross.md`, enforced by `/{project}:review`'s
  quality pass. It names the case where a fallback path satisfies the
  assertion meant for the primary one. `QUAL-GROUND-001` and `QUAL-DELEG-001`
  no longer claim a SHOULD finding leaves `done` unblocked. Spec 036.
- **`/{project}:analyze` reports a malformed or duplicate-ID rule file as
  blocking again**, and withholds that file's rules from steps 11 and 12.
  Spec 008.
- **The Applicable Rules check skips code-pattern rules.** A cited rule whose
  file declares code-pattern verification, as `quality-cross.md` does, is no
  longer flagged as not firing, because `/{project}:review` enforces it
  against code. Spec 016.
- **The analyze record reads the re-check.** A finding the re-check no longer
  produces is recorded `fixed`, whichever confirmed write removed it. `routed`
  marks a finding that still fires. Spec 058.
- **Constitution.** A status commit holds only the transition
  (§spec-lifecycle). Nothing durable goes in a review record's body
  (§implement-phase). A task append that writes nothing reports an outcome, so
  read its returned fields rather than its exit status (§scenarios). Spec 050.

### Fixed

- **`runtime/Cargo.lock` pinned `chacha20` 0.10.1, which was yanked.** It is
  now 0.10.2. No build target compiles it. Spec 061, task 21.

## [0.54.2] — 2026-09-27

### Fixed

- **`ductus exec analyze` asked the host about one rule per step, tiered by
  the step, and recorded none of the rest.** Steps 11 and 12 each sent one
  `assessSpecQuality` request, filled with the first cited rule or else the
  first rule in the loaded files and tiered by the step's prose, so both
  steps asked about the same rule — once as `must`, once as `should` — its
  verdict counted twice, and every other loaded rule went unasked while the
  record read as fully examined. The walker now loads the rule set once per
  walk and sends one request per loaded rule of a step's tier, each rule
  tiered by the RFC 2119 keyword in its own Statement (MUST when a Statement
  carries both), and the host judges whether each rule's trigger fires,
  answering `passed: true` when it does not. A rule with no keyword or no
  Verification is recorded under `rule-assessments-not-checked`, which now
  counts rules rather than requests, and a rule file the walk could not read
  under the new could-not-be-read reason `rule-file-unreadable`. One
  rule-section parser (`runtime/src/primitives/rule_sections.rs`), sharing
  `check-rule-ids`' heading grammar, replaces the walker's two. Spec 060.
- **A failed assessment whose finding named `info` or the empty tier counted
  in no tier.** Validation accepts both, and the tally took the finding's
  tier first, so such a failure left the record reading as though the rule
  had passed. It now counts in the asked rule's tier unless the finding
  names MUST or SHOULD. Spec 060.
- **`ductus exec clarify` asked only the first open question.** Clarify step
  6 promises one `askClarifyQuestion` round trip per question; the walker
  sent one per step and the builder fell back to the first. The step now
  fans out over `read-spec`'s `open-questions`, seeding each as the request's
  `question`, and a spec with none left asks nothing and says so. Spec 022,
  scenario `exec-clarify-asks-each-open-question`.

## [0.54.1] — 2026-09-27

### Fixed

- **A project in a subdirectory of its repository read git as though it sat
  at the repository's root.** Git names every path in its trees, index,
  status and diffs from the work tree, while a project names its files from
  its own root, and the two differ by the project's directory inside the
  repository. Ten runtime readers looked a project path up in git, or opened
  the project root as the repository, and every one failed silently there.
  Each now opens through one helper, `ProjectRepository`
  (`runtime/src/primitives/mod.rs`), which discovers the repository from the
  project root, places the project in its work tree, and converts paths on the
  way in (`to_git`) and on the way out (`to_project`, which drops a path
  outside the project). A project at the repository's root reads exactly as
  before. Spec 059.
  - `check-stuck` found no `in-progress` commit, counted nothing, and never
    reported a spec stuck.
  - `check-artifacts`' scenario-to-task walk found no `tasks.md` history, so a
    scenario whose task had been pruned was reported as never having had one.
  - `diff-cross-spec` and `derive-boundary` found no first spec-directory
    commit and answered as for a spec never committed: cross-spec impact
    unknown, and a boundary of the spec directory alone. Both now report
    every path from the project root and leave out another project's changes.
  - The exec payload's gitignore guard asked libgit2 about the project's
    name for a path, so the project's own `.gitignore` never applied — an
    ignored file under a name no secret pattern matches went into the payload
    — and a root pattern could refuse a tracked file it does not name.
  - `derive-dependencies` and `derive-references` could not open the index,
    so they walked the worktree: `--write` rewrote untracked drafts,
    `untracked-skipped` was always empty, and `--staged` rewrote nothing.
  - The rename exemption on review and analyze freshness missed every
    lookup, so a uniform repo-wide rename staled both records.
  - `check-corpus-links --scope repository` could not open the repository and
    blamed an unreadable index. It now examines the project's tracked
    markdown, and its guidance names both causes it can have: no repository
    contains the project, or its index could not be read.
  - `compute-review-scope`, which spec 058 had already converted, treated a
    project its repository's work tree could not place as the repository
    root and found no history. That case is now the no-repository error, as
    it is for every other reader.
- **An escaping plan path was refused under the wrong label in any git
  repository.** The exec payload asked libgit2 about gitignore before
  checking containment, and libgit2 answers "ignored" for any path containing
  `..`, so an Affected Files entry such as `../outside.txt` was refused as
  `.gitignore` rather than `out-of-repo`. The refusal held; the label named
  the wrong cause and contradicted 022's `writecode-payload-canonicalize-paths`.
  Containment now runs first: a missing path is skipped before git is asked,
  an outside one is refused as `out-of-repo`, and only a path inside the
  project is asked about gitignore before it is read. It predates spec 059,
  which found it.
- **The exec payload's secret and gitignore guards judged a plan path as the
  plan spelled it, not the file it reads.** Both now ask about the path's
  canonical target, named from the project root. An absolute spelling of an
  ignored file inside a subdirectory project reached git as `proj//<path>`,
  which no anchored pattern matches, so the file was read and shipped — a
  regression spec 059 introduced. A `.`, `..` or `//` spelling, and a
  harmless-named link to `.env` or to an ignored file, slipped past the same
  way; those predate 059. The `writeSpecBody` payload's existing-section
  reader, which joined the context's `feature` and `path` with no containment,
  now reads nothing outside the project.
- **A project whose path contains `*`, `?` or `[` had its specs matched as a
  glob, which matched nothing.** `diff-cross-spec` reported no cross-spec
  impact, and the untracked-spec listing behind `derive-dependencies` and
  `derive-references` reported none skipped, without either having looked.
  Every pathspec naming the project's spec tree is now literal, and the
  `--staged` listing is bounded to the spec tree rather than diffing the whole
  repository.
- **A git failure read as an empty answer.** The tracked, untracked and
  staged spec listings returned nothing when a repository was found but its
  index or status could not be read, and `compute-review-scope` treated an
  unreadable `plan.md` as one with no Affected Files. Each is now an error;
  an absent plan is still an empty list.
- **`check-artifacts`' scenario history walk could flag a scenario whose task
  it had not read.** A revision whose `tasks.md` could not be read or decoded
  was skipped, and a shallow clone was walked as though complete, so a task
  living only there was reported as never having existed. Both now fail safe
  to flagging nothing, as 000's `scenario-without-task-visibility` requires,
  and an unchanged `tasks.md` is scanned once rather than once per commit.
- **`check-corpus-links --scope repository` counted a conflicted file once
  per conflict stage,** reporting its broken links two or three times, and its
  empty-scope guidance named the spec root rather than the project's tracked
  files it had examined.

## [0.54.0] — 2026-09-26

### Added

- **A waiver's `file` may list several paths.** A rule waived in several
  files for one reason took one `review.md` waiver entry per file, each
  repeating the rule, the reason, `waived-at` and `waived-by`; the duplicates
  could not even be merged by hand, because a list in `file` failed the whole
  `waivers:` parse and halted `/{project}:review`. `process-waivers`,
  `write-review` and `invalidate-review` now read `file` as one path or a list
  of them, and each listed path is its own `(rule, file)` anchor: it applies,
  expires, is retained and is deduplicated on its own. `process-waivers`
  still returns one `WaiverRef` per anchor, so its result shape is unchanged.
  When an anchor expires, `write-review` removes only that path and drops the
  entry with its last one, keeping every other field and adopter extra; it
  writes one path as a scalar and several as a quoted block list, so a record
  that never used a list is byte-identical. An empty list or a blank path is
  `missing 'file'`, and a list item that is not a string fails the parse, as
  any mistyped field does. Paths are matched literally, never expanded as
  patterns. Specs 020 (`waiver-file-lists`) and 022
  (`process-waivers-file-lists`).

## [0.53.0] — 2026-09-26

Three specs ship together. Spec 058 makes every finding a run produces get a
disposition before its spec can reach `done`, and stops every writer but
`/{project}:log` from appending to the inbox. Spec 022 lets `ductus exec
analyze` complete on the session `/{project}:target` writes. Spec 051 stops a
fold owed to another tree from holding `done`.

### Changed

- **A review observation carries its disposition instead of being written to
  the inbox.** `write-review` no longer appends observations to
  `{specs-root}/inbox.md`. Each observation takes a `disposition` (`fixed`,
  `routed` with a `target`, `discarded` with a `reason`, or `undispositioned`,
  the default) and an optional `decision-key`, and the record counts the four
  outcomes in a `dispositions:` map. An observation newly routed or discarded
  is stored in a `decisions:` list keyed on its rendered line, which needs
  `decided-by`; a re-matched decision keeps its original stamp, and each entry
  in `expired-decisions` is dropped.
- **`write-analysis` takes the findings, not the captured inbox bullets.**
  `findings` replaces `captured-issues`: each finding carries `tier`,
  `family`, `message`, `path`, `live`, a `disposition`, and an optional
  `decision-key`. The record gains the same `dispositions:` map and
  `decisions:` list, keyed `{family} — {message}`. `undispositioned` is the
  live tier total less the live findings routed or discarded, so a finding the
  caller leaves out counts as owed, never as handled. A discard on a
  `hard-fail` or `blocking` finding is refused before anything is written, and
  so are two findings that share a key but not an outcome, in either writer. The
  body renders `## Fixed in this run` in place of `## Captured issues`, and a
  `{n} finding(s) not itemized` line wherever a tier count exceeds the
  findings listed under it.
- **`check-review-gate` blocks on findings nobody decided.** Two checks run
  after every review and analyze check: `record-predates-dispositions`, for a
  `review.md` or `analysis.md` with no `dispositions:` map, and
  `undispositioned-findings`, for a record whose `undispositioned` count is
  above zero. Absence is not zero: a record written before this release blocks
  an `in-progress` spec until its command runs again. `invalidate-review`
  removes the `dispositions:` map with the run's other counts.
- **`check-artifacts` swaps `analyze-state-drift` for `disposition-drift`.**
  The new family reports a `done` spec whose `review.md` records
  undispositioned observations, and is silent for a record with no map. No
  family judges `analysis.md` any more: `/{project}:analyze` judges it from
  the record it has just written, because a finding read from the record a run
  is about to replace could never clear. The count stays at nine families.
- **The standing inbox count moved to `dashboard`.** `dashboard` returns
  `inbox-standing` and renders an `Inbox:` line on every run, in four states:
  outstanding with the oldest item's date, outstanding with its age
  undeterminable, clean, or no readable inbox. The age dates each surviving
  item by its text against `HEAD`'s blame. It is undeterminable, never today,
  when no item is committed or when any item sits behind a shallow clone's
  cut.
- **`ductus exec analyze` records what it detected, and what it did not
  examine.** It used to write 0/0/0 whatever its detection steps found. The
  walker now tallies each detection step's findings into the tier counts
  `write-analysis` records, and records under `unexamined-by-reason` what it
  never looked at: steps 13–15, which it does not run
  (`references-not-checked`, `applicable-rules-not-checked`,
  `grounding-not-checked`); citations checked against no rule file
  (`rule-citations-not-checked`); referrers it could not read
  (`referrer-unreadable`); and assessments whose request carried no rule
  (`rule-assessments-not-checked`), whose verdicts count in no tier. An
  assessment's tier is read case-insensitively, and a failed assessment with no
  finding counts in the tier of the rule it asked about. An exec run itemizes
  no finding, so every live finding is recorded undispositioned.
- **`ductus exec analyze` completes on a session `write-session` wrote.**
  Spec-reading steps take the spec file when the session's `path` names the
  spec directory. When the session carries no `rule-files`, the walker binds
  every rule file in the rule-file directory, whatever the project's surfaces.
  When it carries no lint `paths`, it binds the feature directory's markdown.
  A seeded value is used as given (spec 022).
- **A fold owed to another tree no longer holds `done`.** `check-review-gate`'s
  `pending-fold` check blocks only when the `folds-into` target resolves in
  this tree, as a feature directory holding a `spec.md`, which is fold-back's
  own test. A target on another line passes, and the checks after it decide.
  The fold stays owed: `check-unfolded-specs` and `dashboard` keep reporting
  the spec, and `retire-feature` folds a `done` staging spec like any other
  (spec 051).
- **One constitution resolver.** The exec analyze binding and `writeCode`'s
  constitution excerpts both read `.ductus/constitution.md`, then
  `framework/constitution.md`, so an adopter's constitution is the one that
  loads.

### Added

- **`process-decisions`.** A read-only primitive that classifies the
  decisions stored in `review.md` or `analysis.md` against a run's finding keys
  before any disposition is proposed. Each is `matched` (counted under its
  stored outcome without asking again), `expired` (the finding stopped firing,
  so the writer drops it), or `retained` (the run was `restricted` and may not
  have looked). A malformed entry or a repeated key produces a notice. A list
  that does not parse is an error, never an empty list.
- **`append-task` `dedup-title`.** When set, a pending task with the same title
  and body is returned with `appended: false` instead of being appended again,
  so an interrupted `/{project}:implement` records an out-of-spec finding once.
- **`validate-frontmatter` names an unparseable `decisions:` list** in either
  record as a hard failure.

### Removed

- `write-review`'s `captured-issues` argument, frontmatter field, and
  `## Captured issues` section, and its `observations-captured` and
  `inbox-standing` result fields.
- `write-analysis`'s `captured-issues` argument and section.
- `compute-review-scope`'s `captured-issues` window, and `diff-cross-spec`'s
  `inbox-additions` and `inbox-standing`.
- `append-inbox`'s `dedup-prefix` argument and `deduped` result, whose only
  callers were the removed capture paths.

A record written before this release still parses. `captured-issues` is
dropped the next time the record is written.

### Fixed

- **A record could be written with a value its own reader rejects.** A
  single-line argument carrying a C0 or C1 control character, U+0085, U+2028,
  U+2029, U+FFFE, or U+FFFF is refused, or flattened where the writer takes
  free text. `yaml_string` quotes any value whose unquoted form would not read
  back identically: a value holding a tab before `#` was written unquoted and
  read back truncated.
- **A frontmatter block with no closing fence was reported as missing.** It is
  named `UnclosedFrontmatter`, and a closing fence that ends the file is
  accepted.
- **`compute-review-scope` found no diff base for a project in a subdirectory
  of its repository,** and reported `modified-since` repository-relative. Both
  now resolve against the git work tree, and changes outside the project are
  dropped.

## [0.52.1] — 2026-09-18

### Fixed

- **`criterion-path-existence` flagged a criterion asserting that a path was
  never created.** The `not-a-live-claim` exemption covered a path that
  existed and was *removed* — `deleted`, `no longer exists`, `is renamed to`
  and eleven more — but had no member for one asserting a path had **never
  been brought into existence**. A `done` spec's criterion reading ``…no
  nested `server/.git/` repository was created`` was reported as naming a path
  that no longer resolves, which is the finding exactly backwards: the
  criterion is satisfied *because* the path does not resolve.

  It could not be fixed by appending to the phrase list. Matching is a flat
  substring test over the whole criterion, so `was created` — the only phrase
  spanning every criterion of this shape — would have exempted positive
  delivery claims too (``the migration file `db/migrate/…` was created``) and
  blinded the family, which is strictly worse than the false positive. The
  narrower `was never created` does not over-exempt but misses the reported
  word order, where the negator is separated from its verb by the noun it
  negates.

  A sixth exemption group now carries the construction as a clause-scoped
  predicate: a criterion is exempted whole when one of its clauses holds a
  negator (`no`, `not`, `never`, `without`) **before** a creation verb
  (`created`, `added`, `introduced`), matched as whole words. Clauses split on
  `;`, `,` and a period *followed by a space* — never the bare character,
  which would cut `server/.git/` and `master.key` between the negator and the
  verb. Both word lists are closed and framework-fixed, like the phrases, and
  `/ductus:audit` Family 18 now binds them across all three restatements.

  Precision only: no criterion that was previously exempted becomes a finding,
  and the phrase count is unchanged at fourteen.

## [0.52.0] — 2026-09-17

### Fixed

- **A review finding whose severity was not exactly `must` was silently filed
  as a SHOULD.** `write-review` bucketed findings with the severity test last
  in an `if`/`else` chain whose `else` was the catch-all, so a typo, or a
  value borrowed from the analyze vocabulary, landed in the SHOULD bucket;
  `blocking` was then written `false` and the spec passed
  `check-review-gate`'s MUST check. `finding_rank` shared the same test, so a
  misspelled MUST also lost cross-pass dedup to a correctly-spelled SHOULD
  and the weaker copy survived.

  The reachable path was never a source typo: `severity` arrives from the
  `performReview` extension point, written freehand by an LLM into JSON on
  every review run. An unvalidated permissive default sat directly on the
  path that gates `done`.

### Changed

- **Severity is a closed set, not a string.** The three vocabularies are now
  bound as three types in `schema/severity.rs`: review (`must`, `should`),
  analyze (`hard-fail`, `blocking`, `advisory`, `informational`) and the
  `assessSpecQuality` rule tier (`must`, `should`, `info`, and `""` as a
  named `Unspecified` state). They stay separate deliberately — one merged
  enum would make the cross-vocabulary error representable in the type
  introduced to forbid it.

  An unrecognized value is **rejected**, never defaulted, naming the
  extension point, the vocabulary, the offending value and the legal set:
  `schema-mismatch in`performReview`: unrecognized review severity
  "mandatory" — expected one of: must, should`. Bucketing is a `match` over
  the closed set, so there is no catch-all arm left for a bad value to reach
  and adding a tier is a compile error rather than a silent demotion.

  **Case-insensitivity is preserved** — `MUST` was accepted before and still
  is, since that is the spelling the rule files use; strictness targets
  unrecognized values, not capitalization. Serialization is unchanged, so
  every `review.md` and `analysis.md` is byte-identical across the change and
  no consumer's parsing moves.

## [0.51.0] — 2026-09-16

### Changed

- **`validate-frontmatter` names the constitution's tier per finding**, rather
  than one tier for the whole family. Eight findings emitted `blocking` where
  [§text-first-artifacts](../framework/constitution.md#text-first-artifacts)
  (Validation Severity) says Hard fail — frontmatter that is not valid YAML,
  frontmatter that is not a mapping, `status` missing / non-string / outside the
  allowed set, and `dependencies` missing / not a list / holding a non-string
  entry. Those now emit `hard-fail`. The five genuinely-Blocking findings are
  unchanged: the `folds-into` and `cross-spec-impact` shape checks, and a
  residual `review:`/`analyze:` record block, which the constitution classifies
  Blocking *precisely* because the spec file itself parses.

  **The consumer moved in the same release**, because either side alone
  re-flattens the other. `framework/commands/analyze.md` step 2 claimed the
  primitive emits every finding as `blocking` — untrue since 0.50.0 added two
  `hard-fail` sites — and told the host to render *all* frontmatter findings in
  the hard-fail tier, mis-tiering the residual-block finding in the opposite
  direction from the primitive. The host now renders each finding in the tier
  the finding names.

  Both tiers already block pipeline advancement, so no spec that validated
  before fails now; what changes is which report section a finding lands in and
  how a run's tier counts split.

### Fixed

- **Three operator-facing messages carried runs of 14-18 spaces mid-sentence**,
  left behind where backslash line-continuations were collapsed:
  `check-artifacts`' artifact-unreadable finding, and `check-review-gate`'s two
  cross-spec-impact guidance strings.

## [0.50.0] — 2026-09-16

### Added

- **`relocate-audit-records` — each audit record moves to the artifact that
  owns it.** The review record lived in two places, `spec.md`'s `review:` block
  and `review.md`'s own frontmatter, agreeing on seven of nine keys and held
  together by nothing. They drifted: specs 031 and 041 carried
  `should-violations: 1` in the block while their reports recorded `0`, for
  weeks, because every gate reads exactly one of the two files. The analyze
  record had the opposite problem — no artifact at all, so its per-path digest
  map sat in spec frontmatter that averaged 37.5 lines across the corpus.

  `review.md` now carries the whole review record and a new `analysis.md`
  carries the analyze record. This primitive performs the move, per spec:
  merging the block into an existing artifact (the normal case for `review.md`,
  which every pre-migration spec has), carrying the existing report body
  verbatim, and **naming** any key where the two copies disagreed rather than
  resolving it silently. Idempotent — a converged spec reports
  `changed: false`, so an interrupted sweep is resumable.

### Changed

- **`Frontmatter` no longer carries `review` or `analyze`.** `read-spec` still
  returns both, composed at read time from their artifacts, so the interface is
  unchanged while the storage moved. `check-review-gate` reads each record from
  its owner and now distinguishes **absent** (no run) from **unreadable** (a
  damaged record it cannot judge) — telling an operator to re-run a review that
  may already have happened is a claim the artifact never supplied.
- **`validate-frontmatter`** reports a `review:` or `analyze:` block still
  present in a spec, at the Blocking tier the constitution now declares for it,
  and hard-fails a record artifact that exists but carries nothing readable.
- **The analyze staleness digest excludes `analysis.md` outright**, and
  `spec.md` is digested whole — so a frontmatter edit there now moves the digest
  where the old block-excision hid it. Moving the excision to `analysis.md`'s
  frontmatter was the first cut and was **not enough**: `write-analysis` rewrites
  that file's report *body* in the same call, so any run whose Summary or counts
  differed from the previous one digested a body it was about to replace and
  reported itself stale — two runs to record one analysis. The record artifact is
  its own command's output, exactly as `review.md` is `/{project}:review`'s and is
  excluded from the review digest for the identical stated reason, and nothing
  reads its body. `review.md` stays an analyze subject: it is an analysis's input,
  and excluding it would exempt the edit that most often invalidates the record.
- **`check-review-gate`'s tool description and CLI help** enumerate the gate's
  nine checks and name `review.md` and `analysis.md`. Both still described a
  two-check gate reading a `spec.md` block — the most visible place for a wrong
  location, since they render to an operator.

### Removed

- The frontmatter block-splicing machinery — `splice_review_block`,
  `splice_top_level_block`, and the spec-side record renderers — which existed
  only to maintain the second copy.

## [0.49.8] — 2026-09-15

### Added

- **`check-promotion-coverage` — how much of a rules file has been routed to
  its canonical source.** §drift-prevention's *Shared knowledge stays in git*
  routes a learning by **population** and names the constitution first, but
  nothing reported how much had actually gone there. *Not having promoted* is
  invisible by construction: a universal entry still sitting in the project's
  own rules file looks exactly like one correctly judged project-only, because
  neither carries a marker and a per-entry marker is the authored input
  §design-principles rejects.

  The primitive computes
  `unclassified = rule-bearing − (table-keyed ∪ pointer-citing)` over committed
  markdown, so it needs no authored input, and it reports **all four terms**
  rather than the difference alone. That is not verbosity: the figure has been
  re-derived by hand repeatedly and disagreed with itself every time, and never
  once because of decay — each disagreement was a difference of *method*.
  Counting the pointer side as anchor-bearing lines in the whole file rather
  than as rule-bearing bullets in the named sections gives two honest answers
  to one question, and a matcher that does not strip a trailing parenthetical
  from a table key fails to match every key carrying one, reporting a coverage
  gap that is an artifact of the instrument. The terms are what those
  disagreements turned on.

  `sections` is required with no default, because which sections carry rules is
  a property of the project's own file; a named section that does not occur is
  reported in `missing-sections` rather than contributing zero bullets, since a
  denominator that quietly shrinks when a section is renamed moves coverage in
  the flattering direction. `unmatched-keys` is load-bearing rather than
  diagnostic — it is how a **reworded** entry surfaces, a lead-phrase key
  degrading loudly to no match where a positional identifier would degrade
  silently to whichever entry later occupied its slot. `table-state` separates
  `no-table` from a computed zero, and a `table-file` that was supplied and
  cannot be read is an **error** rather than the absent state, since treating
  it as absent would report full coverage over a file nobody read.

  Bullet counting reuses the shared comment- and fence-aware grammar the inbox
  primitives already use, and keeps the **top-level** constraint local with the
  reason recorded on both sides: that helper trims leading whitespace because
  it was written for inbox bullets where nesting does not occur, so delegating
  wholesale would have widened this count to nested list items. That is
  `QUAL-DELEG-001`, the rule added in the same session, applied to its own
  first case.

- **`/ductus:audit` Family 38 — promotion coverage.** A shell entry point that
  calls the primitive rather than reimplementing the count, per §runtime-boundary
  principle 3. The coverage line goes to stderr and **never affects the exit
  code**, which is Family 19's existing shape: promotion has to stay free for
  the reason §brownfield-inbox gives for capture, and this suite is a hard
  release gate. Two things *are* findings, and both are drift rather than
  coverage — a classification key matching no entry, and a named section absent
  from the rules file. Proven in both directions before being trusted: a bogus
  key and a renamed section each take it to exit 1, and the renamed-section
  probe dropped the denominator 121 → 118, which is the silent shrink the
  finding exists to catch.

  Maintainer-only by design. The inbox row renders in `/{project}:review` and
  `/{project}:implement` because every project is scaffolded with an inbox, so
  silence there would be wrong; a classification table is opt-in and
  maintainer-owned, so an adopter would need configuration invented for them to
  see anything but silence. `resolve-constitutions` sets the precedent for the
  opt-in case: report nothing, and a project without the feature reads exactly
  as before.

### Fixed

- **A history note claimed a release did something it did not.** Spec 022's
  `write-session-primitive` scenario records the sequence of session-file path
  choices, and a rename sweep rewrote 0.10.0's destination to the path that
  only arrived two specs later — so the paragraph asserted that the 0.10.0
  consolidation moved the file somewhere it did not. Repaired by naming
  0.10.0's actual destination and stating why the retired spelling stays: a
  sweep must not rewrite the record of the thing being swept, which is the one
  case §drift-prevention exempts from a blanket substitution. The pre-sweep
  bytes were **not** restored wholesale, because they also named a real adopter
  project, which the sweep removed correctly.

## [0.49.7] — 2026-09-15

### Changed

- **Three doc comments named a slash command that no longer exists.**
  `/{project}:init` was retired from the framework — it was a single
  hand-maintained command file with no `framework/commands/` source, so no
  generator rewrote it and no sweep reached it, and `/ductus` already covered
  the greenfield case it served. `interpreter::payload` and
  `primitives::mod` each described the installed adopter layout
  `{specs-root}/templates/{file}` as what that command scaffolds, and a
  `derive_boundary` test comment named it among the commands that run before
  a repo's first commit. All three now name `/ductus`, which is what ships
  the six spec templates into `{specs-root}/templates/` (the **Shared Files**
  manifest rows) and creates the directory.

  Comment-only: no behaviour, no result field and no shared parser changes,
  so this carries no scenario under `022-deterministic-runtime`. The release
  exists because a `runtime/` edit reaches adopters only through a tag, and
  an unreleased pin on `main` halts acquisition.

### Fixed

- **The sweep-parity conformance test passed by corpus accident and was one
  re-review away from failing permanently.**
  `rust_and_family_19_agree_on_every_done_spec` asserts that the Rust
  staleness rule and Family 19's Python answer alike, and its subject is the
  set of `done` specs whose durable contracts differ from their
  `reviewed-against`. Measured across the corpus: **exactly one spec has ever
  supplied that subject** — `020`'s `data-model.md`, stale since its recorded
  base but sweep-exempt, so Family 19 stayed green while the test stayed
  non-vacuous. Its vacuity guard therefore asserted that an *unhealthy* corpus
  must exist for the test to mean anything, and re-reviewing that one spec
  emptied the set and turned the guard red. Proven by probe in both
  directions before the fix: flipping `020` back to `done` made it pass,
  reopening it made it fail.

  `rust_and_family_19_agree_on_a_built_sweep` now carries the guard. It builds
  its own repository — three durable contracts under a uniform
  `govern` → `ductus` substitution, with a structural edit added to one in the
  second case — and asserts both implementations against a known expected
  verdict in **both** directions: a pure repo-wide sweep exempts everything it
  touched, and one structural edit reads stale without costing the other two
  their exemption. Three contracts rather than two because the exemption turns
  on a pair being repo-wide, so a two-file fixture would leave the surviving
  substitution unexplained and prove nothing about the exempt path.

  The corpus pass keeps running and keeps failing on any disagreement, but an
  empty comparison set there now reports *every review is current* instead of
  failing — a healthy corpus and a check that could not run must not render
  alike (§design-principles), which is the same rule the guard was written to
  serve and was, in this one place, breaking.

## [0.49.6] — 2026-09-15

### Security

- **`rustls` advanced 0.23.41 → 0.23.45 for RUSTSEC-2026-0285** (*TLS 1.3
  handshake messages incorrectly accepted across encryption level
  boundaries*, 5.3 medium), pulling `aws-lc-rs` 1.18.1, `aws-lc-sys` 0.45.0
  and `rustls-webpki` 0.103.15. Lockfile only; `rustls` reaches the tree
  transitively through `reqwest`. The advisory was published 2026-09-14 and
  `ductus-v0.49.5` shipped before it landed in the database, so that release
  links the affected version.

  Note for anyone repeating the bump: a plain `cargo update -p rustls` stops
  at 0.23.43, which is **still affected**, because the MSRV-aware resolver
  caps at the latest version compatible with the declared `rust-version`. It
  reports success, so taking it at face value leaves the advisory live while
  looking fixed. `--precise 0.23.45` is required, and is safe here — all four
  updated crates declare `rust-version 1.71`, under this crate's 1.88.

### Fixed

- **The two frontmatter generators derived no dependency edge for a
  branch-scoped or four-digit sibling.** `derive_dependencies::leading_slug`
  and `derive_references::is_spec_slug` each carried their own copy of the
  feature-directory grammar at *exactly* three digits, disagreeing with
  `parse_feature_dir` — the single membership rule spec 051 established — in
  both directions. A body link to `1234.1-slug` or `1000-slug` harvested
  nothing, silently: an absent edge is indistinguishable from a spec that
  cites nobody. `traverse-deps` inherited the gap through the frontmatter it
  reads, and because `scan_line` also backs the pre-`done` cross-spec-impact
  back-link matcher, a declared impact on such a spec could never be
  discharged.

  Both now delegate the *form* to the shared grammar while keeping the
  charset and the candidate boundary local, and that split is the durable
  lesson. `parse_feature_dir` deliberately leaves a sequential slug's charset
  open because it recognizes directories that already exist on disk; these
  two call sites parse untrusted link text and write it verbatim into
  frontmatter, which is the opposite trust context. Delegating wholesale let
  a crafted target splice a second key onto the `dependencies:` line, and
  terminating the candidate on `/` alone then dropped a legitimate
  directory-only sibling link. Both were introduced and caught inside the
  same review pass, both by probe rather than by reading, and both are pinned
  by tests carrying the probes' own inputs.

- **`write-review` let the `empty-scope` flag decide the denominator.**
  `resolve_scope_size` short-circuited to `0` on the flag instead of deriving
  the scope, and the realistic way to pass it wrongly is an over-cap
  `compute-review-scope` call — 8 of 54 specs' payloads exceed 100KB — which
  errors with no result and reads as *nothing in scope*. The resulting
  `0/0/0`, `scope: 0`, `examined`-absent, non-blocking record was invisible
  to every gate: Family 31's `examined` arms both guard on `total > 0`,
  Family 19 sees a valid digest, and `check-review-gate` sees a non-blocking
  review. The denominator is now always derived, which closes it with **no
  new check** — a false empty scope carries the real `scope: N` beside a zero
  `examined`, the shape Family 31 already reports. The flag keeps its
  Summary-rendering job.

### Added

- **`write-analysis` records `captured-issues`.** `/{project}:analyze`
  records how many findings a run *produced* as `advisory`, while the inbox
  appends that persist them are separate calls — so a run that recorded
  `advisory: 5` and captured nothing was byte-identical on disk to one that
  captured all five. The review side has an agreement check holding
  `review.md` against its own frontmatter; analyze has no counterpart,
  because it writes no report artifact for one to compare against. The count
  is **recorded, not derived**: `append-inbox`'s dedup guard legitimately
  suppresses a re-append, so a correct re-run captures fewer than it found,
  and forcing the two to agree would restore the same conflation from the
  other side.

## [0.49.5] — 2026-09-14

### Changed

- **The placeholder project name used across the runtime's tests, fixtures
  and doc comments is now `acme`, and no artifact here names a real adopter
  project.** `AGENTS.md` §Workflow requires that another project's name is
  never recorded in this repository — describe the shape instead — while
  sanctioning a purpose-built fixture as the replacement. The fixtures were
  named after the adopter they were derived from, so the rule appeared to
  bless and forbid the same bytes, and the ambiguity had been re-opened in
  several grooming passes without being settled.

  The two halves turn out not to be independent, which is what settles it.
  Most occurrences were the name used as an arbitrary value for the `project`
  config key — in `#[cfg(test)]` modules, in the `exec-auggie` and
  `exec-opencode` fixture trees (both as a `.govern.toml` value and as a
  command-directory segment), and in the `parity/install` golden — where any
  string works identically. But a handful named the adopter *as an adopter*
  in prose, and one of those was a doc comment on a `clap::Args` field, which
  renders as `--help` text: `ductus migrate-session-file --help` printed the
  adopter's name in the shipped binary. That single attributing site is what
  stopped the rest from reading as an arbitrary placeholder. With the prose
  sites rewritten to describe the shape, the remaining value is genuinely
  meaningless, and `acme` is the placeholder this repo already uses in
  `framework/migrations/session-file-consolidate.md`.

  No behaviour changes. The two fixture command directories were renamed with
  `git mv` and every reference to them moved in the same commit, including
  two `CHANGELOG.md` entries that cite the fixture path and would otherwise
  have been left pointing at a directory that no longer exists. Historical
  entries that attributed a report or an observation to the named adopter now
  state the reproducing conditions instead, which is what a reader needs and
  what survives outside one maintainer's machine.

## [0.49.4] — 2026-09-13

### Fixed

- **`check-rule-ids` reported no denominator, so a real miss and an empty
  rule-file list were byte-identical.** `run` builds its `known` map by
  iterating `args.rule_files`, so an empty list means nothing is known and
  **every** cited ID falls through to `missing` — and `/{project}:analyze`
  step 5 treats `missing` as blocking. An unguarded invocation therefore
  yielded blocking findings against a correct spec and invited someone to
  "fix" it by deleting a valid citation. `CheckRuleIdsResult` now carries
  `examined`, the count of rule files read: `examined: 0` says *nothing was
  checked against*, while the same `missing` list over `examined: 11` says
  the IDs really are absent. That is `QUAL-CLAIM-001` applied to the
  primitive that enforces rule citations, and it brings it into line with
  every sibling — `resolve-constitutions`, `derive-dependencies`,
  `check-corpus-links` and `check-orphaned-references` all already reported
  a denominator. The trigger was a shell trap rather than carelessness:
  under zsh an unquoted parameter does not word-split, so repeated flags
  built as a string arrive as one argument or as none.

- **`ships_to_adopter` failed to match a candidate written with a trailing
  slash, so a reference to a directory this project ships into an adopter's
  checkout was reported as local breakage.** The helper built its
  directory-prefix test as `format!("{candidate}/")`, so a candidate already
  ending in `/` became `specs/rules//` and matched no destination. The
  exclusion itself was never missing — only its normalization. The candidate
  is now trimmed inside the helper, which is where it belongs: the two call
  sites reach it differently, and only one was affected.
  `criterion-path-existence` passes an already-trimmed span and was never
  reachable by the defect; `check-orphaned-references` passes the raw target
  and was the live path, reporting `AGENTS.md → specs/rules/` on every run
  in this repo. It now reports none.

- **Two `[services]` aliases whose `repo` differed only by a trailing `/` or
  `.git` were one service to the reference harvester and two to the
  duplicate detector, so an ambiguous registry resolved to a confidently
  wrong status with no signal.** `derive-references` keys its registry on a
  normalized URL, so the alphabetically-later alias silently overwrote the
  earlier one; `Services::duplicate_repos` grouped by the raw string and
  reported no duplicate, and its test covered only the byte-identical case.
  A body link written against one spelling therefore resolved its status
  from whichever checkout won, while `030`'s data-model calls a duplicate
  repo a registry-validation finding and `/{project}:link` promises a
  warning at registration. Both halves now share one identity function,
  `schema::services::normalize_repo`, which is the actual fix — the defect
  was two notions of "the same repo", not a missing check.

## [0.49.3] — 2026-09-13

### Fixed

- **The `[constitutions.*]` registry accepted entries the bootstrap says it
  rejects, and reported one of them with a confidently wrong reason.**
  `framework/bootstrap/ductus.md` §Validating the registry states that a
  malformed entry halts per `CFG-ENV-003` naming the alias and field — an
  alias that is not a bare TOML key, a `repo` that is not URL-shaped, or an
  empty `path` — while a `path` that does not *resolve* only warns.
  Nothing enforced the halt: `schema::constitutions`' `from_toml_str` did
  TOML parsing and serde required-field enforcement only, and
  `resolve_constitutions::classify` reads the filesystem without inspecting
  values. Probed against the `0.49.2` binary, `repo = "not-a-url-at-all"`
  over a resolvable checkout resolved `loaded` with no complaint, and
  `path = ""` resolved to the repository root and was reported
  `no-constitution-document` — telling the operator their checkout lacks
  the document when the mistake is an empty path in their own config, the
  exact conflation spec 055's Edge Cases forbid between the two failure
  states. The three value checks now run inside `from_toml_str`, so a
  registry built from a config document is validated by construction rather
  than by a caller remembering to ask; `resolve-constitutions` surfaces a
  rejection as its own error naming the alias and field, distinct from a
  TOML parse failure. Only the first violation is reported, in a stated
  order (alias order, then `alias` / `repo` / `path`), because the entry
  halts. A `path` that does not resolve is untouched and still warns as
  `not-checked-out`: validation is pure value inspection and reads no
  filesystem, which is what keeps a project-config mistake distinguishable
  from a contributor who has not cloned the governance repository yet.
  `write-review` and `write-analysis` keep their existing posture and
  record the unreadable registry rather than failing, so a config typo
  never costs an operator their findings.

  The URL shape check delegates to the parser the crate already depends
  on and already uses for the same question in `fetch-archive`'s SSRF
  screen, rather than hand-rolling a second one: a `repo` is URL-shaped
  when that parser accepts it and reports a host. Two consequences are
  worth knowing. `file://` is **not** URL-shaped — the parser normalises
  `file://localhost/…` and `file:///…` to no host — and `repo` is the
  remote identity anyway, with `path` as the local half. And
  `https:///acme/gov` **is** accepted, resolving a host of `acme`, since
  the parser skips the extra slash exactly as a browser does.

  The offending alias is escaped where the error renders it. A TOML
  quoted key may carry a newline and a non-bare key is precisely what
  the first check rejects, so a raw rendering would let committed config
  forge a second line in an operator-facing message; a bare alias reads
  unchanged. Spec 022, scenario
  `the-constitutions-registry-validates-its-values`.

## [0.49.2] — 2026-09-13

### Fixed

- **Two `/audit` families resolved only two of the three config/session
  tiers, so a `.govern/`-layout subject escaped them silently.**
  `schema::paths`' `CONFIG_CHAIN` / `SESSION_CHAIN` have been three tiers
  (`.ductus/` -> `.govern/` -> legacy root) since 049, and both families
  claimed to mirror them. Family 12 (`fixture-session-shape.sh`) matched
  `.govern.session.toml` and `*/.ductus/session.toml` but not
  `*/.govern/session.toml`, so a fixture on the 049-era layout was found by
  none of its three checks (12a parse, 12b camelCase keys, 12c git-tracked)
  while the family still exited clean — its own header claims it verifies
  "every fixture session file". Family 17 (`host-namespace-parity.sh`)
  probed `.ductus/config.toml` then `.govern.toml`, skipping the middle
  tier, while its comment said it resolves "the way `Host::load` does";
  on a `.govern/`-layout repo it found no config and fell back to the
  directory basename, which can emit a false finding or mask a real one.
  Neither bit today — no fixture uses the middle tier and this repo is on
  `.ductus/` — which is precisely why nothing had surfaced them. Both are
  the `QUAL-CLAIM-001` shape §design-principles names: a check that cannot
  run must never be indistinguishable from one that passed.

### Changed

- **`check-artifacts`' result documentation enumerated eight families where
  the primitive owns nine.** `ArtifactFinding::family` listed eight and then
  ended mid-sentence on a dangling "or" with nothing after it;
  `CheckArtifactsResult::findings` said findings come "across the eight
  families" and its parenthetical listed eight; and `ArtifactFinding::severity`
  — a third site not previously recorded — omitted `analyze-state-drift` from
  its `blocking` list. All three now name `analyze-state-drift`, verified
  against the `family:` literals `check_artifacts.rs` emits. These are
  `JsonSchema`-derived doc comments, so they are served as MCP tool-schema
  descriptions and reach adopters; this is the same eight-for-nine defect
  0.49.1 fixed in the `check-artifacts` tool description, at the sites that
  fix missed.

- **`retire-feature`'s `# Errors` contract described pre-052 behaviour.**
  It stated that `InvalidArgument` fires "when `feature` is not the
  branch-scoped form" unconditionally. Spec 052's `allow-sequential` gate made
  that conditional — the module doc documents the gate at length, but the
  function's own `# Errors` block, which is what a caller reads, still
  described the ungated refusal.

- **Doc comments across the runtime name the resolved config/session file
  rather than a single legacy tier.** Continues the 0.49.1 sweep into the
  sites it did not reach: `main.rs`'s session-seeding comment,
  `discover_rule_files.rs`'s provenance-tag doc, four `schema/primitives.rs`
  result/args docs, `primitives/mod.rs`'s `Toml` error variant,
  `schema/mod.rs`'s module map, `schema/services.rs` (module doc plus two
  parser docs), `dashboard.rs` (module doc, error contract, config shape,
  session reader), and `resolve_references.rs` (module doc, error contract,
  and `load_services`, which also claimed the config is read "from the repo
  root" — which the primary tier is not). No behaviour change. The
  legacy-layout test fixtures and the migration records that rename *to* the
  legacy name are deliberately untouched; the two test module docs now say
  their fixtures use the legacy tier on purpose, which is what proves the
  fallback still resolves.

## [0.49.1] — 2026-09-13

### Fixed

- **A corrupt session file no longer resolves a silently-wrong agent.**
  `Host::load_session_cli_config_dir` read the per-contributor session file
  and returned `None` for *any* failure — missing, unreadable, or malformed —
  while its sibling `load_host_block` logged a warning to stderr on exactly
  the same parse failure. The asymmetry was the defect: a corrupt session file
  was indistinguishable from an absent one, so resolution fell through to the
  default `.claude` and the mistake surfaced far from its cause, as a
  "command file not found" for an agent the contributor does in fact use.
  A missing file stays silent — not having one is the ordinary state — while
  a file that exists and does not parse now names itself and the parse error.
  Behaviour is otherwise unchanged: the fallback chain and its existing test
  (`malformed_session_falls_back_to_legacy_then_default`) are untouched. This
  is `QUAL-CLAIM-001` — *a fully-implemented path whose output overstates what
  it verified* — found reviewing spec 049.

### Changed

- **`host.rs` documentation describes the resolved ladder rather than its
  oldest tier.** Its module doc and four function docs still said the runtime
  reads `project` from `.govern.toml` and `cli-config-dir` from
  `.govern.session.toml`, naming the *legacy* tier as the current source. The
  code has resolved through `schema::paths`' newest-wins chain
  (`.ductus/` → `.govern/` → legacy root) since 049; only the prose lagged.
  No behaviour change. `schema/paths.rs` also carried one `/gov:review`
  reference under the retired command namespace.

- **`discover-rule-files` documentation names the resolved config, not the
  legacy tier.** Two doc comments described current behaviour with the retired
  filename: the `run` error contract said `.govern.toml` is what fails to
  parse, and the `DuctusToml` struct doc called itself a "`.govern.toml`
  shape". Both now name `.ductus/config.toml` and the newest-wins ladder. The
  third occurrence in that file is load-bearing and deliberately untouched —
  `apply_disabled_filter`'s doc explains the provenance tag rendered in a
  drop notice, which really does print the legacy root on a pre-migration
  layout. Same class as the `host.rs` entry above. No behaviour change.

- **The `check-artifacts` MCP tool description enumerates all nine families.**
  It listed eight, silently omitting `analyze-state-drift` — the family whose
  whole point is that the second gate left no trace until it existed. The
  primitive's own module doc says nine and `framework/commands/analyze.md`
  lists nine; only the description an agent host actually reads was short.
  Same stale-enumeration defect `089ba252` fixed in spec 054's AC10, surviving
  in a second place. Description text only; no behaviour change.

## [0.49.0] — 2026-09-13

### Added

- **A clean review states what it read.** `write-review` recorded no
  denominator, so a review whose five passes read the whole scope and found
  nothing and a review whose passes never ran produced **byte-identical**
  records: the same `0/0/0`, the same `reviewed-digest`, the same
  `blocking: false`. `check-review-gate`, audit Family 19, audit Family 31 and
  a human reader were all equally unable to separate them. `write-analysis` has
  required `unexamined` since 0.47.0 for exactly this reason; the reasoning was
  written for the analyze half and never carried across to the review half.
  It now is. The asymmetry between the two new fields is the design: **`scope`
  is derived by the primitive** — resolving `compute-review-scope` against the
  run's own `diff-base`, the same discipline that already derives `blocking`,
  `reviewed-digest`, `inbox-standing` and the Unexamined-governance section —
  so a caller cannot shrink the subject to match what it read, while
  **`examined` is the caller's claim**, because how many files the passes read
  is the one thing only the host knows. Both land in `review.md` frontmatter
  and the spec's `review:` block. An **unstated** `examined` is recorded as
  absent, never as a computed zero: a claim never made and a claim that came
  back empty are different facts, the distinction §grounding draws between
  *could not examine* and *examined and found nothing*.
- **`check-review-agreement` reads the claim.** Two new finding kinds, separate
  because the repairs differ: `examined-nothing` (`examined: 0` over a
  non-empty `scope` — the record states outright that the passes read nothing)
  and `examined-unstated` (no `examined` on a record that carries a `scope`).
  Both fields also join the paired-field set, so a hand-edit to one of the two
  records is caught like any other divergence. A record predating the fields
  carries no `scope`, so there is no denominator to judge and the family stays
  silent — an exemption that is bounded and self-correcting (the spec's next
  review writes a `scope`) rather than a per-spec grandfather date.

### Notes

- **This cannot prove the passes ran, and does not claim to.** An overstated
  `examined` is as available as an omitted one, exactly as an understated
  `unexamined` is. What it buys is that the claim is explicit and checkable
  instead of invisible, which is the bar `QUAL-CLAIM-001` sets and the bar the
  analyze half already cleared — no higher. Saying so here rather than implying
  enforcement the mechanism does not have is the same discipline
  §cross-spec-impact uses for the impact nothing detects.
- Specs [020](../specs/020-code-review/spec.md) (AC15, the requirement) and
  022 ([`a-review-states-what-it-read`](../specs/066-audit-records-and-the-completion-gate/scenarios/a-review-states-what-it-read.md),
  the runtime scenario). Both reviews for this release are the first to carry
  the new fields, and both record honest partials — 16 of 20 and 19 of 43 —
  which is the field working rather than a shortfall hidden.

## [0.48.0] — 2026-09-12

### Added

- **The pre-`done` gate blocks on an undischarged cross-spec impact.** A spec
  declares the specs it owes a change to in a `cross-spec-impact:` frontmatter
  list, and `check-review-gate` refuses `in-progress → done` while any entry is
  undischarged. **Discharge is the reciprocal link, not the key's removal** —
  an entry clears when the named spec's body, or a scenario under it, links
  back — because a key an author deletes to unblock themselves enforces
  nothing. The result reports per entry (`discharged` / `undischarged` /
  `target-missing` / `self-reference`) on a pass as well as a block, since
  partial discharge is the normal state of a multi-entry declaration. Ordered
  beside the `folds-into` check and sharing its reasoning — a spec carrying an
  obligation nobody has discharged is not a candidate for `done` — and sharing
  no code with it. `validate-frontmatter` checks the key's shape. The rule half
  is §cross-spec-impact, which now separates the enforceable half from the
  judgment half and says plainly that nothing detects an impact never declared.
- **`write-review` and `diff-cross-spec` report `inbox-standing`.** Every inbox
  surface before this was window-scoped, so an item older than the feature in
  hand was invisible by construction. The standing row renders on every run —
  `clean`, `outstanding` with the oldest item's `git blame` date,
  `age undeterminable` when blame cannot run, and `no-file` as its own state —
  and is never omitted, because examined-and-empty and not-computed must not be
  the same output. A notice, never a gate: §brownfield-inbox's design rests on
  capture being free.
- **`resolve-constitutions` carries each entry's `description` through to
  `ConstitutionRecord`**, so `/{project}:target`'s reports and `write-review`'s
  `## Unexamined governance` bullets can say what a source governs rather than
  only naming an alias. It renders on **skipped** entries too, where it helps
  most: the value comes from the config, not from the checkout that is missing.
  Whitespace is collapsed in the primitive so a TOML multi-line value cannot
  break a single-line report; absent stays absent.

### Fixed

- **The clap subcommand enum is pinned.** It was the one primitive-wiring
  surface nothing checked — a `Command` variant and its dispatch arm could both
  be deleted with the entire suite green, leaving `ductus <name>` missing from
  the CLI with nothing reporting it, on exactly the surface the markdown-only
  path depends on. A set-equality test now holds it to `PRIMITIVE_REGISTRY`, so
  a missing variant and a phantom verb both fail with the divergence named.
  **This supersedes the 0.47.0 note below**, which recorded the enum as pinned
  by nothing and told contributors to check it by hand; that note stands as the
  record of what was true then, and the manual step it prescribed is
  discharged.
- **A `cross-spec-impact` entry is screened before it becomes a path.** Found
  reviewing the new check: an entry was joined to the spec root unscreened, so
  `["../../elsewhere"]` read a `spec.md` outside the corpus and a back-link
  there *discharged* the obligation. Entries are slugs, not paths, and are now
  screened through `parse_feature_dir`.

### Changed

- `validate_no_traversal` carries the canonical statement of the
  **config-sourced path boundary** — when a primitive reading a path out of
  committed config may skip the check, and what moves that path into the
  validated tier. `resolve-constitutions` and `resolve-references` cite it
  rather than each restating the reasoning. Nothing enforces it, and it says so.
- One `collapse_whitespace` helper replaces three copies of the same
  `split_whitespace` chain.

See [022 — Deterministic runtime](../specs/022-deterministic-runtime/spec.md),
[050 — Constitution](../specs/050-constitution/spec.md), and
[055 — Shared constitution](../specs/055-shared-constitution/spec.md).

## [0.47.0] — 2026-09-11

### Added

- **`resolve-constitutions` — a project can register a governance document its
  organization owns, and every command inherits it.** `.ductus/config.toml`
  gains a `[constitutions.<alias>]` table naming a `repo` and a local `path`.
  The primitive resolves each entry against local disk and returns `loaded` and
  `skipped` separately. `/{project}:target` reads the resolved documents once
  per session alongside `.ductus/constitution.md`, so the rules bind every
  subsequent command with no per-command opt-in step.

  `repo` is identity and navigation only and is **never fetched**. There is no
  transport, no auth story, and no network failure mode — the local checkout is
  the only state read, which is why "unreachable" collapses to `not-checked-out`
  rather than being an error class. Registration is not transitive: a registered
  checkout's own config is never read, so there is no recursion to bound and no
  cycle to detect.

  Iteration is alias order from a `BTreeMap`, not config order, so a TOML
  reformat cannot change which documents load or in what sequence. The document
  filename is the fixed `constitution.md` rather than a per-entry field — a
  configurable name would let two projects registering the same source load
  different documents from it.

  A project with no entries registered behaves exactly as before: an absent
  table and a present-but-empty one both parse to an empty registry, so nothing
  downstream can distinguish them.

### Changed

- **`write-review` renders `## Unexamined governance`; `write-analysis` records
  `constitution-unresolved`.** A registered source the run could not read means
  the command ran under fewer rules than the project's config declares, and
  `QUAL-CLAIM-001` is explicit that a result must distinguish *examined and
  found nothing* from *could not examine*. Both primitives resolve the registry
  **themselves** rather than accepting it as an argument — a caller that had to
  supply it could omit it, and these records are exactly where a later gate
  trusts them. The two skip reasons stay distinct in the render: cloning the
  wrong repository and cloning nothing are different operator mistakes.

  Neither read is `?`-propagated, deliberately. An unreadable `[constitutions]`
  table is itself the strongest form of unexamined governance, so it is
  *rendered* rather than raised: propagating it would mean an unrelated typo in
  the project config cost the operator the whole `review.md`, and — worse —
  suppressed the `analyze:` record, whose **absence** is what the pre-`done`
  gate reads as "never analyzed". Each primitive carries a regression test for
  that path.

  `*None.*` covers both "none registered" and "all registered sources read":
  from the report's side those are the same claim, because in neither case did
  anything go unexamined.

### Note

- `schema/registry.rs` now records which primitive-wiring surfaces are pinned by
  tests and which are not. Verified by deleting each wiring in turn: the shipped
  manifest is asserted set-equal to `TOOL_NAMES`, the `#[tool]` methods are
  pinned by `tests/mcp.rs`, `dispatch_primitive` by
  `dispatch_handles_every_registry_primitive`, and `PRIMITIVE_NAMES` cannot
  drift at all since it aliases `PRIMITIVE_REGISTRY`. **The clap subcommand enum
  in `main.rs` is pinned by nothing** — deleting a variant and its dispatch arm
  leaves the whole suite green. Check it by hand with
  `cargo run -- <name> --help` until something pins it.

See [055 — Shared constitution](../specs/055-shared-constitution/spec.md).

## [0.46.0] — 2026-09-07

### Changed

- **Record staleness compares content, not commits — for both the `review:` and
  `analyze:` records.** This is the release's breaking change and it replaced a
  design that produced false blocks.

  `/{project}:review` and `/{project}:analyze` both read the **working tree**,
  while `reviewed-against` and `analyzed-against` record a **commit**. Those
  coincide only when the tree is clean, and at the moment either command runs it
  usually is not: the review has just written `review.md` and the `review:`
  block, and `mark-task` and `mark-criterion` rewrote `tasks.md` and `spec.md`
  before that. Diffing the recorded sha therefore blocked records whose run had
  genuinely read the current content, as soon as that content was committed —
  and a gate with false positives is the one people route around.

  Each record now carries a per-path sha256 of what its run actually read:
  `reviewed-digest` over the review's durable contracts (`scenarios/*.md`,
  `data-model.md`) and `analyzed-digest` over every `.md` under the feature,
  `review.md` included. `spec.md` is digested with its own `analyze:` block
  excised, since the record is written after the subjects are read and a digest
  covering it could never match. Measured before shipping: without that
  exclusion the rule flagged all 54 of this repo's recorded specs; with it, 1,
  and that one was a true positive.

  The two shas survive as **provenance**, read for exactly one thing — the
  mechanical-sweep rename exemption, which genuinely needs two trees and whose
  candidates are reported rather than dropped when they are unavailable.

  **A record with no digest is `undeterminable`** — not current, not stale, and
  not a sha-diff fallback. It does not block, so freshness stops being enforced
  for a spec until its next run writes a digest; it is self-healing rather than
  a hole, and the passing verdict's `guidance` names which record it could not
  judge. `reviewed_digest` is an `Option` for a case the analyze side cannot
  have: a spec with no scenarios and no data model records an **empty** digest,
  which is current, and `Some({})` is the only way to distinguish that from
  `None`.

- **`compute-review-scope` derives `captured-issues` from the working tree.**
  The section exists to surface issues captured *during* the session being
  reviewed, and such a capture is by definition uncommitted — so a
  `base..HEAD` diff reported none of them. It now compares inbox bullet sets
  between the diff base and the tree.

### Added

- **`check-review-gate` reports a spec already at `status: done`.** Ordered
  ahead of every other check, because every other check presumes a pending
  transition. It is deliberately **not** `passed: true`: a gate reporting a pass
  for a spec it did not examine is the `QUAL-CLAIM-001` conflation the rest of
  the gate exists to prevent, and a caller could read it as authorization to
  transition a spec that is already transitioned.

  It closes a defect the freshness check above introduced. The completing
  `set-status` rewrites `spec.md`, one of the analyze record's own subjects, so
  a spec's analysis is stale the instant it reaches `done` — and re-running the
  completion gate on finished work reported a stale analysis and directed the
  operator to re-run it. Following that advice wrote a fresh record, so the
  advice appeared to work, which is worse than advice that plainly fails. Only
  the exact value `done` short-circuits; an unrecognized status falls through,
  since `validate-frontmatter` owns reporting a bad value.

- **`write-analysis` records the `analyze:` block, and `check-review-gate`
  enforces it.** The pipeline is `review → analyze → done`, and until now only
  the first half left a trace: a spec that had passed both gates and one that
  had passed only the review were byte-identical on disk, so nothing could
  enforce the second.

- **`write-review` returns `analyze-freshness`,** so `/{project}:review` can
  render its `analyze` row from the same comparison the gate makes. One
  implementation, two surfaces, no reference point left for them to differ on.

### Removed

- **`unexaminable_contracts_guidance`.** Its entire subject was the
  committed-tree horizon the digest eliminates — it fired when a durable
  contract was *still* uncommitted and went quiet exactly when the false
  positive became reachable. A check that can no longer fire is
  indistinguishable from one that passed, so it was removed rather than kept
  alongside; the `QUAL-CLAIM-001` obligation it discharged moves to the digest
  comparison's own `undeterminable` arm.

## [0.45.0] — 2026-09-05

### Changed

- **`artifact-unreadable` blocks a spec at `done`.** `check-artifacts`
  reported it as a skipped target — an unknown, never a defect. That general
  rule is right and is unchanged for every other skip reason: it is about
  *other* files, another spec's frontmatter or an upstream service, where the
  defect is not this spec's and blocking it would punish the wrong artifact.

  `artifact-unreadable` names the spec's **own** artifact, in its own
  directory, that its own analysis could not read — not an unknown about
  someone else, but the analysis unable to examine its own subject. And the
  gap was concrete rather than theoretical: `scenario-open-questions` is
  blocking at `done`, and an unreadable scenario contributed no questions and,
  as a skip, no finding. A scenario carrying unresolved questions that would
  not parse therefore passed the gate built to catch exactly that — a check
  that could not run wearing the costume of one that passed, *inside* the gate
  rather than beside it.

  At `done` it is now a blocking finding, so it lands in `blocking-findings`
  rather than `unexamined` and one file is never both. Below `done` it remains
  a skipped target: the questions check is advisory there, the spec is in
  flight, and an unreadable artifact mid-work is a state to report rather than
  a gate to fail.

  Adopters with a `done` spec carrying an unreadable artifact will see it
  block where it previously passed. That is the point; the corpus here had
  zero, which is why this landed now rather than behind a backlog.

- **`resolve-anchor` classifies each `§` reference instead of treating every
  one as a claim about the markers file.** `§` is this corpus's notation for
  *a section*, not for *a constitution section*, so a spec writes
  `AGENTS.md §Workflow`, `[review.md](...) §Behavior` and `per §Design above`
  alongside genuine constitution citations. Resolving all of them against one
  file reported **112 unresolved anchors** across the spec corpus, every one
  surfaced by `/ductus:analyze` as an advisory finding.

  The cost was not noise for its own sake. Spec 023 deleted
  `§lightweight-track` from the constitution — its AC2 asserts the section is
  gone and its own task said "verify the anchor is no longer referenced
  anywhere" — and spec 010's body still cited it. This primitive had been
  reporting that correctly the whole time, among 111 lines that were not
  defects, and it survived four specs' worth of history until the list was
  read by hand. A signal that is 99% noise is not a signal.

  Three kinds, recorded per reference as `kind`:

  - **`qualified`** — the reference's *line* names a markdown document other
    than the markers file. Excluded by construction and counted. Line-scoped
    rather than immediately-preceding, because the dominant real shape is a
    table row naming the file once and citing several of its sections
    (measured: 34/112 vs 136/311). "Other than the markers file" is
    load-bearing — a line citing the constitution *is* the kind this
    primitive exists to check, and an existing test caught the first draft
    excluding it.
  - **`intra-document`** — the anchor names a heading in the citing file,
    resolved against that file's own headings, longest first.
  - **`markers`** — a genuine claim about the markers file, and the only kind
    that can come back unresolved.

  Corpus effect: **112 unresolved → 34**, with the one real dangling anchor
  still among them — asserted as a unit test rather than left as a hope. The
  residue is prose naming a document without naming its file
  (`spec 022 §Versioning`), which can be neither verified nor excluded;
  reporting it is the honest answer, and widening the rule to swallow it
  would mean guessing.

### Added

- `resolve-anchor`'s result carries `qualified` and `intra-document` counts.
  136 of 311 references are excluded on this corpus, and an exclusion nobody
  counts is indistinguishable from a scan that found nothing — so the numbers
  ship with the verdict. `unresolved` now means "every reference that was a
  claim about the markers file resolved", never "every `§` in this file is
  fine", which is a claim the primitive cannot make.

  `AnchorReference.kind` defaults to `markers` when deserializing a result
  written before the field existed, which is what every such reference was.

## [0.44.0] — 2026-09-05

### Added

- **`write-analysis` — the durable record that `/ductus:analyze` ran.** The
  pipeline is `implement → review → analyze → done`, and until now only half
  of it left a trace: `check-review-gate` read the `review:` block, Family 19
  checked its freshness, Family 31 held it against `review.md`, and analyze
  wrote nothing at all. A spec that had passed **both** gates and a spec that
  had passed **only the review** were byte-identical on disk. Nothing could
  tell them apart, so nothing could enforce the second gate, so the only thing
  holding it was whoever remembered — the diligence dependency
  §design-principles rejects outright.

  Not hypothetical: on 2026-09-05 two specs were advanced to `done` on the
  review gate alone, and one of them was published to crates.io — where a
  version can never be reused — before anything noticed, because there was
  nothing to notice.

  The new `analyze:` frontmatter block carries `last-run`,
  `analyzed-against`, `hard-fail`, `blocking-findings`, `advisory`,
  `unexamined`, and a derived `blocking`. It is deliberately **not** a copy of
  `review:`:

  - **`advisory` is recorded and never gates.** An outstanding SHOULD blocks
    `done` at the review gate because §implement-phase says advisory is not
    ignorable there. Analyze's advisory tier is a different contract: its
    members are checks introduced advisory *with published promotion criteria*
    — grounding, Applicable-Rules citations, decision drift — and gating on
    them would promote every one at once, past the criteria each declares.
  - **`unexamined` has no counterpart at all**, and is what makes the record
    honest. A clean analyze is two states, and the command's own contract says
    so: "clean with nothing skipped is verified-clean, clean with something
    skipped is partially examined." A record carrying only finding counts
    would collapse that into the reassuring reading, inside the artifact a
    later gate trusts.

- **`check-artifacts` gains `analyze-state-drift`** — nine residual
  deterministic families rather than eight. A `done` spec with
  `analyze.last-run` unset or `analyze.blocking: true` is drift.

  Grandfathered for specs predating the record, and that is the honest choice
  rather than the convenient one. 046 refused this shape of exemption and the
  criterion-label check backfilled instead — but a criterion label is
  *derivable from the artifact*, so that backfill computed something already
  true. An analyze record asserts *that a run happened*, which nothing on disk
  substantiates; backfilling it would write an unverified claim into the field
  a later gate reads, which is the exact failure the record was added to
  prevent. `/ductus:audit` Family 37 counts the exempt population against a
  committed high-water mark instead, so the set is bounded and shrinking
  rather than a silent permanent hiding place — and since the gate has no
  grandfather clause, it cannot grow.

### Changed

- **BREAKING for the completion gate: `check-review-gate` now blocks on a
  missing or failing analysis.** Two new `blocked-by` variants,
  `not-analyzed` and `analyze-findings`, ordered after every `review:` check
  because the pipeline runs in that order — a spec whose review is missing has
  not reached the point where analysis is the next thing owed, and naming the
  later gate for an earlier defect sends a contributor to the wrong command.

  A host driving `/ductus:implement` against a spec with no `analyze:` block
  will now be blocked where it previously proceeded. That is the point. There
  is deliberately no grandfather clause **at the gate** — it runs when a spec
  is being completed now, so the record is always writable, and an exemption
  there would be a permanent hole rather than a bounded transitional one.

- `/ductus:analyze`'s read-only contract is restated rather than broken. It
  still never mutates an artifact it audits, and `--fix` is still the only
  path that does; it now writes two things **about itself** — the inbox
  capture of surviving findings, and this record. The line is between the
  subject and the observation, and recording that an audit happened is what
  `/ductus:review` has always done.

- The spec template ships the `analyze:` block, so newly-scaffolded specs
  carry it from the start.

## [0.43.0] — 2026-09-05

### Changed

- **BREAKING for a caller that was already broken: `apply-manifest` rejects a
  placeholder-shaped substitution key.** Keys are bare — `project`, never
  `{project}` — because the primitive wraps each key in braces itself to build
  the token it searches for. A braced key therefore built `{{project}}` and
  matched nothing.

  Nothing stated the contract, and the two spellings genuinely disagree on the
  page: the installer's §Placeholder Substitution writes placeholders braced
  because it names tokens *in files*, while `keep-literals: ["project",
  "cli-config-dir"]` two sections earlier writes them bare because it names
  *map keys*. The schema's own doc comment said "`{key}` → value substitution
  map", which reads as though keys are braced.

  An adopter called it with braced keys. Every strategy ran, every file was
  written, and the result came back `{"created":1,"updated":25,"unchanged":11}`
  — indistinguishable from a correct run — while 370 literal placeholders
  shipped across the constitution, all 17 commands, 5 rule files and 2
  templates, every `/{project}:review` arriving with the placeholder intact.

  An empty key, or one containing `{` or `}`, is now
  `PrimitiveError::InvalidSubstitutionKey`. It is checked before the traversal
  validation and before any filesystem operation, so a malformed map halts the
  walk with **zero writes** rather than leaving a half-substituted tree. The
  rejection is exact rather than stylistic — it refuses only keys incapable of
  matching a placeholder — so the legal bare key `One-line project
  description.`, with its spaces and period, still substitutes.

  A host that was passing braced keys will now fail loudly at a call that
  previously appeared to succeed. That is the point: it never worked.

### Added

- **`apply-manifest` reports the substitution count it had been discarding.**
  `read_and_substitute` computed the replacement count on every entry and bound
  it to `_count`. One number — zero replacements across thirty-seven files —
  separated the broken run above from a correct one, and it was thrown away.
  That is `QUAL-CLAIM-001`, the rule this project ships in
  `framework/rules/quality-cross.md`, in the primitive that installs the file.

  Three fields, shaped so no reading of them is silently wrong:

  - `substitutions-applied` on each entry, `Option<u32>`. `0` means the file
    was read, decoded as UTF-8, and matched no placeholder — correct for a file
    carrying none. **Absent** means the question was never asked: pinned,
    skipped-exists, source-missing, `skip-if-conflict` (which never
    substitutes), or non-UTF-8 bytes. Serialized with `skip_serializing_if`, so
    the two never arrive as the same value.
  - `substitutions-applied` on the result — the total across every entry that
    ran.
  - `entries-substituted` on the result — the denominator. Zero replacements
    across twenty substituted entries is a defect; zero across zero is a
    manifest of pinned and skipped files behaving correctly, and one number
    cannot say which.

  Additive for hosts: an existing consumer sees the same JSON for every entry
  that did not substitute, plus one key on those that did.

- The bare-key contract is now **stated** where callers read it — the
  primitive's module docs, the `substitutions` schema doc comment, and
  §Placeholder Substitution in both bootstrap twins — and both counts are
  surfaced in §Post-Scaffolding Output, since a diagnostic nobody prints is one
  nobody reads.

## [0.42.0] — 2026-08-31

### Removed

- **BREAKING: supersession is gone.** Two MCP tools, two CLI subcommands,
  one extension point, one check family, and one frontmatter key are removed.
  A host or script naming any of them will fail rather than degrade.

  Removed tools and subcommands: `read-supersession-pair` and
  `write-supersession-annotation`. Both leave all five registration sites —
  the CLI enum and dispatch arm, the exec-path match arm, the `#[tool]`,
  `PRIMITIVE_REGISTRY`, and `framework/runtime-tools.txt` (71 tools → 69).

  Removed extension point: `classifyClaims`, with `SupersededClaim`,
  `ClassifyClaimsRequest`, `ClassifiedClaim`, and `ClassifyClaimsResponse`.
  The `CLAIM_CLASSIFICATIONS` vocabulary goes with it.

  Removed check family: `supersession-reciprocity`. `check-artifacts` now
  runs **eight** residual deterministic families rather than nine; a caller
  counting families or matching on that name sees the difference.

  **Removed frontmatter key: `supersedes`.** It leaves the `Frontmatter`
  struct, so `read-spec` no longer surfaces it and `validate-frontmatter` no
  longer shape-checks it. This is a **silent** change for an adopter who has
  written one, and silent in the safe direction — measured, not assumed:
  `validate-frontmatter` performs no unknown-key rejection, so a spec
  carrying the key reports `clean: true` with zero findings; `read-spec`
  omits it from the parsed result rather than erroring; and the frontmatter
  writers splice rather than re-serialize, so `set-status` and
  `label-criteria` leave the key byte-for-byte intact on disk (verified
  against a probe spec carrying `supersedes: [005-workflows,
  019-config-decisions]`). The key becomes inert YAML the pipeline neither
  reads nor destroys. No migration strips it, and no adopter spec content is
  edited.

  Reason: supersession kept a countered spec on disk with an annotation,
  which accumulates specs that read as live but describe removed behavior.
  The two operations that keep a corpus true already existed — editing the
  earlier spec in place through the `done → in-progress` back-edge, and
  `/ductus:consolidate`. Spec 054.

- **`blockquote_cites` and `names_feature`** leave `primitives::mod`. Both
  existed only for the two removed surfaces, and `blockquote_cites` was
  extracted specifically so the annotation writer and the reciprocity family
  could not disagree about whether a spec was annotated. No third caller.

### Unchanged, deliberately

- **The blockquote exclusion in `spec_links.rs` stays.** It belongs to the
  shared body scanner behind `derive-dependencies`, `derive-references`,
  `check-corpus-links`, and `rewrite-spec-links`, and its reason — signpost
  links on `done` specs are navigation, not dependencies — predates
  supersession and outlives it. `derive-dependencies` produces byte-identical
  output across this release.

## [0.41.0] — 2026-08-31

### Added

- **`writeCode` requests say what the constitution-excerpt load could not
  examine.** `load_constitution_excerpts` returned a bare `Vec<String>`, and
  five states collapsed into the same value: the command file could not be
  located, it could not be read, it declares no `Reference:` anchors, the
  constitution could not be read, and — via a closing `filter_map` — an anchor
  the command declared relevant resolved to nothing and was dropped from an
  otherwise-populated array. Only the third is "examined and found nothing";
  in the other four the host received a payload reading as "no constitutional
  context applies" and wrote code without it, with no way to tell.

  The request now carries `constitution-excerpts-unexaminable`, one entry per
  cause (`command-file-missing:`, `command-file-unreadable:`,
  `constitution-unreadable:`, `anchor-unresolved: §…`), omitted from the wire
  when empty so a payload whose anchors all resolved is byte-identical to one
  built before the field existed. It sits inside the stable cache prefix,
  ahead of `write-boundary`, because it does not vary between tasks in one
  walk. Labels are repo-relative: they ride in an outbound payload, and an
  absolute path would carry the contributor's home directory with it. This is
  `QUAL-CLAIM-001` applied to the payload bundler, the same shape `read-spec`'s
  `scenario-files-unreadable` answers for scenarios.

### Fixed

- **A `Reference:` line naming one anchor twice no longer sends its excerpt
  twice.** `parse_command_references` deduplicated with `Vec::dedup`, which
  drops only *consecutive* repeats, so `§spec-phase, §text-first-artifacts,
  §spec-phase` emitted the first section's body twice. Now an order-preserving
  seen-set filter, so first-occurrence order — and with it the byte-stability
  of the cache-anchored prefix for already-clean inputs — is unchanged.

## [0.40.0] — 2026-08-30

### Added

- **`check-artifacts`' `review-state-drift` family reports an outstanding
  SHOULD on a `done` spec.** §implement-phase is explicit that advisory is not
  ignorable at the gate: a SHOULD is addressed by being **fixed**, which drops
  it from the count, or by being moved under `review.md`'s **Waived findings**
  with its rationale, which also drops it — so a non-zero count at `done`
  means neither happened and the finding is still filed under its original
  heading. Nothing caught this. The review-block-agreement family compares the
  frontmatter `review:` block against `review.md`, but one review run writes
  both, so a fix applied *during* a pass that never re-runs leaves the two
  consistently stale and that family clean; a spec sat in exactly that state,
  its own report reading "Fixed during this pass" beside a count of 1, and a
  hand sweep found it rather than the tooling.

  The message names **both** dispositions, because a SHOULD whose answer is
  "keep as-is" is waived rather than fixed and naming only the first would
  push an operator toward the wrong one. `--fix` reverts the status and never
  edits the count: the count belongs to the review that regenerates it, and
  rewriting it here would erase the finding rather than resolve it. A spec
  still in flight carrying a SHOULD is not a finding — the rule is about the
  state `done` asserts.

## [0.39.0] — 2026-08-30

### Added

- **`read-supersession-pair` — the bounded read spec 053's reconciliation is
  built on.** It loads the two specs a declared supersession names plus the
  superseded spec's scenarios, and **the absent arguments are the design**:
  there is no way to request a plan, a data model, a tasks file, a source
  path, or a third spec, so the read bound is a property of the code rather
  than a rule a caller is asked to remember. That bound is load-bearing —
  without a declared edge to collapse the search to two specs, this is the
  corpus-wide criterion-supersession check measured at 455 pairs with 215
  firing and every sample a false positive. Only the *superseded* spec's
  scenarios are read; the superseding spec's describe what it delivers, which
  is a different question. An unreadable spec or scenario is a domain outcome
  rather than an error: it lands in `unreadable`, is excluded from
  `examined`, and is never escalated into a conflict, because nothing can be
  proven about a file that will not parse. `guidance` marks a superseded spec
  with nothing to classify at all — examined-and-empty, which is not
  examined-and-clean. 9 unit tests.

- **The `classifyClaims` extension point.** Deciding whether a superseding
  spec removes, contradicts, or leaves alone a predecessor's claim is a
  reading of two documents, so it crosses the runtime boundary rather than
  pretending to be deterministic. Four closed outcomes — `superseded`,
  `still-standing`, `conflicting`, `unclassified` — sent with the request so
  the classifier answers in the framework's terms. Its vocabulary is
  deliberately not `routeFold`'s or `routeInboxItem`'s: those answer *where
  does this work belong*, this answers *what did the later spec do to this
  claim*. The payload is built from `read-supersession-pair`'s result, so the
  read bound holds at the boundary too, and `still-standing` claims are
  carried rather than omitted — dropping them would make examined-and-
  untouched indistinguishable from never-looked-at.

- **`check-corpus-links` gains `--scope repository`.** One resolver, two
  subjects: the default spec-corpus scope is what an adopter's pre-commit
  hook checks, while the repository scope enumerates every *tracked* `.md`
  file from the git index for `/{project}:audit` Family 26. The family
  previously carried its own implementation of the same check and the two had
  already diverged — the primitive resolving a root-absolute target against
  the repository root, the family against the filesystem root. Delegating
  makes that impossible rather than merely repaired. The index rather than a
  worktree walk, so an untracked draft is never reported and build output is
  never descended into. 4 further unit tests, including one pinning that the
  repository scope examines strictly more than the spec-corpus scope — the
  narrowing this consolidation could otherwise have introduced silently.

### Changed

- **`write-supersession-annotation` takes a `criterion` label.** A
  *granularity*, not a second primitive: the constitution states one
  supersession-annotation rule at three granularities, so one writer owns it.
  Absent, it writes the whole-spec banner unchanged; present, it appends the
  annotation to that criterion's line, citing the superseding spec **by name**
  — a criterion is a plain list item with no blockquote exemption, so a link
  there would be harvested into the annotated spec's own `dependencies:`. The
  criterion is annotated, never edited: its checkbox and text are
  byte-identical afterwards, so a superseded criterion stays ticked. An
  unknown label is refused rather than treated as a criterion to create.
  Section-level annotations remain hand-authored. 7 further unit tests; every
  pre-existing whole-spec test passes untouched.

### Fixed

- **`check-corpus-links` no longer hardcodes a host's config directory.** The
  generated-command-copy exclusion resolved `.claude/` literally, which is
  correct for exactly one of the four hosts ductus ships to — every Auggie,
  Antigravity, and OpenCode adopter would have had their generated copies
  examined and their by-construction-broken links reported as defects they
  cannot fix. Resolved from `Host::load` instead. `/{project}:audit` Family
  13 exists for this regression and caught it.

## [0.38.0] — 2026-08-30

### Added

- **`write-supersession-annotation` — the reciprocal half of a declared
  supersession.** A `supersedes:` key records the relation for tooling; what
  stops a reader mistaking a countered spec for a live one is an annotation on
  the spec that was countered. The primitive contributes the frame — placement
  above any annotation already present, the blockquote wrapper, the
  `> **Sunset ([link]):**` citation, and the closer recording that the spec
  stays the account of what shipped — and the author contributes the substance,
  the one sentence naming what stopped being true. The **blockquote is
  structural, not stylistic**: `derive-dependencies` exempts blockquote-prefixed
  lines, so the banner links its successor without the annotated spec acquiring
  a dependency on it; un-blockquote the frame and every annotation silently
  inverts the dependency graph. The frontmatter is spliced back byte for byte,
  so the annotated spec keeps whatever status it holds at any lifecycle state —
  the write is a mechanical edit taking no back-edge, and that guarantee is
  structural rather than a rule the code remembers. An annotation already
  citing the same superseding spec returns `already-present: true` and writes
  nothing, so an interrupted declaration converges on a re-run instead of
  stacking a duplicate. 12 unit tests.

- **`check-artifacts` gains the `supersession-reciprocity` family** at the
  advisory tier: a spec declaring `supersedes: X` whose named target's body
  does not name it back. It reads only **declared** edges and never infers an
  undeclared supersession — that check was measured at 455 criterion pairs with
  215 firing and every sample a false positive. Its coverage bound is therefore
  real and is stated in the family's contract and in `/{project}:analyze`'s
  rendering rather than emitted as a per-spec skip: a corpus of hand-written
  annotations carrying no key is invisible to it, and a spec declaring no edges
  is not an unexamined target.

- **`check-corpus-links` — every relative markdown link in the spec corpus that
  resolves to nothing.** Three checks sat near this ground and none covered it:
  `check-orphaned-references` scopes to the adopter-owned bootstrap referrers,
  `/{project}:analyze` is bounded to one feature plus its dependencies, and
  `/{project}:audit` Family 26 is maintainer-only — adopters never invoke it.
  So an adopter who deleted or renamed a spec directory dangled every inbound
  pointer with no error and no gate. The shipped pre-commit hook now runs it,
  corpus-wide and blocking, so a deletion fails at the commit that makes it
  rather than at a reader's next traversal. Resolution is **lexical**, never
  canonicalized, so the verdict is a property of the text rather than of the
  checkout's symlinks; a root-absolute target resolves against the repo root,
  as a markdown renderer does. Inline code spans are stripped before matching —
  load-bearing rather than tidy, since documentation quoting link syntax is not
  making a link — while blockquoted lines are deliberately **not** skipped:
  that exemption belongs to `derive-dependencies`, where it decides whether a
  link induces an edge, and a sunset banner's link still has to resolve. An
  unreadable file, a spec root that cannot be listed, and a walk deeper than
  the depth cap each block and say so, because a check that could not run must
  not exit like one that passed. 18 unit tests.

### Changed

- **`retire-feature`'s refusal of the sequential `NNN-slug` form is gated
  rather than removed.** The refusal exists because that form is permanent and
  a primitive able to delete one would put an irreversible operation within
  reach of a typo. A new `allow-sequential` argument opts in, and only
  `/{project}:consolidate` passes it, after naming both specs and confirming
  the content loss with the operator; `/{project}:fold` does not pass it and
  has no argument that would, so a mistyped feature name during a fold meets
  the refusal exactly as before. The anti-stranding guard — the target must
  hold a `spec.md` — is untouched and applies to both callers. The flag is not
  a weaker refusal; it is the record that a second, explicit decision was made.

- **`validate-frontmatter` validates `supersedes:`** for **shape only**: each
  entry parses as a feature slug, and a self-reference is rejected. An
  unresolvable-but-well-shaped entry is no finding — resolvability is enforced
  by the declaring command, at the one moment both specs exist. This follows
  the `folds-into:` precedent exactly.

### Fixed

- **One predicate now answers "is this spec already annotated?"** for both
  surfaces that ask it. `write-supersession-annotation` accepted only the
  markdown link form while the `supersession-reciprocity` family accepted a
  link *or* a bare name — so a hand-annotated spec, which is the entire corpus
  the retroactive declaration path exists for, satisfied the check and still
  attracted a duplicate annotation. Matching also moved to a slug boundary: a
  bare substring match reads `043-workflows-sunset` as a citation of
  `043-workflows`, and naming a sunset spec after what it sunsets is this
  corpus's convention rather than an edge case.

## [0.37.0] — 2026-08-30

### Added

- **`merge-permissions` retires formerly-canonical entries (`revoke`).** The
  primitive installed and deduped but never removed, so an allow entry the
  framework once shipped and has since dropped survived in every adopter tree
  indefinitely — nothing owned the removal. A new `revoke: Vec<String>` names
  the entries to retire; only exact string matches are removed, so a pattern an
  adopter authored themselves is never touched however closely it resembles a
  retired one. Retirement is **allow-side only by construction**: there is no
  deny-side counterpart, because an over-broad deny entry refuses more rather
  than approving more, and one argument spanning both arrays would invite
  narrowing the deny set into holes. The revoke pass runs before dedup and
  canonical-presence, so every copy of a retired entry is removed, none is
  re-added, and a doubled retired entry is attributed wholly to the new
  `allow-revoked` count rather than split with `allow-deduped`. An entry present
  in both `allow` and `revoke` is rejected with the new
  `PrimitiveError::ConflictingRevoke` before any filesystem read — naming every
  overlap at once — because the two passes would otherwise fight, the merge
  could never reach a fixed point, and the `unchanged` short-circuit that
  preserves mtime could never fire. 15 new unit tests; 30 on the primitive.

- **`/ductus:audit` Family 32 — permission entry shape**
  (`scripts/audit/permission-entry-shape.sh`). 32a fails when a canonical
  permission entry scopes a file path with `Write(path)` rather than
  `Edit(path)`: Claude Code matches file permissions against `Edit` rules only,
  and an `Edit` rule already covers every file-editing tool, so such an entry
  grants nothing and the host warns about it at session start. Kept separate
  from Family 29 despite the shared subject — that family's defect grants too
  much, so deny is excluded there by construction, while this one grants
  nothing, which is just as wrong on the deny side; both arrays are subjects
  here, so one merged contract would have to relax the exclusion that keeps
  Family 29 safe. 32b fails when an entry is both retired and still canonical,
  moving a `ConflictingRevoke` that would otherwise fire in every adopter's
  `/ductus:configure` run back to maintainer time. The bare `Edit` / `Write`
  tool grants are excluded by construction (the extraction requires a
  parenthesised argument), and the bullet anchor keeps the sections' own
  counter-example prose outside every scanned window.

### Changed

- **`/ductus:configure` retires nine formerly-canonical permission entries.**
  Adopters configured before ductus 0.33.0 carried seven
  `Bash(git -C * <sub> *)` allows whose leading wildcard spans inserted options
  — `-c` and `--exec-path` run arbitrary commands, so each approved arbitrary
  execution with no prompt — and two inert `Write(.ductus/*.toml)` allows. A new
  step 4 lists all nine and passes them to `merge-permissions` as `revoke`; the
  markdown-only path performs the same exact-match splice. This supersedes spec
  023's `configure-permission-pattern-safety` decision that the command does not
  retroactively strip an already-present entry, annotated in place rather than
  deleted.

  Deliberately **not** a `/ductus` migration: `[migrations].last_applied` lives
  in the committed `.ductus/config.toml` while `settings.local.json` is
  gitignored, so the first teammate to run it would mark the entry applied for
  the whole repo and strand everyone else permanently — and `/ductus` never
  writes the full permission set in any case. Spec 027 now records that limit:
  the registry applies an entry once per repository, and a migration must not be
  the sole remedy for a defect in per-contributor state.

- **The canonical allow set no longer ships `Write(.ductus/session.toml)` or
  `Write(.ductus/config.toml)`.** Their `Edit(...)` siblings, which sat directly
  beside them, are what granted the access all along. The bare `Edit` / `Write`
  tool entries are unaffected — they name tools rather than paths.

## [0.36.0] — 2026-08-30

### Added

- **A second feature-directory form, scoped to the branch that creates it.**
  A spec created on a story branch can be numbered `{identifier}.{n}-{slug}`
  — `1234.1-retry-budget` — instead of taking a number from the global
  sequence. The identifier namespaces the counter, so two branches numbering
  under different identifiers cannot collide at merge, and neither counter
  influences the other: a spec root holding `050-a` and `1234.1-b` still
  yields `051-` next. The identifier is an opaque operator token, not a
  number, because trackers disagree (`PROJ-1111`, `1111-PROJ`); it is
  sanitized to the directory grammar rather than refused, and the sanitized
  value is shown before any directory exists.

  The form is **temporary by construction**. Every branch-scoped spec
  declares in `folds-into:` the sequential spec it stands in for, and it has
  no `done` state at all — the pipeline view reports it as carrying a pending
  fold and the pre-`done` gate blocks while the key is present. It is
  retired, not completed.

- **`/{project}:fold` — the command that discharges a branch-scoped spec**
  into its upstream home, run after the merge on the upstream branch, which
  is the first tree in which both specs exist. It routes the content to a
  body edit or a scenario, re-points every inbound pointer, reopens a `done`
  upstream spec through the back-edge §spec-lifecycle already defines, and
  removes the staging directory.

- **Four primitives behind it.** `check-unfolded-specs` reports branch-scoped
  directories still present with their declared targets — nothing runs on a
  merge, so this is what notices. `rewrite-spec-links` re-points body links
  and `folds-into` fields in one action, matching by whole path segment so a
  directory sharing a prefix is untouched, and refusing a destination that is
  not a home. `retire-feature` removes a branch-scoped directory, refusing
  unless the fold target exists — the one place that existence is enforced,
  since before the merge the target normally lives on another branch.
  `invalidate-review` resets a spec's `review:` block to the un-reviewed
  state, preserving operator-recorded waivers.

- **`create-feature` gains `branch-id` and `fold-into`**, and returns the
  sanitized `identifier` it used. Absent `branch-id`, behavior is unchanged.
  `fold-into` is required with it: a branch-scoped spec exists in order to be
  folded, so there is no way to create one that names no home.

- **`append-task` reports `appended`**, and dedups on the scenario pointer: a
  task already referencing `scenarios/{slug}.md` is returned rather than
  duplicated, which is what lets an interrupted fold be completed by
  re-running it.

### Fixed

- **A spec numbered past 999 was invisible to every corpus reader.**
  `create-feature` formats `{number:03}` — a *minimum* width — so the 1000th
  spec is `1000-slug`, while the membership predicate demanded exactly three
  digits. The directory was created successfully and then seen by nothing.
  The predicate now accepts three-or-more digits, rejecting padding beyond
  the minimum so one number keeps one spelling.

- **The same rule survived in five shell surfaces the predicate never
  reached.** Both pre-commit hooks selected staged specs with a digits-only
  pattern, so `label-criteria` never ran on a spec past 999 — nor on **any
  branch-scoped spec**, whose form a digits-only pattern rejects outright.
  Three lint and audit surfaces globbed the same shape and skipped such a
  directory while exiting 0. The hooks now match either form and a runtime
  test holds them to the predicate; the audit families ask the runtime for
  the corpus instead of globbing, which also retired their hand-rolled
  frontmatter scans.

- **Seven primitives rewrote a text file without preserving its line
  endings**, converting a CRLF checkout's file to LF as a side effect of
  changing one line — and two of them converted only *part* of a file,
  leaving the halves disagreeing. Detection now lives in one helper that
  every text writer goes through, enforced by a lint over the sources rather
  than a list of the writers someone thought of.

- **`routeFold`'s payload builder read its fold target uncontained**, so a
  hand-authored `folds-into` carrying `../` lifted a file from outside the
  repository into a payload bound for the host.

### Changed

- **§numbering defines both directory forms** and states that three digits is
  a minimum width; **§spec-lifecycle** records the branch-scoped form as a
  staging form discharged by fold-back, with no `done` state, reconciled
  against the anti-proliferation stance it does not relax.

## [0.35.0] — 2026-08-28

### Added

- **`check-review-agreement` — a spec's frontmatter `review:` block must agree
  with its own `review.md`.** The same review is recorded twice: the spec's
  block carries `last-run`, `reviewed-against`, the three counts, `blocking`,
  and any `waivers`; the report beside it carries `reviewed-at`,
  `reviewed-against`, and the same three counts. Nothing held the two together,
  and they drifted — 031 and 041 both carried `should-violations: 1` in
  `spec.md` while their reports recorded `0`, for weeks. The drift is invisible
  because every gate reads exactly one of the two files: `check-review-gate`
  and `/ductus:analyze`'s review-drift check read the block, while `/audit`
  Family 19 resolves `reviewed-against` without ever comparing the counts on
  either side of it. The cost runs both ways — a stale non-zero count reads as
  outstanding work that does not exist, and a stale zero would hide real
  findings from every gate that trusts the number.

  The primitive compares the five fields both files record, keyed by meaning
  rather than by name since the timestamp is spelled `last-run` on one side and
  `reviewed-at` on the other, and names `review.md` as the source of record in
  every repair: it is what `/ductus:review` writes from the pass it just ran,
  while the block is the summary copied forward for the gates. Two rules
  internal to the block are asserted alongside them, because the observed
  failure reached the counts through them — `blocking: false` alongside a
  non-zero `must-violations`, and a finding waived in the report with no
  matching `review.waivers` entry, which is a waiver with no structural
  existence, so the count it should have retired never moves.

  Fields present on only one side (`diff-base`, `captured-issues`,
  `skipped-passes`; `blocking`, `waivers`) are not compared — they are not
  duplicated facts, and demanding they match would invent a binding the
  artifacts never claimed. The subject is the intersection of specs carrying
  both records, the only ones that can disagree; a spec carrying one is counted
  in `single-sided` and left to Family 19 and `check-review-gate` rather than
  dropped. Unparseable frontmatter on a report that exists is a finding rather
  than a skip, and an empty subject sets `guidance`, because comparing nothing
  reports agreement — the false green the check exists to prevent. Surfaced as
  `/audit` Family 31, whose script is a thin entry point over this primitive.

### Changed

- **Runtime eligibility is a default, not a permission**
  ([§runtime-boundary](../framework/constitution.md#runtime-boundary)). A
  capability that is deterministic, already mechanical, and specifiable as
  prose is implemented as a primitive; a shell script is the fallback, taken
  only when a criterion genuinely fails. The framework still needs shell entry
  points — a hook, a CI step, an `/audit` family — but the entry point resolves
  the runtime and calls the primitive rather than reimplementing the check.
  A script that parses frontmatter or markdown structure has already failed
  principle 3, whether it reaches for `awk` or an embedded interpreter. The
  rule was written against a worked example: Family 31's first implementation
  hand-rolled a frontmatter parser whose `\s*` after the key matched a newline,
  so an empty value walked the match onto the next line and returned that
  line's content, reporting a spec that was correct.

## [0.34.0] — 2026-08-27

### Fixed

- **`derive-references` reports drift it examined but did not write.** Under
  `--staged` the primitive narrowed its *enumeration* to the staged set, on the
  recorded reasoning that a spec's `references:` is a pure function of its own
  body, so "enumerated" and "written" coincide and the in-sync claim was
  already sound. That reasoning was wrong. A `references:` entry is a function
  of the body **and** the `[services]` registry — the harvest resolves each
  link's repo URL through the registry to produce the `service:` alias. Rename
  a service alias and every referencing spec drifts while none of them is
  touched; an untouched spec is never staged, so staged mode could never
  examine one, on any commit. An adopter carried dead references for nine
  commits behind this while the pre-commit hook reported the tree in sync every
  time. The walk now spans every tracked spec and `--staged` filters only the
  write, with drifted-but-unstaged specs reported in the new `unwritten` field —
  the same field, with the same meaning, its sibling already had. Write
  behavior is unchanged: committing one spec still never rewrites another.
- **Both derive primitives name the tracked specs they could not read.** They
  enumerate the git index and skip `!path.is_file()`, but reported
  `examined: tracked.len()` — so a spec tracked in the index and deleted from
  the worktree without the deletion being staged was counted as examined
  without being read, and appeared in none of the result's other lists. Both
  results gain `absent`, and `examined` now counts specs actually read.
- **A review window now contains the commit that opened it.**
  `compute-review-scope` resolved the diff base to the commit at which the spec
  advanced to `in-progress`, and `base..HEAD` excludes that commit's own
  changes. Harmless in the flow the design assumed — `/ductus:implement` flips
  `planned -> in-progress` in a commit holding only the status line — and
  silently fatal in the reopen flow, where `/ductus:amend`'s back-edge is
  naturally committed together with the work it authorises: the window then
  started after the work and held none of it. Two specs in one round would have
  recorded 0 MUST / 0 SHOULD having examined nothing, caught only because the
  scope list was read by hand. The base is now the transition commit's first
  parent. An explicit `--since` is used verbatim — no parent walk, since an
  operator naming a commit means that commit — and a parentless transition
  falls back to itself. `find_in_progress_commit` is unchanged, so `check-stuck`
  still counts from the transition commit itself.
- **`append-task` no longer discards a supplied `slug` when `body` is given.**
  The argument was documented as "ignored when `body` is supplied", which
  silently broke the task-references-its-scenario promise `/ductus:groom` and
  `/ductus:amend` both make: a caller assembling its own body lost the
  `scenarios/{slug}.md` line with nothing to catch it, and the
  `scenario-consistency` family reads that line, so such a task looked like one
  with no scenario rather than one whose link was dropped. The pointer now
  renders above the caller's items whenever both are supplied. A caller wanting
  a bare body says so by omitting `slug`.

### Added

- **`check-command-flags`** — reports flags a command's Flags table documents
  but its `argument-hint:` frontmatter omits. `argument-hint` is the surface a
  host renders when it offers a command, so a flag absent from it is one the
  operator is never shown: an adopter reported `--since` as "doesn't show as an
  option" while `review.md`'s table listed eight flags and its hint named three,
  and the flag had been plumbed through `compute-review-scope` the whole time.
  Surfaced as `/audit` Family 30. Maintainer scope — the subject is
  `framework/commands/`, where the divergence originates, not the generated
  copies an adopter cannot repair.

## [0.33.0] — 2026-08-27

### Added

- **`lint-markdown` prefers a vendored `markdownlint-cli2`.** When
  `node_modules/.bin/markdownlint-cli2` exists at the repo root it is invoked
  directly, removing the `npx` dependency for projects that vendor the tool.
  A path check, not a shell — deterministic and cheap enough per call. A
  vendored binary that cannot be launched reports *itself* rather than falling
  back to `npx`, because a broken vendored tool is a condition worth seeing.

### Fixed

- **A spawn failure names the executable, not the repository.** `lint-markdown`
  mapped any spawn error to `Io { path: repo }`, so an unresolvable `npx`
  produced `I/O error on <repo>: No such file or directory` — which reads as a
  missing repository or fixture, the one thing that is definitely present. The
  new `PrimitiveError::ToolLaunch` names what could not be launched, and on
  `NotFound` explains that a shell-function `npx` (nvm's lazy loader) is
  invisible to a spawned process, which inherits `PATH` but never the parent
  shell's functions. Guidance is attached only to `NotFound`, and only on the
  `npx` branch: a permissions error, or a failure of a vendored binary that
  never consulted `PATH`, must not carry a `PATH` explanation. This is the
  misdirection that cost most of a session on 2026-08-27 — it blocked
  `check-review-gate` twice (forcing the markdown-only fallback), failed
  `runtime/tests/mcp.rs` on an unmodified tree, and got a commit rejected by
  the pre-commit hook, each presenting as an unrelated failure. It is invisible
  in CI, where `npx` is a real binary.

## [0.32.0] — 2026-08-27

### Added

- **`check-review-gate` names the durable contracts it could not examine.** The
  staleness check compares committed trees, so a `scenarios/*.md` or
  `data-model.md` living only in the working tree is outside it — and at the
  moment the gate runs, that is the normal state rather than an edge case: a
  scenario written during the session is uncommitted, `reviewed-against` is
  `HEAD`, the diff is empty, and `passed: true` reads as "examined and current"
  when nothing was examined. A passing gate now carries `guidance` naming those
  contracts. It does **not** block — committing before reviewing is a workflow
  choice, not an error; what it removes is a clean verdict that is silent about
  its own blind spot, the `QUAL-CLAIM-001` contract applied to the gate itself.
  A genuine `ReviewStale` block still wins, a clean tree emits nothing so
  silence keeps meaning "examined", and a working tree that cannot be inspected
  at all reports that rather than returning the same value as "nothing dirty".
  Scoped to durable contracts for the same reason the staleness check is:
  `tasks.md` is rewritten by `mark-task` on every task, and a notice that fires
  on nearly every run is learned-ignored.

### Fixed

- **`/ductus:implement`'s gate enumerations were one check short.** Step 14 and
  the completion gate's step 5 both listed three checks — the latter opening
  with "evaluates all three checks below" — while the primitive evaluates four.
  `ReviewStale` appeared in neither, so the deterministic path and the
  documented markdown-only fallback enforced different gates, with the fallback
  the weaker one and silently so. The primitive's own module doc had drifted
  the same way ("all three block reasons" against five variants).

## [0.31.0] — 2026-08-19

### Added

- **`unparseable` on both `derive-*` results.** A spec whose frontmatter opens
  a block it never closes gave neither splice an anchor, so it was left alone
  and never appeared in `updated` — byte-identical to a spec that genuinely
  needed no change. It is now named. Reported rather than repaired, and never
  an error: an unparseable spec is an unknown, not drift, so the CLI's blocking
  contract is unchanged. The contract this completes is `QUAL-CLAIM-001`'s —
  an empty `updated` means examined-and-clean only when `unparseable` is also
  empty, the same pairing `check-artifacts` has with `skipped`. A file with no
  frontmatter at all is not unparseable; there is no block to close. Detection
  lives in the shared scanner so the two primitives cannot drift on it.

## [0.30.0] — 2026-08-19

### Added

- **`derive-dependencies` and `derive-references` primitives.** The two derived
  frontmatter indexes — `dependencies:` from body links to sibling specs, and
  `references:` from cross-service spec URLs — are now derived by the runtime.
  Both take `--staged` (limit the rewrite to the pending commit) and `--write`;
  **without `--write` they report and never touch the tree.** Read-only is the
  default because the subprocess interpreter dispatches a backticked primitive
  with no arguments, so a writing default would let a safety-net step in
  `/target` or `/clarify` rewrite the whole corpus. `derive-dependencies` also
  runs the dependency-cycle check over the full graph and exits non-zero on a
  cycle, or on drift found by a report-only run — the shape a pre-commit hook
  and CI read.

### Removed

- **The shipped bash generators.** `.ductus/scripts/gen-spec-deps.sh`,
  `.ductus/scripts/gen-cross-service-refs.sh`, and their shared
  `lib/specs-root.sh` are deleted, along with their two bash test suites. The
  `generator-primitives` migration removes them from adopter trees and warns
  when a *pinned* pre-commit hook still calls them. §runtime-boundary principle
  3 names shell pipelines that parse frontmatter or markdown structure as not a
  sanctioned substitute for a primitive; the constraint that had kept them in
  shell — the runtime being optional — was retired by 0.28's acquisition work.

### Changed

- **The pre-commit hook halts when the runtime is unreachable**, rather than
  skipping the derivation. These indexes are captured by the commit, so a
  silent skip lands values the primitives had already superseded — the failure
  that reached adopters on 2026-08-17. Safe to make blocking because the hook
  cannot outrun the binary: `/ductus` wires `core.hooksPath` and acquires the
  runtime in the same run, and `core.hooksPath` is local git config a clone
  never carries. `git commit --no-verify` is the deliberate bypass.
  `label-criteria` keeps its swallowed failure — a missing `AC{n}` label is
  recoverable and audited, while a stale index is committed wrong data.
- The hook matches staged specs by *shape* (any leading segment, then
  `NNN-slug/spec.md`) instead of a configured root name, so it has no value to
  get wrong on a non-default `[paths] specs-root`.

## [0.29.11] — 2026-08-19

### Fixed

- `check-orphaned-references` now examines the spec root's `system.md`
  alongside `CLAUDE.md`, `AGENTS.md`, `README.md`, and `.githooks/pre-commit`.
  `system.md` is `create`-strategy and adopter-owned like the other four, but
  nothing had it in scope on either side: `ductus-rename` step 3 re-points the
  constitution reference in three of them and no migration step names
  `system.md`, while the orphan check did not look at it. An adopter whose
  `system.md` referenced a framework path was therefore left with a file
  pointing at a directory that no longer existed, and the run completed clean —
  found on a live adopter bootstrap by a host reading the file rather than by
  any check.

  The referrer is resolved through `[paths] specs-root`, so a project that
  renamed its spec root is examined at its configured location; a hardcoded
  `specs/system.md` would have examined nothing there and reported clean for
  the wrong reason. Consistent with the existing contract, the check
  **reports** rather than repairs: a migration re-points only the references
  the framework itself wrote, so an adopter-authored one is surfaced and left
  to the adopter.

## [0.29.10] — 2026-08-18

### Fixed

- `compute-review-scope` returns `scope` as the **union** of `plan-affected` and
  `modified-since` rather than whichever set is larger. Taking one set alone
  could exclude the files the work touched: a mature spec's `plan.md` lists its
  whole surface, so on a small follow-on scenario the plan set won on size and
  the review was scoped to files the change never went near, with nothing in the
  report signalling that the changed code went unexamined. The failure grew with
  spec maturity, which is backwards for a quality gate. See
  `specs/022-deterministic-runtime/scenarios/review-scope-union.md`.

## [0.29.9] — 2026-08-17

### Fixed

- **`check-orphaned-references` could not see the orphan it was written to catch.** A reference was examined only when it carried a *current* framework-managed root (`.ductus/`, `.githooks/`, the configured spec root) as a prefix — but the orphan a migration chain leaves behind is a reference to the path *before* the move, which by definition carries the *old* root. Found on the real adopter bootstrap for spec 048's AC10: the generators had moved root `scripts/` → `.govern/scripts/` → `.ductus/scripts/`, the adopter's `AGENTS.md` still named `scripts/gen-spec-deps.sh`, and the primitive returned clean across all four referrers while that orphan sat in one of them. The operator found it by reading the file. The blind spot was known and worked around rather than closed — this primitive's own test carried the comment *"`.govern/` is not a managed root, so the stale reference must be caught by naming a path under one that does not resolve"* and built its fixture accordingly. `managed_roots` now also matches the historical roots `.govern/`, `scripts/gen-` and `scripts/lib/`. The pre-042 entries are prefixes rather than the `scripts/` directory on purpose: the framework owned `scripts/gen-*.sh` and `scripts/lib/`, never the whole directory, so a `scripts/` root would turn every unresolved adopter script into a finding.

### Added

- **`matched-prefixes` on the `check-orphaned-references` result.** `examined` already bounds the claim by *subject* — which referrers were read — and this bounds it by *scope*: which reference forms were recognized. The two are not interchangeable, and the difference is what hid the defect above. A reference carrying no listed prefix is never reported under `skipped`, because nothing recognized it as a reference at all, so a clean result without this field asserts *no orphans* while meaning *no orphans among paths carrying one of these prefixes*. `QUAL-CLAIM-001` about the boundary of a claim rather than its subject. Additive and `serde(default)`, so an older host reading the result is unaffected.

## [0.29.8] — 2026-08-17

No runtime change. `0.29.7` carried the `check-artifacts` change below but never published: its release gate failed the framework self-audit, so no assets and no crates.io publish exist at that version and the `version` pin had to move rather than be reused.

### Fixed

- **A release-gate failure caused by review bookkeeping, not by code.** Spec 000's review was recorded with `reviewed-against` pointing at the HEAD that existed *while* the review ran, and the scenarios it reviewed were then committed on top of it. `/ductus:audit` Family 19 compares `reviewed-against..HEAD` for every `done` spec, so the contracts read as changed since their own review the instant that commit landed. The local audit had passed because it ran **before** the commit, where the edits were invisible to a commit-to-commit diff — a green that meant nothing, which is the failure mode this repository pays for most. The review is re-recorded against the commit that actually contains its subject, and `AGENTS.md` gains the ordering rule: commit the work, review against the new HEAD, commit the review, and run the audit after committing rather than before tagging.

## [0.29.7] — 2026-08-17

The runtime half of spec 000's `scenario-without-task-visibility`: a scenario hand-added to a `done` spec, never implemented and carrying no questions, used to be invisible to every check.

### Changed

- **`check-artifacts`' scenario→task mapping no longer skips `done` specs wholesale.** It returned before examining anything (`if status == "done" { return; }`), on the reasoning that a spent task may have been pruned and `tasks.md` is not a durable index (§tasks-phase). That is true of the file's *current* contents and false of its *history*: an implemented scenario had a task at some point, and one hand-added and never implemented never did. On a `done` spec an unmapped scenario is now a finding only when **no revision of `tasks.md` ever named its slug**, so the pruned case stays a non-finding on evidence rather than on a blanket skip.

  Derived, never declared — a marker would be missed by exactly the authors who bypassed `/ductus:amend` to hand-add the scenario in the first place. Reading the code was excluded by `analyze.md`'s own Scope Boundaries.

  **Advisory, and honest about its limit.** History proves no task ever existed; it cannot separate a scenario documenting already-shipped behavior from one describing unimplemented work. The finding says which of those it cannot tell and leaves the reopen to the operator rather than performing it.

  **Any inability to consult history suppresses the finding** — no repository, unreadable history, non-UTF-8 blob. §tasks-phase mandates that direction, so a check that cannot look must not manufacture a finding from its own blindness. This is deliberately the opposite default from `mechanical_sweep`, where an unreadable diff withholds an *exemption*: there the unprovable thing is a claim of sameness, here a claim of absence.

  Measured over this repo's 46 `done` specs before shipping: one unmapped scenario existed and *was* tasked historically, so the file-shape alternative would have produced exactly one finding and it a false positive. This rule produces zero, and a fixture reproducing the never-tasked shape does fire.

## [0.29.6] — 2026-08-17

Two findings from `/ductus:review` against spec 027, both in `check-orphaned-references` itself — code shipped hours earlier in `0.29.4`.

### Fixed

- **A reference that escapes the repo is no longer treated as a managed path.** Candidates come from a referrer file's *contents*, and `.` and `/` are both path characters, so `.ductus/../../elsewhere/gone.md` matched a managed-root prefix and was joined onto the repo root unexamined. Nothing was opened — the check is existence-only — so this was never an exploit; it was a category error. A path outside the repo is not a ductus-managed path, so reporting it as a broken managed reference makes a claim about something this check does not govern. `apply-manifest` already refuses a destination that escapes the repo root; the same rule now applies where paths *enter* rather than only where they are written.
- **`covers` carried a disjunct that could never fire.** The directory-prefix test read `target.starts_with("{dir}/") || claimed.ends_with('/') && target.starts_with(claimed)`, but `claimed` is normalized with `trim_end_matches('/')` a line earlier, so the second arm matched nothing the first had not already matched. One prefix test now, with both written forms of a registry entry (`.ductus/` and `.ductus`) covered by a test that also pins the sibling-prefix case (`.ductus` must not cover `.ductus-cache/`).

## [0.29.5] — 2026-08-17

The runtime half of spec 048's `state-b-continues-in-session` — a CLI argument surface for the two primitives that had none, which is what a State-B `/ductus` run needs to finish its work in the session that acquires the runtime instead of deferring it to the next one.

### Added

- **`apply-manifest` gains `--entries-json` / `--pinned-json` / `--substitutions-json`, and `enforce-manifest` gains `--expected-json` / `--pinned-json`** — each a path to a JSON file holding that field's value. These fields are arrays and maps of objects, which clap cannot express as flags, so they were `#[arg(skip)]` and reachable only through the JSON context: fine while the CLI surface of these two primitives was "a debug entry point, not the production path", which is exactly how it was documented.

  State B changes that. When `/ductus` acquires the runtime mid-run, the binary is on disk and the CLI is permission-seeded but no MCP server is live until the next session, so the CLI *is* the production path for that run. Walking the eight primitives invoked after the pre-flight phase found all eight had CLI subcommands — availability was never the constraint — but `apply-manifest`'s `entries` is the manifest itself. Called from the CLI as it stood, it would have received an **empty** manifest, copied nothing, and **reported success**, because an empty manifest is a legal one; the run would then have continued through Per-Agent Scaffolding on a project that received not one shared file. The scenario's open question asked for exactly this walk before the abort was moved, and it is the only reason the hazard was found first.

  The new flags are `#[serde(skip)]`, so the MCP tool schema is unchanged and an MCP caller passing one is still rejected as an unknown field. An unreadable or malformed file is an **error**, never an empty default — the silent-empty path is the whole thing the surface guards against, and it is the case the new tests assert hardest.

## [0.29.4] — 2026-08-17

The runtime half of spec 027's `migration-chain-reference-integrity` — the class of defect a real adopter bootstrap surfaced twice, where a migration chain converges every file it owns and silently breaks the ones it does not.

### Added

- **`check-orphaned-references`** — reports adopter-owned files whose references to ductus-managed paths no longer resolve. Each migration is authored against the layout as it stood at its own `introduced_in` and is correct there; nothing validated the **composition**, and an adopter far enough behind runs several in one batch, so the composition is what they actually execute. The referrers are the `create`-strategy files the manifest never overwrites and no migration re-points — `CLAUDE.md`, `AGENTS.md`, `README.md`, `.githooks/pre-commit` — and both real instances lived there: `constitution-relocate` wrote a constitution reference into the first three and `ductus-rename` then moved the file, leaving a dangling `@import` that yields a constitution simply not loaded; and `govern-dir-consolidate` plus `ductus-rename` moved the generators out from under an adopter-owned `pre-commit` that neither re-pointed, so the hook failed at commit time far from the run that broke it. Nothing errored in either case, which is why nothing caught them.

  One primitive, **two call sites**, so one rule cannot become two that disagree: `/ductus:analyze` §Project-level consistency is the durable adopter-facing surface (it ships in the Shared Files manifest, is read-only and advisory, and runs on every invocation regardless of target), and `/ductus`'s Pre-run Migrations calls it at batch end, where it runs closest to the cause. `/ductus:audit` was considered and rejected: it is maintainer-only and absent from the manifest, so an audit family could only ever check this repository, never the adopter whose config both instances were found in.

  **Attribution has two modes and the result names which one ran.** With `framework/migrations.toml` readable — the bootstrap site, where the registry is in the fetched archive — a finding names the entry whose `target_paths` covers the missing path, latest hop winning since that is the one that left the reference dangling, and `attribution` reads `registry`. Without it — an adopter checkout, where the registry is never installed — `attribution` reads `watermark` and `[migrations].last_applied` is the only migration context there is. An empty `migration` therefore means *asked, and no entry claims it* under one mode and *nothing to ask* under the other; rendering them alike would assert an attribution nobody computed, which is `QUAL-CLAIM-001` inside a check written to enforce it. Read-only throughout: it reports and never repairs, because the adopter may have hand-edited the reference and a wrong rewrite is worse than a precise report.

  Two exemptions were earned by running the primitive against ductus's own repo before shipping it, where it produced thirteen findings and zero real defects: a candidate containing `*` or `NNN` is documentation naming a *shape* (`specs/NNN-*/spec.md`), not a file anyone expects to exist; and a path the project declares it *ships* to adopters is a manifest destination written on the way out, not local breakage — the same distinction `criterion-path-existence` draws, reusing its two helpers rather than a second copy of the rule.

## [0.29.3] — 2026-08-16

One fix, found by `/ductus:review` running against spec 010 and reported as an advisory `QUAL-CLAIM-001` — the rule this repository wrote about its own machinery, firing on the machinery that enforces review staleness.

### Fixed

- **A `SweepIndex` built from a diff walk that aborted partway could grant exemptions it had not earned.** `SweepIndex::build` discarded the result of `git2`'s `diff.foreach` with `let _ =`, but a walk that fails partway still leaves the event buffer holding whatever prefix was delivered. Folding that prefix is worse than folding nothing: a file whose *leading* hunk was a rename but whose *later* hunks were structural lands in the per-file index as a pure token substitution, so `changed_beyond_spelling` reports `false`, `check-review-gate` calls the review current, and a spec advances to `done` on a contract change nobody read. The two failure branches of one function also disagreed on direction — a failed `diff_tree_to_tree` already returned an empty index, which is conservative, while the discarded `foreach` error was not. Both branches now return one named `SweepIndex::unreadable()`, so "could not read the diff" has a single representation and every path reports as changed. The direction is deliberate and opposite to the enclosing gate's: `check-review-gate` fails **open** on what it cannot determine, but an exemption is a positive claim that a contract did not really change, and that claim has to be earned from a diff that was actually read (spec 022 scenario `review-staleness-on-done-specs`).
- **`ductus derive-routing-candidates` had no help text.** It was the only variant of the CLI's `Command` enum without a doc comment, and `clap` derives each subcommand's description from exactly that — so it was the one subcommand of roughly fifty listed blank in `ductus --help`. Nothing catches this: no audit family reads clap help text, and the six-site wiring checklist for a new primitive (`AGENTS.md` §Gotchas) tracks the sites a *missing* entry breaks, while this one compiles and runs correctly and is only wrong to a human reading the help.

## [0.29.2] — 2026-08-16

Two defects found by the thing spec 048's AC10 exists to do — running `/ductus` against a real pre-042 adopter, where no CI job could stand in for it. Both were invisible to every check this repository runs against itself.

### Fixed

- **`merge-managed-block` stranded the old block's tail when a subsection's patterns were renamed.** The `line-prefix` style has no end marker, so a block's extent on disk is *inferred* by aligning its blank-line-delimited subsections against the canonical block's, matching on pattern lines (comment wording drifts between releases; patterns don't). A subsection whose patterns were **all** renamed therefore matched nothing, and the walk treated that as the end of the block — stranding everything below it. The cross-boundary dedup pass then removed the stranded pattern lines and left their comment headers orphaned. The `govern → ductus` rename is exactly this shape: `.govern.session.toml` became `/.ductus/session.toml` and that subsection carries no other pattern, so an adopter's `.gitignore` came out of the migration with a dead `.govern.session.toml` plus headerless `# IDE` and `# OS` comments — every run, never converging. Its neighbour survived only by luck: `.govern-cache/` was renamed too, but its group also lists `*.sqlite`, which did not change. The walk now distinguishes a retired subsection from adopter content by what follows it — when a later on-disk group still aligns, the block has not ended — with a **bounded** lookahead, because an adopter tail is arbitrarily long and an unbounded scan would reach a pasted duplicate far below and consume everything above it. The trailing-append invariant (`scenarios/merge-managed-block-trailing-append.md`) is unchanged and its tests are untouched. This corrects a documented trade-off rather than an unknown bug: the walk's own doc comment recorded the stranding as "preservation over consumption when identity is ambiguous" — the ambiguity turned out to be resolvable, and the cost of not resolving it was a real adopter's config (spec 022 scenario `merge-managed-block-renamed-subsection`).

### Changed

- **The `ductus-rename` migration now renames the pre-commit hook and re-points its invoker.** The manifest ships `.githooks/ductus-pre-commit`, but nothing renamed `.githooks/govern-pre-commit` and nothing rewrote the adopter-owned `.githooks/pre-commit` that invokes it — that file is `create`-strategy so it is never overwritten, it is not pinned so the pinned-invoker warning never fired, and no migration named it. It fell through every net. The consequence was not cosmetic: the old hook body calls the generators by their pre-042 root paths, which `govern-dir-consolidate` moved to `.govern/scripts/` and `ductus-rename` moved again to `.ductus/scripts/`, so an adopter running the full chain was left with a pre-commit hook calling a script whose own targets no longer existed — **silently broken by the chain meant to converge it**. The migration gains the rename, the invoker rewrite, a warning for a hand-rewritten `pre-commit`, and matching `target_paths` and idempotency-check entries.
- **`/ductus:implement` offers the next task when one completes.** After the per-task summary and before exiting, a default-mode run names the next unchecked task — number, heading, and how many remain after it — and asks whether to continue. Yes continues in the same run, no exits cleanly as before, and free-text instructions redirect. One task, not a menu: the count is what lets an operator judge the queue without re-reading `tasks.md`. Suppressed entirely when nothing is unchecked, so it never renders as "no next step". `--auto` is unchanged, nothing about the offer writes, and the `in-progress → done` transition keeps its own separate confirmation. On `ductus exec` the walk cannot loop back to an earlier step, so the offer is rendered in the summary and continuing is a fresh invocation — a documented reduction rather than a silent one (spec 010 scenario `implement-offers-the-next-step`).

## [0.29.1] — 2026-08-16

Two fixes in the framework's own machinery, found by taking the measurement spec 022 task 88 asks for *before* its design is finalised. The measurement invalidated the task and exposed a live defect instead.

### Fixed

- **The review-staleness gate fired on rename sweeps — 19 of 46 `done` specs.** `check-review-gate` blocks the `in-progress → done` transition when a spec's durable contracts changed since its recorded `reviewed-against`. It had no mechanical-sweep exemption, so a uniform token substitution across live artifacts — the thing §spec-lifecycle case (a) explicitly calls a *mechanical* edit, and the reason a rename does not reopen a `done` spec — read as a contract change. Measured across this repo: 19 of 46 `done` specs would have been blocked, every one a consequence of 049's `govern → ductus` sweep and none a real change. `005-workflows`, for instance, differed only in `/govern` → `/ductus` inside its `data-model.md`. `/ductus:audit` Family 19 has applied the exemption since it was written and reported 0, so the two enforcement moments disagreed on 19 specs with nothing comparing them. The incident that prompted the investigation was itself an instance: `017-derive-dont-ask` blocked on three contracts that all changed in one commit, the rename. The new `primitives::mechanical_sweep` module carries the rule — one-for-one line runs, equal token counts per line, and rewrites that recur across more than one file, with file-local variants admitted when the repo-wide rewrites explain them — and `stale_review_block` filters through it. A meaning change riding along with a sweep still blocks, because its rewrite is not one the sweep made anywhere else.
- **An abbreviated `reviewed-against` made the staleness check fail open.** `git2`'s `Oid::from_str` zero-pads a short hex string into a 40-character id that matches nothing, so the whole check bailed silently for any spec recording an abbreviated sha (`012-multi-agent-govern` records `d904430`) while Family 19's `git cat-file -e` resolved it and checked. Now resolved with `revparse_single`. A check that silently cannot run is the failure mode this project pays for most, and this one was sitting inside the check that exists to catch it.
- **`/ductus:audit` Family 19 crashed on Windows.** `scripts/audit/review-freshness.sh` decoded `git diff` output with the platform locale, which is cp1252 on Windows and raises `UnicodeDecodeError` on the first em-dash in a spec. Latent since the family was written, because it only ever runs on Linux in CI. Now pinned to UTF-8 with `errors="replace"`, so an undecodable byte reports the spec stale — a finding, the safe direction — rather than crashing a release gate. Adopters running the audit locally on Windows are the ones this affects.

### Added

- **`primitives::mechanical_sweep::SweepIndex` is public.** It is reachable from integration tests, which is how `tests/mechanical_sweep_parity.rs` runs the Rust rule and Family 19's Python rule over the real corpus and fails on the first disagreement. The two implementations exist because the CI job running the self-audit has no Rust toolchain — it *gates* the build, so calling the runtime would make the gate depend on compiling the artifact it gates. The test is what keeps them from drifting again in silence; it asserts that it compared something, so a vacuous pass is itself a failure.

## [0.29.0] — 2026-08-16

Two instances of spec 017's own principle — *never design a framework feature that depends on an author remembering* — found in the framework's own machinery and closed. Both land here per the runtime-work routing rule; the requiring spec keeps the criteria (017 AC25, AC26).

### Added

- **`write-review` takes an `observations` array, and recording one is what captures it.** A reviewer who notices something real that maps to no loaded rule previously had nowhere structured to put it: inventing a rule is forbidden, and the free-text Summary is regenerated wholesale on the next run *and* is per-spec, so a cross-cutting note filed there was both erased and misfiled. Each observation now renders in a new `## Observations` report section **and** is appended to `{specs-root}/inbox.md` in the same call, dedup-guarded on the whole rendered line so a re-run over an unchanged repo appends nothing. There is no path that records an observation in the report without also recording it in the inbox, and no separate `append-inbox` call to forget. The inbox write happens **before** `review.md` and an I/O failure fails the whole call — a report claiming a capture that did not happen is the defect this removes, and the reverse order would reintroduce it one level down. Observations never enter the MUST / SHOULD / low-confidence counts, never affect `blocking`, and never change the exit code; the result reports `observations` and `observations-captured` so *nothing to capture* is distinguishable from *capture ran and everything was already there*. `performReview`'s response gains a matching optional `observations` array, accumulated across passes exactly as `findings` is — without that leg the section would render `*None.*` on every run whether or not the reviewer had any. A run with no observations produces a byte-identical report and leaves the inbox untouched (specs 017 AC25, 022 `review-observations-write-through`).
- **New primitive `derive-routing-candidates`, and `/ductus:specify` now routes before it scaffolds.** `/ductus:groom` walks a five-route decision tree before anything is written; `/ductus:specify` had no equivalent, so the routing rules that exist to *prevent* a new spec bound only for work arriving through the inbox — work arriving through conversation bypassed the tree entirely, at the one moment the rules matter. The primitive derives candidate homes from three sources: **runtime-work** (the description names a primitive or a `runtime/` path; the home is the spec whose `plan.md` claims `runtime/` under Affected Files — derived from the corpus, never a slug named in code, so an adopter's runtime-owning spec is its own and a project with none yields no candidate rather than a wrong one), **rule-surface** (a rule file whose category stem matches, with the `-backend`/`-frontend`/`-cross` suffix set aside), and **spec-corpus** (a spec whose slug shares vocabulary). Matching is lexical and advisory: the semantic decision stays at the `routeInboxItem` extension point — groom's tree, reused rather than copied — and the choice stays with the operator, who can create a new spec over any candidate. A candidate that is `done` carries `reopens: true` so the confirmation names the back-edge before it happens. Every source lands in `sources-examined` or in `skipped`, so empty candidates with an empty `skipped` means *examined and matched nothing* while a non-empty `skipped` means *could not derive* — different answers, per `QUAL-CLAIM-001`. A corpus with no rule files and no matches proceeds unchanged, so a fresh adopter sees no new friction, and `routed-by` returns `gate-required: false` so a groom-initiated specify is not asked twice (specs 017 AC26, 022 `specify-routes-before-scaffolding`).

### Changed

- **`routeInboxItem`'s request gains an optional `candidates` array**, omitted when empty so the `/ductus:groom` payload is byte-unchanged. This keeps **one** routing point across both entry points rather than two rules that could disagree; `item-text` falls back to the `description` key on the specify path.
- **`/ductus:specify`'s procedure is now nine steps**, with the routing derivation, the routing decision, and its confirmation ahead of `create-feature`. Denying that gate ends the run cleanly with no directory created and no session write. The `specify-basic` parity golden is updated for the new walk.

## [0.28.1] — 2026-08-16

Two fixes found by `/ductus:review` after the `0.28.0` tag, both in the framework's own machinery.

### Added

- **`read-spec` reports scenario files it could not read.** A new `scenario-files-unreadable` field sits alongside `scenario-open-questions`, and `check-artifacts` records each unread scenario as a `skipped` target (family `scenario-open-questions`, reason `artifact-unreadable`). Previously an unreadable scenario contributed no questions and was dropped silently, so an empty question list meant either *every scenario was examined and none carries a question* or *a scenario could not be examined* — and every surface asserted the first. Nothing gains a block: an unknown is still not escalated into a defect, and `check-review-gate` still fails open. The field is omitted when empty, so existing payloads and the parity goldens are byte-unchanged (specs 046, 022 `unreadable-scenario-is-reported`).

### Changed

- **BREAKING (opt-in escape hatch): `GVRN_FETCH_ALLOW_INSECURE_HOSTS` is now `DUCTUS_FETCH_ALLOW_INSECURE_HOSTS`.** The 0.28.0 rename missed this variable — its Rust constant carried no project token, so the sweep's identifier grep passed over the string literal. The old name is **not** honoured as a fallback: keeping it would leave a live reference to the retired project name, which is exactly what spec 049's AC1 forbids. If you set the old variable for an internal mirror or local testing, rename it. The failure mode if you do not is safe and loud rather than silent — the host is screened again, so the fetch is refused with an error naming the scheme or the internal address, never quietly allowed. The variable only ever loosens the SSRF guard, so losing it fails closed.

## [0.28.0] — 2026-08-16

The project is renamed to **ductus**, and the runtime it ships stops being optional.

### Changed

- **The binary, crate, library, and MCP server key are all `ductus`.** One name across the project, the release tag prefix (`ductus-v*`), the per-project directory (`.ductus/`), and the default command namespace (`/ductus:`). The retired `gvrn` crate is left published rather than yanked so an existing `cargo install` keeps resolving, and the 47 historical `gvrn-v*` tags and their assets are untouched — they are the record of what was named when (spec 049).

- **The per-project directory resolves across three tiers.** `.ductus/` first, then the 042-era `.govern/`, then the pre-042 repo-root files. An adopter who upgrades the binary before re-running the bootstrap sits on the middle tier and keeps working, so the fallback grew a level rather than swapping one out. Reads name the oldest tier when nothing exists (the caller treats that as "config absent"); writes name the newest, so a fresh project cuts over immediately while a pre-migration write stays on the file that already holds the other sections (022 scenario `project-directory-resolution-chain`).

- **The runtime is required, and `/ductus` acquires it.** `PATH` is no longer consulted anywhere. `/ductus` reads the version this framework revision pins, downloads the matching release asset, verifies its sidecar digest, and installs it into a ductus-owned store at `~/.ductus/bin/ductus`, reached from each project through a gitignored pointer at `.ductus/bin/ductus`. A project may supply its own binary with `[runtime] path` instead. Acquisition failure halts the run naming the store path and the release URL rather than degrading silently (spec 048).

  This amends the constitution: §runtime-boundary principle 3 was "Opt-in for adopters", and the Opt-in invariant asserted a full pipeline cycle with the binary absent. Both are replaced — by the requirement, and by an Acquisition invariant asserting the published assets can actually be fetched, verified, installed, and executed on every supported platform.

- **Detection collapses from three states to two.** State C existed to report a missing binary and suggest installing one; acquisition removes its subject.

### Added

- **`label-criteria` reaches adopters.** The `criterion-label-backfill` migration assigns stable `AC{n}` labels across an adopter's corpus, so citing a criterion never requires counting a list (spec 013).

- **Three registry migrations**, applied in one bootstrap run: `criterion-label-backfill`, `ductus-rename` (the per-project directory, the MCP server key and command, the per-agent permission entries, the installed bootstrap entry point, and the two managed-block markers), and `runtime-store-path` (the MCP command path).

- **Two self-audit families.** Family 21 asserts the retired bootstrap path still serves the current bootstrap byte-for-byte — every pre-rename adopter's self-update fetch is hardcoded to that path, and their bootstrap aborts before migrations when it 404s. Family 19 gains a mechanical-sweep exemption so a uniform rename no longer stales every review it touches (spec 026).

### Fixed

- **A deleted file's diff lines were attributed to the previously-seen file** in Family 19's parser, because `+++ /dev/null` does not match `+++ b/`. A pure rename would have read stale indefinitely.

- **An empty resolution chain resolved to the repo root.** `unwrap_or_default()` over the path chain yielded `""`, and `repo.join("")` is the repo directory — guarded only by a `debug_assert` that compiles out of release. The chains are fixed-size now, so the state is unrepresentable.

## [0.27.2] — 2026-08-14

022 scenario `append-primitive-marker-normalization`. A patch: both fixes are corrective, and neither adds a blocking outcome an adopter will newly hit.

### Fixed

- **Every `append-*` primitive doubled a caller-supplied list marker.** Each renders its own marker — `append-inbox` and `append-task` a checkbox, `append-question` a plain bullet — so text arriving with one already attached was written as `- [ ] - [ ] text`. Nothing on the way in caught it: the caller cannot see the rendering, a doubled marker is valid markdown so `lint-markdown` passes, and the write is atomic. It surfaced only when a human read the file. Observed against this repo's own `specs/022-deterministic-runtime/tasks.md`, where an `append-task` call wrote six doubled checkboxes.

  All four affected arguments now strip one leading marker before rendering, through the `bullet_text` helper `append-inbox`'s dedup and `remove-inbox-item`'s removal already share — reuse rather than a second matcher, so the write side and the dedup read side cannot disagree about which inputs carry a marker. One marker, never more: the failure mode is a single doubling, and stripping to exhaustion would eat content from text that legitimately begins with a dash (`--fix reverts a drifted done spec` is untouched, and tested).

- **`append-inbox`'s `dedup-prefix` silently matched nothing when it carried a marker.** The more consequential half, and it was latent rather than newly introduced. `has_bullet_with_prefix` compares against `bullet_text`-extracted content, which has the marker already stripped — so a prefix like `- [ ] SEC-BE-014:` could never match any bullet, the guard no-opped, and the append went through on every run. That is the guard `/{project}:analyze`'s finding capture relies on for the §brownfield-inbox promise that *re-running an audit against an unchanged repo records nothing new*; a marker-bearing prefix quietly turned an idempotent audit into an accumulating one. The prefix is now normalized exactly as the text is.

  Both arguments are normalized **before** the dedup comparison, not after, so an entry can never be stored in one form and matched in another.

- Each affected argument's schema description now states that the primitive renders the marker and strips a caller-supplied one. An undocumented normalization is its own surprise, and the argument description is where a caller looks.

## [0.27.1] — 2026-08-03

### Fixed

- **The self-audit's release gate failed under CI's shallow checkout.** Family 19 resolves each spec's `review.reviewed-against` against history, and `actions/checkout` clones a single commit by default — so the first tag after the family was wired reported "not a commit in this repo" for all 48 specs and skipped the publish. It had passed locally on every run, because a developer checkout has full history; the environments differ in exactly the dimension the check depends on. Both workflows that run `run-all.sh` now set `fetch-depth: 0`. The unresolvable-sha case stays a finding rather than a skip: with history present it means a rewritten or bogus sha, which is worth reporting.

- **`ReviewGateBlock::ReviewStale` blocked 34 of 48 specs.** v0.27.0 scoped the staleness test to the plan's **Affected Files**, on the reasoning that Family 19 could afford a narrow rule (it judges every spec at release) while the gate could afford a wide one (it judges one spec at completion). That reasoning was never measured. Old specs list shared surfaces — `AGENTS.md`, `README.md`, `framework/bootstrap/govern.md` — that every later spec also touches, so completing spec 004 was blocked by spec 042 having edited `AGENTS.md`. A gate that blocks seven specs in eight is one people route around, which is the failure v0.27.0's own doc comment warned about.

  The gate now applies the **identical** rule `/gov:audit` Family 19 already uses: a review is stale when one of the spec's durable contracts — `scenarios/*.md` or `data-model.md` — changed since `reviewed-against`. `tasks.md` and `plan.md` are excluded because the first is ephemeral by construction and the second churns as Affected Files are revised. Measured across this repo: **34 of 48 blocked → 0**. Two enforcement moments, one definition of stale; the earlier difference between them was the defect, not the design.

  Simpler as well as more precise — the gate no longer parses plans at all, so `compute_review_scope::read_plan_affected` reverts to private and the Affected-Files matcher is gone.

## [0.27.0] — 2026-08-03

022 scenario `review-staleness-gate`. A minor rather than a patch because `check-review-gate` gains a blocking outcome adopters will see — a spec that passed the gate yesterday can be blocked by it today.

### Added

- **`ReviewGateBlock::ReviewStale` — the gate now asks whether a review still *applies*.** The four existing checks ask whether a review exists and whether it passed; none asked whether it covers the current code. So a review recorded against one commit and never re-run read as a pass forever, and hand-editing a `review.md` to mark findings resolved produced a record that satisfied every automated check while describing a diff that was gone. `gvrn-v0.26.2` shipped that way: tagged at `334907f` with spec 022's review reading `reviewed-against: 1f7ee722`, covering none of the adopter-scope suppression it released. Gate check 5 fires when a file the spec's plan declares under **Affected Files** changed between `reviewed-against` and `HEAD`, scoped to that declared surface rather than the repo — a repo-wide test would mark every review stale on the next unrelated commit. The spec's own `review.md` and `spec.md` are excluded as bookkeeping, since `write-review` touches both. `compute_review_scope::read_plan_affected` is now `pub(crate)` and shared rather than reimplemented.

  The check **fails open** on anything it cannot determine — no git repository, an unresolvable `reviewed-against`, a plan with no Affected Files table. A gate that blocked on its own blindness is one people route around, and the four honest checks still run. This is deliberately the opposite posture from Family 17's derivation, where an empty result means *checking nothing* and so must fail closed; here an empty result means *blocking nothing*.

### Framework

- **`/gov:audit` Family 19 (`review-freshness`)** — the release-time half of the same rule, asking it of every `done` spec at once. Scoped to durable contracts (`scenarios/*.md`, `data-model.md`) after the two wider rules were measured against this repo and rejected: the plan's Affected Files flags 42 of 48 specs (old specs list shared surfaces every later spec touches), and the whole spec directory flags 31 of 48 (`tasks.md` churns on every ticked checkbox and is ephemeral by construction). The shipped rule flagged 10 when it landed — a snapshot that drains as reviews are refreshed — and caught both real failures, `gvrn-v0.26.1` and `gvrn-v0.26.2`. **Not wired into `run-all.sh`** — it reports 10 pre-existing stale reviews, so wiring it would block the next release tag until they are cleared; that freeze is a maintainer's call, not a side effect of landing the check.
- AGENTS.md gains the canonical-table sync rule and the backtick-only-when-bindable rule, and **corrects** an entry that claimed `cargo build` can report `Finished` without recompiling. It cannot; the observation came from invoking `cargo build` twice in one shell command and reading only the second, no-op output.

## [0.26.2] — 2026-08-03

022 scenario `criterion-adopter-scope-destinations`, plus a follow-on to `numbered-heading-grammar-single-source`. Takes `criterion-path-existence` from 21 findings on this repo to **0**, with every suppression recorded rather than dropped.

### Added

- **`ships-to-adopter` skip reason.** `criterion-path-existence` reported 19 findings for paths `govern` creates in an *adopter's* checkout — `specs/templates/`, `specs/rules/*`, `specs/system.md`, `.githooks/govern-pre-commit`, `.govern/constitution.md`. Every criterion was correct and satisfied in the repo it describes; they failed only because this repo is the framework source. `root-absent` could not catch them (`specs`, `.govern`, `.githooks` all exist here), and the `in the project` marker only matched criteria that happened to use that phrasing. `adopter_destinations` now derives the destination set from the **Shared Files** manifest in `framework/bootstrap/govern.md` — the canonical registry of what lands where — and suppresses a candidate that equals a destination or is a directory containing one. An absent manifest (the adopter case) yields an empty set, so nothing is suppressed and findings are still emitted; in an adopter those paths resolve anyway. The check runs after the resolve and `root-absent` arms and records `skipped { reason: "ships-to-adopter" }`, so `clean: true` with a non-empty `skipped` keeps its `QUAL-CLAIM-001` meaning.

### Fixed

- **`prune-tasks` no longer allocates to classify a heading it discards.** `Option<(&str, &str)>` is `Copy`, so the phase test reads the borrowed split directly and the owned `String` pair is built only inside the task branch. A numbered heading above the task level — a flat-task remnant in a phased file — is now classified allocation-free, removing a regression against the code `numbered-heading-grammar-single-source` replaced.

### Framework

- `/gov:analyze` captures its findings to the inbox before rendering ([spec 047](../specs/047-analyze-findings-durability/spec.md)). `/gov:review` had a durable sink and analyze had none, so an audit's results lived only in the session that ran it. Prose-only — `append-inbox` already shipped with its dedup guard. Capture is a **host responsibility**: the primitive is named without backticks so the exec walker does not dispatch it, since per-finding arguments are not bindable per-step.
- Constitution §Automatic issue capture widens from incidental capture to cover a command whose primary output is findings, names `/gov:analyze` as a surfacing gate, and adds an idempotency requirement.
- Specs 005 and 025 had genuinely stale acceptance criteria — the two true positives the 19 false ones were hiding. 005 asserted workflow files sit under `framework/workflows/` (sunset by 043); 025 asserted a `scripts/lint-govern-toml.sh` that was never built. Both reworded; neither claim is live.

## [0.26.1] — 2026-08-03

022 scenario `criterion-non-assertion-phrasings`. A precision fix to `criterion-path-existence`, sized by reading every one of the 25 findings its repo-wide sweep produces rather than classifying them by path prefix.

### Fixed

- **Three non-assertion phrasings the marker list missed.** `deleted` replaces the narrower `is deleted` / `are deleted` pair, catching the past-tense-agent form a criterion uses when it names the commit that did the deleting (``after `531e3ea` deleted both`` — spec 045's own AC18). `(was` + space covers the parenthetical rename history (``(was` + space.claude/gov-session.json` pre-0.10.0)``), where the old path is named to date a change rather than to claim it is present; the opening paren keeps it from matching any past-tense `was`. `target paths` is a fifth group alongside the four already documented — a path named as the subject of a migration record is manifest data describing what to remove, not a delivery claim. Measured effect: 25 findings → 21, suppressing exactly the four residual false positives and nothing else, verified by diffing findings on `(spec, path)` keys.

### Unchanged, deliberately

- **The 19 adopter-scope findings stay.** Paths `govern` creates in an adopter's checkout (`.govern/constitution.md`, `specs/rules/*`, `.githooks/govern-pre-commit`, `specs/templates/`, `specs/system.md`) do not resolve in `govern`'s own repo, and spec 045 already triaged them as "a dogfooding artifact, not a check defect" whose class "does not generalize to the projects this check ships to". Suppressing them would ship `govern`-repo-only machinery keyed off a manifest no adopter has, so it would never engage where the check actually runs. 045's recorded verdict — do not promote, re-measure in an adopter repo — is unchanged.
- **Two true positives remain reported**, which is the point of the family: spec 005's `framework/workflows/` (sunset by 043) and spec 025's `scripts/lint-govern-toml.sh` (never built). Both are spec-level drift, not check defects.

## [0.26.0] — 2026-08-02

Spec 045, landing as 022 scenarios `block-element-scanner`, `check-artifacts-skipped-targets`, `link-adjacent-drift-family`, and `criterion-path-existence-family`. `check-artifacts` gains two advisory families that catch an artifact still describing a decision that has since been made — and a way to say what it could not examine.

### Added

- **`link-adjacent-drift` (advisory)** — flags an artifact's own prose asserting an open state that its own sibling link's target contradicts: a question called open while the target reports none, work called unbuilt while the target is `in-progress` or `done`. Six closed tells, matched only outside inline code spans; the scanned unit is the enclosing block-level element; evaluation is per link, so a block with three links fires only for the target whose state actually contradicts. The grounding check cannot catch this class — it verifies a claim is *cited*, not that it is *true*.
- **`criterion-path-existence` (advisory)** — flags a filesystem path named in a `done` spec's acceptance criterion that no longer resolves. An acceptance criterion is a contract, so naming a path asserts it is part of the delivered system, and nothing re-verified that after a later spec deleted the subject. Reads **inside** inline code spans, the inverse of the family above; that inversion is why the two are separate families rather than one check with a flag.
- **`skipped` on `CheckArtifactsResult`** — a list of `{family, reason, path}` naming each target a family could not examine, over the closed reason set `target-missing` / `target-unparseable` / `no-readable-state` / `root-absent` / `artifact-unreadable`. Both new families read targets that may be unreadable, and spec 045 forbids escalating an unreadable one into a finding; without this list a family that examined every target and found nothing would return exactly what a family that could examine nothing returns (`QUAL-CLAIM-001`). `clean` keeps its meaning — the assurance now lives in the pair.
- **`split_blocks`** — a shared block-level markdown splitter yielding each table row, list item, and paragraph with its starting line, plus `strip_inline_comments`. `SkipScanner` is deliberately unchanged: `read-tasks`, `mark-task`, `prune-tasks`, and the task-number walkers share it, so teaching it a fourth skip region would alter how each reads a quoted task line. `inline_code_spans` is now shared rather than private, and is the single span computation both new families use from opposite sides.

### Changed

- **`mark-task`'s done-when reconciliation is symmetric.** A checkbox-form `- [ ] Done when: …` clause now mirrors its task's tally in **both** directions — it ticks when a flip leaves every real subtask checked, and unticks when a flip leaves any unchecked. v0.25.0 shipped the ticking direction only, which left a ticked clause sitting above an unchecked subtask: the mirror image of the incoherence that change was made to remove. The clause is still outside the subtask index space, the bold and bulletless forms are still untouched, and an already-coherent block still produces no write.
- **`check-artifacts` reads its spec once, through `read-spec`.** It previously read and frontmatter-parsed `spec.md` itself and then called `read-spec` on the same file, leaving two independent notions of the spec's frontmatter in one function. The documented error contract is unchanged — `read-spec` raises the same variants on the same file.
- **The `## N.` task-heading grammar has one home.** `primitives::mod` now owns `split_numbered_heading` (borrowed, `Option<(&str, &str)>`) with `heading_is_numeric` defined as `.is_some()` on it, replacing three private copies across `read_tasks`, `prune_tasks`, and the phase scanner. The copies agreed, but nothing held them together, and all three read the *same tasks file* — a divergence would have let one primitive see a task another does not. `prune_tasks`'s task branch now parses once instead of testing a predicate and re-parsing. Landed as 022 scenario `numbered-heading-grammar-single-source`, closing the `QUAL-REUSE` SHOULD on spec 041.
- **A sibling link that traverses a symlink is reported, not followed.** `link-adjacent-drift` resolves lexically, which refuses a link *target* that climbs out (`../../../etc/passwd`) but cannot see a symlink committed *inside* the feature directory pointing outside it. `traverses_symlink` tests every component at or below the feature dir with `symlink_metadata`, which does not follow links — so the answer depends only on whether a component is a link, never on where it points, and AC8's repeat-run determinism is preserved without reintroducing `canonicalize`. The up-front readability pass short-circuits before the read, so a linked entry's destination is never opened; `read_target_state` returns the existing `target-unparseable` reason. Landed as 022 scenario `sibling-symlink-trust-boundary`, closing `BE-INPUT-004` on spec 045.

### Fixed

- **Config resolution no longer probes twice for one answer.** `discover-rule-files` and `dashboard` resolved the config path at load time and re-resolved its display name at render time — two existence probes at two moments, so a `/govern` migration landing between them could make a primitive read one file and attribute its contents to the other. `paths::resolve_config` now returns both from a single probe, and both primitives carry the resolved name forward with the parsed content. The serial-pipeline mitigation still holds, but it was an argument about who else is running rather than about the primitive being self-consistent. `DashboardConfig`'s serialized shape is unchanged. Landed as 022 scenario `config-resolution-single-probe`, closing `BE-RACE-001` carried across two reviews of spec 042.

### Framework

- `analyze.md` documents both checks, the unexamined-targets tier, and a shared promotion criterion: the house threshold (5+ findings on two consecutive `--all` runs) **and** a precision guard requiring every finding confirmed a true positive. The second half is required here and not for the LLM-judged checks because these are mechanical matches — a noisy implementation clears a volume threshold exactly as reliably as an accurate one.
- Constitution §drift-prevention gains a *Decision resolution* subsection: resolving a decision carries the same audit obligation as editing a document, and a resolution is not complete while a sibling artifact still describes the prior state.
- First full-repo run: 51 findings and 27 skips across 47 specs at 69% precision, including the originating case (026's `framework/workflows/registry.json`). Promotion verdict recorded as **do not promote**.

## [0.25.0] — 2026-08-01

Spec 022 tasks 70-71, from two scenarios grooming routed. An uncommitted spec directory stops being a crash and becomes a domain outcome, and `mark-task` reconciles the checkbox-form `Done when` clause it deliberately excludes from the subtask index.

### Changed

- **`derive-boundary` no longer errors on an uncommitted spec dir** — a spec directory with no commit touching it yields the spec-dir glob alone plus a new optional `guidance` string, instead of `no commits found that touch {root}/{feature}`. The boundary is *unknowable*, not broken, so the walk reaches a legible next-step (commit the directory, or seed a `write-boundary`) rather than dying at `/gov:implement` step 2. Enforcement stays fail-closed on that result. `PrimitiveError::NoSpecHistory` is removed — nothing raises it.
- **`diff-cross-spec` degrades the same way** — it raised the identical error at implement steps 7 and 12, so fixing only `derive-boundary` would have relocated the crash rather than removed it. It now returns an empty window carrying `guidance`, because bare empty lists are contractually *"no cross-spec impact"* and would otherwise assert a clean bill of health the primitive cannot vouch for.
- **Unborn HEAD is the same no-history state** — a repo with no commits at all (the fresh-repo case where `/gov:specify` and `/gov:plan` both run before the first commit) previously crashed. Probed with `Repository::is_empty`, since libgit2 reports a missing HEAD ref through more than one error code and matching on the code alone is brittle.
- **`mark-task` ticks an unchecked checkbox-form `Done when` clause** once the flip completes every real subtask, in the same atomic write. The clause remains outside the subtask index space (the read/mark contract is unchanged), but the block a reader sees no longer shows an unchecked box under a task the tally calls complete. Tick-only: unchecking a subtask does not untick the clause. The write guard now compares rebuilt content against the file as read, so an already-coherent block produces no write.
- **`read-tasks` reports `done-when-checked`** for the checkbox form only — the signal that lets a host distinguish "all subtasks checked" from "task block fully checked". `None` for the canonical bold and bulletless forms, which carry no checkbox and can never disagree with the tally.

### Framework

- `/gov:plan`'s validation gate gains "the spec directory is committed" as its first substantive readiness check, with both evaluation paths (`derive-boundary`'s `guidance`, or `git log -1 --` markdown-only). That is the gate that creates the state, so it is where the precondition belongs.
- `/gov:implement`'s steps 2, 6, 8, 12, the markdown-only walk, and the completion-gate tally document the new outcomes.

Both new schema fields are `Option` with `skip_serializing_if`, so they are absent from every ordinary result and no existing consumer or golden sees a new key.

## [0.24.0] — 2026-07-22

Spec 044: the shipped constitution relocates from the adopter repo root to `.govern/constitution.md`. The paired `constitution-relocate` migration (`introduced_in = 0.24.0`, no sunset) moves an existing adopter's root `constitution.md`, re-points the `CLAUDE.md` import, the `AGENTS.md` / `README.md` links, and any `[pinned] files` entry, and converges a stale root copy.

### Changed

- **No runtime code changes** — no primitive reads the constitution (loading it is host responsibility in the command bodies), and the parity fixtures' constitution paths are synthetic, so no fixture or golden changes ship. The version exists so the migration's `introduced_in` has a published release.

## [0.23.0] — 2026-07-22

Spec 043: the workflows feature is removed from the framework — `/govern` no longer offers or scaffolds tech-stack workflow slash commands, and nothing reads the `[workflows]` config section. The paired `workflows-sunset` migration (`introduced_in = 0.23.0`) cleans adopter projects: scaffolded workflow commands (current and legacy filenames), any legacy `skills/` directory, the synced `workflows/registry.json`, and the `[workflows]` config section, all pinned-aware. It subsumes the retired `skills-to-workflows` and `workflow-filename-rename` migrations, whose recipes moved to the root `CHANGELOG.md` § Archived migrations.

### Changed

- **Comment-only code changes** — the config-path resolver and `enforce-manifest` module docs no longer cite the removed `[workflows]` surface. No runtime behavior changes; the version exists so the migration's `introduced_in` has a published release.

## [0.22.0] — 2026-07-22

Spec 042: per-project govern files consolidate under `.govern/` — `.govern.toml` → `.govern/config.toml`, `.govern.session.toml` → `.govern/session.toml`, and the three adopter-facing generators → `.govern/scripts/`. The runtime resolves the new locations with an indefinite legacy-root fallback; the paired `/govern` migration (`govern-dir-consolidate`, `introduced_in = 0.22.0`) is the sole cutover, so an adopter who upgrades gvrn before re-running `/govern` keeps working off the legacy files.

### Changed

- **Config/session reads resolve `.govern/` first** — new `config_path` / `session_path` resolvers return the consolidated location when present and fall back to the legacy root files, new-wins on a split layout. All six config reads (`host.rs`, `paths.rs`, `resolve-references`, `dashboard`, `discover-rule-files`, the exec walker's config seed) and the three session readers route through them.
- **Writers target the active file** — `write-session` (via `session_path_for_write`) and the config-write rule land on the `.govern/` file when it exists and the legacy file when only it does; no runtime write outside the migration ever creates a partial `.govern/config.toml` that would strand the legacy file's other sections.
- **`SESSION_FILE` is `.govern/session.toml`** — `migrate-session-file` lands the pre-0.10.0 session-JSON migration directly at the consolidated path.
- **Provenance tags name the resolved config path (042 task 14)** — the disabled-rule-file notices from `discover-rule-files` and `dashboard`'s callout render the repo-relative resolved config file (`(.govern/config.toml)`, or the legacy root literal on a pre-migration layout) instead of a hardcoded `(.govern.toml)`; the `review.md` / `status.md` doc mirrors document the dynamic tag (doc↔runtime message parity preserved).
- **User-visible doc strings name the active paths (042 task 15)** — clap `--help` for the Command variants, the MCP tool descriptions (`write-session`, `migrate-session-file`, `dashboard`, `resolve-references`), and the schema doc comments describe config/session access via the active file; legacy filenames remain only where the fallback or a migration is being described.

### Fixed

- **Subprocess test harnesses always rebuild the release binary** — `ensure_binary_built()` in the exec/parity suites runs an unconditional incremental `cargo build --release` (under a `Once`, one build per test binary) instead of early-returning when a stale `target/release/gvrn` merely exists — a stale binary from a prior version or source state can no longer be the one under test.

## [0.21.1] — 2026-07-12

A read-side parser fix. `read-tasks` recognized only the `- **Done when**:` form its writer (`append-task`) emits — but `/gov:plan` authors the task breakdown directly (the runtime provides no primitive for it), so LLM- and hand-authored `tasks.md` files use other spellings the parser silently rejected, flagging every task as missing a done-when clause under `/gov:analyze` and the `/gov:review` done-gate. Captured as 022 follow-on scenario `done-when-authoring-forms`.

### Fixed

- **`read-tasks` / `check-artifacts` recognize every done-when authoring form** — a task's completion condition now parses from the checkbox-nested `- [x] Done when: …`, the bulletless `Done when: …`, and the colon-less `Done when <cond>` forms, alongside the canonical `- **Done when**: …` that `append-task` writes. Recognition routes through one shared `primitives::mod::parse_done_when` that `read-tasks` and `mark-task` both consume, so a checkbox-form clause is excluded from the subtask index space by **both** sides (the read/mark index contract) rather than counted as a subtask by `mark-task` while `read-tasks` treats it as the clause. The label must land on a word boundary — `Done whenever …` stays an ordinary subtask — and a task carrying no condition in any form is still flagged. Previously `check-artifacts` reported "task N has no Done when clause" for every task in any spec whose breakdown `/gov:plan` authored in a non-bold form: a 100%-false-positive block observed on an adopter project whose specs were authored end-to-end by `/gov:plan`.

### Changed

- **The `tasks.md` template documents the canonical done-when form** — the paired framework change (per the runtime/framework lockstep): `framework/templates/spec/tasks.md`'s example carries a `- **Done when**: …` line per task and states the requirement, and `/gov:plan`'s task-breakdown reference names the exact marker, so the LLM-authored breakdown converges on the form the reader and `append-task` prefer. Tolerant reader plus writer guidance — new specs write the canonical form, existing specs in any form still parse.

## [0.21.0] — 2026-07-12

The rmcp 1.x → 2.x port the 0.19.0 notes deferred, plus the follow-on scenarios it unblocked. The migration landed **source-compatible** — rmcp's 2.x breaking changes are in the MCP model types (2025-11-25 spec alignment), not the `#[tool]` / `ToolRouter` / `Parameters` surface the runtime builds on — so the bump is a dependency change, and MCP unknown-field strictness (022 task 65) ships alongside it as promised. Four more 022 follow-on scenarios (tasks 66–69) close latent network-hardening and parser gaps surfaced during the 0.20.0 alignment review and this port. Spec 022 (`deterministic-runtime`) advances to `done` with this release.

### Changed

- **rmcp bumped `1` → `2` (resolves 2.2.0)** — the MCP SDK's 1.x→2.x breaking changes are confined to the model types (MCP 2025-11-25 alignment); `Parameters<Args>`, `ToolRouter` (`pub map`), `ToolRoute` (`pub attr`), and `model::Tool` (`input_schema` / `output_schema`) are all preserved, so the runtime compiles against 2.x with no call-site changes and the full suite (incl. `tests/mcp.rs`) stays green. Certificate validation is unchanged (rustls default). Ships as its own minor per the versioning note in 0.19.0.

### Follow-on scenarios (022 tasks 65–69)

- **MCP unknown-field strictness (task 65)** — an MCP tool call carrying an argument key outside the target primitive's known set is rejected with an `invalid_params` error naming the field, instead of silently dropping the misspelling and running with that field's default (which would flip `fix` / `apply` / `force` / `recursive` / the `sha256-url` verification-skip). Enforced by a `reject_unknown_fields` pass in `GovRuntimeServer::new` (a sibling of `strip_nonstandard_formats`) that derives each tool's allowlist from its input schema's `properties` — the schema `schemars` already produces, so the list can never drift from the `Args` type — and wraps the route's call to screen incoming keys before the macro's lenient deserialize. No `Args` struct is touched and none uses `#[serde(flatten)]`; the subprocess interpreter never builds this router, so its deliberate superset-context binding stays lenient. Resolves the scenario's derive-vs-wrapper open question in favor of the schema-derived allowlist — no custom `Parameters` wrapper was needed once the port proved source-compatible.
- **fetch-archive DNS-rebinding hardening (task 66)** — `fetch-archive` now pins its connection to the exact address `validate_fetch_url` screened (via reqwest's `resolve_to_addrs`), so reqwest connects to a screened address rather than re-resolving the host independently at connect time. Closes the resolve-then-connect TOCTOU where a host that resolved to a public address during validation and an internal one at connect (DNS rebinding) could slip past the SSRF screen. An operator-allowlisted insecure host still resolves normally; every pinned address has passed the internal-range screen. Hardens BE-INPUT-007 beyond its accepted resolution-time block.
- **append-inbox comment-aware write (task 67)** — `append-inbox`'s write side is now comment/fence-aware like its read side. A bullet appended to an `inbox.md` that ends inside an unterminated HTML comment (or code fence) lands *before* the region — a position `count_inbox_bullets` counts — rather than inside it, where the reader would skip it and the count would undercount. Well-formed inboxes append unchanged. Closes the asymmetry between the comment-blind `append_bullet` and the comment-aware `iter_bullets` / `count_inbox_bullets`.
- **write-review known-field quoting (task 68)** — `write-review` renders every value in the `review:` block — the known waiver fields (`rule` / `file` / `reason` / `waived-at` / `waived-by`) as well as the open-schema extras — through `yaml_string`, so a known field whose value would re-parse as a non-string (a bare `1234` / `true` / `null`) is quoted and round-trips through `RawWaiver` instead of breaking the next parse. Timestamps and IDs stay unquoted (no golden churn). Closes the gap where the known-field path kept `yaml_scalar` after the extras path was hardened in 0.20.0.
- **SkipScanner inline-code exemption (task 69)** — `SkipScanner` now treats HTML-comment and code-fence delimiters inside a backtick inline-code span as inert, so prose that merely mentions a backticked comment-open delimiter no longer opens a phantom comment region that hides all following structure from the comment-aware parsers (`read-tasks` / `mark-task`, `dashboard` open-question counts, the section walkers). Delimiters in ordinary text skip exactly as before; the inline-code span is matched per the CommonMark equal-length-backtick-run rule, and the scan is short-circuited on lines with no delimiter. Fixes the class where a task's `done-when` mentioning a backticked delimiter hid the following task from `read-tasks` / `mark-task`.

## [0.20.0] — 2026-07-12

Command-runtime alignment pass: an audit of every `framework/commands/*.md` against the primitives it invokes surfaced a set of divergences where the deterministic runtime and the command prose (and its markdown-only fallback) disagreed. These fix the runtime side so the two paths share one contract again; the prose side is corrected in the same change.

### Fixed

- **`append-question` no longer back-edges a `done` spec** — `BACK_EDGE_STATUSES` dropped `done`, so a question appended to a `done` spec is recorded without silently reverting its status to `draft`. Per constitution §spec-lifecycle and spec 014 a `done` spec reopens via the scenario route (`done → in-progress`), never via a question; the primitive now matches the command, the markdown-only path, and the constitution, closing a two-paths divergence reachable through amend's `flip` override.
- **`process-waivers` retains waivers on a dimension-restricted run** — a new `skipped-passes` argument marks the run as partial (`--security` / `--simplicity` / `--quality`). When set, a waiver whose rule did not fire is **retained** (a new result bucket) rather than expired, so `write-review` never prunes a waiver anchored to a dimension the run didn't evaluate. Previously a `--security`-only run classified every non-security waiver as expired and deleted it from the spec frontmatter — silent data loss the command explicitly promised not to cause. Only an unrestricted run expires waivers.
- **`append-inbox` / `remove-inbox-item` are comment-aware** — inbox bullet scanning (dedup, removal, counting) now skips fenced-code and HTML-comment regions via the shared `SkipScanner`, so the `-` lines inside the inbox template's `<!-- Rules: … -->` guidance are no longer miscounted as items or matchable for removal.
- **`write-review` preserves adopter-authored waiver fields** — unknown keys on a `review.waivers` entry (`ticket`, `co-waived-by`, org policy fields) are captured and re-emitted verbatim on the frontmatter rewrite, honoring the §text-first-artifacts open-schema rule across a re-render instead of silently dropping them on the next review run.

### Changed

- **Inbox entries are written as checkboxes** — `append-inbox` now writes `- [ ] {text}` rather than a plain `- {text}` bullet, matching the inbox template's documented forms and the constitution's §bug-handling ("tracked as a checkbox … resolved by being done, then removed"). Dedup and removal strip the checkbox marker, so both forms still match by content.
- **`append-inbox` reports `item-count`** — the result gains the total inbox bullet count (comment/fence-aware) after the call, so `/gov:log` reports it without hand-counting, mirroring `remove-inbox-item`'s `remaining-count`.
- **`resolve-anchor` accepts an optional `markers-path`** — references in one file can now be resolved against a *different* file's `<!-- §anchor -->` markers (e.g. a spec's `§` references against the constitution). Omitted, it keeps the same-file self-consistency behavior. `/gov:analyze` uses it to verify spec references resolve against the constitution instead of firing every reference as unresolved noise.

### Documentation

- **`create-plan-artifacts` MCP description** dropped its claim that a kept artifact is reported "with its last-modified timestamp" — the result carries no wall-clock data; the keep-or-replace prompt stats the file itself.
- Stale in-code references corrected: `check-artifacts`'s family notes no longer cite drifting `analyze.md` line numbers, and `discover-rule-files` drops the retired "review Behavior step 5" phrasing.

## [0.19.0] — 2026-07-12

Follow-up review of the 0.18.0 runtime (see `specs/022-deterministic-runtime/review.md`). Closes a partially-resolved SSRF finding, a bootstrap parse regression the 0.18.0 parser change introduced, and a set of newly-surfaced input-validation and correctness gaps — plus the full follow-on scenario set the review captured (tasks 64–68 below): six new coverage primitives, the dashboard's rendered pipeline view, the derived writeCode boundary, two retired primitives, and the documented clarify exec scope. One review item is deliberately excluded: MCP unknown-field strictness (022 task 65) rides the rmcp 1.x → 2.x port tracked in `specs/inbox.md` and ships as the next minor, because its strict-params wrapper builds on exactly the rmcp layer that migration reworks.

### Follow-on scenarios (022 tasks 64–68)

- **Parser nested-list continuation (task 64)** — a recognized primitive named in continuation text after a nested ordered list closes (where the parent step was already finalized) now raises `ParseError::Invalid` naming the primitive, instead of silently dropping the dispatch on the exec path. Guarded on being inside the step list, so a primitive named in the Instructions preamble stays a legitimate reference. `scripts/lint-procedure-parseability.sh` catches the class across `framework/commands/*.md` + `framework/bootstrap/*.md`.
- **`remove-inbox-item` primitive (task 67, in progress)** — the complement of `append-inbox`: removes the first `{specs-root}/inbox.md` bullet whose text matches `item` (shared bullet grammar, atomic write, double-blank seam collapsed), reporting the remaining count. A no-match or missing inbox is a clean `removed: false` outcome. `/gov:groom` step 8 now invokes it instead of a host `Edit`, closing the review's highest-value coverage gap (groom's per-item hot loop).
- **`create-plan-artifacts` primitive (task 67, in progress)** — the plan-side mirror of `create-feature`: copies the plan/tasks (and, with `include-data-model`, data-model) templates into an existing feature directory, atomic and mode-preserving, resolving every needed template before the first write. Pre-existing artifacts are never touched by default — each reports `kept`, feeding `/gov:plan`'s keep-or-replace prompt; only `overwrite: true` (the confirmed replace branch) copies over them. `/gov:plan` gains step 3 invoking it, closing the review's template-copy / existing-artifact-detection gap. The template resolver is now a shared `primitives::resolve_template` helper (lifted from `create-feature`); the `plan-basic` parity fixture gains templates and a re-blessed golden (step 3 changes the stream).
- **`check-review-gate` primitive (task 67, in progress)** — evaluates `/gov:implement`'s pre-done review gate in one call, first failure wins: the feature directory's markdown lint (recursive glob through the `lint-markdown` machinery, absorbing the step's raw `npx markdownlint-cli2` invocation), then the spec `review:` block (`not-reviewed` on an absent block or null `last-run`; `must-violations` on `blocking: true`). Returns the verdict plus the canonical blocked message — `/{project}:review` references resolved through the `[host] project` namespace — and the resolve-or-waive guidance. Implement step 13 invokes it, replacing the branch the host re-walked by hand on every completion attempt; a blocked gate is a domain outcome, never an error.
- **`append-question` primitive (task 67, in progress)** — `/gov:amend`'s question-route write, previously primitive-less (asymmetric with the scenario route's `create-scenario` + `append-task`): appends `- {question}` to the target's `## Open Questions` (the spec, or `scenarios/{slug}.md` via `scenario`), creating a missing section per template order and replacing `*None …*` scaffold placeholders. Dedup reuses `read-spec`'s question parser with amend's normalized-whitespace comparison (a match is a clean `appended: false` outcome reporting `duplicate-of`); on a non-`draft` spec target the status reverts to `draft` in the same atomic write — the back-edge that keeps "questions resolved" status claims honest. Amend's question route now invokes it.
- **`diff-cross-spec` primitive (task 67, in progress)** — the cross-spec impact filter `/gov:implement` steps 7 and 12 re-derived by hand per task (step 12 self-declared "no primitive owns this filter yet"): diffs the feature's first spec-dir commit (shared `derive-boundary` walk) against the working tree, scoped to the spec root — sibling-spec paths outside the feature's own dir, plus the inbox's added bullet lines (shared bullet grammar) as the captured issues. Working-tree diffing means the per-task summary sees the run's uncommitted captures; a clean tree equals the documented `first-commit..HEAD -- specs/` form. Implement steps 7/12 now invoke it; review's captured-issues stays on `compute-review-scope` (in-progress window). Resolves the scenario's own-primitive-vs-derive-boundary-mode fork: separate primitive, shared walk helper.
- **`dashboard` rendered-markdown field (task 67, completes the scenario)** — the dashboard result gains `rendered-markdown`: the full `/gov:status` pipeline view pre-rendered as one markdown fragment (preamble, table, counts/callouts, cross-service references readout), absorbing the five LLM-side rendering steps the coverage review counted. The runtime resolves each spec's `references:` index internally for the readout (same classification as `resolve-references`, service `description` appended) and substitutes the `[host] project` namespace into `/{project}:…` texts. Returned data the host may restyle, never stdout printing; the structured payload stays authoritative. `/gov:status` collapses to invoke-dashboard → emit-rendered → target prompt, with the piece-by-piece formats moved to its Rendering reference as the markdown-only path.
- **`substitute-templates` and `merge-claude-md` retired (task 68)** — both were exposed, tested, and permission-listed with no command step invoking either: `substitute-templates`' tree-copy was subsumed by `apply-manifest` (which now owns the shared `apply_substitutions` helper), and `merge-claude-md` was already a compat shim over `merge-managed-block` slated for removal. Six-site removal in reverse: registry (hence `TOOL_NAMES` / `PRIMITIVE_NAMES`), interpreter dispatch, CLI subcommands, MCP `#[tool]` methods, schema Args/Results, `runtime-tools.txt`, and the regenerated configure permission blocks; the exec bootstrap-chain test re-targets `extract-archive` → `apply-manifest` → `merge-managed-block`. Removal lands inside the unreleased 0.19.0, so no released MCP surface breaks.
- **Clarify exec-path scope documented (task 68, completes the scenario)** — `gvrn exec clarify` no-ops steps 7–8 (edge-case enumeration, criterion verification) by design: they cannot fold into `askClarifyQuestion`'s one-question-per-round-trip ABI because they are spec-wide passes that run even on the zero-questions short-circuit. The reduction is now stated in the command's Instructions preamble and the data-model's exec-path note instead of standing as a silent gap; the markdown-only path keeps performing both steps in full.
- **writeCode boundary derivation (task 66)** — `derive-boundary` now emits **directory-zone globs** (`runtime/src/main.rs` → `runtime/src/**`; root-level files stay exact — their zone would be `**`), because the writeCode validator must admit *new* files, which never exact-match a previously-changed path. The walker merges the result into the `write-boundary` enforcement key as a **union** with any session seed (a deliberate grant is never revoked; on a fresh feature the seed admits the first out-of-spec edit; with neither, enforcement stays fail-closed). Previously nothing populated the key — enforcement silently depended on a pre-seeded boundary. The `implement-basic` fixture gains a two-commit fixed-time history and drops its seeded `write-boundary`, so the re-blessed golden's enforcement runs on the derivation alone. Resolves the scenario's boundary-format fork against changing `path_in_boundary` semantics (an exact-path entry that silently grants its directory would make boundary entries lie).

### Security

- **Redirect SSRF (BE-INPUT-007 completion)** — `fetch-archive` now re-runs the full scheme/internal-range screen on **every** redirect hop via a custom `reqwest` redirect policy (capped at 10 hops). Previously the default client followed up to 10 redirects with no re-validation, so a single `302` to `http://169.254.169.254/…` defeated both the https-only rule and the internal-range denial.
- **Path traversal via `feature` (BE-INPUT-001 sibling)** — `set-status`, `mark-task`, `mark-criterion`, `prune-tasks`, `write-review`, and the read-only feature primitives (`read-spec`, `read-tasks`, `check-artifacts`, `check-stuck`, `compute-review-scope`, `traverse-deps`, `derive-boundary`, `process-waivers`, `resolve-references`) now validate the MCP-supplied `feature` argument with `validate_no_traversal`, closing an out-of-repo write/read escape (`feature: "../../other/specs/001-x"`).
- **`write-review` frontmatter injection** — `reviewed-at`, `reviewed-against`, `diff-base`, `feature`, `scenario`, and `skipped-passes` are single-line-validated before being spliced into `review.md` and the spec's `review:` frontmatter block, so a newline can no longer inject a spoofed top-level key (e.g. `status: done`) into the spec.
- **`run-generator` / `lint-markdown` containment** — `run-generator` bounds its `script` argument to the repo (`validate_no_traversal`), and `lint-markdown` rejects a `paths` entry beginning with `-` (which markdownlint-cli2 would parse as a flag such as `--config`, loading arbitrary `customRules` JS).

### Fixed

- **`gvrn exec govern` parse regression** — `framework/bootstrap/govern.md` step 6 (which named two primitives, `merge-managed-block` + `write-session`) is split into two steps, so the bootstrap procedure parses again under the 0.18.0 two-primitive hard-error. `scripts/lint-procedure-parseability.sh` now also parses `framework/bootstrap/*.md`, so this class can no longer slip through CI.
- **`gvrn exec target <feature>` retarget** — a `resolve-feature` `resolved` result now overrides the session-seeded `feature`/`path` on the `target` command, so switching the target actually writes the new feature instead of rewriting the stale one with a fresh timestamp.
- **Numbered-heading grammar** — `mark-task` and `read-tasks` now require the trailing `.` on a task heading (`## N.`), matching `append-task`/`prune-tasks`, so a prose heading like `## 3 quick wins` is no longer treated as task 3.
- **`writeCode` edit contract** — `validate_response` now rejects a `create` edit with no `content` and an `edit` edit with neither `patch` nor `content` (`ValidationError::EditContent`), instead of admitting an undefined edit the schema alone allowed.
- **`discover-rule-files` surfaces** — the MCP-boundary `detected-surfaces` argument is validated like a `[rules] surfaces` config value; an unrecognized member (e.g. `"Backend"`) fails fast instead of silently loading only `-cross.md` rules.
- **Exec operational-error envelopes** — the command-not-found, unreadable-file, and walker-I/O exit paths now emit a terminal `error` protocol message on stdout (with the runtime version), honoring the 1–127 clean-band contract that lets a host distinguish a clean operational error from a signal-killed crash.
- **Parser robustness** — a heading that emits no text (empty, or a code-span only like `` ## `gvrn` ``) no longer opens the Instructions section over the real one; an extension marker on its own line between steps is attached to the next step rather than dropped; and a marker merely quoted in a code span is no longer mistaken for a live seam.
- **CRLF idempotency** — `merge-managed-block`'s `html-comment` style normalizes `\r` before its unchanged-compare, so `merge-claude-md` no longer rewrites `CLAUDE.md` on every run on a CRLF checkout.
- **`merge-managed-block` marker** — the `marker` argument is rejected when it contains a newline or `-->`, which would corrupt the managed-region delimiters.

### Changed

- **`write_atomic` mode preservation (Unix)** — an in-place rewrite now re-applies the destination file's prior permissions after the tempfile rename, so `set-status` / `mark-task` / `write-review` / etc. no longer narrow an existing `0644` file to `0600`.
- Four no-op `matches!(…)` statement tests are wrapped in `assert!(…)`; stale doc comments (`dashboard` `session-path`, `prune-tasks` `size-after`, the `ParseDiagnostics` data-model note) are corrected.

### Not fixed (documented)

- The `fetch-archive` DNS-rebinding TOCTOU (guard resolves, reqwest re-resolves) remains as logged in `specs/inbox.md`; the redirect fix above is orthogonal to it.
- On the `/gov:implement` exec path, `derive-boundary`'s computed boundary does not auto-populate the `write-boundary` key the writeCode validator enforces on (that key is a seeded input); enforcement is fail-closed without a seed. Auto-binding it is a design decision (seed-vs-derived precedence) that also needs a multi-commit `implement-basic` fixture, so it is deferred rather than changed here.
- `deny_unknown_fields` was not added to the primitive `Args` structs: the exec interpreter binds every primitive's args from a clone of the *entire* walker context (a deliberate superset), so rejecting unknown fields would break all primitives on the exec path. A misspelled kebab-case field still silently defaults; closing this needs a per-primitive field allowlist, not a blanket attribute.

## [0.18.0] — 2026-07-11

Remediation of the nine MUST and twenty-one SHOULD findings from the 0.17.0 `/gov:review` (see `specs/022-deterministic-runtime/review.md`).

### Security

- **writeCode boundary (BE-INPUT-004)** — the edit-path segment screen now splits on both `/` and `\`, closing a Windows-only traversal escape (`runtime/a\..\..\x` no longer satisfies `runtime/**`).
- **Archive extraction (BE-INPUT-006)** — `extract-archive` now bounds cumulative decompressed bytes (`MAX_EXTRACT_BYTES` = 2 GiB) and entry count (`MAX_EXTRACT_ENTRIES` = 100000), erroring with the cap named before writing past it, closing a decompression-bomb DoS through the unverified bootstrap path.
- **Outbound fetch (BE-INPUT-007)** — `fetch-archive` enforces an https-only scheme allowlist and denies hosts resolving to loopback / link-local / RFC-1918 / unique-local / cloud-metadata ranges (SSRF). An opt-in `GVRN_FETCH_ALLOW_INSECURE_HOSTS` allowlist (empty by default) exempts named hosts for internal mirrors and local testing.
- **Slug validation (BE-INPUT-001/002)** — `validate_slug` is now an allowlist (`^[a-z0-9]+(?:-[a-z0-9]+)*$`), rejecting newlines and control characters that previously reached written filenames and headings; `append-task` validates its `slug` before interpolating it into `tasks.md`.

### Fixed

- **Command exec-path correctness** — `/gov:clarify` step 2 no longer collapses `read-spec` and the recovery-branch `set-status` into one step (the parser bound only the last), `/gov:clarify`'s draft→clarified flip is gated by a dedicated `gate-confirm` step, and `/gov:groom`'s scenario route no longer drops `create-scenario`. Root cause fixed structurally: the parser now raises `ParseError::Invalid` when one numbered step names two distinct primitives, so this class fails CI instead of silently dropping a dispatch.
- **`gvrn exec specify`** binds the newly-created feature into the session write (a `create-feature` `created: true` result overrides the stale session-seeded `feature`/`path`), instead of rewriting the session to the old target.
- **`verifyCriteria`** verdicts now gate `mark-criterion` on the exec path — a criterion the response does not affirm `met: true` is left unchecked.
- **`read-spec`** folds wrapped acceptance-criterion continuation lines into the criterion text (no more mid-sentence truncation reaching `verifyCriteria`), preserving the checkbox-index contract.

### Performance

- `check-stuck` memoizes status parses by the spec's blob OID (one parse per distinct spec version, not per commit); the git-walk MCP tools (`check-stuck`, `compute-review-scope`, `derive-boundary`) run under `spawn_blocking`; `is-gitignored` opens the repo once per plan-file loop; `first-rule-with-verification` is a single pass.

### Internal

- Consolidation (behavior-preserving): shared `list_feature_dirs`, `feature_number`, `read_scenario_section`, `list_scenario_files`, `template_candidates`, and `frontmatter_status` helpers replace ~nine hand-rolled copies across the new primitives and payload builders; `UNBLOCKING_STATUSES`/`PLANNED_OR_LATER` derive from `schema::status`; `append-inbox` and `check-artifacts` reuse the shared checkbox grammar and case-insensitive scenario walk (closing two latent divergences); the seven extension-request builders share `typed_with_legacy_context`/`typed_only`. Command-prose restatements now point at the markdown-only reference rather than duplicating it.

### Notes

- `cargo test` (760), `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `scripts/audit/run-all.sh`, and the ubuntu/macos/windows CI matrix are clean. Ships in lockstep with the framework per [§runtime-boundary](../framework/constitution.md#runtime-boundary).

## [0.17.0] — 2026-07-11

The backlog burn-down release: every SHOULD-tier and coverage finding from the 2026-07-11 full-runtime review, implemented as nine scenarios on spec 022 (`primitive-robustness-hardening`, `archive-network-hardening`, `parser-walker-conventions`, `extension-request-hygiene`, `implement-completion-gate`, `clarify-command-acceleration`, `groom-command-acceleration`, `scaffolding-primitives`, `analyze-artifact-checks`) plus the CI/README/consolidation chores. The minor bump is driven by the four new primitives and the parser/walker convention changes below.

### Added

- **`resolve-feature`** — resolves a feature by exact directory name, bare or zero-padded number, or unique partial slug under the configured specs root; reports ambiguity/not-found as domain outcomes and optionally a scenario file's existence + `section`. Adopted by `/gov:target`.
- **`create-feature`** — next-NNN computation, kebab slug derivation, directory creation, and mode-mirrored template copy (installed template wins over the framework source); refuses existing directories. Adopted by `/gov:specify`.
- **`append-inbox`** — atomic bullet append to `specs/inbox.md` with create-from-template and optional dedup-by-prefix. Adopted by `/gov:log` and named by `/gov:implement`'s auto-capture rule.
- **`check-artifacts`** — the residual deterministic `/gov:analyze` families: artifact completeness per status, task numbering/done-when consistency, scenario→task mapping (prune-aware per constitution §tasks-phase), and review-state drift, with severities mirroring the command reference. Adopted by `/gov:analyze` step 8.
- **`write-session` clear mode** — removes the session target while preserving the per-contributor `cli-config-dir`; `/gov:target --clear` and `/gov:specify` now route through the primitive (the stale "no session-shaped primitive" claim is gone).
- **`verifyCriteria` extension point** — typed request/response for `/gov:implement`'s per-criterion verification; the completion gate is now numbered parseable steps (`read-tasks` tally, `read-spec`, per-criterion `mark-criterion`, review-gate reads, `gate-confirm`, `set-status`), giving `mark-criterion` its first prose consumer.
- **Typed `askClarifyQuestion` / `routeInboxItem` builders** — the two long-deferred extension points now have schema types, data-model entries, and typed request construction; unknown extension identifiers are an error, never a context dump.
- `/gov:clarify`, `/gov:groom`, and `/gov:log` are rewritten to the parseable conventions and leave `legacy-prose-commands.txt` (now just `amend.md` and `help.md`); `/gov:plan` and `/gov:specify` approval gates use `gate-confirm`; raw `gen-spec-deps.sh` references route through `run-generator`.
- The tool list grows from 33 to **37**; claude/auggie configure permission blocks regenerated.

### Changed

- **Parser step numbering** honors the document: ordered-list `start` seeds the counter, lists separated only by HTML comments continue the sequence, and nested bullets no longer become phantom steps — `progress.step` and gate names sent to hosts now match the literal step numbers in every command file.
- **Gate semantics**: a step invoking `gate-confirm` blocks by virtue of the primitive, phrase or no phrase; the "ask the user to approve" prose trigger applies only to prose steps; a primitive step never silently drops its dispatch.
- **Span heuristic**: only backticked spans in invoking position (or within edit distance 2 of a primitive name) fail parseability — ordinary kebab-case vocabulary parses; `prune.md` and `govern.md` now parse cleanly. `gvrn parse --check` distinguishes legacy-prose (rc 2) from invalid (rc 1), and the parseability lint rejects invalid even for allowlisted files.
- **Extension-request hygiene**: walker accumulator keys (`llm:*`, `findings`) are filtered from every request payload; `writeSpecBody` populates `template-path`/`template-content`/`section`/`feature-description` with command-aware existing-section reads.
- **MCP seam**: `fetch-archive`, `extract-archive`, `run-generator`, and `lint-markdown` run under `spawn_blocking` (no more blocking-client panic in debug builds or pinned tokio workers).
- Path validation is uniform: `enforce-manifest` requires the target directory inside the repo, `apply-manifest` validates every manifest entry, `merge-managed-block`/`merge-permissions` require repo-relative paths. On Windows, `validate_no_traversal` also rejects rooted (`\foo`), drive-relative (`C:foo`), and UNC-prefixed paths that `is_absolute()` alone missed — surfaced by the first windows-latest CI run.
- `check-stuck` tolerates branchy history (parent-blob transition detection, reachability-based counting, CRLF-aware frontmatter); `write-review` computes both outputs before writing either; `substitute-templates` mirrors source file modes; `create-scenario` YAML-escapes `section`; `append-task` rejects embedded newlines; `dashboard` degrades to a detail-less target on malformed scenario frontmatter; `check-rule-ids` scopes deprecation to the rule's own section; `set-status` validates lifecycle membership; `fetch-archive` errors on over-cap bodies instead of truncating; `extract-archive` skips zip symlinks and masks modes to `0o777`.

### Internal

- Consolidation (behavior-preserving, goldens byte-exact): one shared `resolve_path` (nine private copies deleted), one checkbox grammar for the read and mark sides of the criterion/task index contract, a canonical lifecycle-status constant in `schema/status.rs` with the compatibility subset derived from it, one shared frontmatter-status reader, and a single `PRIMITIVE_REGISTRY` from which the parser's and MCP server's name lists are both defined — with tests pinning the interpreter dispatch and `runtime-tools.txt` to set-equality against it.

### Notes

- CI now tests on ubuntu/macos/windows with `--locked` everywhere and a pinned toolchain; a repo-wide `.gitattributes` pins LF. Ships in lockstep with the framework per [§runtime-boundary](../framework/constitution.md#runtime-boundary). `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `scripts/audit/run-all.sh` are clean.

## [0.16.1] — 2026-07-11

Bug-fix release for the eleven MUST-level defects surfaced by the 2026-07-11 full-runtime accuracy review, recorded as six scenarios on spec 022 (`spec-side-parser-hardening`, `host-protocol-conformance`, `write-boundary-path-normalization`, `merge-managed-block-trailing-append`, `resolve-references-cli-exec-wiring`, `waiver-processing-order`).

### Fixed

- **`merge-managed-block`** — the group-alignment walk consumed the first adopter section *after* the managed block as a "rewrite" whenever the new canonical block appended trailing subsection(s), deleting adopter content on every `/govern` update. Unmatched trailing canonical groups are now pure insertions; adopter content beyond the block is preserved verbatim (including when an adopter heading collides with an appended canonical heading).
- **`validate-frontmatter`** — a spec missing `status` or `dependencies` entirely returned `clean: true`; both now produce blocking findings per the constitution's hard-fail tier. A present-but-empty frontmatter block (`---`/`---`) is treated as an empty mapping (both missing-field findings) rather than a `MissingFrontmatter` operational error — a small widening that applies to every frontmatter consumer.
- **`read-spec` / `mark-criterion`** — the acceptance-criteria walkers had no HTML-comment/fence awareness, so the spec template's comment-embedded example checkbox counted as a phantom criterion (and was flippable). Both now share a single `SkipScanner`-aware section walker, keeping their index contract aligned by construction; template-state specs report zero criteria.
- **`set-status`** — the frontmatter splice offset hardcoded the 4-byte `---` + LF opener; on CRLF spec files every transition wrote one byte early and corrupted the frontmatter. The offset now comes from the opener actually matched.
- **`check-rule-ids`** — the deprecation scan sliced the rule file at a raw byte offset and panicked when the window edge landed mid-UTF-8 character (em-dash-dense rule files made this reachable); the slice now backs up to a char boundary.
- **`writeCode` boundary validation** — edit paths were prefix-matched without normalization, so `runtime/../framework/x` satisfied `runtime/**`; absolute paths and `.`/`..`/empty segments are now rejected before pattern matching.
- **`assessSpecQuality`** — the extension point sent a raw walker-context dump instead of its documented request; it now builds the typed `spec-path` / `spec-content` / `rule{id, verification, severity}` shape (analyze golden re-blessed).
- **`gvrn exec` parse failures** — exited 2 with no terminal message, violating the exit-code contract; they now emit a terminal `error` envelope carrying the runtime version and a version-mismatch note.
- **stdio robustness** — a stray envelope, mismatched request-id, malformed line, or blank keepalive on stdin halted the walk (or surfaced a raw I/O error); per the data model these are now logged to stderr and ignored while the runtime keeps waiting. Stdin EOF remains an operational error.
- **`resolve-references`** — was reachable only over MCP; now wired as a CLI subcommand, an interpreter dispatch arm, and a `PRIMITIVE_NAMES` entry, with a registry-equality test so the name lists cannot silently diverge again.
- **`/gov:review` waiver ordering** — the procedure classified waivers *before* the passes produced findings, mass-expiring every valid waiver on the exec path; `process-waivers` now runs after the five passes (binding the accumulated findings to `fired`) and immediately before `write-review` (review golden re-blessed).

### Notes

- Ships in lockstep with the framework per [§runtime-boundary](../framework/constitution.md#runtime-boundary). `cargo test`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `scripts/audit/run-all.sh` are clean.

## [0.16.0] — 2026-07-10

Adds the `prune-tasks` primitive backing the new `/gov:prune` command (spec 041).

### Added

- **`prune-tasks`** — reduces a feature's `tasks.md`. The default keep-pending prune drops every *spent* (≥ 1 checkbox, all checked) task section and any phase container left with no surviving task section, preserving the preamble and every pending / no-checkbox section verbatim; `--reset` rewrites the file to the template's initial state (the existing `# …` heading plus the canonical empty-tasks body). Reset is gated on spec status — permitted only when the spec is `done`, unless `--force` is supplied. A two-mode `apply` flag keeps the file body out of the caller's context: `apply: false` returns a compact summary (per-section classification, removed/kept counts, size before/after) and writes nothing; `apply: true` writes atomically. New MCP tool `prune-tasks`; new CLI subcommand `gvrn prune-tasks`; the tool list grows from 32 to 33.

### Notes

- Ships in lockstep with the framework per [§runtime-boundary](../framework/constitution.md#runtime-boundary). `cargo test`, `cargo fmt --check`, and `cargo clippy --all-targets -- -D warnings` are clean.

## [0.15.0] — 2026-07-06

The **review-runtime-acceleration** series (spec 022 scenario `review-runtime-acceleration`): the deterministic bookkeeping around `/gov:review`'s five semantic passes moves into primitives, and the passes themselves become a new `performReview` extension point. The bump to `0.15.0` is driven by the breaking `create-scenario` arg-shape change below.

### Added

- **`discover-rule-files`** — owns `/gov:review` rule-file selection: rule-dir listing, suffix classification (`-backend` / `-frontend` / `-cross` / unrecognized), `[rules] surfaces` selection (valid list / `[]` cross-only / unset derive-from-stack / degenerate fail-fast), and the `[[review.disabled-rule-files]]` filter — returning the selected set plus the ordered stdout notices verbatim.
- **`process-waivers`** — per-run classification of a spec's `review.waivers` against the currently-firing `(rule, file)` findings: apply / expire (with the `waiver expired: …` notice) / do-not-extend / malformed (skip-and-warn, never prune) / duplicate (first applies, dup warns). The anchor is the `(rule, file)` pair, so code moving within a file does not expire a waiver.
- **`compute-review-scope`** — resolves the `diff-base` (the status-to-`in-progress` commit, or a `--since` override), the file scope (plan `Affected Files` unioned with files modified since `diff-base`, larger set wins), and the inbox additions in the window, using `git2`.
- **`write-review`** — renders `specs/NNN/review.md` (frontmatter + fixed skeleton) and updates the spec `review:` frontmatter block. Applies the deterministic cross-pass dedup (highest-severity-wins on rule-id + file + overlapping range) before counting, buckets survivors into MUST / SHOULD / low-confidence / waived (applied waivers drop out of the counts), prunes expired waivers from `review.waivers`, and emits the 0-findings / `blocking: false` report for empty scope. `blocking` is true exactly when `must-violations` exceeds zero; both writes are atomic.
- **`performReview` extension point** — the LLM seam for each review pass. `PerformReviewRequest` carries the in-scope files (the cache-stable prefix) plus the rules loaded for the pass; `PerformReviewResponse` returns findings in the shape `write-review` consumes. The interpreter emits one `llm-request` per pass step and accumulates each pass's findings into the shared `findings` context key so a later `write-review` step consumes the union across all passes.

### Changed

- **BREAKING — `create-scenario` takes a single `body` argument** instead of separate `context` / `behavior` / `edge-cases` params. LLM-authored content now crosses the runtime boundary as one payload (the content-ingestion convention): the host assembles the `## Context` … `## Edge Cases` markdown in-context and hands it over whole, and the primitive keeps framing it with the `section:` frontmatter, the H1-from-slug, the atomic write, the slug-conflict refusal, and the auto-appended Open / Resolved Questions scaffolding. This removes the host MCP-encoder failure mode where one of several large sibling string params is silently dropped. `framework/commands/amend.md`'s scenario-branch prose is updated to match.
- **The procedure parser recognizes the four review primitives.** `discover-rule-files`, `process-waivers`, `compute-review-scope`, and `write-review` are added to `parser::PRIMITIVE_NAMES`, so the rewritten `/gov:review` Instructions resolve their backticked `Invoke` steps instead of raising `ParseError::Invalid`.

### Notes

- **Framework, shipping in this tagged archive (no runtime-crate behavior change):** `framework/commands/review.md` is rewritten from 604 lines of prose into a parseable nine-step procedure — `compute-review-scope` → `discover-rule-files` → `process-waivers` → five `performReview` passes → `write-review` — with the mechanical bookkeeping delegated to the primitives; `review.md` is removed from `runtime/legacy-prose-commands.txt`. The `#3/#4` prose tightening moves the ~1.1 KB "For agent runtimes" host-integration blockquote into `framework/constitution.md` §runtime-host-integration once and replaces it in every command with a one-line pointer, dropping the redundant `(MCP: …)` parentheticals and "Otherwise, follow the markdown-only path" tails; `scripts/lint-tool-coverage.sh` now exempts a command that carries the pointer from the per-reference proximity check.
- Ships in lockstep with the framework per [§runtime-boundary](../framework/constitution.md#runtime-boundary). `cargo test` (475 lib tests + the integration suites, including new MCP coverage for the four review primitives and a review-command parse-parity test), `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, the parseability / tool-coverage lints, the framework self-audit, and markdownlint are clean.

## [0.14.2] — 2026-07-01

### Fixed

- **MCP tool schemas no longer carry non-standard numeric `format` hints.** `schemars` stamps OpenAPI-style `format` values (`uint32`, `uint64`, `uint8`) onto every Rust integer/float field, so every tool's input/output schema exposed over MCP annotated its numeric properties (`task-number`, `created`, `updated`, `unchanged`, `commit-count`, `line`, `bytes`, …) with a `format` JSON Schema defines no meaning for. Strict clients validate the served schemas and log a `unknown format "uint32" ignored in schema at path …` warning for each occurrence — opencode emits dozens on connect. `GovRuntimeServer::new` now walks every tool's input and output schema at construction and drops `format` from any node typed `integer`/`number`, leaving string formats (`date-time`, `uri`, …) untouched. This required pointing the `#[tool_handler]` at the stored, sanitized `self.tool_router` (`router = self.tool_router`) instead of its default `Self::tool_router()`, which regenerated a fresh, unsanitized router on every `list_tools`/`call_tool`. New `tests/mcp.rs` regression `no_tool_schema_carries_a_nonstandard_numeric_format` asserts no served schema exposes a numeric `format`. Schema-only change: no primitive behavior or parity golden is affected.

## [0.14.1] — 2026-06-30

### Fixed

- **`apply-manifest` preserves the source file's executable bit on the destination.** Every primitive write goes through `write_atomic_bytes`, which materializes the destination from a fresh `NamedTempFile` (mode `0600` on Unix) and renames it into place — so the written file always lost its executable bit. Invisible for the markdown/JSON the manifest mostly ships (git tracks only the exec bit), but fatal for the generator scripts: the **Shared Files** manifest ships `scripts/gen-spec-deps.sh` and `scripts/gen-cross-service-refs.sh` with `update` strategy, and the `govern-pre-commit` hook execs them. A `/govern` run therefore rewrote those generators to `0600`, failing the adopter's pre-commit hook; worse, a fresh adopter's first run hit the `created` path and emitted a non-executable generator from the start. `apply-manifest` now mirrors the source file's `Permissions` onto the destination after each write (`mirror_source_mode`, `cp -p` semantics) on the `created` / `updated` / `skip-if-conflict`-created paths; the `unchanged` path still never touches the file, preserving its mtime and its already-correct mode. This makes `apply-manifest` consistent with `extract-archive`, which already preserves archive-header modes (`apply_unix_mode`) so the staging source carries `+x` for the mirror to copy. New Unix-only regression tests `source_executable_bit_propagates_to_dest_on_create_and_update` and `skip_if_conflict_propagates_source_executable_bit`.

## [0.14.0] — 2026-06-30

### Added

- **Configurable spec-root directory name (spec 040).** The directory holding all govern artifacts is no longer hardcoded to `specs`. A new optional `[paths] specs-root` key in the committed `.govern.toml` names it, defaulting to `specs` so every existing adopter is unaffected. New `runtime/src/schema/paths.rs` parses and validates the value: a single directory-name segment in the conservative `[A-Za-z0-9_-]` charset (no path separators, no `.`/`..`, no regex metacharacters), so the name is safe both as a literal path component and when interpolated into the generators' regexes; an empty or out-of-charset value falls back to `specs`. Per [§runtime-boundary](../framework/constitution.md#runtime-boundary) the runtime reads this git-tracked source of truth — it does not own it. New `runtime/tests/specs_root_override.rs` covers the default, a non-`specs` override, and rejection of malformed values end-to-end.

### Changed

- **Every spec-root-joining primitive resolves the configured root through one shared resolver.** The primitives that take a bare *feature name* and join it under the root internally — `read-spec`, `set-status`, `mark-task`, `mark-criterion`, `read-tasks`, `traverse-deps`, `check-stuck`, `derive-boundary`, `resolve-references` — and the tree-enumerating `dashboard` previously hardcoded `repo.join("specs")`; they now resolve `[paths] specs-root` from `.govern.toml` (default `specs`) through a single helper in `primitives/mod.rs`, so the two resolutions cannot drift and the default path stays byte-identical to before. Primitives that take a full spec *path* argument (`write-session`, `lint-markdown`, `substitute-templates`) are unchanged — the host bakes the resolved root into that path. Runtime error messages that name a spec artifact now reflect the configured root (e.g. `governance/040-foo`) with no hardcoded `specs/` prefix.
- **`/gov:implement` no longer emits the `planned → in-progress` gate-confirm (spec 000).** Invoking the command is itself the user's approval to start work, so the implement walk drops the gate-confirm step and `writeCode` becomes the first request; the `implement-basic` golden, fixtures, and parity expectation are re-blessed to match.

### Fixed

- **`check-rule-ids` harvests digit-bearing rule-ID categories and matches the canonical deprecation label.** The rule-ID regex is widened to the schema grammar `[A-Z][A-Z0-9]+`, so digit-bearing categories (e.g. `FE-A11YFORM-*`, `FE-A11YMEDIA-*`) are harvested instead of being reported missing; `is_deprecated` now matches the canonical `**DEPRECATED in {version}:**` label rather than a bare `**Deprecated**` / `[DEPRECATED]`.

### Notes

- The `/ask` slash command was renamed to `/amend`; the runtime's mechanical references (the legacy-prose-command list, primitive doc strings and comments) were updated in lockstep — no behavior change.
- **Framework, shipping in this tagged archive (no runtime-crate impact):** `gen-cross-service-refs.sh` is now root-aware — it resolves a *referenced* service's own `[paths] specs-root` (spec 030 task 13, scenario `referenced-service-spec-root`) instead of assuming `specs/`, so a referenced service that renamed its spec root still harvests. `lib/specs-root.sh` gained a reusable `specs_root_of <toml>` helper.
- Ships in lockstep with the framework per [§runtime-boundary](../framework/constitution.md#runtime-boundary). `cargo test` (412 tests + the integration suites), `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, both generator suites, the framework self-audit, and markdownlint are clean.

## [0.13.0] — 2026-06-20

### Added

- **`gvrn exec` resolves the `opencode` layout's singular `command/` directory (spec 032 runtime follow-up).** OpenCode installs its slash-command files under a *singular* `.opencode/command/{project}/<name>.md`, unlike claude-style's plural `.opencode/commands/…` (Claude, Auggie). The two command-resolution callsites (`main::run_exec` and `interpreter::payload::locate_command_file`) baked in `commands/` (plural), so `gvrn exec <name>` could never find an OpenCode adopter's command file — OpenCode shipped on the markdown-only path only (spec 032 Decision 10 deferred this to a 022 follow-up). The plural/singular knowledge is now factored into one new public method, `Host::command_file_candidates` (`runtime/src/host.rs`), which returns both flat-namespaced layout paths in order (plural first, then singular); both callsites consume it, so they can no longer drift. The two candidates are mutually exclusive per adopter (each agent's `cli_config_dir` and `layout` select exactly one), so trying both resolves any supported layout without the runtime knowing which agent wrote the file. Existing claude-style adopters are unaffected — the plural form is tried first and resolves exactly as before. Layout detection needs no new host-config field (the layouts are told apart by trying both candidates); where `cli-config-dir` is read from is the separate concern handled by the relocation below. New unit test `command_file_candidates_cover_both_layouts_plural_first` and a new `exec-opencode` parity fixture + subprocess test (`exec_resolves_command_via_opencode_singular_command_dir`) cover it.

### Changed

- **`cli-config-dir` relocated from committed `.govern.toml` to the per-contributor `.govern.session.toml`.** The `[host]` block committed both `cli-config-dir` and `project`, but `cli-config-dir` names the contributor's agent config dir (`.claude` / `.augment` / `.opencode` / `.agents`) — a per-contributor choice, since teammates on one project may use different agents. Committing it baked one contributor's agent into shared config, so everyone else resolved command files against the wrong directory. `project` (the shared slash-command namespace) stays committed; `cli-config-dir` moves to the gitignored, per-contributor `.govern.session.toml`. `crate::host::Host::load` now reads `cli_config_dir` from the session file → legacy `.govern.toml` `[host]` (so existing adopters keep working) → default `.claude`, and `project` from `.govern.toml`. `write-session` becomes a **merge-writer** over the session file (all on-disk keys optional): a *target write* (`feature`+`path`) sets the target and preserves `cli-config-dir`; a *host-config write* (`cli-config-dir`, no `feature`) sets the agent identity and preserves the target — so `WriteSessionArgs.feature`/`path` are now optional and a new `cli-config-dir` arg is accepted (existing callers that always pass `feature`+`path` are unaffected). `dashboard`'s session reader treats `feature` as optional (a session file with only `cli-config-dir` reports no target rather than erroring). On the framework side, `/govern` (`framework/bootstrap/govern.md` §Instructions step 6) writes `project` to `.govern.toml` `[host]` and `cli-config-dir` to the session file; `/{project}:target` (`framework/commands/target.md`) preserves `cli-config-dir` across a target switch and on `--clear`. Migration is self-healing — no dedicated primitive: the legacy fallback keeps adopters resolving until their next `/govern`, which drops `cli-config-dir` from the committed managed block and records it in the session file. New unit tests cover the resolution order, the merge-writer's target/host-config writes and validations, and the dashboard's optional-feature path.

### Fixed

- **`merge-managed-block` (line-prefix) now absorbs a *structure-changing* canonical edit without leaving an orphan tail.** The previous fix (v0.12.0, see below) made `find_line_prefix_block`'s `walk_body_extent` bound the on-disk block by walking `block.lines().count()` lines using the new canonical as a *structural* template — expected-blank matches on-disk-blank, expected-non-blank-but-on-disk-blank terminates. That works only when the on-disk block and the new canonical share the same shape (same line count, same blank-line positions): a comment-wording tweak, the realistic case it was written for. It does **not** hold when the canonical gains or loses a subsection — exactly what happens when a new agent's gitignore block is added (e.g. Auggie's `.augment/*` + `!.augment/commands/` inserted between the Claude and Antigravity subsections in [`framework/templates/project/gitignore`](../framework/templates/project/gitignore)). The new canonical is then longer than what an existing adopter has on disk; the structural walk drifts by the inserted line count and terminates ~4 lines early, so the old block's tail subsections (`# govern session state` / `# IDE` / `# OS`) spilled past `body_end` into `after`. The cross-boundary dedup pass then stripped their pattern bodies but preserved their comment headers, leaving a trail of orphan headers below the freshly written block. The merge converged on the second run (no infinite accumulation) but the orphan comment lines were permanent. `walk_body_extent` now bounds the on-disk block by **group alignment** instead of a line-shape walk: it splits both the canonical and the on-disk region into blank-line-delimited subsections, reduces each to its pattern lines (non-blank, non-comment — the stable identity across wording drift), and aligns them with a two-pointer walk. An on-disk group is in the block when it shares a pattern with the current canonical group (structure-preserving edit), shares a pattern with a *later* canonical group (the canonical inserted a subsection — skip past it), or shares no pattern while canonical groups remain (a full rewrite of that group); the block ends at the first on-disk group reached after the canonical's groups are exhausted (adopter territory, preserved). Full-content replacement of a single-group block and adopter language sections appended after the block are both preserved unchanged. New regression test `line_prefix_multi_subsection_inserts_new_subsection_without_orphan_tail` exercises the inserted-subsection case (clean replacement, Auggie present once, no duplicated headers, adopter `# Rust` tail preserved, idempotent rerun). All 28 `merge_managed_block` unit tests pass, including the v0.12.0 structure-preserving update test unchanged.

- **`.augment/` (Auggie) added to the framework-managed `.gitignore` block.** [`framework/templates/project/gitignore`](../framework/templates/project/gitignore) gained a `.augment/*` + `!.augment/commands/` subsection mirroring the Claude `commands` carve-out — Auggie is a `claude-style` agent (per the Agent Registry in `framework/bootstrap/govern.md`), so its config dir tracks `commands/` exactly as `.claude` does. The block had `.claude` and `.agents` (Antigravity) subsections but Auggie's was never added when the agent was introduced. The two prose enumerations of the managed block in `framework/bootstrap/govern.md` now list `.augment/` alongside `.claude/` and `.agents/`.

## [0.12.1] — 2026-06-17

### Fixed

- **`gen-cross-service-refs.sh` was never distributed to adopters (spec 030 follow-up).** The cross-service-references generator shipped wired only into this repo's own `.githooks/pre-commit`, so it ran on `govern`'s own commits but a `/govern` update in an adopter project never installed or ran it — the derived `references:` index silently never materialized downstream. The generator is now a row in the **Shared Files** manifest (`framework/bootstrap/govern.md`, `update` strategy) so `/govern` copies it, and is invoked by the adopter pre-commit hook (`framework/bootstrap/hooks/govern-pre-commit`) so commits regenerate `references:`.

### Changed

- **Adopter `govern-pre-commit` scopes generators to staged specs (`--staged`).** Both `gen-spec-deps.sh` and `gen-cross-service-refs.sh` gained a `--staged` flag, and the adopter hook now passes it: committing one spec only rewrites the specs in that commit, never dirtying or restaging unrelated tracked specs. `gen-spec-deps.sh` still builds the dependency-cycle graph from the **full** spec set (read all, write staged), so a staged edge that closes a cycle through an unstaged spec is still caught. This repo's own `.githooks/pre-commit` continues to run the generators unscoped, keeping the whole tree in sync.

- **Cross-service-reference authoring is now documented.** A new README "Documenting a reference in a spec" subsection and a spec-template comment show the concrete inline-link shape, the dependency-vs-reference distinction (sibling `../NNN-slug/` links stay dependencies; absolute service URLs become references), and the harvest opt-outs. The template's example link is backtick-wrapped so it is never harvested into an adopter's frontmatter if the comment is left in place.

### Notes

- No runtime crate code changes — the version is bumped in lockstep ([§runtime-boundary](../framework/constitution.md#runtime-boundary)) to ship the framework fixes via the tagged archive. The 416 runtime tests, both generator test suites (with new `--staged` coverage), the framework self-audit, and markdownlint are clean.

## [0.12.0] — 2026-06-14

### Added

- **`resolve-references` primitive + `[services]` registry — cross-service reference resolution (spec 030).** A spec can link a spec in another service by its canonical repo URL; when that service is registered in `.govern.toml` `[services]` (alias → `repo` / `path` / optional `description`) and checked out locally, `govern` resolves the linked spec's lifecycle `status` from the local checkout. The new `resolve-references` MCP tool (`runtime/src/primitives/resolve_references.rs`) classifies each entry of a consumer spec's derived `references:` index into a closed outcome enum — `ok` (status surfaced), `unregistered`, `not-checked-out`, `broken`, or `status-unreadable` — by deterministic predicates, reusing the `validate-frontmatter` machinery (`read_text` / `split_frontmatter` / `ALLOWED_STATUSES`). The canonical repo URL is identity and navigation only and is **never fetched**; resolution reads only `.govern.toml` and the registered local checkouts. References are informative, never dependencies — kept strictly distinct from `dependencies:` and never entering the blocking dependency graph.

- **`[services]` registry schema (`runtime/src/schema/services.rs`).** A new public `Services` / `ServiceEntry` type plus a `from_toml_str` parser and a `duplicate_repos` detector for the `.govern.toml` `[services]` table. An absent table is an empty registry (single-service adopters write nothing); a missing required field surfaces as a parse error rather than a silent default.

- **MCP exposure and command wiring.** `resolve-references` joins `TOOL_NAMES` and gains a `#[tool]` handler in `runtime/src/mcp/server.rs`; the `ResolveReferencesArgs` / `ResolveReferencesResult` / `ResolutionRecord` / `ReferenceOutcome` types land in `runtime/src/schema/primitives.rs`. The tool is registered in `framework/runtime-tools.txt` for the markdown-only-pipeline opt-in invariant and is referenced (each with a graceful markdown-only fallback) by `/gov:status` and `/gov:analyze`.

### Notes

- Backward-compatible, additive release: no existing tool, type, or behavior changes. Adopters with no `[services]` table and no cross-service references see no difference. The markdown-only path resolves references identically via host file tools, and the no-runtime CI job exercises that fallback end-to-end. All 416 runtime tests pass, including the new `runtime/tests/cross_service.rs` golden/parity test asserting the runtime and markdown-only paths produce byte-identical resolution records.

## [0.11.3] — 2026-06-09

### Fixed

- **`lint-markdown` silently dropped every violation.** markdownlint-cli2 v0.22.1 emits a severity token (`error`/`warning`) between the location and the rule ID — `path:line error MD028/alias message` — but `parse_violation_line`'s regex expected the rule immediately after the location (`path:line MD028 …`). No real output line matched, so `violations` came back empty even when markdownlint exited non-zero. The regex now accepts an optional `(?:error|warning)` token before the rule. Both the `gvrn lint-markdown` CLI and the `lint-markdown` MCP tool share the same `run()`, so both paths are fixed.

- **`clean` ignored the exit code.** It was derived solely from `violations.is_empty()`, contradicting the module's documented contract ("exit code 1 and 2+ both flow through as `clean: false`"). A parse miss — or any config/runtime error that produced no recognizable violation lines — therefore reported `clean: true` against a non-zero exit. `clean` is now `violations.is_empty() && exit_code == 0`, mirroring `run-generator`'s exit-code-derived `drift`, so a non-zero exit can never be reported as clean even if output-format drift defeats the parser again. Two parser tests cover the severity-token form (with and without a column).

## [0.11.2] — 2026-05-24

### Changed

- **Bump `toml` 0.8 → 1.x (post-1.0 stable line).** The `toml` crate's first stable major (1.1.2, spec TOML 1.1.0) is now the runtime's TOML parser. The session-file TOML→JSON bridge in `runtime/src/main.rs::run_exec` now deserializes directly into `serde_json::Value` (`toml::from_str::<Value>(&text)`) rather than parsing into `toml::Value` first and re-serializing — the indirect path tripped on `toml::Value`'s reworked Serialize impl in 1.x (table values were nested in a wrapper rather than flattened). The direct deserialization is also a simplification — fewer hops, one less crate-internal contract to track. No public API changes; the `PrimitiveError::Toml` variant's source type updates from `toml v0.8`'s `toml::de::Error` to the structurally-identical `toml v1.x` equivalent.

- **20 in-range dependency refreshes via `cargo update`.** Brings every direct-dep within its current Cargo.toml version range to the latest published version (mostly transitive bumps: `icu_*` 2.1 → 2.2, `wasm-bindgen` 0.2.121 → 0.2.122, `wit-bindgen` 0.46 → 0.57, `web-sys` 0.3.98 → 0.3.99). One direct-dep increment: `serde_json` 1.0.149 → 1.0.150. No code changes required; all 359 lib + 6 + 16 + 10 + 3 + 2 tests pass against the refreshed graph.

### Verified current

`cargo update --dry-run` reports zero pending updates after this release. Every direct dependency declared in `runtime/Cargo.toml` matches the latest published version compatible with its version range (`anyhow`, `clap`, `flate2`, `git2`, `pulldown-cmark`, `regex`, `reqwest`, `rmcp`, `schemars`, `serde`, `serde_json`, `serde_norway`, `sha2`, `tar`, `tempfile`, `thiserror`, `toml`, `tokio`, `walkdir`, `zip`). The only major bump deferred is `zip 8 → 9.0.0-pre2`, held back because 9.x has not yet shipped a stable release.

## [0.11.1] — 2026-05-24

### Changed

- **Swap deprecated `serde_yaml` for the actively-maintained `serde_norway` fork.** Frontmatter parsing (5 callsites under `runtime/src/primitives/{dashboard,read_spec,traverse_deps,validate_frontmatter}.rs` plus the shared `PrimitiveError::Yaml` variant in `runtime/src/primitives/mod.rs`, plus one test-side parse in `runtime/tests/parity.rs`) moves from `serde_yaml v0.9.34+deprecated` (last release March 2024; dtolnay no longer maintaining) to `serde_norway v0.9.42`. The fork is API-compatible — pure `serde_yaml::` → `serde_norway::` mechanical swap, no behavior changes — and currently the most actively-maintained option in the post-deprecation YAML-for-Rust landscape. All 359 lib + 6 exec_subprocess + 16 parity + 10 atomic + 3 mcp + 2 walker tests pass against the new dep unchanged.

## [0.11.0] — 2026-05-24

### Added

- **`Host` config loader and `.govern.toml` `[host]` block — parameterized command-file resolution.** The runtime previously resolved slash-command files via three hardcoded candidate paths, the middle of which baked in both Claude Code's config-dir name (`.claude`) and this repo's slash-command namespace (`gov/`). That combination broke adopters whose layout matched neither default: an Auggie adopter named `acme` has commands under `.augment/commands/acme/*.md` and the runtime's lookup never reached them. A new `gvrn::host::Host` public type and its `Host::load(repo: &Path) -> Self` constructor read the host's values from `.govern.toml`'s `[host]` block (`cli-config-dir` and `project` keys); both command-resolution callsites (`gvrn exec`'s `run_exec` in `runtime/src/main.rs` and the anchor-extractor's `locate_command_file` in `runtime/src/interpreter/payload.rs`) now construct the middle candidate as `{host.cli_config_dir}/commands/{host.project}/{name}.md`. When the block is absent the loader falls back to `.claude` / repo directory basename — preserving this repo's behavior unchanged — via two module-level consts (`DEFAULT_CLI_CONFIG_DIR`, `FALLBACK_PROJECT`). Six unit tests under `runtime/src/host::tests` cover missing file, empty file, block absent, full override, partial override (per-field defaults), and malformed-TOML fall-soft. An integration test under `runtime/tests/exec_subprocess.rs` (`exec_resolves_command_via_parameterized_host_block`) exercises the parameterized lookup against an Auggie-shaped fixture at `runtime/tests/fixtures/exec-auggie/` (no `framework/commands/` tree, command file at `.augment/commands/acme/smoke.md`, `.govern.toml` declaring the override). Closes spec 022's `commands-dir-parameterization` scenario.

- **Bootstrap procedure writes the `[host]` block on every `/govern` run.** `framework/bootstrap/govern.md` gains a new step 6 (between the existing `.gitignore` merge and `enforce-manifest` cleanup) that invokes `merge-managed-block` against `.govern.toml` with `marker-style: "line-prefix"` and `marker: "govern (host)"`. First-run creates the file with just the managed block; subsequent runs update the values in place under the `# govern (host)` preamble line, preserving every other section (`[pinned]`, `[workflows]`, `[migrations]`, `[review]`) byte-for-byte. The §Project Configuration section's example TOML and per-key reference now document the `[host]` schema; step 1's host-context list picks up the new `host-block` item; subsequent steps are renumbered (prior 6/7/8 → 7/8/9).

- **Audit Family 13 — runtime hardcoded paths (`scripts/audit/runtime-hardcoded-paths.sh`).** `git grep`-scans `runtime/src/**` for the literal `.claude/commands/gov/` string. Any match signals a regression to the host/project hardcode the parameterization removed, with a SUGGESTED-FIX pointing at `runtime/src/host.rs::Host::load`. Specs, scenarios, migration bodies, tests, and fixtures are out of scope (the prior path appears in them legitimately as historical context). Wired into `scripts/audit/run-all.sh` after Families 1–12.

### Fixed

- **Auggie / non-default-layout adopters can resolve their command files.** Before this release, an adopter whose `cli-config-dir` was not `.claude` or whose project namespace was not `gov` would invoke `gvrn exec <name>` and get `runtime exec: command file not found` — because the runtime's second candidate path was the literal `.claude/commands/gov/<name>.md` regardless of the adopter's actual layout. With the parameterized lookup, the runtime reads the adopter's values from `.govern.toml`'s `[host]` block and resolves the correct path on the first try.

### Changed

- **`framework/commands/analyze.md` and `framework/migrations/spec-and-plan-sunset.md` no longer cite the dropped "frozen archaeology" rule.** Spec 023's `living-specs` scenario removed the exception from constitution §drift-prevention; two live framework files still referenced it. `analyze.md`'s scenario-frontmatter check is reframed as backward compatibility (pre-017 scenarios written before the `section` field existed may still carry `spec-ref`). `spec-and-plan-sunset.md`'s done-spec rename is reframed as a §spec-lifecycle mechanical edit (uniform body-identical change, no back-edge). The generated `.claude/commands/gov/analyze.md` mirror was regenerated by `scripts/gen-claude-commands.sh`.

### Documentation

- `AGENTS.md` Workflow section gains a new entry codifying the "never use frozen-archaeology phrasing" convention (rule + **Why:** + **How to apply:**), so the dropped term doesn't sneak back into future work.

### Test infrastructure

- **`runtime/tests/common/mod.rs` — shared helpers for integration test crates.** Each `tests/*.rs` compiles as its own integration-test binary, so previously-shared logic (`copy_dir_recursive`) had been duplicated across `parity.rs` and `exec_subprocess.rs`. The new sub-module path is the idiomatic Rust workaround: `tests/common/mod.rs` is NOT auto-built as a test binary (a `tests/foo.rs` file would be), and each integration test declares `mod common;` to bring the helper into scope. Both call sites now import `common::copy_dir_recursive` from the canonical implementation.

## [0.10.0] — 2026-05-24

### Added

- **`migrate-session-file` primitive — backs the `session-file-consolidate` bootstrap migration with tested code.** New MCP tool and CLI subcommand that reads a pre-0.10.0 legacy session JSON (`.claude/{project}-session.json`), applies the camelCase→kebab-case key renames (`scenarioPath`→`scenario-path`, `setAt`→`set-at`), writes the result to `.govern.session.toml` via tempfile+rename, and deletes the legacy file. Idempotent: returns `action: "no-legacy"` when no legacy file exists, `action: "kept-existing"` when `.govern.session.toml` is already present (legacy still deleted), `action: "migrated"` on a fresh translation. The destination path is sourced from `write_session::SESSION_FILE`, so the migration cannot drift from the runtime's read/write path. 10 new unit tests under `runtime/src/primitives/migrate_session_file.rs` cover happy path, idempotency, preserve-existing, non-standard-key preservation, malformed-input rejection, path-traversal rejection, and (critically) a compile-time assertion that the primitive's destination equals `SESSION_FILE`. The `framework/migrations/session-file-consolidate.md` body now invokes the primitive on the runtime path with a markdown-only fallback for adopters without `gvrn` on `PATH`.

- **Audit Family 11 — consolidation-pair drift (`scripts/audit/consolidation-pair.sh`).** Verifies the consolidated session-file path agrees across every artifact that references it: the runtime `SESSION_FILE` constant, the migration body's destination prose, the framework gitignore template, and the Claude configure-permission file. Also asserts the migration body names both legacy keys (`scenarioPath`, `setAt`) AND their kebab-case replacements (`scenario-path`, `set-at`) so the rename contract is auditable — a silently dropped rename would leave adopters with camelCase keys the runtime ignores. Verified against synthetic drift in both axes.

- **Audit Family 12 — fixture session-file shape (`scripts/audit/fixture-session-shape.sh`).** Verifies every `runtime/tests/fixtures/*/.govern.session.toml` (a) parses cleanly as TOML and (b) does not use the legacy camelCase keys. Test-data complement to Family 11. Verified against synthetic drift.

### Changed

- **Session state consolidated onto `.govern.session.toml` at the repo root.** `write-session` and `dashboard` previously read/wrote `.claude/gov-session.json` — a hardcoded path that baked in both the AI CLI's config directory (`.claude/` for Claude Code) and the adopting project's name (`gov-session.json` for this repo). That broke adopters whose project name or AI CLI didn't match the runtime's baked-in constants (observed against an adopter whose project name and AI CLI both differed from the runtime's defaults, whose canonical session would have been `.claude/{project}-session.json`): `/{project}:target` wrote the gov-shaped filename while every downstream consumer read the bootstrap-substituted one, and the session never round-tripped. The fix is consolidation, not parameterization — both primitives now read/write `<repo>/.govern.session.toml`, a single location with no `{cli-config-dir}` or `{project}` variability. The new file sits alongside `.govern.toml` at the repo root, is gitignored (per-user, ephemeral state), and uses TOML to align with `.govern.toml`'s on-disk format. Keys are kebab-case (`scenario-path`, `set-at`) rather than the legacy camelCase (`scenarioPath`, `setAt`). The runtime CLI's walker-context seed in `gvrn exec` reads the same file via TOML→JSON bridging so parity fixtures keep working. Closes spec 022's reopened consolidation scope.

- **`merge-permissions`'s `path` is now required.** The previous `DEFAULT_PATH = ".claude/settings.local.json"` constant silently routed non-Claude hosts to a Claude-shaped destination. The bootstrap procedure already passes the path explicitly via `{cli-config-dir}/settings.local.json`, so the default was unused on every supported invocation path; removing it makes a missing path fail loudly instead of corrupting an Auggie adopter's settings.

### Migration

- **Bootstrap migration `session-file-consolidate`.** Added to `framework/migrations.toml` (introduced in 0.10.0). On the next `/govern` run, the bootstrap detects any legacy `{config_dir}/{project}-session.json` files (across every selected agent's config dir), translates the most recent one into `.govern.session.toml` with kebab-case key renames (`scenarioPath` → `scenario-path`, `setAt` → `set-at`), and deletes the legacy file(s). Adopters who skip the bootstrap can also clear the legacy file by re-running `/{project}:target` once after upgrade.

- **`.govern.session.toml` added to the framework-managed `.gitignore` block.** The next `/govern` run rewrites `.gitignore` via `merge-managed-block` and the new entry lands automatically.

### Schema (wire-compatible, no breaking deserialization)

- `MergePermissionsArgs.path` changed from `Option<String>` to `String`. CLI callers that omitted `--path` now get a clap error instead of a silent default; the bootstrap procedure already passes the path on every call.
- `DashboardArgs` reverts to an empty struct. Session-target reads use the constant `.govern.session.toml` path; callers do not pass a `session-path` arg.
- `WriteSessionArgs.session_path` was *not* added (intermediate path-parameterization design rejected). The TOML on-disk shape uses kebab-case keys (`scenario-path`, `set-at`); the JSON wire shape for CLI/MCP args is unchanged from 0.9.x.

### Documentation

- Framework command sources (`framework/commands/*.md`), the bootstrap procedure (`framework/bootstrap/govern.md`), the configure permission file (`framework/bootstrap/configure/claude.md`), and `framework/constitution.md` reference `.govern.session.toml` exclusively. The "Session JSON path" derived-value row in the bootstrap's per-agent registry table is removed (the session file is no longer per-agent).

## [0.9.2] — 2026-05-23

### Fixed

- **`merge-managed-block` (line-prefix) detects the end of a multi-subsection canonical correctly.** The primitive's `find_line_prefix_block` helper previously used a "next blank line is the terminator" heuristic to bound the on-disk canonical block. That heuristic mis-truncated canonicals containing interior blank lines between subsections — the shipped `.gitignore` template ([`framework/templates/project/gitignore`](../framework/templates/project/gitignore)) is exactly this shape — so the `body == block` comparison could never succeed on multi-subsection canonicals and the *updated* arm's `after = &text[body_end..]` re-emitted the tail subsections as adopter-area content. The cross-boundary dedup pass stripped non-comment body lines but preserved subsection-header comment lines, so each `/govern` rerun left an orphan trail of empty `# Environment and secrets` / `# Claude Code …` / `# IDE` / `# OS` headers below the real block. The helper now walks up to `block.lines().count()` lines from the marker using the supplied block as a *structural* template: expected blank lines (interior subsection separators) match against on-disk blanks; an unexpected blank (non-blank expected, blank found) signals the end-of-block terminator. Two new unit tests under `runtime/src/primitives/merge_managed_block.rs::tests` cover the regression — a stable-rerun assertion (`action == "unchanged"`, `dedup_removed == 0`, mtime preserved) and a content-changed-update assertion (clean replacement, each subsection header appears exactly once). All 27 existing `merge_managed_block` unit tests pass unchanged. Closes spec 022's `merge-managed-block-multi-subsection-end` scenario.

## [0.9.1] — 2026-05-23

### Fixed

- **`traverse-deps` now detects cycles in the reachable dep subgraph.** The primitive previously checked dependency existence and per-edge status compatibility but ignored graph-level acyclicity, so a 2-cycle (`A → B → A`), a self-cycle (`A → A`), or any deeper SCC slipped through silently. Spec 017's `gen-spec-deps.sh` blocks cycles at commit time, but that check does not cover adopters on an older shipped script, skipped pre-commit hooks, or stale frontmatter edits that drift from the body links — all paths where a cycle can re-enter the artifact tree without the generator firing. The primitive now runs Tarjan's strongly-connected-components algorithm over the subgraph it walks and reports every non-trivial SCC (size ≥ 2, or size 1 with a self-edge) in a new `cycles: Vec<Vec<String>>` result field; each entry names the participating slugs in traversal order. Any non-empty `cycles` flips `compatible` to `false`, so `/gov:analyze` step 3 fails its gate without further wiring. Eight new unit tests under `runtime/src/primitives/traverse_deps.rs` cover the scenario's five edge cases (cycle among `done` specs still reported; self-cycle as 1-cycle; multiple disjoint SCCs; missing-node-doesn't-close-a-cycle; stale-frontmatter cycle visible) plus a 3-node hop case; an MCP integration test in `runtime/tests/mcp.rs` and a CLI-subprocess parity test in `runtime/tests/parity.rs` exercise both surfaces against a hand-built 2-cycle fixture so the markdown-only walker (agent + MCP) and the runtime walker (`gvrn traverse-deps`) surface equivalent findings. Closes spec 022's `traverse-deps-cycle-check` scenario.

### Changed

- **`TraverseDepsResult` schema gains a `cycles` field.** Additive and `#[serde(default)]` — adopters reading the JSON envelope on `0.9.0` continue to deserialize cleanly; consumers reading `0.9.1` see an empty array on every acyclic invocation. `framework/commands/analyze.md` step 3 prose now names cycles as blocking and explains the defense-in-depth relationship with spec 017's generator-side check.

## [0.9.0] — 2026-05-23

### Added

- **`write-session` primitive — atomic rewrite of `.claude/gov-session.json`.** New MCP tool and CLI subcommand that writes the session-target record (feature, path, optional scenario + scenarioPath, setAt) through the same tempfile + rename pattern every other state-modifying primitive uses. Pairs with `dashboard`'s read of the same file: spec 022 already listed the session file as one of two durable journals (markdown + `.claude/gov-session.json`), and the read path was in the runtime since 0.8.0; the write path closes the asymmetry. On Claude Code, routing the write through MCP moves consent from the per-invocation `Write({cli-config-dir}/{project}-session.json)` permission prompt — which existing `Write(...)` allow entries did not reliably suppress across sessions — into the MCP tool-permission lane, where a single allow covers every subsequent `/gov:target` and `/gov:ask` scenario-switch. 13 new unit tests under `runtime/src/primitives/write_session.rs` cover the happy paths (with/without scenario, fresh-vs-overwrite, directory creation), error paths (mismatched scenario pair, parent-component path, absolute scenario path), and the atomic-write contract (dropped tempfile leaves destination unchanged).

### Changed

- **`framework/commands/target.md` step 7 now invokes `write-session`.** The host-write prose is replaced with the primitive call; the markdown-only fallback still writes the same JSON shape directly with the same tempfile + rename semantics. Step 1 additionally names `{cli-config-dir}/{project}-session.json` inline (with the Claude resolution to `~/.claude/gov-session.json`) so hosts no longer have to derive the path from the parity `strict-files` frontmatter.

- **`framework/commands/ask.md` scenario-route step 4 now invokes `write-session`.** Same migration as target.md: the "host responsibility — the runtime exposes no session-shaped primitive" wording is removed; the markdown-only fallback remains.

- **`framework/runtime-tools.txt` gains the `write-session` line.** Matched by the parser's `PRIMITIVE_NAMES` and the MCP server's `TOOL_NAMES`.

- **`runtime/tests/golden/target-basic.jsonl` updated.** The byte-stream now includes the `write-session` dispatch envelope between `read-spec` and `complete`. Re-blessed via `BLESS=1 cargo test target_basic`.

## [0.8.1] — 2026-05-23

### Changed

- **Internal: `section_lines` extracted to `primitives/mod.rs`.** Both `read_spec::parse_open_questions` and `dashboard::{count_open_questions, context_summary}` now share the section-traversal helper via `primitives::section_lines` (new `pub(crate)` function). The iteration semantics are the single source of truth; consumers diverge only in how they fold the yielded lines into their result shape. Closes the `count_open_questions` / `parse_open_questions` semantic-drift surface the `/gov:review` pass against `c15ae0e` flagged on pathological inputs. Six new direct unit tests in `primitives::tests` cover the extracted helper.

- **Internal: `is_feature_slug` promoted to `primitives/mod.rs`.** The `NNN-feature` pattern check moves from `dashboard.rs` to `primitives/mod.rs` as `pub(crate)`, alongside `validate_slug` and `validate_no_traversal`. Currently one caller, but the pattern recurs across the codebase and the helper is small enough to promote ahead of demand.

- **Internal: `load_session_target` no longer accepts an unused `&[DashboardSpec]` parameter.** The dashboard scenario's last edge case explicitly forbids the session-target validation that parameter was prospective for ("Return the session-target field as-recorded; do not validate against the `specs` array"). The parameter existed for a use case the scenario contract rules out; removing it tightens the signature without changing behavior.

No behavior changes, no schema changes, no public surface changes. CLI subcommands, MCP tool shapes, and protocol envelopes are byte-identical to `0.8.0`. Patch bump per the runtime's convention for internal cleanups that leave the wire contract unchanged (precedent: `0.5.2`, `0.7.3`).

## [0.8.0] — 2026-05-23

### Added

- **`dashboard` primitive — single-call surface for `/gov:status`.** New MCP tool and CLI subcommand returning the per-spec inventory (slug / status / dependencies / tags / open-question-count / has-plan / has-tasks / has-data-model / scenarios-count / blocked-by), the repo-wide `tags-union`, the `.govern.toml` review-state summary (`{present, disabled-rule-files}`), and the optional session target (with `scenario-detail` populated when a scenario is targeted) in one call. Collapses the previous `/gov:status` "list specs + N read-spec + shell for-loop + cat .govern.toml" dance — which the §Instructions preamble already forbade as a fallback substitute — into a single MCP round-trip. `blocked-by` is computed in-primitive as the subset of `dependencies` whose own status is below `clarified`; `tags-union` is the sorted, deduplicated fold across every spec's `tags` array. 15 new unit tests under `runtime/src/primitives/dashboard.rs` cover the happy path plus every edge case enumerated in the scenario (empty `specs/`, missing `spec.md`, non-pattern dirs, `.govern.toml` absent / present-empty / parse-failure, scenarios with non-md files, session absent, session targeting a stale scenario, blocked-by computation, open-question continuation lines).

### Changed

- **`framework/commands/status.md` collapses to a single path.** The short-circuit branch (steps 2.1 / 2.2 — "stop after read-spec when target is not `done`") is removed. The procedure now invokes `dashboard` unconditionally and renders a one-line preamble above the table that surfaces the targeted feature (and scenario, when present) plus its next action. The §Instructions preamble names `dashboard` as the deterministic target for the status command so the shell-utility ban has a positive callout.

- **`Frontmatter` schema gains a serde-default `tags` field.** Backwards-compatible: specs that omit `tags:` continue to deserialize with an empty `Vec<String>`. Existing primitives (`read-spec`, `traverse-deps`) see the new field but do not surface it; `dashboard` is the first consumer.

- **Two new `PrimitiveError` variants.** `Toml` wraps `toml::de::Error` for `.govern.toml` parse failures; `MissingSpecFile` surfaces when an `NNN-feature` directory under `specs/` lacks the expected `spec.md`. Both surface as operational errors that halt the procedure with structured envelopes, consistent with the partial-failure semantics resolved in spec 022.

### Dependencies

- **`toml = "0.8"`.** New dependency used by the `.govern.toml` reader inside the `dashboard` primitive. Small, well-maintained crate; standard choice for TOML parsing.

## [0.7.4] — 2026-05-22

### Fixed

- **`merge-managed-block` cross-boundary dedup no longer destroys canonical content past the first interior blank line.** The dedup pass in `runtime/src/primitives/merge_managed_block.rs` previously re-derived the managed block's bounds by calling `find_line_prefix_block` on the post-merge content, which terminates at the first blank line. Canonical blocks shipped by the framework (notably `framework/templates/project/gitignore`) contain blank-line-separated subsections, so every canonical line past the first subsection was flagged `!in_block` and removed as an "adopter duplicate," leaving section comment headers with no patterns under them. `merge_line_prefix` now computes `block_start` and `block_end` directly from what it writes — `header.len() + 1 + block.len() + 1` past the start offset — and passes them as parameters to `dedup_outside_block`, which no longer re-scans for marker bounds. The contract for canonical blocks (string-equal-line removal in adopter territory, canonical-block wins) is unchanged; only the bounds computation moved from a fragile blank-line walk to a direct measurement. New regression test `line_prefix_preserves_multi_subsection_block_with_interior_blank_lines` exercises a multi-subsection block mirroring the shipped `.gitignore` template.

## [0.7.3] — 2026-05-22

### Changed

- **`writeCode` payload bundling now canonicalizes plan-relevant paths and case-folds the secret-pattern guard.** `load_plan_relevant_files` (in `runtime/src/interpreter/payload.rs`) previously joined each Affected-Files entry under `repo` and read it without verifying the resolved path stayed under the repo root. A plan entry of `../../etc/passwd` or `/etc/hosts` bypassed the basename-only secret-pattern check and exfiltrated through the outbound `writeCode` payload. The function now canonicalizes both `repo` and each candidate `abs` and rejects entries whose canonical form does not `starts_with(canon_repo)`, emitting `PayloadError::SecretExfiltration { pattern: "out-of-repo" }` so the existing `secret-exfiltration-blocked` error envelope stays the single surface for the whole class. Missing files (planned-new) still skip cleanly via the canonicalize-fails-continues branch — the existing happy path is preserved. `secret_pattern` also lowercases the basename before pattern matching so `.ENV` on macOS APFS cannot bypass the guard. Four new regression tests cover relative escape, absolute escape, in-repo happy path (positive), and case-fold bypass; the existing planned-new test continues to exercise the canonicalize-skip path for a fifth scenario. Closes the BE-INPUT-004 SHOULD finding recorded in `specs/022-deterministic-runtime/review.md`.

## [0.7.2] — 2026-05-21

### Changed

- **`enforce-manifest` contract trimmed to slash-command manifest enforcement only.** The module docstring previously claimed the primitive replaced three bootstrap cleanup loops (slash-command manifest enforcement, legacy `skills/` directory removal, legacy workflow filename removal). Adopter cleanup of historical conventions has moved out of the primitive's contract and into the registry-driven `## Pre-run Migrations` loop introduced by spec [027 — Bootstrap Migration Registry](../specs/027-bootstrap-migration-registry/spec.md); per-entry procedures live at `framework/migrations/{id}.md` and are dispatched by the bootstrap loop, not by `enforce-manifest`. The primitive itself is unchanged — same `expected` / `pinned` inputs, same `removed` / `kept` / `pinned-kept` outputs, byte-identical behavior — but it is now the slash-command directory's enforcer only, and the docstring says so. The `govern-basic` parity fixture grows one pre-seeded `framework/skills/old-skill.md` plus a `runtime/tests/parity.rs` assertion that the file survives the bootstrap, locking the contract trim in place against regression.

## [0.7.1] — 2026-05-21

### Changed

- **Direct dependencies refreshed to latest majors.** `git2` 0.20 → 0.21, `reqwest` 0.12 → 0.13, `rmcp` 0.8 → 1.7, `sha2` 0.10 → 0.11, `zip` 5 → 8. Plus the transitive bumps cargo picked up (`digest` 0.10 → 0.11, `pulldown-cmark` 0.13.3 → 0.13.4, `tar` 0.4.45 → 0.4.46, `tower-http` 0.6.10 → 0.6.11, etc.). Hygiene-driven; no bug pushed for the bumps. The runtime had no driver to update before, but staleness compounds — clearing the backlog while the runtime is quiescent is cheaper than absorbing the migrations one CVE at a time.

  Migration touched two API surfaces. `reqwest` 0.13 renamed the rustls feature flag (`rustls-tls` → `rustls`), so `Cargo.toml` updated. `rmcp` 1.x made `ServerInfo` and `CallToolRequestParam` (renamed `CallToolRequestParams`) `#[non_exhaustive]`, so the construction sites in `src/mcp/server.rs` (`ServerInfo::new(caps).with_instructions(...)` builder) and `tests/mcp.rs` (`CallToolRequestParams::new(name).with_arguments(args)`) were rewritten through the new builder paths. One `#[allow(dead_code)]` annotation on `GovRuntimeServer::tool_router` because rustc's dead-code analysis doesn't see through the `#[tool_router]` macro — the field is required structurally even though rustc thinks it's unread.

  No behavior change visible at the protocol surface. All 325 tests pass; `cargo clippy --all-targets -- -D warnings` clean; `cargo fmt --check` clean. Two parity goldens unchanged.

- **MSRV bumped from 1.85 to 1.88.** `zip` 8 requires Rust 1.88. Adopters installing on a toolchain older than 1.88 will get a clean cargo rejection rather than a confusing build error. The release matrix runs on `stable`, which is well past 1.88. Side effect: clippy 1.95's `collapsible_if` now suggests let-chains (stabilized in 1.88) for `if let X { if Y { … } }` patterns. Six call sites in `interpreter::payload`, `primitives::append_task`, `primitives::read_spec`, `primitives::mod`, and `main` were rewritten to use `if let X && Y` — REUSE only, no behavior change.

## [0.7.0] — 2026-05-20

### Added

- **`writeCode` request bundling.** The interpreter now populates the typed `WriteCodeRequest` shape end-to-end before emitting an `llm-request` envelope. Three fields previously left empty are now filled from disk: `plan-relevant-files` (entries parsed from the targeted feature's `plan.md` Affected Files table, each inlined as `{path, content}`; planned-new files absent on disk are omitted, not errored), `constitution-excerpts` (sections resolved from the running command file's `Reference: §<anchor>, …` line under Scope Boundaries, each anchor's body extracted from `framework/constitution.md`), and `task` (the targeted feature's tasks.md entry matching `task-number`, with `number`, `heading`, and `subtasks[].text` all populated). The legacy context-dump fields are appended after the typed prefix for backward compatibility with hosts that already parse them.

- **`writeSpecBody` re-run state.** When `/gov:plan` or `/gov:specify` re-runs against a partially-filled spec or plan section, the interpreter reads the existing section body from disk and emits it in the request's `existing-content` field. Empty sections elide the field. Section identification is heuristic-driven for v1 — matches `Fill the <name> section` in the step prose.

- **Secret-exfiltration guard for `plan-relevant-files`.** A new read-side guard refuses to inline files matching `.env`, `.env.*`, `*-secrets.*`, or `credentials*`, plus paths that match the repo's `.gitignore`. The first match halts the procedure with a structured `secret-exfiltration-blocked` error envelope and an unambiguous remediation hint (rename or remove the entry from `plan.md`'s Affected Files). No override flag in v1 — the plan author resolves by editing the plan.

### Changed

- **`WriteCodeRequest` field order is now cache-anchored.** Struct fields reorder to `constitution-excerpts`, `plan-relevant-files`, `write-boundary`, `task` (was `task`, `plan-relevant-files`, `write-boundary`, `constitution-excerpts`). The stable prefix — three fields that do not vary between tasks in the same `/gov:implement` walk — is contiguous and front; the per-task variable (`task`) is last. Hosts implementing prompt caching SHOULD place a cache anchor between `write-boundary` and `task` per the new contract documented in spec 022's `LLM extension points` section. A `serialize-order-lock` test in `schema::extensions::tests` enforces the new layout. Two parity goldens re-blessed (`implement-basic`, `plan-basic`) for the new field order and bundled-payload contents.

### Tests

- 12 new unit tests under `interpreter::payload::tests` cover the new readers and the secret guard (`parse_affected_files`, `parse_command_references`, `extract_anchor_body`, `extract_section_body`, `extract_section_name`, `secret_pattern` for each pattern family, gitignore matching via libgit2, `load_plan_relevant_files` happy and rejection paths, `build_write_code_request` field-order lock, `build_write_spec_body_request` existing-content inlining). The fixture `runtime/tests/fixtures/implement-basic/` grows a small `plan.md` and `framework/constitution.md` exercising the populated bundle; `runtime/tests/fixtures/plan-basic/` grows a partially-filled `plan.md` exercising the writeSpecBody re-run state.

  Origin: spec 022 scenario `writecode-payload-bundling`. Bumps the minor (additive bundling + field reorder — no host wire-format breakage thanks to backward-compat merge).

## [0.6.1] — 2026-05-19

### Fixed

- **`mark-task` ignored phased `tasks.md` files.** The primitive only matched task headings at level 2 (`## N. ...`), so phased files (`## Phase X — … / ### N. Task`, the shape `read-tasks` learned to handle in 0.5.1) returned `task '{N}' not found` for every task. Surfaced 2026-05-19 during `/gov:implement` on spec 023 task #19 — the heading happened to contain backticks (``### 19. Dedup `/configure` permission entries``), so the bug initially looked like a backtick-parser issue, but inline-code spans were never the root cause; the parser handled them correctly via `parse_atx_heading` already.

  Resolution: `mark-task` now calls `detect_tasks_structure` before splitting the file into lines and walks the appropriate task level (2 for flat, 3 for phased), exactly the way `read_tasks.rs` already does. `locate_task_range` takes a new `task_level` parameter; its terminator condition relaxes from the hardcoded `level <= 2` to `level <= task_level` so a phased task's range correctly ends at the next sibling `### N.` heading OR the next `## …` phase container, whichever comes first.

  REUSE-only: `read-tasks`'s observable behavior is unchanged; `mark-task` and `read-tasks` now consume the same `detect_tasks_structure` helper, eliminating the structure-detection drift that caused this bug. Future heading-shape edge cases fix once, propagate to both primitives.

  Four new regression tests cover the previously-broken path: `flips_subtask_in_phased_tasks_md` (basic phased success), `resolves_phased_task_with_backticks_in_heading` (the exact symptom from spec 023 task #19), `phased_task_range_terminates_at_next_phase_container` (range-termination correctness), and `phased_task_set_matches_read_tasks` (cross-primitive agreement — `read-tasks` and `mark-task` recognize the same set of tasks on a phased fixture, the contract named in the scenario's done-when). The 6 existing tests still pass; lib total 268 → 269.

  Origin: spec 022 scenario `mark-task-backtick-headings`, routed from `specs/inbox.md` via `/gov:groom`.

## [0.6.0] — 2026-05-19

### Added

- **`merge-permissions` primitive.** Idempotently merges a canonical permission allow/deny set into a JSON file (default `.claude/settings.local.json`) with exact-match dedup across `permissions.allow` and `permissions.deny`. Inputs: optional `path` (defaults to `.claude/settings.local.json`), `allow: Vec<String>`, `deny: Vec<String>`. Behavior: creates the file with `{"permissions":{"allow":[...],"deny":[...]}}` when absent (`created`); on an existing file, removes exact-match duplicates from each array (first-occurrence wins), ensures every canonical entry is present (appended at end if absent), preserves untouched top-level keys and unspecified keys under `permissions` byte-for-byte, and writes atomically via tempfile + rename. When the parsed file already equals the post-merge value structurally, emits `unchanged` and does not rewrite — preserves mtime for build-tool idempotency, matching `merge-managed-block`'s contract. Result envelope reports per-array counts of entries added vs. duplicates removed. Refuses with a `Json` parse error on malformed JSON, with a `JsonSchema` error when `permissions.allow` / `permissions.deny` exists but is not an array, or when the top-level value is not a JSON object. New `PrimitiveError::Json` and `PrimitiveError::JsonSchema` variants. 15 unit tests cover every edge case.

  Origin: spec 022 scenario `framework-list-dedup` (consumed by spec 023 `configure-dedup-permissions` to land the `/configure` dedup invariant). Registered as both the CLI subcommand `gvrn merge-permissions` and the MCP tool exposed under the bare name `merge-permissions` (Claude: `mcp__gvrn__merge-permissions`; Auggie: `mcp:gvrn:merge-permissions`). `framework/runtime-tools.txt` updated.

### Changed

- **`merge-managed-block` cross-boundary dedup (line-prefix style only).** After the existing block install/update pass, the primitive scans adopter-owned territory (everything outside the `# {marker}` preamble line and its blank-line terminator) for lines that string-equal a non-blank, non-comment line inside the canonical block. Matching adopter-area lines are removed in place — canonical-block wins. Adopter-owned blank lines and comment lines (`#` lines other than the marker itself) are preserved untouched even when their content matches a canonical line. Comparison is exact string-equality after stripping trailing `\r`; no glob expansion, no path normalization (`.claude/` and `.claude/*` are distinct). The result envelope grows two new fields on `line-prefix` invocations: `dedup-removed` (count of removed lines) and `dedup-removed-lines` (verbatim removed lines in source order). The `html-comment` style is unchanged — `dedup-removed` and `dedup-removed-lines` are `None` and elided from the JSON envelope when serialized (`skip_serializing_if = "Option::is_none"`). 10 new unit tests cover the line-prefix dedup paths; the 13 existing tests still pass.

  Motivating use case: `.gitignore` managed via `merge-managed-block` accumulated duplicates outside the `# govern` marker when adopters pasted a canonical pattern (e.g., `.claude/`) into adopter-owned territory. With cross-boundary dedup the canonical block stays the single source of those entries.

- **`check-stuck` `find_in_progress_commit` REUSE refactor.** Inline `tree.get_path(...).find_blob(...).content()` chain replaced with the existing `read_blob_from_tree` helper (introduced for `check-stuck-tasks-md-advancement` in 0.5.2). REUSE-only; observable behavior unchanged. Origin: spec 022 scenario `check-stuck-read-blob-reuse`.

- **`serde_json` `preserve_order` feature.** Enabled so user-supplied JSON key order in `.claude/settings.local.json` survives `merge-permissions` round-trips. Side effect: every JSON `Value` serialized by the runtime now preserves insertion order rather than alphabetizing keys. Three parity goldens re-blessed (`analyze-basic`, `implement-basic`, `plan-basic`) for the new key order in `llm-request` envelopes. New `BLESS=1` env-var path in `runtime/tests/parity.rs` enables future bulk re-blessing of the corpus.

### Tests

- 25 new unit tests added (15 for `merge-permissions`, 10 for `merge-managed-block` cross-boundary dedup). Total: 299 passing (`cargo test --release`); clippy clean across `--all-targets`; fmt clean.

## [0.5.2] — 2026-05-18

### Fixed

- **`check-stuck` over-reported false positives.** The primitive set `stuck = count >= threshold` based purely on the commit count of `tasks.md` since the most-recent `in-progress` transition. `/gov:implement`'s contract specifies a second condition that was not enforced: `stuck: true` should only fire when the same task is still the first incomplete one (no checkbox flipped to `- [x]` between commits in the window). Once 3+ commits landed on `tasks.md` — even when each flipped a different subtask — `stuck: true` fired on every subsequent run for the remainder of the feature, training operators to dismiss the warning.

  Resolution: the new `first_incomplete_index_unchanged` helper reads `tasks.md` at both `since-sha` and HEAD, finds the line index of the first `- [ ]` group in each (skipping fenced code blocks), and returns `true` only when both indices exist and match. `stuck` now requires `count >= threshold AND first_incomplete_index_unchanged`. Vacuous-false cases (no `tasks.md` at `since-sha`; all subtasks complete at HEAD) leave `stuck: false` — completion is the opposite of stuck.

  Subtask-identity equality is position-based for v1 (per scenario `check-stuck-tasks-md-advancement` Q1 resolution): matches how `/gov:implement` already walks tasks; reordering during implementation is rare and breaks the implicit ordering contract anyway. Heading-text equality is a future enhancement if reorder churn surfaces.

  New regression test `stuck_false_when_checkboxes_flipped_across_threshold_commits` exercises the false-positive case (4 commits, each flipping a different subtask). The five existing tests still pass — they each flip no checkboxes between commits, so the new condition holds and `stuck` correctly fires.

  No schema changes; `CheckStuckArgs` and `CheckStuckResult` JSON shapes are unchanged. Lib tests 238 → 239; full crate suite still passes.

  Reported 2026-05-17 from an adopter repo's own spec 017 (second occurrence). Inbox-routed via `/gov:groom`.

## [0.5.1] — 2026-05-17

### Fixed

Four structural bugs in `tasks.md` primitives surfaced during spec 023's living-specs work, resolved by the `runtime-primitive-structural-bugs` scenario on spec 022:

- **`append-task`'s default body line used the title as the slug** — a title like `Implement scenarios/living-specs.md` produced `scenarios/scenarios/living-specs.md.md`, doubled prefix and extension. Resolution: new explicit `slug` argument (`AppendTaskArgs.slug: Option<String>`). When `body` is omitted, `slug` is required; the primitive refuses with the new `PrimitiveError::MissingArgument` variant if both are omitted. When `body` is supplied, `slug` is ignored. The buggy heuristic that derived the slug from the title is removed entirely.
- **`append-task` numbering hardcoded to `## N.` top-level** — on phased `tasks.md` files (`## Phase A — … / ### N. Task` shape), the primitive found no `## N.` matches and fell back to `## 1.` at the file's bottom, colliding with the existing `### 1.` task and breaking the file's H3 convention. Resolution: new `TasksStructure` enum (`Flat` / `Phased`) detected by presence of any `### N.` heading. New `AppendTaskArgs.parent_heading: Option<String>` lets the caller name the phase to extend; refuses with the new `PrimitiveError::ParentHeadingNotFound` variant when the supplied heading does not match. When `parent_heading` is omitted, the primitive extends an existing `Phase X — Follow-on scenarios` phase if one is present, otherwise creates `Phase {next-letter} — Follow-on scenarios` with the auto-computed next letter. Phase containers explicitly exclude `## N.` numeric headings, so mixed-structure files keep their phase set clean.
- **`read-tasks` returned empty on phased files** — the parser only matched `## N.` level-2 headings and blinded `/gov:implement` on every phased spec. Resolution: structure-aware task-level dispatch — phased files walk `### N.` at level 3, flat files keep walking `## N.` at level 2. New `Task.phase: Option<String>` carries the heading text of the containing phase for phased tasks; the field is absent from JSON output for flat tasks (`skip_serializing_if = "Option::is_none"`) so pre-existing consumers parse unchanged. Mixed-structure files walk only the phased layer per the scenario's edge case.
- **`check-stuck` reopen regression coverage** — investigation showed the topological-reverse revwalk already tracked the most-recent `in-progress` transition correctly (the bug had been resolved in earlier 022 work without closing the scenario task). Added three regression tests under `primitives::check_stuck::tests` to lock the correct behavior in place: `reopen_measures_from_most_recent_in_progress_transition`, `first_in_progress_works_when_never_reopened`, and `mechanical_sweeps_do_not_disturb_since_sha`.

### Changed

- New shared helpers in `primitives::mod`: `TasksStructure`, `detect_tasks_structure`, `iter_task_numbers_at_levels`, `iter_phase_ranges`, `PhaseRange`. Used by both `append-task` (Phase 2) and `read-tasks` (Phase 3) to keep phased-structure detection in one place. The deprecated single-purpose `iter_numbered_headings` wrapper is removed; callers in tests now invoke `iter_task_numbers_at_levels(content, &[2])` directly.

### Tests

- 26 new unit tests across `append_task`, `read_tasks`, and `check_stuck` covering the four bug fixes and their edge cases. Total lib tests: 235 → 238; full crate suite: 269 passing.

## [0.5.0] — 2026-05-17

### Changed (breaking)

- **MCP wire format**: tool names no longer carry the `gov-rt:` prefix. The 23 tools are now registered as bare `<verb>-<noun>` strings (`read-spec`, `read-tasks`, `mark-task`, …) — the same names already used by the `gvrn <subcommand>` CLI surface, so the binary's two surfaces finally agree on identifiers. Server-level namespacing is supplied by the adopter's `.mcp.json` server registration. The canonical server name is **`gvrn`** (was conceptually `gov-rt`), aligning the MCP server name with the binary/crate name. Resulting per-host wire identifiers:
  - Claude Code: `mcp__gvrn__<verb>-<noun>`
  - Auggie: `mcp:gvrn:<verb>-<noun>`

  **Adopter impact**: adopters who previously registered the runtime under the name `gov-rt` in `.mcp.json` must rename it to `gvrn`. Adopters who hand-authored permissions entries referencing `mcp__gov-rt__<tool>` or `mcp:gov-rt:<tool>` must update those entries to use `gvrn`. `framework/bootstrap/configure/{claude,auggie}.md` and the generated `.claude/commands/gov/configure.md` carry the new identifiers; re-running `/gov:configure` after a framework update is sufficient to refresh permission lists. No CLI-level changes — `gvrn <subcommand>` invocations are unchanged.

  **Why now**: the `gov-rt:` namespace was chosen in spec 022 to disambiguate tool names from `/gov:` slash commands at a time when the tool name itself carried the prefix (and a colon, which is not a valid identifier character in Claude Code MCP tool names). Switching to bare names removed the colon; the remaining `gov-rt` token then existed only at the server-name boundary, where it duplicated the `gvrn` binary/crate identity without adding meaning.

### Changed

- `scripts/gen-configure-mcp.sh`: trap-based tempfile cleanup so any early-exit path (set -e, splice failure, signal) releases the per-host block tempfiles instead of leaking them into `$TMPDIR`. Unused `label` parameter dropped from `process()`. SHOULD-tier findings from `/gov:review --fix`.
- `scripts/lint-tool-coverage.sh`: tool references inside a command file's `## Markdown-only reference` section are now skipped — that section *is* the fallback path, so references there do not require a paired fallback marker. Whitespace-strip on manifest lines tightened from "one leading/trailing space" to "any run of `[[:space:]]`". `|| true` added to the section-header lookup so `set -euo pipefail` does not abort when a command file has no markdown-only-reference section. SHOULD-tier findings from `/gov:review --fix`.

## [0.4.1] — 2026-05-16

### Changed

- `create-scenario` and `append-task` now validate caller-supplied path components before any filesystem operation, addressing the four SHOULD findings from `/gov:review` against scenario `022.ask-consolidation`:
  - **BE-INPUT-004 defense-in-depth** — new `validate_slug` and `validate_no_traversal` helpers in `primitives/mod.rs` reject slugs containing path separators or leading dots and reject `feature_path` values that are absolute or contain `..` components. New `PrimitiveError::InvalidSlug { slug, reason }` and `PrimitiveError::InvalidPath { path, reason }` variants surface the rejections as clean operational errors. Defense-in-depth: the existing `is_dir` checks remain, but the new validators close the rule's letter (canonical-path + base-dir check) as well as its spirit.
  - **REUSE** — new shared `iter_numbered_headings(content)` helper in `primitives/mod.rs` yields ATX-2 numbered headings while skipping fenced code blocks. `append-task`'s `next_task_number` is now a one-line `iter_numbered_headings(content).max().unwrap_or(0) + 1`, dropping ~30 lines of duplicate parsing. Available to future primitives that walk `tasks.md` headings.
  - **QUALITY** — `append-task`'s newly-created `tasks.md` now emits `Tasks. Complete in order.` (unlinked) when no `plan.md` exists at the time of creation, and the original `Tasks derived from the [plan](plan.md). Complete in order.` (linked) when `plan.md` is present. Closes the dangling-link case that markdownlint MD051 would flag.
- 19 new unit tests cover the validators, the shared heading-iterator helper, and the conditional intro behavior. Total lib tests grow 203 → 222; full suite 256 passing.

## [0.4.0] — 2026-05-16

### Added

- Two primitives for the `/ask` scenario branch introduced in spec 023, landing via scenario `022.ask-consolidation`:
  - `create-scenario` — write a `scenarios/{slug}.md` file under a feature with `section` frontmatter and Context / Behavior / (optional) Edge Cases body sections. Atomic via tempfile-in-parent + `persist` rename. Creates the `scenarios/` subdirectory if absent. Refuses with `ScenarioConflict` when the destination already exists; refuses with `FeaturePathNotFound` when the feature directory is missing.
  - `append-task` — append a numbered `## N. <title>` block to a feature's `tasks.md`, computing the next number as `max(existing) + 1` so a tasks file with gaps doesn't overwrite existing entries. Creates `tasks.md` with a heading derived from the feature's spec H1 (or a minimal `# Tasks` fallback when the spec is unreadable). Atomic via tempfile-in-parent + `persist` rename. Skips numeric headings inside fenced code blocks.
- New MCP tool names: `gov-rt:create-scenario`, `gov-rt:append-task`. Tool list grows from 21 to 23 entries; both `framework/runtime-tools.txt` and `mcp::server::TOOL_NAMES` carry them.
- New CLI subcommands: `gvrn create-scenario` and `gvrn append-task` (clap-derive args; same JSON envelope on stdout as other write primitives).
- New `PrimitiveError` variants: `ScenarioConflict { path }` and `FeaturePathNotFound { path }`.

## [0.3.1] — 2026-05-12

### Changed

- `enforce-manifest`'s glob compiler now delegates per-character escaping to `regex::escape` (already a transitive dependency via `regex`) instead of maintaining a hand-written metacharacter table. Internal refactor only; the glob → regex translation is byte-for-byte identical, all 14 `enforce_manifest::tests` still pass unchanged (including the metacharacter and bracket-literal coverage). Surfaced by `/gov:review`'s simplicity pass against 022-deterministic-runtime scenario `apply-manifest`.

## [0.3.0] — 2026-05-12

### Added

- Three primitives for strategy-aware bulk install + cleanup (scenario `022.apply-manifest`):
  - `apply-manifest` — strategy-aware bulk substitute + write driven by a typed manifest. Each `ManifestEntry { source, dest, strategy, keep-literals }` records an outcome (`created` / `updated` / `unchanged` / `skipped-exists` / `skipped-pinned` / `source-missing`); aggregate counts roll up across entries. Strategies: `update` (substitute, write only on diff, preserve mtime when unchanged), `create` (substitute, write only when dest absent), `skip-if-conflict` (write verbatim without substitution, only when dest absent). `pinned` short-circuits before any read/write — adopter customizations are never touched. `keep-literals` per entry masks named substitution keys, used by the `govern.md` self-install to keep `{project}` / `{cli-config-dir}` literal for the next adopter's bootstrap.
  - `enforce-manifest` — directory cleanup against an expected file list. Removes files matching `glob-include` (default `*.md`) whose relative path is neither expected nor pinned. `recursive: false` (default) is top-level only; `recursive: true` descends. One call replaces the bootstrap's three legacy cleanup loops (slash-command manifest enforcement, legacy `skills/` removal, legacy workflow filename removal).
  - `merge-managed-block` — generalization of `merge-claude-md` to support configurable marker shapes. `marker-style: "html-comment"` (default) reproduces `merge-claude-md`'s exact behavior; `marker-style: "line-prefix"` uses a `# {marker}` preamble line followed by the block, terminated by a blank line or EOF — matching `.gitignore` and `.gitattributes` conventions.
- New MCP tool names: `gov-rt:apply-manifest`, `gov-rt:enforce-manifest`, `gov-rt:merge-managed-block`. Tool list grows from 14 to 17 entries; both `framework/runtime-tools.txt` and `mcp::server::TOOL_NAMES` carry them, and the MCP integration test exercises every new tool.
- `framework/bootstrap/govern.md` Instructions section rewritten to drive the bootstrap through six primitive calls (`fetch-archive` → `extract-archive` → `apply-manifest` → `merge-managed-block` for `.gitignore` → `enforce-manifest` → `apply-manifest` with `keep-literals` for the `govern.md` self-install) plus a gate-confirm and two prose steps. No host-generated bash walker required; no `python3 -c '...'` substitution fallback; no per-file Edit calls from the host.
- `govern-basic` parity fixture extended to exercise every new strategy + marker style + cleanup path end-to-end. New companion test `govern_basic_post_run_filesystem_state_matches_expectations` walks the post-run target tree and asserts the per-primitive on-disk effects (substitutions applied where expected and NOT where suppressed, pinned file preserved verbatim, keep-literals placeholders kept literal, line-prefix `.gitignore` created, legacy file pruned).

### Changed

- `merge-claude-md` is now a thin compat shim that delegates to `merge-managed-block` with `marker-style: "html-comment"` and `marker: "govern-managed"`. The primitive's public API (`MergeClaudeMdArgs`, `MergeClaudeMdResult`) is unchanged, so existing callers — the bootstrap fixture, parity goldens, and any host scripts — keep working byte-for-byte. Slated for removal in the next major release of `gvrn`.

## [0.2.1] — 2026-05-12

### Changed

- **BREAKING** — `fetch-archive` argument `sha256_url` becomes `Option<String>`. Callers that omit the field download without sidecar verification; the primitive returns the computed sha256 digest and `verified: false` so the host can compare against a known-good value out-of-band. Callers that supply the field keep the verified path verbatim. The result struct grows a `verified: bool` field that reports which path the call took. Motivation: the `/govern` bootstrap operates live-on-main and fetches GitHub's auto-generated source tarballs (`/archive/refs/heads/main.tar.gz`), which ship without sidecars; before 0.2.1 the runtime couldn't drive that fetch and `/govern` always fell back to the markdown-only path.

### Updated

- `framework/bootstrap/govern.md`: step 2 prose acknowledges the sidecar-optional behavior and documents what `verified: false` means for callers that care about integrity.

## [0.2.0] — 2026-05-12

### Added

- Four primitives for the bootstrap procedure (scenario `022.govern-bootstrap`):
  - `fetch-archive` — download an archive plus its sha256 sidecar via reqwest's blocking client and verify the hash before persisting. Adds `reqwest` (blocking, rustls-tls) and `sha2` deps; a 256 MiB per-fetch ceiling caps memory defensively.
  - `extract-archive` — extract `.tar.gz`/`.tgz`/`.zip` in-process (no shell-out) via `flate2` + `tar` and the `zip` crate. Path-traversal protection rejects absolute paths and `..` components before writing.
  - `substitute-templates` — walk a source tree, apply `{key}` → value replacements to text files, copy binary files unchanged, write to a destination tree. Args use `source-dir` / `target-dir` (distinct from extract-archive's `dest` so both primitives can share a single procedure context).
  - `merge-claude-md` — idempotent BEGIN/END marker insert/update for a framework-managed block in CLAUDE.md. Four actions: created / inserted / updated / unchanged; unchanged preserves mtime.
- `gvrn exec` command resolution now considers `framework/bootstrap/<name>.md` as a third candidate after the existing two paths, so the `/govern` bootstrap procedure runs through the runtime.
- `framework/bootstrap/govern.md` gains a parseable `## Instructions` section that exercises the four new primitives plus a gate-confirm for the install approval; the existing 788-line procedure stays in place as the markdown-only reference.

### Changed

- **BREAKING** — package, binary, and library all renamed `runtime` / `govern_runtime` / `govern-runtime` → `gvrn`. Release tag pattern becomes `gvrn-v*` (was `runtime-v*`); release artifacts become `gvrn-<TARGET>.tar.gz` (was `runtime-<TARGET>.tar.gz`).
- **BREAKING** — `substitute-templates` argument names `source` / `dest` → `source-dir` / `target-dir` to avoid clashing with `extract-archive`'s `dest` in shared procedure context.

## [0.1.0] — 2026-05-12

### Added

- Crate scaffold under `runtime/`: `Cargo.toml`, binary entrypoint at `src/main.rs`, library root at `src/lib.rs`, module placeholders for `parser`, `interpreter`, `primitives`, `mcp`, `schema`, and `io`.
- Lint configuration in `Cargo.toml`: `unsafe_code = "forbid"`, `missing_docs = "warn"`, clippy `all` + `pedantic` at warn, plus `unwrap_used` and `expect_used` at warn.
- Pre-commit verification (`.githooks/pre-commit`): when staged changes touch `runtime/`, the hook runs `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`. Set `GOVERN_SKIP_RUNTIME_CHECKS=1` to bypass for a single commit.
- `runtime/bacon.toml` — `bacon` job definitions (`check`, `clippy`, `test`, `fmt`) with `clippy` as the default. Install with `cargo install --locked bacon`.
