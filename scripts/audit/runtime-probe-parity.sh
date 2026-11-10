#!/usr/bin/env bash
# scripts/audit/runtime-probe-parity.sh — Family 15 of /audit.
#
# Verifies that each agent whose bootstrap permission *seed* (the
# settings_template cell of its §Agent Registry row in
# framework/bootstrap/ductus.md) carries CLI entries also seeds the store
# probe, `~/.ductus/bin/ductus --version` and its Windows twin.
#
# The probe is how ductus runtime detection tells State A from State B, and it
# runs on every /ductus run that finds no ductus tool. §Permission Setup seeds
# before the Pre-flight Phase probes, so a seed carrying the probe is what keeps
# that run from prompting for it. A maintainer who drops the probe from a seed
# that still grants the CLI ships a prompt on every such run, and nothing else
# reports it.
#
# The rule is the owner's (2026-11-10, spec 029 task 15). It replaced a
# both-or-neither comparison of the seed against configure/{key}.md on the
# retired `command -v` entry, which no step runs, so that check passed without
# testing the probe. The configure files grant the steady-state pipeline, which
# runs no probe, so they are not read.
#
# Scope is each agent's own registry row, never the whole file: the probe's
# command text also appears in ductus.md's prose, which would satisfy a
# whole-file match for every agent. A row that is missing or appears twice is a
# finding, never a pass.
#
# Each probe is a fixed string in its agent's native grammar, delimiters
# included, so the Windows twin never satisfies the Unix form:
#   claude    "Bash(~/.ductus/bin/ductus --version)", and the .exe twin
#   auggie    "^~/\\.ductus/bin/ductus(\\.exe)? --version$" (one regex, both)
#   opencode  "~/.ductus/bin/ductus --version": "allow", and the .exe twin
#
# Two agents are skipped by name on stderr, never silently:
#   antigravity  its seed carries no CLI entry, so every CLI call prompts there,
#                the probe among them. The skip states that premise, so a CLI
#                entry in its row (`command(.ductus/bin/ductus` or
#                `command(~/.ductus/bin/ductus`) is a finding: the rule applies.
#   pi           no permission-gating settings (spec 064), so no seed. Family 14
#                holds its settings_template at `{}`.
#
# Adding an agent that seeds the CLI is one more check_agent line.
# macOS bash 3.2: no associative arrays, no mapfile.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family runtime-probe-parity

DUCTUS="framework/bootstrap/ductus.md"

if [ ! -f "$DUCTUS" ]; then
  emit "$DUCTUS" "agent registry source missing" "restore $DUCTUS"
  exit 1
fi

# The §Agent Registry table: the lines from its heading to the next heading.
# §MCP registration's table has rows keyed the same way, so the bound is what
# keeps a row match to the seed's table.
REGISTRY="$(awk '/^## Agent Registry$/ { f = 1; next } f && /^#/ { exit } f' "$DUCTUS")"

if [ -z "$REGISTRY" ]; then
  emit "$DUCTUS" "no §Agent Registry section found" "restore the '## Agent Registry' heading and its table in $DUCTUS"
  exit 1
fi

# registry_row <key> — set ROW to the agent's one registry row, or emit and
# return 1 when there is none or more than one.
registry_row() {
  key="$1"
  ROW="$(printf '%s\n' "$REGISTRY" | grep -F -- "| \`$key\` |")"
  rows=0
  if [ -n "$ROW" ]; then
    rows="$(printf '%s\n' "$ROW" | wc -l | tr -d ' ')"
  fi
  if [ "$rows" -ne 1 ]; then
    emit "$DUCTUS (agent $key)" \
      "expected one §Agent Registry row for '$key', found $rows" \
      "keep exactly one '| \`$key\` |' row in the §Agent Registry table"
    return 1
  fi
}

# check_agent <key> <probe-literal>... — the agent's seed carries every literal.
check_agent() {
  key="$1"
  shift
  registry_row "$key" || return
  for probe in "$@"; do
    if ! printf '%s\n' "$ROW" | grep -qF -- "$probe"; then
      emit "$DUCTUS (agent $key)" \
        "the settings_template seed grants the CLI but does not seed the store probe '$probe'" \
        "add '$probe' to the '$key' settings_template in the §Agent Registry table"
    fi
  done
}

# skip_agent <key> <reason> <cli-marker>... — name the skip on stderr, and emit
# when the row carries a CLI entry, the skip's stated premise.
skip_agent() {
  key="$1"
  reason="$2"
  shift 2
  registry_row "$key" || return
  for marker in "$@"; do
    if printf '%s\n' "$ROW" | grep -qF -- "$marker"; then
      emit "$DUCTUS (agent $key)" \
        "the settings_template seed now carries a CLI entry ('$marker'), so the store-probe rule applies" \
        "seed the store probe for '$key' and replace its skip_agent line with a check_agent line"
    fi
  done
  echo "runtime-probe-parity: skipped $key ($reason)" >&2
}

check_agent "claude" \
  '"Bash(~/.ductus/bin/ductus --version)"' \
  '"Bash(~/.ductus/bin/ductus.exe --version)"'
check_agent "auggie" \
  '"shellInputRegex": "^~/\\.ductus/bin/ductus(\\.exe)? --version$", "permission": { "type": "allow" }'
check_agent "opencode" \
  '"~/.ductus/bin/ductus --version": "allow"' \
  '"~/.ductus/bin/ductus.exe --version": "allow"'
skip_agent "antigravity" "its seed carries no CLI entry, so the store probe prompts there" \
  'command(.ductus/bin/ductus' 'command(~/.ductus/bin/ductus'
skip_agent "pi" "no permission-gating settings — no seed to carry the probe"

exit "$drift"
