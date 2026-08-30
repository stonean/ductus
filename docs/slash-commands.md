# Slash commands

The full reference for every command `ductus` installs. The [README](../README.md#commands) carries the one-line summary of each; this is where each one says what it is for, why it exists, and what it will not do.

Commands are **session-aware**: `/target` sets the feature the others act on, and `/specify` targets the feature it creates. Every command that writes to a second spec, or removes anything, confirms first.

## Pipeline — advance state

### `/specify` — a new feature spec, targeted for the session

**Reach for it when you have new work and no spec covers it.** It creates the feature directory and makes it the session target.

Before scaffolding anything it checks whether an existing spec already owns the work, because a duplicate spec is the one mistake here that is expensive to undo. It takes a rich description or a one-line sketch — sparse brownfield input is expected, not merely tolerated.

Flags: `--branch` / `--branch-id <id>` with `--fold-into <feature>` create the branch-scoped form (see `/fold`), and `--supersedes <feature>` records that this spec counters an existing one.

### `/clarify` — open questions resolved, and the spec advanced to `clarified`

**Reach for it when the spec has open questions and you are about to plan against it.** It walks them one at a time and records each answer with its reasoning.

Planning over an unresolved question is how a plan gets written twice.

### `/plan` — technical decisions, affected files, and an ordered task list

**Reach for it when the spec is settled and you need to know what the change actually touches.** Persistence-heavy features also get a data model.

### `/implement` — the code, and the spec's move to `in-progress` then `done`

**Reach for it when it is time to write code.** It walks the task list one task at a time, and is the only command that writes application code.

The spec reaches `done` once every task and acceptance criterion is checked and the review gate passes — not before.

### `/review` — `review.md`, and a hold on `done` while violations stand

**Reach for it before you call a feature done.** It audits the code against the project's rule files across five dimensions: security, reuse, quality, efficiency, and simplicity.

MUST violations keep the spec out of `done` until they are fixed or waived with a recorded reason — so "we will fix it later" leaves a trace instead of evaporating. `--all`, `--fix`, and `--waive <rule-id> --reason "<text>"` are supported.

### `/analyze` — a report of where a feature's own artifacts disagree

**Reach for it when something is out of sync and you want to know what.** A task list that no longer matches the plan, a ticked criterion whose file is gone, a spec at `done` with a blocking review.

Unlike `/review`, it audits artifacts against *each other* rather than against code, and it runs at any time — it is a safety check, not a gate. Read-only; `--fix` corrects checkbox drift and `--all` scans every feature.

## Refine — adjust a spec's artifacts

**Commands split on how many specs they write, and that is what decides whether a capability is a flag or a command of its own.** `/amend`, `/prune`, `/clarify`, `/plan` and `/implement` each write **one** spec, and each declares that single-spec scope. `/fold`, `/supersede` and `/consolidate` write **two**, so none of them fits inside a single-spec command as a flag — widening one to accommodate a two-spec operation qualifies every statement it makes about its own scope. That is why `--supersedes` is a flag on `/specify` (which is writing the spec anyway) while a *retroactive* supersession is `/supersede`, and why `/fold` gains no `--into`.

### `/amend` — a question or scenario recorded, with the lifecycle back-edge taken

**Reach for it when a question or a new behavior surfaces against a spec that has already moved on.**

It takes the back-edge for you: a new question reopens the spec to `draft`, a new scenario reopens a `done` spec to `in-progress`. A spec's status never quietly disagrees with its content. One spec.

### `/supersede` — the `supersedes:` key on one spec, the annotation on the other

**Reach for it when a later spec countered an earlier one and nothing records that.**

Without a marker, a reader — human or agent — cannot tell a live decision from one that was overturned, and the reflex is to delete the stale spec, which strands every pointer into it. This writes the key on the newer spec and a reciprocal annotation on the older one naming what no longer holds.

The earlier spec **stays**, annotated, as the record of what shipped. Use `/specify --supersedes` instead when you are writing the countering spec right now. Two specs.

## Destructive — these remove content

Three commands delete rather than rewrite. All three confirm before they act, and none writes a backup.

### `/prune` — spent task sections in one `tasks.md`

**Reach for it when `tasks.md` has turned into a changelog of finished work and you cannot see what is left.** It drops fully-checked task sections, or `--reset` takes the file back to template state.

Scoped to a single artifact the framework classes as ephemeral work-tracking: the durable record of what was done lives in the spec, the code, and git history — which is also the only recovery. One spec.

### `/fold` — the branch-scoped staging directory, after migrating its content

**Reach for it when a spec was created on a branch only to avoid colliding with the upstream branch's spec, and that branch has now merged.**

Two branches each adding the next `NNN-` spec is a merge conflict in the one file two people are most likely to add at once. So `/specify --branch` numbers the second one from a branch identifier instead — `1234.1-slug` — which cannot collide, and the merge is clean by construction.

That number is scaffolding, not a home. `/fold` is how you take it down: run it on the upstream branch after the merge, and the staging spec's content moves into the durable spec it was standing in for (as a body edit, or as a scenario under it), every inbound pointer is re-pointed, and the directory is removed.

**The point is that one feature ends up with one durable home** — rather than its decisions spread across a permanent spec and a leftover branch spec nobody ever consolidated. A branch-scoped spec is therefore **retired, not completed**: `/status` reports one as pending and the `done` gate blocks while the fold is outstanding, so the framework will not let you forget it. Two specs.

### `/consolidate` — an entire spec directory

`spec.md`, its scenarios, plan, tasks, and review.

**Reach for it when one spec replaces another and you want only the new one left.** Two triggers lead here:

- **The feature was redefined or dropped, and a new spec took its place.** The old spec is not a historical record of something that shipped — it is a description of a thing that no longer exists in that form, and keeping it means readers keep finding it.
- **The old spec was never really a separate concern.** It overlapped a sibling from the start, or duplicates one you have since written.

**The reference cleanup is the point, not a side effect.** Deleting a spec directory by hand is easy; what is hard is that every inbound pointer into it — sibling body links, scenario links a tier deeper, and the `dependencies:` edges derived from them — is now dead, and nothing in a plain `rm -rf` tells you so. `/consolidate` re-points all of them at the target *before* it removes anything, which is what the constitution's **no dead references in live artifacts** rule requires and what `rm -rf` cannot give you. The pre-commit hook's corpus-wide link check is the backstop if anything is missed.

It confirms before acting, naming the source's scenarios individually, because it **migrates no content** — it does not verify that the new spec covers what the old one said, and deliberately does not try to. If you want that check, ask your agent to compare the two specs before you confirm; that is a question an agent answers well and a poor fit for a flag. Recovery is git history. Two specs.

Reach for `/supersede` instead when the old spec *did* ship and you want the record kept: that spec stays on disk, annotated with what no longer holds.

> **This is the only one of the three that removes a durable artifact.** The other two remove something that was never meant to last: `tasks.md` sections are ephemeral work-tracking, and a branch-scoped directory is a staging form whose content `/fold` has just migrated into its durable home. Consolidation is different in kind — it **migrates nothing**. The guard proves the target exists, never that anything actually landed there, so the confirmation names the content you are losing, scenario by scenario, rather than merely naming the directory. Reach for `/supersede` instead whenever the earlier spec actually delivered something: that spec is the record of what shipped, and consolidating it would invert the relationship.

## Brownfield — absorb existing reality

### `/log` — one raw line in `specs/inbox.md`

**Reach for it when you notice something mid-task and do not want to derail to deal with it.** No triage, no routing, no decision — that is `/groom`'s job, later.

### `/groom` — every inbox item routed to its real home

**Reach for it when the inbox has accumulated and you are ready to decide where each item belongs.**

It walks the list one item at a time and routes each: a rule for a cross-cutting concern, a new spec, a scenario under an existing spec, a chore left alone, or a discard. Each route is confirmed before anything is written, and a `done` spec is reopened when an item lands under it.

## Orient

### `/target` — the session's working feature

**Reach for it when you are about to work on a different feature.** Sets the feature (or `feature/scenario`) for the session, so the other commands stop needing an argument.

### `/status` — the pipeline view of every feature

**Reach for it when you need to answer "where is everything?"** A dashboard of every feature's progress, or a focused view of the current target — including which specs are blocked, which carry unresolved scenario questions, and which have a pending fold.

### `/link` — a registered sibling service, so cross-service references resolve

**Reach for it when a spec in this repo relates to a spec in another service's repo.** Registering the service in `.ductus/config.toml` makes a cross-service reference resolve to the linked spec's real status instead of a dead link. `--list` shows registered services and their resolution health.

### `/help` — the command reference, generated from what is installed

**Reach for it when you forget which command does what.** Generated rather than hand-maintained, so it cannot describe a command set the project does not have.

## Bootstrap — one-time per project

### `/ductus` — the framework installed or updated in this project

**Reach for it when adopting the framework, or pulling the latest version of it.** The installer that placed every other command. Idempotent — safe to re-run any time.

### `/configure` — agent permissions for the `ductus` commands

**Reach for it when your agent keeps asking permission for the same `ductus` operations.** Configures the permission set so the pipeline stops prompting on every step.
