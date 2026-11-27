---
section: "Follow-on scenarios"
---

# Ductus bootstrap

## Context

The `/ductus` bootstrap installer at `framework/bootstrap/ductus.md` does almost no LLM-judgment work: it resolves the framework source (URL + version), fetches a tarball, verifies its checksum, untars into a staging directory, walks the staged tree applying include/exclude globs, substitutes template variables (`{project}`, `{cli-config-dir}`, etc.) into each file, writes the result to the adopter's project, and optionally merges into `CLAUDE.md`. Every step is deterministic file plumbing. By the spec's own §runtime-boundary principles, this is exactly the kind of procedure the runtime should accelerate — and yet the v0.1.0 runtime can't run it.

Three things kept `/ductus` out of the v0.1.0 initial-release scope:

1. **Command-resolution surface.** `ductus exec <name>` looks for `framework/commands/<name>.md` or `.claude/commands/ductus/<name>.md`. The bootstrap doc lives at `framework/bootstrap/ductus.md` — the one file that ships outside the project-installable namespace because it's invoked in projects that don't yet have `ductus`.
2. **Missing primitives.** The 14 primitives in v0.1.0 are spec/task/git-shaped (`read-spec`, `mark-task`, `derive-boundary`, etc.). The bootstrap needs file-fetch, archive-extract, template-substitute, and CLAUDE.md-merge primitives that don't exist.
3. **Bootstrap paradox.** The first invocation of `/ductus` runs in a project that doesn't yet have `ductus` files. The runtime may or may not be installed at that moment. The §runtime-boundary opt-in invariant requires the markdown-only path to keep working unaffected.

Item 3 is resolved by `/ductus` acquiring the pinned runtime itself during adoption (`framework/bootstrap/ductus.md`, its **Runtime acquisition** step), so the run needs no pre-installed binary and has no markdown-only walk.

## Behavior

This scenario extends the runtime to cover the `/ductus` bootstrap procedure end-to-end. Concretely:

1. **New primitives ship in `ductus`**, each a thin pure-Rust function with a `clap` args struct and an MCP tool name:
   - `fetch-archive` — download a URL to a tempfile and verify a sha256 sidecar; returns the local tempfile path.
   - `extract-archive` — untar/unzip a local archive into a staging directory; returns the staging path and a list of extracted files.
   - Substitute-and-write and the managed-block merge are `apply-manifest` and `merge-managed-block` (scenario `apply-manifest`).
2. **Command-resolution surface extends to bootstrap procedures.** `ductus exec ductus` resolves to `framework/bootstrap/ductus.md` after `framework/commands/<name>.md` and the installed candidates (`Host::command_file_candidates`, both layouts) fail. The bootstrap file gains a `## Instructions` section under the parseable conventions; its prose stays in the file's other sections, split between `framework/bootstrap/ductus.md` and `framework/bootstrap/ductus-procedure.md` (spec 056), which the Instructions cite.
3. **The bootstrap carries no `parity:` frontmatter field.** Its stream is checked byte for byte against `runtime/tests/golden/ductus-basic.jsonl` by `ductus_basic_stream_matches_golden`, and the post-run file state is asserted by `ductus_basic_post_run_filesystem_state_matches_expectations`, both in `runtime/tests/parity.rs`.
4. **Fixture `runtime/tests/fixtures/ductus-basic/`** ships a minimal adopter-project skeleton plus a tarball-shaped staging asset, and a golden JSONL stream capturing the expected envelope sequence. The parity test stages the fixture, runs `ductus exec install` (the fixture's stand-in for the production procedure), and asserts file-tree equivalence against the expected post-run tree.
5. **Bootstrap completion message** ends its "Next steps" list with a line saying the runtime is already acquired and wired by the run (`framework/bootstrap/ductus-procedure.md` §Post-Scaffolding Output) — no detect-and-warn elsewhere.

> **Signpost:** `048-govern-acquired-runtime` made the runtime required and acquired by `/ductus`.

## Edge Cases

- **First-run vs update-mode CLAUDE.md merge.** `CLAUDE.md` is a `create` `apply-manifest` entry, written only when absent, with its placeholders substituted, and never overwritten; the managed block inside it is merged by `merge-managed-block`, which reports `created`, `inserted`, `updated`, or `unchanged` (scenario `apply-manifest`).
- **Partial extraction failure.** If `extract-archive` errors mid-extract, the staging tempdir is left behind for inspection; the primitive surfaces the partial state via the `error` envelope (operational failure, not domain finding). The adopter's target tree is never written to with partial state — substitution and write are a separate step on a fully-extracted staging tree.
- **Template substitution with literal `{project}` content.** Adopters may legitimately have `{project}` as literal text in their source files (e.g., docs that describe templating). `apply-manifest` substitutes only the entries it reads from the staging tree the framework just extracted, never the adopter's pre-existing files, so the collision is bounded to framework content. Framework content that carries a placeholder name as literal text is protected per entry: `keep-literals` exempts named keys, `skip-if-conflict` entries are never substituted, and the `ductus.md` self-install passes an empty substitutions map.
- **Cross-platform tar/zip handling.** The release ships `ductus-<TARGET>.tar.gz` on every published platform, none of which is Windows (048's AC25). `extract-archive` chooses the format by extension and reads both; the primitive returns an operational error for unknown extensions rather than guessing.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Why not split this into its own spec?** The bootstrap is one usage scenario of the runtime, not a separate architectural concept. It elaborates the parent spec's "Follow-on scenarios" section rather than introducing a new spec lineage.
- **Move `framework/bootstrap/ductus.md` into `framework/commands/`?** No. The bootstrap file's location is meaningful — it's the one entry point that lives outside the project-installable namespace. The runtime extends its command-resolution surface to also look at `framework/bootstrap/<name>.md` instead of forcing a file move that would break every existing bootstrap integration.
