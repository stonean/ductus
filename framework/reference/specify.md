# specify — markdown-only reference

The full new-feature-creation procedure (directory creation, template copy, frontmatter conventions, session write, and next-step prompt) is documented below for the markdown-only path. The numbered steps in `specify.md`'s Instructions invoke the mechanical primitives plus the writeSpecBody extension that automate the deterministic phases.

**Where a named section lives.** This file is read alongside its command, `specify.md`, which is already loaded whenever this one is. A section named below that has no heading in this file is in the command file, along with the numbered Instructions steps this file cites. A bare `§name` is a heading in this file or a marker in the constitution, `.ductus/constitution.md`; a `§name` with another file's path on the same line is a heading in that file.

## Route before scaffolding

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

Then walk **§Groom decision tree** in `.ductus/reference/groom.md`, which is its
single canonical statement, over the description in place of an inbox item,
with those candidates as evidence, and confirm the result with the user before
anything is written:

- **Candidates found** — name them, name the route each implies, and let the
  operator pick: a scenario on `NNN`, a rule-file amendment, or a new spec
  anyway. A new spec stays creatable over any candidate; this reports, it
  does not veto. When a named candidate is `done`, say that accepting it
  reopens it `done → in-progress`, so the operator consents to the back-edge
  before it happens — the same wording groom's per-item confirmation uses.
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
— `/{project}:groom` routing an item to `/{project}:specify` is the case — and
say why. Asking a question that has been answered is friction, not rigor.

## Resolve feature number and slug

1. The command's argument is the feature description (e.g., "webhook delivery"). This is required — if empty, ask the user what feature to specify.
2. Determine the next available feature number by checking existing directories under `specs/` matching the NNN-feature pattern; the next number is the highest existing NNN plus one (zero-padded to three digits).
3. Generate the slug from the feature description: lowercase, hyphenated, no whitespace, no punctuation beyond hyphens.

## Create the feature directory

1. Create `specs/{NNN-feature-name}/`.
2. Copy `specs/templates/spec.md` into the directory as `spec.md`.

Both sections above are what the `create-feature` primitive automates on the runtime path (number scan, slug derivation, directory creation, atomic template copy); walk them by hand only when no runtime is available.

## Settle branch-scoped creation

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

The branch-scoped directory is a staging form: it is discharged by fold-back,
whatever its status (§numbering, §spec-lifecycle).

## Fill the spec body

Fill in the spec following `.ductus/constitution.md` rules (§spec-requirements, §text-first-artifacts):

- Frontmatter `status` starts at `draft` (template default); `dependencies` starts at `[]` and is generator-managed (do not author by hand).
- Describe behavior and contracts, not implementation.
- No language-specific code, function signatures, or package paths.
- Acceptance criteria must be concrete and testable when present. For brownfield use, sparse acceptance criteria are expected and valid — leave the section with a placeholder comment if no criteria are known yet; criteria emerge as real work touches the feature (§brownfield-process).
- List all open questions in the spec body, each as one list item under `## Open Questions` — a paragraph there is not read as a question.
- When the spec depends on other specs, link them inline in the body (e.g., `[NNN-feature](../NNN-feature/spec.md)`) — `ductus derive-dependencies` (run by the pre-commit hook) derives the `dependencies:` frontmatter from those links on every commit.

## Label the acceptance criteria

Assign each criterion its stable `AC{n}:` label, written between the checkbox and the criterion's text (`- [ ] AC7: …`), and record `next-criterion` in the frontmatter (primitive: `label-criteria`). A new spec starts at `AC1:` and numbers in body order; `next-criterion` is one past the last label assigned. Leave the section untouched when it holds no criteria — an absent `next-criterion` means "no labels assigned yet", which is a truthful state rather than a defect (§text-first-artifacts).

Write the label rather than leaving it to a later pass: the label is what a criterion is cited by, in prose, across specs, and by tooling, and a criterion discussed in the session that created it needs its identifier during that conversation. On this path the derivation is `max(highest label in body, next-criterion)` — never `max(body) + 1`, which would reissue the label of a criterion that has since been deleted. The rule is arithmetic, so both paths agree by construction (spec 013).

## Lint the new file

Run `npx markdownlint-cli2` on the new file (primitive: `lint-markdown`).

## Write the session target

Write the shared default `.ductus/session.toml` to set this feature as the session target (primitive: `write-session`, gated by `gate-confirm` at step 9 of the command's Instructions). Without the runtime the host cannot read its session identity, so this path writes the shared default only (§concurrent-features). First read any existing `.ductus/session.toml` to capture its cli-config-dir (the per-contributor agent identity written by /ductus) and carry it forward, so creating a new feature never drops the agent identity. Use tempfile + rename atomic-write semantics analogous to the runtime's spec write primitives.

## Display the next step

Display: "Run `/{project}:clarify` to resolve open questions and advance to clarified."
