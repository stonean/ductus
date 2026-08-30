---
description: Create a new feature spec.
argument-hint: "[--branch] [--branch-id <identifier>] [--fold-into <feature>] [--supersedes <feature>] [feature description]"
parity:
  strict-fields:
    - frontmatter
  strict-files:
    - "specs/{NNN-feature}/spec.md"
  semantic-fields:
    - spec-body
---

# Specify

Create a new feature spec.

## Purpose

First step in the pipeline. Creates a new numbered feature directory with a spec from template and sets it as the session target. Accepts both greenfield input (rich description with concrete acceptance criteria) and brownfield input (sparse description of an existing feature) — richness scales with the input. Sparse acceptance criteria are valid for brownfield use; the spec gains precision through subsequent bug fixes, scenarios, and clarifications.

## Context

This command does not require a session target — it creates a new feature. If `.ductus/session.toml` exists, the session target will be overwritten with the new feature.

If the constitution has not been loaded in this session (e.g., `/ductus:target` has not been run), read `.ductus/constitution.md` now to load `ductus` rules. If the constitution was already loaded by `/ductus:target`, do not re-read it.

### Flags

| Flag | Behavior |
| --- | --- |
| *(none)* | Sequential numbering — the next `NNN-` in the global sequence. This is the default and nothing persisted changes it. |
| `--branch` | Create a branch-scoped spec, prompting for the identifier (a candidate is proposed from the current git branch name when one can be extracted) |
| `--branch-id <identifier>` | Create a branch-scoped spec under this identifier, supplied directly. An opaque token, not a number: it is sanitized to `^[a-z0-9]+(?:-[a-z0-9]+)*$`, never refused |
| `--fold-into <feature>` | The upstream sequential spec the branch-scoped spec folds back into, written to its `folds-into:` frontmatter. Required with the two flags above and meaningless without them |
| `--supersedes <feature>` | The spec this new one counters. Writes `supersedes:` into the new spec's frontmatter and a reciprocal annotation onto the named spec. Repeatable — `supersedes:` is a list, because one spec routinely counters several in a single change. Independent of the three flags above and usable without any of them |

## Scope Boundaries

- This command creates spec artifacts only. Do NOT read or write source code, test files, or implementation files.
- With `--supersedes`, the run writes to a **second** spec — the reciprocal annotation on the one being superseded. That is the only case where this command touches a spec it did not create, and it is a **mechanical edit**: the annotated spec keeps whatever status it holds and takes no back-edge, because the annotation changes no claim that spec makes about what it delivered (§spec-lifecycle).
- Read only what is needed: existing spec directory names (for numbering and routing), each candidate's `plan.md` Affected Files and frontmatter `status`, the rule-file directory listing, the spec template, and — only when branch-scoped creation is requested — the current git branch name, to propose an identifier candidate. Do NOT read other specs' bodies unless checking for naming conflicts.
- The routing gate (step 1–3) writes nothing: it derives candidates, presents them, and confirms. A denial ends the run with no directory created and no session write.
- Reference: §spec-phase, §spec-requirements, §numbering (both directory forms, and why the branch-scoped one is temporary), §spec-lifecycle (a branch-scoped spec is retired by fold-back, never completed), §text-first-artifacts, §brownfield-process, §bug-handling (the routing tree, canonical in `groom.md`).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write).

1. Invoke `derive-routing-candidates` with the feature description from `$ARGUMENTS` (required — if empty, ask the user what feature to specify), plus `routed-by` when another command's routing tree already chose this route (`/ductus:groom` recommending `/ductus:specify` is the case that matters). It derives the homes this work could already have — the runtime-owning spec when the description names a primitive or a `runtime/` path, a rule surface whose category it shares, a spec whose subject it overlaps — and reports every source as examined or skipped. A `gate-required: false` result means the decision has been made already: skip steps 2 and 3 and continue at step 4.

2. <!-- llm:routeInboxItem --> Walk the **same** decision tree `/ductus:groom` walks (the **Groom decision tree** reference in `groom.md` is its single canonical statement — do not restate it here), over the feature description in place of an inbox item, with step 1's candidates as evidence. Return the route and, when it targets an existing spec, the feature. `spec` — no existing home covers this — is the route that proceeds to create one.

3. Invoke `gate-confirm` with a `gate` name (e.g. `specify-route`) and a `prompt` naming the chosen route and the alternatives, so the operator confirms the routing **before** anything is scaffolded. When a candidate spec is `done`, the prompt names the `done → in-progress` back-edge accepting it implies, exactly as groom's per-item confirmation does. When the derivation reported no candidates and no skipped sources, say so and proceed — a fresh adopter sees one line, not a decision. When any source was **skipped**, report *could not derive candidates* and name the source: that is a different answer from *none found*, and stating it as the latter would claim a check ran that did not. Denial ends the run cleanly with nothing written, which is how the operator takes a candidate instead; the framework reports and does not veto, so creating a new spec over any candidate stays one keystroke away. A candidate admits a third answer beside *amend it* and *unrelated*: **this new spec supersedes it**, which continues the run at step 4 exactly as though `--supersedes {candidate}` had been passed. Offering it here is what keeps it from being a diligence dependency: the candidate surfaces anyway, so forgetting the flag costs nothing, and this is the one moment the relation is cheap — nothing recovers it afterward. The classification is not exhaustive, because the derivation matches over slug vocabulary and a predecessor sharing none of it surfaces as no candidate at all; the flag therefore stays available directly.

<!-- audit:ignore-promotion -->
4. Settle branch-scoped creation before anything is scaffolded (host responsibility; no primitive — the decision is a conversation with the operator, and the sanitization it previews belongs to create-feature in step 6). **Sequential numbering is the default and this step decides nothing without a request for the other form**: with neither `--branch` nor `--branch-id` present, continue at step 5 with the arguments unchanged and settle nothing here — that is the path §numbering calls the default, and the one an adopter who never asks for a branch-scoped spec must keep observing. Otherwise settle three things, in this order:

   - **The identifier.** `--branch-id <identifier>` supplies it directly; `--branch` requests branch-scoped creation without one, in which case prompt for it. Offer a candidate read from the current git branch name (`git rev-parse --abbrev-ref HEAD`) when a tracker-style token can be extracted from it, and prompt with **no** candidate when it cannot — a branch named `main` or `wip` yields nothing to propose, and inventing one would be worse than asking. The framework parses no branch-name grammar, so a candidate is a convenience the operator confirms or corrects, never an authority; the value they settle on is the value used.
   - **What the identifier will become.** A non-conforming identifier is sanitized rather than refused — lowercased, with every run of non-alphanumerics collapsed to a single hyphen — so `PROJ-1111` becomes `proj-1111`, `1111-PROJ` becomes `1111-proj`, and a `.` becomes a hyphen, which is what leaves the `{branch-id}.{n}` delimiter unambiguous. Name the sanitized form in the prompt, **before any directory exists**: a transformation the operator never sees is one they discover from a directory name later. Step 6 reports the identifier actually used, and that one is authoritative — say so if it differs from what was previewed here.
   - **The fold target, which is required.** `--fold-into <feature>` supplies it; prompt when it is absent. There is no path that creates a branch-scoped spec naming no upstream home: the number is what keeps the merge clean, and the target is what makes the spec actionable once it lands, so a staging spec with nowhere to land is not a case this framework has (§numbering, §spec-lifecycle). The target names a **sequential** spec and need not exist in this tree — a branch-scoped spec exists *because* upstream diverged, so its home routinely lives on the branch this one forked from, and existence is enforced at fold-back rather than here. An operator who will not name one is asking for a sequential spec: offer that, and end the run with nothing created if they decline both.

   **On `ductus exec`** there is no operator to prompt, so this step reduces to passing through whatever `--branch-id` and `--fold-into` the invocation already supplied; branch-scoped creation without them is refused by the primitive in step 6 rather than prompted for. The reduction is documented rather than silent (§runtime-host-integration's two-paths guarantee).

<!-- audit:ignore-promotion -->
5. Settle the supersession declaration, before anything is scaffolded (host responsibility; no primitive dispatches here — the decision is a conversation with the operator, and the writes it authorizes belong to step 8). **With neither `--supersedes` nor a supersession classification from step 3, this step decides nothing**: continue at step 6 with the arguments unchanged, and creation behaves exactly as it does without this feature. Otherwise settle two things, in this order:

   - **Which specs are named.** Resolve each `--supersedes` argument to a feature directory with resolve-feature; `ambiguous` and `not-found` are domain outcomes to surface, and a name resolving to nothing is the one refusal on this path. The frontmatter schema validates `supersedes:` for **shape** only, so resolvability is enforced by the declaring command — this being the first moment both specs exist.
   - **Whether each is `done`.** Read each resolved target's `status` with read-spec. A target that is not `done` is **accepted, never refused**: say that supersession's justification is a spec that shipped and was later countered, that a `draft` or `clarified` spec delivered nothing to counter — so annotating it would record a decision as enacted-then-undone when it was never enacted — and that `/ductus:consolidate` is the likelier outcome. Then proceed if the operator still wants the declaration. `in-progress` is genuinely ambiguous and the operator may hold context the status does not carry, so this reports and does not veto, exactly as step 1's derivation does.

   **On `ductus exec`** there is no operator to classify a candidate or answer the non-`done` guidance, so this step and step 8 reduce to the no-supersession line and creation proceeds unchanged — the declaration is an interactive surface, and a run that wrote one unattended would record a relation nobody declared. The reduction is documented rather than silent (§runtime-host-integration's two-paths guarantee).

6. Invoke `create-feature` with the feature description from `$ARGUMENTS` as the title, plus the `branch-id` and `fold-into` arguments step 4 settled when it settled any. The primitive computes the next feature number from the existing NNN-prefixed directories under the configured specs root, derives the kebab-case slug, creates `specs/{NNN-slug}/`, and copies the spec template into it atomically (mode-preserving); it resolves the template from `{specs-root}/templates/spec.md` and falls back to the framework source layout `framework/templates/spec/spec.md` (the ductus repo's own layout). With `branch-id` supplied it instead sanitizes that identifier, numbers `{identifier}.{n}-{slug}` from the directories already under that identifier alone, stamps `folds-into:` into the new spec's frontmatter, and returns the **sanitized** `identifier` it used — report that value, since it is what the operator actually got. An already-existing target directory is the `created: false` domain outcome — report the collision and stop rather than overwrite; under one identifier that is also what makes two contributors creating at the same moment safe, the loser being refused rather than overwriting the winner.

7. <!-- llm:writeSpecBody --> Fill the new spec body following §spec-requirements: a Motivation section, Acceptance Criteria with concrete and testable checkboxes (sparse acceptance criteria are valid for brownfield use — leave the section with a comment noting criteria will emerge from real work), Open Questions, and any inline links to other specs that `derive-dependencies` will derive the frontmatter dependencies from. The host returns the markdown body for the new file; the walker forwards the response through the context. **When step 5 settled a supersession, cite the superseded spec by name and write no markdown link to it.** The pointer is frontmatter; a body link to a sibling spec is harvested by `derive-dependencies` into `dependencies:`, which would make the superseding spec declare a dependency on the very spec it counters — silently, on the first commit through the pre-commit hook. The link belongs to the reciprocal annotation written in step 8, which carries it from inside a blockquote, the one construct `derive-dependencies` exempts.

<!-- audit:ignore-promotion -->
8. Write the declaration step 5 settled — the frontmatter key and its reciprocal annotation (host responsibility; conditional on step 5, so nothing dispatches when no supersession was settled). Stamp `supersedes:` into the new spec's frontmatter as a list naming each resolved feature: it is **hand-authored**, deliberately unlike the generated `dependencies:` and `references:` indexes beside it, which no command writes by hand (§text-first-artifacts, Frontmatter Schema). Then, once per named spec, write the reciprocal annotation onto it with write-supersession-annotation, passing the superseded feature, this new feature as the superseding one, and the **substance** — what no longer holds — as the authored payload. The primitive contributes the frame (placement, the blockquote wrapper, the `> **Sunset ({link}):**` citation, and the closer recording that the spec stays the record of what shipped) and never invents the substance: a generated banner can name the superseding spec and the date and nothing else, while the sentence a reader actually needs is the one naming what stopped being true. Phrase that sentence as a **non-claim** ("no longer exists", "is removed") rather than as an assertion whose truth depends on whether its paths resolve. An already-present result is a re-run converging rather than a failure — report it and continue. The operator's consent for this second-spec write is step 3's routing confirmation, or the flag they passed; no separate gate is added.

9. Invoke `label-criteria` against the new feature to assign a stable `AC{n}:` label to every criterion the step above wrote, and to record `next-criterion` in the frontmatter. The initial batch is labelled in the run that created it, so a criterion can be cited by label in the same conversation that authored it — that is the moment citation matters most. The pass is idempotent and writes nothing when the section is empty, so a brownfield spec with a placeholder comment and no criteria is unaffected. **Never derive the label in the LLM**: picking `max + 1` means tallying the list, which is exactly the counting this labelling exists to remove.

10. Invoke `lint-markdown` against the new spec file to surface any markdown violations the LLM may have introduced.

11. Invoke `gate-confirm` with a `gate` name (e.g. `specify-create`) and a `prompt` asking the user to approve creating the new feature and setting it as the session target before any session-file write. `gate-confirm` is non-blocking — it returns the prompt payload (`gate`, `prompt`, `request-id`) and the host routes the decision out-of-band. On confirmation, continue to the session write below; on denial, the walker exits cleanly without writing the session.

12. Invoke `write-session` with the new feature slug and its repo-relative spec directory — under the configured `[paths] specs-root` (default `specs`; spec 040) — as the feature and path arguments. This is a target write: the primitive stamps a fresh set-at while preserving any cli-config-dir already in the file (the per-contributor agent identity written by `/ductus`), at `.ductus/session.toml`, through tempfile + rename atomic-write semantics. On the markdown-only path, the host writes the file by hand per the markdown-only reference's Write the session target section — the cli-config-dir preservation rule there applies verbatim.

## Markdown-only reference

The full new-feature-creation procedure (directory creation, template copy, frontmatter conventions, session write, and next-step prompt) is documented below for the markdown-only path. The numbered steps above invoke the mechanical primitives plus the writeSpecBody extension that automate the deterministic phases.

### Route before scaffolding

Run this **before** the feature directory exists — creating a spec is the one
action the routing rules exist to prevent, so a gate after the write is no gate
at all.

Derive the homes this work could already have (primitive:
`derive-routing-candidates`; by hand, read the three sources directly — no
shell-pipeline substitution):

1. **Runtime work.** Does the description name a runtime primitive or a path
   under `runtime/`? If so, the home is the spec whose `plan.md` lists a
   `runtime/` path under `## Affected Files` — **derived from the corpus, not
   assumed.** Each project's runtime-owning spec is its own; a project with
   none yields no candidate rather than a wrong one. The route is a scenario on
   that spec, via the back-edge.
2. **Rule surfaces.** Does the description share vocabulary with a rule file's
   category stem (`security-backend.md` → `security`, with the
   `-backend`/`-frontend`/`-cross` surface suffix set aside)? A rule belonging
   to an existing surface is amended on that surface's home spec, not spawned
   as a spec of its own.
3. **Spec corpus.** Does it share vocabulary with an existing spec's slug? Then
   it may be a scenario on that spec.

Then walk the **Groom decision tree** — the one in `groom.md`, which is its
single canonical statement — over the description in place of an inbox item,
with those candidates as evidence, and confirm the result with the user before
anything is written:

- **Candidates found** — name them, name the route each implies, and let the
  operator pick: a scenario on `NNN`, a rule-file amendment, a new spec that
  **supersedes** `NNN`, or a new spec anyway. A new spec stays creatable over
  any candidate; this reports, it does not veto. The supersession answer is
  offered here rather than left to a flag the operator has to remember,
  because the candidate surfaces anyway and this is the one moment the
  relation is cheap to record — nothing recovers it afterward. It is not an
  exhaustive offer: the derivation matches over slug vocabulary, so a
  predecessor sharing none of it surfaces as no candidate, which is why
  `--supersedes` stays available directly. When a named candidate is `done`, say that accepting it reopens it
  `done → in-progress`, so the operator consents to the back-edge before it
  happens — the same wording groom's per-item confirmation uses.
- **No candidates, nothing skipped** — say so in one line and proceed. Every
  source ran and matched nothing, so a new spec is the right answer. A fresh
  adopter with no rule files and a single-spec corpus is exactly this case and
  must see no new friction.
- **A source could not be examined** — report *could not derive candidates* and
  name the source and reason. This is **not** the same answer as *none found*
  (`QUAL-CLAIM-001`): an unreadable rule directory reported as "no candidates"
  tells the operator a new spec is correct on the strength of a check that
  never ran.

Skip this section entirely when the routing tree has already run for this work
— `/ductus:groom` routing an item to `/ductus:specify` is the case — and
say why. Asking a question that has been answered is friction, not rigor.

### Resolve feature number and slug

1. `$ARGUMENTS` is the feature description (e.g., "webhook delivery"). This is required — if empty, ask the user what feature to specify.
2. Determine the next available feature number by checking existing directories under `specs/` matching the NNN-feature pattern; the next number is the highest existing NNN plus one (zero-padded to three digits).
3. Generate the slug from the feature description: lowercase, hyphenated, no whitespace, no punctuation beyond hyphens.

### Create the feature directory

1. Create `specs/{NNN-feature-name}/`.
2. Copy `specs/templates/spec.md` into the directory as `spec.md`.

Both sections above are what the `create-feature` primitive automates on the runtime path (number scan, slug derivation, directory creation, atomic template copy); walk them by hand only when no runtime is available.

### Settle branch-scoped creation

Only when the operator asked for it — `--branch`, or `--branch-id
<identifier>`. With neither, the two sections above are the whole story and
this one does not apply: sequential numbering is the default, and no
persisted setting exists that could change it.

1. **Identifier.** Take it from `--branch-id`, or prompt. When prompting,
   propose a candidate extracted from the current git branch name (`git
   rev-parse --abbrev-ref HEAD`) if a tracker-style token can be found in it,
   and prompt with no candidate when it cannot — the framework parses no
   branch-name grammar, so a proposal is a convenience and the operator's
   confirmed or corrected value is what is used.
2. **Preview the sanitized form** in the prompt, before any directory is
   created. The identifier passes through the same rule the slug does —
   lowercase, every run of non-alphanumerics collapsed to one hyphen — so
   `PROJ-1111` becomes `proj-1111` and any `.` becomes a hyphen, which is what
   keeps the `{branch-id}.{n}` delimiter unambiguous. A sanitization the
   operator never sees is one they discover from a directory name later.
   An identifier that sanitizes to nothing at all is refused before anything
   is created, exactly as an empty derived slug is.
3. **Fold target.** Take it from `--fold-into`, or prompt; it is required.
   It names a sequential spec (`NNN-slug`) and need not exist in this tree —
   the upstream home of a branch-scoped spec normally lives on the branch this
   one forked from. There is no way to create a branch-scoped spec that names
   none: an operator unwilling to name one is asking for a sequential spec.
4. **Name and stamp.** The directory is `{identifier}.{n}-{slug}`, where `{n}`
   is one past the highest already present *under that identifier alone*, and
   the new spec's frontmatter carries `folds-into: {feature}`. An existing
   target directory is reported and never overwritten, which is also what makes
   two contributors creating under one identifier at the same moment safe.

The branch-scoped directory is a staging form: it is discharged by fold-back
and never reaches `done` (§numbering, §spec-lifecycle).

### Declare a supersession

Only when the operator asked for it — `--supersedes <feature>`, or the
supersession answer to a routing candidate. With neither, this section does not
apply and creation is unchanged.

1. **Resolve each named spec** (primitive: `resolve-feature`), then read its
   `status` (primitive: `read-spec`). A name that resolves to nothing is the
   one refusal on this path: `supersedes:` is validated for *shape* only, and
   resolvability is enforced by the declaring command, at the first moment both
   specs exist.
2. **A target that is not `done` is accepted, never refused.** Supersession's
   justification is a spec that shipped and was later countered, which is the
   whole reason the superseded spec stays on disk; a `draft` or `clarified`
   spec delivered nothing to counter, so annotating it would record a decision
   as enacted-then-undone when it was never enacted. Say that, name
   `/ductus:consolidate` as the likelier outcome, and proceed if the
   operator still wants the declaration. `in-progress` is genuinely ambiguous
   and the operator may hold context the status does not carry.
3. **Write the key.** `supersedes:` is a **list** of feature slugs in the new
   spec's frontmatter — one spec routinely counters several in a single
   change, which is where it diverges from the scalar `folds-into:`. It is
   hand-authored, unlike the generated `dependencies:` and `references:`
   indexes beside it (§text-first-artifacts, Frontmatter Schema).
4. **Write the reciprocal annotation** onto each superseded spec (primitive:
   `write-supersession-annotation`). The key on the new spec is bookkeeping;
   the annotation is what stops a reader — human or agent — mistaking a
   countered spec for a live one. The primitive frames it and the author
   supplies the substance, the same content-ingestion split `create-scenario`
   uses. The blockquote wrapper is **load-bearing, not stylistic**:
   `derive-dependencies` exempts blockquote-prefixed lines, so the banner may
   link the superseding spec without the annotated spec acquiring a dependency
   on it. Phrase the substance as a non-claim ("no longer exists", "is
   removed") rather than as an assertion whose truth depends on whether its
   paths resolve.
5. **The annotated spec keeps its status.** The write is a mechanical edit
   under §spec-lifecycle — it changes no claim that spec makes about what it
   delivered — so it takes no back-edge at any lifecycle state.
6. **Write no body link to the superseded spec** from the new spec. The
   pointer is frontmatter; a plain body link is harvested into
   `dependencies:`, making the superseding spec depend on the one it counters.
   Cite it by name; the annotation carries the link.
7. **A re-declaration converges.** An annotation already naming this
   superseding spec is reported as already-applied and nothing is written —
   distinct from accumulation, which stacks annotations from *different*
   superseding specs, newest first.

Declaring a supersession over **two specs that already exist** is
`/ductus:supersede`, not this command; it produces the same key and the
same annotation.

### Fill the spec body

Fill in the spec following `.ductus/constitution.md` rules (§spec-requirements, §text-first-artifacts):

- Frontmatter `status` starts at `draft` (template default); `dependencies` starts at `[]` and is generator-managed (do not author by hand).
- Describe behavior and contracts, not implementation.
- No language-specific code, function signatures, or package paths.
- Acceptance criteria must be concrete and testable when present. For brownfield use, sparse acceptance criteria are expected and valid — leave the section with a placeholder comment if no criteria are known yet; criteria emerge as real work touches the feature (§brownfield-process).
- List all open questions in the spec body.
- When the spec depends on other specs, link them inline in the body (e.g., `[NNN-feature](../NNN-feature/spec.md)`) — `ductus derive-dependencies` (run by the pre-commit hook) derives the `dependencies:` frontmatter from those links on every commit.

### Label the acceptance criteria

Assign each criterion its stable `AC{n}:` label, written between the checkbox and the criterion's text (`- [ ] AC7: …`), and record `next-criterion` in the frontmatter (primitive: `label-criteria`). A new spec starts at `AC1:` and numbers in body order; `next-criterion` is one past the last label assigned. Leave the section untouched when it holds no criteria — an absent `next-criterion` means "no labels assigned yet", which is a truthful state rather than a defect (§text-first-artifacts).

Write the label rather than leaving it to a later pass: the label is what a criterion is cited by, in prose, across specs, and by tooling, and a criterion discussed in the session that created it needs its identifier during that conversation. On this path the derivation is `max(highest label in body, next-criterion)` — never `max(body) + 1`, which would reissue the label of a criterion that has since been deleted. The rule is arithmetic, so both paths agree by construction (spec 013).

### Lint the new file

Run `npx markdownlint-cli2` on the new file (primitive: `lint-markdown`).

### Write the session target

Write `.ductus/session.toml` to set this feature as the session target (primitive: `write-session`, gated by `gate-confirm` above). First read any existing `.ductus/session.toml` to capture its cli-config-dir (the per-contributor agent identity written by /ductus) and carry it forward, so creating a new feature never drops the agent identity. Use tempfile + rename atomic-write semantics analogous to the runtime's spec write primitives.

### Display the next step

Display: "Run `/ductus:clarify` to resolve open questions and advance to clarified."
