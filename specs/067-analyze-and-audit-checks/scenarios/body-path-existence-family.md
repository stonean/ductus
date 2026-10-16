---
section: "Artifact checks"
---

# Body-path-existence-family

## Context

`criterion-path-existence` checks paths only in a `done` spec's acceptance criteria, and it is scoped that way on purpose: body prose may name a dead path correctly while describing history, and a check over whole bodies would flag those true statements (045's Resolved Questions; [criterion-path-existence-family](criterion-path-existence-family.md)). So nothing checks the rest of a spec, and dead references accumulated there. A sweep on 2026-10-16 found 75 stale Affected Files rows across 20 plans, and a backticked-path scan of done specs' bodies, scenarios and data models still turned up about 610 unresolved candidates.

Two discriminators make a body-wide check possible without flagging true history:

- **A dead reference is a path that existed in this repository and no longer does.** An example (`runtime/src/primitives/new_thing.rs`), a hypothetical, or a rejected alternative that was never built (`scripts/normalize-permissions.sh`) never existed. It is a name, not a file, and nothing about it can go stale.
- **A line that describes a transition is not a claim the path is present.** The criterion family's fourteen markers already say so for deletions, renames, migration subjects, adopter scope and hedges. Body prose also states transitions in imperative and tabular forms criteria do not use — `Delete`, `Remove`, a `` `old` → `new` `` rename — so the body family adds those.

Measured over the done specs with a prototype of this design: about 610 raw candidates; 224 after the criterion family's grammar, `root-absent` and `ships-to-adopter`; 134 after the history gate; 70 in 17 specs after the earned markers. Each extra marker was measured alone and kept only when what it exempted was a transition statement.

## Behavior

- **A tenth `check-artifacts` family, `body-path-existence`, advisory, on `done` specs only.** It reads `spec.md` outside `## Acceptance Criteria` (the criterion family owns that section), `plan.md`, `data-model.md` and every scenario, skipping fenced code blocks and HTML comments, and only inside inline code spans.
- **Candidates follow the criterion family's grammar**, with a `#fragment` stripped first. A candidate starting `../` resolves from the citing file's directory and is dropped if it climbs out of the repository; any other resolves from the project root.
- **An unresolved candidate is examined in order.** A missing top-level segment is recorded `root-absent`; an adopter destination `ships-to-adopter`; a line carrying a criterion marker, a body marker, or a negated-creation clause `not-a-live-claim`. The rest are checked against history.
- **History decides.** One walk from HEAD per call, every remaining candidate batched, stopping once each is found. A candidate no revision ever held is recorded `never-existed`, a new reason in the excluded-by-construction class. One a revision held is a finding that quotes the line, so the fix is in front of the reader.
- **A history that cannot be consulted flags nothing.** No repository, a shallow clone or an unreadable revision records each remaining candidate as `no-readable-state`.
- **The seven body markers are body-only.** Listed canonically in [data-model.md](../data-model.md) §Body transition markers, they are added to the criterion family's fourteen, not merged into them, so the criterion family's list is unchanged.
- **Markers match the whole logical line.** A logical line is a paragraph, a list item, a blockquote paragraph or a table row with its hard wraps joined, so a `| path | Delete |` row exempts itself without a table parser, a wrapped sentence keeps a marker that lands on its first physical line, and a double-backtick span quoting an example is read whole.

## Edge Cases

- **A transition stated without a marker.** "The generators under `.ductus/scripts/` are executed by adopters" names a deleted path as present. It is a finding: the line is stale, and living documents are edited to say what is true.
- **A marker on an unrelated clause of the line.** `remove` in "to remove a dep, remove the inline link" exempts a stale path named elsewhere on the same line. The family errs toward silence, as the criterion family does; a one-time sweep reads the exempted lines by eye.
- **An example relative link that resolves to a real past path.** `../spec.md` cited from a plan resolves to `specs/spec.md`, which existed before it became `specs/README.md`. The history gate cannot tell an example from a reference there; the finding is advisory, and a stored discard settles it.
- **A spec below `done`.** Not scanned: a spec in flight may name files it is about to create.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
