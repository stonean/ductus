# 053 — Supersession reconciliation Plan

Implements [053 — Supersession reconciliation](spec.md).

## Overview

One new extension point, one widened primitive, one new bounded reader, and the command wiring that runs them at declaration time. Nothing here invents machinery: `write-supersession-annotation` already frames a supersession annotation, `check-artifacts`' families already report what they could not examine, `read-spec` already scans a feature's scenarios and names the ones it could not read, and the five `performReview` passes already establish how semantic judgment crosses the runtime boundary.

The shape follows from one observation: **classifying a claim is judgment, and everything around it is mechanical.** So the reads are a primitive (which is what makes the §Scope-of-the-read bound structural rather than a rule the host remembers), the classification is an extension point, and the writes are the annotation primitive it already has.

Reconciliation is deliberately **not** a new artifact. It produces no `reconciliation.md`: its outputs are annotations on the superseded spec and a report the operator acts on at declaration time. A durable report would be a fourth spec artifact whose staleness nothing owns.

## Technical Decisions

### The bounded read is a primitive, so the bound is structural

`read-supersession-pair` returns both specs' bodies and criteria plus the superseded spec's scenarios, and can return nothing else. AC7 forbids reading a plan, data model, tasks file, source tree, or third spec — and a rule the host is asked to remember is a diligence dependency, which §design-principles rejects. Putting the read behind a primitive that has no argument for those paths makes the bound a property of the code.

It reuses `list_scenario_files` (`runtime/src/primitives/mod.rs:1939`) so scenario ordering matches every other surface, and it reports unreadable scenarios the way `collect_scenario_open_questions` already does — `ScenarioQuestionScan.unreadable` (`runtime/src/primitives/read_spec.rs:132-139`) exists for exactly the reason AC12 states, and its doc comment already argues the case: "a scenario that cannot be read yields no questions, which is indistinguishable from one that was read and carried none unless the scan says so."

### Classification is an extension point, not a primitive

A new `<!-- llm:classifyClaims -->` marker, registered in `build_extension_request` (`runtime/src/interpreter/payload.rs:154-171`) beside `routeFold` and `performReview`. Deciding whether a superseding spec *removes*, *contradicts*, or *leaves alone* a predecessor's claim is a reading of two documents; no primitive can do it, and §runtime-boundary principle 2 forbids pretending otherwise.

The vocabulary is deliberately its own rather than `routeFold`'s or `routeInboxItem`'s — those answer *where does this belong*, this answers *what did the later spec do to this claim*.

### `write-supersession-annotation` gains a granularity, rather than a second writer

The criterion-level annotation is a new *form*, not a new *concept*: the constitution's `§supersession-annotations` defines one rule at three granularities. Adding a separate `annotate-criterion` primitive would put two writers behind one concept — and spec 052's review found exactly that defect, where two surfaces answered "is this spec already annotated?" differently and drifted.

So the existing primitive takes an optional `criterion` label. Absent, it writes today's whole-spec banner. Present, it appends the inline annotation to that criterion, citing the superseding spec **by name** — a criterion is a plain list item with no blockquote exemption, so a link there would be harvested into `dependencies:` (constitution `§supersession-annotations`; spec 052 AC15).

The **section-level** granularity stays hand-authored. The constitution documents it, nothing in this spec needs it programmatically, and a third code path serving no caller is the overengineering the simplicity pass exists to catch.

### The criterion is annotated, never edited, and that is enforced by the write shape

AC6 forbids editing a superseded criterion. The primitive appends to the criterion's line and touches neither its checkbox nor its text, so a superseded criterion stays ticked. This is the same structural guarantee the whole-spec form already gives for frontmatter: `write_supersession_annotation` splices the head back byte for byte, which is what makes "the status is untouched" a property of the code rather than a rule it remembers.

`mark-criterion` remains the only writer of a criterion's checkbox. Two primitives touch a criterion line and they own disjoint halves of it.

### Conflicts are reported, never resolved

AC2 is a refusal, and the cheapest way to keep a refusal true is to give the code no way to break it: the classification result carries a conflict list, and no primitive consumes it. Resolution is the operator's, in conversation.

Body-prose edits (AC5) are the one place reconciliation may change a claim, and they run through `gate-confirm` with a prompt naming the `done → in-progress` back-edge **before** the edit — the same shape `/{project}:groom` and `/{project}:fold` use for the reopen they cause.

### Three outcomes that must not read alike

The result distinguishes them by construction rather than by a caller's care (AC3, AC11, AC12):

| Outcome | Shape |
| --- | --- |
| Examined claims, none conflict | non-empty `classified`, empty `conflicts`, empty `unreadable` |
| Examined, nothing to classify | empty `classified` **and** a `guidance` string naming the empty subject |
| Could not examine | non-empty `unreadable`, excluded from every count |

This is `check-corpus-links`' contract and `check-orphaned-references`' before it — the pattern `QUAL-CLAIM-001` shaped.

### Declaration time, on both routes

Reconciliation is a step in the shared **Declaration semantics** reference in `supersede.md`, which `specify.md` already points at rather than restating (spec 052). Both declaration routes therefore reconcile by construction, and AC10's "not at the completion gate" needs no separate guard: nothing in `check-review-gate` learns about reconciliation at all.

## Affected Files

| File | Action | Purpose |
| --- | --- | --- |
| `runtime/src/schema/primitives.rs` | Modify | `read-supersession-pair` args/result; the `criterion` argument on the annotation writer |
| `runtime/src/primitives/read_supersession_pair.rs` | Create | The bounded read of the declared pair plus scenarios |
| `runtime/src/primitives/write_supersession_annotation.rs` | Modify | Criterion-level granularity, citing by name |
| `runtime/src/interpreter/payload.rs` | Modify | Register the `classifyClaims` extension point and build its request |
| `runtime/src/interpreter/mod.rs` | Modify | Exec-path arm for the new primitive |
| `runtime/src/main.rs` | Modify | CLI enum plus dispatch arm |
| `runtime/src/mcp/server.rs` | Modify | Register the new primitive |
| `runtime/src/schema/registry.rs` | Modify | `PRIMITIVE_REGISTRY` entry |
| `framework/runtime-tools.txt` | Modify | Shipped manifest entry |
| `framework/commands/supersede.md` | Modify | Reconciliation steps in the canonical Declaration semantics |
| `framework/commands/specify.md` | Modify | Point its declaration step at the reconciliation the shared reference now carries |
| `framework/commands/analyze.md` | Modify | Name the reconciliation outcomes it can surface |
| `framework/constitution.md` | Modify | Record that a criterion annotation may be written by the runtime |
| `runtime/CHANGELOG.md` | Modify | The release this runtime change obliges |

## Trade-offs

**A granularity argument rather than a second annotation primitive.** A separate `annotate-criterion` would leave each function with one unqualified job. Rejected because the two forms are one concept the constitution already states as one rule, and 052's review found the concrete cost of splitting a concept across two implementations: they answered the same question differently and nobody noticed until a review compared them.

**The bounded read as a primitive rather than command prose.** Prose is cheaper and it is what AC7 literally asks for. Rejected because §design-principles forbids a rule that depends on the host's diligence when the code can hold it instead — and the read bound is the one thing standing between reconciliation and the corpus-wide check that was measured and rejected.

**No `reconciliation.md`.** A durable report would make the classification reviewable later and would survive the session. Rejected: nothing would own its staleness, it would become a fourth artifact every completeness check has to reason about, and the spec asks for claims to be *surfaced to the operator* at declaration time, not archived.

**Section-level granularity left hand-authored.** Accepted limitation. The constitution documents the form; an adopter writes one by hand, as twelve specs in this corpus already did. If a caller ever needs it, the granularity argument is where it goes.

**Known limitation:** reconciliation classifies what the superseding spec *declares* it removes. It cannot tell that a claim was never delivered in the first place — that needs a read of the tree, which AC7 forbids — so such a claim is reported unclassified and the determination stays with the criterion-verification pass (AC9). A reader who expects "unclassified" to mean "nothing to do" will be wrong; the result says which of the two it is.
