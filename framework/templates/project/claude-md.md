# CLAUDE.md

@AGENTS.md

<!-- /ductus maintains the region between the BEGIN and END markers below.
     Registered shared constitutions (.ductus/config.toml [constitutions.*])
     are imported there, one line each. Empty when none are registered. Edits
     inside it are overwritten; the rest of this file is preserved
     byte-for-byte. -->
<!-- BEGIN ductus:constitutions -->
<!-- END ductus:constitutions -->

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
