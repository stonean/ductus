---
section: "Follow-on scenarios"
---

# Host-namespace-parity

## Context

This repo's `.ductus/config.toml` carries only a `[review]` block — no `[host]`. `Host::load` therefore falls back to the repo directory basename, `ductus`, while the installed slash commands live under `.claude/commands/ductus/` and are invoked as `/ductus:*`. Every runtime-rendered next-action string consequently names a namespace that does not exist: `/ductus:dashboard` output reads "Run /ductus:target …", "/ductus:clarify", "/ductus:implement".

The fallback itself is correct, documented behavior — `project` is explicitly the shared, committed value naming the slash-command namespace, and a repo that never sets it gets its basename. So this is not a runtime defect. It is drift between two committed artifacts that must agree: the configured namespace and the installed one.

It went unnoticed for a long time because nothing compares them. A repo whose rendered commands do not match its installed commands is exactly the drift [§drift-prevention](../../../framework/constitution.md#drift-prevention) exists to catch, and the framework's own dogfooding did not catch it. Surfaced 2026-07-30 while implementing spec 046 task 5, whose new dashboard callout is one more place the wrong namespace appears.

## Behavior

A new `/ductus:audit` check family — **host namespace parity** — resolves the effective host namespace exactly as `Host::load` does (`[host] project` from `.ductus/config.toml`, falling back to the repo directory basename) and compares it against the slash-command namespace directories actually installed under each agent config directory present in the repo (`{cli-config-dir}/commands/<ns>/`).

A mismatch is a finding that names both values and the fix — the one-block `[host]` / `project = "<ns>"` addition — so the maintainer does not have to rediscover the fallback rule to act on it.

The family is not green in this repo until `.ductus/config.toml` gains `[host]` / `project = "gov"`, so landing the check lands the fix with it. `scripts/audit/run-all.sh` is a hard release gate here, which means the parity holds from that point on rather than depending on anyone remembering.

The check belongs to `/ductus:audit` rather than `/ductus:analyze` per the [§Boundary with `/ductus:analyze`](../spec.md#boundary-with-govanalyze) rule of thumb: it reads across cross-cutting repo artifacts (config plus installed command directories), not within one spec's directory.

## Edge Cases

- **No commands directory installed** — a repo that has never run `/ductus` or `/ductus:init` has nothing to compare against; not a finding. The check asserts agreement between two things that exist, not that either exists.
- **Fallback happens to match** — an adopter whose repo directory basename is literally the installed namespace passes with no `[host]` block. The fallback is documented behavior, so the family checks agreement, not the presence of the block.
- **Multiple namespace directories under one `commands/`** — a finding only when *none* matches the effective namespace; an adopter may legitimately install commands from more than one source.
- **Multiple agent config directories present** (`.claude`, `.augment`, `.opencode`, `.agents`) — each is checked independently. `project` is the shared committed value across agents per spec 012's multi-agent contract, so a namespace present under one config dir and absent under another is itself the finding.
- **Placeholder overlap with Family 4** — Family 4 (placeholder roundtrip) forbids a hardcoded `gov:` inside `framework/commands/` sources; this family compares *resolved* values in an installed repo. The two never look at the same file, so no duplicate finding arises.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
