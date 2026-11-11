# /acme-smoke

A minimal procedure exercising the parameterized command-resolution
path. The fixture omits `framework/commands/` entirely so the only way
for `ductus exec smoke` to find this file is via the installed candidate
(`{cli-config-dir}/prompts/{project}-<name>.md`), which expands to
`.pi/prompts/acme-smoke.md` after reading `.govern.toml`'s
`[host]` block.

## Instructions

1. Invoke `read-spec` against the targeted feature.
