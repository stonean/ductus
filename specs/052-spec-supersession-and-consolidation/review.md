---
spec: 052-spec-supersession-and-consolidation
reviewed-at: 2026-08-30T20:23:28Z
reviewed-against: 9f51730821bf1e89105bfa4c784c3820391f8cc9
diff-base: 86657b3a3386cf9c45bd55a6880e22f4528da67e
must-violations: 0
should-violations: 0
low-confidence: 0
captured-issues: 0
skipped-passes: []
---

# Review — 052-spec-supersession-and-consolidation

## Summary

Scope was this spec's whole surface: the `supersedes:` frontmatter field and its shape validation, `write-supersession-annotation`, the gated `retire-feature` refusal, the `supersession-reciprocity` check family, the three command files (`specify.md`'s flag, the new `supersede.md` and `consolidate.md`), `fold.md`'s account of the gate, the constitution's `§supersession-annotations`, the installer manifest, the README, and 051's post-completion note.

All five passes ran. Three defects were found and fixed rather than recorded, each in the reviewed spec's own scope.

The reuse pass found the substantive one: two surfaces answered the same question — "is this spec already annotated as superseded by that one?" — with different predicates. `write-supersession-annotation` accepted only the markdown link form; the reciprocity family accepted a link *or* a bare name, and documents that contract. Hand-written annotations cite by name, which is the entire corpus the retroactive declaration path exists for — twelve specs at the time this spec was written — so `/{project}:supersede` over one of them stacked a duplicate annotation onto a spec the check already reported as reciprocally annotated. The predicate is now shared (`blockquote_cites`), and matching moved to a slug boundary: a substring match reads `043-workflows-sunset` as a citation of `043-workflows`, and naming a sunset spec after what it sunsets is this corpus's convention rather than an edge case. On the reciprocity side that had been failing in the direction that matters — reporting a missing annotation as present, in the family built to catch that omission. Recorded as task 16 rather than fixed silently, and fixed in `1a78e73`.

The quality pass found two more, both about paths the command files describe rather than the runtime. `consolidate.md`'s report step named `rewrite-spec-links` in backticks, which makes it a **primitive** step — so `ductus exec` would dispatch the rewrite a second time, after the directory it re-points away from is already gone. Caught on the first run of the new step-order tests (`d77e7c5`), which is what those tests exist for. And neither new command documented its `ductus exec` reduction: `/{project}:supersede`'s frontmatter append has no primitive, so an exec walk writes the annotation and never the key — half a declaration, and the half that leaves the spec invisible to every check that walks declared edges. Both reductions are now stated (`9f51730`), per the two-paths guarantee.

Nothing outstanding. The security pass produced no findings: `retire-feature`'s traversal guard runs on both arguments **before** the gated refusal, so the opt-in widens which *forms* are removable and never what paths are reachable; the anti-stranding refusal is untouched and applies to both callers. The efficiency and simplicity passes produced none — the annotation primitive splices the frontmatter back byte for byte rather than re-serializing it, which is what makes "the status is untouched" structural rather than a rule the code remembers.

## MUST violations (blocking)

*None.*

## SHOULD violations (advisory)

*None.*

## Low-confidence findings

*None.*

## Waived findings

*None.*

## Captured issues

*None.*

## Observations

- convention: `/{project}:consolidate` never re-targets the session, so an operator whose session pointed at the removed spec is left pointing at a directory that no longer exists until they run `/{project}:target`. This is deliberate and the command reports it — consolidating a spec asserts its content belongs with the target, not that the operator's next work does — but `/{project}:fold` does re-target in the same situation, and the two commands giving opposite answers to the same stranded-session problem is worth a decision recorded somewhere, rather than two commands that each look locally reasonable. — `framework/commands/consolidate.md`

## Skipped passes

*None.*
