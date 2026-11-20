---
section: "Promote to rule file (the framework's own enforceable-rule mechanism)"
---

# A-throwaway-credential-is-not-a-secret

## Context

`CFG-ENV-013` barred every credential from source, a container-build manifest and version control, so a `docker-compose.yml` `POSTGRES_PASSWORD: postgres` or a committed `.env.test` key for a local emulator was a finding, though committing it exposes nothing: the credential opens no production or shared system. The spec's Resolved Questions record the decision (owner, 2026-11-19).

## Behavior

- **A credential for a throwaway local or test resource that grants no access to a production or shared system is not a secret under `CFG-ENV-013`**, in the Statement and the Verification (a `docker-compose.yml` `POSTGRES_PASSWORD: postgres`, a committed `.env.test` key for a local emulator). The rule stays a MUST.
- **The encrypted-copy permission is as strict as before:** every condition still applies to a secret committed encrypted. `CFG-ENV-011` stays a MUST.

## Edge Cases

- **A credential that also opens a production or shared system is a secret.** A test key valid against a shared staging service, or a local value reused in production, is not throwaway.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
