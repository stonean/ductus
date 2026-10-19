# 036 — Cross-cutting code-quality rules Data Model

Registers the `QUAL` rule-ID surface and its inaugural `STUB` category. It does not restate the rule-file schema — that is defined once in `specs/008-security-rules/data-model.md` (Statement / Rationale / Verification; `### {ID}` headings; permanent IDs; MUST/SHOULD severity) and `quality-cross.md` follows it, mirroring how `configuration-cross.md` (017) follows the same schema.

## `QUAL` surface

`QUAL` is a new rule-ID surface — the third segment of the registered-surface set alongside `BE`/`FE` (`008-security-rules/data-model.md`) and `CFG` (`017-derive-dont-ask/data-model.md`).

| Property | Value |
| --- | --- |
| Surface prefix | `QUAL` (always uppercase) |
| Scope | Cross-cutting code-quality discipline — applies to every stack |
| File | `framework/rules/quality-cross.md` (the `-cross.md` suffix loads it for every stack via 024's rule-loader; 033's surface filter keeps cross files unconditionally) |
| ID grammar | `QUAL-{CATEGORY}-{NNN}` — accepted by both the `scripts/lint-rule-ids.sh` surface allowlist (extended to include `QUAL` by this spec) and the runtime `check-rule-ids` harvester. The two are close but not identical: the lint allows a one-character category (`[A-Z][A-Z0-9]*`) while the harvester requires at least two (`[A-Z]{2,5}-[A-Z][A-Z0-9]+-\d{3,4}`). Every shipped category (including `STUB`) is ≥2 characters, so the difference is latent. |

The surface prefix disambiguates `QUAL-` IDs from `BE-`/`FE-`/`CFG-` IDs (AC #3). `scripts/lint-rule-ids.sh` is the machine-checked registry of accepted surfaces; this data-model is the documented source of truth its comment block cites for `QUAL`.

## `QUAL` rule ID format

```text
QUAL-{category}-{NNN}
```

| Element | Type | Constraints |
| --- | --- | --- |
| `QUAL` | literal | Always uppercase. The fixed prefix for code-quality rules. |
| `category` | string | Short uppercase abbreviation drawn from the table below (`[A-Z][A-Z0-9]*`, first char a letter). |
| `NNN` | integer | Zero-padded sequence number, starting at `001`. Never renumbered. Never reused after removal. |

### Category abbreviations

| Category | Abbreviation |
| --- | --- |
| Silent stubs | `STUB` |
| Unverified contracts | `GROUND` |
| Unsubstantiated clean results | `CLAIM` |
| Unenumerated delegation to shared code | `DELEG` |
| Tests that cannot fail | `TEST` |

The category set is declared in the `quality-cross.md` file header per the per-file category-declaration policy (`016-cross-cutting-rules`). It grows as concerns promote — adjacent categories (e.g. swallowed errors, dead code) are added when a concrete need appears, via `/ductus:amend` or a follow-on spec.

## Rule set

`framework/rules/quality-cross.md` shipped its first version with one rule (`QUAL-STUB-001`); `QUAL-GROUND-001`, `QUAL-CLAIM-001`, `QUAL-DELEG-001` and `QUAL-TEST-001` were added later per the category-growth policy above, each through this spec's back-edge rather than a spec of its own, per [§rules](../../framework/constitution.md#rules). Numbering is permanent. This sentence enumerates the shipped set and is therefore a claim of its own — extend it whenever a rule is added, per [§grounding](../../framework/constitution.md#grounding).

### QUAL-STUB namespace (Silent stubs category)

- `QUAL-STUB-001` (MUST) — partial or unimplemented code paths fail loudly (panic / explicit error / failing test fixture) rather than silently passing through; stubs that return zero values, no-op middleware that returns `next` unchanged, handlers that return early without an error, and methods that return `nil, nil` are forbidden when the surrounding contract implies the path performs work. Verified at review time by `/ductus:review`'s quality pass; cites `api-backend.md` `BE-SCHEMA-002` for the API-schema case (a published schema bound to its implementation).

### QUAL-GROUND namespace (Verify contracts category)

- `QUAL-GROUND-001` (SHOULD) — code whose correctness depends on a contract defined outside that code (a database schema, another service's API shape, a file or wire format, or a list, schema, or format defined elsewhere in the repository) binds to that contract so a wrong assumption fails loudly (a typed or generated binding, a schema or migration reference that the build or a test checks, a startup or first-use validation, or a test that exercises the real shape) rather than silently encoding an unverified assumption. A contract used through its own definition, so the compiler or type checker enforces it, is compliant. Configuration keys and values are not its subject: `configuration-cross.md` `CFG-ENV-003` and `CFG-ENV-008` govern them at MUST, so a configuration defect is reported once, under the `CFG` ID. Verified at review time by `/ductus:review`'s quality pass; the code-side counterpart to `/ductus:analyze`'s artifact-grounding check, both enforcing constitution §grounding.

### QUAL-CLAIM namespace (Unsubstantiated clean results category)

- `QUAL-CLAIM-001` (SHOULD) — a clean / empty / in-sync result distinguishes "examined the subject and found nothing" from "could not examine the subject", through a distinct return variant, a status or guidance field, or a message naming what was skipped — rather than emitting the same bare zero or empty collection for both. Distinct from `QUAL-STUB-001` (unimplemented paths returning success) and `QUAL-GROUND-001` (unverified assumptions inside logic): this rule governs a fully-implemented path whose *output* overstates what it verified. Advisory, with a documented promotion criterion to MUST. Verified at review time by `/ductus:review`'s quality pass; enforces constitution §grounding on the reporting surface.

### QUAL-DELEG namespace (Delegation to shared code category)

- `QUAL-DELEG-001` (SHOULD) — replacing a hand-rolled predicate, parser, validator, or comparator with a shared one is preceded by enumerating what the local version enforced *by construction* (character set, length or form, where the candidate ended, ordering, input trust level) rather than only what its name declares, and any property the shared implementation does not provide is kept local with the reason recorded on both sides. Distinct from `QUAL-CLAIM-001` (a result overstating what it verified) and `QUAL-GROUND-001` (an unverified assumption about an unowned contract): this rule governs a correct-looking substitution of one implementation for another, whose two failure directions are opposite — a widened predicate accepting input written verbatim into a structured file, and a narrowed one silently dropping input it used to accept. Verified at review time by `/ductus:review`'s quality pass. Added through this spec's back-edge by `050-constitution`'s second promotion round, which classified the rule as universal but code-governing, so [§rules](../../framework/constitution.md#rules) routes it to a rule file with a permanent ID rather than into the constitution.

### QUAL-TEST namespace (Tests that cannot fail category)

- `QUAL-TEST-001` (SHOULD) — a test fails when the behavior it names is removed or broken; one that exercises a path without asserting its result, pins only an incidental value, or passes because a fallback or default path satisfies its assertion as well as the primary path does is not coverage and is not added or counted as such. Distinct from `QUAL-CLAIM-001` (a code path's output overstating what it examined) and `QUAL-STUB-001` (an unimplemented path returning success): this rule governs the test itself, a check that reports a pass it could never withhold. Verified at review time by `/ductus:review`'s quality pass, which also flags a test whose ability to fail depends on a condition it does not establish (assertions compiled or branched away inside its body on some platform or feature set, or an outcome that differs only when an environment variable it never sets is present) and a test that never reaches the unit it names (mocked, stubbed, re-implemented inside the test, or not called). Compliant: a test whose report, commit or comment records how it was shown to fail; one that establishes the condition it depends on itself; one gated as a whole, by a test-level attribute or skip marker, to the configurations where the named behavior applies; and one that replaces the unit's collaborators with doubles but calls the unit itself. Added through this spec's back-edge by `050-constitution`'s third promotion round, which classified the rule as universal but code-governing, as its second round did `QUAL-DELEG-001`.

## Severity and ID-stability invariants

Inherited unchanged from `specs/008-security-rules/data-model.md`: MUST/MUST NOT are blocking errors, SHOULD/SHOULD NOT are advisory warnings; each Statement's obligation keywords come from one tier, with MAY permitted beside them as a permission that carries no tier; an assigned ID is permanent (never renumbered, moved-within-file, or reused after removal); two rules in one file never share an ID.
