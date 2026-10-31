---
title: "013-text-first-artifacts — data-model"
---

# 013 — Text-First Artifacts Data Model

The data structure introduced by this feature is the **YAML frontmatter schema** for spec and scenario files. This document records what 013 introduced and why; it is **not** the schema's canonical home.

## The schema lives in the constitution

[§drift-prevention](../../framework/constitution.md#drift-prevention)'s canonical-sources map names
[`framework/constitution.md` §text-first-artifacts](../../framework/constitution.md#text-first-artifacts)
as the authoritative source for the frontmatter schema for specs and scenarios, and
[§text-first-artifacts](../../framework/constitution.md#text-first-artifacts)'s own Validation Severity
subsection as the authoritative source for the severity tiers. Read the schema and the tiers there.

**This document used to reproduce both tables, and the copy rotted.** That is the failure
[§drift-prevention](../../framework/constitution.md#drift-prevention) names in as many words — *referencing
means a pointer, never a copy* — and it played out exactly as that section predicts: nothing detected it,
because no link check, anchor resolver or audit family compares two prose tables for agreement, and the
copy was read in full by every contributor and every agent that opened the file for as long as it existed.
By 2026-09-14 the copy declared a `tags` field the constitution had deleted, named `spec-ref` as a
scenario's required field where the constitution requires `section`, carried a nine-row starter tag
vocabulary the constitution no longer publishes, and assigned an advisory severity to a check that now
produces no findings at all. Every one of those was retired by
[017 — Derive, don't ask](../017-derive-dont-ask/spec.md); see the signpost in this spec's body.

The pointer replaces the copy rather than re-syncing it, per the same section's standing instruction:
when a table needs syncing more than once, it belongs to the spec that owns the behavior with a pointer
left behind.

## What 013 introduced

The durable contribution is the **frontmatter block itself** — the decision that structured spec metadata
is YAML at the top of the markdown file rather than bold-prefix prose in the body:

```yaml
---
status: clarified
dependencies: [000-slash-commands, 007-govern-workflow]
---
```

Delimiters are `---` on the first line and a closing `---` on a subsequent line. The body of the document
begins after a blank line following the closing delimiter. The frontmatter MUST be parseable as YAML.
Which fields that block carries, and at what severity each is checked, is the constitution's to state and
has changed since — the block's shape and its parse contract are what 013 fixed and what still holds.

Two properties of the schema are 013's own decisions rather than field lists, so they are recorded here
and remain true:

- **The schema is open by design.** Additional fields beyond those the constitution lists are permitted
  and ignored by uninterested consumers. This is the structural representation of "extensibility without
  coordinated parser changes" — adding an optional field requires no migration, no parser update, and no
  breakage for consumers that do not read it. The constitution states the rule and its consequences for
  stale fields.
- **Scenarios carry no `status` field, by design** — per the constitution, a scenario is written or not;
  its completion is tracked through the parent spec's `tasks.md`.
- **Order of keys is not significant** — YAML parsers are insensitive to key order.

## Notes

- **The frontmatter block is the unit the migration converts.** `framework/bootstrap/ductus-procedure.md` §Frontmatter Migration
  is the procedure, and it is the live description of the conversion; the steps
  enumerated in this spec's body record what 013 specified.
