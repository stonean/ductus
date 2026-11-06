# status — reference

**Where a named section lives.** A section named below with no heading here is in the command file, `status.md`, with the numbered steps this file cites. A bare `§name` is a heading here or a constitution marker; one with another file's path on its line is a heading there, unless it is a constitution marker.

## Resolving cross-service references

A spec's derived `references:` frontmatter index records each cross-service reference as a `{service, spec}` pair (see [030 — Cross-Service References](https://github.com/stonean/ductus/blob/main/specs/030-cross-service-references/spec.md)). The `resolve-references` primitive classifies each entry. The repo URL is identity and navigation only and is **never fetched**; status is read from the local checkout.

**Surfacing in `/ductus:status`.** `dashboard`'s readout lists one line per reference beneath its spec: the resolution `outcome` and, on `ok`, the linked lifecycle `status`. The matched `[services.<alias>]` entry's `description` — read from the same `.ductus/config.toml` the resolution consults — is appended for orientation when present (informational only; no rendering branches on it, so a blank description degrades to nothing). An `unregistered` reference is surfaced with a pointer to `/ductus:link` so the user can register the service; `not-checked-out` and `status-unreadable` surface as `unknown` with their distinguishing reason; a `broken` reference is surfaced here and is the same provable defect `/ductus:analyze` reports as an Advisory finding.
