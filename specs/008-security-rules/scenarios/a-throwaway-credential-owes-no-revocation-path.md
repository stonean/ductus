---
section: "Two Rule Files"
---

# A-throwaway-credential-owes-no-revocation-path

## Context

`configuration-cross.md`'s `CFG-ENV-013` sets aside a credential for a throwaway local or test resource that grants no access to a production or shared system (017's scenario `a-throwaway-credential-is-not-a-secret`, owner, 2026-11-19). 017's review of 2026-11-20 asked whether the carve-out reaches the siblings that still say "secret" unqualified, and waived the question as the owner's. Read against each sibling's text, only `BE-DATA-012` holds a throwaway credential to a duty the carve-out's reasoning removes: it requires every secret's revocation path to be documented and flags a secret-introducing spec that omits one, and nothing routes that duty to `CFG-ENV-013`, so a `docker-compose.yml` `POSTGRES_PASSWORD: postgres` owes a revocation path though revoking it protects nothing (the agent's recommendation, adopted by the operator, 2026-11-23).

## Behavior

`framework/rules/security-backend.md` states, and `/{project}:analyze` flags accordingly:

- **`BE-DATA-012`.** A credential for a throwaway local or test resource that grants no access to a production or shared system is not a secret under this rule, worded as `CFG-ENV-013` words it, so a spec introducing one owes no revocation path or replacement procedure for it. The rule stays a MUST for every other secret.
- **`BE-DATA-003` and `BE-DATA-008` keep "secret" unqualified.** Each already routes a secret kept in source, version control or a committed manifest to `CFG-ENV-013`, which sets the throwaway credential aside, so the carve-out reaches the compose example through that routing, and widening their wording would change no outcome.

## Edge Cases

- **A credential that also opens a production or shared system is a secret,** as `CFG-ENV-013`'s scenario has it: a test key valid against a shared staging service, or a local value reused in production, still owes `BE-DATA-012`'s revocation path.
- **`CFG-ENV-001` and `CFG-ENV-010` keep the word too,** as 017's Resolved Questions record: an in-application default runs wherever the application runs, production included, and an artifact delivered to end users is no throwaway local or test context.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
