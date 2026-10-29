---
section: "Backend (`security-backend.md`)"
---

# Single-host-appliance-rule-fit

## Context

An adopter ships an on-premises appliance: one host, installed by the customer or a reseller, often air-gapped — no external call at start, no KMS, secrets manager or orchestrator, no reach to a public CA or ACME. Verified 2026-10-26 at HEAD c0952038 (the rules' Statements and Verifications unchanged since 0.59.11), three rules misfit:

- `BE-DATA-003`'s Statement sources secrets from a secrets management system or orchestrator injection, narrower than its own Verification, which accepts a mounted file, and than `BE-DATA-008`'s Statement, which accepts mounted secret volumes — so a secret generated on the host at install and mounted fails the Statement as written.
- `BE-DATA-010` requires backup keys stored separately from both the backup media and the production data plane, a term the rule never defines. On one host the only such custody is an operator-held key, which the adopter's own constitution forbids because resellers misplace it.
- `BE-DATA-011` requires validation against a trusted CA set, accepting pinning or a private CA with a documented trust anchor, and forbids disabling verification in production. The adopter wants an operator switch turning validation off for third-party servers it does not control (a backup server, an activity-forwarding destination, a virtualization manager, SMTP).

Already fitting: `BE-DATA-002`'s host-level keystore clause, and `BE-DATA-011`'s private-CA clause for the appliance's own install-minted certificates. A rule can already carry an in-place MAY conditioned on what `specs/system.md` declares (`BE-DATA-001`'s documented loopback plaintext, `BE-DATA-002`'s host keystore), so a deployment precondition needs no new schema or config key; a config key would reopen 008 Resolved Q3 and 033's `[rules]` scope. IDs are kept and text amended in place in the framework's rule sources.

## Behavior

- `BE-DATA-003`'s Statement accepts what its Verification and `BE-DATA-008` accept: a secret mounted as a file, including one generated on the host at install.
- `BE-DATA-010` defines its custody term and carries an in-place MAY for a single-host deployment declared in `specs/system.md`: a backup key distinct from the live-data key, held on the host outside the backup stream and outside the database account's reach, with the restore-test cadence still applying.
- Servers the product controls still verify TLS, which `BE-DATA-011`'s private-CA clause already covers; what happens for third-party servers is the owner's policy call below.

## Edge Cases

- Backups written to an off-host server: the single-host MAY does not describe that custody.
- A single-host deployment not declared in `specs/system.md`: the MAY does not apply, since the rule reads the declaration.
- A per-spec decided exception to `BE-DATA-011` depends on the separate ask for an `/{project}:analyze` waiver; this scenario does not create one.

## Open Questions

- Owner's policy call on `BE-DATA-011`: amend the shared rule to allow disabling validation for third-party servers the product does not control (relaxing a MUST NOT for every adopter, against the rule's Rationale that the failure is invisible at runtime), or leave it unchanged so an adopter records a decided per-spec exception?
- The report places every service, the database and its backups on the host, yet lists a backup server among the third-party servers the appliance calls: does `BE-DATA-010`'s single-host MAY cover only on-host backups, and does an off-host backup server need a custody form of its own?

## Resolved Questions

*None yet.*
