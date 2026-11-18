---
title: "003-bootstrap-automation — tasks"
next-task: 9
---

# 003 — Bootstrap Automation Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 5. Implement scenario: a-language-with-no-template-of-its-name

- [x] Resolve the scenario's open question with `/ductus:clarify`
- [x] Implement the behavior described in `scenarios/a-language-with-no-template-of-its-name.md`
- [x] `framework/bootstrap/ductus.md`'s `.gitignore` merge step 5, and its byte copy `framework/bootstrap/govern.md`, carry the decided rule and name each unappended language in the closing summary
- [x] A scratch-adopter bootstrap naming TypeScript shows the decided outcome

- **Done when**: the scenario's open question is resolved, the step follows it, and a scratch-adopter bootstrap naming TypeScript ends as the scenario states.

## 7. Exercise the shared bootstrap window's review fixes in a scratch-adopter bootstrap

- [ ] A first adoption naming `TypeScript, JavaScript` appends GitHub's Node template once, which records the run evidence the step requires of the `JavaScript → Node` map entry (the window review's contract break on 1fa264dc)
- [ ] A `[project] languages` value holding `/` halts the run at Collect Project Inputs, and one holding `#` or a space reaches the fetch percent-encoded (1fa264dc, cfb348b8)

- **Done when**: one batched run, per `docs/developing-ductus.md` §Exercising an installer change without a push, shows each outcome, which the commit ticking this task states; a defect it finds is fixed first, and the review of the shared window is written after

## 8. Disposition out-of-spec finding: the CLI's JSON-file flags never state their JSON's shape

- [x] runtime/src/schema/primitives.rs — each `--{name}-json` flag's help names the argument it holds and not its shape, so a host on the State B path, which every first adoption takes, learns an `apply-manifest` entry's fields one `missing field` error at a time (`framework/bootstrap/ductus.md`'s State B `apply-manifest` paragraph names only the flags); found by 003 task 7's run F3, whose host copied a sibling run's entries file — deferred to `specs/022-deterministic-runtime/scenarios/every-mcp-argument-has-a-cli-form.md`, 022 task 53 (owner, 2026-11-18)
- [ ] runtime/src/schema/primitives.rs — `ductus <subcommand> --help` renders developer notes literally: over the 78 subcommands the top-level help lists, at the release build of 3e45b653's runtime source, 10 subcommands' help holds 20 lines with a rustdoc path link (such as `[`ApplyManifestArgs::entries_json`]`) and 17 lines naming `#[serde(skip)]`

- **Done when**: each finding is fixed, deferred, or waived, with a waiver's reason written on the task
