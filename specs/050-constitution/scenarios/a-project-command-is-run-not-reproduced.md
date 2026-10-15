---
section: "Follow-on scenarios"
---

# A-project-command-is-run-not-reproduced

## Context

An agent working in an adopting project built the project's Docker image with a command it composed itself, although the project declares `npm run docker:build` for exactly that. It has been observed repeatedly, and is not particular to one agent.

The cost is not the duplicated effort. A substitute that succeeds is genuine evidence about the substitute and says nothing about the command every contributor and CI job runs — with the flags, environment and build arguments it carries. The project's own script goes unexercised, so its defects stay hidden; running it is how they would surface.

Nothing adjacent covers it. [§grounding](../../../framework/constitution.md#grounding)'s connected-database bullet tells the agent to discover what is reachable from the project's own configuration, but its subject is a source to *read*, not a command to *run*. Source 2 of §grounding counts "an actual test run" as live state without saying whose invocation earns it. The adopter `AGENTS.md` template carries a `## Commands` section, but as a commented-out example, and nothing says a declared command binds.

The rule is universal: every adopter has some command runner, and the rule survives the reword test because it names no machinery only this repository has.

## Behavior

[§grounding](../../../framework/constitution.md#grounding)'s **Rules** list gains a bullet immediately after *A connected database is a primary source, not a hazard*, whose discovery clause it parallels — a bullet rather than a section, so the anchor set AC10 protects is unchanged. It states:

- **Run the project's own command, never a hand-rolled equivalent of one.** When the project declares how to build, test, lint, migrate, or package something, that declaration is the command. A reproduction by hand — a raw `docker build` beside an `npm run docker:build` — yields genuine evidence about the wrong subject.
- **Discover the commands from the project's own configuration** — `package.json` scripts, Makefile or justfile targets, compose files, CI workflow steps — and from any `Commands` section its `AGENTS.md` declares. A declared command is authoritative when present, and its absence is not evidence that none exists.
- **A narrower invocation while iterating is fine** — one test, one package. The result reported when the iteration ends is earned by the project's command.
- **A project command that fails is a finding, not an obstacle.** It takes one of [§design-principles](../../../framework/constitution.md#design-principles)' three dispositions — fix it, record it where the pipeline will surface it again, or decide it is out of scope and record why — and is never routed around, because the substitute's success hides exactly the defect the real command would have exposed.
- **When the declared command cannot do what the task needs** — a flag it does not pass through, a target it lacks — the agent deviates and says which command it bypassed and why.
- **When an operation has no declared command and the project will need it again** — a build, a test run, a migration, a seed, a package — the agent proposes adding it to the project's own command runner rather than running an ad-hoc sequence. Extending the command set is work, and takes a task like any other.
- **Nothing enforces this, and the bullet says so.** The substitute usually runs in a shell and never lands in the tree, so no review of a diff can see it.

## Edge Cases

- **A ductus-owned wrapper around a project command is rejected.** It is a second entry point for one operation — [§drift-prevention](../../../framework/constitution.md#drift-prevention)'s *a pointer, never a copy*, applied to commands. A wrapper that adds nothing is indirection to keep in sync; one that adds a flag or a variable makes agents run something different from what humans and CI run, which is the divergence the rule exists to remove.
- **`.ductus/` is not a home for project commands.** It holds framework-owned state: the constitution under the `update` strategy, `config.toml` with managed blocks, the gitignored `bin/` pointer, the session files. A build script is project code, and under a framework directory it is invisible to contributors who do not run ductus and to CI. Who runs a command decides where it lives: the pipeline's operations are runtime primitives, the project's are its command runner's.
- **No detector for repeated operations.** Repetition across sessions is not derivable — an agent has no record of what it ran in another session — so a mechanism that proposed scripts from observed repetition would depend on someone noticing, the diligence dependency §design-principles rejects. The clause that fires at the moment of need is the version that does not degrade silently.
- **Sharing across an organization splits in two.** Which commands every project exposes, and what each means, is shared-tier material for a constitution registered under `[constitutions.*]` ([055](../../055-shared-constitution/spec.md)). The implementation is distributed by the organization's own mechanism — a package, a reusable CI workflow, a shared include — which pins versions. 055 reads a checkout it never fetches or pins, and a missing one degrades to `not-checked-out`; that is acceptable for a document an agent reads and not for a script a build runs.
- **A declared command is not an authorization.** The rule says *which* command to run when running one is in scope; it does not license running a declared `deploy`, `publish`, or destructive migration the task did not authorize.
- **Not a rule file.** [§rules](../../../framework/constitution.md#rules) requires a rule to be citable, and this one mostly is not, since the substitute rarely lands in the tree. Where one does — a committed script or CI step duplicating a declared command — it falls within the review's reuse pass, whose subject is logic that duplicates what already exists.
- **The adopter `AGENTS.md` template is not edited.** The rule derives commands from the runner files, so the `## Commands` section is an optional index rather than a load-bearing input, and `AGENTS.md` is scaffolded with strategy `skip` — an edit would reach new adopters only, while the constitution reaches every one.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
