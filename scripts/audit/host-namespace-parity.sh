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

# --- 17b/17c: compare against the installed namespaces ----------------------

# The four agent config dirs govern supports (spec 012 / 028 / 032).
for cli_dir in .claude .augment .opencode .agents; do
  [ -d "$cli_dir" ] || continue

  # Both layouts: plural `commands/` (claude, auggie, antigravity) and
  # singular `command/` (opencode). An adopter installs into exactly one,
  # so collecting from both is safe.
  installed=""
  for subdir in commands command; do
    [ -d "$cli_dir/$subdir" ] || continue
    for ns_path in "$cli_dir/$subdir"/*/; do
      [ -d "$ns_path" ] || continue
      ns="$(basename "$ns_path")"
      installed="$installed $ns"
    done
  done

  # Nothing installed under this agent dir — nothing to compare.
  [ -z "$installed" ] && continue

  matched=0
  for ns in $installed; do
    if [ "$ns" = "$project" ]; then
      matched=1
      break
    fi
  done

  if [ "$matched" -eq 0 ]; then
    # Trim the accumulator's leading space for a clean message.
    installed_list="${installed# }"
    installed_list="${installed_list// /, }"
    # With exactly one installed namespace the fix is unambiguous, so name
    # it directly — the suggested-fix column is meant to be copy-pasteable,
    # and "<one of: gov>" would have to be edited before it works.
    count=0
    for ns in $installed; do count=$((count + 1)); done
    if [ "$count" -eq 1 ]; then
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
