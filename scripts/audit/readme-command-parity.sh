#!/usr/bin/env bash
# scripts/audit/readme-command-parity.sh — Family 33 of /audit.
#
# Every command shipped to adopters appears in the README's command tables.
#
# `/ductus:fold` shipped with spec 051 and never reached the README. Neither
# did the rest of that feature's adopter-facing surface: `/specify`'s row
# omitted `--branch`, `--branch-id`, and `--fold-into` while the `/review`,
# `/analyze`, `/prune`, and `/link` rows all documented theirs, and the
# branch-scoped `{identifier}.{n}-{slug}` numbering form appeared nowhere.
# The README was the one adopter-facing surface with no parity check behind
# it, and a manual correction guarantees nothing about the next command —
# the diligence dependency §design-principles forbids outright.
#
# WHY NEITHER NEIGHBOUR COVERS IT.
#
#   - Family 16 (installer-command-parity) pins the *installer manifest* to
#     the command sources. Its own rationale records this failure class in a
#     different artifact: prune.md "was the first to bite", a command that
#     existed in the framework and was dogfooded here yet never reached
#     adopters.
#   - Family 30 (command-flag-hint-parity) checks a command's Flags table
#     against that same command's `argument-hint:` frontmatter. It is
#     command-internal and never looks outward at the README.
#
# SCOPE, STATED RATHER THAN IMPLIED.
#
# **Presence is the assertion, not accuracy.** This family checks that a
# command is *documented*, never that its description is current or that its
# row enumerates its flags. Asserting either needs semantic judgment and
# belongs to a reviewer, not a shell family — and folding flag-level parity in
# here silently would make one family's name describe two checks. A clean exit
# means "every shipped command is named in the README", and nothing more.
#
# `/ductus:audit`'s absence from the README is **correct** and must stay so:
# it is maintainer-only and adopters never invoke it. The exclusion set comes
# from `scripts/maintainer-only-commands.txt` through lib.sh — the same list
# Family 16 uses — because two copies of it is how the two would drift.
#
# HOW A COMMAND IS RECOGNIZED, AND THE CONSTRAINT THAT PUTS ON THE README.
# A command counts as documented when the README contains the bare backticked
# token `/name` somewhere. Anywhere: a table row, a heading, a callout — this
# checks coverage, not a rendering.
#
# The constraint is the *bare* token. A command that appears only inside a
# wider code span — `/specify --supersedes`, say — does not match, and reads
# as undocumented. The failure is loud and in the safe direction (a false
# finding a maintainer resolves by writing the bare token once), but it is a
# rule about how the README may write a command name and it is stated here
# because nothing else states it. It was met on the very next README edit
# after this family shipped, when a table moved to HTML and `<code>/prune</code>`
# stopped matching.
#
# Widening the matcher to recognize a command inside a longer span was
# considered and is the obvious alternative; it risks matching prose that
# merely mentions a flag, and documenting the constraint is the cheaper answer
# to beat.
#
# A failed listing is a finding, never a clean pass: a README that could not
# be read must not report as a README documenting everything.
#
# Bash 3.2 compatible (macOS system bash).

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family readme-command

README="README.md"
SRC_DIR="framework/commands"

if [ ! -f "$README" ]; then
  emit "$README" \
    "the README was not found, so no command could be checked against it" \
    "restore README.md at the repository root"
  exit "$drift"
fi

# The command sources. An empty glob means the directory is gone or empty,
# which is a precondition failure rather than a corpus with nothing to check.
actual="$(for f in "$SRC_DIR"/*.md; do [ -e "$f" ] && basename "$f" .md; done | sort -u)"
if [ -z "$actual" ]; then
  emit "$SRC_DIR" \
    "no command sources were listed, so the README was checked against nothing" \
    "confirm $SRC_DIR exists and holds the command markdown files"
  exit "$drift"
fi

# Maintainer-only commands, from the one list shared with Family 16 and
# gen-help-tables.sh. Empty is a finding: without it every deliberate
# omission — `/audit`'s, today — reads as a README gap.
excl="$(maintainer_only_commands)"
if [ -z "$excl" ]; then
  emit "scripts/maintainer-only-commands.txt" \
    "the maintainer-only command list is missing or empty, so a deliberately withheld command would be reported as a README gap" \
    "restore scripts/maintainer-only-commands.txt with one command base name per line"
  exit "$drift"
fi

expected="$(comm -23 <(printf '%s' "$actual") <(printf '%s' "$excl"))"

# Documented commands: a `/name` mention anywhere in the README, matched on a
# whole token so `/log` never satisfies `/logs` and `/plan` never satisfies
# `/planner`. Deliberately not anchored to a table row — the README documents
# `/consolidate` in a callout beside its row, and a family that demanded a
# specific rendering would dictate prose rather than check coverage.
documented="$(grep -oE '`/[a-z][a-z-]*`' "$README" \
  | tr -d '`/' | sort -u)"

missing=0
while IFS= read -r name; do
  [ -z "$name" ] && continue
  emit "$README §Commands" \
    "/$name is shipped to adopters but appears nowhere in the README — an adopter has no way to learn the command exists" \
    "add a row for /$name to the matching command table in README.md"
  missing=$((missing + 1))
done <<< "$(comm -23 <(printf '%s' "$expected") <(printf '%s' "$documented"))"

# A maintainer-only command that the README *does* document is the mirror
# defect: adopters are told about a command /ductus never installs for them.
while IFS= read -r name; do
  [ -z "$name" ] && continue
  emit "$README §Commands" \
    "/$name is maintainer-only but is documented in the README — adopters are told about a command /ductus never installs" \
    "remove the /$name row, or drop $name from scripts/maintainer-only-commands.txt if it should ship"
done <<< "$(comm -12 <(printf '%s' "$excl") <(printf '%s' "$documented"))"

# Both counts to stderr, so "every shipped command is documented" and "every
# documented row is accurate" do not read alike. This family only claims the
# first.
shipped_count="$(printf '%s\n' "$expected" | grep -c '[^[:space:]]' || true)"
echo "readme-command: $shipped_count adopter-facing command(s) checked for presence in $README;" \
     "$missing undocumented. Presence only — row accuracy and flag-level parity are out of scope." >&2

exit "$drift"
