---
section: "Behavior"
---

# One-line installer (curl | sh)

## Context

The bootstrap command `ductus.md` must be placed into an adopting project before `/ductus` can run. Originally the README documented that placement as a multi-step recipe per agent: `mkdir` the agent's command directory, `curl` `ductus.md` into it, and — for Antigravity — additionally pipe through `awk` to strip `ductus.md`'s own frontmatter and wrap the body as a `name: ductus` skill. Three supported agents meant three different multi-line snippets, none copy-pasteable in a single shot.

Adopters expect the now-standard one-line installer experience modeled by tools like rustup (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`). Delivering that requires a hosted script that encapsulates the per-agent placement logic, so each README install reduces to a single `curl … | sh` line.

## Behavior

- A POSIX-`sh` script `install.sh` lives at the repo root and is fetched and executed via `curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/stonean/ductus/main/install.sh | sh`.
- It resolves the target agent from the optional positional argument (`sh -s -- <agent>`), defaulting to `claude` when none is given. The accepted names are the §Agent Registry keys — `claude`, `auggie`, `antigravity` — plus `agy`, the Antigravity CLI command name, as a documented alias for `antigravity` (canonicalized internally so it maps to the same registry key).
- It fetches `framework/bootstrap/ductus.md` from `main`, consistent with ductus's live-on-main model. There is no release-pinning knob: `ductus.md`'s own self-update check and archive fetch both target `main`, so a pinned bootstrap would be overwritten on the next `/ductus` run anyway — tags are milestones, not pinning targets.
- It places the bootstrap per agent, matching the destinations declared in the [012 multi-agent](../../012-multi-agent-govern/spec.md) agent registry: claude → `.claude/commands/ductus.md`; auggie → `.augment/commands/ductus.md`; antigravity → `.agents/skills/ductus/SKILL.md`, with the body wrapped in `---\nname: ductus\n---` skill frontmatter and `ductus.md`'s own frontmatter stripped (everything up to and including the second `---`).
- The download lands in a `mktemp` tempfile cleaned by an `EXIT` trap; a failed fetch (`curl -f`) aborts under `set -e` before the destination is touched, so a partial or empty `ductus.md` is never written. Re-running is idempotent.
- An unrecognized agent name, or a `curl` not found on `PATH`, prints a diagnostic to stderr and exits non-zero.
- The README's per-agent install instructions are each reduced to a single `curl … | sh` line; the Quick start uses the bare form, which installs for `claude`.

## Edge Cases

- **Piped to `sh`** — stdin is the script itself, so the installer performs no interactive prompting; it runs unattended end to end (unlike installers that read from `/dev/tty`).
- **Beyond first-touch placement** — `install.sh` only handles the initial bootstrap drop. Adopting additional agents later, or any registry-driven multi-agent scaffolding, remains the job of `/ductus --add-agent`; the installer does not duplicate that logic.

## Open Questions

*None — resolved below.*

## Resolved Questions

- **Installer ↔ agent-registry parity.** `install.sh` hard-codes the agent → destination-path mapping that also lives in the [012](../../012-multi-agent-govern/spec.md) agent registry, so adding a fourth agent requires a matching `case` arm in the installer *in addition to* the "single registry row plus a permission file" the registry advertises. **Resolved: enforce the parity with a `/ductus:audit` check** rather than rely on hand-maintenance — the choice the "never depend on human diligence" design principle argues for. `scripts/audit/installer-registry-parity.sh` (Family 14) parses the §Agent Registry table and `install.sh`'s `case`-arm → `dest=` mapping and asserts per-key parity in both directions: a registry agent with no installer arm, an installer arm naming no registry agent, or a dest that doesn't match the layout-derived `ductus` install path each surface as a finding. Wired into `scripts/audit/run-all.sh` and listed in `framework/commands/audit.md`.
