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

Item 3 is soluble: adopters who install `ductus` separately (via `cargo install ductus` or the release tarball) before running `/ductus` get the acceleration; adopters who don't get the existing markdown-only walk. That's the same fallback contract every other runtime-aware command honors.

## Behavior

This scenario extends the runtime to cover the `/ductus` bootstrap procedure end-to-end. Concretely:

1. **New primitives ship in `ductus`**, each a thin pure-Rust function with a `clap` args struct and an MCP tool name:
   - `fetch-archive` — download a URL to a tempfile and verify a sha256 sidecar; returns the local tempfile path.
   - `extract-archive` — untar/unzip a local archive into a staging directory; returns the staging path and a list of extracted files.
   - Substitute-and-write and the managed-block merge are `apply-manifest` and `merge-managed-block` (scenario `apply-manifest`); the `substitute-templates` and `merge-claude-md` primitives first specified here were retired by `coverage-residue-cleanup`.
2. **Command-resolution surface extends to bootstrap procedures.** `ductus exec ductus` resolves to `framework/bootstrap/ductus.md` after `framework/commands/<name>.md` and the installed candidates (`Host::command_file_candidates`, three layouts) fail. The bootstrap file gains a `## Instructions` section under the parseable conventions; its existing prose stays in the file's other sections, which the Instructions cite as the markdown-only reference — no `## Markdown-only reference` heading was added. Spec 056 later split that prose between `framework/bootstrap/ductus.md` and `framework/bootstrap/ductus-procedure.md`.
3. **Frontmatter gains a `parity:` field** for the bootstrap. The natural bound is `strict-files` on the post-run project tree (every adopter-visible file written by `/ductus` is byte-equal between the runtime-driven and LLM-driven paths) plus `semantic-fields` for any human-readable completion-message text that varies between runs.
4. **Fixture `runtime/tests/fixtures/ductus-basic/`** ships a minimal adopter-project skeleton plus a tarball-shaped staging asset, and a golden JSONL stream capturing the expected envelope sequence. The parity test stages the fixture, runs `ductus exec ductus`, and asserts file-tree equivalence against the expected post-run tree.
5. **Bootstrap completion message** continues to advertise the optional runtime as it does today (per task 22 of the parent spec) — no detect-and-warn elsewhere.

The markdown-only path remains unchanged for adopters without `ductus` installed: the LLM walks the same prose, performs the same fetch/extract/substitute steps, and produces the same output tree. The §runtime-boundary invariant holds end-to-end.

> **Signpost:** `048-govern-acquired-runtime` made the runtime required and acquired by `/ductus`; the optional-runtime and markdown-only-path statements above describe the design this scenario shipped under.

## Edge Cases

- **Adopter installed `ductus` after `/ductus` started.** Not supported; the runtime is detected at invocation, not mid-procedure. Adopters re-run `/ductus` to pick up the acceleration on subsequent passes.
- **First-run vs update-mode CLAUDE.md merge.** `CLAUDE.md` is a `create` `apply-manifest` entry, written only when absent, with its placeholders substituted, and never overwritten; the managed block inside it is merged by `merge-managed-block`, which reports `created`, `inserted`, `updated`, or `unchanged` (scenario `apply-manifest`).
- **Partial extraction failure.** If `extract-archive` errors mid-extract, the staging tempdir is left behind for inspection; the primitive surfaces the partial state via the `error` envelope (operational failure, not domain finding). The adopter's target tree is never written to with partial state — substitution and write are a separate step on a fully-extracted staging tree.
- **Template substitution with literal `{project}` content.** Adopters may legitimately have `{project}` as literal text in their source files (e.g., docs that describe templating). `apply-manifest` substitutes only the entries it reads from the staging tree the framework just extracted, never the adopter's pre-existing files, so the collision is bounded to framework content. Framework content that carries a placeholder name as literal text is protected per entry: `keep-literals` exempts named keys, `skip-if-conflict` entries are never substituted, and the `ductus.md` self-install passes an empty substitutions map.
- **Cross-platform tar/zip handling.** The release shipped `.zip` on Windows until 048's AC23; it now ships `ductus-<TARGET>.tar.gz` on every platform. `extract-archive` still chooses the format by extension and reads both; the primitive returns an operational error for unknown extensions rather than guessing.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

- **Why not split this into its own spec?** The bootstrap is one usage scenario of the runtime, not a separate architectural concept. It elaborates the parent spec's "Follow-on scenarios" section rather than introducing a new spec lineage.
- **Move `framework/bootstrap/ductus.md` into `framework/commands/`?** No. The bootstrap file's location is meaningful — it's the one entry point that lives outside the project-installable namespace. The runtime extends its command-resolution surface to also look at `framework/bootstrap/<name>.md` instead of forcing a file move that would break every existing bootstrap integration.
