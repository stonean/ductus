---
section: "Acceptance Criteria"
---

# An-unknown-config-key-is-named

## Context

`configuration-cross.md` `CFG-ENV-012` (SHOULD, added 2026-10-19) says an unknown key in configuration a program owns is never silently ignored: it fails startup, or — where several versions of the program read the same file — it is reported by name. `ductus` reads `.ductus/config.toml` in slices: each runtime module deserializes only the section it needs and serde drops every other key (`runtime/src/host.rs` reads `[host]` this way, and `schema::paths` and the migrations-registry reader rely on the same idiom). A mistyped key in a section `ductus` owns — `[rules] surface` for `surfaces`, `[review] disabled-rule-file` — is therefore dropped without a word, and the setting the operator meant silently takes its default. Contributors on one team can run different `ductus` versions against the same committed file, so the report-by-name branch applies rather than a hard failure. The owner decided on 2026-10-19 to record the gap here so `ductus` meets the rule.

## Behavior

- A key in a `ductus`-owned section of `.ductus/config.toml` (or `.ductus/session.toml`) that no reader of that section recognizes is reported by name, with the section and the file, on the run that reads it. The run continues: a key a newer `ductus` added is not an error for an older one.
- Sections `ductus` does not own are not examined.
- The set of recognized keys comes from one place, so adding a key to a reader cannot leave it reported as unknown.

## Edge Cases

- A key `ductus` once accepted and has since retired is reported as unknown, naming it, unless a migration has removed it.
- The legacy config locations (`.govern/config.toml`, `.govern.toml`) are read the same way while they are still honored.
- A field an adopter adds to an entry of a `ductus`-owned array of tables — an approval field on a `[[review.disabled-rule-files]]` entry, which 025's Non-goals invite — is reported the same way and never fails the run. That keeps the §text-first-artifacts open-schema rule's promise that no consumer errors on an unknown field, which reports such a field as informational rather than ignoring it.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
