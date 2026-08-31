# Cross-service references

The deep reference for linking a spec in another service. The README's [Cross-service references](../README.md#cross-service-references) section covers what a reference is and why it never gates anything; this is where the authoring rules, the generator's matching semantics, and the resolution outcomes live.

See [030 — Cross-Service References](../specs/030-cross-service-references/spec.md) for the originating spec and [its data model](../specs/030-cross-service-references/data-model.md) for the `[services]` registry schema.

## Documenting a reference in a spec

You author a reference by writing a **normal inline markdown link** in the spec **body** — nothing goes in the frontmatter, and there is no special syntax. The link's href must be an **absolute `http(s)` URL** whose path contains the target spec's `/specs/NNN-slug/` segment in the other service's repo:

```markdown
Tokens follow the contract in
[api 014-auth-tokens](https://github.com/acme/api/blob/main/specs/014-auth-tokens/spec.md).
```

On the next commit (or any `ductus derive-references --write` run) the derivation harvests that link into the frontmatter:

```yaml
references:
  - service: api      # the [services] alias whose repo matches the URL host
    spec: 014-auth-tokens
```

## What the generator keys on

- **`NNN-slug` is the identity.** Everything in the URL before a `/blob/<ref>/` or `/tree/<ref>/` branch segment is the repo, matched against `.ductus/config.toml [services]` to resolve the alias; the branch is ignored, so two links to the same spec on different branches collapse to one reference. A URL matching no registered service is still recorded, with `service: null` (the `unregistered` outcome below).
- **Absolute URL, not a sibling link.** `[label](../014-auth-tokens/spec.md)` is a *sibling* link and becomes a **dependency** (a different generator, the blocking `dependencies:` graph) — never a cross-service reference. Use the full canonical URL precisely so the two stay distinct.
- **Opt-outs are honored.** A link is **not** harvested if it sits under a `## See also` heading, inside a fenced code block, or on a blockquote (`>`) line. These are the same navigational opt-outs `dependencies:` honors — use them for "see also" links you don't want to register. (`## References` is deliberately *not* an opt-out: it is the formal body-authored section, and links under it are meant to register.)

## Registering a service

Register a service with `/link` (alias, repo URL, local checkout path, optional description):

```toml
[services.api]
repo = "https://github.com/acme/api"
path = "../api"
description = "owns shared data models"
```

The registry is **required for status resolution, optional for referencing** — an unregistered link is just navigation.

## Resolution outcomes

`/status` shows each reference's resolution outcome (and, on `ok`, the linked status); `/analyze` reports a provably broken one as an Advisory finding. The outcome depends on what can be proven:

| Outcome | Meaning |
| --- | --- |
| `ok` | Registered, checkout reachable, target spec resolves — surfaces the linked lifecycle status |
| `unregistered` | The repo matches no `[services]` entry — a plain navigational link; run `/link` to register the service |
| `not-checked-out` | Registered, but the local `path` is missing or unusable — `unknown`, never reported as broken |
| `broken` | Registered and reachable, but the target spec does not resolve (renamed, moved, deleted, or mistyped) — an `/analyze` finding |
| `status-unreadable` | The target exists but its `status` cannot be read — `unknown`, the defect is upstream's |

Status resolution runs only where the linked service is already checked out locally; `ductus` never fetches or clones a repo. The target spec is resolved under the **linked service's own** `[paths] specs-root`, read from that checkout's `.ductus/config.toml` — each service may configure its own spec root.
