---
description: Create a new feature spec.
argument-hint: "[feature description]"
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

## Scope Boundaries

- This command creates spec artifacts only. Do NOT read or write source code, test files, or implementation files.
- Read only what is needed: existing spec directory names (for numbering) and the spec template. Do NOT read other specs' bodies unless checking for naming conflicts.
- Reference: §spec-phase, §spec-requirements, §numbering, §text-first-artifacts, §brownfield-process.

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the optional ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. With no ductus MCP server registered, walk the same prose using the host file-reading tools (Read, Edit, Write).

1. Invoke `create-feature` with the feature description from `$ARGUMENTS` as the title (the description is required — if empty, ask the user what feature to specify). The primitive computes the next feature number from the existing NNN-prefixed directories under the configured specs root, derives the kebab-case slug, creates `specs/{NNN-slug}/`, and copies the spec template into it atomically (mode-preserving); it resolves the template from `{specs-root}/templates/spec.md` and falls back to the framework source layout `framework/templates/spec/spec.md` (the ductus repo's own layout). An already-existing target directory is the `created: false` domain outcome — report the collision and stop rather than overwrite. With no ductus runtime registered, walk the markdown-only path below instead.

2. <!-- llm:writeSpecBody --> Fill the new spec body following §spec-requirements: a Motivation section, Acceptance Criteria with concrete and testable checkboxes (sparse acceptance criteria are valid for brownfield use — leave the section with a comment noting criteria will emerge from real work), Open Questions, and any inline links to other specs that .ductus/scripts/gen-spec-deps.sh will derive the frontmatter dependencies from. The host returns the markdown body for the new file; the walker forwards the response through the context.

3. Invoke `label-criteria` against the new feature to assign a stable `AC{n}:` label to every criterion the step above wrote, and to record `next-criterion` in the frontmatter. The initial batch is labelled in the run that created it, so a criterion can be cited by label in the same conversation that authored it — that is the moment citation matters most. The pass is idempotent and writes nothing when the section is empty, so a brownfield spec with a placeholder comment and no criteria is unaffected. **Never derive the label in the LLM**: picking `max + 1` means tallying the list, which is exactly the counting this labelling exists to remove.

4. Invoke `lint-markdown` against the new spec file to surface any markdown violations the LLM may have introduced. With no ductus runtime registered, run `npx markdownlint-cli2` per the markdown-only path.

5. Invoke `gate-confirm` with a `gate` name (e.g. `specify-create`) and a `prompt` asking the user to approve creating the new feature and setting it as the session target before any session-file write. `gate-confirm` is non-blocking — it returns the prompt payload (`gate`, `prompt`, `request-id`) and the host routes the decision out-of-band. On confirmation, continue to the session write below; on denial, the walker exits cleanly without writing the session.

6. Invoke `write-session` with the new feature slug and its repo-relative spec directory — under the configured `[paths] specs-root` (default `specs`; spec 040) — as the feature and path arguments. This is a target write: the primitive stamps a fresh set-at while preserving any cli-config-dir already in the file (the per-contributor agent identity written by `/ductus`), at `.ductus/session.toml`, through tempfile + rename atomic-write semantics. On the markdown-only path, the host writes the file by hand per the markdown-only reference's Write the session target section — the cli-config-dir preservation rule there applies verbatim.

## Markdown-only reference

The full new-feature-creation procedure (directory creation, template copy, frontmatter conventions, session write, and next-step prompt) is documented below for the markdown-only path. The numbered steps above invoke the mechanical primitives plus the writeSpecBody extension that automate the deterministic phases.

> **Spec-root resolution.** Every `specs/…` path below is written under the configured `[paths] specs-root` (default `specs`; spec 040, constitution §spec-phase). When `.ductus/config.toml` sets `[paths] specs-root`, substitute that name for the literal `specs/` throughout — the feature-number scan, the new feature directory, the `templates/spec.md` source, and the session `path`. The literal `specs/` is the documented default; the runtime primitives already resolve it, so only this markdown-only path performs the substitution by hand.

### Resolve feature number and slug

1. `$ARGUMENTS` is the feature description (e.g., "webhook delivery"). This is required — if empty, ask the user what feature to specify.
2. Determine the next available feature number by checking existing directories under `specs/` matching the NNN-feature pattern; the next number is the highest existing NNN plus one (zero-padded to three digits).
3. Generate the slug from the feature description: lowercase, hyphenated, no whitespace, no punctuation beyond hyphens.

### Create the feature directory

1. Create `specs/{NNN-feature-name}/`.
2. Copy `specs/templates/spec.md` into the directory as `spec.md`.

Both sections above are what the `create-feature` primitive automates on the runtime path (number scan, slug derivation, directory creation, atomic template copy); walk them by hand only when no runtime is available.

### Fill the spec body

Fill in the spec following `.ductus/constitution.md` rules (§spec-requirements, §text-first-artifacts):

- Frontmatter `status` starts at `draft` (template default); `dependencies` starts at `[]` and is generator-managed (do not author by hand).
- Describe behavior and contracts, not implementation.
- No language-specific code, function signatures, or package paths.
- Acceptance criteria must be concrete and testable when present. For brownfield use, sparse acceptance criteria are expected and valid — leave the section with a placeholder comment if no criteria are known yet; criteria emerge as real work touches the feature (§brownfield-process).
- List all open questions in the spec body.
- When the spec depends on other specs, link them inline in the body (e.g., `[NNN-feature](../NNN-feature/spec.md)`) — `.ductus/scripts/gen-spec-deps.sh` (run by the pre-commit hook) derives the `dependencies:` frontmatter from those links on every commit.

### Label the acceptance criteria

Assign each criterion its stable `AC{n}:` label, written between the checkbox and the criterion's text (`- [ ] AC7: …`), and record `next-criterion` in the frontmatter (primitive: `label-criteria`). A new spec starts at `AC1:` and numbers in body order; `next-criterion` is one past the last label assigned. Leave the section untouched when it holds no criteria — an absent `next-criterion` means "no labels assigned yet", which is a truthful state rather than a defect (§text-first-artifacts).

Write the label rather than leaving it to a later pass: the label is what a criterion is cited by, in prose, across specs, and by tooling, and a criterion discussed in the session that created it needs its identifier during that conversation. On this path the derivation is `max(highest label in body, next-criterion)` — never `max(body) + 1`, which would reissue the label of a criterion that has since been deleted. The rule is arithmetic, so both paths agree by construction (spec 013).

### Lint the new file

Run `npx markdownlint-cli2` on the new file (primitive: `lint-markdown`).

### Write the session target

Write `.ductus/session.toml` to set this feature as the session target (primitive: `write-session`, gated by `gate-confirm` above). First read any existing `.ductus/session.toml` to capture its cli-config-dir (the per-contributor agent identity written by /ductus) and carry it forward, so creating a new feature never drops the agent identity. Use tempfile + rename atomic-write semantics analogous to the runtime's spec write primitives.

### Display the next step

Display: "Run `/ductus:clarify` to resolve open questions and advance to clarified."
