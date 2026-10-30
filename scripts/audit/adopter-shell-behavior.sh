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
# shell runs against a tree shaped like an adopter's. So this family builds
# such trees and runs the real shipped hooks in them: the inner hook directly,
# and, for a project in a subdirectory of its repository (spec 059), the outer
# stub and the inner hook together through a real `git commit`.
#
# **What this family covers changed with spec 067's adopter-generator-promotion.**
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
# The fixtures are deliberately hostile to the masking conditions above:
#   * one run sets `[paths] specs-root = "features"`, where a hardcoded default
#     matches nothing (the others use the default, isolating everything else)
#   * config only at `.ductus/config.toml` — no legacy tier to fall back to
#   * the runtime reachable ONLY at `.ductus/bin/ductus`, nothing on `PATH`
#   * every fixture commit runs with the user's global and system git config
#     ignored, so a `commit.gpgsign` or a hook setting there cannot fail a case
#     for a reason that is not the shipped hook's
#
# Vacuity guard: every precondition failure is a finding, never a silent pass.
# A fixture that cannot be built is a family that did not run, and this file
# exists because checks that cannot run are what let the three defects ship.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family adopter-shell-behavior

HOOK="$ROOT/framework/bootstrap/hooks/ductus-pre-commit"
OUTER="$ROOT/framework/bootstrap/hooks/pre-commit"

for shipped in "$HOOK" "$OUTER"; do
  if [ ! -f "$shipped" ]; then
    emit "${shipped#"$ROOT"/}" "shipped adopter hook is missing — the fixture cannot be built" \
      "restore it; this family cannot verify adopter behavior without it"
    exit "$drift"
  fi
done

# fixture_failed WHAT DIR — report a fixture that could not be built, and remove
# whatever of it exists. A fixture that cannot be built is a case that did not
# run, which must not read as a pass.
fixture_failed() {
  emit "scripts/audit/adopter-shell-behavior.sh" \
    "could not build $1" \
    "ensure mktemp and git work here — a skipped run must not read as a pass"
  if [ -n "$2" ]; then rm -rf "$2"; fi
}

# fixture_git ARGS... — git as a fixture commit runs it: from the current
# directory, blind to the user's global and system config.
fixture_git() {
  GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 git "$@"
}

# scaffold FIXTURE SPECS_DIR [FEATURE] [GIT_ROOT] — lay down an adopter-shaped
# tree. Returns non-zero when the fixture could not be built (a finding, never a
# skip): a directory or hook copy it cannot lay down, a repository git did not
# create, or a seeded spec the add left untracked.
#
# GIT_ROOT defaults to FIXTURE. Naming a directory above it makes the project a
# subdirectory of its repository (spec 059), which is the layout where the work
# tree's root and the project root stop being the same directory.
#
# FEATURE defaults to `001-example`. It is a parameter because the hook
# matches spec paths by *shape*, and a fixture that only ever uses a
# three-digit name cannot tell a hook that handles every directory form from
# one that silently drops all but the oldest — which is what it did until
# spec 051 task 24: a spec numbered past 999, and every branch-scoped spec,
# went unstaged and therefore unlabelled.
scaffold() {
  local fixture="$1" specs_dir="$2" feature="${3:-001-example}" git_root="${4:-$1}"
  mkdir -p "$fixture/.ductus/bin" "$fixture/.githooks" \
           "$fixture/$specs_dir/$feature" || return 1
  cp "$HOOK" "$fixture/.githooks/ductus-pre-commit" || return 1
  chmod +x "$fixture/.githooks/ductus-pre-commit" || return 1
  printf '[paths]\nspecs-root = "%s"\n' "$specs_dir" > "$fixture/.ductus/config.toml"

  # A spec whose `dependencies:` frontmatter is stale against its link-free
  # body, seeded in YAML **block** form so a rewrite has a continuation line to
  # strand. The stub below collapses it, standing in for the primitive.
  cat > "$fixture/$specs_dir/$feature/spec.md" <<'SPEC'
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
    cd "$git_root" || exit 1
    git init -q . 2>/dev/null
    git config user.email audit@example.invalid
    git config user.name audit
    git add -A > /dev/null 2>&1
  )
  [ -f "$git_root/.git/HEAD" ] || return 1
  # A repository alone is not a fixture: every case stages or commits the
  # seeded spec, so one the add did not track (an ignore rule in the user's
  # own git config, a failed add) would run each case against nothing. The
  # path is relative to FIXTURE, which git resolves from inside the work tree
  # whether FIXTURE is its root or a subdirectory of it.
  if ! (cd "$fixture" && git ls-files --error-unmatch -- "$specs_dir/$feature/spec.md") > /dev/null 2>&1; then
    echo "adopter-shell-behavior: $specs_dir/$feature/spec.md is not tracked after git add in $fixture" >&2
    return 1
  fi
}

# install_stub FIXTURE — a runtime that records its invocations and simulates
# the dependency derivation. Not the real binary: what is under test is the
# shell's resolution and scoping, not the primitive, which has its own tests.
# It rewrites only on a `--write` call, so the hook's `--help` capability
# probe cannot stand in for the derivation it precedes.
install_stub() {
  local fixture="$1"
  cat > "$fixture/.ductus/bin/ductus" <<'STUB'
#!/usr/bin/env bash
root="$(cd "$(dirname "$0")/../.." && pwd)"
echo "$@" >> "$root/.ductus-stub-invoked"
if [ "${1:-}" = "derive-dependencies" ] && [ "${2:-}" = "--write" ]; then
  # Stand in for the primitive: collapse the seeded block-form entry, so the
  # hook's re-stage loop has a real worktree change to capture.
  for f in "$root"/*/*/spec.md; do
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
    fixture_failed "the no-runtime fixture" "$fixture"
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

# --- Case 1b: a too-old runtime is named without its own control output -----
#
# The capability guard echoes the binary's `--version` output, which this hook
# does not control. It must reach the terminal as printable text on one line:
# an escape sequence or a forged second line in it would otherwise be written
# to the operator's terminal as is (048's analysis, BE-INPUT-011).
check_too_old_runtime_echo_is_printable() {
  local fixture out status esc
  fixture="$(mktemp -d 2>/dev/null)" || fixture=""
  if [ -z "$fixture" ] || ! scaffold "$fixture" specs; then
    fixture_failed "the too-old-runtime fixture" "$fixture"
    return
  fi
  cat > "$fixture/.ductus/bin/ductus" <<'STUB'
#!/usr/bin/env bash
if [ "${1:-}" = "--version" ]; then
  printf 'ductus 0.1.0\033[31m-red\nFORGED second line\n'
  exit 0
fi
exit 2
STUB
  chmod +x "$fixture/.ductus/bin/ductus"
  out="$(cd "$fixture" && PATH=/usr/bin:/bin bash .githooks/ductus-pre-commit 2>&1)"
  status=$?
  esc="$(printf '\033')"
  if [ "$status" -eq 0 ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "the hook exited 0 over a runtime lacking the subcommands it calls" \
      "halt naming the installed version and the missing subcommand"
  fi
  case "$out" in
    *"reports: ductus 0.1.0[31m-red"*) ;;
    *) emit "framework/bootstrap/hooks/ductus-pre-commit" \
         "the too-old halt did not name the installed version as printable text: ${out:-<no output>}" \
         "echo the binary's first --version line with its control characters removed" ;;
  esac
  case "$out" in
    *"$esc"*|*FORGED*) emit "framework/bootstrap/hooks/ductus-pre-commit" \
         "the too-old halt echoed the binary's control output to the terminal" \
         "keep the echoed version to one line of printable characters" ;;
  esac
  rm -rf "$fixture"
}

# --- Case 2: the hook orchestrates correctly at a given spec root ------------
#
# Run at BOTH the default and a non-default root: with the default, a scoping
# bug is invisible and the run isolates runtime resolution; with a non-default
# root it isolates scoping. Sharing one fixture would let either mask the other.
build_and_run() {
  local specs_dir="$1" feature="${2:-001-example}"
  local fixture invoked hook_out hook_status unstaged
  fixture="$(mktemp -d 2>/dev/null)" || fixture=""
  if [ -z "$fixture" ] || ! scaffold "$fixture" "$specs_dir" "$feature"; then
    fixture_failed "a fixture for specs-root '$specs_dir' feature '$feature'" "$fixture"
    return
  fi
  install_stub "$fixture"

  hook_out="$(cd "$fixture" && PATH=/usr/bin:/bin bash .githooks/ductus-pre-commit 2>&1)"
  hook_status=$?
  if [ "$hook_status" -ne 0 ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "the shipped hook exited $hook_status in an adopter fixture with specs-root '$specs_dir', feature '$feature': ${hook_out:-<no output>}" \
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

  # Assertion 2 — both derivations run, as writes over the staged specs, not
  # only as the `--help` capability probe that precedes them. Dropping one is
  # silent: its index just stops updating, and nothing else in the pipeline
  # recomputes it on commit.
  for primitive in derive-dependencies derive-references; do
    case "$invoked" in
      *"$primitive --write --staged"*) ;;
      *) emit "framework/bootstrap/hooks/ductus-pre-commit" \
           "with specs-root '$specs_dir' the hook never invoked $primitive" \
           "invoke both derivation primitives with --write --staged before the re-stage loop" ;;
    esac
  done

  # Assertion 3 — the derivation reached the spec: the stub rewrites only on
  # the hook's `--write` call. If it did not, assertion 4
  # would compare an unchanged file against itself and report clean having
  # examined nothing (QUAL-CLAIM-001, the shape this family exists to catch).
  if grep -q '000-stale-entry' "$fixture/$specs_dir/$feature/spec.md" 2>/dev/null; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "with specs-root '$specs_dir' and feature '$feature' the seeded stale dependency survived — the hook's derivation step never reached the spec" \
      "invoke derive-dependencies from the project root so it enumerates the configured spec tree"
  fi

  # Assertion 4 — staged-spec scoping reaches any configured root. The
  # primitives rewrite the spec; the hook's re-stage loop is the only thing
  # that stages that rewrite, so a root the loop cannot match leaves the commit
  # carrying frontmatter the worktree has already superseded.
  unstaged="$(cd "$fixture" && git diff --name-only 2>/dev/null)"
  if [ -n "$unstaged" ]; then
    emit "framework/bootstrap/hooks/ductus-pre-commit" \
      "with specs-root '$specs_dir' and feature '$feature', the worktree and index disagree on $unstaged after the hook ran — a rewrite was left unstaged" \
      "match staged specs by shape — any leading segment, then a feature directory in either form parse_feature_dir accepts (NNN-slug with three-or-more digits, or identifier.n-slug) — rather than a hardcoded root or an exactly-three-digit run"
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
    fixture_failed "the deleted-spec fixture" "$fixture"
    return
  fi
  install_stub "$fixture"

  # A deletion needs something to delete *from*, so unlike the other cases this
  # one commits the scaffolded tree first, then stages the removal.
  if ! (
    cd "$fixture" || exit 1
    fixture_git commit -qm seed && git rm -q "specs/001-example/spec.md"
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

# --- Case 4: a project in a subdirectory of its repository -------------------
#
# Git runs a hook from the work tree's root and resolves a relative
# `core.hooksPath` from there (githooks(5)), so a project that is not the work
# tree's root has two places a hook can mistake for its own. Before spec 059
# both shipped hooks changed to `git rev-parse --show-toplevel` and looked for
# `.githooks/` and `.ductus/` there, where a subdirectory project keeps neither:
# the outer stub could not find the inner hook, and the inner hook could not
# find the runtime or match a staged spec, whose path git names from the work
# tree. A real `git commit` drives it, so git's own hook resolution is under
# test too rather than assumed.
check_subdirectory_project() {
  local repo_root project commit_out commit_status invoked committed call fix
  repo_root="$(mktemp -d 2>/dev/null)" || repo_root=""
  project="$repo_root/proj"
  if [ -z "$repo_root" ] || ! scaffold "$project" specs 001-example "$repo_root" \
    || ! cp "$OUTER" "$project/.githooks/pre-commit" \
    || ! chmod +x "$project/.githooks/pre-commit"; then
    fixture_failed "the subdirectory-project fixture" "$repo_root"
    return
  fi
  install_stub "$project"

  commit_out="$(cd "$repo_root" && PATH=/usr/bin:/bin \
    fixture_git -c core.hooksPath=proj/.githooks commit -qm seed 2>&1)"
  commit_status=$?
  if [ "$commit_status" -ne 0 ]; then
    emit "framework/bootstrap/hooks/pre-commit" \
      "a commit in a project in a subdirectory of its repository failed in the shipped hooks (exit $commit_status): ${commit_out:-<no output>}" \
      "find the inner hook, the runtime and the spec root from the hook's own location, not from the work tree's root"
    rm -rf "$repo_root"
    return
  fi

  invoked="$(cat "$project/.ductus-stub-invoked" 2>/dev/null)"
  for call in "derive-dependencies --write --staged" "derive-references --write --staged" \
    "label-criteria --feature 001-example"; do
    case "$call" in
      label-criteria*) fix="list staged specs from the project root (git diff --cached --relative), so the shape match sees the project's own paths" ;;
      *) fix="run the derivation from the project root — the directory above the hook — so the runtime and the spec root are found there" ;;
    esac
    case "$invoked" in
      *"$call"*) ;;
      *) emit "framework/bootstrap/hooks/ductus-pre-commit" \
           "in a project in a subdirectory of its repository the hook never ran \`$call\`" \
           "$fix" ;;
    esac
  done

  # The stub rewrote the spec on disk; only the hook's re-stage loop puts that
  # rewrite into the commit.
  committed="$(cd "$repo_root" && git show HEAD:proj/specs/001-example/spec.md 2>/dev/null)"
  case "$committed" in
    "") emit "scripts/audit/adopter-shell-behavior.sh" \
          "the subdirectory-project commit holds no proj/specs/001-example/spec.md to inspect" \
          "the fixture did not commit the spec — a skipped assertion must not read as a pass" ;;
    *000-stale-entry*) emit "framework/bootstrap/hooks/ductus-pre-commit" \
          "in a project in a subdirectory of its repository the commit captured the stale dependency the derivation had already rewritten on disk" \
          "re-stage each rewritten spec by its path from the project root" ;;
  esac

  rm -rf "$repo_root"
}

check_halts_without_runtime
check_too_old_runtime_echo_is_printable
build_and_run specs      # default root — isolates runtime resolution
build_and_run features   # configured root (spec 040) — isolates scoping
# Both directory forms the runtime recognizes, against the shipped hook.
# A digits-only filter passes the two runs above and drops both of these,
# which is precisely the state that shipped until spec 051 task 24: the
# criteria of a spec past 999, and of every branch-scoped spec, went
# unlabelled with nothing reporting it.
build_and_run specs 1000-thousandth   # sequential past the three-digit pad
build_and_run specs 1234.1-staged     # branch-scoped staging form (spec 051)
check_survives_deleted_spec
check_subdirectory_project

exit "$drift"
