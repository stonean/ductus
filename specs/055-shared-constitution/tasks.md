---
next-task: 20
---

# 055 — Shared Constitution Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 19. AC13: /ductus maintains AGENTS.md's constitutions region for every layout

- [x] Update `plan.md`'s design record for AC13 first; the decision and its two rejected options are this spec's last Resolved Question (2026-11-21).
- [x] `framework/bootstrap/ductus.md` **Shared constitution imports**, steps 2 and 3: besides each layout's native rules file, merge the block into `AGENTS.md` for `claude-style` too, as `- See [{alias}](<{document-path}>)` links, appended with its markers where `AGENTS.md` has none. Check the comment above the region in `framework/templates/project/agents.md` reads true afterwards, and that 050's scenario `memory-routing-lives-in-agents-md` and `docs/shared-constitution.md` §How it loads still hold.
- [x] Run the bootstrap tests, then fresh `claude-style` and `pi` scratch adoptions with a registered shared constitution (`.ductus/docs/agents/developing-ductus.md`, *Exercising an installer change*), checking the region `AGENTS.md` actually received, not only the code.

- **Done when**: Both fresh adoptions show the region written in `AGENTS.md`, and the bootstrap step, the template comment and every spec describing the block agree.
