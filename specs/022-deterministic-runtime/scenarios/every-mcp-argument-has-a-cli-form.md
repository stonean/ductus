---
section: "The primitive library"
---

# Every-mcp-argument-has-a-cli-form

## Context

Each primitive "has a CLI subcommand (invokable standalone)" beside its MCP tool, but a structured argument the MCP tool takes has no CLI flag. `write-review`'s CLI takes `--feature`, `--reviewed-at`, `--reviewed-against`, `--diff-base`, `--scenario`, `--empty-scope`, `--summary`, `--skipped-pass` and `--examined` — no observations — and `process-waivers` and `process-decisions` take no findings or fired anchors (0.59.11 and HEAD f24c55c6, 2026-10-25); `write-analysis`'s findings are the same. Recording a review or an analysis outside an MCP host therefore means scripting `ductus mcp` over stdio (`initialize`, `notifications/initialized`, `tools/call`), so the standalone CLI cannot do what the command's own steps do.

On 2026-11-18, at 3e45b653, each such flag's `--help` named the argument its file carries and not that argument's shape, and `framework/bootstrap/ductus.md`'s State B `apply-manifest` paragraph names only the flags. So a host on the CLI path, which every first adoption takes before its MCP server is registered, learned an entry's fields one `missing field` error at a time: 003 task 7's scratch-adopter run F3 got through by copying a sibling run's entries file (003 task 8).

## Behavior

- Every argument a primitive's MCP tool accepts can be supplied on its CLI subcommand, so a CLI call can produce any result the MCP call can.
- A structured argument is supplied through its own `--{name}-json FILE` flag, carrying the same JSON the MCP tool takes for that argument and validated against the same schema, so the two surfaces cannot accept different inputs. The file is read only from a staging directory, as the existing `--*-json` flags of `apply-manifest` and `enforce-manifest` read theirs (`read_cli_json_file` in `runtime/src/primitives/mod.rs`, `BE-INPUT-004`).
- A primitive whose arguments are all scalar keeps its flags unchanged.
- Each `--{name}-json` flag's `--help` states the JSON its file holds: the argument's shape, its fields and their types, rendered from the schema the MCP tool publishes for that argument, so the help and the tool cannot disagree and a host builds the file from the help alone.
- No subcommand's `--help` prints a developer note: a rustdoc link or a serde attribute in a doc comment documents the code, and stays out of what the help renders.
- A test asserts that every argument each primitive's MCP tool accepts has a CLI form, beside the test that every registry primitive has a subcommand, and fails when an argument is added without one. Another fails when a `--{name}-json` flag's help omits its argument's shape, or when any subcommand's help holds a rustdoc path link or names a serde attribute.

## Edge Cases

- A JSON flag and a scalar flag that both set the same argument cannot arise: each structured argument has exactly one flag, and no scalar flag sets it.
- A `--{name}-json` file outside a staging directory: refused before it is read, naming the path, as the existing flags refuse one.
- Malformed JSON, or JSON failing the tool's schema: an error naming the field, the same error the MCP surface returns.
- An argument whose items nest another type, as `write-review`'s observations nest a disposition: the help states the nested type's fields too, so no field is left for an error to name.
- An empty observations or findings list supplied explicitly: distinct from the argument being absent where the MCP tool distinguishes them.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **What is the CLI form of a structured argument, and is the set-equality between MCP arguments and CLI flags asserted by a test?** A per-argument `--{name}-json FILE` flag on each primitive whose tool takes a structured argument, staging-bound like the five existing ones, with a test in `runtime/src/main.rs`'s tests asserting every MCP argument has a CLI form (owner, 2026-11-12). Measured at `ab2ad085`: 19 `#[arg(skip)]` fields on 7 argument structs in `runtime/src/schema/primitives.rs`, 5 already covered, so 14 new flags — `write-review` 8, `write-analysis` 3, and `process-waivers`, `process-decisions` and `prune-project` 1 each. Rejected: one `--input FILE|-` per primitive, which overlaps every scalar flag and so needs a conflict rule for each, reads stdin outside the staging bound, and, by replacing the five existing flags, rewrites `framework/bootstrap/ductus.md`'s `apply-manifest` invocation, which needs a real adopter bootstrap.
- **Does a JSON flag's `--help` state the JSON's shape?** Yes, rendered from the schema the MCP tool publishes for the argument, so the two cannot drift, with a test holding every flag to it (owner, 2026-11-18, deferring 003 task 8's finding). Rejected: listing `apply-manifest`'s entry fields in `framework/bootstrap/ductus.md`'s State B paragraph, a copy of the primitive's schema that drifts when a field is added and covers only the bootstrap's own calls.
