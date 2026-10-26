---
description: Create a new feature spec.
argument-hint: "[--branch] [--branch-id <identifier>] [--fold-into <feature>] [feature description]"
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

This command does not require a session target — it creates a new feature, and sets it as this process's session target (and the shared default `.ductus/session.toml`, §concurrent-features) once created.

If the constitution has not been loaded in this session (e.g., `/ductus:target` has not been run), read `.ductus/constitution.md` now to load `ductus` rules. If the constitution was already loaded by `/ductus:target`, do not re-read it.

### Flags

| Flag | Behavior |
| --- | --- |
| *(none)* | Sequential numbering — the next `NNN-` in the global sequence. This is the default and nothing persisted changes it. |
| `--branch` | Create a branch-scoped spec, prompting for the identifier (a candidate is proposed from the current git branch name when one can be extracted) |
| `--branch-id <identifier>` | Create a branch-scoped spec under this identifier, supplied directly. An opaque token, not a number: it is sanitized to `^[a-z0-9]+(?:-[a-z0-9]+)*$`, never refused |
| `--fold-into <feature>` | The upstream sequential spec the branch-scoped spec folds back into, written to its `folds-into:` frontmatter. Required with the two flags above and meaningless without them |

## Scope Boundaries

- This command creates spec artifacts only. Do NOT read or write source code, test files, or implementation files.
- Read only what is needed: existing spec directory names (for numbering and routing), each candidate's `plan.md` Affected Files and frontmatter `status`, the rule-file directory listing, the spec template, and — only when branch-scoped creation is requested — the current git branch name, to propose an identifier candidate. Do NOT read other specs' bodies unless checking for naming conflicts.
- The routing gate (step 1–3) writes nothing: it derives candidates, presents them, and confirms. A denial ends the run with no directory created and no session write.
- Reference: §spec-phase, §spec-requirements, §numbering (both directory forms, and why the branch-scoped one is temporary), §spec-lifecycle (a branch-scoped spec is retired by fold-back, whatever its status), §text-first-artifacts, §brownfield-process, §bug-handling (the routing tree, canonical as **§Groom decision tree** in `.ductus/reference/groom.md`).

## Instructions

> **For agent runtimes**: the Invoke steps below call the MCP tools of the ductus runtime; the host-integration contract — bare↔prefixed tool names, lazy ToolSearch schema fetch, the no-shell-utilities rule, and the two-paths guarantee — lives once in the constitution, §runtime-host-integration. Before the server is registered — the window between acquisition and the restart that loads it — walk the same prose using the host file-reading tools (Read, Edit, Write).

**Where a named section lives.** This command's markdown-only reference — the procedure for the path without the runtime, and the policy these steps apply — is `.ductus/reference/specify.md`. A section named here, or cited elsewhere as part of this command, that has no heading in this file is there; read it when a step points at it.

1. Invoke `derive-routing-candidates` with the feature description from `$ARGUMENTS` (required — if empty, ask the user what feature to specify), plus `routed-by` when another command's routing tree already chose this route (`/ductus:groom` recommending `/ductus:specify` is the case that matters). It derives the homes this work could already have — the runtime-owning spec when the description names a primitive or a `runtime/` path, a rule surface whose category it shares, a spec whose subject it overlaps — and reports every source as examined or skipped. A `gate-required: false` result means the decision has been made already: skip steps 2 and 3 and continue at step 4.

2. <!-- llm:routeInboxItem --> Walk the **same** decision tree `/ductus:groom` walks (**§Groom decision tree** in `.ductus/reference/groom.md` is its single canonical statement — do not restate it here), over the feature description in place of an inbox item, with step 1's candidates as evidence. Return the route and, when it targets an existing spec, the feature. `spec` — no existing home covers this — is the route that proceeds to create one.

3. Invoke `gate-confirm` with a `gate` name (e.g. `specify-route`) and a `prompt` naming the chosen route and the alternatives, so the operator confirms the routing **before** anything is scaffolded. When a candidate spec is `done`, the prompt names the `done → in-progress` back-edge accepting it implies, exactly as groom's per-item confirmation does. When the derivation reported no candidates and no skipped sources, say so and proceed — a fresh adopter sees one line, not a decision. When any source was **skipped**, report *could not derive candidates* and name the source: that is a different answer from *none found*, and stating it as the latter would claim a check ran that did not. Denial ends the run cleanly with nothing written, which is how the operator takes a candidate instead; the framework reports and does not veto, so creating a new spec over any candidate stays one keystroke away.

<!-- audit:ignore-promotion -->
4. Settle branch-scoped creation before anything is scaffolded (host responsibility; no primitive — the decision is a conversation with the operator, and the sanitization it previews belongs to create-feature in step 5). **Sequential numbering is the default and this step decides nothing without a request for the other form**: with neither `--branch` nor `--branch-id` present, continue at step 5 with the arguments unchanged and settle nothing here — that is the path §numbering calls the default, and the one an adopter who never asks for a branch-scoped spec must keep observing. Otherwise settle three things, in this order:

   - **The identifier.** `--branch-id <identifier>` supplies it directly; `--branch` requests branch-scoped creation without one, in which case prompt for it. Offer a candidate read from the current git branch name (`git rev-parse --abbrev-ref HEAD`) when a tracker-style token can be extracted from it, and prompt with **no** candidate when it cannot — a branch named `main` or `wip` yields nothing to propose, and inventing one would be worse than asking. The framework parses no branch-name grammar, so a candidate is a convenience the operator confirms or corrects, never an authority; the value they settle on is the value used.
   - **What the identifier will become.** A non-conforming identifier is sanitized rather than refused — lowercased, with every run of non-alphanumerics collapsed to a single hyphen — so `PROJ-1111` becomes `proj-1111`, `1111-PROJ` becomes `1111-proj`, and a `.` becomes a hyphen, which is what leaves the `{branch-id}.{n}` delimiter unambiguous. Name the sanitized form in the prompt, **before any directory exists**: a transformation the operator never sees is one they discover from a directory name later. Step 6 reports the identifier actually used, and that one is authoritative — say so if it differs from what was previewed here.
   - **The fold target, which is required.** `--fold-into <feature>` supplies it; prompt when it is absent. There is no path that creates a branch-scoped spec naming no upstream home: the number is what keeps the merge clean, and the target is what makes the spec actionable once it lands, so a staging spec with nowhere to land is not a case this framework has (§numbering, §spec-lifecycle). The target names a **sequential** spec and need not exist in this tree — a branch-scoped spec exists *because* upstream diverged, so its home routinely lives on the branch this one forked from, and existence is enforced at fold-back rather than here. An operator who will not name one is asking for a sequential spec: offer that, and end the run with nothing created if they decline both.

   **On `ductus exec`** there is no operator to prompt, so this step reduces to passing through whatever `--branch-id` and `--fold-into` the invocation already supplied; branch-scoped creation without them is refused by the primitive in step 5 rather than prompted for. The reduction is documented rather than silent (§runtime-host-integration's two-paths guarantee).

5. Invoke `create-feature` with the feature description from `$ARGUMENTS` as the title, plus the `branch-id` and `fold-into` arguments step 4 settled when it settled any. The primitive computes the next feature number from the existing NNN-prefixed directories under the configured specs root, derives the kebab-case slug, creates `specs/{NNN-slug}/`, and copies the spec template into it atomically (mode-preserving); it resolves the template from `{specs-root}/templates/spec.md` and falls back to the framework source layout `framework/templates/spec/spec.md` (the ductus repo's own layout). With `branch-id` supplied it instead sanitizes that identifier, numbers `{identifier}.{n}-{slug}` from the directories already under that identifier alone, stamps `folds-into:` into the new spec's frontmatter, and returns the **sanitized** `identifier` it used — report that value, since it is what the operator actually got. An already-existing target directory is the `created: false` domain outcome — report the collision and stop rather than overwrite; under one identifier that is also what makes two contributors creating at the same moment safe, the loser being refused rather than overwriting the winner.

6. <!-- llm:writeSpecBody --> Fill the new spec body following §spec-requirements: a Motivation section, Acceptance Criteria with concrete and testable checkboxes (sparse acceptance criteria are valid for brownfield use — leave the section with a comment noting criteria will emerge from real work), Open Questions, and any inline links to other specs that `derive-dependencies` will derive the frontmatter dependencies from. The host returns the markdown body for the new file; the walker forwards the response through the context.

7. Invoke `label-criteria` against the new feature to assign a stable `AC{n}:` label to every criterion the step above wrote, and to record `next-criterion` in the frontmatter. The initial batch is labelled in the run that created it, so a criterion can be cited by label in the same conversation that authored it — that is the moment citation matters most. The pass is idempotent and writes nothing when the section is empty, so a brownfield spec with a placeholder comment and no criteria is unaffected. **Never derive the label in the LLM**: picking `max + 1` means tallying the list, which is exactly the counting this labelling exists to remove.

8. Invoke `lint-markdown` against the new spec file to surface any markdown violations the LLM may have introduced.

9. Invoke `gate-confirm` with a `gate` name (e.g. `specify-create`) and a `prompt` asking the user to approve creating the new feature and setting it as the session target before any session-file write. `gate-confirm` is non-blocking — it returns the prompt payload (`gate`, `prompt`, `request-id`) and the host routes the decision out-of-band. On confirmation, continue to the session write below; on denial, the walker exits cleanly without writing the session.

10. Invoke `write-session` with the new feature slug and its repo-relative spec directory — under the configured `[paths] specs-root` (default `specs`; spec 040) — as the feature and path arguments. This is a target write: the primitive stamps a fresh set-at while preserving any cli-config-dir already in the file (the per-contributor agent identity written by `/ductus`), at this process's own target under `.ductus/sessions/` when it has a session identity and at the shared default `.ductus/session.toml`, through tempfile + rename atomic-write semantics. Display what the write reports: any `notices` it delivered, the `peers` — each other session on this feature, with its scenario when it has one and when it last used its target, since this write is when the writing session is told who shares its feature (§concurrent-features) — and any `expired` or `unreadable` session. On the markdown-only path, the host writes the file by hand per **§Write the session target** in `.ductus/reference/specify.md` — the cli-config-dir preservation rule there applies verbatim.
