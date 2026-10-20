# 019 — Config-Persisted Decisions Data Model

Conventions for the `.ductus/config.toml` file. The installer's **Project Configuration** section (`framework/bootstrap/ductus.md`) documents every section; this file records what they have in common.

## File location and lifecycle

- Path: `.ductus/config.toml` (inside the committed `.ductus/` directory at the project root).
- Written by `/ductus`, which records the `[host]` block on every run and persists the inputs it collects on first run, so an adopted project always has one.
- Committed: it is the team-shared store. Per-contributor state lives in the gitignored `.ductus/session.toml`.
- Format: TOML. Parse errors are a hard abort in `/ductus`.

## Sections

The file is a flat collection of top-level sections. There is no umbrella namespace (`[settings]`, `[decisions]`, etc.). Each section is keyed to the thing it governs, with internal keys chosen to fit that domain's vocabulary. Every section is optional; an absent section means default behavior.

### `[pinned]` — file pinning

| Key | Type | Required | Description |
| --- | --- | --- | --- |
| `files` | array of strings | no | Destination paths (post-placeholder-resolution) for files `/ductus` should treat as `skip` instead of `update`. |

```toml
[pinned]
files = [
  ".claude/commands/myapp/implement.md",
  ".ductus/constitution.md",
]
```

Every other section is declared by the spec that introduced it and documented with the rest in the installer's Project Configuration section.

## Future sections

The flat-section layout is additive. A spec that introduces a new persisted-decision domain adds its own top-level section and chooses keys that fit its decision shape (boolean toggles, arrays, structured records); there is no required naming convention. It declares the section and its keys in the registry (`runtime/src/schema/config_keys.rs`) and documents them in the installer's Project Configuration in the same change, or every reader reports them as unknown (§Unknown keys). Domains without a section yet include recorded preferences for the agent-selection prompt (`[agents]`) and for legacy-file cleanup confirmations (`[cleanup]`).

## Schema validation

`/ductus` does not run a schema validator over `.ductus/config.toml`. Values are validated per key, at the point each section is consumed — for example, `[pinned] files` entries that match no manifest path are a no-op, and `[rules] surfaces` fails fast on an unrecognized member.

There is no commit hook for `.ductus/config.toml`. Two checks each read one section: `030-cross-service-references` gave `/ductus:analyze` a rule that reads the `[services]` registry and reports a **broken** reference — registered, checked out, target spec does not resolve — as an Advisory finding, and `/ductus:audit` Family 17 (`host-namespace-parity.sh`) reads `[host] project` to verify the slash-command namespace this repo renders matches the one it installed. Neither validates the file as a whole. The one check that reads the whole file reads key names only (§Unknown keys).

## Unknown keys

Every section the installer's Project Configuration documents is `ductus`-owned. A key in one that no reader recognizes is reported by name, with its section and the file, and never fails the run (`CFG-ENV-012`; [the scenario](scenarios/an-unknown-config-key-is-named.md)):

```text
unknown config key: [rules] surface (.ductus/config.toml) — this version of ductus does not read it
unknown config key: [[review.disabled-rule-files]] entry [0] approved-by (.ductus/config.toml) — this version of ductus does not read it
```

- **One declaration.** The recognized keys are declared once, in `runtime/src/schema/config_keys.rs`: plain keys per section; the fields of each named entry under `[constitutions]` and `[services]`, whose entry names are the project's own and are not checked; and the fields of each `[[review.disabled-rule-files]]` entry. `[host] cli-config-dir` is recognized because `Host::load` still reads it as a fallback for a project predating its move to the session file. `[workflows]`, retired by 043, recognizes nothing, so its keys are reported until the `workflows-sunset` migration removes the section.
- **Where.** `discover-rule-files` (`[rules]`, `[review]`) and `check-artifact-size` (`[artifacts]`) return the line among their notices; `dashboard` returns every unknown key in the file as `config.unknown-keys` and renders one `unknown config keys:` callout; the other runtime readers write it to stderr, naming the file by the path they read — a registered service checkout's config among them — once per process, never failing on a write stderr cannot take; and `/ductus` names those in the sections it reads in its post-scaffolding summary. On the markdown-only path a host reading a section by hand compares it with the keys the installer documents.
- **Names only.** A value of the wrong type is its reader's to report, and a document that does not parse reports nothing here. A key that needs quoting is shown quoted and escaped, as TOML writes it.
- **Not examined.** A section `ductus` does not own, and the session file, whose top level is the exec walker's open seed.
