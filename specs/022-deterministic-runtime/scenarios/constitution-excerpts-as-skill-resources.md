---
section: "LLM extension points"
---

# Constitution-excerpts-as-skill-resources

## Context

`load_constitution_excerpts` in `runtime/src/interpreter/payload.rs` parses the running command file's `Reference: §<anchor>, §<anchor>` line, resolves each anchor, and inlines the resolved section bodies as a `Vec<String>` in the `writeCode` request. The excerpts are the leading field of the cache-anchored prefix declared at `spec.md` §LLM extension points, so within one `/ductus:implement` walk a caching host pays for them once.

The Agent Skills standard — open since 2025-12-18, governed at agentskills.io, implemented by roughly forty client products including every host in ductus's Agent Registry — packages exactly this shape as an on-disk `references/` directory beside a `SKILL.md`, loaded on demand under progressive disclosure. ductus already meets its hosts at the Skills surface one altitude up: `/ductus:*` commands install as per-command `SKILL.md` files for Antigravity ([028](../../028-antigravity-agent/spec.md)) and OpenCode ([032](../../032-opencode-agent/spec.md)). The question this scenario decides is whether the *inner* seam — the payload the runtime hands the host at an extension point — should reference those resources instead of inlining their bodies.

Origin: inbox item surfaced 2026-05-19 during runtime-improvement investigation, held from 2026-07-11, narrowed and released 2026-08-30 after the two hold conditions cleared — the standard stabilized, and task 34's payload bundler plus the cache-breakpoint contract shipped. The original framing claimed three benefits; two are already banked. Cache anchoring came from field-order declaration, not from Skills. Host integration against a Skills protocol is already true at the command layer. Only the bundled-resource thread survives, and it is the one this scenario is scoped to.

## Behavior

The decision is not pre-made; the Open Questions below carry it. Two candidate shapes are on the table, and the scenario is settled by choosing one (including "neither — the inlined array stands"):

1. **Reference-passing.** The `writeCode` request carries `constitution-excerpt-refs` — resolved `{path, anchor}` pairs — in place of, or alongside, the inlined bodies. The runtime still decides *which* anchors apply, so anchor selection stays deterministic and the markdown-only and runtime walkers still name the same set. Only the read moves to the host.
2. **Bundled resource.** The excerpts are materialized into a `references/` directory beside the installed command skill, and the payload names files within it. The host loads them under progressive disclosure.

Whichever shape is chosen, these constraints hold and are what the scenario must prove:

- **Determinism is not traded away.** §runtime-boundary makes the runtime the deterministic layer; a shape where the host's model decides which excerpts enter context moves a resolution decision across the boundary. Any accepted shape MUST keep anchor *selection* in the runtime.
- **Two-paths parity holds.** A `runtime/tests/parity/` test MUST show the markdown-only walker and the runtime walker put the same constitution content in front of the LLM for `/ductus:implement`, whichever shape ships.
- **The cache anchor survives.** `constitution-excerpts` leads the stable prefix today. A shape that shortens the field to references makes the prefix smaller, which is a token win only if the host does not then re-read the same bodies per task — measure before claiming it.
- **Adopters without the skill install still work.** The markdown-only path and hosts that install commands rather than skills MUST keep receiving usable excerpts; a `references/`-only shape that assumes the skill layout is on disk breaks three of the registry's four agents.

## Edge Cases

- **Command file with no `Reference:` line** — yields an empty array today; whatever shape ships MUST keep yielding an empty set, not a missing-resource error.
- **Anchor that resolves to nothing** — an anchor naming a section the constitution no longer carries; the reference shape defers the failure from resolve-time to host-read-time, which is later and quieter. The chosen shape MUST fail at the same point the inlined shape does.
- **Adopter with a pinned or customized constitution** — references must resolve against the adopter's constitution, not the framework's shipped copy.
- **Host that ignores the reference** — a host that never reads the named resource produces a `writeCode` response written without constitutional context. The inlined shape makes that failure impossible; the reference shape makes it silent. Weigh this in the decision.

## Open Questions

- Which shape ships — reference-passing, bundled `references/`, or neither (the inlined array stands)? The scenario is not implementable until this is answered.
- Is there a measured token win at all? The cache anchor already amortizes the excerpts across a walk, so the saving only exists for hosts without prompt caching, or for walks short enough that the anchor never pays off. Measure before building.
- Does a `references/` shape force the Antigravity and OpenCode skill layouts to become the reference layout for all four registry agents, and if so is that a packaging change larger than this scenario?

## Resolved Questions

*None yet.*
