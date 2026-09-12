---
status: in-progress
dependencies: [030-cross-service-references, 040-configurable-specs-dir, 050-constitution]
review:
  last-run: 2026-09-11T16:12:09Z
  reviewed-against: bc216c85218a30650828175a80063a7ddd83359a
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  reviewed-digest:
    data-model.md: 5c429d0d761de4d5bd5dd49802e582d2ed47ca617843d725e68c9f040e085938
  blocking: false
analyze:
  last-run: 2026-09-11T16:14:35Z
  analyzed-against: a482fe4fe91857c710cecb085504e11f5b9aa7d5
  hard-fail: 0
  blocking-findings: 0
  advisory: 1
  unexamined: 0
  analyzed-digest:
    data-model.md: 5c429d0d761de4d5bd5dd49802e582d2ed47ca617843d725e68c9f040e085938
    plan.md: 9f0c139487d77b5df04038ab22ac8eabe5ad41e5fbf84e565975bef59e9b086b
    review.md: 0931c1367356d781171984b647be27a7ac5df5bd3066e1aeea39e432850fce87
    spec.md: 7a91b500e05939b7673dfb47454b09c18b40f7c7c219a31ce95f9eecbf8d1277
    tasks.md: 327eac576a24f5881d2b2484fe04fccc971671c9de7b6dacdc01afff1d06004e
  blocking: false
next-criterion: 13
---

# 055 — Shared constitution

An organization running ductus in more than one repository has no way to state a rule once and have every project receive it. This feature adds a `.ductus/config.toml` entry naming a **shared constitution** — a governance source the organization owns, distinct from the framework constitution ductus ships — and puts the rules it establishes in effect alongside the shipped ones. Unset, it changes nothing.

## Motivation

`.ductus/constitution.md` had exactly one source: the `framework/constitution.md` → `.ductus/constitution.md` row of the **Shared Files** manifest, delivered by `/ductus` with strategy `update`. That covered rules true for *every* ductus project, which is what [050-constitution](../050-constitution/spec.md) governs. Nothing covered rules true for *one organization's* projects and no one else's — a house code style, a deployment gate, a review convention, a naming standard.

An organization had two ways to carry such a rule, and both were bad:

- **Repeat it per repository.** Write it into each project's `AGENTS.md`. That file is project-owned and ships to nobody, so the rule was retyped once per repository and drifted independently from then on. Nothing detected the divergence, because from inside any one repository there was nothing to diverge from.
- **Hand-edit the shipped constitution and pin it.** Add the rule to `.ductus/constitution.md` and list that path under `[pinned] files` so `/ductus` stopped overwriting it. This worked once and then froze the file: a pinned constitution receives no framework updates at all, so the price of one house rule was every future framework rule.

The framework's rule tier has a structurally similar gap one level down — a rule file a project authors for itself registers by being placed in the rule-file directory, and is therefore discovered only in the repository that holds it. That gap is real and is **not** what this spec closes; see Non-Goals.

The result was that the same guidance was typed once per project and the copies drifted — the outcome §drift-prevention exists to prevent, reproduced one level above the one the framework governs.

## Shape of the setting

Presence of the entry enables the feature; absence is today's behavior exactly. The entry's shape is settled under Resolved Questions — an alias-keyed `[constitutions.<alias>]` table naming a `repo` (identity only, never fetched) and a local `path` (the only state read).

- **Unset is unchanged.** With no entry, no source is resolved, no filesystem or network access is attempted, and every command and the runtime behave as they do today. This is the posture [040-configurable-specs-dir](../040-configurable-specs-dir/spec.md) established for `[paths] specs-root`: one default, one explicit override, nothing derived.
- **Additive, never a replacement.** A project that declares a shared constitution still receives `framework/constitution.md` through the manifest on every `/ductus` run. Declaring one must not require pinning `.ductus/constitution.md` — pinning is exactly the workaround that already existed and is what made it unusable.
- **In effect, not merely referenced.** The rules the shared source establishes bind the commands that read the constitution, the same way the shipped ones do. A pointer a contributor is expected to go and read is the diligence dependency §design-principles rejects.
- **The source is attributable.** A contributor reading a rule can tell which source it came from, and a run says which sources it loaded. Two governance documents in effect with no way to tell them apart is worse than one.

### Edge cases

- **Table present but empty.** Indistinguishable from absent: nothing is resolved, nothing is reported.
- **`path` does not resolve.** The `not-checked-out` state — warned, reported as unexamined, never a config error.
- **`path` resolves but holds no `constitution.md`.** A distinct reason from a missing checkout, and reported as
  such: an operator who cloned the wrong repository must not read the same message as one who cloned nothing.
- **Two entries naming the same `path`.** Warned and allowed, matching `/ductus:link`'s duplicate-repo posture; the
  document is loaded once.
- **Registration is not transitive.** A registered checkout's own `.ductus/config.toml` is never read, so a shared
  constitution cannot pull in further constitutions. There is no recursion to bound, no cycle to detect, and the set
  of documents in effect is exactly what the project's own config names.

## Non-Goals

- **Shared rule files.** The rule tier has its own distribution gap (see Motivation), and closing it is not this
  spec's job — this spec shares the constitution *document*. A later spec may extend the same `[constitutions.*]`
  registry to a `rules/` directory in the checkout; nothing decided here forecloses that, and the origin-neutrality
  §rules already requires would carry most of it. It is deliberately not bundled: the request was about the
  constitution.
- **Shared templates.** Structural rather than normative, and not requested.
- **A shared `AGENTS.md`.** `AGENTS.md` is the project-only tier by 050's classification — see Resolved Questions.
- **Fetching anything.** A registered checkout is the operator's to obtain and update; ductus reads it and never
  acquires it.

## Acceptance Criteria

- [x] AC1: With no `[constitutions.*]` entry in `.ductus/config.toml` — the key absent, or the table present and empty — no source resolution is attempted, no new prompt fires, and every command and the runtime behave exactly as they do today.
- [x] AC2: With the entry present, the rules the shared source establishes are in effect for every command that reads the constitution, in the same run and with no per-command opt-in step.
- [x] AC3: Declaring a shared constitution does not require pinning `.ductus/constitution.md`; such a project still receives framework constitution updates on the next `/ductus` run.
- [x] AC4: A run that loads a shared constitution reports the sources it loaded, and any rule in effect is attributable to the source that established it.
- [x] AC5: A shared source that cannot be resolved is reported by name and reason, and is never silently treated as "no shared constitution declared".
- [x] AC6: A malformed entry value is rejected with a clear message at configuration time rather than silently accepted.
- [x] AC7: Which constitution documents a project loads is determined solely by its `[constitutions.*]` entries and the content of the checkouts they name — two projects with the same entries over the same checkout content load the same documents, in the same order, on any machine.
- [x] AC8: A run that could not load a registered source records it as unexamined and never folds it into a clean result; a `/ductus:review` or `/ductus:analyze` report produced without a registered constitution says so in the report itself.
- [x] AC9: The import of a registered constitution is maintained by `/ductus` inside a managed block; the rest of the adopter-owned file it lives in is preserved byte-for-byte, and `.ductus/constitution.md` is never rewritten.
- [x] AC10: The precedence order — framework as floor, and project > shared > framework where the framework is silent — is stated once in **the constitution**, together with the fact that nothing enforces it; every other mention points at it rather than restating it.
- [x] AC11: More than one `[constitutions.*]` entry may be registered, and every registered entry is loaded.
- [x] AC12: `resolve-constitutions` carries each entry's `description` onto its `ConstitutionRecord`, so it reaches the surfaces that name a source — `/{project}:target`'s loaded, skipped and duplicate-path reports, and `write-review`'s `## Unexamined governance` bullets — including on a **skipped** entry, where the value comes from the config rather than the checkout and is available precisely when the document is not. Internal whitespace is collapsed by the primitive and an over-long value is truncated by the renderer, so a multi-line description cannot break a single-line report. Absent stays absent: an entry without one renders exactly as it did before.

## Open Questions

<!-- All open questions resolved — see Resolved Questions below. -->

*None — all resolved.*

## Resolved Questions

- **Where does the precedence order live?** Resolved during implementation, reversing this spec's own AC10 as first written. AC10 said "the config-schema documentation", meaning `framework/bootstrap/ductus.md`. That file is the **installer**: it ships to nobody and no adopter reads it, so a normative rule about which governance binds would have been invisible to every project it governs. The canonical statement is therefore `framework/constitution.md` §governance-precedence — which does ship, to every adopter, as `.ductus/constitution.md` — and the config schema and `/ductus:target` step 4 both point at it rather than restating it. That is [050-constitution](../050-constitution/spec.md)'s promotion mechanism applied: canonical text in the constitution, a pointer that says nothing the constitution does not. It is also placed deliberately beside §grounding's *Sources, in order of authority*, because that section ranks **evidence** and the adjacent question — which governance document wins — is the one readers conflate with it.
- **Who validates a shared constitution?** Resolved: **nothing validates it, and that is stated rather than implied.** With rule files out of scope there is no structural schema to check — a shared constitution is prose, and the framework's own constitution prose is unvalidated too: `/ductus:audit` is maintainer-only and checks *this* repository's cross-artifact drift, while adopters get markdownlint and nothing more. Adding a validation pass for shared prose would hold an organization's document to a standard the framework does not meet itself. The consuming project reads the document; whether it is any good is the publisher's problem, exactly as the framework constitution's quality is this repository's. Recorded explicitly because "the org's constitution is checked somehow" is the kind of assumption that becomes a support question.
- **If shared rule files are in scope, how do their IDs behave?** Resolved: **not applicable — rule files are out of scope** (see Non-Goals). The question was live only while scope included them, and is recorded rather than deleted because it is the first thing a later spec extending `[constitutions.*]` to rule files will have to answer. What was established before the narrowing, and still holds as input to that spec: the ID grammar is `{surface}-{category}-{NNN}` with an **extensible** surface element — `quality-cross.md` registers its own `QUAL` surface in its own header — so an organization can namespace its rules by registering a surface token the same way, and cross-file duplicate IDs are unaddressed by the current schema (008 governs duplicates *within* a file only).
- **What happens when the source is unreachable?** Resolved: **warn, proceed, and report the source as unexamined.** There is no outage to handle — ductus never fetches, so "unreachable" collapses to the local `path` not resolving, which is the `not-checked-out` state `[services]` already defines as "surfaced at resolution time, not a config error". Ductus warns and the run continues. The addition over `[services]` is that the stakes differ: a missing service checkout only downgrades a cross-service link to plain navigation, whereas a missing constitution checkout means every command that reads governance runs under fewer rules than the config declares. So a run reports the source it could not load **as unexamined, never folded into a clean result** — the existing discipline of `QUAL-CLAIM-001` (*examined and found nothing* is not *could not examine*) and §grounding's *a partial read is not a read*, applied to a new source. No new gate. Rejected blocking: it would make the pipeline unrunnable for a teammate who has not cloned the governance repository, which is the hard-dependency trade already declined over pinning.
- **Is the composed result materialized on disk or resolved at read time?** Resolved: **nothing is materialized.** There is no composed result to write — the document is imported from the registered checkout in place (Q3), never copied into the repository. No copy means no second copy to drift, and no collision with the manifest's `update` strategy or with `[pinned]`. The repo-visible record is the committed `.ductus/config.toml` entry plus AC4's report of the sources a run loaded: *what is in effect* is declared in the repository even though the text is not. The import line itself has an existing non-destructive mechanism — `merge-managed-block` already maintains framework-owned regions inside adopter-owned files, preserving the rest byte-for-byte — so `/ductus` maintains the import rather than the adopter hand-editing it.
- **Who wins a conflict, and can a project opt out of a single shared rule?** Resolved, in two halves. **Precedence: the framework constitution is a floor.** A shared or project source may add rules and tighten existing ones, never loosen them; where the framework is silent, the more specific source wins (project > shared > framework). The framework constitution is what every ductus command reads to know how to behave, so a shared source able to disable a gate would make the pipeline's own invariants negotiable per repository. Note that the constitution's §grounding *Sources, in order of authority* ranks **evidence** (code > live state > inference) and says nothing about governance documents — this order is established here, not inherited. **Nothing mechanizes it**: prose precedence is a rule for whoever resolves a contradiction, not a check, and that is recorded rather than implied. **Opt-out: there is none at rule granularity.** A shared constitution is a document, imported whole (Q3); prose carries no per-statement handle to disable, and inventing one would mean a project could silently neuter a rule its config still advertises. A project that will not accept a rule stops registering that constitution, or takes it up with the publisher. All-or-nothing granularity is the accepted cost of sharing prose, stated rather than implied.
- **What travels — prose, rules, or both?** Resolved: **prose only** — the `constitution.md` document in the registered checkout, and nothing else. This reverses an earlier answer in this same clarification, which had extended scope to rule files on the argument that the Motivation named a gap at that tier too; the requester narrowed it back, on the grounds that the request was about the constitution. The rule-tier gap is real and is recorded under Non-Goals rather than solved here. Narrowing also removes two questions this spec would otherwise have had to answer — rule-file ID collision and the agent-native rule mirror — neither of which the request asked about.
- **Does this extend to `AGENTS.md`?** Resolved: **out of scope**, on [050-constitution](../050-constitution/spec.md)'s reasoning rather than as scope-trimming. 050's §Classification sorts every `AGENTS.md` entry into *universal* ("true for any project running the ductus pipeline. Promoted.") or *project-only* ("true because of something particular to this repository... Stays in `AGENTS.md`, unchanged"), and its §Promotion mechanism keeps exactly one normative statement per rule: the constitution carries it and `AGENTS.md` keeps a single pointer line, because "two copies of a rule is the drift §drift-prevention exists to prevent". `AGENTS.md` is therefore the project-only tier **by definition** — a convention true across five repositories is not project-only, so its home is the shared constitution this spec adds, with `AGENTS.md` keeping the pointer exactly as it does for promoted framework rules today. A shared-`AGENTS.md` channel would give the same content two homes and reopen the drift the promotion mechanism was built to close. Rejected a generalized "list of documents to import" entry for the same reason plus a second: an unstructured list has no layout to validate and no rule directory to discover.
- **Is the source pinned to a version?** Resolved: **no pin.** Ductus never fetches (Q1), so it cannot acquire a version and `/ductus` cannot report drift against upstream — it has no upstream access by construction. The checkout's own git state is the version, and keeping contributors in sync is the organization's job, exactly the posture `[services]` takes by recording no ref. **Accepted cost, stated rather than implied:** contributor skew stays silent. One contributor pulls the governance repository and another does not, and the two run under different rules with nothing detecting it. Rejected an optional compared-and-warned `ref` field, which would have caught exactly that at the price of a field and a git read. **AC7 was reworded as a direct consequence** — it previously said "at the same version", which names nothing once no version is recorded; it now states the determinism requirement over config entries and checkout content instead.
- **How does shared content compose with the shipped constitution?** Resolved: **separate documents, never merged.** An adopter's constitution is not assembled — it is imported, one line per governance document (`framework/templates/project/claude-md.md` imports `.ductus/constitution.md` and `AGENTS.md`), so a registered shared constitution is one more import alongside them. Two consequences follow. Each document owns its own anchors, so a shared source cannot redefine what `§grounding` resolves to — it can only add its own sections; the anchor hazard disappears rather than needing a rule. And `.ductus/constitution.md` stays byte-identical to what the manifest ships, which is what AC3 requires. Rejected merge-at-`/ductus`-time: it makes the shipped constitution a generated artifact, collides with the manifest's `update` strategy, and creates the second copy §drift-prevention exists to prevent. **Accepted limitation:** a shared constitution can add to the framework's rules but cannot contradict them — see the precedence question below.
- **What does the entry point at?** Resolved: it mirrors the `[services]` registry that [030-cross-service-references](../030-cross-service-references/spec.md) established and `/ductus:link` writes. An alias-keyed `[constitutions.<alias>]` table names a canonical `repo` URL and a local `path`. The `repo` is **identity and navigation only — recorded verbatim and never fetched**; the local `path` is the only state read, never the network. A `path` that does not resolve is the `not-checked-out` state — warned and surfaced at resolution time, never a config error. Many entries are allowed, so an organization-level and a team-level source can layer. Rejected the archive-at-a-ref form (spec 015's codeload transport): it would add a transport, an authentication story, and a network failure mode to a read ductus already performs off local disk everywhere else. The table is named `[constitutions.*]` because `[services]` is taken and the name states what it holds.
- **Is `./ductus/config.toml` in the logged request the existing `.ductus/config.toml`?** Resolved 2026-09-10 by the requester: `./ductus` was a typo in the logged item. The entry lives in the existing `.ductus/config.toml`, alongside `[paths]`, `[rules]`, and `[review]`. This feature introduces no second config file and no non-dotted directory.

## See also

- [019-config-decisions](../019-config-decisions/spec.md) — established the project config as a persisted-decisions store rather than a pin-only file; this entry is another decision recorded there.
- [033-rule-surface-setting](../033-rule-surface-setting/spec.md) — the `[rules] surfaces` operator setting that filters which shipped rule files an adopter receives; a close structural model for an operator entry that changes which governance a project loads.
