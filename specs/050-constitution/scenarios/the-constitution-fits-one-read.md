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

- **The cut reached 53,813 bytes, not 50,000: which is the constitution's target?** Measured 2026-10-26 with `wc -c`. Moving the single-command rules and compressing every section reached 49,851 bytes, but a sweep of the 2,156 live-artifact lines that cite the constitution found the compression had falsified about 30 passages quoted or relied on in the bodies, scenarios and data models of 16 `done` specs (000, 008, 010, 011, 013, 023, 036, 038, 040, 045, 046, 051, 052, 054, 062, 066), and in the paused 022, the in-progress 029 and the draft 070. Restoring them, net of 840 bytes of uncited glosses and examples, added 3,962 bytes; no uncited text was left to take without dropping a load-bearing reason. The ceiling is lowered to the size reached, and the constitution is not imported at startup, because `072-governing-documents-stay-within-one-read` AC6 bounds an import at 50,000 bytes. Options: **(a)** keep 53,813 as the ceiling and import the constitution at startup, amending 072 AC6's bound, since an `@path` import loads whole and the 50,000-byte figure is the Read-tool heuristic spec 063 chose; **(b)** hold 50,000, correcting the quoting passages in those 16 specs, each reopened through its back-edge and re-closed by a review and an analyze, then cutting the restored bytes again and importing; **(c)** keep 53,813 with no startup import, leaving `/{project}:target`'s read as the only load. Recommended: (a). A session started from a handoff prompt runs no command, so (c) leaves it without the constitution, and (b) spends 16 reopen cycles, the process cost 072 opened this work to remove.

## Resolved Questions

*None yet.*
