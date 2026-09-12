# Shared constitutions

The deep reference for sharing one governance document across several projects. The README's [Shared constitutions](../README.md#shared-constitutions) section covers what the feature is and when you want it; this is where the registry schema, the loading order, the resolution outcomes, and the deliberate limits live.

See [055 — Shared constitution](../specs/055-shared-constitution/spec.md) for the originating spec and [its data model](../specs/055-shared-constitution/data-model.md) for the `[constitutions]` schema.

## The problem it solves

`ductus` ships one constitution to every adopter. That covers rules true for *any* project running the pipeline. It never covered rules true for **one organization's** projects and nobody else's — a house code style, a deployment gate, a review convention, a naming standard.

Before this, an organization had two options and both were bad:

- **Retype the rule into every repo's `AGENTS.md`.** That file is project-owned and ships to nobody, so the rule was written once per repository and drifted independently from then on. Nothing detected the divergence, because from inside any one repo there was nothing to diverge from.
- **Hand-edit `.ductus/constitution.md` and pin it.** Adding the path to `[pinned] files` stops `/ductus` overwriting your edit — and stops it delivering *every future framework update* too. The price of one house rule was all of them.

A shared constitution is a third option that costs neither.

## What belongs in a shared constitution

A shared constitution sits in a gap with a tier on each side, and the test is which of the three a rule is true for.

| True for… | Home |
| --- | --- |
| One of your projects | that project's `AGENTS.md` |
| **Several of your projects, but not everyone's** | **your shared constitution** |
| Any project running the pipeline | contribute it upstream to the framework constitution |

The middle row is the whole point. The top row is why `AGENTS.md` exists and why this feature does not ship a shared one — a convention true across five repositories is not project-only by definition. The bottom row is worth taking seriously rather than treating as theory: if a rule is genuinely true for anyone running `ductus`, putting it in your org's constitution means every other adopter pays to learn it independently, which is the asymmetry the framework's own promotion mechanism exists to close.

**Apply the reword test.** A rule belongs at this tier only if you can state it without naming machinery that only one of your projects has. "Handlers must call `AcmeAuth.verify()` before touching the session store" names one service's class; "every request-handling entry point must authenticate before reading session state" is the same rule at the tier that actually applies. If a rule cannot survive the rewording without losing what makes it actionable, it was a project rule.

Good candidates look like: a house code style with a rationale, a deployment or release gate, a review convention, a naming standard, a decision-record practice, an on-call or incident expectation that shapes how code is written.

**What cannot work here**, as distinct from what is merely a bad fit:

- **Anything that loosens a framework rule.** The framework constitution is a floor. "Skip the review gate for hotfixes" will not take effect and will quietly mislead anyone who reads it.
- **Rule files.** Prose only — IDs, Verification steps, and the `{surface}-{category}-{NNN}` grammar belong to the rule tier, which this feature does not carry. A rule you want `/review` to enforce mechanically stays a per-repo rule file for now.
- **Machine-local or contributor-specific facts.** The document is committed and read by everyone; paths, credentials, and personal tooling preferences are none of its business.

**Writing it:**

- **Own your own section anchors.** You can add `§acme-deploy-gate`; you cannot redefine `§grounding`. Each document owns its anchors, so a citation elsewhere in the framework keeps meaning what it meant.
- **Say who it governs, at the top.** An agent loads it next to the framework constitution with no other context. One opening line naming the organization and who owns the document prevents it being read as a framework document.
- **State the rule *and* its reason.** The framework constitution carries the reasoning for its rules because a rule whose rationale is unstated gets argued with at the worst moment. Yours will be read the same way.
- **Keep it tight.** It is loaded once per session, into the context of every command, alongside a framework constitution that is already large. The cost of a page of exposition is paid on every invocation by every contributor — prefer the rule and its reason to the essay.

## Setting one up

**1. Create the governance repository.** Any git repo will do. Put the document at its **root**, named exactly `constitution.md` — that filename is fixed, not configurable, so that two projects registering the same source cannot end up reading different documents from it.

```text
acme/governance
└── constitution.md
```

The repo needs no `ductus` install, no specs directory, and no pipeline of its own. It is read, never run.

**2. Agree a checkout layout — this is the step teams get wrong.** `.ductus/config.toml` is **committed and team-shared**, so the `path` you register is the path *every* contributor resolves. Use a **relative sibling path** and have everyone clone to the same shape:

```text
~/src/acme/
├── governance/     ← the shared constitution
├── api/            ← path = "../governance"
└── web/            ← path = "../governance"
```

An **absolute path resolves on exactly one machine** and reports `not-checked-out` for everyone else. Record the expected layout in each project's `AGENTS.md` so a new contributor clones correctly the first time — a teammate who clones the governance repo somewhere else is not broken, but they silently run under fewer rules until they notice.

**3. Register it in each project.** Hand-edit `.ductus/config.toml` and commit — this is team-shared config, not local setup:

```toml
[constitutions.acme]
repo = "https://github.com/acme/governance"
path = "../governance"
description = "Acme engineering house rules"
```

**4. Run `/ductus` in each project.** That installs or refreshes the managed import block in the project's native rules file. It is not what loads the rules — `/target` does that — so you can skip it and still be governed; you just lose the pointer a contributor sees when they open the file.

**5. Verify.** From the project root:

```console
$ ductus resolve-constitutions
{"loaded":[{"alias":"acme","repo":"https://github.com/acme/governance",
  "path":"../governance","document":"../governance/constitution.md",
  "outcome":"loaded"}],"skipped":[],"examined":1,"duplicate-paths":[]}
```

`examined` is the count of registered entries; anything in `skipped` carries the reason it did not resolve. `/target` reports the same at the start of a session.

**6. Updating.** Pull the governance repo. There is no pin and no notification — see [Nothing is fetched](#nothing-is-fetched) for why, and for the skew this accepts. If your organization cares about contributors converging, the practical lever is a periodic reminder or a shared setup script, not a `ductus` setting.

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
| `description` | no | Free text: what this source governs. It changes nothing about which documents load, and it is shown wherever `ductus` names the source — when `/{project}:target` reports the sources it loaded, when it reports one it could **not** load, and in a review's **Unexamined governance** section. Write one: an alias alone does not tell a contributor what a document governs, and it is the missing-checkout message where the note helps most. |

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
