---
section: "The primitive library"
---

# Every-mcp-argument-has-a-cli-form

## Context

Each primitive "has a CLI subcommand (invokable standalone)" beside its MCP tool, but a structured argument the MCP tool takes has no CLI flag. `write-review`'s CLI takes `--feature`, `--reviewed-at`, `--reviewed-against`, `--diff-base`, `--scenario`, `--empty-scope`, `--summary`, `--skipped-pass` and `--examined` — no observations — and `process-waivers` and `process-decisions` take no findings or fired anchors (0.59.11 and HEAD f24c55c6, 2026-10-25); `write-analysis`'s findings are the same. Recording a review or an analysis outside an MCP host therefore means scripting `ductus mcp` over stdio (`initialize`, `notifications/initialized`, `tools/call`), so the standalone CLI cannot do what the command's own steps do.

## Behavior

- Every argument a primitive's MCP tool accepts can be supplied on its CLI subcommand, so a CLI call can produce any result the MCP call can.
- A structured argument is supplied as the same JSON the MCP tool takes — from a file or from stdin — and is validated against the same schema, so the two surfaces cannot accept different inputs.
- A primitive whose arguments are all scalar keeps its flags unchanged.

## Edge Cases

- A JSON input and a flag that both set the same argument: refused, naming the argument, rather than one silently winning.
- Malformed JSON, or JSON failing the tool's schema: an error naming the field, the same error the MCP surface returns.
- An empty observations or findings list supplied explicitly: distinct from the argument being absent where the MCP tool distinguishes them.

## Open Questions

- What is the CLI form of a structured argument: one `--input FILE` (with `-` for stdin) carrying the whole MCP argument object for every primitive, or a per-argument `--{name}-json FILE` flag only on the primitives whose tools take one — and is the set-equality between MCP arguments and CLI flags asserted by a test, the way AC28 asserts registry and subcommand?

## Resolved Questions

*None yet.*
