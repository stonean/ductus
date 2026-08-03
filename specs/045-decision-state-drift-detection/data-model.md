# 045 — Decision-State Drift Detection Data Model

Canonical source for the open-state tell list, the tell → contradicted-state mapping, the acceptance-criterion path grammar, and the `SkippedTarget` shape. Referenced from `framework/constitution.md` §drift-prevention (Canonical sources) so the list has one owner rather than three restatements.

Every tell below is written inside a code span on purpose: the check exempts inline code spans, so this document describes the tells without tripping them.

## Open-state tells

Exactly six entries, framework-fixed, with no per-project configuration surface (AC11). `open question` and `open questions` are one entry matched as a stem.

| # | Tell | Class |
| --- | --- | --- |
| 1 | `open question` / `open questions` | question-state |
| 2 | `unresolved` | question-state |
| 3 | `still open` | question-state |
| 4 | `not yet` | implementation-state |
| 5 | `does not exist` | existence |
| 6 | `left unimplemented` | implementation-state |

Matching is case-insensitive on ASCII and substring-based within the block's text, at byte offsets outside every inline code span.

Two entries were dropped from the seed list during clarification and must not be re-added without new evidence: `TBD`, which marks the *citing* document's incompleteness and so can never be a true positive under the co-occurrence design, and `deferred`, which contradicts the convention that a deferral is a resolution-with-a-condition belonging in `## Resolved Questions`.

## Target states and the contradiction mapping

A link target is read for whatever state it carries. Three target kinds:

| Target kind | Readable state |
| --- | --- |
| `spec.md` | frontmatter `status`, spec-body open-question count, file existence |
| `scenarios/*.md` | open-question count, file existence — a scenario has no `status` |
| `plan.md`, `tasks.md`, `data-model.md` | file existence only |

A finding is emitted only where a tell class and a readable state actually contradict:

| Tell class | Contradicted when |
| --- | --- |
| question-state | the target's open-question count is zero |
| implementation-state | the target is `spec.md` with `status` in {`in-progress`, `done`} |
| existence | the target file exists |

Every other pairing yields nothing. Concretely, per AC14: a question-state tell against a scenario fires on the scenario's own open-question count, while an implementation-state tell against a scenario produces no finding — a scenario carries no lifecycle status, and AC9 forbids escalating an unknown to a defect.

A target file that does not exist yields no readable question-count or status, so only the existence class is evaluable against it — and an existence tell against a missing target is *consistent*, not contradicted. Nothing is emitted; the target is recorded as skipped instead.

## Finding shape

Reuses the existing `ArtifactFinding` (`runtime/src/schema/primitives.rs:2189`) with no structural change:

| Field | `link-adjacent-drift` | `criterion-path-existence` |
| --- | --- | --- |
| `family` | `link-adjacent-drift` | `criterion-path-existence` |
| `severity` | `advisory` | `advisory` |
| `path` | repo-relative path of the **citing** artifact | repo-relative path of the citing `spec.md` |
| `message` | citing line number, the link target, every tell that fired in list order, and the target's contradicting state | the criterion text, the named path, and that it does not resolve |

One finding per (block, link) pair; the message carries the line number, so AC4's "citing file and line" is satisfied without adding a field the other five families do not carry.

## `SkippedTarget`

New type on `CheckArtifactsResult`, satisfying `QUAL-CLAIM-001` without violating AC9.

```rust
/// One target a family could not examine. Distinguishes "examined and
/// found nothing" from "could not examine" — a family that emits no
/// finding because a target was unreadable says so here rather than
/// letting `clean` read as positive assurance (QUAL-CLAIM-001).
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub struct SkippedTarget {
    /// Family that skipped it, matching `ArtifactFinding::family`.
    pub family: String,
    /// Why it could not be examined.
    pub reason: String,
    /// Repo-relative path of the target that was not examined.
    pub path: String,
}
```

`reason` is a closed set of strings, so repeat runs are byte-identical:

| `reason` | Raised when |
| --- | --- |
| `target-missing` | the link resolves to a path with no file on disk |
| `target-unparseable` | the target exists but its frontmatter will not parse |
| `no-readable-state` | the target carries no state the tell's class can be evaluated against |

`clean` keeps its existing definition — `findings.is_empty()` — so no existing consumer changes. A non-empty `skipped` alongside `clean: true` is the state the host renders in the Informational tier.

## Acceptance-criterion path grammar

Applies to `criterion-path-existence` only, over the contents of inline code spans inside a `done` spec's `## Acceptance Criteria` entries.

A span's whole trimmed content is a candidate path when **all** hold:

- it contains at least one `/`;
- it contains no whitespace;
- it contains none of `{`, `}`, `*`, `?`, `[`, `]`, `<`, `>`, `$`, `|`, `:`;
- it does not begin with `-` or `/`.

A candidate resolves when the repo-root-relative path exists as either a file or a directory; a trailing `/` is stripped before the test. Anything else emits a finding.

Worked example — 026's AC5 (AC18), after `531e3ea` deleted both subjects:

| Span content | Candidate | Resolves |
| --- | --- | --- |
| `framework/workflows/registry.json` | yes | no → finding |
| `scripts/audit/registry-equivalence.sh` | yes | no → finding |

Rejected by the grammar, and why each exclusion is load-bearing in this repo: `/{project}:analyze` (leading `/`, `{`, `:`), `https://example.com/x` (`:`), `runtime/src/primitives/mod.rs:841` (`:`), `specs/*/spec.md` (`*`), `--staged` (leading `-`).
