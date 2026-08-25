#!/usr/bin/env bash
# scripts/audit/adopter-shell-behavior.sh — Family 22 of /audit.
#
# The shipped adopter shell works in an adopter's tree, not just in ours.
#
# `framework/bootstrap/hooks/ductus-pre-commit` is executed by adopters, but
# this repo runs a *different copy* of that same job: our `.githooks/pre-commit`
# is a separate file, and our own layout uses the default spec root with a
# locally built runtime. Every assumption those two facts mask is invisible to a
# green run here. Three defects reached adopters through exactly that gap on
# 2026-08-17, all silent and all exit 0:
#
#   * the config ladder had no `.ductus/` tier, so a *converged* adopter fell
#     through to the default spec root and enumerated the wrong tree;
#   * the hook guarded the runtime on `command -v ductus` while spec 048 had
#     moved it into the store, which is never on `PATH`;
#   * the hook's staged-spec detection hardcoded `specs`, so on a non-default
#     `[paths] specs-root` (spec 040) nothing was re-staged and every commit
#     landed with frontmatter the generators had already superseded on disk.
#
# None was reachable by grep: each is a *behavior* that only appears when the
# shell runs against a tree shaped like an adopter's. So this family builds one
# and runs the real shipped hook in it.
#
# **What this family covers changed with spec 022's adopter-generator-promotion.**
# The two frontmatter derivations moved out of shell and into the
# `derive-dependencies` / `derive-references` primitives, which carry their own
# end-to-end golden tests (`runtime/tests/derive_*_golden.rs`). What remains
# shell — and therefore what this family owns — is the hook's *orchestration*:
# resolving the runtime through the pointer, halting when it is unreachable,
# scoping staged specs at any configured root, and re-staging what the
# primitives rewrote. The runtime is stubbed so the family stays hermetic (no
# `cargo build`, identical in CI and on a laptop); the stub simulates the
# derivation so the re-stage assertion still has a rewrite to catch.
#
# The fixture is deliberately hostile to the masking conditions above:
#   * `[paths] specs-root = "features"` — never the default
#   * config only at `.ductus/config.toml` — no legacy tier to fall back to
#   * the runtime reachable ONLY at `.ductus/bin/ductus`, nothing on `PATH`
#
# Vacuity guard: every precondition failure is a finding, never a silent pass.
# A fixture that cannot be built is a family that did not run, and this file
# exists because checks that cannot run are what let the three defects ship.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family adopter-shell-behavior

HOOK="$ROOT/framework/bootstrap/hooks/ductus-pre-commit"

if [ ! -f "$HOOK" ]; then
  emit "${HOOK#"$ROOT"/}" "shipped adopter hook is missing — the fixture cannot be built" \
    "restore it; this family cannot verify adopter behavior without it"
  exit "$drift"
fi

# scaffold FIXTURE SPECS_DIR — lay down an adopter-shaped tree. Returns
# non-zero when the fixture could not be built (a finding, never a skip).
scaffold() {
  local fixture="$1" specs_dir="$2"
  mkdir -p "$fixture/.ductus/bin" "$fixture/.githooks" \
           "$fixture/$specs_dir/001-example" || return 1
  cp "$HOOK" "$fixture/.githooks/ductus-pre-commit" || return 1
  chmod +x "$fixture/.githooks/ductus-pre-commit" || return 1
  printf '[paths]\nspecs-root = "%s"\n' "$specs_dir" > "$fixture/.ductus/config.toml"

  # A spec whose `dependencies:` frontmatter is stale against its link-free
  # body, seeded in YAML **block** form so a rewrite has a continuation line to
  # strand. The stub below collapses it, standing in for the primitive.
  cat > "$fixture/$specs_dir/001-example/spec.md" <<'SPEC'
---
status: draft
dependencies:
  - 000-stale-entry
next-criterion: 1
---

# Example

## Acceptance Criteria

- [ ] the hook re-stages this spec after the primitives rewrite it
SPEC

  (
    cd "$fixture" || exit 1
    git init -q . 2>/dev/null
    git config user.email audit@example.invalid
    git config user.name audit
    git add -A > /dev/null 2>&1
  )
  [ -f "$fixture/.git/HEAD" ] || return 1
}

# install_stub FIXTURE — a runtime that records its invocations and simulates
# the dependency derivation. Not the real binary: what is under test is the
# shell's resolution and scoping, not the primitive, which has its own tests.
install_stub() {
  local fixture="$1"
  cat > "$fixture/.ductus/bin/ductus" <<'STUB'
#!/usr/bin/env bash
root="$(cd "$(dirname "$0")/../.." && pwd)"
echo "$@" >> "$root/.ductus-stub-invoked"
if [ "${1:-}" = "derive-dependencies" ]; then
  # Stand in for the primitive: collapse the seeded block-form entry, so the
  # hook's re-stage loop has a real worktree change to capture.
  for f in "$root"/*/001-example/spec.md; do
    [ -f "$f" ] || continue
    sed -e 's/^dependencies:$/dependencies: []/' -e '/^  - 000-stale-entry$/d' \
      "$f" > "$f.tmp" && mv "$f.tmp" "$f"
  done
fi
exit 0
STUB
  chmod +x "$fixture/.ductus/bin/ductus"
}

# --- Case 1: an unreachable runtime halts the commit -------------------------
#
# The generators produce derived frontmatter the commit captures, so a silent
# skip lands values the primitives would have superseded — the third defect
# above, wearing the costume of a check that passed. §runtime-boundary settles
# the direction: acquisition failure halts rather than degrades.
check_halts_without_runtime() {
  local fixture out status
  fixture="$(mktemp -d 2>/dev/null)" || fixture=""
  if [ -z "$fixture" ] || ! scaffold "$fixture" specs; then
    emit "scripts/audit/adopter-shell-behavior.sh" \
      "could not build the no-runtime fixture" \
      "ensure mktemp and git work here — a skipped run must not read as a pass"
    [ -n "$fixture" ] && rm -rf "$fixture"
    return
  fi
  # Deliberately no .ductus/bin/ductus, and nothing named ductus on PATH.
  out="$(cd "$fixture" && PATH=/usr/bin:/bin bash .githooks/ductus-pre-commit 2>&1)"
  status=$?
  if [ "$status" -eq 0 ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "the hook exited 0 with no runtime reachable — the derived frontmatter was silently skipped and the commit would capture stale values" \
      "halt with a non-zero exit naming /ductus as the fix; --no-verify is the deliberate bypass"
  fi
  case "$out" in
    *runtime*) ;;
    *) emit "framework/bootstrap/hooks/ductus-pre-commit" \
         "the hook halted without naming the runtime as the cause: ${out:-<no output>}" \
         "say what is missing and how to get it, so the halt is actionable" ;;
  esac
  rm -rf "$fixture"
}

# --- Case 2: the hook orchestrates correctly at a given spec root ------------
#
# Run at BOTH the default and a non-default root: with the default, a scoping
# bug is invisible and the run isolates runtime resolution; with a non-default
# root it isolates scoping. Sharing one fixture would let either mask the other.
build_and_run() {
  local specs_dir="$1"
  local fixture invoked hook_out hook_status unstaged
  fixture="$(mktemp -d 2>/dev/null)" || fixture=""
  if [ -z "$fixture" ] || ! scaffold "$fixture" "$specs_dir"; then
    emit "scripts/audit/adopter-shell-behavior.sh" \
      "could not build a fixture for specs-root '$specs_dir'" \
      "ensure mktemp and git work here — a skipped run must not read as a pass"
    [ -n "$fixture" ] && rm -rf "$fixture"
    return
  fi
  install_stub "$fixture"

  hook_out="$(cd "$fixture" && PATH=/usr/bin:/bin bash .githooks/ductus-pre-commit 2>&1)"
  hook_status=$?
  if [ "$hook_status" -ne 0 ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "the shipped hook exited $hook_status in an adopter fixture with specs-root '$specs_dir': ${hook_out:-<no output>}" \
      "run it against a tree with that spec root and a store-only runtime"
  fi

  invoked="$(cat "$fixture/.ductus-stub-invoked" 2>/dev/null)"

  # Assertion 1 — the runtime resolves through the pointer. /ductus never puts
  # `ductus` on PATH (spec 048), so a PATH-only guard never fires for anyone.
  if [ -z "$invoked" ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "with specs-root '$specs_dir' the hook never invoked the runtime, although .ductus/bin/ductus was present and executable" \
      "resolve the runtime through the .ductus/bin/ductus pointer before falling back to PATH"
  fi

  # Assertion 2 — both derivations run. Dropping one is silent: its index just
  # stops updating, and nothing else in the pipeline recomputes it on commit.
  for primitive in derive-dependencies derive-references; do
    case "$invoked" in
      *"$primitive"*) ;;
      *) emit "framework/bootstrap/hooks/ductus-pre-commit" \
           "with specs-root '$specs_dir' the hook never invoked $primitive" \
           "invoke both derivation primitives with --write --staged before the re-stage loop" ;;
    esac
  done

  # Assertion 3 — the derivation reached the spec. If it did not, assertion 4
  # would compare an unchanged file against itself and report clean having
  # examined nothing (QUAL-CLAIM-001, the shape this family exists to catch).
  if grep -q '000-stale-entry' "$fixture/$specs_dir/001-example/spec.md" 2>/dev/null; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "with specs-root '$specs_dir' the seeded stale dependency survived — the hook's derivation step never reached the spec" \
      "invoke derive-dependencies from the repo root so it enumerates the configured spec tree"
  fi

  # Assertion 4 — staged-spec scoping reaches any configured root. The
  # primitives rewrite the spec; the hook's re-stage loop is the only thing
  # that stages that rewrite, so a root the loop cannot match leaves the commit
  # carrying frontmatter the worktree has already superseded.
  unstaged="$(cd "$fixture" && git diff --name-only 2>/dev/null)"
  if [ -n "$unstaged" ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "with specs-root '$specs_dir', the worktree and index disagree on $unstaged after the hook ran — a rewrite was left unstaged" \
      "match staged specs by shape (any leading segment, then NNN-slug/spec.md) rather than a hardcoded 'specs' path"
  fi

  rm -rf "$fixture"
}

# --- Case 3: a staged spec DELETION does not fail the hook -------------------
#
# The re-stage loop walks paths from `git diff --cached --name-only`, which
# includes deletions — a path that is staged but no longer on disk. Under
# `set -euo pipefail` that turns the loop's own guard into a failure mode: an
# adopter carrying `[ -f "$f" ] && git add "$f"` as the body's *last* command
# gets exit 1 from the `&&` on every deleted spec, and the whole commit dies.
# `|| continue` avoids it, and does so without the `|| true` variant that would
# also swallow a genuine `git add` failure.
#
# This shape is not hypothetical and it is not grep-able: the hook reads
# correctly and the loop is only wrong for input this repo rarely produces.
# It shipped to adopters between 2026-07-22 (2715b5b) and 2026-08-14 (2339eb0),
# where it was corrected *incidentally* while wiring `label-criteria` — nobody
# was looking at the deletion path, and nothing would have told them if the
# rewrite had kept the `&&`. A downstream project reported it on 2026-08-25,
# still running the pre-rename hook. So: exercise the deleted-spec path.
check_survives_deleted_spec() {
  local fixture hook_out hook_status
  fixture="$(mktemp -d 2>/dev/null)" || fixture=""
  if [ -z "$fixture" ] || ! scaffold "$fixture" specs; then
    emit "scripts/audit/adopter-shell-behavior.sh" \
      "could not build the deleted-spec fixture" \
      "ensure mktemp and git work here — a skipped run must not read as a pass"
    [ -n "$fixture" ] && rm -rf "$fixture"
    return
  fi
  install_stub "$fixture"

  # A deletion needs something to delete *from*, so unlike the other cases this
  # one commits the scaffolded tree first, then stages the removal.
  if ! (
    cd "$fixture" || exit 1
    git commit -qm seed && git rm -q "specs/001-example/spec.md"
  ) > /dev/null 2>&1; then
    emit "scripts/audit/adopter-shell-behavior.sh" \
      "could not stage a spec deletion in the fixture" \
      "ensure git can commit and rm here — a skipped run must not read as a pass"
    rm -rf "$fixture"
    return
  fi

  hook_out="$(cd "$fixture" && PATH=/usr/bin:/bin bash .githooks/ductus-pre-commit 2>&1)"
  hook_status=$?
  if [ "$hook_status" -ne 0 ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "the shipped hook exited $hook_status on a commit that deletes a spec: ${hook_out:-<no output>}" \
      "guard the re-stage loop with \`[ -f \"\$f\" ] || continue\` on its own line — a trailing \`&&\` short-circuits to 1 under set -e and kills the commit"
  fi

  rm -rf "$fixture"
}

check_halts_without_runtime
build_and_run specs      # default root — isolates runtime resolution
build_and_run features   # configured root (spec 040) — isolates scoping
check_survives_deleted_spec

exit "$drift"
