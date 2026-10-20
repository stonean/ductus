---
section: "Acceptance Criteria"
---

# An-unknown-config-key-is-named

## Context

`configuration-cross.md` `CFG-ENV-012` (SHOULD, added 2026-10-19) says an unknown key in configuration a program owns is never silently ignored: it fails startup, or — where several versions of the program read the same file — it is reported by name. `ductus` reads `.ductus/config.toml` in slices: each runtime module deserializes only the section it needs and serde drops every other key (`runtime/src/host.rs` reads `[host]` this way, and `schema::paths` and the migrations-registry reader rely on the same idiom). A mistyped key in a section `ductus` owns — `[rules] surface` for `surfaces`, `[review] disabled-rule-file` — is therefore dropped without a word, and the setting the operator meant silently takes its default. Contributors on one team can run different `ductus` versions against the same committed file, so the report-by-name branch applies rather than a hard failure. The owner decided on 2026-10-19 to record the gap here so `ductus` meets the rule.

## Behavior

- A key in a `ductus`-owned section of `.ductus/config.toml` that no reader of that section recognizes is reported by name, with the section and the file. The run continues: a key a newer `ductus` added is not an error for an older one.
- It is reported on the run that reads it wherever that run has a channel. `discover-rule-files` names the unknown keys in `[rules]` and `[review]` among its notices, so `/{project}:review` reports them. `check-artifact-size` names those in `[artifacts]` among its notices, so `/{project}:clarify`, `/{project}:plan` and `/{project}:analyze` report them. Every other runtime reader warns on stderr. `/ductus` names the unknown keys in the sections it reads in its summary.
- `dashboard` names every unknown key in the file in one callout, so `/{project}:status` lists them all whichever command reads them.
- Sections `ductus` does not own are not examined.
- The set of recognized keys comes from one place, so adding a key to a reader cannot leave it reported as unknown.

## Edge Cases

- A key `ductus` once accepted and has since retired is reported as unknown, naming it, unless a migration has removed it.
- The legacy config locations (`.govern/config.toml`, `.govern.toml`) are read the same way while they are still honored.
- A field an adopter adds to an entry of a `ductus`-owned array of tables — an approval field on a `[[review.disabled-rule-files]]` entry, which 025's Non-goals invite — is reported the same way and never fails the run. That keeps the §text-first-artifacts open-schema rule's promise that no consumer errors on an unknown field, which reports such a field as informational rather than ignoring it.
- The session file, `.ductus/session.toml`, is not examined. Its top level is the exec walker's open seed: `ductus exec` copies every key into the walker context, and a host may seed a walk's arguments there (022's `feature-description`), so the file has no closed set of keys to check against.

## Open Questions

*None — all resolved.*

## Resolved Questions

- **Where is an unknown key reported, given that most readers have no channel to report it through?** Resolved 2026-10-20 by the owner, option (a): through each reader that has a notice channel — `discover-rule-files` and `check-artifact-size` — plus a whole-file callout in `dashboard`, so `/status` names every unknown key; every other runtime reader warns on stderr, and `/ductus` names the unknown keys in the sections it reads in its summary. The measurement it rested on, taken the same day: the runtime's readers of the config file were `discover-rule-files` (`[rules]`, `[review]`) and `dashboard` (`[review]`), whose contracts sit in 022; `check-artifact-size` (`[artifacts]`, notices described in 063's data model); `resolve-constitutions` (`[constitutions]`, 055), `resolve-references` and `derive-references` (`[services]`, 030) and `check-orphaned-references` (`[migrations]`, 067), none with a notice field; and `Host::load` (`[host]`) and `Paths::load` (`[paths]`), which return no result. `/ductus` reads `[source]`, `[project]`, `[pinned]`, `[migrations]` and `[runtime]` by hand. Over MCP a primitive's stderr goes to the host's server log, never into the tool result the agent reads, so a typo in `[paths]`, `[services]` or `[constitutions]` reaches an agent-driven run only through stderr and reaches the operator on the next `/status` — accepted. It reopens one done spec, 063, for its `notices` row. Rejected: (b) a notice list on every reader, which reopened 063, 062, 055, 030 and 067 for their result contracts; and (c) reporting only through `/status`, `/ductus` and stderr, which left this scenario's own `[rules] surface` example unreported on the review run that reads it.
- **Is the session file examined?** Resolved 2026-10-20 by the owner, option (a): no — see the last edge case. Measured the same day: `ductus exec` seeds the walker context from every top-level key of the session file (`runtime/src/main.rs`, `seed_context`), 022's data model names the file as a channel for `feature-description`, and 7 of the 9 parity fixtures' session files seed walk arguments there (`url`, `archive`, `rule-files`, `task-number`, `title`, …), outside the six target keys `write-session` writes. The file is gitignored and per-contributor, so the case `CFG-ENV-012`'s report-by-name branch exists for, several versions reading one committed file, does not arise. Rejected: (b) reporting every key outside the six as an informational seed, which fires on every seeded walk; and (c) a closed list of the target keys plus the walker keys documented as seedable, two at the time against about 25 the fixtures seed, so a host following the documented channel would be told its keys are unknown.
