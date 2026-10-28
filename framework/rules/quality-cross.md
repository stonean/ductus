# Code Quality Rules

Code-quality rules for every stack. They verify **code patterns** rather than design-time commitments; `/{project}:review`'s quality pass enforces them against source in scope. Rule IDs follow the format `QUAL-{CATEGORY}-{NNN}`. Categories: `STUB` (silent stubs), `GROUND` (unverified contracts), `CLAIM` (unsubstantiated clean results), `DELEG` (unenumerated delegation to shared code), `TEST` (tests that cannot fail). Severity, ID permanence, and how a project excludes this file from review follow constitution §rules.

## QUAL-STUB — Silent stubs

### QUAL-STUB-001

> A partial or unimplemented code path whose surrounding contract implies it performs work MUST fail loudly — a panic, an explicit error return, or a failing or skipped test fixture — never silently pass through. Returning a zero value, `next` unchanged from middleware, or `nil, nil`, or returning early from a handler without an error, is a silent pass-through.

**Verification:** `/{project}:review`'s quality pass flags a code path in scope when **all three** hold: (1) it is reachable under the current spec; (2) its contract implies work — it is named for a behavior, documented to do something, or called by code that depends on its effect; and (3) it returns a success, zero, or pass-through value with no loud signal. **Not flagged:** a loud signal the Statement names, `todo!`, `unimplemented!`, or a raised not-implemented error; pass-through middleware documented as deliberate; a default or interface implementation meant to be empty; a branch not yet reachable behind a feature flag or guard; and a published API schema that can diverge silently from its implementation, which `api-backend.md` `BE-SCHEMA-002` owns.

**Source:** Fail-fast principle (Jim Shore, "Fail Fast", IEEE Software, September/October 2004, pp. 21–25).

## QUAL-GROUND — Verify contracts

### QUAL-GROUND-001

> Code whose correctness depends on a contract defined outside that code — a database schema, another service's API shape, a file or wire format, or a list, schema, or format defined elsewhere in the repository — SHOULD bind to that contract so that a wrong assumption fails loudly: a typed or generated binding, a schema or migration reference the build or a test checks, a startup or first-use validation, or a test that exercises the real shape.

**Verification:** `/{project}:review`'s quality pass flags code in scope when **all three** hold: (1) its correctness depends on such a contract; (2) it encodes the assumed shape directly — a literal column, table, field, or key name, or an assumed response structure; and (3) it has none of the Statement's fail-loud bindings. **Not flagged:** a contract used through its own definition, so the compiler or type checker enforces it (an internal type referenced rather than restated); and configuration keys and values, which `configuration-cross.md` `CFG-ENV-003` and `CFG-ENV-008` own.

**Source:** Constitution §grounding; fail-fast principle (Jim Shore, "Fail Fast", IEEE Software, September/October 2004, pp. 21–25).

## QUAL-CLAIM — Unsubstantiated clean results

### QUAL-CLAIM-001

> A result that reports a clean, empty, or in-sync state SHOULD distinguish *examined the subject and found nothing* from *could not examine the subject*. When a code path skips part of its subject, cannot reach it, or has no basis to inspect it, its output SHOULD say so — a distinct return variant, an accompanying status or guidance field, or a message naming what was not examined — not a bare zero, empty collection, or success string.

**Verification:** `/{project}:review`'s quality pass flags a code path in scope when **all three** hold: (1) it emits a clean, empty, zero, or success result; (2) that result is reachable when the code did not examine part of its subject — a skipped input, an unavailable precondition, an empty or underivable window, a filtered-out set; and (3) a caller cannot tell that state from a genuinely examined clean one. **Not flagged:** an output the Statement names, or a count of what *was* examined; a message that quantifies the subject examined rather than asserting a global property; and a documented total function whose subject is always fully examinable. This rule governs a fully implemented path whose output overstates what it verified: an unimplemented path returning success is `QUAL-STUB-001`'s, and an unverified assumption about a contract defined outside the code is `QUAL-GROUND-001`'s. **Promotion criterion:** promote to MUST when two consecutive `/{project}:review --all` runs each report 5 or more instances across a repo.

**Source:** Constitution §grounding.

## QUAL-DELEG — Delegation to shared code

### QUAL-DELEG-001

> Replacing a hand-rolled predicate, parser, validator, or comparator with a shared one SHOULD be preceded by enumerating what the local version enforced **by construction** — its accepted character set, its length or form constraints, where it decided the candidate ended, its ordering, and the trust level of the input it was written for — not only what its name or doc comment declares. Any property the shared implementation does not provide SHOULD be kept local, with the reason recorded at the call site and at the shared implementation.

**Verification:** `/{project}:review`'s quality pass flags a change in scope when **all three** hold: (1) it replaces a local predicate, parser, validator, or comparator with a shared one; (2) the local version enforced at least one property the change does not name — character set, length or form, termination, ordering, or input trust level; and (3) neither the call site nor the shared implementation records which properties were checked against the shared rule and which were kept local. **Not flagged:** a delegation whose comment, commit message, or test names both the properties enumerated and the ones retained locally; a shared implementation documented as the single definition of the property, with its call sites cited; and a delegation covered by tests exercising **both** directions: the shared rule accepting input the local one rejected, and rejecting input it accepted. This rule governs the substitution of one implementation for another: a result that overstates what it verified is `QUAL-CLAIM-001`'s, and an unverified assumption about a contract defined outside the code is `QUAL-GROUND-001`'s.

**Source:** Constitution §drift-prevention and §grounding.

## QUAL-TEST — Tests that cannot fail

### QUAL-TEST-001

> A test SHOULD fail when the behavior it names is removed or broken. A test that passes whether or not the code under test behaves as named SHOULD NOT be added or counted as coverage.

**Verification:** `/{project}:review`'s quality pass flags a test in scope when **any** of these holds: (1) it exercises a path without asserting on the result the test is named for; (2) its assertions pin only values incidental to that behavior — such as a length, a type, or non-emptiness — where the content is what the behavior produces; (3) the code under test has a fallback or default path, and the test's input makes the primary path and the fallback produce the same observable result; (4) whether it can fail depends on a condition the test does not establish — its assertions compiled or branched away inside its body on some platform or feature set, or an outcome that differs only when an environment variable it never sets is present — so that in a configuration the suite runs in, it passes with the behavior broken; or (5) it never reaches the code it names, because the unit under test is mocked, stubbed, re-implemented inside the test, or not called at all. **Not flagged:** a test whose report, commit message, or comment records how it was shown to fail, by reverting the fix or mutating the code; a test asserting on the named behavior's own output, with inputs that separate it from any fallback; a test that establishes the condition it depends on, by setting the variable itself or re-running in a child process that does; a test gated as a whole, by a test-level attribute or skip marker, to the configurations where the named behavior applies; a test that replaces the unit's collaborators with doubles but calls the unit itself; and a test explicitly named as a smoke test, which claims only that a path runs. This rule governs the test itself: a code path whose output overstates what it examined is `QUAL-CLAIM-001`'s, and an unimplemented path that returns success is `QUAL-STUB-001`'s.

**Source:** Constitution §design-principles.
