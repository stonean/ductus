---
section: "Bash script relationships"
---

# Adopter-generator-promotion

## Context

§Bash script relationships states the rule this scenario overturns:

> **`gen-*.sh`** (called by the pre-commit hook) — stay bash. The runtime never replaces them: pre-commit has no LLM in the loop, so they fail the eligibility rule from §runtime-boundary principle 3.

Both halves of that have lapsed. "No LLM in the loop" is not a disqualifier — §runtime-boundary's eligibility criteria require *determinism*, and criterion 2(b) names "a bash script the framework invokes (pre-commit hooks, generators, CI)" as **currently mechanical**, which is the eligible case rather than the excluded one. The reason that actually held was principle 3 as it then read — "Opt-in for adopters — the runtime MUST NOT be a prerequisite for any pipeline gate" — and a pre-commit hook is a pipeline gate. [048](../../048-govern-acquired-runtime/spec.md) retired that principle and the Opt-in invariant with it. The constraint is gone; the shell it protected remains, defended by a citation that now argues the other way.

What remains in bash runs on every adopter's machine on every commit:

- `.ductus/scripts/gen-spec-deps.sh` (323 lines) — harvests sibling-spec links into `dependencies:`, plus the SCC cycle check
- `.ductus/scripts/gen-cross-service-refs.sh` (336) — harvests cross-service URLs into `references:`, resolved against `[services]`
- `.ductus/scripts/lib/specs-root.sh` (142) — the `[paths] specs-root` resolution shared by both

All three parse YAML frontmatter and markdown structure in bash: inline-link extraction outside fenced code blocks, outside blockquote-prefixed lines, and outside `## See also`, followed by a frontmatter rewrite. Principle 3's surviving clause names that shape directly — shell pipelines that parse frontmatter or markdown structure are "**not** a sanctioned substitute for the runtime primitives or the host's file tools."

The cost is measured, not projected. Three defects reached adopters through this shell on 2026-08-17, every one silent and exit 0, recorded in `scripts/audit/adopter-shell-behavior.sh`:

- `config_path_of` resolved `.govern/config.toml` then the legacy root with no `.ductus/` tier, so a converged adopter fell through to the default spec root and their generators enumerated the wrong tree.
- The hook guarded `label-criteria` on `command -v ductus` after 048 had moved the runtime into a store that is never on `PATH` — false for every adopter.
- The hook's staged-spec detection hardcoded `specs`, so on a non-default `[paths] specs-root` nothing was re-staged and every commit landed with frontmatter the generators had already superseded on disk.

Family 22 exists because of them, and the shape of its remedy is itself the argument: it builds an adopter-shaped fixture and runs the real shipped hook inside it, because none of the three was reachable by grep. That harness is compensation for the implementation language.

The replacement is largely written already. `traverse_deps.rs` runs Tarjan's SCC over the same dependency graph — its own doc comment describes itself as "defense-in-depth that complements spec 017's `gen-spec-deps.sh` generator-side cycle check". `schema/paths.rs` owns the specs-root resolution `lib/specs-root.sh` reimplements by hand, with `tests/specs_root_override.rs` already covering it. `resolve_references.rs` reads the same `[services]` registry `gen-cross-service-refs.sh` matches against.

## Behavior

Two primitives, kept distinct because the indexes are: `dependencies:` is a blocking graph and `references:` is explicitly never one ([030](../../030-cross-service-references/spec.md) keeps them "strictly distinct"), mirroring the existing `traverse-deps` / `resolve-references` split.

- **`derive-dependencies`** — harvests sibling-spec inline links into frontmatter `dependencies:`, honoring the fenced-block, blockquote, and `## See also` exclusions, and runs the SCC cycle check across the full graph. Reuses `traverse_deps.rs`'s Tarjan implementation rather than standing up a second one.
- **`derive-references`** — harvests absolute cross-service spec URLs into frontmatter `references:`, resolving each repo against `[services]` with the root-aware matching the current script documents: a registered service with a reachable checkout is matched against that checkout's own `[paths] specs-root`, an unreachable or unregistered one against any single `[A-Za-z0-9_-]` segment. Absent-when-empty.

Each is wired at the six Rust sites plus `framework/runtime-tools.txt` per AGENTS.md's primitive-wiring rule. Both preserve the contracts their callers already depend on: a `--staged` scoping mode, so a commit rewrites only the specs it touches, and a dry-run mode reporting drift as a non-zero exit, which `framework-checks.yml` and `/ductus:target`'s safety net both read.

`.ductus/scripts/lib/specs-root.sh` is retired outright — `schema/paths.rs` is the resolution both primitives use.

`framework/bootstrap/hooks/ductus-pre-commit` stays shell, because git invokes hooks as executables, but reduces to a wrapper: resolve the runtime through the `.ductus/bin/ductus` pointer, invoke the two primitives and `label-criteria`, re-stage the rewritten specs. Its staged-spec detection reads the configured spec root from the runtime rather than resolving it in bash.

Retired alongside the scripts: `scripts/tests/test-gen-spec-deps.sh` (786 lines) and `scripts/tests/test-gen-cross-service-refs.sh` (568), replaced by Rust tests that run on every platform the runtime builds for — including Windows, where the bash suites never ran.

Consequent updates: the Shared Files manifest and the four `framework/bootstrap/configure/*.md` permission blocks drop their `gen-*.sh` entries; `framework/migrations.toml` gains an entry retiring the scripts from adopter trees; and §Bash script relationships' first bullet is rewritten to record the promotion and why the old rule lapsed.

## Edge Cases

- **Pinned generators.** An adopter listing a `gen-*.sh` in `.ductus/config.toml` `pinned.files` has opted out of updates, so the migration must not delete their copy; it reports the pin and leaves the file, matching how `govern-dir-consolidate` handles pinned invokers.
- **A binary predating the subcommands.** The same shape as the `label-criteria` case the hook already documents — an older store binary exits non-zero on an unknown subcommand. Governed by the runtime-absent policy under Resolved Questions: it halts the commit, and `/ductus`'s version compare-and-re-acquire is what clears it.
- **Empty is not one shape.** The two indexes differ at zero and a port that unifies them rewrites committed frontmatter across every spec in the corpus: a spec with no sibling links gets `dependencies: []` (the field is present and empty), while a spec with no cross-service links carries **no** `references:` field at all — absent-when-empty, with a stale block removed when its last link goes. Each primitive preserves its own rule.
- **A spec with no frontmatter, or frontmatter that will not parse.** The generators walk every tracked spec, so one malformed file must not abort the pass or silently drop the rest. The primitive reports it and continues, matching how the scenario-file readers already decline to prove anything about a file they could not read.
- **Concurrent commits and mid-write crashes.** The primitives rewrite frontmatter in place across many files, so both use the tempfile + rename contract AC13 already requires of the state-modifying primitives — a crash mid-pass leaves every spec coherent, never half-rewritten.
- **`--no-verify` is a real path, and the backstop is CI.** The escape hatch means derived frontmatter can be committed stale on purpose. That is acceptable precisely because it is not the only gate: `framework-checks.yml` runs both primitives in dry-run mode, so a bypassed hook surfaces as a failed check rather than as silently wrong data on `main`.
- **`run-generator` is retained.** It stays the sanctioned way to invoke *adopter-owned* generators from a procedure; what changes is that ductus's own two stop being among them.
- **CI.** `framework-checks.yml` invokes both generators with `--dry-run` and `generators.yml` runs them for real alongside their bash tests; both move to the primitives. The `/audit` families are deliberately untouched — the release gate runs them with no Rust toolchain by design, so migrating them would make the gate depend on compiling the artifact it gates. That constraint is separate from this one and still live.
- **`install.sh` is out of scope** — it is the curl-pipe bootstrap that runs before any runtime exists.

## Open Questions

*None — all resolved.*

## Resolved Questions

- When the runtime is unreachable — a fresh clone before `/ductus` has acquired it, or a store binary predating the two subcommands — should the pre-commit hook halt the commit or skip the generator pass? **Halt on the generators; keep the swallow for `label-criteria`.** The objection that a halt blocks every commit in a not-yet-bootstrapped clone does not survive contact with how the hook is wired: `/ductus` activates it with `git config core.hooksPath .githooks` (`framework/bootstrap/ductus.md` §Hook Installation, mirrored by `scripts/install-hooks.sh`), and that value lives in `.git/config` — local config, never carried by a clone. A fresh clone therefore has no ductus hook at all until `/ductus` runs, and `/ductus` is the command that acquires the runtime, so the two arrive together and the hook cannot fire before the binary exists.

  The two cases differ by blast radius, which is why one policy does not cover both. The generators produce **derived frontmatter the commit captures**: a silent skip lands `dependencies:` / `references:` the generators had already superseded on disk, which is verbatim the third of the 2026-08-17 defects and exactly the "check that cannot run wearing the costume of one that passed" shape Family 22 exists to catch. So an unreachable runtime fails the commit with a message naming `/ductus` as the fix, per §runtime-boundary's post-048 rule that acquisition failure halts rather than degrades — "a requirement that quietly is not one leaves both paths alive". The escape hatch is the one git already provides and adopters already know, `git commit --no-verify`; no new mechanism is introduced. `label-criteria` is recoverable by contrast — a missing `AC{n}` label is caught by the audit and assigned on the next pass, and nothing wrong is committed — so it keeps its swallow, with the justification comment rewritten to cite that difference instead of the retired opt-in principle it cites today.

  Residual risk, accepted: an adopter wired by an older `/ductus` whose store binary predates the two subcommands sees commits halt until they re-run `/ductus`. That is the same exposure the current comment already flags for `label-criteria`, and 048's compare-pinned-version-and-re-acquire is the mechanism that closes it.
