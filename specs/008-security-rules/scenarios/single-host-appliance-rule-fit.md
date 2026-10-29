---
section: "Backend (`security-backend.md`)"
---

# Single-host-appliance-rule-fit

## Context

An adopter ships an on-premises appliance: one host, installed by the customer or a reseller, often air-gapped — no external call at start, no KMS, secrets manager or orchestrator, no reach to a public CA or ACME. Verified 2026-10-26 at HEAD c0952038, three rules misfit. Since then the rules carry no Rationale and `BE-DATA-003`'s at-rest half moved to `configuration-cross.md` `CFG-ENV-013` (008 task 21); each rule's obligation below is unchanged:

- `BE-DATA-003`'s Statement sources secrets from a secrets management system or orchestrator injection, narrower than its own Verification, which accepts a mounted file, and than `BE-DATA-008`'s Statement, which accepts mounted secret volumes — so a secret generated on the host at install and mounted fails the Statement as written.
- `BE-DATA-010` requires backup keys stored separately from both the backup media and the production data plane, a term the rule never defines. On one host the only such custody is an operator-held key, which the adopter's own constitution forbids because resellers misplace it.
- `BE-DATA-011` requires validation against a trusted CA set, accepting pinning or a private CA with a documented trust anchor, and forbids disabling verification in production. The adopter wants an operator switch turning validation off for third-party servers it does not control (a backup server, an activity-forwarding destination, a virtualization manager, SMTP).

Already fitting: `BE-DATA-002`'s host-level keystore clause, and `BE-DATA-011`'s private-CA clause for the appliance's own install-minted certificates. A rule can already carry an in-place MAY conditioned on what `specs/system.md` declares (`BE-DATA-001`'s documented loopback plaintext, `BE-DATA-002`'s host keystore), so a deployment precondition needs no new schema or config key; a config key would reopen 008 Resolved Q3 and 033's `[rules]` scope. IDs are kept and text amended in place in the framework's rule sources.

## Behavior

- `BE-DATA-003`'s Statement accepts what its Verification and `BE-DATA-008` accept: a secret mounted as a file, including one generated on the host at install, that no account but the application's and the host administrator's can read (owner, 2026-10-29).
- `BE-DATA-010` defines its custody term and carries an in-place MAY for a single-host deployment declared in `specs/system.md`, for a backup kept on the host: a backup key distinct from the live-data key, held on the host outside the backup stream and outside the database account's reach, with the restore-test cadence still applying.
- Servers the product controls still verify TLS, which `BE-DATA-011`'s private-CA clause already covers; for a third-party server the product does not control, `BE-DATA-011` carries a MAY accepting a trust anchor the operator supplies at configuration time (the server's CA certificate or a pinned certificate fingerprint), and disabling verification stays forbidden.

## Edge Cases

- Backups written to an off-host server: the single-host MAY does not cover them, and the rule's full custody applies — the key stored apart from both the backup media and the appliance — so the off-host copy stays restorable after the host is lost.
- An appliance keeping both an on-host backup and an off-host copy: the on-host backup may use the single-host MAY only under a key of its own; a key that also encrypts the off-host copy keeps the full custody (owner, 2026-10-29).
- A single-host deployment not declared in `specs/system.md`: the MAY does not apply, since the rule reads the declaration.
- A third-party server whose CA certificate or certificate fingerprint the operator cannot supply: `BE-DATA-011`'s MAY does not apply, and the server is validated against a trusted CA set or not called.
- A third-party server that rotates its certificate under a pinned fingerprint: validation fails and the connection is refused until the operator supplies the new anchor; the appliance never falls back to an unverified connection.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Does `BE-DATA-011` let an operator disable validation for third-party servers the product does not control** (relaxing a MUST NOT for every adopter, against CWE-295's consequence that an attacker "might … spoof a trusted entity by interfering in the communication path"), or does it stay unchanged so an adopter records a decided per-spec departure (spec 073, still in draft)? Neither (owner, 2026-10-29): `BE-DATA-011` gains a MAY — for a server the product does not control, a trust anchor the operator supplies at configuration time, that server's CA certificate or a pinned certificate fingerprint, satisfies validation — and disabling verification stays forbidden.
- **The report places every service, the database and its backups on the host, yet lists a backup server among the third-party servers the appliance calls: does `BE-DATA-010`'s single-host MAY cover only on-host backups, and does an off-host backup server need a custody form of its own?** The MAY covers backups kept on the host only (owner, 2026-10-29). A backup sent off the host keeps the rule's full custody, its key stored apart from both the backup media and the appliance, so no new form is needed: an off-host copy exists to survive the host's loss, and a key held only on the host would leave it undecryptable then, failing the restore test the rule requires.
