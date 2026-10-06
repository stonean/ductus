---
description: Verify the ductus bridge extension for Pi and report the trust state.
---

# Configure

Verify and, when drifted, repair the Pi extension bridge that exposes the `ductus` runtime tools — and report what cannot be configured because Pi has no permission-gating settings.

## Scope Boundaries

- Read `.pi/prompts/` only to confirm trust is granted (`/ductus` scaffolds there), and `.ductus/config.toml` only for its `[source] ref`; read and write only `.pi/extensions/ductus.ts`. Do NOT modify any other file.
- The bridge file — the copy of `framework/bootstrap/pi/ductus-bridge.ts` installed by `/ductus` at State-B wire time — is the single persistent artifact this agent's runtime access depends on. A pip-installed adopter's checkout gets the bridge from `/ductus`; this command repairs it between runs.
- Do NOT scan source code, specs, or git history. This command manages the bridge file and reports the permission/trust facts; it does not manage session state.
- Reference: no constitution sections apply — this command operates on agent-specific wiring state, not `ductus` artifacts.

## Instructions

> Pi has **no built-in MCP client** and **no permission-gating settings** (verified against the shipped pi install, spec 064 §Verified Pi Layout). The runtime's tools reach the model through the project's `.pi/extensions/ductus.ts` extension bridge, which wraps the runtime's own MCP server over stdio; tool calls run without any host permission prompt, and project trust is the only gate. There is therefore nothing to merge into a permissions file — the canonical "merge permissions" step of the other agents' configure commands is a **documented no-op** here, and this command's whole job is the bridge check below plus the facts report.

## Verify-and-repair

1. **Locate the canonical source.** The bridge this project should have is the one its framework source ships, so name the source exactly as `/ductus` does (`.pi/prompts/ductus.md` §Source resolution): `.ductus/config.toml` `[source] ref` when it is set (`main` or a `ductus-v<MAJOR>.<MINOR>.<PATCH>` tag), else the latest release — the tag in the `Location` header of `curl -sSI https://github.com/${DUCTUS_REPO:-stonean/ductus}/releases/latest`. Fetch the bridge at that ref into a fresh `{tempdir}` (`mktemp -d`):

   ```text
   curl -fsSL "https://raw.githubusercontent.com/${DUCTUS_REPO:-stonean/ductus}/{ref}/framework/bootstrap/pi/ductus-bridge.ts" -o {tempdir}/ductus-bridge.ts
   ```

   Comparing against any other ref — `main` for a project on a release — would have this command and `/ductus` overwrite each other's bridge on every run.

2. **Compare bytes.** Read the installed `.pi/extensions/ductus.ts` and diff against the canonical source. When they are identical, proceed to step 4 — the bridge is current.

3. **Repair on divergence.** When the installed file differs — whether from a hand edit, an aborted write, or a stale release — overwrite it with the canonical source **byte-for-byte** (no `ductus` / `.pi` substitution; the bridge holds no placeholders). Write atomically (tempfile + rename). Then verify the repair: re-read the installed file and confirm the diff is now empty. If the write cannot be made (permissions, exotic filesystem), report the error and stop — a half-repaired bridge is worse than the stale one.

4. **Report the facts.** State:
   - **Bridge state** — current (and repaired, when step 3 ran) or the error.
   - **No permission settings exist to configure.** Pi displays no host permission prompt for tool calls; the model runs them with the session's project permissions. Nothing needs seeding, and nothing can be added.
   - **Trust is the only gate.** Project-local `.pi/` resources — the prompt templates and this bridge — load only after the project is trusted. A fresh checkout shows a trust prompt on interactive start; non-interactive runs need `--approve` (or `defaultProjectTrust: "always"`). If the operator reports that `/ductus-*` commands or `ductus__*` tools are missing, trust state is the first thing to check.
