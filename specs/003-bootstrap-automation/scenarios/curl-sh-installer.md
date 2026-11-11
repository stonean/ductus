---
section: "Behavior"
---

# One-line installer (curl | sh)

## Context

The bootstrap command `ductus.md` must be placed into an adopting project before `/ductus` can run. Originally the README documented that placement as a multi-step recipe per agent: `mkdir` the agent's command directory and `curl` `ductus.md` into it. Each supported agent meant its own multi-line snippet, none copy-pasteable in a single shot.

Adopters expect the now-standard one-line installer experience modeled by tools like rustup (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`). Delivering that requires a hosted script that encapsulates the per-agent placement logic, so each README install reduces to a single `curl … | sh` line.

## Behavior

- A POSIX-`sh` script `install.sh` lives at the repo root, is attached to every `ductus-v*` release as an asset, and is fetched and executed via `curl --proto '=https' --tlsv1.2 -sSfL https://github.com/stonean/ductus/releases/latest/download/install.sh | sh` — the latest release's copy, with `-L` following GitHub's redirect to its asset host.
- It resolves the target agent from the first non-flag argument (`sh -s -- <agent>`), defaulting to `claude` when none is given; a second non-flag argument, or an unknown `--` flag, exits non-zero. The accepted names are the `framework/bootstrap/ductus.md` §Agent Registry keys — `claude` and `pi`.
- It fetches `framework/bootstrap/ductus.md` from the latest release by default, resolved from the `Location` header of `releases/latest` and never falling back to `main`, or from the source `--ref=<latest|main|ductus-vX.Y.Z>` names, accepted in any position beside the agent. Every check on the source — the grammar, a repeated `--ref`, the release floor, and a named tag that does not exist — runs before anything is written. The installer reads and writes no project configuration, so the `/ductus --ref=<value>` its completion message names is the run that records the choice.
- It places the bootstrap per agent, matching the destinations declared in the [012 multi-agent](../../012-multi-agent-govern/spec.md) agent registry: claude → `.claude/commands/ductus.md`; pi → `.pi/prompts/ductus.md`.
- The download lands in a `mktemp` tempfile cleaned by an `EXIT` trap; a failed fetch — a transport error, or any HTTP status but 200 — exits before the destination is touched, so a partial or empty `ductus.md` is never written. Re-running is idempotent.
- An unrecognized agent name, or a `curl` not found on `PATH`, prints a diagnostic to stderr and exits non-zero.
- The README's per-agent install instructions are each reduced to a single `curl … | sh` line; the Quick start uses the bare form, which installs for `claude`.

> **Signpost:** the installer's source — the latest-release default, `--ref`, the release floor, and the release asset the one-liner fetches — is [061 — Updates track the latest release tag](../../061-updates-track-the-latest-release-tag/spec.md)'s; this scenario states only how the installer places the bootstrap it fetched.

## Edge Cases

- **Piped to `sh`** — stdin is the script itself, so the installer performs no interactive prompting; it runs unattended end to end (unlike installers that read from `/dev/tty`).
- **Beyond first-touch placement** — `install.sh` only handles the initial bootstrap drop. Adopting additional agents later, or any registry-driven multi-agent scaffolding, remains the job of `/ductus --add-agent`; the installer does not duplicate that logic.

## Open Questions

*None — resolved below.*

## Resolved Questions

- **Installer ↔ agent-registry parity.** `install.sh` hard-codes the agent → destination-path mapping that also lives in the [012](../../012-multi-agent-govern/spec.md) agent registry, so adding a fourth agent requires a matching `case` arm in the installer *in addition to* the "single registry row plus a permission file" the registry advertises. **Resolved: enforce the parity with a `/ductus:audit` check** rather than rely on hand-maintenance — the choice the "never depend on human diligence" design principle argues for. `scripts/audit/installer-registry-parity.sh` (Family 14) parses the §Agent Registry table and `install.sh`'s `case`-arm → `dest=` mapping and asserts per-key parity in both directions: a registry agent with no installer arm, an installer arm naming no registry agent, or a dest that doesn't match the layout-derived `ductus` install path each surface as a finding. Wired into `scripts/audit/run-all.sh` and listed in `framework/commands/audit.md`.
