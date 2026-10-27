---
section: "Follow-on scenarios"
---

# The-constitution-fits-one-read

## Context

On 2026-10-26 the constitution was 119,262 bytes. Every pipeline command loads it, here and in every adopting project, and at the 50,000-byte read-size threshold `check-artifact-size` applies to spec artifacts (spec 063) it took three reads. The 2026-10-17 concision pass (`the-constitution-states-rules-not-stories`) had cut it 21.5% by removing stories; reaching one read meant removing a further 69,262 bytes, 58% of what remained. A first measurement put about 48 KB in sections that mix command-specific mechanics with policy: review windows and denominators, audit-record field tables, rename-sweep rules and session mechanics. Moving the mechanics alone would therefore not reach the threshold. [072 — Governing documents stay within one read](../../072-governing-documents-stay-within-one-read/spec.md) capped the constitution at its size when its ceiling check landed, and routed the cut here, because this spec owns what the constitution holds. This is the concrete complaint the last Resolved Question anticipated for splitting an overlong section.

## Behavior

- The constitution is at most the read-size threshold, 50,000 bytes on disk, so an agent reads it whole in one call.
- The constitution keeps what every command needs: the principles, the lifecycle, the artifact schema an author writes against, and the rules that bind more than one command. A rule that only one command applies moves to that command's markdown-only reference, `framework/reference/<command>.md`, which is read when a step points at it. The constitution keeps at most a pointer, following the pattern `071-command-reference-split` set for command files.
- The cut is measured before it is planned. The plan names which sections move, where each goes and how many bytes each removes, so it shows the threshold is reachable rather than assuming it.
- The change that lands the cut lowers 072's ceiling for `framework/constitution.md` to the read-size threshold, so the document cannot grow back.
- Once the constitution fits one read, it loads at startup, not only when `/{project}:target` reads it. The project template's `CLAUDE.md` imports `@.ductus/constitution.md`, this repository's `CLAUDE.md` imports `@framework/constitution.md`, and a migration adds the import line to an existing adopter's `CLAUDE.md`. A session started from a handoff prompt then has the constitution without running a command, and `/{project}:target`'s read becomes the fallback for a host that does not resolve `@path`. Resolved 2026-10-26 by the owner, with `002-project-scaffolding`'s `claude-md-template-imports-load`.

## Edge Cases

- **A passage another spec quotes stays verbatim**, as in `the-constitution-states-rules-not-stories`. 021 quotes the runtime principles, the eligibility criteria and the acquisition invariant. A move that would falsify a quoting spec keeps the passage, or updates that spec through its own back-edge.
- **Anchors stay resolvable.** A moved rule's `<!-- §anchor -->` either stays in place with a pointer, or every reference to it is re-pointed in the same change. AC10 still binds.
- **The cut changes what adopters receive.** It reaches default-source adopters only at the next `ductus-v*` release (061).
- **The measurement may show that one read cannot be reached without dropping rules adopters need.** In that case the threshold returns to the owner as an open question on this scenario, rather than as a caveat, and the ceiling is lowered to the size the cut does reach.

## Open Questions

*None.*

## Resolved Questions

- **The cut reached 53,813 bytes, not 50,000: which is the constitution's target?** Resolved 2026-10-27 by the owner: **(b)**, hold 50,000 and import the constitution at startup. The first cut reached 49,851 bytes, but restoring the passages 16 `done` specs quoted or relied on brought it to 53,813 (`wc -c`, 2026-10-26). The other options were (a), keeping 53,813 and importing it with 072 AC6's 50,000-byte import bound amended, and (c), keeping 53,813 with no startup import. The 50,000 bytes bind a read, not an import: in a `claude -p` run with every tool disallowed, a 53,811-byte `@path` import loaded whole, last sentence included. Two hosts do cap a single read at 51,200 bytes, measured from their installed code: OpenCode's read tool (`ReadTool.lines`, `xe=51200`) and Pi's (`DEFAULT_MAX_BYTES = 50 * 1024` in pi-coding-agent 1.0.4's `dist/core/tools/truncate.js`). At 53,811 bytes, `/{project}:target`'s read on either returned lines 1–500 and a continuation notice. Claude Code's Read returned all 528 lines in one call. Antigravity's cap was not found in its installed binary, and Auggie was not installed, so neither was measured. At or under 50,000 bytes, the read size, every measured host reads the constitution whole. The second cut, recorded in `plan.md` §The constitution fits one read, brought it to 49,989 bytes with every rule kept. It was swept by meaning, not by shared phrasing, and the passages it made false were corrected where they live: five `done` specs through their back-edges (021, 051, 062, 064, 066), and 050's own artifacts and the framework sources in place. An option's process cost was not weighed in recommending it (`AGENTS.md` §Workflow, owner 2026-10-27).
