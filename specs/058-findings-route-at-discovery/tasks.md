# 058 — Findings route at discovery Tasks

Tasks derived from the [plan](plan.md). Complete in order.

<!-- Each task should be small enough to implement and verify independently.
     Mark subtasks as they are completed. Every task MUST close with a
     `- **Done when**: …` line stating its completion condition — the
     tooling reads this exact form to confirm the task is fully specified.

     A task body may also carry working notes — what the next session needs
     to resume it: the mechanics, the order, what is left on disk — placed
     as §tasks-phase says. Write them as prose on the pending task they
     concern, so they go when the task is pruned. Put an ordering constraint
     on the task that must wait, never above the first task or under a
     heading of its own, which outlive every task; and never write a note as
     a checkbox, which would count toward the task's completion. Anything
     that must outlast the task belongs in its durable home first. Example:

## 1. Create sessions table migration

- [ ] Write SQL migration for `sessions` table
- [ ] Run migration and verify schema

- **Done when**: the migration applies cleanly and `sessions` matches the data model.

## 2. Implement session store

Blocked on task 1: the store tests run against the migrated schema.

- [ ] Create `shared/auth/session.go` with Create, Get, Delete methods
- [ ] Write store integration tests against real PostgreSQL

- **Done when**: all store methods are covered by passing integration tests.

## 3. Update README link to migration guide

- [ ] Edit `README.md` to point at the new path

- **Done when**: the README link resolves to the new path.

-->

## 1. Findings cost in proportion to risk

- [x] Implement the behavior described in `scenarios/findings-cost-in-proportion-to-risk.md`

Start with `/ductus:target 058-findings-route-at-discovery/findings-cost-in-proportion-to-risk` and `/ductus:clarify`; the owner pre-authorized the scenario's `Recommended:` answers (2026-10-26). A record's `dispositions:` map does not split its counts by tier, but the gating findings already gate through other checks — an analyze hard-fail or blocking finding keeps `blocking: true` until the re-check no longer produces it, and review MUST and SHOULD violations have their own counts — so read `check-review-gate`'s undispositioned check against that before changing it, and keep every other gate check as is. The constitution text to change is §brownfield-inbox Finding dispositions (the gate bullet) and the §implement-phase outstanding-SHOULD bullet, within the ceiling in `scripts/lint-governing-size.sh` (then the file's exact size), so it removes as many bytes as it adds; `AGENTS.md` (about 3 KB under its 50,000 ceiling) gets the fix-round entry's scope.

- **Done when**: the scenario's described behavior is correctly implemented and tested

## 2. Remove the guidance comment and placeholder from inboxes that kept them

- [x] Add the later `framework/migrations.toml` entry and its `framework/migrations/{id}.md` procedure as the plan's **The later migration** describes, satisfying AC35
- [x] Verify it and `inbox-guidance-refresh`'s removal branch with a real adopter bootstrap from a subject adopted before 0.53.0, then tick AC22 and AC35

The entry is `strip-inbox-guidance` at 0.60.0, the release that ships it, sorting after `remove-legacy-constitutions-region` on the id tie-break. It must not be pushed without `013-text-first-artifacts` task 25, the fix for `scenarios/the-precheck-ignores-what-the-same-run-wrote.md`, which goes out in the same push: before it, the Frontmatter Migration's precheck refused a tree a Pre-run Migration dirtied under `specs/`, and this entry would put every adopter past 0.53.0 in front of that refusal, a `--ref=main` adopter on the push. A subject adopted before 0.53.0 crosses both inbox entries, and its Pre-run Migrations write under `specs/`, so one bootstrap verifies this task and 013 task 25 together. After this task, 058's review shares `bca9a2fd`'s window with 006's, which `specs/006-bug-workflow/review.md` reviewed in depth at `797799ba`: review only what changed since, per `AGENTS.md`'s entry on one commit reopening several specs.

- **Done when**: a real adopter bootstrap from a pre-0.53.0 subject leaves an inbox with no guidance comment and no placeholder, every other item byte-identical, and a second run writes nothing

## 3. Disposition out-of-spec findings from task 2's real adopter bootstrap

- [x] framework/bootstrap/ductus.md — Shared Files' Shared constitution imports step 1 passes `--unmeasured` to `resolve-constitutions` (055 task 15, `e8ed5d62`), which the pinned 0.59.11 runtime a `--ref=main` run acquires rejects on the CLI (`error: unexpected argument '--unmeasured' found`, exit 2), while its MCP server accepts `unmeasured: true` silently; every `--ref=main` adopter meets it until a release carries the flag (all three subjects, 2026-10-30) — discarded (owner, 2026-10-30, option a): the gap closes when 0.60.0 is tagged and the pin moves to a runtime carrying the flag; running `main`'s procedure against the last release's runtime is how `--ref=main` works for every runtime-dependent change, and a fallback would cost every `--ref=main` adopter a self-update restart to cover a window it would outlive
- [x] framework/bootstrap/hooks/ductus-pre-commit — the comment naming `{project}/.githooks/ductus-pre-commit` is substituted with the project name on install — discarded: the substituted comment still names the project's own `.githooks/` directory, and nothing reads a comment
- [x] framework/bootstrap/ductus.md and ductus-procedure.md — ambiguities the three hosts named in the pipeline's own machinery — discarded, per AGENTS.md's rule on such observations made while verifying framework work: the walker's placement of the archive-half sections is already `056-bootstrap-archive-boundary-split`'s scenario `the-walker-places-every-archive-half-section`; the Frontmatter Migration's walk matching `specs/templates/spec.md` was discarded before (052's review: the template has frontmatter); State A detection counting another project's `mcp__ductus__*` tools is an artifact of running the bootstrap from this repository's session; no active entry's `sunset_after` falls between 0.59.11 and `main`, so the sunset filter's version on `main` changes nothing; `generator-primitives` and `audit-record-relocate` defining no report line for their normal path, `ln -sf` re-linking an identical pointer, a per-language `.gitignore` 404 reported as a fetch failure, and `cli-config-dir` on the shared `apply-manifest` call change no file a run writes

- **Done when**: each finding is fixed, routed, or discarded, with a discard's reason written on the task
