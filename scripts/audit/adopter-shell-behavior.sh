#!/usr/bin/env bash
# scripts/audit/adopter-shell-behavior.sh — Family 22 of /audit.
#
# The shipped adopter shell works in an adopter's tree, not just in ours.
#
# `framework/bootstrap/hooks/ductus-pre-commit` and `.ductus/scripts/**` are
# executed by adopters, but this repo runs *different copies* of that same job:
# our `.githooks/pre-commit` is a separate file, and our own layout uses the
# default spec root with a locally built runtime. Every assumption those two
# facts mask is invisible to a green run here. Three defects reached adopters
# through exactly that gap on 2026-08-17, all silent and all exit 0:
#
#   * `config_path_of` resolved `.govern/config.toml` then the legacy root with
#     no `.ductus/` tier, so a *converged* adopter fell through to the default
#     spec root and their generators enumerated the wrong tree.
#   * the hook guarded `label-criteria` on `command -v ductus` while spec 048
#     had moved the runtime into the store, which is never on `PATH`.
#   * the hook's staged-spec detection hardcoded `specs`, so on a non-default
#     `[paths] specs-root` (spec 040) nothing was re-staged and every commit
#     landed with frontmatter the generators had already superseded on disk.
#
# None was reachable by grep: each is a *behavior* that only appears when the
# shell runs against a tree shaped like an adopter's. So this family builds one
# and runs the real shipped hook in it.
#
# The fixture is deliberately hostile to the masking conditions above:
#   * `[paths] specs-root = "features"` — never the default
#   * config only at `.ductus/config.toml` — no legacy tier to fall back to
#   * the runtime reachable ONLY at `.ductus/bin/ductus`, nothing on `PATH`
#   * a staged spec whose frontmatter the generators must rewrite
#
# The runtime is a stub, not the real binary: what is under test is the shell's
# *resolution and scoping*, not the primitive. That keeps the family hermetic —
# no `cargo build`, so it behaves identically in CI and on a laptop.
#
# Vacuity guard: every precondition failure is a finding, never a silent pass.
# A fixture that cannot be built is a family that did not run, and this file
# exists because checks that cannot run are what let the three defects ship.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family adopter-shell-behavior

HOOK="$ROOT/framework/bootstrap/hooks/ductus-pre-commit"
LIB="$ROOT/.ductus/scripts/lib/specs-root.sh"
GEN_DEPS="$ROOT/.ductus/scripts/gen-spec-deps.sh"
GEN_REFS="$ROOT/.ductus/scripts/gen-cross-service-refs.sh"

for f in "$HOOK" "$LIB" "$GEN_DEPS" "$GEN_REFS"; do
  if [ ! -f "$f" ]; then
    emit "${f#"$ROOT"/}" "shipped adopter file is missing — the fixture cannot be built" \
      "restore it; this family cannot verify adopter behavior without it"
    exit "$drift"
  fi
done

# build_and_run SPECS_DIR — stand up one adopter-shaped fixture with the given
# spec root, run the real shipped hook in it with nothing named `ductus` on
# PATH, and assert. Run at BOTH the default and a non-default root: with the
# default, a scoping bug is invisible and the run isolates runtime resolution;
# with a non-default root it isolates config-ladder and scoping. Sharing one
# fixture between them would let either failure mask the other.
build_and_run() {
  local specs_dir="$1"
  local fixture observed_root hook_out hook_status unstaged
  fixture="$(mktemp -d 2>/dev/null)" || fixture=""
  if [ -z "$fixture" ] || [ ! -d "$fixture" ]; then
    emit "scripts/audit/adopter-shell-behavior.sh" \
      "could not create a temp fixture directory for specs-root '$specs_dir'" \
      "ensure mktemp works here — a skipped run is indistinguishable from a pass"
    return
  fi

  mkdir -p "$fixture/.ductus/scripts/lib" "$fixture/.ductus/bin" \
           "$fixture/.githooks" "$fixture/$specs_dir/001-example"
  cp "$GEN_DEPS" "$GEN_REFS" "$fixture/.ductus/scripts/"
  cp "$LIB" "$fixture/.ductus/scripts/lib/"
  cp "$HOOK" "$fixture/.githooks/ductus-pre-commit"
  chmod +x "$fixture/.ductus/scripts/"*.sh "$fixture/.githooks/ductus-pre-commit"

  printf '[paths]\nspecs-root = "%s"\n' "$specs_dir" > "$fixture/.ductus/config.toml"

  # Stub runtime: records that the hook resolved and invoked it. Not the real
  # binary — this tests the shell's resolution, not label-criteria itself.
  cat > "$fixture/.ductus/bin/ductus" <<'STUB'
#!/usr/bin/env bash
echo "$@" >> "$(dirname "$0")/../../.ductus-stub-invoked"
exit 0
STUB
  chmod +x "$fixture/.ductus/bin/ductus"

  # A spec whose `dependencies:` frontmatter is stale against its (link-free)
  # body, so gen-spec-deps.sh must rewrite it. If the hook fails to re-stage
  # that rewrite, the worktree and the index disagree afterwards.
  cat > "$fixture/$specs_dir/001-example/spec.md" <<'SPEC'
---
status: draft
dependencies:
  - 000-stale-entry
next-criterion: 1
---

# Example

## Acceptance Criteria

- [ ] the hook re-stages this spec after the generators rewrite it
SPEC

  (
    cd "$fixture" || exit 1
    git init -q . 2>/dev/null
    git config user.email audit@example.invalid
    git config user.name audit
    git add -A > /dev/null 2>&1
  )
  if [ ! -f "$fixture/.git/HEAD" ]; then
    emit "scripts/audit/adopter-shell-behavior.sh" \
      "fixture git repo was not created for specs-root '$specs_dir'" \
      "ensure git is available; a fixture that cannot run must not read as clean"
    rm -rf "$fixture"
    return
  fi

  # Assertion 1 — the config ladder reaches the converged tier. A ladder that
  # stops at the legacy tiers resolves a converged adopter to a nonexistent
  # file and silently answers with the default root.
  observed_root="$(
    cd "$fixture" || exit 1
    ROOT="$fixture"
    . .ductus/scripts/lib/specs-root.sh
    resolve_specs_root
  )"
  if [ "$observed_root" != "$specs_dir" ]; then
    emit ".ductus/scripts/lib/specs-root.sh" \
      "resolve_specs_root answered '$observed_root' for an adopter whose .ductus/config.toml sets specs-root = '$specs_dir'" \
      "give config_path_of a .ductus/config.toml tier ahead of the .govern/ and legacy-root fallbacks"
  fi

  hook_out="$(cd "$fixture" && PATH=/usr/bin:/bin bash .githooks/ductus-pre-commit 2>&1)"
  hook_status=$?
  if [ "$hook_status" -ne 0 ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "the shipped hook exited $hook_status in an adopter fixture with specs-root '$specs_dir': ${hook_out:-<no output>}" \
      "run it against a tree with that spec root and a store-only runtime"
  fi

  # Assertion 2 — the runtime resolves through the pointer. /ductus never puts
  # `ductus` on PATH (spec 048), so a PATH-only guard never fires for anyone.
  if [ ! -f "$fixture/.ductus-stub-invoked" ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "with specs-root '$specs_dir' the hook never invoked the runtime, although .ductus/bin/ductus was present and executable" \
      "resolve the runtime through the .ductus/bin/ductus pointer, and scope staged specs with resolve_specs_root so the invoking block is reached"
  fi

  # The stale entry must be gone. It is seeded in YAML **block** form against a
  # link-free body, which makes this one check cover two failures at once:
  #
  #   * the generator never rewrote the spec at all — assertion 3 below then
  #     compares an unchanged file against itself and reports clean having
  #     examined nothing (QUAL-CLAIM-001, in the family written to catch that
  #     shape);
  #   * the generator rewrote the key but orphaned the block item beneath it,
  #     leaving invalid YAML while reporting a successful `Updated` — the
  #     defect found while building this fixture, which the inline form never
  #     reaches because it has no continuation line to strand.
  if grep -q '000-stale-entry' "$fixture/$specs_dir/001-example/spec.md" 2>/dev/null; then
    emit ".ductus/scripts/gen-spec-deps.sh" \
      "with specs-root '$specs_dir' the seeded block-form dependency survived the generator — either it never rewrote the spec, or it replaced the key and left the list item orphaned as invalid YAML" \
      "replace the whole dependencies entry, key line plus indented continuation, the way gen-cross-service-refs.sh already splices references:"
  fi

  # Assertion 3 — staged-spec scoping honors the configured root. The
  # generators rewrite the spec; the hook's re-stage loop is the only thing
  # that stages that rewrite, so a hardcoded root leaves the commit carrying
  # frontmatter the worktree has already superseded.
  unstaged="$(cd "$fixture" && git diff --name-only 2>/dev/null)"
  if [ -n "$unstaged" ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "with specs-root '$specs_dir', the worktree and index disagree on $unstaged after the hook ran — a generator rewrite was left unstaged" \
      "scope the hook's staged-spec detection with resolve_specs_root instead of a hardcoded 'specs' path"
  fi

  rm -rf "$fixture"
}

build_and_run specs      # default root — isolates runtime resolution
build_and_run features   # configured root (spec 040) — isolates ladder + scoping

exit "$drift"
