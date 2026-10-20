---
section: "Follow-on scenarios"
---

# Config-readers-name-unknown-keys

## Context

[019](../../019-config-decisions/scenarios/an-unknown-config-key-is-named.md) requires a key in a `ductus`-owned section of `.ductus/config.toml` that no reader recognizes to be reported by name on the run that reads it (`CFG-ENV-012`), and its owner decision of 2026-10-20 picks where. Two of those surfaces are 022's: `discover-rule-files`, whose notices `/{project}:review` emits verbatim, and `dashboard`, whose callouts `/{project}:status` renders. The one declaration of recognized keys that every reader shares is a shared runtime surface, which `AGENTS.md` routes here.

## Behavior

- One registry, `runtime/src/schema/config_keys.rs`, declares the keys each `ductus`-owned section recognizes: plain keys; the fields of each named entry under `[constitutions]` and `[services]`; the fields of each `[[review.disabled-rule-files]]` entry; and `[workflows]`, retired by 043, which recognizes none. A test fails when a reader reads a key the registry lacks.
- `discover-rule-files` returns one notice per unknown key in `[rules]` and `[review]`, ahead of its other notices: `unknown config key: [rules] surface (.ductus/config.toml) — this version of ductus does not read it`.
- `dashboard`'s `config` carries `unknown-keys`, every unknown key in the file's `ductus`-owned sections, and its rendering adds a callout naming them and the file.
- Every other runtime reader of the file writes the same report to stderr for the section it reads — prefixed with `ductus:` and naming the file by the path it read — once per process, and drops a write stderr cannot take rather than failing the run: `Host::load` (`[host]`), `Paths::load` (`[paths]`), `resolve-constitutions` (`[constitutions]`), `resolve-references` and `derive-references` (`[services]`), and `check-orphaned-references` (`[migrations]`).
- The check reads key names only. A value of the wrong type is its own reader's to report, so it never stops another section's report, and a file that does not parse reports nothing here: whether its reader reports that is the reader's own contract, which this scenario leaves unchanged.

## Edge Cases

- A key name that needs quoting is shown as TOML writes it, quoted and escaped, so a newline or a terminal escape in a key cannot forge a second line (`BE-INPUT-011`).
- A field an adopter adds to a `[[review.disabled-rule-files]]` entry is reported the same way, naming the entry's index, and the entry still applies.
- Sections `ductus` does not own are not examined, and neither is the session file, whose top level is the exec walker's open seed.
- The legacy config locations are examined the same way while readers still honor them, and the notice names the file that was read.
- `resolve-references` and `derive-references` also read each registered service checkout's `[paths]` through `Paths::load`, so an unknown key in that checkout's config is named too, with that file's path: its `specs-root` decides where the service's specs are found. That path comes from committed config, so it is shown with the same characters escaped as a key.
- One key can be reported twice in one CLI run: `dashboard` names it in its callout, and the reader of its section on stderr.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
