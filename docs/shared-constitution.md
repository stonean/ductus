# Shared constitutions

The deep reference for sharing one governance document across several projects. The README's [Shared constitutions](../README.md#shared-constitutions) section covers what the feature is and when you want it; this is where the registry schema, the loading order, the resolution outcomes, and the deliberate limits live.

See [055 — Shared constitution](../specs/055-shared-constitution/spec.md) for the originating spec and [its data model](../specs/055-shared-constitution/data-model.md) for the `[constitutions]` schema.

## The problem it solves

`ductus` ships one constitution to every adopter. That covers rules true for *any* project running the pipeline. It never covered rules true for **one organization's** projects and nobody else's — a house code style, a deployment gate, a review convention, a naming standard.

Before this, an organization had two options and both were bad:

- **Retype the rule into every repo's `AGENTS.md`.** That file is project-owned and ships to nobody, so the rule was written once per repository and drifted independently from then on. Nothing detected the divergence, because from inside any one repo there was nothing to diverge from.
- **Hand-edit `.ductus/constitution.md` and pin it.** Adding the path to `[pinned] files` stops `/ductus` overwriting your edit — and stops it delivering *every future framework update* too. The price of one house rule was all of them.

A shared constitution is a third option that costs neither.

## Registering one

Add a `[constitutions.<alias>]` table to `.ductus/config.toml`. This is a hand-edit — there is no `/link`-style command for it, matching `/link`'s own rule that edits to an existing entry stay hand-edits.

```toml
[constitutions.acme]
repo = "https://github.com/acme/governance"
path = "../governance"
description = "Acme engineering house rules"
```

| Field | Required | Meaning |
| --- | --- | --- |
| `<alias>` | yes | A bare TOML key — letters, digits, hyphens, underscores. How the source is named in every report. |
| `repo` | yes | Canonical repository URL. **Identity and navigation only — never fetched.** |
| `path` | yes | Local checkout, relative to the repo root or absolute. `..` is fine; a sibling checkout is the normal case. **The only thing read.** |
| `description` | no | Free text. Informational; no behavior depends on it. |

**More than one may be registered**, so an organization-level and a team-level source can layer. They load in alias order — stable across machines, and unaffected by how the TOML happens to be written.

`ductus` reads exactly one file out of a registered checkout: `{path}/constitution.md`. Nothing else. Not its `rules/`, and **not its own `.ductus/config.toml`** — registration is not transitive, so a shared constitution cannot pull in further constitutions. There is no recursion to bound and no cycle to detect.

## Nothing is fetched

This is the design decision everything else follows from. `repo` is recorded verbatim for identity and navigation; `ductus` never requests it. The governance repository is a checkout **you** clone and **you** update, exactly like a `[services]` entry.

That means there is no transport, no authentication story for a private governance repo, no network failure mode, and no proxy configuration. Offline is the normal case, not a degraded one.

It also means there is no version pin. The checkout's own git state *is* the version. **The accepted cost is that contributor skew is silent**: if you pull the governance repo and a teammate does not, you are running under different rules and nothing detects it. Keeping contributors in sync is the organization's job — the same posture `[services]` takes.

## What travels

**Prose only** — the `constitution.md` document. Rule files are explicitly out of scope.

The rule tier has a structurally similar gap (a rule file a project authors registers only in the repo holding it), and a later spec may extend this same registry to a `rules/` directory in the checkout. Nothing here forecloses that. It is simply not what this feature does.

## How it loads

`/ductus:target` loads the shipped constitution and every resolved shared document **once per session**, so every subsequent command inherits them with no per-command step. That is the loading mechanism, and it is the same for every agent and every layout.

The import line in your `CLAUDE.md` or `AGENTS.md` is **navigation, not loading**. `/ductus` maintains it inside a managed block:

```markdown
<!-- ductus:constitutions -->
@import ../governance/constitution.md
<!-- /ductus:constitutions -->
```

The block's content is layout-derived, because the file is. Claude-style agents read `CLAUDE.md` and get `@import` lines; Antigravity and OpenCode read `AGENTS.md`, which has **no import directive at all**, so they get plain links. Only the region between the markers is rewritten — the rest of your file is preserved byte-for-byte, and `.ductus/constitution.md` is never touched.

An adopter whose rules file has no block still gets the rules, because `/ductus:target` is what loads them.

## Which document wins

The framework constitution is a **floor**: a shared or project source may add rules and tighten existing ones, never loosen them. Where the framework is silent, the more specific source wins — project > shared > framework.

The canonical statement, with the explicit note that **nothing enforces it**, is [§governance-precedence](../framework/constitution.md#governance-sources-and-which-governs) in the constitution itself. Prose contradiction is not mechanically detectable; the order is the rule for whoever resolves a disagreement, not a check.

There is also **no per-rule opt-out**. A shared constitution is imported whole, and prose carries no per-statement handle to disable. A project that will not accept one of its rules stops registering that constitution, or takes it up with the publisher. (This differs from rule *files*, which a project can stand down individually via `[[review.disabled-rule-files]]`.)

## Resolution outcomes

Each registered entry resolves to exactly one outcome.

| Outcome | Condition |
| --- | --- |
| `loaded` | `path` resolves to a directory holding `constitution.md` |
| `not-checked-out` | `path` does not resolve to a directory |
| `no-constitution-document` | `path` resolves, but holds no `constitution.md` |

The two failures are deliberately distinct: cloning nothing and cloning the *wrong* repository are different mistakes, and collapsing them into one message would send you looking in the wrong place.

Neither is an error. A missing checkout is a valid state — correct for any teammate who has not cloned the governance repo yet — so `ductus` **warns and continues**. Blocking would make your pipeline a hard dependency on someone else's repo state.

## An unread source is never a clean result

Warning and continuing has an obvious failure mode: the run proceeds under fewer rules than your config declares, and the output looks normal. `ductus` closes that directly.

- **`/ductus:review`** renders an **Unexamined governance** section in `review.md`, naming each source it could not read and its reason. The primitive that writes the report resolves the registry itself, so the section cannot be omitted.
- **`/ductus:analyze`** records the same under `constitution-unresolved` in the `analyze:` record's `unexamined-by-reason` breakdown, where the pre-`done` gate and any later reader will see it.

This is `QUAL-CLAIM-001` applied to the review's own inputs: a result must distinguish *examined and found nothing* from *could not examine*. A project with nothing registered renders `*None.*` and reads exactly as it did before.

## What it deliberately does not do

- **Fetch anything.** The checkout is yours to obtain and update.
- **Pin a version.** No `ref` field; contributor skew is silent and accepted.
- **Carry rule files or templates.** Prose only.
- **Support a shared `AGENTS.md`.** That file is the project-only tier by definition — a convention true across five repos belongs in a shared constitution, with `AGENTS.md` keeping a pointer.
- **Validate the shared document.** Nothing checks it, and nothing checks the framework's own constitution prose either. Whether your organization's constitution is any good is your organization's problem.
- **Enforce precedence.** Stated, not checked.
