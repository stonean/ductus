---
section: "Follow-on scenarios"
---

# A-partial-read-is-not-a-read

## Context

An agent opened this project's `AGENTS.md` with a shell command whose output exceeded the tool's cap. The harness wrote the full text to a temp file and returned the **first 2KB** as a preview, with an explicit notice naming the file it had saved. The agent used the preview, never opened the saved file, and carried on as though it had read `AGENTS.md`.

The preview ended partway through §Project Structure. §Workflow's first rule — *commit directly to `main`; this repo is trunk-based; do not branch first* — sits at line 40, past the cut. The agent branched. The rule had been stated unambiguously since 2026-06-11 and had already been violated once before.

**[§grounding](../../../framework/constitution.md#grounding) did not catch this, and could not have.** It governs *whether* a source was consulted — "Read the file; do not recall it" — and says nothing about whether the consultation was **complete**. A read that returns 2% of a file satisfies it exactly as well as one that returns all of it. The agent did not reason from memory; it reasoned from a real read that had silently dropped 98% of its subject.

That is the shape [`QUAL-CLAIM-001`](../../../framework/rules/quality-cross.md) already forbids everywhere else: a result that cannot distinguish *examined and found nothing* from *could not examine*. The rule set applies it to code output — `derive-dependencies` reports `examined` and `untracked-skipped`, `check-artifacts` reports `skipped`, `check-corpus-links` reports both — and the whole framework was shaped by it. It had never been turned on the agent's own reading, which is the one place the discipline was assumed rather than required.

The cost is asymmetric in the usual direction. A truncated read fails silently and reads as success: nothing errors, the content that *did* arrive is genuine, and the missing part is invisible precisely because it is missing. The states where a read gets truncated — a large governing document, a long procedure, a wide diff — are disproportionately the states where the missed content matters.

## Behavior

[§grounding](../../../framework/constitution.md#grounding) gains a subsection stating that a partial read is not a read.

- **A tool that elides content has not delivered the source.** Truncation to a preview, a saved-output pointer, a page or line cap, a `head`/`tail` window, a match-only search — each returns *part* of the subject. Treating the remainder as read is the same defect as not reading it, and §grounding's existing prohibition on reasoning from an unread source applies to the elided portion.
- **Two dispositions, and prose is not a third.** Either read the remainder — follow the pointer, page through the ranges — or state precisely what was not examined, in the artifact where a later reader meets the claim. Proceeding while treating the unread part as absent is the defect; mentioning it only in a transcript is the caveat channel [completion-claims-carry-no-caveats](completion-claims-carry-no-caveats.md) already rules out.
- **Prefer a reader that bounds itself explicitly.** A tool that reports the subject's size and takes an offset lets the agent know what it has not seen; a shell command whose output is capped downstream does not, because the cap is applied after the fact and says nothing about what was dropped relative to what was needed. Where both are available, the explicit reader is the grounded choice.
- **This is `QUAL-CLAIM-001` applied to the agent rather than to its code.** The rule requires a primitive's result to distinguish examined from unexaminable; this requires the agent's own reading to do the same, and names the constitution as the place that binding lives so a reviewer can cite it.

The rule ships to adopters, because the failure is not specific to this repository: any adopter whose `AGENTS.md`, constitution, or command procedures grow past a tool's output cap has the same hole, and the agent reading them has the same incentive to accept the preview.

## Edge Cases

- **A deliberate excerpt is not a partial read.** Reading one section, one function, or one range *because that is the subject* is grounded and complete. The defect is a read whose subject was the whole file and whose result was a fragment. The test is whether the agent can say what it did not see.
- **A search that reports matches is not a read of the file.** `grep` answers "does this token appear"; it does not answer "what does this file say". A negative result additionally depends on the pattern being right — this session also had a sweep miss every occurrence of "supersession" because the pattern was `supersed`, which the word does not contain. A pattern is an assumption about the subject and carries the same burden as any other.
- **An unreadable file is the already-covered case.** Where a source cannot be read at all, the existing discipline applies unchanged: it is named as unexamined, never silently dropped. This subsection covers the harder case where the read *partly* succeeded and therefore looks like success.
- **The rule cannot be mechanically enforced, and says so.** Nothing intercepts an agent's tool choice, so this is a governed requirement rather than a gate — cited by `/{project}:review` and `/{project}:analyze` when a claim rests on a partial read, in the same way §grounding's existing rules are cited. Stating the limit is itself the discipline: a rule that implied enforcement it does not have would be the defect it describes.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
