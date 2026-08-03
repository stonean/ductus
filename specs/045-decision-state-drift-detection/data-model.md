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
| 5 | `does not exist` | implementation-state |
| 6 | `left unimplemented` | implementation-state |

Entry 5 was classed as an *existence* tell — contradicted by a target file that is present — until the full-repo run of 2026-08-02 showed that test can only ever fire and never filter: a link that resolves always points at a present file, so the check cannot fail. Its single finding across 47 specs was a false positive (`017/detect-dependency-cycles.md:28`, whose prose says an *override mechanism* "does not exist today" while linking to a scenario that does). Judging it against the target's lifecycle status is the reading in this spec's `## Behavior` section, and the one that can be wrong.

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

Every other pairing yields nothing. Concretely, per AC14: a question-state tell against a scenario fires on the scenario's own open-question count, while an implementation-state tell against a scenario produces no finding — a scenario carries no lifecycle status, and AC9 forbids escalating an unknown to a defect.

File existence still governs whether a target is examinable at all: a target that does not resolve yields no readable question-count or status, so nothing is emitted and it is recorded as skipped instead.

## Finding shape

Reuses the existing `ArtifactFinding` (`runtime/src/schema/primitives.rs:2189`) with no structural change:

| Field | `link-adjacent-drift` | `criterion-path-existence` |
| --- | --- | --- |
| `family` | `link-adjacent-drift` | `criterion-path-existence` |
| `severity` | `advisory` | `advisory` |
| `path` | repo-relative path of the **citing** artifact | repo-relative path of the citing `spec.md` |
| `message` | citing line number, the link target, every tell that fired in list order, and the target's contradicting state | the criterion text, the named path, and that it does not resolve |

One finding per (block, link) pair; the message carries the line number, so AC4's "citing file and line" is satisfied without adding a field the other families do not carry.

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
| `root-absent` | a criterion's path names a top-level segment this repo does not contain |
| `artifact-unreadable` | a scanned *citing* artifact could not be read at all |

`clean` keeps its existing definition — `findings.is_empty()` — so no existing consumer changes. A non-empty `skipped` alongside `clean: true` is the state the host renders in the Informational tier.

## Acceptance-criterion path grammar

Applies to `criterion-path-existence` only, over the contents of inline code spans inside a `done` spec's `## Acceptance Criteria` entries.

A span's whole trimmed content, with surrounding quotes and a leading `./` stripped, is a candidate path when **all** hold:

- it contains a `/` that is not its final character — a token whose only separator is trailing is a bare directory *name* used conceptually ("the feature's `scenarios/` directory"), not a path to resolve;
- it contains no whitespace;
- it contains none of `{`, `}`, `*`, `?`, `[`, `]`, `<`, `>`, `$`, `|`, `:`;
- it does not begin with `-` or `/`;
- it does not contain `NNN`, the framework's spec-number placeholder — the unbraced sibling of the `{…}` forms above;
- it is entirely ASCII, which in practice rejects the `…` of an elided path (`scripts/…`).

A candidate resolves when the repo-root-relative path exists as either a file or a directory; a trailing `/` is stripped before the test.

### The criterion must be a live claim

A path is only checked when its criterion actually claims the path is **present**. A criterion carrying any of these thirteen phrases is exempted whole, and each of its paths is recorded as `not-a-live-claim`:

| Group | Phrases | Why the finding would be backwards |
| --- | --- | --- |
| deletion / retirement | `is deleted`, `are deleted`, `does not exist`, `no longer exists`, `is removed`, `are removed`, `since retired` | The criterion is *satisfied* by the path being gone |
| rename | `is renamed to`, `are renamed to`, `renamed from` | The old path is named deliberately |
| adopter scope | `in the project` | Describes a scaffolded checkout, not this one |
| hedge / example | `if it exists`, `e.g.` | Claims nothing at all |

This is the open-state tell list's co-occurrence design **inverted**. There, a phrase asserting an open state is contradicted by a target that is closed. Here, a phrase asserting *absence* is **confirmed** by a path that does not resolve. Same closed-list, framework-fixed discipline, for the same reason: a per-project list would make the promotion threshold measure configuration rather than drift.

The phrases are matched as phrases, not words, and that is load-bearing: bare `adopter` would exempt 018's `runs the adopter-relevant generators (currently \`scripts/gen-spec-deps.sh\`)`, which is a genuine stale path.

The whole criterion is exempted rather than the matched path, because a criterion about a transition names its endpoints together.

An unresolved candidate emits a finding **only when its own top-level segment exists in this repo**. When that segment is absent, nothing can be proven — a framework repo's criteria legitimately name paths that live in an *adopter's* checkout (`.govern/…`, `.agents/…`) — so the candidate is recorded as `root-absent` instead. The rule self-corrects where it matters: in an adopter repo those roots do exist, so real drift beneath them is provable again.

Worked example — 026's AC5 (AC18), after `531e3ea` deleted both subjects:

| Span content | Candidate | Resolves |
| --- | --- | --- |
| `framework/workflows/registry.json` | yes | no → finding |
| `scripts/audit/registry-equivalence.sh` | yes | no → finding |

Rejected by the grammar, and why each exclusion is load-bearing in this repo: `/{project}:analyze` (leading `/`, `{`, `:`), `https://example.com/x` (`:`), `runtime/src/primitives/mod.rs:841` (`:`), `specs/*/spec.md` (`*`), `--exclude=a/b` (leading `-`), `scenarios/` (no internal separator), `scripts/…` (non-ASCII), `specs/NNN-feature/review.md` (`NNN`).

## Measured precision (2026-08-03)

Across all 47 specs, after the live-claim exemption landed:

| | Findings | Skips |
| --- | --- | --- |
| `criterion-path-existence` | 28 | 35 `not-a-live-claim`, 21 `root-absent` |
| `link-adjacent-drift` | 0 | 1 `no-readable-state` |

Triaging the 28 **by reading each criterion**, not by classifying its path:

| Class | Count | Verdict |
| --- | --- | --- |
| Real stale paths | 5 | True |
| Paths `govern` creates *in an adopter project* | 19 | False here, would resolve there |
| Residual | 4 | False |

The five true positives are `specs/triage.md` in 006 (×2, renamed to `specs/inbox.md` alongside `/{project}:triage` → `/{project}:groom`), `scripts/gen-spec-deps.sh` in 018 (×2, moved to `.govern/scripts/` by 042), and 005's criterion still asserting a workflow registry `exists` after 043 sunset the feature.

**The 19 are a dogfooding artifact, not a check defect.** `.govern/constitution.md`, `specs/rules/security-backend.md`, `specs/system.md`, `.githooks/govern-pre-commit`, `specs/templates/` — every one is a path `govern` *creates in an adopter's checkout*. They are absent here for the single reason that `govern` is the source rather than an adopter, and each criterion naming one is correct and satisfied in the repo it describes. `root-absent` cannot catch them because their top-level segments (`specs`, `.govern`, `.githooks`) do exist here. The class is structural to a framework repo documenting adopter layout while also being its own adopter, and it does not generalize to the projects this check ships to.

**Promotion verdict: do not promote — and re-measure in an adopter repo before deciding.** The volume half of the criterion is met several times over. The precision half reads 5/28 (18%) here and roughly 5/9 (56%) once the framework-repo artifact is set aside, and neither number is the one that matters: this repo is the least representative sample the check will ever run against.

**Correction to a prior record.** An earlier pass reported 35 true positives at 69% precision. That number came from classifying findings by *path prefix* without reading the criteria they came from, and it was wrong — most of what it counted were deletion criteria (`X is deleted`, satisfied *because* the path is gone), rename criteria naming their own source path, and adopter-scoped paths. Reading the criterion text is what produced both the live-claim exemption above and the honest figure here. The verdict happened to be right; the reasoning behind it was not.

The originating case behaves better in the wild than the acceptance criterion anticipated. 026's AC5 has itself been rewritten since this spec was authored — it now reads in past tense (`verified … Met at v1 and since retired`), so the live-claim exemption now skips it rather than flagging it. That is the correct outcome and the scope boundary §Behavior argued for, observed rather than asserted: a contract claiming a path is present is a defect when it is gone, the same path in a past-tense record is a true statement. The unit test pins both paths in the present-tense form the criterion had when the case was found, which is what AC18 specifies.
