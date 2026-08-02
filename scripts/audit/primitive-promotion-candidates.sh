#!/usr/bin/env bash
# scripts/audit/primitive-promotion-candidates.sh — Family 9 of /audit.
#
# Scan framework/commands/*.md Instructions sections for numbered steps
# that have neither a backtick-quoted runtime-primitive name nor an
# `<!-- llm:* -->` extension-point marker. Each such "prose-only" step is
# a candidate for primitive promotion (deterministic logic that could
# become a `gvrn` primitive) or for an LLM-marker annotation (when the
# step requires semantic judgment but the marker is missing).
#
# Allowlist: a numbered step preceded by `<!-- audit:ignore-promotion -->`
# on the previous content line is exempt. Genuine host-responsibility prose
# (e.g., "render the dashboard", "aggregate findings") gets the annotation.
#
# Method:
#   1. Read framework/runtime-tools.txt to load the set of primitive names.
#   2. Walk each framework/commands/*.md file.
#   3. Find the ## Instructions section; iterate numbered steps within it.
#   4. For each step, check whether it contains a backticked primitive name
#      OR an `<!-- llm:* -->` marker. If neither AND no ignore-promotion
#      annotation on the previous content line, emit a finding.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family primitive-promotion

MANIFEST="framework/runtime-tools.txt"
LEGACY_ALLOWLIST="runtime/legacy-prose-commands.txt"

# Load primitive names (skip blank lines and # comments).
primitives=()
while IFS= read -r raw; do
  line="${raw%%#*}"
  line="${line#"${line%%[![:space:]]*}"}"
  line="${line%"${line##*[![:space:]]}"}"
  [ -z "$line" ] && continue
  primitives+=("$line")
done < "$MANIFEST"

# Load legacy-prose allowlist: command files in this list have not yet
# been rewritten to the parseable conventions. They're already known to
# be prose-only and out of /audit's scope — lint-procedure-parseability
# uses the same allowlist. Family 9 inherits it for symmetry.
legacy_files=()
if [ -f "$LEGACY_ALLOWLIST" ]; then
  while IFS= read -r raw; do
    line="${raw%%#*}"
    line="${line#"${line%%[![:space:]]*}"}"
    line="${line%"${line##*[![:space:]]}"}"
    [ -z "$line" ] && continue
    legacy_files+=("$line")
  done < "$LEGACY_ALLOWLIST"
fi

is_legacy() {
  local target="$1"
  for entry in "${legacy_files[@]}"; do
    if [ "$entry" = "$target" ]; then
      return 0
    fi
  done
  return 1
}

# report_step FILE START_LINE HAS_IGNORE HAS_PRIMITIVE HAS_LLM BUFFER
#
# Emit a finding when the described step is prose-only and not allowlisted.
# A pure function of its arguments: it reads no caller-scope state and
# clears none, so the walker below owns step state end-to-end and this
# reporter can be reasoned about (and reused) in isolation. `emit` setting
# the shared `drift` flag is lib.sh's documented contract, not caller-scope
# mutation by this function.
#
# A START_LINE of 0 means "no step pending" — the no-op case at a section
# boundary or EOF before any step was seen.
report_step() {
  local file="$1" start_line="$2" has_ignore="$3" has_primitive="$4" has_llm="$5" buffer="$6"
  [ "$start_line" -eq 0 ] && return 0
  [ "$has_ignore" -eq 1 ] && return 0
  [ "$has_primitive" -eq 1 ] && return 0
  [ "$has_llm" -eq 1 ] && return 0
  # First-line summary of the step for the finding (truncate to 120 chars).
  local summary
  summary="$(printf '%s' "$buffer" | head -n 1 | cut -c 1-120)"
  emit "$file:$start_line" \
    "prose-only step without primitive call or <!-- llm:* --> marker: $summary" \
    "either invoke a runtime primitive, add an <!-- llm:* --> marker, or annotate with <!-- audit:ignore-promotion --> on the preceding line"
}

# Walk each command file.
for file in framework/commands/*.md; do
  if is_legacy "$file"; then
    continue
  fi
  # State: are we inside ## Instructions? Did the previous content line
  # carry the audit:ignore-promotion marker?
  in_instructions=0
  ignore_next=0
  step_start_line=0
  step_buffer=""
  step_has_primitive=0
  step_has_llm_marker=0
  step_has_ignore=0
  line_no=0

  # shellcheck disable=SC2094  # report_step takes "$file" as a label only; it writes to stdout via emit, never to the file this loop reads
  while IFS= read -r line; do
    line_no=$((line_no + 1))
    # Detect section boundaries.
    if [[ "$line" =~ ^##[[:space:]]+Instructions[[:space:]]*$ ]]; then
      in_instructions=1
      continue
    fi
    # Any other H2 ends the Instructions section. Flush a pending step
    # before moving on.
    if [[ "$line" =~ ^##[[:space:]] ]] && [ "$in_instructions" -eq 1 ]; then
      report_step "$file" "$step_start_line" "$step_has_ignore" \
        "$step_has_primitive" "$step_has_llm_marker" "$step_buffer"
      # Clear the pending step so a later `## Instructions` in the same
      # file starts from a clean slate rather than inheriting this one.
      step_start_line=0
      in_instructions=0
      continue
    fi
    [ "$in_instructions" -eq 0 ] && continue

    # Track ignore marker for the next step.
    trimmed="${line#"${line%%[![:space:]]*}"}"
    if [ "$trimmed" = "<!-- audit:ignore-promotion -->" ]; then
      ignore_next=1
      continue
    fi

    # Detect a numbered-step line: "N. ..." at start of line (with or
    # without leading whitespace for sub-steps, but we only care about
    # top-level for promotion candidates).
    if [[ "$line" =~ ^[0-9]+\.[[:space:]] ]]; then
      report_step "$file" "$step_start_line" "$step_has_ignore" \
        "$step_has_primitive" "$step_has_llm_marker" "$step_buffer"
      # Initialize state for the step just opened. Every field is assigned
      # here rather than cleared by the reporter, which is what keeps the
      # reporter pure — the walker owns step state end to end.
      step_start_line=$line_no
      step_buffer="$line"
      step_has_primitive=0
      step_has_llm_marker=0
      step_has_ignore=$ignore_next
      ignore_next=0
      # Check the opening line for primitive backticks or llm marker.
      for prim in "${primitives[@]}"; do
        if [[ "$line" == *"\`${prim}\`"* ]]; then
          step_has_primitive=1
          break
        fi
      done
      if [[ "$line" == *"<!-- llm:"* ]]; then
        step_has_llm_marker=1
      fi
      continue
    fi

    # Continuation lines of the current step (until next numbered step or
    # H2 section).
    if [ "$step_start_line" -gt 0 ]; then
      step_buffer+=$'\n'"$line"
      for prim in "${primitives[@]}"; do
        if [[ "$line" == *"\`${prim}\`"* ]]; then
          step_has_primitive=1
          break
        fi
      done
      if [[ "$line" == *"<!-- llm:"* ]]; then
        step_has_llm_marker=1
      fi
    fi
  done < "$file"

  # Report any final pending step at EOF. No reset is needed — the next
  # file iteration re-initializes every field at the top of the loop.
  report_step "$file" "$step_start_line" "$step_has_ignore" \
    "$step_has_primitive" "$step_has_llm_marker" "$step_buffer"
done

exit "$drift"
