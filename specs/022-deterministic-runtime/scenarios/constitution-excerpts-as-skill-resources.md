---
section: "LLM extension points"
---

# Constitution-excerpts-as-skill-resources

## Context

`load_constitution_excerpts` in `runtime/src/interpreter/payload.rs` parses the running command file's `Reference: §<anchor>, §<anchor>` line, resolves each anchor, and inlines the resolved section bodies as a `Vec<String>` in the `writeCode` request. The excerpts are the leading field of the cache-anchored prefix declared at `spec.md` §LLM extension points, so within one `/ductus:implement` walk a caching host pays for them once.

The Agent Skills standard — open since 2025-12-18, governed at agentskills.io, implemented by roughly forty client products including every host in ductus's Agent Registry — packages exactly this shape as an on-disk `references/` directory beside a `SKILL.md`, loaded on demand under progressive disclosure. ductus already meets its hosts at the Skills surface one altitude up: `/ductus:*` commands install as per-command `SKILL.md` files for Antigravity ([028](../../028-antigravity-agent/spec.md)) and OpenCode ([032](../../032-opencode-agent/spec.md)). The question this scenario decides is whether the *inner* seam — the payload the runtime hands the host at an extension point — should reference those resources instead of inlining their bodies.

Origin: inbox item surfaced 2026-05-19 during runtime-improvement investigation, held from 2026-07-11, narrowed and released 2026-08-30 after the two hold conditions cleared — the standard stabilized, and task 34's payload bundler plus the cache-breakpoint contract shipped. The original framing claimed three benefits; two are already banked. Cache anchoring came from field-order declaration, not from Skills. Host integration against a Skills protocol is already true at the command layer. Only the bundled-resource thread survives, and it is the one this scenario is scoped to.

## Behavior

**Resolved 2026-08-30: no shape ships.** `writeCode` continues to carry `constitution-excerpts` as an inlined `Vec<String>`, leading the cache-anchored prefix exactly as `spec.md` §LLM extension points describes. This scenario is the record of that decision, not a change to be implemented.

Two shapes were considered and both rejected:

1. **Reference-passing** — the request carries `{path, anchor}` pairs and the host performs the read. Rejected: it converts a failure that is currently impossible into one that is silent. A host that never reads the named resource produces a `writeCode` response written with no constitutional context, and nothing in the envelope reveals it.
2. **Bundled `references/`** — the excerpts are materialized beside the installed command skill under progressive disclosure. Rejected: it reaches only the two registry agents that install dir-form skills, so it needs either a parallel mechanism for `claude-style` and `auggie` or a migration of all four layouts — a packaging change larger than this scenario, on a surface with no automated per-layout behavior gate.

The measurement that settles it is recorded under Resolved Questions: the transport cost the bundled-resource idea existed to remove is already gone, taken partly by the cache-breakpoint contract and partly by the host's own constitution load.

These constraints stand, and are what any future proposal on this seam must satisfy — they outlive the decision:

- **Determinism is not traded away.** §runtime-boundary makes the runtime the deterministic layer. Any shape MUST keep anchor *selection* in the runtime; moving the *read* to the host is what both rejected shapes did, and is where the risk sits.
- **Two-paths parity holds.** A `runtime/tests/parity/` test MUST show the markdown-only walker and the runtime walker put the same constitution content in front of the LLM.
- **The cache anchor survives.** `constitution-excerpts` leads the stable prefix; a shape that shortens it is a token win only if the host does not then re-read the same bodies per task.
- **Adopters without the skill install still work.** Any shape MUST keep serving excerpts to hosts that install commands rather than skills — three of the registry's four agents.

Reopening this needs new evidence, not a new framing: a measured cost that survives both the cache anchor and the fact that claude-style hosts already hold the whole constitution in context.

## Edge Cases

Retained as the checklist any future proposal on this seam must answer — each is a case the inlined array handles today by construction:

- **Host already holds the constitution** — every adopter's `CLAUDE.md` carries `@import .ductus/constitution.md` (`framework/templates/project/claude-md.md:3`), so on claude-style layouts the excerpts duplicate context the host loaded before the request arrived. This is the case that removed the motivation, and any future proposal must state what it saves *given* this.
- **Host that ignores the reference** — a `writeCode` response written without constitutional context, with nothing in the envelope revealing it. The inlined shape makes this impossible; both rejected shapes make it silent.
- **Command file with no `Reference:` line** — yields an empty array today; any shape MUST keep yielding an empty set rather than a missing-resource error.
- **Anchor that resolves to nothing** — an anchor naming a section the constitution no longer carries. A reference shape defers the failure from resolve-time to host-read-time, which is later and quieter; any shape MUST fail where the inlined shape fails.
- **Adopter with a pinned or customized constitution** — references must resolve against the adopter's constitution, never the framework's shipped copy.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Which shape ships — reference-passing, bundled `references/`, or neither?** Neither; the inlined array stands. The bundled-resource thread was the last surviving benefit of the originating inbox item, and its justification was transport cost, which the measurement below shows is already banked. What remains is cost without benefit: a reference shape turns an impossible failure into a silent one (a host that skips the read writes code with no constitutional context), moves a resolution decision across the boundary §runtime-boundary draws, and makes third-party integration harder — a host receives strings today and would have to resolve paths against an adopter's layout instead. Resolved 2026-08-30.
- **Is there a measured token win at all?** No, not for the hosts that matter. `/ductus:implement`'s `Reference:` line resolves to five anchors — §implement-phase, §pipeline-boundaries, §text-first-artifacts, §brownfield-inbox, §spec-phase — totalling 20,657 bytes (~5.2k tokens) per request. Without a cache anchor a 20-task walk re-sends that per task (~103k tokens); with the anchor that shipped in task 34 it is one write plus nineteen cache reads (~16k), removing roughly 85%. The remainder is largely not a cost: `framework/templates/project/claude-md.md:3` puts `@import .ductus/constitution.md` in every adopter's `CLAUDE.md`, so a claude-style host already holds the entire 92 KB constitution before the request arrives and the excerpt array is a duplicate subset of it. A host *without* prompt caching still pays the full per-task cost — that case is real but is answered by implementing the SHOULD contract, not by changing the payload shape. Resolved 2026-08-30.
- **Does a `references/` shape force the Antigravity and OpenCode skill layouts on all four registry agents?** The question does not arise, since no `references/` shape ships — but the answer is yes, and it is why the shape would have been rejected on its own. Dir-form skills reach only `antigravity` and `opencode`; `claude-style` and `auggie` install command files, so the shape needs either a parallel mechanism for them or a migration of all four layouts. `AGENTS.md` records that nothing audits per-layout *behavior* parity, so that migration would be carried by contributor discipline rather than a gate. Resolved 2026-08-30.
