---
section: "Rule set scope"
---

# Repr-003-bounded-numbers-and-patch-001-format-home

## Context

The owner decided on 2026-11-19 how the audit findings on `BE-REPR-003` and `BE-PATCH-001` are disposed of. The first is stricter than its source and its Verification flagged more than it needs to; the second states less than its Verification passes.

## Behavior

- `BE-REPR-003` keeps its MUST and says in one clause that it goes beyond RFC 7493 section 2.2 on purpose. RFC 7493 section 2.2 (fetched 2026-11-19) says I-JSON messages SHOULD NOT include numbers of greater magnitude or precision than an IEEE 754 double provides and RECOMMENDS a JSON string for exact interchange; it has no MUST NOT.
- Its Verification flags a field that holds such a number, declared as a JSON number, only when the spec does not bound its values within −(2^53 − 1) to 2^53 − 1. An `int64` field whose spec states that bound is not flagged.
- `BE-PATCH-001` accepts a project patch format documented in `specs/system.md` or in the spec that introduces the endpoint, provided it defines how an absent field and an explicit `null` each apply. Its Verification already passes exactly that: it flags a `PATCH` spec that accepts a plain `application/json` body with no stated absent-versus-`null` semantics, wherever the format is documented, so it is unchanged.
- `BE-COND-001` stays a MUST as written.

## Edge Cases

- A field whose spec states the safe-integer bound is not flagged whatever its declared type; the Statement still requires the string encoding for a value outside the bound.
- A project format documented in neither `specs/system.md` nor the spec that introduces the endpoint is not accepted.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
