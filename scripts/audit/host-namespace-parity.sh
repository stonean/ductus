#!/usr/bin/env bash
# scripts/audit/host-namespace-parity.sh — Family 17 of /audit.
#
# Verifies the slash-command namespace this repo *renders* matches the one
# it has *installed*.
#
# `Host::load` (runtime/src/host.rs) resolves the namespace from the
# `[host] project` key in the active config file, falling back to the repo
# directory basename when the key is absent. Every runtime-rendered
# next-action string ("Run /{project}:target …") is built from that value.
# The slash commands themselves live in `{cli-config-dir}/commands/<ns>/`
# (or `command/<ns>/` for opencode's singular layout).
#
# Nothing compared the two. A repo whose `[host]` block is missing — or
# whose `project` disagrees with the installed directory — renders
# next-actions naming a namespace that does not exist, and the operator
# gets instructions they cannot run. That is exactly the drift
# §drift-prevention exists to catch, and it went unnoticed in govern's own
# repo for a long time: the basename is `govern`, the installed namespace
# is `gov`, so `/gov:dashboard` output read "Run /govern:target …".
#
# Method:
#   17a Resolve the effective namespace the way `Host::load` does:
#       `[host] project` from `.govern/config.toml` (new-wins) or the
#       legacy root `.govern.toml`; else the repo directory basename.
#   17b Collect the installed namespace directories under every agent
#       config dir present in the repo, trying both the plural
#       `commands/` and singular `command/` layouts.
#   17c Emit a finding for each agent config dir that has installed
#       namespaces but none matching the effective one.
#
# Deliberately NOT a finding:
#   - No agent config dir, or no installed namespace under one. Nothing to
#     compare; the check asserts agreement between two things that exist,
#     not that either exists.
#   - A missing `[host]` block whose basename fallback already matches the
#     installed namespace. The fallback is documented behavior, so this
#     family checks agreement, not the presence of the block.
#
# No overlap with Family 4 (placeholder roundtrip): that one forbids a
# hardcoded `gov:` inside `framework/commands/` *sources*; this one
# compares *resolved* values in an installed repo. The two never read the
# same file.
#
# Requires `python3` (3.11+) for TOML parsing.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family host-namespace-parity

if ! command -v python3 >/dev/null 2>&1; then
  emit "(precondition)" "python3 not on PATH — cannot parse TOML" \
    "install python3 (3.11+) and re-run"
  exit 1
fi
if ! python3 -c "import tomllib" 2>/dev/null; then
  emit "(precondition)" "python3 lacks tomllib (need Python 3.11+)" \
    "upgrade python3 to 3.11+ and re-run"
  exit 1
fi

# --- 17a: resolve the effective namespace -----------------------------------

# New-wins config resolution, mirroring schema::paths::config_display_name.
CONFIG_FILE=""
if [ -f ".govern/config.toml" ]; then
  CONFIG_FILE=".govern/config.toml"
elif [ -f ".govern.toml" ]; then
  CONFIG_FILE=".govern.toml"
fi

project=""
if [ -n "$CONFIG_FILE" ]; then
  # A malformed config is treated as absent, matching `Host::load`'s
  # own tolerance — namespace resolution must not fail because of an
  # unrelated config error.
  project="$(python3 - "$CONFIG_FILE" <<'PY' 2>/dev/null || true
import sys, tomllib
try:
    with open(sys.argv[1], "rb") as fh:
        data = tomllib.load(fh)
except Exception:
    sys.exit(0)
value = (data.get("host") or {}).get("project")
if isinstance(value, str) and value:
    print(value)
PY
)"
fi

source_desc="[host] project in $CONFIG_FILE"
if [ -z "$project" ]; then
  project="$(basename "$ROOT")"
  source_desc="repo directory basename (no [host] project key)"
fi

# --- 17a2: bind the borrowed contracts to their canonical sources ------------

# This family reproduces `Host::load`'s namespace resolution in shell, so it
# depends on four contracts it does not own. Encoding them as bare literals
# made it a drift-detector that could itself drift: rename the key or add a
# fifth agent and the family keeps exiting 0 while checking the wrong thing,
# which reads as assurance rather than absence (QUAL-GROUND-001, surfaced by
# 026's review 2026-08-02). Each is now either derived from its canonical
# source or asserted against it, and a failed derivation is a finding rather
# than a silent fallback.

HOST_RS="$ROOT/runtime/src/host.rs"
REGISTRY_MD="$ROOT/framework/bootstrap/govern.md"

# The agent config dirs are DERIVED from the Agent Registry table in
# framework/bootstrap/govern.md — the canonical source per the constitution's
# canonical-sources map — rather than listed here. `config_dir` is the table's
# third column, so with a leading `|` it is awk's field 4.
CLI_DIRS=()
while IFS= read -r dir; do
  [ -n "$dir" ] && CLI_DIRS+=("$dir")
done < <(awk -F'|' '
  /^\| *`key` *\| *`name` *\| *`config_dir`/ { intable = 1; next }
  intable && /^\| *-/                        { next }
  intable && !/^\|/                          { exit }
  intable { gsub(/[` ]/, "", $4); if ($4 != "") print $4 }
' "$REGISTRY_MD" 2>/dev/null | sort -u)

if [ "${#CLI_DIRS[@]}" -eq 0 ]; then
  emit "framework/bootstrap/govern.md" \
    "could not derive the agent config-dir set from the Agent Registry table — this family would otherwise check a hardcoded set and pass while ignoring real agents" \
    "restore the Agent Registry table's \`config_dir\` column, or update this family's derivation to match its new shape"
  exit 1
fi

# The remaining three contracts are single-site in the runtime, so they are
# asserted rather than derived: if the assertion fails the literals above are
# stale and every comparison below is meaningless.
assert_contract() {
  # assert_contract FILE PATTERN DESCRIPTION
  grep -qF "$2" "$1" 2>/dev/null && return 0
  emit "${1#"$ROOT"/}" \
    "$3 is no longer present at its canonical source, so this family's mirrored copy has drifted" \
    "re-read the canonical definition and update this family, or expose the resolved namespace from the runtime so the shell cannot diverge at all"
  return 1
}

contracts_ok=1
assert_contract "$HOST_RS" '["commands", "command"]' \
  "the commands/command subdirectory pair (Host::command_file_candidates)" || contracts_ok=0
assert_contract "$HOST_RS" 'project' \
  "the [host] project key (HostBlock)" || contracts_ok=0
assert_contract "$ROOT/runtime/src/schema/paths.rs" '.govern/config.toml' \
  "the new-wins config resolution order (schema::paths)" || contracts_ok=0
[ "$contracts_ok" -eq 1 ] || exit 1

# --- 17b/17c: compare against the installed namespaces ----------------------

for cli_dir in "${CLI_DIRS[@]}"; do
  [ -d "$cli_dir" ] || continue

  # Both layouts: plural `commands/` (claude, auggie, antigravity) and
  # singular `command/` (opencode). An adopter installs into exactly one,
  # so collecting from both is safe.
  # An array, not a space-joined string: a namespace directory containing a
  # space would otherwise be re-split into bogus entries by word splitting,
  # and the count below would be wrong. Bash 3.2 compatible (indexed array).
  installed=()
  for subdir in commands command; do
    [ -d "$cli_dir/$subdir" ] || continue
    for ns_path in "$cli_dir/$subdir"/*/; do
      [ -d "$ns_path" ] || continue
      installed+=("$(basename "$ns_path")")
    done
  done

  # Nothing installed under this agent dir — nothing to compare.
  [ "${#installed[@]}" -eq 0 ] && continue

  matched=0
  for ns in "${installed[@]}"; do
    if [ "$ns" = "$project" ]; then
      matched=1
      break
    fi
  done

  if [ "$matched" -eq 0 ]; then
    installed_list="$(printf '%s, ' "${installed[@]}")"
    installed_list="${installed_list%, }"
    # With exactly one installed namespace the fix is unambiguous, so name
    # it directly — the suggested-fix column is meant to be copy-pasteable,
    # and "<one of: gov>" would have to be edited before it works.
    if [ "${#installed[@]}" -eq 1 ]; then
      value="\"$installed_list\""
    else
      value="\"<one of: $installed_list>\""
    fi
    if [ -n "$CONFIG_FILE" ]; then
      fix="set [host] project = $value in $CONFIG_FILE"
    else
      fix="create .govern/config.toml with a [host] block: project = $value"
    fi
    emit "$cli_dir" \
      "rendered namespace \"$project\" ($source_desc) matches no installed command namespace ($installed_list) — every rendered next-action names /$project:… which does not exist" \
      "$fix"
  fi
done

exit "$drift"
