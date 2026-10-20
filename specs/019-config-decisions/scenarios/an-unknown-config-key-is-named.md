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

- Where is an unknown key reported, given that most readers have no channel to report it through? The one declaration of recognized keys every reader shares is common to every option; the surface is what differs. Measured 2026-10-20, the runtime's readers of the two files are: `discover-rule-files` (`[rules]`, `[review]`) and `dashboard` (`[review]`, the session file), whose contracts sit in 022, which is in progress; `check-artifact-size` (`[artifacts]`, notices described in 063's data model, done); `resolve-session` and `write-session` (the session file, typed notices in 062, done); `resolve-constitutions` (`[constitutions]`, 055, done), `resolve-references` and `derive-references` (`[services]`, 030, done) and `check-orphaned-references` (`[migrations]`, 067, done), none with a notice field; and `Host::load` (`[host]`) and `Paths::load` (`[paths]`), which return no result and already warn on stderr. `/ductus` reads `[source]`, `[project]`, `[pinned]`, `[migrations]` and `[runtime]` by hand. Over MCP a primitive's stderr goes to the host's server log, never into the tool result the agent reads. (a, recommended) report through each reader that has a notice channel — `discover-rule-files` and `check-artifact-size` — plus a whole-file callout in `dashboard`, so `/status` names every unknown key; every other runtime reader warns on stderr, and `/ductus` names the keys in the sections it reads in its summary. One reopen, 063, for its `notices` row. Both of this scenario's examples (`[rules] surface`, `[review] disabled-rule-file`) are reported on the review run that reads them; a typo in `[paths]`, `[services]` or the session file reaches an agent-driven run only through stderr, and the operator on the next `/status`. (b) give every reader a channel: a notice list on `resolve-constitutions`, `resolve-references`, `derive-references` and `check-orphaned-references`, and a notice kind on the session notices, so every run that reads a key reports it. Reopens 063, 062, 055, 030 and 067 for their result contracts. (c) report in one place — the `dashboard` callout, `/ductus`'s summary and stderr — and amend this scenario's "on the run that reads it" to "on the next `/status` or `/ductus` run". No reopen; the scenario's own `[rules] surface` example then goes unreported on the review run that reads it.

## Resolved Questions

*None yet.*
