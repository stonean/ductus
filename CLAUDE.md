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
  Releases are cut from `main` by tag, and `main` is the integration branch an
  adopter opts into with `--ref=main`, so there is no release branch and no
  feature-branch/PR step. The
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

This repository keeps **no** auto-memory — no entries and no `MEMORY.md` (`AGENTS.md` §Workflow). Adopting projects may keep one; this is this repository's rule. So a learning worth keeping goes into git or is not kept. Before keeping one, ask **two** questions, in this order.

**1. Would this learning help any other contributor?**

- **Yes** → it belongs in a git-tracked artifact. Local memory lives under the user's home directory, invisible to everyone else and absent from clones — parking contributor-beneficial guidance there defeats the purpose of a committed framework. Answer question 2.
- **No** → it is not kept. In this repository even a fact purely personal to one user (role, a style preference, a dashboard bookmark) is not written to auto-memory; if it shapes how agents should work here, it is not purely personal — record it in `AGENTS.md`.

**2. Who is it true for?** This decides *which* committed artifact, and it is a question about the learning's population, never about its kind — a gotcha, a convention and a workflow rule all route by the same test.

- **Every project running this pipeline** → `framework/constitution.md`, the one artifact adopters receive. It is constitution material however it was learned; filing it here because it reads like a gotcha is the routing default §drift-prevention now rules out.
- **This repository alone** → `AGENTS.md`, matching section (Workflow, Gotchas, Boundaries, Design Principles).
- **A framework rule, schema, or behavior** → its canonical artifact under `framework/` (see constitution §drift-prevention for the canonical-source map), whichever population it has.
