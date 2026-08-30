#!/usr/bin/env bash
# scripts/audit/broken-relative-links.sh — Family 26 of /audit.
#
# A relative markdown link whose target does not exist. Nothing else catches
# these: markdownlint's MD051 validates heading *fragments* and says nothing
# about whether the file exists, and `check-orphaned-references` scopes to
# adopter-owned referrers and ductus-managed path prefixes, so a spec linking
# a sibling spec at the wrong depth is outside both.
#
# The dominant class is a depth error in a scenario file. A scenario lives at
# `specs/NNN-foo/scenarios/bar.md`, so a sibling spec is `../../NNN-other/` and
# the constitution is `../../../framework/`; writing one `../` too few produces
# a link that renders fine, reviews fine, and resolves to nothing. The second
# class is a link to a file a later spec deleted.
#
# THE CHECK IS A PRIMITIVE; THIS IS THE ENTRY POINT — the shape Family 30
# established. The scanning, the code-span stripping, the shape filter, and the
# lexical resolution live in `check-corpus-links`; this script resolves the
# runtime, calls it, and renders the result through `emit`.
#
# It was not always so. The family carried its own python implementation of the
# same check, and when `check-corpus-links` shipped for adopters (spec 022) the
# two diverged immediately: the primitive was fixed to resolve a root-absolute
# target against the repository root, and this family still resolved it against
# the *filesystem* root, where `os.path.join(here, '/specs/x.md')` discards
# `here` entirely. Two implementations of one rule, one of them wrong in the
# reassuring direction. Delegating is what makes that impossible rather than
# merely fixed (spec 026 scenario `link-check-consolidation`).
#
# SCOPE — WIDER THAN THE PRIMITIVE'S DEFAULT, DELIBERATELY. This family passes
# `--scope repository`, so its subject is every *tracked* `.md` file in the
# repository: framework sources, scripts, this suite's own documentation. The
# primitive's default subject is the spec corpus, which is what an adopter's
# pre-commit hook checks and all an adopter has business being told about.
# One resolver, two subjects, and the difference is an argument rather than a
# second copy of the rule.
#
# **The count is the guard.** Consolidating this family into the primitive
# could have silently narrowed its subject from the repository to the spec
# corpus — a smaller subject nobody stated, which is `QUAL-CLAIM-001` in its
# purest form. It examined 457 files before the consolidation and examines 457
# after; the count goes to stderr on every run so a future narrowing is visible
# rather than inferred.
#
# Excluded by construction, and **counted** rather than silently dropped:
# `.claude/` (generated command copies, whose links are broken by construction
# because the generator changes their depth without rewriting them),
# `framework/templates/project/` (links resolve in the adopter's repo root),
# `runtime/tests/` (fixtures and goldens mirror arbitrary adopter content), and
# the spec root's `templates/`.
#
# A scan that examines no files is a finding, never a pass.
#
# Bash 3.2 compatible (macOS system bash). The runtime emits JSON; python3
# renders tab-separated records and the shell renders those through `emit`,
# so the pipe-separated finding shape never appears in the python — the
# convention ./README.md records for the families that compute in python.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family broken-link

ductus_bin="$(ductus_bin)"

# An unreachable runtime is a finding, never a silent pass. A family that
# cannot run must not exit 0 wearing the costume of one that passed.
if [ -z "$ductus_bin" ]; then
  emit "(precondition)" \
    "ductus runtime not reachable — the link check could not run" \
    "run /ductus to acquire the runtime, or build it with cargo build --release in runtime/"
  exit "$drift"
fi

if ! payload="$("$ductus_bin" check-corpus-links --scope repository 2>/dev/null)"; then
  # Exit 1 is the primitive's *finding* signal as well as its failure signal —
  # it gates on broken links — so a non-zero exit is not on its own an error.
  # Only unparseable output is. The payload is re-read below either way.
  :
fi

records="$(
  printf '%s' "$payload" | python3 -c '
import json, sys

try:
    data = json.load(sys.stdin)
except ValueError:
    print("\t".join(["(precondition)",
                     "check-corpus-links produced no parseable output, so no link was checked",
                     "run `ductus check-corpus-links --scope repository` directly to see the error"]))
    raise SystemExit(0)

for link in data.get("broken", []):
    print("\t".join(["%s:%s" % (link["path"], link["line"]),
                     "relative link `%s` resolves to nothing" % link["target"],
                     link["guidance"]]))

for skip in data.get("skipped", []):
    print("\t".join([skip["path"],
                     "could not be examined (%s), so its links were never checked" % skip["reason"],
                     "make the file readable and re-run; a target this family cannot open is not one it found clean"]))

guidance = data.get("guidance", "")
if guidance:
    print("\t".join(["repository", guidance,
                     "run from a git checkout with tracked .md files"]))

sys.stderr.write(
    "broken-link: %d markdown file(s) examined; %d excluded by construction "
    "(generated copies, project templates, test fixtures, spec templates); "
    "%d link(s) skipped as documentation shapes\n"
    % (len(data.get("examined", [])), data.get("excluded-by-construction", 0),
       data.get("shapes-skipped", 0)))
'
)" || {
  emit "(precondition)" \
    "could not parse check-corpus-links output" \
    "run $ductus_bin check-corpus-links --scope repository and inspect the JSON"
  exit "$drift"
}

while IFS="$(printf '\t')" read -r where message fix; do
  [ -n "$where" ] || continue
  emit "$where" "$message" "$fix"
done <<EOF
$records
EOF

exit "$drift"
