#!/usr/bin/env bash
# scripts/audit/permission-wildcard-position.sh — Family 29 of /audit.
#
# `/ductus:configure` writes a canonical allow-set into every adopter's agent
# settings. An allow pattern that places its wildcard BEFORE the subcommand is
# an arbitrary-execution hole, because the wildcard spans inserted options and
# not merely the argument the author had in mind:
#
#     Bash(git -C * status *)          <- the shipped shape, before spec 023 task 21
#     git -C . -c core.pager='!sh -c "…"' status
#
# The second line matches the first and is approved with NO prompt. Both `-c`
# and `--exec-path` run arbitrary commands, so the entry approves far more than
# "git status in another worktree". Seven entries of this shape shipped in
# `configure/claude.md` and reached every adopter that ran the command; Claude
# Code's own startup linter is what eventually surfaced them.
#
# The invariant this family enforces: in a canonical ALLOW set, a wildcard may
# appear only AFTER the subcommand, where it can no longer introduce an option
# that changes what executes. `Bash(git status *)` is fine. `Bash(git * status)`
# and `Bash(git -C * status *)` are not.
#
# DENY sets are deliberately NOT a subject. There the same wildcard is a
# stronger guard — a deny pattern that matches more refuses more — so
# `Bash(git -C * rm *)` is correct and narrowing it would weaken the deny set.
# Auditing both arrays with one rule would push a maintainer to "fix" the deny
# entries into holes, which is why the extraction is allow-scoped rather than
# filtered after the fact.
#
# METHOD. Per-host, because the four configure files use four permission
# grammars and only two can express the shape at all:
#
#   29a configure/claude.md — bullet entries of the form `- `Bash(...)`` inside
#       the `permissions.allow` section, whose bounds are derived from the
#       numbered headings rather than hardcoded line numbers. Anchoring to the
#       bullet form is load-bearing: the section's own prose cites the unsafe
#       pattern as an example of what not to write, and a looser grep would
#       report that explanation as a violation of itself.
#   29b configure/opencode.md — `"pattern": "allow"` keys in the `bash` action
#       map. Value-scoped, so the `"deny"` keys in the same object are excluded
#       without needing section bounds.
#
# SKIPPED, and why the skip is not a coverage gap:
#   - configure/auggie.md uses anchored regexes (`^git add `). `*` there is a
#     quantifier, not a glob; the shape is unrepresentable.
#   - configure/antigravity.md uses token-prefix `command(git add)` with no
#     wildcard grammar at all — which is also why its line ~89 note already
#     omits the `-C` variants rather than narrowing them.
# A host whose format gains a glob later is a new subject here, not a silent
# pass; the per-host extraction is explicit so adding one is a visible edit.
#
# NOT a subject: `.claude/commands/ductus/configure.md`. It is generator output,
# so a finding there would report the generator rather than a defect, and its
# sync with the source is already gated by check-zero — the same reasoning
# Family 28 applies to the generated copy of `audit.md`.
#
# An empty extraction on a subject that exists is a finding, not a pass: an
# allow set that parses to zero entries means the parse broke, and reporting
# clean on it is the false green /audit exists to prevent.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family permission-wildcard-position

CLAUDE="framework/bootstrap/configure/claude.md"
OPENCODE="framework/bootstrap/configure/opencode.md"

for f in "$CLAUDE" "$OPENCODE"; do
  if [ ! -f "$f" ]; then
    emit "$f" "not found — this family cannot examine its subject, which is not the same as clean" \
      "restore the file or update this family's paths"
    exit 1
  fi
done

# A pattern violates the invariant when it contains a standalone `*` token with
# further content after it: ` * ` followed by a non-space. A trailing `* )` is
# the safe form, and an in-word glob (`scripts/gen-*.sh`) is not a token at all.
mid_wildcard() {
  case "$1" in
    *" * "*) return 0 ;;
    *) return 1 ;;
  esac
}

# 29a — claude.md, bullet-form Bash() entries inside the allow section.
# The section's list number is not read — only where the heading sits. Matching
# `^[0-9]+\.` rather than a literal "2."/"3." keeps the bounds derived, so
# renumbering the command's steps moves the window instead of breaking it.
start_line="$(grep -nE '^[0-9]+\. Canonical `permissions\.allow` entries:' "$CLAUDE" | cut -d: -f1 | head -1)"
end_line="$(grep -nE '^[0-9]+\. Canonical `permissions\.deny` entries:' "$CLAUDE" | cut -d: -f1 | head -1)"

if [ -z "$start_line" ] || [ -z "$end_line" ]; then
  emit "$CLAUDE" "could not locate the canonical allow/deny section headings — the parse is broken, not the file" \
    "check the \"N. Canonical \`permissions.allow\` entries:\" heading format"
else
  claude_count=0
  while IFS=: read -r lineno entry; do
    [ -n "${entry:-}" ] || continue
    claude_count=$((claude_count + 1))
    # grep -n numbered the awk slice, which starts at start_line + 1; shift back
    # to a file line so the finding is clickable.
    if mid_wildcard "$entry"; then
      emit "$CLAUDE:$((start_line + lineno))" "allow entry \`Bash($entry)\` places a wildcard before the subcommand — it also matches inserted options such as -c and --exec-path, which run arbitrary commands" \
        "pin the value that wildcard stands for, or move the wildcard after the subcommand (e.g. Bash(git status *)); if neither is possible, drop the entry and let the invocation fall through to the host's Ask prompt"
    fi
  done <<EOF
$(awk -v s="$start_line" -v e="$end_line" 'NR>s && NR<e' "$CLAUDE" \
  | grep -n '^[[:space:]]*- `Bash(' \
  | sed 's/^\([0-9][0-9]*\):[[:space:]]*- `Bash(\(.*\))`.*/\1:\2/')
EOF

  if [ "$claude_count" -eq 0 ]; then
    emit "$CLAUDE" "no Bash() allow entries extracted from the canonical allow section — the parse is broken, not the set" \
      "check the \"- \`Bash(...)\`\" bullet format inside the allow section"
  fi
fi

# 29b — opencode.md, allow-valued keys in the bash action map.
opencode_count=0
while IFS=: read -r lineno entry; do
  [ -n "${entry:-}" ] || continue
  opencode_count=$((opencode_count + 1))
  if mid_wildcard "$entry"; then
    emit "$OPENCODE:$lineno" "allow entry \"$entry\" places a wildcard before the subcommand — it also matches inserted options that run arbitrary commands" \
      "pin the value that wildcard stands for, or move the wildcard after the subcommand"
  fi
done <<EOF
$(grep -n '"[^"]*": *"allow"' "$OPENCODE" | sed 's/^\([0-9][0-9]*\):[[:space:]]*"\([^"]*\)": *"allow".*/\1:\2/')
EOF

if [ "$opencode_count" -eq 0 ]; then
  emit "$OPENCODE" "no allow entries extracted from the canonical permission map — the parse is broken, not the set" \
    "check the '\"pattern\": \"allow\"' entry format"
fi

echo "permission-wildcard-position: checked $claude_count claude + $opencode_count opencode allow entries (auggie/antigravity skipped — their formats cannot express a leading glob)" >&2

exit "$drift"
