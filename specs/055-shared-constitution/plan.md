# 055 — Shared Constitution Plan

Implements [055 — Shared constitution](spec.md).

## Overview

An alias-keyed `[constitutions.<alias>]` registry in `.ductus/config.toml`, read the
same way `[services]` already is; one new runtime primitive, `resolve-constitutions`,
that resolves each entry to a document on disk and reports what it could not resolve;
`/ductus:target`'s existing once-per-session constitution load extended to every
resolved document; and a layout-derived managed block in the adopter's native rules
file so the document is also reachable by navigation.

Nothing is fetched, nothing is copied, and `.ductus/constitution.md` is never
rewritten. The feature is a read of local disk plus a report.

## Technical Decisions

### The registry reader is `load_services` with a different table name

`runtime/src/primitives/resolve_references.rs:94` (`load_services`) is the model:
it resolves the config path through `paths::config_path(repo)`, treats an absent
file as an empty registry, and maps a malformed one to `PrimitiveError::Toml`.
`classify` (`:107`) then resolves the checkout with `resolve_path(repo, &service.path)`
— `..` is permitted, documented at `:121-124` as "a sibling checkout is the normal
case (`path = "../api"`), and this is machine-local config, not an LLM-supplied
path" — and treats `!checkout.is_dir()` as `NotCheckedOut` rather than an error.

`[constitutions.*]` takes that path unchanged. The `not-checked-out` semantics AC5
and AC8 depend on therefore come from an existing, exercised code path rather than
a newly invented one, and the two registries stay legible to each other.

### One new primitive: `resolve-constitutions`

It returns `loaded` (alias, checkout path, document path) and `skipped` (alias,
reason). The reason distinguishes `not-checked-out` from `no-constitution-document`,
because the spec's Edge Cases require that "an operator who cloned the wrong
repository must not read the same message as one who cloned nothing."

That examined/skipped split is the shape `derive-routing-candidates` and
`derive-dependencies` already return, and it is what makes AC8 mechanical instead of
a discipline: a caller cannot report a clean result over a source the primitive
placed in `skipped` without dropping a field.

Ordering is alias order, from the registry's `BTreeMap`. AC7 requires two projects with
the same entries to load the same documents in the same order on any machine — alias
order delivers that, while config order would not survive a TOML reformat and hash-map
traversal order is not stable at all.

**Wiring spans eight sites, not two** (`AGENTS.md:99`). `AGENTS.md` names
`parser/mod.rs` for `PRIMITIVE_NAMES`, but that constant is *defined from*
`PRIMITIVE_REGISTRY` (`runtime/src/schema/registry.rs:16`), which is the single
source feeding both it and the server's `TOOL_NAMES` — so the registry is the site
and the parser needs no edit. The set: `runtime/src/schema/constitutions.rs` (new,
modelled on `services.rs`) plus its `pub mod` in `schema/mod.rs`;
`runtime/src/schema/primitives.rs` (Args/Result);
`runtime/src/primitives/resolve_constitutions.rs` plus its `pub mod`;
`runtime/src/schema/registry.rs`; `runtime/src/mcp/server.rs` (the hand-written
`#[tool]` method); `runtime/src/interpreter/mod.rs` (`dispatch_primitive` arm);
`runtime/src/main.rs` (`Command` variant + dispatch); and
`framework/runtime-tools.txt`.

### `/ductus:target` is the load mechanism; the managed block is navigation

`framework/commands/target.md:37` already states: "Load the constitution file once
per session to make its sections available for subsequent commands. (Host
responsibility — no primitive reads the constitution.)" Every other command's Scope
Boundaries then says "constitution loaded by `/ductus:target` — do not re-read."

Extending that one step to also load each `loaded` document satisfies AC2 for every
command with no per-command edit, and it is agent- and layout-independent — which
matters, because the import line is not.

`framework/templates/project/claude-md.md:3` uses `@import .ductus/constitution.md`,
but that is a `claude-style` feature. The Agent Registry's derived-values table gives
`antigravity` and `opencode` a native rules file of `AGENTS.md`, and
`framework/templates/project/agents.md:9` is a plain link — it points at
`.ductus/constitution.md` in prose — not an import. So AC9's managed block
is **layout-derived**: an `@import` line under `claude-style`, a `See [alias](path)`
link under the `AGENTS.md` layouts. Treating the import as the loading mechanism
would leave two of the four registered agents loading nothing.

### The managed block uses `merge-managed-block`

`CLAUDE.md` and `AGENTS.md` are both strategy `skip` in
`framework/bootstrap/ductus.md` §Shared files with conflict handling — adopter-owned,
never overwritten. The `.gitignore` merge in that same section is documented as
mirroring "the runtime `merge-managed-block` contract (line-prefix style, marker
`ductus`)", so the same primitive under a distinct marker gives AC9's byte-for-byte
preservation of everything outside the block without a second mechanism.

### Validation happens at configuration time, and adds no command

`/ductus` §Project Configuration validates each entry when it reads the config:
alias is a bare TOML key, `repo` is URL-shaped, `path` is non-empty. A `path` that
does not resolve is a **warning**, never a rejection — `framework/commands/link.md:65`
settles that shape for `[services]` and the spec adopts it.

No `/ductus:link`-style command is added. Registration is a hand-edit plus `/ductus`
validation, which is what `link.md:25` already says for the analogous case
("Removal and edits of an existing entry stay hand-edits to `.ductus/config.toml`").

### Both bootstrap twins take every edit

`framework/bootstrap/govern.md` is byte-identical to `framework/bootstrap/ductus.md`
(verified with `diff -q`; audit Family 21 pins it). Every bootstrap edit is followed
by `cp framework/bootstrap/ductus.md framework/bootstrap/govern.md`.

### Editing `target.md` stales a parity golden

Per `AGENTS.md:116`, `stage_fixture` copies `framework/commands/<cmd>.md` into the
fixture tree before its first commit, so editing a command source changes that tree's
shas and the `implement-basic` golden records them. After the `target.md` edit, re-run
`cargo test --release --locked --test parity`, re-bless if it fails, and verify the
diff is shas only before committing.

### AC8 is a runtime change, not a markdown edit

Discovered during implementation, and recorded because the plan said otherwise.
`write_review.rs` renders the report's sections in Rust — review.md states that the
primitive emits them so both paths produce byte-identical reports — so adding an
**Unexamined governance** section is a runtime change. Editing prose alone could
only have produced a report-contract sentence nothing enforces, which is the
diligence dependency AC8 exists to prevent.

Both primitives **resolve the registry themselves** rather than taking it as an
argument. That is what keeps the guarantee mechanical: a caller that had to supply
the skipped set could omit it. It also avoided a new dispatch step in review.md and
analyze.md, and therefore avoided renumbering their steps and staling every prose
cross-reference to those numbers.

### Precedence is documented, never enforced

AC10 is a documentation commitment on purpose. Prose precedence between governance
documents cannot be checked, and the constitution is explicit that claiming otherwise
is itself the defect: "A rule that implied enforcement it does not have would be
committing the defect it describes" (§grounding). The statement lands once in the
config-schema documentation and says plainly that nothing enforces it.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/schema/constitutions.rs` | Create | `[constitutions]` registry shape + parser |
| `runtime/src/schema/mod.rs` | Modify | `pub mod constitutions` |
| `runtime/src/schema/primitives.rs` | Modify | `ResolveConstitutionsArgs` / `Result` types |
| `runtime/src/primitives/resolve_constitutions.rs` | Create | Registry read, path resolution, loaded/skipped split |
| `runtime/src/primitives/mod.rs` | Modify | `pub mod resolve_constitutions` |
| `runtime/src/mcp/server.rs` | Modify | `TOOL_NAMES` entry + `#[tool]` method |
| `runtime/src/schema/registry.rs` | Modify | `PRIMITIVE_REGISTRY` entry (feeds `PRIMITIVE_NAMES` + `TOOL_NAMES`) |
| `runtime/src/interpreter/mod.rs` | Modify | `dispatch_primitive` match arm |
| `runtime/src/main.rs` | Modify | `Command` variant + CLI dispatch |
| `framework/runtime-tools.txt` | Modify | `lint-tool-coverage` coverage |
| `framework/commands/target.md` | Modify | Step 4 loads registered constitutions |
| `runtime/src/primitives/write_review.rs` | Modify | Render `## Unexamined governance` (AC8) |
| `runtime/src/primitives/write_analysis.rs` | Modify | Record `constitution-unresolved` (AC8) |
| `framework/commands/review.md` | Modify | Document the new report section (AC8) |
| `framework/commands/analyze.md` | Modify | Document the informational tier entry (AC8) |
| `framework/bootstrap/ductus.md` | Modify | Config schema, validation, managed-block step |
| `framework/bootstrap/govern.md` | Modify | Byte-identical copy of the above (Family 21) |
| `framework/constitution.md` | Modify | Precedence statement + schema pointer (AC10) |
| `framework/templates/project/claude-md.md` | Modify | Managed-block marker for `claude-style` |
| `framework/templates/project/agents.md` | Modify | Managed-block marker for `AGENTS.md` layouts |
| `.claude/commands/ductus/*.md` | Regenerate | `scripts/gen-claude-commands.sh` output |
| `specs/055-shared-constitution/data-model.md` | Create | `[constitutions.*]` schema (canonical) |

## Data Model

The `[constitutions.<alias>]` table schema is declared in [data-model.md](data-model.md),
which is its canonical home — the same split `030-cross-service-references` uses for
`[services]`.

## Trade-offs

**Considered and rejected: a fetching transport.** Spec 015's `codeload` archive fetch
already exists and would let ductus acquire a governance repo directly. Rejected during
clarification: it adds a transport, an authentication story for private repos, and a
network failure mode to a read that is otherwise pure local disk. `[services]` set the
never-fetch precedent and this follows it.

**Considered and rejected: merging into `.ductus/constitution.md`.** One assembled
document reads better for a contributor, but it turns a manifest-shipped file into a
generated one, collides with the `update` strategy and `[pinned]`, and creates the
second copy §drift-prevention exists to prevent.

**Considered and rejected: a `ref` field compared against the checkout's `HEAD`.**
It would detect contributor skew — one contributor pulls the governance repo, another
does not — for the cost of one field and one git read. Declined during clarification
in favour of matching `[services]`' no-pin posture.

**Known limitation: contributor skew is silent.** Following directly from the above.
Nothing detects two contributors running under different revisions of the same shared
constitution.

**Known limitation: opt-out is all-or-nothing.** A shared constitution is imported
whole; prose carries no per-statement handle, so a project that rejects one rule must
stop registering the whole document or take it upstream.

**Known limitation: precedence is unenforced.** Stated in documentation, resolved by
whoever reads the contradiction. No check exists or is planned.
