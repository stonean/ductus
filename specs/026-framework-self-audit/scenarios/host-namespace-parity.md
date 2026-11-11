---
section: "Follow-on scenarios"
---

# Host-namespace-parity

## Context

When this surfaced the repo's config carried only a `[review]` block — no `[host]` — so `Host::load` fell back to the repo directory basename, which was then `govern`, while the installed slash commands lived under `.claude/commands/gov/` and were invoked as `/gov:*`. Every runtime-rendered next-action string consequently named a namespace that did not exist: `/gov:dashboard` output read "Run /govern:target …", "/govern:clarify", "/govern:implement". The two names are kept as written because the mismatch **is** the record: 049's word-boundary sweep sent both `govern` and `gov` to `ductus`, which collapsed the two sides of this sentence onto one name and left it asserting that `/ductus:*` names a namespace that does not exist while `.claude/commands/ductus/` sat in the same clause.

The fallback itself is correct, documented behavior — `project` is explicitly the shared, committed value naming the slash-command namespace, and a repo that never sets it gets its basename. So this is not a runtime defect. It is drift between two committed artifacts that must agree: the configured namespace and the installed one.

It went unnoticed for a long time because nothing compares them. A repo whose rendered commands do not match its installed commands is exactly the drift [§drift-prevention](../../../framework/constitution.md#drift-prevention) exists to catch, and the framework's own dogfooding did not catch it. Surfaced 2026-07-30 while implementing spec 046 task 5, whose new dashboard callout is one more place the wrong namespace appears.

## Behavior

A new `/ductus:audit` check family — **host namespace parity** — resolves the effective host namespace exactly as `Host::load` does (`[host] project` from `.ductus/config.toml`, falling back to the repo directory basename) and compares it against the slash-command namespace directories actually installed under each agent config directory present in the repo (`{cli-config-dir}/commands/<ns>/`).

A mismatch is a finding that names both values and the fix — the one-block `[host]` / `project = "<ns>"` addition — so the maintainer does not have to rediscover the fallback rule to act on it.

The family was not green in this repo until `.ductus/config.toml` gained a `[host]` block naming the installed namespace, so landing the check landed the fix with it; the key is present today and reads `project = "ductus"`. `scripts/audit/run-all.sh` is a hard release gate here, which means the parity holds from that point on rather than depending on anyone remembering.

The check belongs to `/ductus:audit` rather than `/ductus:analyze` per the [§Boundary with `/ductus:analyze`](../spec.md#boundary-with-ductusanalyze) rule of thumb: it reads across cross-cutting repo artifacts (config plus installed command directories), not within one spec's directory.

## Edge Cases

- **No commands directory installed** — a repo that has never run `/ductus` has nothing to compare against; not a finding. The check asserts agreement between two things that exist, not that either exists.
- **Fallback happens to match** — an adopter whose repo directory basename is literally the installed namespace passes with no `[host]` block. The fallback is documented behavior, so the family checks agreement, not the presence of the block.
- **Multiple namespace directories under one `commands/`** — a finding only when *none* matches the effective namespace; an adopter may legitimately install commands from more than one source.
- **Multiple agent config directories present** — each is checked independently. `project` is the shared committed value across agents per spec 012's multi-agent contract, so a namespace present under one config dir and absent under another is itself the finding.
- **Placeholder overlap with Family 4** — Family 4 (placeholder roundtrip) forbids a hardcoded `ductus:` inside `framework/commands/` sources; this family compares *resolved* values in an installed repo. The two never look at the same file, so no duplicate finding arises.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
