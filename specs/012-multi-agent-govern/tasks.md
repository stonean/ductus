---
title: "012-multi-agent-govern — tasks"
next-task: 14
---

# 012 — Multi-Agent Ductus Tasks

Tasks derived from the [plan](plan.md). Complete in order.

## 12. Real adopter bootstrap after the agent removal

- [ ] framework/bootstrap/ductus.md, install.sh and the migrations — the agent removal (chore: drop Auggie, Antigravity and OpenCode support) rewrote the registry, Permission Setup, MCP wiring, the Pi layout and three migrations, and no real run has exercised them; AGENTS.md asks a real adopter bootstrap of any framework/bootstrap/ or framework/migrations/ change, on a subject far enough behind to cross the whole chain. Bootstrap a scratch adopter from an older release on claude, then upgrade it with /ductus --ref=main, per docs/developing-ductus.md (Exercising an installer change without a push); and run a real pi against a pi-selected scratch adopter, since pi's loader has no other test. Run from a session on the pulled, rebuilt tree, never this repository's stale MCP server; budget a fix-and-rerun. Holds 012's done, and so the ductus-v0.60.0 tag (owner, 2026-11-11). The runs must use a tree at or after `e2009d70`, which quoted the pin and migration-registry fetch URLs in `framework/bootstrap/ductus.md` §Source resolution steps 5 and 6 with no real run behind it (064's review): step 5 runs on every resolved ref, step 6 only on a tag ref when `[migrations] last_applied` is set, so a `--ref=main` upgrade does not reach it. The closing commit says whether step 6 ran, and names it unexercised if no run could reach it. Run after task 13 lands, on a subject carrying a dropped host's tree, so the same run checks its notice. Commit in the upgraded adopter so its installed `.githooks/ductus-pre-commit` runs: 067's task 26 corrected that hook's header, comments only, with no real run of its own, so this run covers it (owner, 2026-11-11). The closing summary must also name the temp directory, `temp directory: {tempdir} — left for the OS to sweep`, which 015's task 6 added with `{tempdir}` in the archive abort message; the abort message has a real run only if one fails, so the closing commit names it unexercised otherwise (owner, 2026-11-11)

- **Done when**: both runs completed on main's bootstrap, each run's outcome is stated in the commit that closes this task, and every defect found is fixed or dispositioned

## 13. Agent Selection names a dropped host's leftover tree

- [x] Record the requirement in 012 first, as a scenario (`a-dropped-hosts-leftover-tree-is-named`) or an acceptance criterion: when the project holds `.augment/`, `.agents/`, `.opencode/` or a root `opencode.json` or `opencode.jsonc` and no Agent Registry row names it, `/ductus` prints one non-gating line saying the host is no longer supported, that `/ductus` leaves the tree alone (the runtime refuses writes there), and how to remove it by hand
- [x] `framework/bootstrap/ductus.md` §Agent Selection: add the notice beside rule 2's auto-detection, worded so it never prompts and never changes the agent set; mirror it to `framework/bootstrap/govern.md` (Family 21)
- [ ] Verify it in task 12's real adopter run on a subject carrying a dropped host's tree, and state the line it printed in the closing commit

- **Done when**: 012 records the requirement, both bootstrap copies print the notice for each of the five paths when no registry row names them and stay silent otherwise, and a real adopter run shows it
