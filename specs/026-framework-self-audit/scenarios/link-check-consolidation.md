---
section: "Follow-on scenarios"
---

# Link-check-consolidation

## Context

Spec 052's review left three findings against audit families this spec owns. They are one conversation rather than three, because the first two have a single answer between them.

- **Family 26 resolves a root-absolute link target against the filesystem root, not the repo root.** `os.path.join(here, '/specs/x.md')` discards `here` entirely — the exact defect fixed in `check-corpus-links` while 022 was closing. No such link exists in this corpus today, so the family is not currently wrong about anything; it would be the moment one is written, and it would be wrong in the *reassuring* direction on a machine that happens to hold that absolute path.
- **Family 26 and `check-corpus-links` now perform substantially the same check** over overlapping subjects. The family covers the whole repository including maintainer-only files; the primitive covers the spec corpus and is what adopters actually run. Spec 022's scenario deliberately left delegation undecided — "Family 26 is not necessarily retired by this" — and named the Family 30 shape (logic in the runtime, script as entry point) as the obvious candidate.
- **Family 33 recognizes a documented command by the backticked `/name` token**, so a command appearing only inside a wider code span — `` `/specify --supersedes` `` — reads as undocumented. The failure is loud and in the safe direction, but it is an undocumented constraint on how the README may write a command name, and nothing states it where an author would meet it. It was met on the very next edit after the finding was recorded.

The first two are the same decision. Fixing Family 26's resolution in place leaves two implementations of one check to keep in step — and this session already paid for that shape once, when two surfaces disagreed about whether a spec was annotated and only a review comparing them noticed. Delegating collapses the divergence into one implementation instead of fixing it twice.

## Behavior

Family 26 and `check-corpus-links` answer link resolution through **one** implementation.

- The family becomes an entry point over the primitive, the shape Family 30 already uses: the check lives in the runtime, the script sources `lib.sh`, renders findings through `emit`, and exits `"$drift"`.
- The subjects stay different and that difference is stated. The family's subject is the whole repository, including maintainer-only files an adopter's primitive has no reason to know about; the primitive's subject is the spec corpus. One resolution rule, two scopes.
- Root-absolute targets resolve against the **repository** root on both, as a markdown renderer does.
- Family 33's token-recognition rule is stated where an author meets it — in the family's own header and in `scripts/audit/README.md` — so "write the bare `/name` somewhere" is a documented requirement rather than a constraint discovered by a failing check.

## Edge Cases

- **Delegation must not narrow the family's subject.** The primitive walks the spec root; the family walks the repository. If delegation is implemented by simply calling the primitive, the family silently stops examining `framework/`, `scripts/`, and its own documentation — a smaller subject nobody stated, which is the failure `QUAL-CLAIM-001` names and the one this consolidation is most likely to introduce. Whatever the primitive gains to serve both callers, the family's examined-file count must not drop.
- **A decision not to delegate is a valid outcome**, and it is not free: it must then fix the root-absolute resolution in Family 26 directly and record why two implementations are worth keeping, so the next reader does not re-open the question.
- **Family 33's constraint may instead be removed** rather than documented — recognizing a command inside a wider code span would make the rule unnecessary. That is a larger change to the matcher and risks matching prose that merely mentions a flag; documenting the constraint is the cheaper answer and the one to beat.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
