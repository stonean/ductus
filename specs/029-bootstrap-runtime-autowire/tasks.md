---
next-task: 18
---

# 029 — Bootstrap Runtime Auto-Detect and Wire Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 17. A step whose primitive cannot run keeps its primitive's path screens (deferred from 007's analysis, owner decision 2026-11-18)

- [x] 007's analysis (BE-INPUT-004): a step whose primitive cannot run falls back to its shell spec, which carries none of the primitive's screens, so a path built from the project name or spec root is written or removed with no canonicalization or containment check; `extract-archive`'s `tar -xzf` fallback leaves archive entry names uncontained the same way
- [x] No step whose primitive screens paths (`apply-manifest`, `enforce-manifest`, `extract-archive`) runs an unscreened shell fallback when its call cannot run. Put the mechanism to the owner first: the step halts naming the primitive, or its shell spec canonicalizes and contains each path
- [x] `framework/bootstrap/ductus.md` §State A — runtime live this session, scenario `state-a-deterministic-path-forcing` and AC20 state the result, and 007's plan decision "Paths built from the inputs are contained" corrects its last sentence (a correction to 007's plan.md)
- [ ] A scratch-adopter bootstrap shows a normal run unchanged

- **Done when**: no path built from user input is written, removed or extracted uncontained on any route the bootstrap specifies, the artifacts above say so, and 029's analysis records BE-INPUT-004 addressed
