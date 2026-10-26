#!/usr/bin/env bash
# scripts/audit/placeholder-roundtrip.sh — Family 4 of /audit.
#
# Scan framework/commands/*.md and framework/reference/*.md for hardcoded
# tokens that should be placeholders per spec 012's multi-agent contract.
# Three patterns:
#
#   .claude/    should be {cli-config-dir}/
#   /ductus:       should be /{project}:
#   `ductus:`   should be `{project}:` (backticked command-prefix mentions)
#
# A reference file (spec 071) is held to a stricter rule. It is installed
# once, at .ductus/reference/, and shared by every agent, so it has no
# per-agent value to substitute: `{cli-config-dir}` and every agent's config
# directory are findings there. The directories are derived from the Agent
# Registry (`agent_config_dirs` in lib.sh), so a newly registered agent is
# checked without an edit here. A command file can name `{cli-config-dir}`
# because it is copied per agent; its reference cannot.
#
# `$ARGUMENTS` is a finding in a reference file too. A host substitutes the
# invocation's arguments into the command body it loads and nowhere else, so
# in a reference the token is never substituted: a sentence using it to name
# the argument reads the literal token. A line that documents the token
# rather than using it takes the line-scope marker below.
#
# Allowlist (two forms):
#   1. Line scope: a line preceded by `<!-- audit:ignore-placeholders -->`
#      on the previous (non-blank) line is skipped.
#   2. File scope: a file containing `<!-- audit:ignore-placeholders:file -->`
#      anywhere is skipped wholesale. Use for maintainer-only command files
#      that are not scaffolded into adopters (e.g., framework/commands/audit.md).
#      A reference file inherits its command file's file-scope marker, so the
#      marker is stated once per command rather than copied beside it.
#      The line marker doesn't compose with markdown tables — inserting a
#      comment between table rows breaks the table — so file scope is the
#      right tool when /ductus: references span a table or are pervasive.
#
# False-positive shape: prose that mentions `.claude/` for documentary
# purposes (e.g., "Auggie reads `CLAUDE.md` natively — no `.claude/` dir
# is created"). When this surfaces, the right fix is the ignore-comment;
# do NOT widen the regex to match audience-specific prose.
#
# Expected: this check fires on /ductus: throughout the framework's command
# sources today — the framework files are pre-substituted for the ductus
# project rather than properly templated with /{project}: placeholders.
# This is a known framework-level drift /audit is designed to surface.
# The maintainer's next move after /audit ships is either to (a) fix the
# templating across all command files, or (b) declare /ductus: as the
# canonical literal and adjust the audit. Both are valid decisions made
# AFTER /audit exists to surface the gap. v1 of /audit emits the findings;
# resolution is a follow-on framework refactor.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family placeholder-roundtrip

# Every agent's config directory, for the reference-file rule below.
AGENT_DIRS=()
while IFS= read -r dir; do
  [ -n "$dir" ] && AGENT_DIRS+=("$dir/")
done < <(agent_config_dirs)
if [ "${#AGENT_DIRS[@]}" -eq 0 ]; then
  emit "framework/bootstrap/ductus.md" \
    "could not derive the agent config-dir set from the Agent Registry table — the reference-file rule would check no agent directory and pass" \
    "restore the Agent Registry table's \`config_dir\` column, or update \`agent_config_dirs\` in scripts/audit/lib.sh to match its new shape"
  exit 1
fi

# Walk every command source and reference file, looking for the patterns.
# Per-file: skip lines preceded by the ignore comment.
for file in framework/commands/*.md framework/reference/*.md; do
  [ -f "$file" ] || continue
  is_ref=0
  case "$file" in framework/reference/*) is_ref=1 ;; esac
  # File-level skip: bail before walking lines if the marker is present —
  # in the file itself or, for a reference file, in its command file.
  if grep -q '<!-- audit:ignore-placeholders:file -->' "$file"; then
    continue
  fi
  if [ "$is_ref" -eq 1 ]; then
    cmd_file="framework/commands/$(basename "$file")"
    if [ -f "$cmd_file" ] && grep -q '<!-- audit:ignore-placeholders:file -->' "$cmd_file"; then
      continue
    fi
  fi
  prev_was_ignore=0
  line_no=0
  while IFS= read -r line; do
    line_no=$((line_no + 1))
    trimmed="${line#"${line%%[![:space:]]*}"}"  # left-trim
    # Track ignore marker on the previous content line.
    if [ "$trimmed" = "<!-- audit:ignore-placeholders -->" ]; then
      prev_was_ignore=1
      continue
    fi
    # Blank lines do not reset the ignore state — the marker still applies
    # to the next non-blank content line. (Common case: the marker sits
    # one blank line above the content.)
    if [ -z "$trimmed" ]; then
      continue
    fi
    if [ "$prev_was_ignore" -eq 1 ]; then
      prev_was_ignore=0
      continue
    fi
    # Pattern checks. Each token is an inline literal that should have
    # been a placeholder.
    if [ "$is_ref" -eq 1 ]; then
      if [[ "$line" == *"{cli-config-dir}"* ]]; then
        emit "$file:$line_no" "\`{cli-config-dir}\` in a reference file — it is shared by every agent, so there is no per-agent value to substitute" "name the agent in words (e.g. the \`cli-config-dir\` recorded in \`.ductus/session.toml\`), or move the sentence into the command file"
      fi
      if [[ "$line" == *'$ARGUMENTS'* ]]; then
        emit "$file:$line_no" "\`\$ARGUMENTS\` in a reference file — a host substitutes it only in the command body it loads, so here it is never substituted" "name the invocation's argument in words, or move the sentence into the command file; add <!-- audit:ignore-placeholders --> on the line above if the line documents the token rather than using it"
      fi
      for dir in "${AGENT_DIRS[@]}"; do
        if [[ "$line" == *"$dir"* ]]; then
          emit "$file:$line_no" "hardcoded agent directory \`$dir\` in a reference file — it is shared by every agent" "describe the layout-derived path in words, or add <!-- audit:ignore-placeholders --> if literal is intentional"
        fi
      done
    elif [[ "$line" == *".claude/"* ]]; then
      emit "$file:$line_no" "hardcoded \`.claude/\` should be \`{cli-config-dir}/\`" "replace with the placeholder, or add <!-- audit:ignore-placeholders --> if literal is intentional"
    fi
    if [[ "$line" == *"/ductus:"* ]]; then
      emit "$file:$line_no" "hardcoded \`/ductus:\` should be \`/{project}:\`" "replace with the placeholder, or add <!-- audit:ignore-placeholders --> if literal is intentional"
    fi
    # Match backticked `ductus:` mentions (e.g., \`ductus:specify\`). The earlier
    # check catches `/ductus:` with a leading slash; this one catches the
    # bare prefix that appears in code spans.
    if [[ "$line" == *'`ductus:'* ]]; then
      emit "$file:$line_no" "hardcoded \`ductus:\` should be \`{project}:\`" "replace with the placeholder, or add <!-- audit:ignore-placeholders --> if literal is intentional"
    fi
  done < "$file"
done

exit "$drift"
