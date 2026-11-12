---
section: "The primitive library"
---

# Every-mcp-argument-has-a-cli-form

## Context

Each primitive "has a CLI subcommand (invokable standalone)" beside its MCP tool, but a structured argument the MCP tool takes has no CLI flag. `write-review`'s CLI takes `--feature`, `--reviewed-at`, `--reviewed-against`, `--diff-base`, `--scenario`, `--empty-scope`, `--summary`, `--skipped-pass` and `--examined` — no observations — and `process-waivers` and `process-decisions` take no findings or fired anchors (0.59.11 and HEAD f24c55c6, 2026-10-25); `write-analysis`'s findings are the same. Recording a review or an analysis outside an MCP host therefore means scripting `ductus mcp` over stdio (`initialize`, `notifications/initialized`, `tools/call`), so the standalone CLI cannot do what the command's own steps do.

## Behavior

- Every argument a primitive's MCP tool accepts can be supplied on its CLI subcommand, so a CLI call can produce any result the MCP call can.
- A structured argument is supplied through its own `--{name}-json FILE` flag, carrying the same JSON the MCP tool takes for that argument and validated against the same schema, so the two surfaces cannot accept different inputs. The file is read only from a staging directory, as the existing `--*-json` flags of `apply-manifest` and `enforce-manifest` read theirs (`read_cli_json_file` in `runtime/src/primitives/mod.rs`, `BE-INPUT-004`).
- A primitive whose arguments are all scalar keeps its flags unchanged.
- A test asserts that every argument each primitive's MCP tool accepts has a CLI form, beside the test that every registry primitive has a subcommand, and fails when an argument is added without one.

## Edge Cases

- A JSON flag and a scalar flag that both set the same argument cannot arise: each structured argument has exactly one flag, and no scalar flag sets it.
- A `--{name}-json` file outside a staging directory: refused before it is read, naming the path, as the existing flags refuse one.
- Malformed JSON, or JSON failing the tool's schema: an error naming the field, the same error the MCP surface returns.
- An empty observations or findings list supplied explicitly: distinct from the argument being absent where the MCP tool distinguishes them.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **What is the CLI form of a structured argument, and is the set-equality between MCP arguments and CLI flags asserted by a test?** A per-argument `--{name}-json FILE` flag on each primitive whose tool takes a structured argument, staging-bound like the five existing ones, with a test in `runtime/src/main.rs`'s tests asserting every MCP argument has a CLI form (owner, 2026-11-12). Measured at `ab2ad085`: 19 `#[arg(skip)]` fields on 7 argument structs in `runtime/src/schema/primitives.rs`, 5 already covered, so 14 new flags — `write-review` 8, `write-analysis` 3, and `process-waivers`, `process-decisions` and `prune-project` 1 each. Rejected: one `--input FILE|-` per primitive, which overlaps every scalar flag and so needs a conflict rule for each, reads stdin outside the staging bound, and, by replacing the five existing flags, rewrites `framework/bootstrap/ductus.md`'s `apply-manifest` invocation, which needs a real adopter bootstrap.
