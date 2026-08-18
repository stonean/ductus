---
section: "Check families"
---

# Family-22-adopter-shell-behavior

## Context

Every other family reads artifacts. This one *runs* them.

`framework/bootstrap/hooks/ductus-pre-commit` and the generators under
`.ductus/scripts/` are executed by adopters, but this repository exercises
different copies of that same job: `.githooks/pre-commit` is a separate file,
this repo's spec root is the default `specs`, and its runtime is a local
`cargo` build rather than the pointer `/ductus` installs. Each of those three
facts masks a class of assumption, and a green run here says nothing about any
of them.

Three defects reached adopters through that gap on 2026-08-17, found only by a
real adopter bootstrap under [048](../048-govern-acquired-runtime/spec.md)'s
AC10. All were silent and all exited 0:

- `config_path_of` resolved `.govern/config.toml` then the legacy root with no
  `.ductus/` tier, so a converged adopter fell through to the default spec root
  and the generators enumerated a tree that was not theirs.
- The hook guarded its `label-criteria` backstop on `command -v ductus`, while
  048 had moved the runtime into the ductus-owned store — never on `PATH`. The
  [013](../013-text-first-artifacts/spec.md) labelling backstop was dead for
  every adopter for four releases.
- The hook's staged-spec detection hardcoded `specs`. On a non-default
  `[paths] specs-root` ([040](../040-configurable-specs-dir/spec.md)) it matched
  nothing, so the re-stage loop never ran and each commit captured frontmatter
  the generators had already superseded on disk.

None was reachable by grep, because none is a stale string — each is a behavior
that appears only when the shell runs against a tree shaped like an adopter's.

## Behavior

The family builds that tree and runs the real shipped hook in it. The fixture is
constructed to be hostile to every masking condition above: `[paths] specs-root`
set to a non-default name, config present *only* at `.ductus/config.toml` with
no legacy tier to fall back to, the runtime reachable only at
`.ductus/bin/ductus` with nothing named `ductus` on `PATH`, and one staged spec
whose `dependencies:` frontmatter the generators must rewrite.

Three assertions, each isolating one cause:

1. `resolve_specs_root` answers the configured root — the config ladder reaches
   the converged tier.
2. The hook invokes the runtime — resolution goes through the pointer.
3. The worktree and index agree afterwards — the re-stage loop was scoped to the
   configured root, so no generator rewrite was left unstaged.

The fixture is built twice, at the default root and at a configured one. With
the default root a scoping bug is invisible, so that run isolates runtime
resolution; the configured-root run isolates the ladder and the scoping. Sharing
one fixture would let either failure mask the other — which is how the second
defect above first presented, as a runtime-resolution failure that was really a
scoping failure one step upstream.

The runtime is a **stub** that records its invocation, not the real binary. What
is under test is the shell's resolution and scoping, not the primitive, so the
family stays hermetic: no `cargo build`, identical behavior in CI and locally.

## Edge Cases

- **A fixture that cannot be built is a finding, never a pass.** A missing
  shipped file, an unavailable `mktemp`, or a git repo that fails to initialize
  each emit rather than skip. This family exists because checks that cannot run
  are what let the three defects ship, so it must not join them
  ([§quality-cross `QUAL-CLAIM-001`](../036-quality-cross-rules/spec.md)).
- **The spec root is interpolated into a regex.** `specs_root_of` validates the
  name against `[A-Za-z0-9_-]`, which is what makes that safe; the generators
  rely on the same guarantee.
- **The stub is not a runtime conformance test.** Whether `label-criteria`
  labels correctly belongs to [022](../022-deterministic-runtime/spec.md) and
  its own suite. Asserting only *that the hook reached it* keeps the boundary
  clean and the family fast.

## Open Questions

*None — captured during scenario authoring.*

## Resolved Questions

*None yet.*
