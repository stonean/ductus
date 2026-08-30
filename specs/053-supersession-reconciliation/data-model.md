# 053 — Supersession reconciliation Data Model

The structures reconciliation adds. All of them are runtime types on the MCP/exec wire — this feature has no database and no persisted artifact of its own (see the plan's Trade-offs for why there is no `reconciliation.md`).

## The claim

The unit reconciliation classifies. A superseded spec makes claims in two forms, and they are not interchangeable: an **acceptance criterion** records a delivery event and is never edited (AC6), while **body prose** describes current state and may be edited on a confirmed back-edge (AC5). The kind travels with the claim so a consumer cannot lose the distinction.

```rust
/// Where a claim came from, which decides what may be done to it.
pub enum ClaimKind {
    /// An acceptance criterion of the superseded spec, addressed by its
    /// stable `AC{n}` label. Never edited — annotated only.
    Criterion,
    /// A body section of the superseded spec, addressed by heading.
    BodyProse,
    /// A claim carried by one of the superseded spec's scenarios.
    Scenario,
}

pub struct Claim {
    pub kind: ClaimKind,
    /// `AC7` for a criterion, the heading text for body prose, the scenario
    /// slug for a scenario claim. Stable enough to address a write by.
    pub anchor: String,
    /// The claim's text as written, so the classifier judges what the spec
    /// says rather than a summary of it.
    pub text: String,
}
```

## The classification

```rust
/// What the superseding spec did to one claim.
pub enum Classification {
    /// The superseding spec declares it removes this. Annotated.
    Superseded,
    /// Untouched by the superseding spec. Left exactly alone — the default
    /// and the majority case.
    StillStanding,
    /// Contradicted without being removed. Surfaced to the operator and
    /// never resolved here, by picking a side or otherwise (AC2).
    Conflicting,
    /// Not decidable from the declared pair. Reported as such rather than
    /// guessed (AC8) — most often a claim that may never have been
    /// delivered, which needs a read of the tree that AC7 forbids, so the
    /// determination stays with the criterion-verification pass (AC9).
    Unclassified,
}

pub struct ClassifiedClaim {
    pub claim: Claim,
    pub classification: Classification,
    /// One sentence of reasoning, quoting what in the superseding spec
    /// drove it. Required for every classification except `StillStanding`:
    /// an operator settling a conflict needs to see the evidence, and a
    /// bare verdict is not reviewable.
    pub rationale: String,
}
```

`StillStanding` is a **recorded** classification rather than an omission. Leaving untouched claims out of the result would make "examined and still standing" indistinguishable from "never looked at" — the failure `QUAL-CLAIM-001` names, and the reason the counts below are over the classified set rather than over the conflicts alone.

## The bounded read

```rust
/// Args for `read-supersession-pair`. The absent arguments are the design:
/// there is no way to ask this primitive for a plan, a data model, a tasks
/// file, a source path, or a third spec (AC7).
pub struct ReadSupersessionPairArgs {
    /// The superseded feature — the one whose claims are walked.
    pub feature: String,
    /// The superseding feature named by the declaration.
    pub superseded_by: String,
}

pub struct ReadSupersessionPairResult {
    /// Both specs' bodies and criteria, and the superseded spec's
    /// scenarios. Nothing else is reachable from here.
    pub superseded: SpecRead,
    pub superseding: SpecRead,
    pub scenarios: Vec<ScenarioRead>,
    /// Scenario slugs and spec paths that could not be read or parsed.
    /// Named, and excluded from every count (AC12) — the shape
    /// `ScenarioQuestionScan.unreadable` already uses.
    pub unreadable: Vec<String>,
    /// Repo-relative paths actually read. The subject the verdict
    /// describes, so a caller quantifies "examined" rather than asserting
    /// it.
    pub examined: Vec<String>,
}
```

## The reconciliation result

The host's own record of one reconciliation pass, assembled from the classification and handed to the report step. It is not written to disk.

```rust
pub struct Reconciliation {
    pub superseded: String,
    pub superseding: String,
    /// Every claim examined, including `StillStanding` ones.
    pub classified: Vec<ClassifiedClaim>,
    /// Annotations actually written, by anchor. A re-run finds them
    /// already present and writes nothing.
    pub annotated: Vec<String>,
    /// Files that could not be examined, carried through from the read.
    pub unreadable: Vec<String>,
    /// Set when the pass could not establish a subject at all — a
    /// superseded spec with no criteria and no classifiable prose. Empty
    /// otherwise.
    ///
    /// This is what separates *examined and nothing to reconcile* from
    /// *examined and clean* (AC11). Without it both are an empty
    /// `conflicts` list, and the reassuring reading is the wrong one.
    pub guidance: String,
}
```

### Reading the result

The three states are distinguished by construction, not by the reader's care:

| State | `classified` | `unreadable` | `guidance` |
| --- | --- | --- | --- |
| Examined, no conflicts | non-empty, no `Conflicting` | empty | empty |
| Examined, conflicts to settle | non-empty, some `Conflicting` | empty | empty |
| Nothing to reconcile | empty | empty | set |
| Could not fully examine | any | non-empty | either |

A pass is **complete** only when `unreadable` is empty and `guidance` is unset (AC3). Any other combination is reported as incomplete, and the report names which files were not examined rather than folding them into a total.

## The criterion annotation

The written form, appended to the criterion's existing line. The checkbox and the criterion's own text are untouched (AC6), so the line grows and nothing in it changes:

```text
- [x] AC4: The generated workflow files are written during bootstrap — superseded by 043-workflows-sunset: the generated files no longer exist.
```

**Cited by name, never linked.** A criterion is a plain list item with no blockquote exemption, so a markdown link here is harvested by `derive-dependencies` into the annotated spec's `dependencies:` — giving it a dependency on its own successor. The whole-spec banner may link precisely because it is blockquoted (constitution `§supersession-annotations`; spec 052 AC14/AC15).

## Notes

- **Idempotence key** — a criterion already carrying an annotation naming this superseding spec is left alone, matching the whole-spec form's `already-present` outcome. The predicate is the shared `blockquote_cites`' non-blockquote sibling: same slug-boundary matching, so `043-workflows` is not satisfied by `043-workflows-sunset`.
- **No status field anywhere.** Reconciliation never writes a spec's `status`. The annotation is a mechanical edit; the one path that can reopen a spec is the confirmed body-prose edit, and that goes through `set-status` like every other back-edge, not through anything here.
