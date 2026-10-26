#!/usr/bin/env bash
# scripts/audit/check-zero.sh — `/audit`'s precondition pass.
#
# Invokes every generator (in --dry-run) and every lint script the
# framework ships. Any non-zero exit produces a `check-zero` finding to
# stdout pointing at the failing script. When this script exits non-zero,
# /audit halts before running the family checks — running them
# against known-stale generator output produces misleading findings (per
# spec 026's bootstrap-order resolution).
#
# Order matters: the frontmatter derivations run first so downstream checks
# see fresh `dependencies:` / `references:` if anything was out of sync.
#
# Those two are runtime primitives since spec 067's adopter-generator-promotion,
# so this pass needs the binary. It builds it explicitly rather than assuming a
# prior step did: `lint-procedure-parseability.sh` below also builds it, but
# depending on the *order of a later entry* to satisfy an earlier one is the
# kind of implicit coupling that breaks silently when the list is reordered.
# Cargo is a no-op when nothing changed, so the second build is free.
#
# The primitives are invoked WITHOUT `--write`: report-only is their default,
# and this is a precondition pass, not a repair pass.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family check-zero

# Each entry is `script arg1 arg2 ...`. Run in order. Generators use the
# flag they support (`--dry-run` for the older generators; `--check` was
# added to `gen-claude-commands.sh` by spec 026 task 2 to close the gap).
# Lints already run in read-only mode by design — no flag.
runtime_bin="runtime/target/release/ductus"
# `--locked` builds from the committed Cargo.lock alone, as every other CI
# build does (BE-DEPS-002); run-all.sh runs it in framework-checks.yml and
# in the release workflow's audit job.
# The build's output is captured and shown on failure, as the loop below does
# for each check: a lockfile that disagrees with Cargo.toml is one cause among
# several (a compile error, a missing toolchain, a network failure), and the
# finding cannot say which without the output.
build_output="$(cd runtime && cargo build --release --locked --quiet 2>&1)" && build_status=0 || build_status=$?
if [ "$build_status" -ne 0 ]; then
  emit "runtime/" "cargo build --release --locked failed (exit $build_status) — the frontmatter derivations cannot run" \
    "fix what the build output below reports; if it names the lockfile because Cargo.toml changed, refresh runtime/Cargo.lock (one build without --locked); a precondition that cannot run must not read as a pass"
  # The last 40 lines, so a long warning log cannot bury the finding above.
  while IFS= read -r line; do
    echo "             $line"
  done <<< "$(printf '%s\n' "$build_output" | tail -n 40)"
  exit "$drift"
fi

checks=(
  "$runtime_bin derive-dependencies"
  "$runtime_bin derive-references"
  "scripts/gen-help-tables.sh --dry-run"
  "scripts/gen-configure-mcp.sh --dry-run"
  "scripts/gen-claude-commands.sh --check"
  "scripts/lint-rule-filenames.sh"
  "scripts/lint-frontmatter.sh"
  "scripts/lint-procedure-parseability.sh"
  "scripts/lint-tool-coverage.sh"
)

for entry in "${checks[@]}"; do
  # Capture stdout+stderr; only print on failure to keep clean runs quiet.
  output="$(eval "$entry" 2>&1)" && status=0 || status=$?
  if [ "$status" -ne 0 ]; then
    script="${entry%% *}"
    # One pipe-separated finding line, plus the captured output indented
    # for readability. The aggregator surfaces the finding line; humans
    # read the indented output to diagnose.
    emit "$script" "precondition failed (exit $status)" "re-run the script, fix what it reports, commit, and re-invoke /audit"
    while IFS= read -r line; do
      echo "             $line"
    done <<< "$output"
  fi
done

exit "$drift"
