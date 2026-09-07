---
status: in-progress
dependencies: [017-derive-dont-ask, 022-deterministic-runtime, 023-govern-refinement, 024-rule-loader, 025-rule-opt-out]
review:
  last-run: 2026-09-07T22:39:12Z
  reviewed-against: 9b855180eee416407f576f2b25e9869aa70f59df
  must-violations: 0
  should-violations: 0
  low-confidence: 0
  reviewed-digest:
    scenarios/audit-ci-hard-gate.md: a7bad7a167532019112d79a746696ad32d963171598d2283072af0e9f3234be7
    scenarios/audit-script-refactors.md: f27e039741ab1fce4c868b2af03a84ac3626f780e7af7b092abc8c9a112081a2
    scenarios/family-10-migration-coverage.md: 988d53541ffc5ea7fd752e698de8143c31dcb7b6b0a49f590deb31da777df76f
    scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
    scenarios/family-18-marker-list-parity.md: e2f5e57dc51efec37f63fdc419d1c78ae4afb8319bcd41b604a2c22507af6296
    scenarios/family-19-mechanical-sweep-exemption.md: 1aed9678cb90da55fb314f9f8bd26addb3baebdbad04bda48fb11deeb95c4089
    scenarios/family-19-review-freshness.md: 15df43ccf633bb8813ecb0ab8555be0a46ab9aca2b66419f75836262a88950e5
    scenarios/family-19-says-what-it-examined.md: cbdb9363ab1e3b38938864a44fe4a5ae90ca36294631b900abdab43789331ee7
    scenarios/family-22-adopter-shell-behavior.md: 2eac3b7db9587dd354168941be8a1816d11a9e6f7fb27ecd8b8f9c9f8f719754
    scenarios/family-23-sweep-target-manifest-parity.md: 9dfa1299cdeb27ad691c66d6e9adeb68187c7c60039576cafd6ba55f29274b38
    scenarios/family-24-rename-sweep-residue.md: 7853d334d6836dfb565348172ef47377f73aa1687ab0e22f04d1e3cd18f63634
    scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
    scenarios/family-26-broken-relative-links.md: 60627b24354610f3e0f819abe7d0603faa9644c27e8cee5163a452804bb94b6c
    scenarios/family-27-done-spec-unchecked-criteria.md: 24fccf828388c32fee261a5e62e6475fc08dae8146ca2943d1602790376d2651
    scenarios/family-28-audit-family-registry-parity.md: 0d7b15a3c103b50b3aeafa3145941910582c16dbda08148ebd804346da4ae15a
    scenarios/family-31-review-block-agreement.md: 4f5f35a88a5ceb3668de37b4cf39d01427e813f170398901089b2a52073351b2
    scenarios/family-34-step-reference-integrity.md: 4d24ed6a08c54ae98f135ad108d3b8a7b85f782f5af7edf26c67d92aa509f067
    scenarios/family-35-manifest-destination-links.md: 8e94dce4172e2b326612da806327af31777d99b8c6ce89642c9d10a98dc148b3
    scenarios/family-36-self-url-resolution.md: f5f69dd3e566a0e07ef2825d5843ec76cf18203355c7c7ddff8737673dfadb09
    scenarios/host-namespace-parity.md: d828792118d7e04ff2312c5097b1f578421e910af0dacd1468d06b16b8777ad6
    scenarios/link-check-consolidation.md: f838133a535aa090e09d7f82903facc4dff8cc822c1dec5c22f9b4d1e17a5049
    scenarios/readme-command-parity.md: 3788aa1103dba1860af8cb9950a6425ed33e4a24498f825fcd980e0c9bb7f8bc
  blocking: false
next-criterion: 26
analyze:
  last-run: 2026-09-07T22:40:08Z
  analyzed-against: 0620b10744997cfdf8cb8a51ecd7e2c7c16104f3
  hard-fail: 0
  blocking-findings: 0
  advisory: 0
  unexamined: 0
  analyzed-digest:
    plan.md: d310b9dd28b461ba2e5494e3d36d666dc3cc4e1f8a3a3e3866037b59d19c05c7
    review.md: 437c8937b687bb6781ae6c76b7b9c2a78cb8f3a23ea4259490e0ce7cda15a036
    scenarios/audit-ci-hard-gate.md: a7bad7a167532019112d79a746696ad32d963171598d2283072af0e9f3234be7
    scenarios/audit-script-refactors.md: f27e039741ab1fce4c868b2af03a84ac3626f780e7af7b092abc8c9a112081a2
    scenarios/family-10-migration-coverage.md: 988d53541ffc5ea7fd752e698de8143c31dcb7b6b0a49f590deb31da777df76f
    scenarios/family-17-contract-binding.md: 1ab3dfbc5c53ca5ab082b587a299b1951c8dd3c3e29148db5ed3a5df50903faf
    scenarios/family-18-marker-list-parity.md: e2f5e57dc51efec37f63fdc419d1c78ae4afb8319bcd41b604a2c22507af6296
    scenarios/family-19-mechanical-sweep-exemption.md: 1aed9678cb90da55fb314f9f8bd26addb3baebdbad04bda48fb11deeb95c4089
    scenarios/family-19-review-freshness.md: 15df43ccf633bb8813ecb0ab8555be0a46ab9aca2b66419f75836262a88950e5
    scenarios/family-19-says-what-it-examined.md: cbdb9363ab1e3b38938864a44fe4a5ae90ca36294631b900abdab43789331ee7
    scenarios/family-22-adopter-shell-behavior.md: 2eac3b7db9587dd354168941be8a1816d11a9e6f7fb27ecd8b8f9c9f8f719754
    scenarios/family-23-sweep-target-manifest-parity.md: 9dfa1299cdeb27ad691c66d6e9adeb68187c7c60039576cafd6ba55f29274b38
    scenarios/family-24-rename-sweep-residue.md: 7853d334d6836dfb565348172ef47377f73aa1687ab0e22f04d1e3cd18f63634
    scenarios/family-25-unbalanced-inline-markup.md: bddc8f9f37f21e3404f24bf77f732d2adc591faa76038983b3b44a41e640e1c6
    scenarios/family-26-broken-relative-links.md: 60627b24354610f3e0f819abe7d0603faa9644c27e8cee5163a452804bb94b6c
    scenarios/family-27-done-spec-unchecked-criteria.md: 24fccf828388c32fee261a5e62e6475fc08dae8146ca2943d1602790376d2651
    scenarios/family-28-audit-family-registry-parity.md: 0d7b15a3c103b50b3aeafa3145941910582c16dbda08148ebd804346da4ae15a
    scenarios/family-31-review-block-agreement.md: 4f5f35a88a5ceb3668de37b4cf39d01427e813f170398901089b2a52073351b2
    scenarios/family-34-step-reference-integrity.md: 4d24ed6a08c54ae98f135ad108d3b8a7b85f782f5af7edf26c67d92aa509f067
    scenarios/family-35-manifest-destination-links.md: 8e94dce4172e2b326612da806327af31777d99b8c6ce89642c9d10a98dc148b3
    scenarios/family-36-self-url-resolution.md: f5f69dd3e566a0e07ef2825d5843ec76cf18203355c7c7ddff8737673dfadb09
    scenarios/host-namespace-parity.md: d828792118d7e04ff2312c5097b1f578421e910af0dacd1468d06b16b8777ad6
    scenarios/link-check-consolidation.md: f838133a535aa090e09d7f82903facc4dff8cc822c1dec5c22f9b4d1e17a5049
    scenarios/readme-command-parity.md: 3788aa1103dba1860af8cb9950a6425ed33e4a24498f825fcd980e0c9bb7f8bc
    spec.md: a1add3c57db6d9efff4ce3aeac2c2c769d71a51c3f4d083d6230a0c6590c9518
    tasks.md: 7414cfe3b1d8400cbffd6fa7fd4c8295fd81d66bc325414d72c6574eca990c36
  blocking: false
---

# 026 — Framework self-audit (`/audit`)

A maintainer-grade slash command that audits `ductus`'s own framework artifacts for the kinds of drift [`/ductus:analyze`](../../framework/commands/analyze.md) is not scoped to catch. `/ductus:analyze` audits a single feature spec's artifacts against each other; `/audit` audits the framework's cross-doc, cross-manifest, cross-registry consistency. Run by maintainers before tagging a release; not invoked by adopters.

## Motivation

The pre-v0.1.0 review passes surfaced ~30 of 45 findings as framework-level drift that `/ductus:analyze` was not scoped to catch — `/ductus:analyze`'s contract is bounded to a single feature directory and its dependencies, so it cannot see drift between, e.g., `README.md`'s spec-status table and the per-spec frontmatter, or between `configure/claude.md` and `configure/auggie.md`'s canonical permission set. The drift-prevention principles are codified in [`framework/constitution.md`](../../framework/constitution.md) §drift-prevention; `/audit` automates the checks AGENTS.md §Design Principles already forbids designing around author diligence.

A separate problem surfaced 2026-05-17 when [024](../024-rule-loader/spec.md) and [025](../025-rule-opt-out/spec.md) shipped as two specs that should have been one — both touched `framework/commands/review.md` §Behavior step 5, both went through separate clarify walks, and the boundary decision was never made explicit. The fix can't be "Claude remembers the 020 precedent" because that depends on author discipline. It has to be a derived signal the framework surfaces from artifacts alone: inline links + frontmatter `dependencies` + Affected Files tables. `/audit` is the natural home for that check.

## Behavior

`/audit` runs across the entire framework (not a single feature). It loads no rule files — its checks are about *framework consistency*, not spec quality. Each check family produces structured findings (severity, location, suggested fix). The maintainer reviews findings; `/audit` does not auto-fix.

### Check families

Nine families were the v1 scope, described below by what each checks, where it reads, and what a finding looks like. **Family 3 has since been retired** — spec `043-workflows-sunset` sunset the workflows feature and deleted the artifacts it compared — so eight of the nine ship today. Family numbers are permanent identifiers: 3 is not reused, and the families added by later scenarios continue from 10.

#### 1. Cross-doc claim consistency

Every claim a doc makes about another part of the framework must agree with the source of truth.

- **README's spec-status table vs. per-spec frontmatter.** The README table is generated by `scripts/gen-readme-table.sh` (per [017](../017-derive-dont-ask/spec.md) AC10); the audit runs the generator with `--check` and reports a finding when the output differs.
- **Pipeline diagrams across docs.** The pipeline shape (`draft → clarified → planned → in-progress → done` plus back-edges) appears textually in `framework/constitution.md` §spec-lifecycle, `docs/introduction.md`, and `framework/templates/project/project-readme.md`. The audit extracts each diagram and compares them; a textual divergence is a finding.
- **Back-edge wording.** The three back-edges defined in §spec-lifecycle (`/amend` for questions, `/amend` for scenarios, meaningful body edit) are referenced in `framework/commands/amend.md` and `framework/commands/target.md`'s Status→next-action table. The audit checks each reference matches the constitution's canonical wording.

#### 2. Manifest parity

Two installer paths (`/ductus` for greenfield bootstrap; `/ductus:init` for adopter-side initialization) must scaffold the same files. Two agent permission files (`framework/bootstrap/configure/claude.md`, `framework/bootstrap/configure/auggie.md`) must define the same canonical permission set in their respective formats.

- **Installer manifest parity.** Compare the file list scaffolded by `/ductus` against `/ductus:init`. Any file in one but not the other is a finding.
- **Permission set parity.** Extract the canonical permission set from each configure source. Differences are findings — same semantic permission must appear in both, in each agent's native format.

#### 3. Registry equivalence — retired

Implemented in v1 as `scripts/audit/registry-equivalence.sh`, which checked that `framework/workflows/registry.json` and `framework/workflows/*.md` agreed: every registry entry referenced a real workflow file, every workflow file appeared in the registry, and registry descriptions matched the workflow file's frontmatter `description:`.

Retired by spec `043-workflows-sunset`, which sunset the workflows feature and deleted `framework/workflows/` along with the check script (commit `531e3ea`). It is referenced by name rather than inline link because 026 does not depend on 043 — the reverse-chronological relationship would induce a `026 → 043 → 027 → 026` cycle in the derived dependency graph. Nothing remains for the family to compare, so it is no longer registered in `scripts/audit/run-all.sh`. The number stays reserved rather than reused.

#### 4. Placeholder roundtrip

Command sources under `framework/commands/` use placeholders (`{project}`, `{cli-config-dir}`) substituted at scaffold time. The audit checks no command source contains a hardcoded `.claude/`, `gov:`, etc. that should be a placeholder — those break the multi-agent contract spec 012 established.

#### 5. Template-validate alignment

Every blocking check in `framework/commands/analyze.md` must have a corresponding scaffolding element in the spec/plan/tasks templates. If analyze blocks on a missing field, the template must scaffold that field. Surface mismatches.

#### 6. Single-source-of-truth invariants

A rule that appears textually in only one location should be referenced from other locations rather than restated. The audit identifies high-value SSOT candidates (open-question counting rule, status state machine, back-edge ownership) and reports cases where the rule's text appears redundantly across multiple artifacts.

#### 7. Sibling-spec coupling check

When two specs whose `status` is not yet `done` reference each other via inline markdown link AND list overlapping rows in their `## Affected Files` tables, surface the pair as a bundling candidate. This is purely derived (inline links, frontmatter `dependencies`, Affected Files row equality); no author discipline required.

Background: [024](../024-rule-loader/spec.md) (rule-file loader) and [025](../025-rule-opt-out/spec.md) (rule-file opt-out) both touched `framework/commands/review.md` §Behavior step 5 and went through separate clarify walks — the boundary decision was never made explicit. `/audit`'s coupling check would have surfaced the pair at the second spec's `/ductus:specify` (or at the first run of `/audit` after both were drafted). The maintainer makes the call (split with rationale recorded, or fold one into the other); `/audit` only surfaces.

#### 8. Introducing-spec body drift

After a command, file, or identifier is renamed via the mechanical sweep ([023's living-specs scenario](../023-govern-refinement/scenarios/living-specs.md) established the rule), introducing-spec bodies retain references to the old name as accurate historical-action prose written before the consolidation landed. These references sit at a tension: they were correct when written, but a reader treating the spec body as current-state truth will be misled because the verb tense and surrounding language still implies the old name is in active use.

The audit flags each introducing-spec body that retains references to old names in current-tense or imperative prose (sentences using `is`/`provides`/`exposes` rather than `was`/`provided`/`introduced`) and lists the affected sentences. The maintainer chooses per-spec whether to past-tense-rewrite (small `/ductus:amend` cycle on that spec) or accept the prose as-is.

Background: 2026-05-17 the living-specs sweep left bare-backticked old names in ~9 specs (011, 014, 017, 020, 021, 022, 023, 024, 000) because mechanical substitution would break sentences. For example, in 011:

> "A new `/capture` command provides... separate from `/specify`"

Each spec's cleanup is small but procedurally heavy if done as 9 separate `/ductus:amend` cycles. The audit surfaces the remaining drift so the cleanup happens organically when authors touch the affected specs for other reasons; the back-edge fires anyway during those edits, and the past-tense rewrite rides along.

#### 9. Primitive-promotion candidates

Scan `framework/commands/*.md` Instructions sections for numbered steps that have neither a backticked runtime-primitive name nor an `<!-- llm:* -->` extension-point marker. Each such "prose-only" step is a candidate for primitive promotion (deterministic logic that could become a `ductus` primitive) or for an explicit LLM-marker annotation (when the step actually requires semantic judgment).

The check is deterministic — it doesn't try to decide whether a given step *should* be a primitive (that's an LLM judgment captured per-spec). It surfaces every prose-only step so the maintainer can either annotate it (`<!-- audit:ignore-promotion -->` for genuine host-responsibility prose like "render the report") or schedule a primitive design.

Background: closing the loop on the runtime expansion path. Spec 022 designed the runtime primitives against existing slash-command prose; 026's v1 audit family closes the inverse direction — surfacing remaining prose that has accumulated since 022 (or that 022's primitive set didn't cover). Together, the two directions make primitive expansion data-driven rather than ad-hoc.

### Boundary with `/ductus:analyze`

| Concern | Owner |
| --- | --- |
| Spec's frontmatter parses; required fields present | `/ductus:analyze` |
| Dependency graph well-formed for one feature | `/ductus:analyze` |
| Rule IDs cited in spec exist in loaded rule files | `/ductus:analyze` |
| Plan / tasks / data-model present per status tier | `/ductus:analyze` |
| Cross-doc claim consistency (README vs frontmatter, etc.) | `/audit` |
| Manifest / permission / registry parity | `/audit` |
| Sibling-spec coupling (bundling candidates) | `/audit` |
| Introducing-spec body drift (current-tense prose around renamed names) | `/audit` |
| Primitive-promotion candidates (prose-only steps in command sources) | `/audit` |

Rule of thumb: `/ductus:analyze` reads within one spec's directory plus its declared dependencies; `/audit` reads across the framework's cross-cutting artifacts. The two never duplicate a check.

### Output

`/audit` writes findings to stdout in a maintainer-friendly format: family, severity, location, what failed, suggested fix. Exit code `0` when no findings; `1` when any finding is present. CI may invoke `/audit` as a release gate.

The maintainer reviews findings and routes each to a fix path:

- Drift between artifacts → edit the artifact whose content is wrong, or update the source of truth and re-run the generator.
- Sibling-spec coupling → record the bundling decision in the second spec's clarify resolution (split with rationale, or fold).
- Missing parity → add the missing file/permission/registry entry to the lagging side.

## Acceptance Criteria

- [x] AC1: A new slash command `/audit` exists at `framework/commands/audit.md` (and its generated `.claude/commands/ductus/audit.md` mirror). The command runs without a session target and audits the framework's own cross-cutting artifacts.
- [x] AC2: The check families listed in Behavior are implemented — all nine at v1, eight today after Family 3's retirement (see Behavior §3). Each produces at least one finding shape with severity, location, what failed, and a suggested fix.
- [x] AC3: **Cross-doc claim consistency** check #1 (README spec-status table) is implemented by invoking `scripts/gen-readme-table.sh --check` and reporting non-zero exit as a finding. Check #2 (pipeline diagrams) and check #3 (back-edge wording) extract the relevant textual blocks from each doc and report differences.
- [x] AC4: **Manifest parity** compares the file list scaffolded by `/ductus` against `/ductus:init` (extracted from each command's source body, not from runtime execution); compares the canonical permission set in `framework/bootstrap/configure/claude.md` against `framework/bootstrap/configure/auggie.md` (extracted in each agent's native format, normalized to a comparable shape).
- [x] AC5: **Registry equivalence** verified every entry in `framework/workflows/registry.json` referenced a real workflow file in `framework/workflows/*.md`, every workflow file was registered, and registry descriptions matched workflow file frontmatter `description:`. Met at v1 and **since retired** — spec `043-workflows-sunset` deleted the workflows feature and this check with it (see Behavior §3). Retained as a record of what shipped, not as a live requirement.
- [x] AC6: **Placeholder roundtrip** scans `framework/commands/*.md` for hardcoded `.claude/`, `gov:`, or other tokens that should be placeholders per spec 012's multi-agent contract.
- [x] AC7: **Template-validate alignment** enumerates blocking checks in `framework/commands/analyze.md` and confirms each has a corresponding scaffolding element in the spec/plan/tasks/scenario templates.
- [x] AC8: **Single-source-of-truth invariants** detects when a normative rule (e.g., the open-question counting rule) appears textually in multiple locations and reports cases where the rule should be referenced rather than restated.
- [x] AC9: **Sibling-spec coupling check** scans `specs/` for pairs of specs at `status ∈ {draft, clarified, planned, in-progress}` that (a) inline-link each other AND (b) share at least one row in their `## Affected Files` tables; surfaces each pair as a bundling candidate. Findings name both specs and the overlapping rows.
- [x] AC10: **Introducing-spec body drift** scans every done spec's body for current-tense or imperative prose containing references to renamed commands (any old name listed in the framework's rename history per git log). Findings name the spec, the affected sentences, and propose past-tense rewrites. The maintainer accepts the rewrite (via a small `/ductus:amend` cycle on the spec) or leaves the prose as-is.
- [x] AC11: **Primitive-promotion candidates** scans `framework/commands/*.md` Instructions sections for numbered steps that have neither a backtick-quoted runtime-primitive name nor an `<!-- llm:* -->` extension-point marker. Each such prose-only step is emitted as a finding. Genuine host-responsibility prose (e.g., "render the report") is annotated with `<!-- audit:ignore-promotion -->` to silence the check on that step.
- [x] AC12: `/audit`'s exit code is `0` when no findings are present and `1` when any finding is present. CI can gate releases on the exit code without parsing the report.
- [x] AC13: `/audit` writes its output to stdout in a maintainer-friendly format: one section per check family, one finding per row within a family, with severity / location / message / suggested-fix columns. No `audit.md` report file is produced (unlike `/ductus:review`'s `review.md`) — the framework-level audit is run interactively, not stored as an artifact.
- [x] AC14: The boundary with `/ductus:analyze` is documented in `framework/commands/audit.md` §Notes (or equivalent): `/ductus:analyze` is feature-spec-scoped; `/audit` is framework-scoped; the two never duplicate a check. Adopters never invoke `/audit` — it is a ductus-maintainer tool.
- [x] AC15: **Adopter shell behavior** runs the shipped `framework/bootstrap/hooks/ductus-pre-commit` and `.ductus/scripts/**` against a generated fixture shaped like an adopter's tree — a non-default `[paths] specs-root`, config only at `.ductus/config.toml`, and the runtime reachable only through the `.ductus/bin/ductus` pointer with nothing named `ductus` on `PATH` — and asserts that the spec root resolves to the configured value, that the hook reaches the runtime, and that no generator rewrite is left unstaged. The fixture is built at both the default and a configured root so neither failure masks the other, the runtime is stubbed so the family stays hermetic, and any precondition that prevents the fixture from being built is emitted as a finding rather than skipped.
- [x] AC16: **Sweep-target manifest parity** asserts that the live-artifact enumeration a rename sweep greps — delimited in `AGENTS.md` by `<!-- audit:sweep-targets:begin -->` / `<!-- audit:sweep-targets:end -->` — covers every source path the **Shared Files** manifest ships, so a relocated directory cannot leave the list naming somewhere clean while the sweep misses the files that moved. The check runs manifest → list only and reports that direction along with the entry and path counts, since it proves no shipped file goes unswept but not that the list is complete; an empty extraction on either side is a finding rather than a pass.
- [x] AC17: **Rename-sweep residue** asserts that the project name never appears where English grammar requires a verb, catching the residue a word-boundary rename sweep leaves when the retired name was also an ordinary verb. Detection is two closed word classes — a modal followed by the project name, and the project name followed by a demonstrative or wh-word — so it is exact rather than heuristic; `the`, `to`, and `that` are excluded because each is ordinary before the name. A scan that examines no files is a finding rather than a pass, and the examined-file count is reported.
- [x] AC18: **Unbalanced inline markup** asserts that no line in `AGENTS.md` or the `AGENTS.md` template carries an odd number of backticks or `**` markers outside fenced code blocks — the malformed-entry class markdownlint cannot see, since an unclosed backtick never becomes a code span. Scoped to those two files because their bullets are single-line, which is what makes a per-line check exact; the family reports that scope on stderr so a clean exit is never read as a corpus-wide guarantee, and reports a wrapped bullet rather than narrowing silently once the convention lapses.
- [x] AC19: **Broken relative links** asserts that every relative markdown link resolves, closing the gap between `MD051` (which validates heading fragments and never checks that the file exists) and `check-orphaned-references` (which scopes to adopter-owned referrers and ductus-managed prefixes). Findings anchor to `file:line` and distinguish the two repair paths — a depth error, whose corrected path the family states outright, from a target a later spec deleted, which is named in prose rather than linked. Inline code spans are stripped so a document *describing* a link is never reported as making a broken one; fences are skipped without shifting line numbers; generated copies and adopter templates are excluded by construction and counted; a failed file listing is a finding rather than a silent pass.
- [x] AC20: **Done-spec unchecked criteria** asserts that no spec at `status: done` carries an unchecked acceptance criterion, anchoring each finding to `file:line` with the criterion's label and naming both repairs — tick it, or reopen the spec — since the family cannot tell which is right. Only `done` specs are examined, because unchecked criteria are the expected state everywhere else; status is read from the frontmatter block and the checkbox from the Acceptance Criteria section, never by a repo-wide grep for either token; fenced blocks are skipped so a document quoting checkbox syntax is not reported as carrying one; specs are enumerated from git with untracked ones skipped and counted; and the examined `done`-spec count is reported, so a clean exit says what it examined and an empty enumeration is a finding rather than a pass.
- [x] AC21: **Audit family registry parity** asserts that the family set `scripts/audit/run-all.sh` registers, the set `framework/commands/audit.md` enumerates, and the scripts listed in `scripts/audit/README.md` §Scripts all agree, reporting registered-but-undocumented and documented-but-unregistered separately because the repairs differ. Both sets are derived rather than hardcoded — a hardcoded expectation would be a third copy of the fact under test — retired family numbers need no allowance because a spent number appears in neither list, only the enumerated `(Family N — …)` entries count so prose mentions do not, the generated copy of `audit.md` is not a subject, and an empty derivation on either side is a finding rather than the agreement two empty sets would otherwise report.
- [x] AC22: **Review block agreement** asserts that a spec's frontmatter `review:` block agrees with its own `review.md`, comparing the five fields both files record — keyed by meaning, since the timestamp is spelled `last-run` on one side and `reviewed-at` on the other — and naming the spec, the field, and both values, with `review.md` as the source of record in the repair. Two rules internal to the block are asserted alongside them, because the observed failure reached the counts through them: `blocking: false` alongside a non-zero `must-violations`, and a finding waived in the report with no matching `review.waivers` entry, which leaves the waiver with no structural existence so the count it should retire never moves. Both records are derived from frontmatter rather than hardcoded; fields appearing on only one side are not compared, since demanding they match would invent a binding the artifacts never claimed; the subject is the intersection of specs carrying both records, the only ones that can disagree, leaving a block with no report to Family 19 and `check-review-gate`; unparseable frontmatter on a report that exists is a finding rather than a skip; and the examined count is reported, with an empty subject set a finding rather than the agreement two empty sets would otherwise report.
- [x] AC23: **Manifest destination links** asserts that every relative markdown link in a file the installer ships resolves against that file's *destination* path rather than its source path. The subject is derived from the installer's own manifest: the **Shared Files** entries, whose destinations are literal, are copied into a throwaway tree and delegated to `check-corpus-links --scope repository`; the **Slash commands** entries, whose destinations carry `{config_dir}` and `{project}` placeholders and whose installed directory the primitive excludes by construction, are checked lexically against the fact that every ductus-authored file there is a sibling `.md`. Both counts are reported, so a clean exit claims that the shipped set resolves and never that every link an adopter has resolves. An empty extraction from either table, an unreachable runtime, and an unparseable result are each findings.
- [x] AC24: **Self-URL resolution** asserts that every absolute GitHub URL pointing back into this repository names a path that exists. The repository slug is derived from the installer's archive URL rather than hardcoded; `main` URLs are resolved against the working tree while tag- and sha-pinned URLs are counted rather than resolved; and `unresolved`, `blob`-naming-a-directory and `tree`-naming-a-file are reported as distinct findings because the repairs differ. This is the guard on the repair the previous criterion's findings require — an absolute link is invisible to every relative-link check in the suite, so without it a spec rename rots 32 shipped citations silently.
- [x] AC25: **Review freshness** compares content where the record supports it and commits where it does not: `review-freshness.sh` judges a `done` spec carrying `review.reviewed-digest` by that digest — the same field and the same working-tree comparison `check-review-gate` makes, so the release gate and the completion gate cannot disagree about a record that carries one — and falls back to the `reviewed-against`..HEAD diff for a record predating the field, keeping the pre-digest population enforced rather than reporting it undeterminable. The mechanical-sweep exemption applies on both arms when the sha resolves; a digest with an unresolvable `reviewed-against` is examined rather than counted unresolvable, since the digest needs no commit. Every run closes with a coverage line, above any findings, naming the examined count **split into digest and proxy** alongside the grandfathered and unresolvable exclusions — the split because the two arms do not carry the same strength of claim: the proxy arm's known false positive, a contract reviewed before it was committed, is guarded there only by a commit-ordering convention, so the line states how many verdicts rest on a discipline rather than on a record, and reaching zero proxy is the evidence for deleting that arm. The line never affects the exit code, so the aggregator is unchanged; a spec the family cannot compare against at all counts as unresolvable rather than being skipped silently.

## Open Questions

## Resolved Questions

- **Is `/audit` adopter-facing or maintainer-only?** **Maintainer-only in v1.** The command lives at `framework/commands/audit.md` and is invoked from the ductus repo's checkout. `/ductus`'s manifest does not scaffold it into adopter projects — adopters never see `/audit` in their `.claude/commands/{project}/` directory. If real adopter demand surfaces later, a follow-on scenario splits `/audit` into framework-checks (stays maintainer-only) and adopter-checks (gets scaffolded). Rationale: the motivation already pins this answer, expanding scope before a concrete forcing function violates the **Design Principles** rule, and several v1 checks (cross-doc claim consistency, manifest parity, registry equivalence) are inherently framework-scoped — an adopter `/audit` would need a different check list, not a subset.
- **Should `/audit` consume the deterministic runtime primitives ([022](../022-deterministic-runtime/spec.md))?** **Defer implementation per-check to plan; record the rule here.** Prefer runtime primitives where they fit (read-only file comparisons, JSON parsing, directory walks); fall back to bash scripts for genuinely shell-shaped work (running existing generator scripts like `gen-readme-table.sh --check`, walking `git log` for the rename history). Matches §runtime-boundary principle 4 — the runtime is an accelerator, not a replacement. Per-check resolution: registry equivalence / placeholder roundtrip / single-source-of-truth invariants lean primitive; cross-doc claim consistency / introducing-spec body drift lean shell; manifest parity / template-validate alignment / sibling-spec coupling are mixed and decided at plan time. Forcing every check through the runtime would require new primitives invented for `/audit` alone — overengineering ahead of the plan.
- **Severity levels.** **Binary in v1: finding / no finding.** Exit `0` when no findings, `1` when any finding. No MUST/SHOULD tier, no waivers. Each finding carries `family`, `location`, `message`, and `suggested-fix` columns so the maintainer can route quickly, but no severity flag. Implicit severity is "fix before tagging the next release." Rationale: the MUST/SHOULD tier in `/ductus:review` exists because adopter projects have heterogeneous risk profiles; `/audit` runs against ductus's own internal framework where the risk profile is uniform — any drift is something the maintainer should know about. Binary exit codes are CI-trivial. The Q7 generator-failure precondition still exits non-zero on its own (before family checks run); both end up at exit `1` and the maintainer reads stdout to distinguish. A future scenario can introduce severity if real maintainer workflows surface a need.
- **CI integration.** **`/audit` runs from the ductus repo's CI only — never from the adopter GHA template.** Two specific owners: (1) the PR workflow adds a step that runs `/audit` on every PR, composing with the existing markdown-lint / parseability / tool-coverage / frontmatter checks; a finding fails the workflow and blocks merge. That step landed in `.github/workflows/markdown-only-pipeline.yml`; `048-govern-acquired-runtime` removed that workflow and the step now runs as `(h) Framework self-audit` in `.github/workflows/framework-checks.yml`. (2) `.github/workflows/runtime-release.yml` adds a pre-tag gate step that runs `/audit` before the release matrix fires; a finding aborts the release before artifacts are built. The shipped `framework/templates/ci/adopter-generators.yml` is not modified — `/audit` is invisible to adopters per Q1's maintainer-only resolution. Workflow #2 is belt-and-suspenders against tags pushed directly to main without PR.
- **Coupling-check ergonomics.** **Defer implementation mechanics to plan; pin the suggested-fix output shape and the suppression contract here.** Every coupling-check finding's `suggested-fix` column renders both resolution paths: (a) fold one spec into the other (delete the redundant spec's directory, merge ACs and open questions into the kept spec, update inline links in any spec that referenced the deleted one), or (b) record the split rationale in the second-drafted spec's `## Resolved Questions` section using the literal phrase `Why split from {first-spec-slug}: <reason>`. The (b) suppression rule is the only behavior pinned at clarify: the audit greps the second spec's Resolved Questions for that exact pattern and stops surfacing the pair once an entry exists. Without this contract, every audit run after a deliberate split would keep surfacing the same pair, training maintainers to ignore findings. The literal phrase matches the existing Resolved-Questions convention — no new artifact, no new tooling.
- **Manifest parity normalization.** **Inline in the audit command's logic, not a shared script.** Normalization extracts each format's permission set into a canonical comparable shape `(tool-name, command-pattern)`, sorts both, and diffs: Claude `Bash(X *)` allow entries map to `("Bash", "X *")`; Auggie `{ toolName: "launch-process", shellInputRegex: "^X " }` entries strip the `^` anchor and trailing space, normalize case, and map to the same shape. Rationale for inline: `/audit` is the only consumer of this normalization (a shared `scripts/normalize-permissions.sh` would add discovery overhead and a second source of truth); the Claude/Auggie format-specific knowledge already lives adjacent to the configure sources being audited; `scripts/gen-configure-mcp.sh` is the existing format-aware code path, and adding a second normalization script would create a third place to update on format changes. Promotion path stays open: if a second consumer surfaces later, the normalization graduates to a shared script then. Per-check implementation (inline function in `framework/commands/audit.md`, helper script, or runtime primitive) is decided at plan per Q2's "primitives where they fit" rule.
- **Bootstrap order of checks.** **Yes — `/audit` runs a "check-zero" generator/lint precondition pass before any of the eight family checks.** The v1 precondition list, in order: `gen-spec-deps.sh --dry-run`, `gen-readme-table.sh --dry-run`, `gen-help-tables.sh --dry-run`, `gen-configure-mcp.sh --dry-run`, `gen-claude-commands.sh --dry-run`, `lint-rule-filenames.sh`, `lint-frontmatter.sh`, `lint-procedure-parseability.sh`, `lint-tool-coverage.sh`. (Order: `gen-spec-deps.sh` first so the readme-table check sees fresh `dependencies:` fields if deps updated anything.) Any non-zero exit surfaces as a check-zero finding with location pointing at the failing script; the eight family checks are skipped after a check-zero failure to avoid misleading findings against known-stale generator output. Exit code stays binary `1` per Q3. New generators added by future specs land at the end of this list AND in the PR workflow's existing generator-orchestration step — `.github/workflows/framework-checks.yml`, which `048-govern-acquired-runtime` renamed from `markdown-only-pipeline.yml`. A generator that fails for a non-drift reason (script bug, missing dependency) also halts `/audit` — a broken generator is itself a finding the maintainer should address before tagging.
