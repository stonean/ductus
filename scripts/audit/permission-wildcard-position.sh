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
# The same holds for `/ductus`'s Claude seed, the other allow set ductus
# ships, which every adopter's settings file receives before /configure
# runs. This family once read configure/claude.md alone, and the seed's
# `Bash(certutil -hashfile * SHA256)` shipped from 022's task 18 on, until
# Claude Code's startup check warned on it in an adopter ("allows any
# option at its *"; spec 023's scenario configure-permission-pattern-safety).
#
# METHOD. One host, Claude, in two allow sets:
#
#   ductus.md — the `permissions.allow` entries of the claude row's
#     settings_template in the Agent Registry section, read as JSON.
#     install.sh's copy is held to this row by Family 14 and govern.md's
#     byte copy by Family 21, so the row is the one seed subject.
#
#   configure/claude.md — bullet entries of the form `- `Bash(...)`` inside
#     the `permissions.allow` section, whose bounds are derived from the
#     numbered headings rather than hardcoded line numbers. Anchoring to the
#     bullet form is load-bearing: the section's prose explains the unsafe
#     shape as a counter-example, and a looser grep would report that
#     explanation as a violation of itself. The prose now spells the shape
#     with <wildcard> placeholders rather than a copyable literal, so this is
#     defence in depth rather than the only guard.
#
# Pi has no permission file to examine. A host whose format gains a glob later
# is a new subject here, not a silent pass; the extraction is explicit so
# adding one is a visible edit.
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

if [ ! -f "$CLAUDE" ]; then
  emit "$CLAUDE" "not found — this family cannot examine its subject, which is not the same as clean" \
    "restore the file or update this family's paths"
  exit 1
fi

# A pattern violates the invariant when it contains a standalone `*` token with
# further content after it: ` * ` followed by a non-space. A trailing `* )` is
# the safe form, and an in-word glob (`scripts/gen-*.sh`) is not a token at all.
mid_wildcard() {
  case "$1" in
    *" * "*) return 0 ;;
    *) return 1 ;;
  esac
}

# claude.md, bullet-form Bash() entries inside the allow section.
# Initialized before the branch below: when the headings do not parse, that
# branch is skipped, and a summary line referencing an unset counter would die
# on `set -u` — turning a legible finding into a shell error. Found by this
# family firing on its own subject after the heading text changed.
claude_count=0

# The section's list number is not read — only where the heading sits. Matching
# `^[0-9]+\.` rather than a literal "2."/"3." keeps the bounds derived, so
# renumbering the command's steps moves the window instead of breaking it. The
# trailing colon is not required: the heading carries a sentence after it in
# some revisions, and anchoring to punctuation made an editorial edit read as a
# broken parse.
start_line="$(grep -nE '^[0-9]+\. Canonical `permissions\.allow` entries' "$CLAUDE" | cut -d: -f1 | head -1)"
end_line="$(grep -nE '^[0-9]+\. Canonical `permissions\.deny` entries' "$CLAUDE" | cut -d: -f1 | head -1)"

if [ -z "$start_line" ] || [ -z "$end_line" ]; then
  emit "$CLAUDE" "could not locate the canonical allow/deny section headings — the parse is broken, not the file" \
    "check the \"N. Canonical \`permissions.allow\` entries:\" heading format"
else
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

# The Claude seed: Bash() allow entries of the Agent Registry claude row,
# printed as line<TAB>content. A function rather than a heredoc inside
# "$(...)": bash 3.2 misparses one whose body holds backticks.
DUCTUS="framework/bootstrap/ductus.md"
seed_count=0
seed_entries() {
  python3 - "$DUCTUS" <<'PY'
import json, re, sys

lines = open(sys.argv[1]).read().split("\n")
start = next(i for i, l in enumerate(lines) if l == "## Agent Registry")
end = next(i for i, l in enumerate(lines) if i > start and l == "### Derived values")
row = re.compile(r"^\| `claude` \| [^|]* \| `[^`]*` \| `[^`]*` \| `(\{[^`]*)` \|")
for i in range(start, end):
    m = row.match(lines[i])
    if m:
        for entry in json.loads(m.group(1)).get("permissions", {}).get("allow", []):
            b = re.fullmatch(r"Bash\((.*)\)", entry)
            if b:
                print(f"{i + 1}\t{b.group(1)}")
        break
PY
}

if [ ! -f "$DUCTUS" ]; then
  emit "$DUCTUS" "not found — this family cannot examine the seed, which is not the same as clean" \
    "restore the file or update this family's paths"
else
  seed_out="$(seed_entries)"
  seed_rc=$?
  if [ "$seed_rc" -ne 0 ]; then
    emit "$DUCTUS" "the seed parse exited $seed_rc — it could not examine its subject, which is not the same as clean" \
      "check the ## Agent Registry and ### Derived values headings and the claude row's settings_template cell"
  fi
  while IFS="$(printf '\t')" read -r lineno entry; do
    [ -n "${entry:-}" ] || continue
    seed_count=$((seed_count + 1))
    if mid_wildcard "$entry"; then
      emit "$DUCTUS:$lineno" "seed allow entry \`Bash($entry)\` places a wildcard before a further word — Claude Code lets it span inserted options, approves them with no prompt, and warns on it in every adopter" \
        "pin the value that wildcard stands for, or move the wildcard to the end; if neither is possible, drop the entry and let the step fall through to the host's Ask prompt; then append the old entry to the retired seed list"
    fi
  done <<EOF
$seed_out
EOF
  if [ "$seed_count" -eq 0 ]; then
    emit "$DUCTUS" "no Bash() allow entries extracted from the claude row's settings_template — the parse is broken, not the seed" \
      "check the row's fifth cell is its settings_template JSON in one code span"
  fi
fi

echo "permission-wildcard-position: checked $claude_count claude canonical allow entries and $seed_count claude seed allow entries" >&2

exit "$drift"
