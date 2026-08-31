# CLAUDE.md

@import framework/constitution.md
@import AGENTS.md

## Non-negotiables

> Restated here, not only in `AGENTS.md`, because a rule that depends on an agent
> reading a 90KB file in full is a diligence dependency, which
> [§design-principles](framework/constitution.md#design-principles) rejects. This
> file is small and always loaded whole, so the rule is unmissable by construction
> rather than by care.

- **Commit directly to `main`. This repo is trunk-based — never branch first.**
  `ductus` is live-on-main: the installer and everything `/ductus` fetches track
  `main`, so there is no release branch and no feature-branch/PR step. The
  general "branch off the default branch before committing" default does **not**
  apply here. Branch only when the user explicitly asks. Full entry, with its
  history, in `AGENTS.md` §Workflow.
- **A truncation notice is not a read.** When a tool returns a preview, a
  saved-output pointer, or any capped fragment, the rest is unread — follow the
  pointer or say what you did not examine. The rule and its reasoning live in
  [§grounding](framework/constitution.md#grounding); this is a pointer, not a
  second copy.

## Auto-Memory Routing

> Agent-specific routing for the constitution's *shared knowledge stays in git* principle ([§drift-prevention](framework/constitution.md#drift-prevention)).

Before saving an auto-memory entry, ask: **would this learning help any other contributor to this project?**

- **Yes** → it belongs in a git-tracked artifact, never local auto-memory. Local memory lives under the user's home directory, invisible to everyone else and absent from clones — parking contributor-beneficial guidance there defeats the purpose of a committed governance framework. Route a project learning to `AGENTS.md` (matching section: Gotchas, Workflow, Boundaries, Code Style, Testing, Design Principles); route a framework rule, schema, or behavior to its canonical artifact under `framework/` (see constitution §drift-prevention for the canonical-source map). Skip the memory entry.
- **No** → auto-memory is correct. Reserve it for facts that are purely personal to this user and carry no value to other contributors: cross-project user facts (role, persistent style preferences) and external reference pointers (Linear/Slack/dashboard bookmarks).
