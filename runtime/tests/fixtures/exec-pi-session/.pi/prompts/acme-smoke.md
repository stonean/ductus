# /acme-smoke

A minimal procedure exercising the parameterized command-resolution
path for the `pi` layout. The fixture omits `framework/commands/`
entirely so the only way for `ductus exec smoke` to find this file is via
the installed candidate, which for Pi expands to the flat
`{cli-config-dir}/prompts/{project}-<name>.md` —
`.pi/prompts/acme-smoke.md` after the session file names `.pi`.

## Instructions

1. Invoke `read-spec` against the targeted feature.
