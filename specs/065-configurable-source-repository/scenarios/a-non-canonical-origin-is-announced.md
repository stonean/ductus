---
section: "Behavior"
---

# A-non-canonical-origin-is-announced

## Context

`DUCTUS_REPO` redirects every fetch: the bootstrap, the version pin, the framework archive, and the runtime binary. A value left set in a shell profile after testing a fork would pull that fork's runtime into the next adoption with nothing on screen to say so.

## Behavior

When `DUCTUS_REPO` is set, non-empty, and not `stonean/ductus`, both entry points name the origin before fetching from it:

- `install.sh` prints `ductus: source repository is {repo} (from DUCTUS_REPO), not the canonical stonean/ductus` on stderr.
- `install.sh`'s next-step line then says to start the agent with `DUCTUS_REPO={repo}` exported. A variable set inline for the installer alone (`DUCTUS_REPO=… sh`) never reaches the agent, and a bootstrap from the fork would then fetch the pin, the archive and the runtime from `stonean/ductus` — the half-forked adoption this spec exists to prevent.
- `/ductus` prints `Source repository: {value} (from DUCTUS_REPO), not the canonical stonean/ductus. Every fetch this run, the runtime binary included, comes from it.` in Source resolution step 1, before any fetch.

An unset or empty variable, or one naming `stonean/ductus`, prints nothing.

## Edge Cases

- `/ductus` reads the variable with `awk 'BEGIN { print ENVIRON["DUCTUS_REPO"] }'`, which every agent's Permission Setup seed already allows (Pi has no permission gating), so no layout gains a prompt.
- The line is an announcement, not a halt: a fork is a supported source, and the point is that it is visible.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
