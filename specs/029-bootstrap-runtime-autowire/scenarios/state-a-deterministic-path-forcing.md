---
section: "Follow-on scenarios"
---

# State-A-deterministic-path-forcing

## Context

029 wires `ductus` in State B and restarts so the next session runs the cheap deterministic path. That payoff only lands if the next session actually *takes* that path. In end-to-end testing it did not: the post-restart session had `ductus` live (it skipped the binary probe — State A — and read the registered server's instructions), yet it walked the markdown `curl`/`tar`/`python3` reference path anyway, re-incurring exactly the token cost and the shell-permission prompts the wiring was meant to remove. Surfaced 2026-06-11.

The cause is that the State-A handoff was **advisory**. The runtime mapping note in `framework/bootstrap/ductus.md` §Instructions says "call the corresponding tool … that is the deterministic path," and State A said only "take the deterministic primitive path for the rest of the run." A capable model honors that; a weaker host model detects the runtime but then executes the prose steps the document spends ~900 lines describing in shell, because nothing told it the shell blocks are *off-limits* once the runtime is live. The procedure described two paths but did not make taking the right one binding.

## Behavior

State A becomes a **binding execution contract** rather than a preference. At the State-A detection point the procedure now states, for the rest of the run:

- **Every step that names a backticked primitive** (a bare name — `fetch-archive`, `extract-archive`, `apply-manifest`, `merge-managed-block`, `enforce-manifest`, `merge-permissions`, … — matching a `ductus` tool in the inventory) **MUST be performed by calling that MCP tool**. **Superseded in part by `048-govern-acquired-runtime`**, whose scenario `state-a-version-checks-the-pin` version-checks a live runtime against the pin: when it finds the runtime live but stale, each such step runs as `.ductus/bin/ductus <primitive>` instead, because the live server is the old binary; in neither case does it run the shell text.
- **The shell commands shown under those steps** (`curl`, `tar -xzf`, `python3`, `awk`, byte-compares, scaffold loops) document the contract each tool fulfills, and are **not instructions to execute** in either state: a State-B run reaches the same primitives through the CLI. An explicit self-check is given: about to run `curl`/`tar`/`python3` for a primitive-backed step ⇒ stop and call the tool.
- **Steps with no backticked primitive run as shown in every state** — the per-language `.gitignore` `curl` (`github.com/github/gitignore`), `git config core.hooksPath`, `chmod`, the git repo / tracked-file checks, and the §Collect Project Inputs prompts in `framework/bootstrap/ductus.md`. The contract names these explicitly so the boundary is unambiguous in both directions (don't shell a primitive step; don't try to tool-ify a non-primitive step).
- **Per-step graceful degradation**: if a primitive call cannot run (e.g. a project-supplied `[runtime] path` at another version raises a parse error per §Versioning enforcement in `specs/022-deterministic-runtime/spec.md`; since 048, a too-old store binary is re-acquired by the version check before any step calls it), fall back to that one step's shell spec and continue — the whole run does not abandon the deterministic path.
- **A primitive's refusal of its input is not a failure to degrade around.** A primitive that refuses what it was given has run, and its refusal stands: the run halts, naming what was refused, and never falls back to the step's shell spec, which carries none of the primitive's screens. For `extract-archive` that is an archive entry with an absolute path, a `..` component or a name that is not UTF-8, one that would write through a link under the staging directory, or an archive past its entry or byte cap; the `tar -xzf` fallback commits to none of those screens, since what it refuses is the host tar's own, and it has no entry or byte cap. Only a call that cannot run falls back. (`BE-INPUT-004`, found by 015's analysis; the owner deferred it here, 2026-11-11.)

A second, spot reinforcement is added at the top of `framework/bootstrap/ductus.md` **§File Fetching** — the step observed failing — naming the four primitive-backed steps there (`fetch-archive`/`extract-archive`/`apply-manifest`/`enforce-manifest`) and reminding that the `curl`/`tar` shown specify what those primitives do, not commands to execute.

## Edge Cases

- **Lazy/deferred tool schemas (Claude Code).** Already State A per the existing rule — fetch the schema via the host mechanism and call the tool; the contract applies unchanged.
- **A step with no primitive.** Stays shell in State A (gitignore curl, git config, chmod, repo checks, input prompts). The contract lists these so the model neither shells a primitive step nor invents a tool for a non-primitive one.
- **Mixed run.** A single run legitimately interleaves tool calls (primitive steps) and shell (non-primitive steps); that is correct, not a contract violation.
- **Primitive missing from inventory mid-run.** Treated like a call that cannot run — that step falls back to its shell spec; the rest of the run stays on tools.
- **A refusal and an inability to run look alike to a host that reads only success or failure.** Neither an MCP tool error nor a CLI error carries a code, so the host tells them apart by the error's text, and for `extract-archive` by name: a refusal of the archive's content is `unsafe archive entry path: …`, a symbolic link that resolves outside the staging directory, or the extract cap on entries or bytes; an I/O or format error, or one about the call's own paths, means the call could not run.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Why not hard-enforce the tool path instead of strengthening language?** A markdown procedure cannot force a model's tool selection — the only lever it owns is unambiguous, binding language plus making the shell blocks explicitly subordinate (fallback spec, not instructions). Host-level enforcement (denying `curl`/`tar` while `ductus` is registered, or generating a State-A-only procedure with the shell blocks stripped) is a **deliberate non-goal, not a deferral** — `ductus` will not add it. It would block the legitimate non-primitive shell steps the bootstrap still needs (gitignore `curl`, `git config`, `chmod`); it is host-specific and a permanent per-host maintenance tax; and, as [§runtime-boundary](../../021-runtime-boundary/spec.md) read when this was decided, it broke that section's parity — *"the two paths share a contract; neither one wraps the other"* — by making the runtime path coercive and demoting the markdown path from first-class to a second-class fallback. That sentence is gone from the constitution (`ad4fae5b`), since 048 made the runtime required, and the first two reasons carry the decision alone. The binding contract is therefore the strongest lever consistent with ductus's text-first design, and it is honest that it maximizes rather than guarantees compliance: a model that still ignores it falls back to the correct-but-pricier markdown path, which is an **accepted outcome** (the remedy is a more capable model), not a gap to coerce around.
- **Why list the non-primitive steps explicitly?** The failure mode is not only "didn't use tools" but also "didn't know which steps have tools." Enumerating the non-primitive steps (gitignore curl, git config, chmod, repo checks, input prompts) draws the boundary in both directions, so the contract is actionable rather than aspirational.
