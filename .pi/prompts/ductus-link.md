---
description: Register a service so cross-service references resolve to its lifecycle status.
argument-hint: "[alias repo path [--description <text>] | --list]"
---

# Link

Invocation arguments: `$ARGUMENTS` (empty when none were supplied)

Register a service in `.ductus/config.toml` so cross-service references to it resolve to the linked spec's lifecycle status.

## Purpose

Records a service in the `.ductus/config.toml` `[services]` registry — its canonical repository URL and its local checkout path — so a cross-service reference whose href matches that repo can surface the linked spec's status (see [030 — Cross-Service References](https://github.com/stonean/ductus/blob/main/specs/030-cross-service-references/spec.md)). The registry is **required for status resolution, optional for referencing**: a reference whose repo matches no `[services]` entry stays a plain navigational link (the `unregistered` outcome). Registration is not derived — the local `path` is machine-local knowledge `ductus` cannot infer — so it is captured here rather than harvested.

## Context

This command does not require a session target — it edits the project-level `[services]` registry in `.ductus/config.toml`, not any feature spec. The repo URL is **identity and navigation only**: it is recorded verbatim and is **never fetched**. Status is read later from the local `path`, never over the network.

## Scope Boundaries

- Read and write only the config file — the newest of `.ductus/config.toml`, `.govern/config.toml`, or the legacy root `.govern.toml` that exists (the active-file rule, specs 042 and 049). Do NOT modify any spec, plan, scenario, source file, or session state.
- Do NOT fetch the repo URL or reach across the network. The local checkout `path` is the only state read at resolution time, and that resolution belongs to `/ductus:status` and `/ductus:analyze`, not to this command.
- Removal and edits of an existing entry stay hand-edits to `.ductus/config.toml`.
- Reference: §text-first-artifacts, §runtime-boundary, and [030 `data-model.md`](https://github.com/stonean/ductus/blob/main/specs/030-cross-service-references/data-model.md) — the canonical source for the `[services]` schema (constitution loaded by `/ductus:target`; do not re-read).

## Instructions

> **For agent runtimes**: the Invoke steps below call the ductus runtime's MCP tools; without the server, stop and say to restart or re-run `/ductus` (§runtime-host-integration).

The host prompts for and validates each field, and the register-service primitive writes the `[services.<alias>]` block; `--list` reads `.ductus/config.toml` with the host's file tools. The full procedure — argument forms, per-field validation, the additive write, and `--list` — is in the **Procedure** reference below.

<!-- audit:ignore-promotion -->
1. Parse the arguments. With `--list`, run the **List the registry** branch (read-only) and stop. With three positional values (alias, repo, path) plus an optional `--description <text>`, take them as pre-supplied field values, validate them, and prompt only for anything missing or invalid. With no arguments, collect every field interactively.

2. <!-- llm:registerService --> Run the registration flow: prompt for each missing field one at a time — alias, then repo URL, then local path, then an optional description (empty to skip) — validating each value as it is entered (see **Validation**). Then invoke register-service with the alias, repo URL, local path and description, which writes the `[services.<alias>]` block additively to the active config file, preserving every other byte (see **Additive write**). It refuses a duplicate alias, or a field its rules reject, before any write: explain the refusal and re-prompt that field. A duplicate repo and a path that resolves to no directory are warned of in its result, never refused.

<!-- audit:ignore-promotion -->
3. Confirm the result: echo the `block` register-service returned, surface each of its `warnings`, and point the user at `/ductus:status` to see the linked spec's status once the service is checked out.

## Procedure

The host walks the prompts and the validation, and the `register-service` primitive makes the write (**Additive write**), applying the rules its compiled description states (`ductus register-service --help`). This procedure is the contract the `registerService` step fulfills.

### Argument forms

- `/ductus:link` (no arguments) — interactive: prompt for every field one at a time.
- `/ductus:link <alias> <repo> <path> [--description <text>]` — shortcut: the positional values pre-supply the fields. They are still validated as if typed; prompt only for anything missing or rejected.
- `/ductus:link --list` — read-only: list the registry and stop. `--list` never combines with field arguments.

### Interactive registration

Prompt one field at a time, the same one-at-a-time interaction `/ductus:clarify` uses. Validate each field before moving to the next; on invalid input, explain the problem and re-prompt the same field. Do not batch the prompts.

1. **alias** — the short, stable handle for the service (e.g., `api`).
2. **repo** — the canonical repository URL.
3. **path** — the local checkout location.
4. **description** (optional) — a one-line note on the service's purpose; empty input skips it.

### Validation

Per field, as entered:

- **alias** — a valid bare TOML key (letters, digits, hyphens, underscores; no whitespace, dots, or quotes) and **not already present** as a `[services.<alias>]` table in `.ductus/config.toml`. A collision is **rejected** with no write — changing an existing entry is a hand-edit. This is the duplicate-alias rejection.
- **repo** — URL-shaped (a scheme such as `https://` and a host). Recorded verbatim; it is identity and navigation only and is never fetched. If the same repo is already registered under another alias, compared as resolution compares it, with a trailing `/` and `.git` removed, **warn** — a duplicate repo makes the match ambiguous (a registry finding) — but allow the write.
- **path** — the local checkout location, relative to the repo root or absolute, recorded exactly as written, and **rejected** when empty, since every reader of the registry halts on an empty one, which would resolve to this repository. If it does not currently resolve to a directory, **warn** but do **not** block: a missing checkout is the valid `not-checked-out` state, surfaced at resolution time, not a config error.
- **description** — free text, optional, informational only; no behavior depends on it.

`register-service` applies the alias, repo and path rules again, and the duplicate-alias rejection, refusing before any write.

### Additive write

The `register-service` primitive performs it, so a host walking this command and `ductus exec link` make one write:

- It writes the active config file per Scope Boundaries above (when neither location exists, it creates `.ductus/config.toml` with just the new block — a write outside the `/ductus` migration never creates a partial `.ductus/config.toml` alongside a legacy file).
- It adds a `[services.<alias>]` table with `repo`, `path`, and `description` (omitted when skipped), appended after the file's last byte, so every other table — `[host]`, `[project]`, `[paths]`, `[rules]`, `[pinned]`, `[migrations]`, `[review]`, and any sibling `[services.*]` entries — stays byte-for-byte. (The operative rule is "preserve every table other than the one being added"; the list is illustrative, not exhaustive.) This is the additive discipline already used for `.mcp.json` and permission merges.
- It writes atomically (tempfile + rename), and refuses, writing nothing, when the config changed since its read or the appended table would not read back as the registry plus the new entry.

### List the registry

- Read `.ductus/config.toml`. If `[services]` is absent or empty, report that no services are registered and show the register form (`/ductus:link <alias> <repo> <path>`).
- Otherwise, for each registered service show its alias, `repo`, `path`, the `description` when present, and a resolution-health line: **resolves** when the `path` exists as a checkout, **not checked out** when it is missing or unusable. Listing reads only the registry and the existence of each local `path` — it does not read linked specs or touch the network.

### Notes

- The `[services]` schema (`repo`, `path`, optional `description`) is declared canonically in [030 `data-model.md`](https://github.com/stonean/ductus/blob/main/specs/030-cross-service-references/data-model.md).
- A duplicate `repo` across two aliases is a registry-validation finding (ambiguous match), surfaced on resolution — this command warns at registration time but does not block it.
- Outcome semantics (`ok`, `unregistered`, `not-checked-out`, `broken`, `status-unreadable`) belong to `/ductus:status` and `/ductus:analyze`; this command only records the registry the resolver reads.
