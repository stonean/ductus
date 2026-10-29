---
section: "Shape of the setting"
---

# Target-reports-each-constitutions-size

## Context

[075](../../075-agents-md-holds-what-every-session-needs/spec.md) counts each registered shared constitution in a session's every-session load, beside the framework constitution and `AGENTS.md`, and asks `/{project}:target` to report its size. Nothing measured it, so it was the one file in that load with no limit anyone could see.

## Behavior

- **Measured on load.** `resolve-constitutions` measures each `loaded` document's bytes on disk against `[artifacts] read-size-bytes`, 50,000 by default. It adds to the document's record the size, whether it is over the limit, and a warning the runtime words: the alias, the size, the limit, and a fix that is a cut in the repository its `repo` names. The result carries the threshold it measured against.
- **Only when something is registered.** With no `[constitutions.*]` entry, nothing new is read and the result is unchanged, so AC1 holds.
- **Two callers stay unmeasured.** `write-review` and `write-analysis` resolve constitutions through the same function, and they keep the resolution without measurement. So a malformed `read-size-bytes` halts `/{project}:target` but never newly halts either of them.
- **`/{project}:target` step 4** shows each loaded source's size beside its alias, and the warning for one over the limit. It never blocks. Nor is the report an `/{project}:analyze` finding, since nothing in the project can fix, route or discard it.
- `data-model.md` records the new fields.
- A runtime test proves that a fixture constitution one byte over the limit is named as over it, with its owning repository as the fix; that one at the limit is not; and that the run continues. The test fails with the measurement removed.

## Edge Cases

- A `skipped` entry (`not-checked-out`, `no-constitution-document`) carries no size, and is reported as *not measured*, never as under the limit.
- A malformed `read-size-bytes` halts `/{project}:target`, naming the setting, the value, the file and the accepted form, as it halts `/{project}:clarify`, `/{project}:plan` and `/{project}:analyze`.
- Two aliases on one checkout measure one document, and both report its size.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
