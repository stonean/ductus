# 075 — AGENTS.md holds what every session needs Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 1. Record the owner scenarios

Recording a scenario on `063` and `055` takes each from `done` to `in-progress`, and that transition needs the owner's approval first.

- [x] Run `/ductus:amend` on `063-oversized-artifacts-warn-with-a-fix`, recording a scenario for the project subject set, the staged mode, and the `/analyze` advisory for project files, as the plan's `check-artifact-size` decision states. Link it to this spec. Its appended task waits on `041`'s, because the advisory names `/prune --project`, and `--ref=main` adopters run whatever is on `main`.
- [x] Run `/ductus:amend` on `041-task-pruning`, recording a scenario for `/prune --project`, the `prune-project` primitive, and step 8's routing, as the plan's `/prune --project` decision states. Link it to this spec.
- [x] Run `/ductus:amend` on `055-shared-constitution`, recording a scenario for `/target`'s size report and the measuring wrapper. Link it to this spec.
- [x] Run `/ductus:amend` on `050-constitution`, recording a scenario for AC1's routing text and AC13's tree lines, the byte cut that offsets them, and the rewrite of `artifacts-are-as-short-as-their-required-content`. Link it to this spec.

- **Done when**: each of the four specs has a scenario that links `../../075-agents-md-holds-what-every-session-needs/spec.md`, each scenario's task is appended to its spec's `tasks.md`, and `063` and `055` are `in-progress`.

## 2. Rewrite the memory-routing block

- [ ] Replace the one line in `framework/bootstrap/ductus.md` §Memory routing, step 1, so it names `AGENTS.md` for what every session needs, and names each other home with when to read it.
- [ ] Copy the change into `framework/bootstrap/govern.md` byte for byte.
- [ ] Record each layout's impact in the commit (`claude-style`, `antigravity`, `opencode`, `pi`), walking `framework/bootstrap/ductus.md` §Derived values' Layout-derived rows.

- **Done when**: AC2 holds, `scripts/audit/transitional-bootstrap-parity.sh` reports the two installers identical, and markdownlint passes on both.

## 3. Point the templates at the constitution

- [ ] In `framework/templates/project/agents.md`, point the header at the constitution's four-part test, and drop the instruction to append a project-only rule to its matching section.
- [ ] In `framework/templates/project/agents.md`, replace the Project Structure's `specs/` subtree with a pointer to §spec-phase.
- [ ] Point the headers of `framework/templates/project/system.md`, `events.md` and `errors.md` at the §spec-phase tree, restating none of it.
- [ ] Make the same header change in this repository's `AGENTS.md`, which carries the same append instruction.
- [ ] Record each layout's impact in the commit.

- **Done when**: AC3, AC4 and AC9 hold against the tree, and this repository's `AGENTS.md` stays within `scripts/lint-governing-size.sh`'s ceiling.

## 4. Document the limit's wider scope

Blocked on `063-oversized-artifacts-warn-with-a-fix`'s task 15, since these entries describe the limit that task makes the check apply.

- [ ] Update `README.md`'s `[artifacts]` entry to say `read-size-bytes` also limits the four project files, `AGENTS.md` among them even though it loads in every session rather than through a file read.
- [ ] Make the same change to the commented schema in `framework/bootstrap/ductus.md` §Project Configuration, and copy it into `framework/bootstrap/govern.md`.

- **Done when**: AC16 holds, and 063's AC17 still holds: the default, the whole-number form, and no off switch are each still stated.

## 5. Refuse growth in the shipped hook

Blocked on `063-oversized-artifacts-warn-with-a-fix`'s task from task 1, because the hook calls the staged mode that task builds. Also blocked on `041-task-pruning`'s task, because the refusal names `/prune --project`.

- [ ] Add `check-artifact-size --project-files --staged` to `framework/bootstrap/hooks/ductus-pre-commit` after the re-stage block, beside `check-corpus-links`.
- [ ] Add a capability probe for the flag: capture `check-artifact-size --help` into a variable, check it for `--project-files`, and halt with the too-old message when the flag is missing.
- [ ] Add Family 22 cases to `scripts/audit/adopter-shell-behavior.sh`. A commit that grows an over-limit file, and one that adds a new file over the limit, are each refused, with the file, size, limit and `/prune --project` named. A commit that shrinks the file, leaves it unchanged, or deletes it passes.
- [ ] Show that the cases fail with the hook step removed.
- [ ] Record each layout's impact in the commit.

- **Done when**: AC14 holds, Family 22 passes, and Family 22 fails when the step is removed.

## 6. Sweep the prior claims

- [ ] Grep the live-artifact set (`AGENTS.md`, Workflow) by meaning for the retired routing ("this file or a rule file", "append a project-only rule", "contributor knowledge into `AGENTS.md`").
- [ ] Grep the same set for every statement that `check-artifact-size` measures spec artifacts only, or runs only in `/clarify`, `/plan` and `/analyze`, including the schema comment's *Consumed by* line and `docs/`.
- [ ] Correct each hit that states current behavior. A hit in a `done` spec's body is a meaningful edit, and its back-edge is named before the write.

- **Done when**: no live artifact states the prior routing or the spec-only subject set as current behavior.

## 7. Verify the criteria

Blocked on the tasks task 1 appends to `063`, `041`, `055` and `050`, and on tasks 2–6. Those tasks are `063` task 15, `041` task 23, `055` task 14 and `050` task 44.

- [ ] Read each of this spec's criteria against the tree, including AC6, AC11 and AC12's recorded `claude -p` run over a fixture project under `/tmp`.
- [ ] For AC7, confirm that `041`'s walk landed on `main` no later than the size check and the hook step. Confirm with `git tag --contains` that no `ductus-v*` tag holds either commit without the other.
- [ ] Confirm that each commit landing the block, the templates and the size check records every layout's impact (AC8).
- [ ] Run every step of `.github/workflows/framework-checks.yml`, `generators.yml` and `runtime.yml` locally, and run `scripts/audit/run-all.sh` after committing.

- **Done when**: every criterion holds against the tree and the local gate and the audit pass. The release follows the specs reaching `done`, per `AGENTS.md`'s Workflow.
