# Developing `ductus`

Procedures for changing this repository, each read when doing the job it names. A rule an agent needs before acting in any session stays in [`AGENTS.md`](../AGENTS.md).

## Adding or removing a runtime primitive

Registering one is six sites, and two of them are found only by tests; removing one is the same six:

- `PRIMITIVE_REGISTRY` in `runtime/src/schema/registry.rs`
- its one description, a constant in `runtime/src/schema/descriptions.rs`, which the `#[tool]` and the `Command` below name
- a `#[tool]` in `runtime/src/mcp/server.rs`, or a `CLI_ONLY_PRIMITIVES` entry
- a `dispatch_primitive` arm in `runtime/src/interpreter/mod.rs`
- a `Command` in `runtime/src/main.rs`
- a tool or a `# cli-only:` line in `framework/runtime-tools.txt`

**A result with two shapes** returns one struct with two flattened `Option` halves, and the halves must never share a key. rmcp requires an object output schema, which rules out an untagged enum (`prune-tasks`, `prune-plan`), and serde's flatten consumes a shared key even when that half fails. Keep a round-trip test per shape.

## Adding a slash command

A new command wires into five places outside `framework/commands/`, and two of them are load-bearing for adopters. The hook, Families 16 and 21 and `scripts/gen-help-tables.sh` cover four. Nothing flags the fifth: the list in `runtime/src/parser/mod.rs`'s `rewritten_command_files_parse_with_document_step_numbers`.

## Adding a member to a closed set

A family, a skip reason or a marker that `check-artifacts`' families use is a corpus sweep, and Family 18 binds only the marker lists. Before the release commit, grep an existing sibling (`root-absent`) across `framework/`, `docs/`, `runtime/src/` and `specs/`, then update every list and count naming it, or drop a needless count.

## Adding a rule category

Register a new category in its file's introducing-spec data-model in the same change, or `/ductus:analyze` rejects the whole rule file: `framework/reference/analyze.md` (*Which ID format governs a file*) reads it first, while `scripts/lint-rule-ids.sh` accepts any category. Update the data-model and the file's header format line together.

## Proving a test fails

Run unit tests in the dev profile, and rebuild the release binary after any parity mutation. `scripts/mutation/run.py` runs unit tests only; add a mutation with its killing test, and run `--check` after editing anchored code. A parity mutation is a separate `cargo test --release` run that leaves `runtime/target/release/ductus`, which `.ductus/bin/ductus` and the MCP server run, a mutant, so run `cargo build --release` after it.

**Changing what a shared predicate or a fixture's record means can hollow a test elsewhere, so run the whole list over the code it reaches.** Holding a record with no `dispositions:` map left M25's test in `scripts/mutation/scope-attribution.py` passing for that reason alone, and the mutant surviving was the only sign.

**Run a large list from `git worktree`s of the commit, not this checkout.** Its `runtime/target` grows without bound and slows every build: each mutant took 34–111 s in a checkout whose `target/debug` held 255 GB, against 3–9 s after one fresh build in a worktree (`run.py`'s per-mutant times, tasks 1 and 5 of spec 079). `run.py` finds its tree from its own path, so `git worktree add --detach <dir> HEAD` per worker and `python3 scripts/mutation/run.py <list> <ids>` from each `<dir>` split a list across workers, while this checkout stays free to edit. Commit the list and the code first, since a worktree holds only what is committed, and `git worktree remove --force` each when done. For work not yet committed, point `CARGO_TARGET_DIR` at a fresh directory instead: mutants took about 4 s each that way, and the suites follow it. The hook and the audit still run `runtime/target/release/ductus`, so build that plainly before committing, and never edit under `runtime/` while `run.py` runs.

## Writing a destination check

A destination check judges where a write lands, not only where its path leads, and compares names as the file system folds them.

- Judge the parent resolved with the final name as given, and the whole path resolved, since `write_atomic_bytes` replaces a link and never follows it.
- Compare names after `host_config::fold`.
- Write the bytes the check judged.
- Find the root among the path's resolved ancestors.
- Judge an exception's own parameter, and list every caller before granting a shared check an exception.
- Prove each refusal from the base where it alone refuses, by its own mutation, with the whole cell table beside the tests, as `runtime/src/primitives/host_config.rs` keeps it.

## Narrowing a permission rule

An exact allow rule bounds the command text, not every way to run it, so probe a narrowed rule against the commands the steps spell and against hostile forms before claiming it bounds anything.

- Claude Code strips a bare `xargs` before matching, so `echo x | xargs cp a b` runs `Bash(cp a b)` with operands appended.
- Antigravity's `command(...)` rules are token prefixes.
- zsh glob qualifiers (`*(e:'cmd':)`) run code, so a regex argument class (Auggie's `shellInputRegex`) excludes parentheses.

Test what each rule admits and refuses (separators, substitution, redirects, `xargs`, zsh qualifiers, `run-generator`, `exec`; `re.search` for regexes), and disclose what it still admits.

## Exercising an installer change without a push

Drive `apply-manifest` and `enforce-manifest` from a staging copy of `framework/`: `cp -R framework` to a `mktemp -d` `--source-root`, and build `--entries-json` from its `bootstrap/ductus.md`. On `main`, a subagent (no nested CLI) runs `install.sh` in scratch, with no `mcp__ductus__*`.
