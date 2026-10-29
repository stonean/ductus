---
section: "Promote to rule file (the framework's own enforceable-rule mechanism)"
---

# A-secret-at-rest-is-flagged-in-every-project-shape

## Context

008's task 21 found that a project without `security-backend.md` — frontend-only, or cross-only (`[rules] surfaces = []`) — had no rule for a secret that reaches no client artifact but sits in source or version control (a token in a `vite.config.ts` dev proxy, a deploy token in a committed script), in a container build or image (`ARG NPM_TOKEN`, a copied `.npmrc`), or in a build artifact CI packages. `CFG-ENV-009` and `CFG-ENV-010` left these to `BE-DATA-003` and `BE-DATA-008`, which install only for the `backend` surface, while a `-cross.md` file installs for every surface ([008-security-rules](../../008-security-rules/spec.md)).

## Behavior

- **`CFG-ENV-013` (MUST) owns a secret at rest in every project shape** (owner, 2026-10-28): written in source or a container-build manifest, committed to version control, passed to a container build as a build argument or environment variable, or placed into a container image or other build artifact. A build step MAY receive a secret through the build tool's secret mechanism, such as a Docker secret mount, and a secret MAY be committed encrypted when each service in each environment has its own copy, each environment its own value, decryption key and decryptor, no developer can decrypt any copy, only that service's decryptor can decrypt a copy, and the decryption keys are outside version control and every artifact (owner, 2026-10-29, per OWASP's Secrets Management Cheat Sheet §3.6).
- **The routing reads** in-application default (a fallback the application's own code supplies) → `CFG-ENV-001`, wherever the source holding it travels; reaches a client artifact (a browser bundle, a mobile app binary, or a distributed desktop or CLI build) → `CFG-ENV-010`; at rest anywhere else, a value or default in a committed configuration file included → `CFG-ENV-013`. `CFG-ENV-009`, `CFG-ENV-010`, `BE-DATA-003`, `BE-DATA-008` and `BE-DATA-012` name it that way.
- **`BE-DATA-003` keeps only its runtime-sourcing commitment and `BE-DATA-008` its container runtime-injection commitment**, for a secret the running application or container uses, each flagging one whose runtime source or injection goes unnamed; an unnamed injection into a running container the spec or plan describes is `BE-DATA-008`'s (owner, 2026-10-28), and a secret only a build step uses is neither's.
- `data-model.md` § Rule set enumerates `CFG-ENV-013` with its tier.

## Edge Cases

- **A full-stack adopter's finding changes ID.** A committed or image-embedded secret reported under `BE-DATA-003` or `BE-DATA-008` is now `CFG-ENV-013`'s; a review waiver is anchored on its rule ID and file, so one recorded against the old ID expires and the finding blocks until it is waived again.
- **A frontend-only or cross-only adopter gains blocking findings** for those cases, which no installed rule flagged before.
- **Which `/{project}:review` pass checks code against `CFG-ENV-013` is 020's open question**, as for every rule in this file: `BE-DATA-003`'s check was the security pass's, and `framework/reference/review.md` §Security pass walks "the loaded security rules" (owner, 2026-10-29, recorded on [020's every-loaded-rule-file-has-a-pass](../../020-code-review/scenarios/every-loaded-rule-file-has-a-pass.md)).
- **An adopter that excludes `configuration-cross.md` from review** (`[[review.disabled-rule-files]]`) no longer has review check code for a committed or hardcoded secret, which `BE-DATA-003` did before; `/{project}:analyze`, which no project can exclude the file from, still flags it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
