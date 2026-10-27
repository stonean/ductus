# memory-routing-leaves-claude-md

**Introduced in:** ductus 0.60.0
**Summary:** Remove `CLAUDE.md`'s `## Auto-Memory Routing` section when it is a copy the template shipped, unchanged, now that `AGENTS.md`'s `ductus:memory-routing` block carries where a learning goes.

## Background

Through ductus 0.59 the `CLAUDE.md` template carried a `## Auto-Memory Routing` section restating the constitution's §drift-prevention *Shared knowledge stays in git*. Only `claude-style` agents read `CLAUDE.md`, so the routing never reached Antigravity, OpenCode or Pi, and its oldest version routes a learning by kind, which the constitution no longer does. Since `050-constitution`'s scenario `memory-routing-lives-in-agents-md`, `/{project}` writes the routing into a framework-managed block in `AGENTS.md` on every run, every agent reads `AGENTS.md`, and a `claude-style` session reads it through `@AGENTS.md` beside the imported constitution, so the section is a second copy.

`CLAUDE.md` is installed once, with the `skip` strategy (`framework/bootstrap/ductus.md`, **Shared files with conflict handling**), so an existing adopter keeps the section until this migration removes it. The adopter owns the file, so the migration removes only a section the framework wrote and nobody has edited: one that matches a shipped version, ignoring line endings and trailing whitespace. An edited section is left in place and reported. The block is written later in the same run, by Shared Files; until then, and in a run that stops before it, the imported constitution still states the rule.

## Procedure

1. **Resolve the file.** The file is `CLAUDE.md` at the project root. When it does not exist, stop silently and report nothing.

2. **Find the section.** Here and below, a line's **text** is the line without its ending, `\n` or `\r\n`, and a **blank** line is one whose text is empty or holds only spaces and tabs. A **heading** is a line outside a fence whose text, after at most three spaces, is `#` or `##` alone or followed by a space or tab. A **block start** is a line outside a fence whose text, with leading spaces and tabs removed, begins `<!-- BEGIN ductus:` — the first line of a block the framework maintains. A run that found no markers appended the `ductus:constitutions` block at the end of the file, after this section. Walk the file line by line, tracking fenced code blocks as `claude-md-import-syntax` step 2 defines them, applied to each line's text. The section starts at the first heading whose text, with up to three leading spaces and any trailing spaces and tabs removed, reads `## Auto-Memory Routing`, and runs to the line before the next heading or block start, or to the end of the file.

3. **Idempotency check.** When no line starts the section, the file is converged: report `memory-routing-leaves-claude-md: CLAUDE.md has no Auto-Memory Routing section — left as is` and stop without writing.

4. **Skip a pinned file.** When `CLAUDE.md` is listed in `.ductus/config.toml` `[pinned] files`, report one line and stop without writing. Pinning opts a file out of framework writes, and this is one:

   `warning: CLAUDE.md is pinned; remove its ## Auto-Memory Routing section by hand — AGENTS.md's ductus:memory-routing block carries the routing, and a claude-style session loads both.`

5. **Compare.** Normalize the section for comparison only: take each line's text, remove its trailing spaces and tabs, join the lines with `\n`, and drop the blank lines at the end. Normalize each shipped version below the same way, taking the lines between its fence lines, once with `{project}` substituted by the project name this run resolved (`host.project`) and once as written. The section **matches** when it equals any of them.

6. **Leave an edited section.** When it matches none, report one line and stop without writing:

   `memory-routing-leaves-claude-md: CLAUDE.md's ## Auto-Memory Routing section differs from every shipped version — left as is; remove it by hand, since AGENTS.md's ductus:memory-routing block carries the routing and a claude-style session loads both.`

7. **Remove a matching section.** Delete its lines, from the heading through the line before the next heading or block start, or through the end of the file. When it ran to the end of the file, also delete the blank lines before it, so the file ends with the line ending after its last remaining non-blank line, or is empty when none remains. Preserve every other byte, line endings included. Write atomically, keeping the file's mode: a temporary file in the same directory, then a rename.

8. **Report** `memory-routing-leaves-claude-md: removed the Auto-Memory Routing section from CLAUDE.md — AGENTS.md's ductus:memory-routing block carries the routing`.

## Shipped versions

The section as `framework/templates/project/claude-md.md` carried it, from its first commit to the last before it was removed. Each is the complete section, compared as step 5 describes.

**Version 1** — shipped from `7de12166` (2026-05-06):

````markdown
## Auto-Memory Routing

Before saving an auto-memory entry, ask: is this learning specific to *this project*?

- **Yes** → append to `AGENTS.md` in the matching section (Gotchas, Workflow, Boundaries, Code Style, Testing) and skip the memory entry.
- **No** → auto-memory is correct. Reserve it for cross-project user facts (role, persistent style preferences) and external references (Linear/Slack/dashboard pointers).
````

**Version 2** — shipped from `2778a26d` (2026-06-18):

````markdown
## Auto-Memory Routing

> Agent-specific routing for the constitution's *shared knowledge stays in git* principle (Drift Prevention).

Before saving an auto-memory entry, ask: **would this learning help any other contributor to this project?**

- **Yes** → it belongs in a git-tracked artifact, never local auto-memory. Local memory lives under the user's home directory, invisible to everyone else and absent from clones — parking contributor-beneficial guidance there defeats the purpose of a shared, committed codebase. Route a project learning to `AGENTS.md` (matching section: Gotchas, Workflow, Boundaries, Code Style, Testing); route a cross-cutting requirement to the relevant spec, scenario, or rule under `specs/`. Skip the memory entry.
- **No** → auto-memory is correct. Reserve it for facts that are purely personal to this user and carry no value to other contributors: cross-project user facts (role, persistent style preferences) and external reference pointers (Linear/Slack/dashboard bookmarks).
````

**Version 3** — shipped from `59eb3bb9` (2026-09-15):

````markdown
## Auto-Memory Routing

> Agent-specific routing for the constitution's *shared knowledge stays in git* principle (Drift Prevention).

Before saving an auto-memory entry, ask **two** questions, in this order.

**1. Would this learning help any other contributor?**

- **Yes** → it belongs in a git-tracked artifact, never local auto-memory. Local memory lives under the user's home directory, invisible to everyone else and absent from clones — parking contributor-beneficial guidance there defeats the purpose of a shared, committed codebase. Skip the memory entry and answer question 2.
- **No** → auto-memory is correct. Reserve it for facts that are purely personal to this user and carry no value to other contributors: cross-project user facts (role, persistent style preferences) and external reference pointers (Linear/Slack/dashboard bookmarks).

**2. Who is it true for?** This decides *which* committed artifact, and it is a question about the learning's population, never about its kind — a gotcha, a convention and a workflow rule all route by the same test.

- **Every project running the pipeline** → the constitution at `.ductus/constitution.md` is where such a rule is canonically stated. It is upstream of this project, so raise it with the framework rather than restating it here.
- **Every project in your organization** → the shared constitution your organization registers under `.ductus/config.toml` `[constitutions.*]`, which `/{project}:target` loads alongside the shipped one.
- **This project alone** → `AGENTS.md`, matching section (Gotchas, Workflow, Boundaries, Code Style, Testing), or the relevant spec, scenario, or rule under `specs/` when it is a durable requirement rather than a working convention.
````

**Version 4** — shipped from `d1e3ec96` (2026-10-26):

````markdown
## Auto-Memory Routing

> Agent-specific routing for the constitution's *shared knowledge stays in git* principle (Drift Prevention).

Before saving an auto-memory entry, ask **two** questions, in this order.

**1. Would this learning help any other contributor?**

- **Yes** → it belongs in a git-tracked artifact, never local auto-memory. Local memory lives under the user's home directory, invisible to everyone else and absent from clones — parking contributor-beneficial guidance there defeats the purpose of a shared, committed codebase. Skip the memory entry and answer question 2.
- **No** → auto-memory is correct. Reserve it for facts that are purely personal to this user and carry no value to other contributors: cross-project user facts (role, persistent style preferences) and external reference pointers (Linear/Slack/dashboard bookmarks).

**2. Who is it true for?** This decides *which* committed artifact, and it is a question about the learning's population, never about its kind — a gotcha, a convention and a workflow rule all route by the same test.

- **Every project running the pipeline** → the constitution at `.ductus/constitution.md` is where such a rule is canonically stated. It is upstream of this project, so raise it with the framework rather than restating it here.
- **Every project in your organization** → the shared constitution your organization registers under `.ductus/config.toml` `[constitutions.*]`, which this file's managed block imports, or `/{project}:target` loads when the import did not.
- **This project alone** → `AGENTS.md`, matching section (Gotchas, Workflow, Boundaries, Code Style, Testing), or the relevant spec, scenario, or rule under `specs/` when it is a durable requirement rather than a working convention.
````

## Verification

- After a removal, `git diff CLAUDE.md` shows only the section's lines removed, and the blank lines before it when it ended the file; a `ductus:` block that followed it is unchanged.
- A `CLAUDE.md` whose section was edited, or that is pinned, is unchanged, and the run reported the one line naming it.
- After a removal, a second run reports `has no Auto-Memory Routing section` and writes nothing; after a pinned or edited file's report, a second run repeats that report. No run writes to a converged file.
- The file's line-ending style is unchanged.
