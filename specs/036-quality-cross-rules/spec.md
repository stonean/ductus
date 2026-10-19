---
status: in-progress
dependencies: [008-security-rules, 016-cross-cutting-rules, 017-derive-dont-ask, 024-rule-loader, 033-rule-surface-setting]
next-criterion: 12
---

# 036 — Cross-cutting code-quality rules

Introduce `framework/rules/quality-cross.md`, a cross-cutting code-quality rule set whose inaugural rule `QUAL-STUB-001` forbids silent stubs: partial or unimplemented code paths that pass through without failing loudly. The file follows the canonical rule schema, uses the `-cross.md` suffix (loaded for every stack), and is wired into the `/ductus` Shared Files manifest. This is a rule-introducing feature spec — the same path [008-security-rules](../008-security-rules/spec.md) used to introduce the first rule files.

## Motivation

Silent stubs ship indistinguishably from working implementations — a no-op rate-limiter, an always-allow permission check, a publisher that drops events on the floor — and the failure mode surfaces only when the missing behavior is needed, which is precisely when the system is under stress. A partial implementation that returns a zero value, a no-op middleware that returns `next` unchanged, a handler that returns early without an error, or a method that returns `nil, nil` is not visibly distinct from a correct one until production exercises the gap.

None of the existing rule files cover this discipline. `api-backend.md` `BE-SCHEMA-002` carries a "fail loudly at build time" rationale, but it is API-schema-specific; the stub-discipline is genuinely cross-cutting — it applies to background workers, domain methods, frontend stores, middleware, anything whose surrounding contract implies the path performs work. The constitution treats code-quality discipline as part of the governance-recognized cross-cutting class (§rules, the "etc." that sits alongside security, performance, concurrency, observability, accessibility, audit/compliance, data handling), so the concern belongs at the rules tier rather than re-litigated per feature.

**Motivating incident.** In an adopter project running a backend service, 008-rate-limiting Task 4 left a passthrough stub in `RateLimit` enabled-mode that would have silently disabled rate limiting in production had Task 5 been skipped. The team manually added a panic with a "not implemented" message until Task 5 landed — a discipline the framework should make automatic rather than depending on a contributor remembering to add the guard.

## Rule set scope

`quality-cross.md` uses the **cross** surface (`-cross.md` suffix → loaded for every stack via the rule-loader's suffix discovery, [024-rule-loader](../024-rule-loader/spec.md)). Its ID grammar is `QUAL-{CATEGORY}-{NNN}`, mirroring the domain-prefixed cross-file convention `configuration-cross.md` established (`CFG-{CONST|ENV}-{NNN}`, [017-derive-dont-ask](../017-derive-dont-ask/spec.md)). The inaugural category is `STUB`; the file header declares the `QUAL` category abbreviation(s) per the per-file category-declaration policy framed in [016-cross-cutting-rules](../016-cross-cutting-rules/spec.md).

Inaugural rule:

- **`QUAL-STUB-001`** — *partial or unimplemented code paths MUST fail loudly (panic / explicit error / failing test fixture) rather than silently pass through; stubs that return zero values, no-op middleware that returns `next` unchanged, handlers that return early without an error, and methods that return `nil, nil` are forbidden when the surrounding contract implies the path performs work.*

### Boundaries (cross-reference, do not duplicate)

- **Build-time schema fail-loud** is already `api-backend.md` `BE-SCHEMA-002` — `quality-cross.md` cites it for the build-time case rather than restating it; `QUAL-STUB-001` governs the broader runtime/contract case across all surfaces.

### Added categories

- **`GROUND`** (`QUAL-GROUND-001`, SHOULD) — code whose correctness depends on an external contract it does not own (database schema, external API shape, config key, file/wire format) should bind to it so a wrong assumption fails loudly, rather than silently encoding a guess. The code-side counterpart to `/ductus:analyze`'s artifact-grounding check; both enforce constitution §grounding. Added after the inaugural delivery per the category-growth policy (`data-model.md` §Category abbreviations), consolidating the code-side grounding enforcement into this existing `QUAL`-surface home rather than a new spec.
- **`CLAIM`** (`QUAL-CLAIM-001`, SHOULD) — a clean, empty, or in-sync result should distinguish *"examined the subject and found nothing"* from *"could not examine the subject"*, rather than emitting the same value for both. Where `STUB` governs unimplemented paths returning success and `GROUND` governs unverified assumptions inside logic, `CLAIM` governs a fully-implemented path whose **output** overstates what it verified — absence of evidence rendered as evidence of absence. Added per the same category-growth policy, derived from four instances observed in `ductus`'s own tooling rather than proposed speculatively. The rule's Source paragraph is the record of which they were, and of the later fifth instance an adopter surfaced.
- **`DELEG`** (`QUAL-DELEG-001`, SHOULD) — replacing a hand-rolled predicate, parser, validator, or comparator with a shared one should be preceded by enumerating what the local version enforced *by construction* — character set, length or form, where the candidate ended, ordering, input trust level — rather than only what its name declares, with any property the shared implementation does not provide kept local and the reason recorded on both sides. Where `CLAIM` governs a result that overstates what it verified and `GROUND` an unverified assumption about an unowned contract, `DELEG` governs a correct-looking substitution of one implementation for another, whose two failure directions are opposite: a widened predicate accepting input written verbatim into a structured file, and a narrowed one silently dropping input it used to accept. Added per the same category-growth policy, routed here by `050-constitution`'s second promotion round — see the signpost below. Cited as a slug rather than linked: this is provenance, and 036 does not depend on 050.
- **`TEST`** (`QUAL-TEST-001`, SHOULD) — a test should fail when the behavior it names is removed or broken; one that exercises a path without asserting its result, pins only an incidental value, or passes because a fallback or default path satisfies its assertion as well as the primary path does is not coverage. Where `CLAIM` governs a code path whose output overstates what it examined and `STUB` an unimplemented path returning success, `TEST` governs the test itself: a check that reports a pass it could never withhold. The fallback case is the one that survives review, because only breaking the code shows it. Routed here by `050-constitution`'s third promotion round, as its second routed `DELEG` — see the signpost below.

> **Signpost ([050-constitution](../050-constitution/spec.md)):** `QUAL-DELEG-001` was added here rather than to the
> constitution. 050's second promotion round classified the `AGENTS.md` entry
> *replacing a hand-rolled predicate with a shared one widens everything it was quietly doing*
> as universal by its reword test, but the rule states a requirement about **code** rather than
> about the pipeline — a fourth destination the three-tier classification cannot express — so
> [§rules](../../framework/constitution.md#rules) routes it to a rule file with a permanent ID
> and a Verification clause, through this spec's back-edge because this spec owns the `QUAL`
> surface. 050 records the obligation in its `cross-spec-impact:` frontmatter; this signpost is
> its discharge. 050's third round routed a second entry the same way on 2026-09-28:
> *a test must fail when the behavior it names is removed* is `QUAL-TEST-001`, universal by
> the reword test and code-governing, so it lands here through the same back-edge.
>
> **Signpost ([008-security-rules](../008-security-rules/scenarios/a-statement-carries-one-obligation-keyword.md)):**
> the severity invariant this spec's `data-model.md` inherits is stated in tier terms: a
> Statement's obligation keywords come from one tier, and MAY may appear beside either as a
> permission that carries no tier. It previously read "exactly one RFC 2119 keyword", which
> 66 single-tier shipped Statements pairing MUST with MUST NOT, or SHOULD with SHOULD NOT,
> contradicted — counted over the 192 rules before 008 split its 16 mixed-tier ones. 008 records the obligation in its `cross-spec-impact:` frontmatter; this
> signpost is its discharge.

## Acceptance Criteria

- [x] AC1: `framework/rules/quality-cross.md` exists, ends in the `-cross.md` suffix, and follows the canonical rule schema (`### {ID}` headings; Statement / Rationale / Verification; RFC 2119 language) per [008-security-rules](../008-security-rules/spec.md)'s data-model.
- [x] AC2: `QUAL-STUB-001` is present with a Statement (RFC 2119), a Rationale, and a Verification clause.
- [x] AC3: Every rule ID uses the `QUAL-{CATEGORY}-{NNN}` format with the `QUAL` prefix and category abbreviation disjoint from the existing `BE-`/`FE-`/`CFG-` namespaces; `scripts/lint-rule-ids.sh` passes.
- [x] AC4: The file header declares the `QUAL` category abbreviation(s) per the per-file category-declaration policy ([016-cross-cutting-rules](../016-cross-cutting-rules/spec.md)).
- [x] AC5: The Verification clause is expressed as a check `/ductus:review` can apply to code (silent passthrough vs. loud failure), and is scoped so legitimately-empty implementations are not flagged (the three-part discriminator and exemption list in Resolved Questions — Verification mechanism).
- [x] AC6: Rules whose surface overlaps an existing rule cite it rather than restating it (`BE-SCHEMA-002` for the build-time fail-loud case).
- [x] AC7: The file is added to the `/ductus` **Shared Files** manifest in `framework/bootstrap/ductus.md` (slotted between `performance-frontend.md` and `security-backend.md`) and is auto-selected for every stack via the `-cross.md` suffix ([024-rule-loader](../024-rule-loader/spec.md)), composing with [033-rule-surface-setting](../033-rule-surface-setting/spec.md).
- [x] AC8: `QUAL-GROUND-001` (SHOULD) is present with Statement / Rationale / Verification, the `GROUND` category is declared in the file header and registered in the data-model, and the rule is enforced by `/ductus:review`'s quality pass as the code-side counterpart to `/ductus:analyze`'s grounding check (constitution §grounding).
- [x] AC9: `QUAL-CLAIM-001` (SHOULD) is present with Statement / Rationale / Verification, the `CLAIM` category is declared in the file header and registered in the data-model, and its Rationale states the discriminator against `QUAL-STUB-001` and `QUAL-GROUND-001` so a reviewer choosing between the three has the distinction in hand. Its Verification carries a promotion criterion to MUST, and its Source cites the observed instances it was derived from — naming which are confirmed and which are unassessed, so the rule does not itself assert more than was verified.
- [x] AC10: `QUAL-DELEG-001` (SHOULD) is present with Statement / Rationale / Verification, the `DELEG` category is declared in the file header and registered in the data-model, and its Rationale states the discriminator against `QUAL-CLAIM-001` and `QUAL-GROUND-001` — this rule governs a correct-looking substitution of one implementation for another, whose two failure directions are opposite. The Verification clause names the enumeration a delegation owes (character set, length or form, termination, ordering, input trust level) and treats a test exercising **both** the widened and the narrowed direction as compliant, since one direction proves half.
- [x] AC11: `QUAL-TEST-001` (SHOULD) is present with Statement / Rationale / Verification, the `TEST` category is declared in the file header and registered in the data-model, and its Rationale states the discriminator against `QUAL-CLAIM-001` and `QUAL-STUB-001`. The Verification clause names the fallback case — an input on which the primary path and a fallback agree — and treats a test whose report, commit or comment records how it was shown to fail as compliant.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Single category at launch, or seed siblings now?** Resolved: **ship `STUB` alone.** Mirrors how every prior rule set was introduced (008-security, 034-performance each started from a motivated nucleus and grew as concerns promoted) — a single-rule file is valid. `QUAL-STUB-001` has a concrete motivating incident (the adopter rate-limiter passthrough); "swallowed errors" and "dead code" have no grounding incident yet, so seeding them now would be speculative governance with no verifiable Statement/Rationale. The `QUAL-{CATEGORY}-{NNN}` grammar leaves room to promote adjacent categories later via `/ductus:amend` or a follow-on spec when a concrete need appears.
- **Verification mechanism.** Resolved: **review-time framing confirmed, with a three-part discriminator and an explicit exemption list.** `QUAL-STUB-001` is a source-code pattern, so its checker is `/ductus:review`'s quality pass against code in scope — not `/ductus:analyze` (which audits artifacts). The Verification clause flags a path only when **all three** hold: (1) it is **reachable** under the current spec, (2) its **surrounding contract implies work** (named for a behavior, documented to do something, or called by code that depends on its effect), and (3) it returns a success/zero/pass-through value with **no loud signal**. Exemptions named in the clause so genuine no-ops are not flagged: an explicit incompleteness marker *is* compliance (`panic`/`todo!`/`unimplemented!`, a raised `NotImplementedError`, or a failing/skipped test fixture — that *is* failing loudly); intentional pass-through middleware documented as deliberate; default/interface implementations meant to be empty; and not-yet-reachable branches behind a feature flag or guard. One-line discriminator: a stub is forbidden only when the contract implies work **and** there is no loud signal marking it incomplete.
- **MUST vs. SHOULD.** Resolved: **MUST.** A silent stub is a correctness/availability hazard, not a tunable trade-off — the motivating incident (a rate-limiter that would have silently disabled itself in production) is exactly what RFC 2119 MUST exists for; SHOULD would let it pass `/ductus:review` without blocking. The compliance cost is trivial and always available (one `panic`/`todo!`/error line to fail loudly), while the violation cost is a production outage, so it belongs at MUST — making it a blocking violation that gates `done`. MUST does not trap legitimate cases: the Verification exemptions exclude genuine no-ops, and per-finding waivers (`--waive` with a recorded reason) plus whole-file `[[review.disabled-rule-files]]` remain available for any case the rule should not apply to.
- **`-cross` vs. surface split.** Resolved: **single `-cross.md` file.** The discipline is surface-independent — the failure mode (a contract-implying path that silently passes through) and the remedy (fail loudly) are identical for backend workers/domain methods/middleware and frontend stores/handlers/reducers. Splitting would duplicate one rule across two files (a `QUAL-STUB-001` wearing two IDs), which the framework avoids — overlapping rules cite, not restate. `-cross.md` is exactly the "applies to every stack" suffix: the rule-loader (024) auto-selects it regardless of detected stack and it composes with 033 (cross files are unconditional, never surface-filtered), so backend-only and frontend-only projects both get stub discipline. A genuinely surface-specific stub concern, if one ever emerges, can live in a `-backend.md`/`-frontend.md` file later; the general discipline stays cross.
