#!/usr/bin/env bash
# scripts/audit/command-flag-hint-parity.sh — Family 30 of /audit.
#
# Every flag a command's Flags table documents also appears in that command's
# `argument-hint:` frontmatter. `argument-hint` is the surface a host renders
# when it offers the command, so a flag absent from it is a flag the operator
# is never shown — the defect an adopter hit when `review.md`'s table listed
# eight flags and its hint named three, leaving `--since` documented, plumbed
# through `compute-review-scope`, and invisible.
#
# THE CHECK IS A PRIMITIVE; THIS IS THE ENTRY POINT. The parsing lives in the
# runtime (`check-command-flags`), which reuses the tested frontmatter-fence
# and fenced-block scanners the derive primitives already share. Re-deriving
# them in awk here would be a second implementation of markdown structure
# parsing — the thing §runtime-boundary principle 3 names, and the thing that
# left Family 7 dead on macOS until a portability trap was found by hand.
# This script exists because the /audit contract in ./README.md is a shell
# one: source lib.sh, render findings through `emit`, exit "$drift", stay
# directly invocable. Entry point in shell, logic in the runtime.
#
# SCOPE. The primitive reads `framework/commands/*.md` — the sources, not the
# generated copies under a host's commands directory, which carry whatever the
# source carries and which an adopter cannot repair anyway. Only a `Flags`
# section's table rows count, and only each row's first cell, so a command
# documenting a flag in prose (`implement.md`'s `--auto`) is examined and
# contributes nothing. A clean exit therefore means "every tabled flag is
# surfaced", never "every documented flag is surfaced"; the counts go to
# stderr so the two do not read alike.
#
# MEASURED. 6 findings at the commit before `review.md`'s hint was corrected
# (`--security`, `--simplicity`, `--quality`, `--since`, `--waive`, and
# `--reason` — the waiver row names both halves of the pair); 0 after.
#
# Bash 3.2 compatible (macOS system bash). The runtime emits JSON; python3
# renders tab-separated records and the shell renders those through `emit`,
# so the pipe-separated finding shape never appears in the python — the
# convention ./README.md records for the families that compute in python.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family command-flag-hint

ductus_bin=""
if [ -x .ductus/bin/ductus ]; then
  ductus_bin=".ductus/bin/ductus"
elif [ -x runtime/target/release/ductus ]; then
  ductus_bin="runtime/target/release/ductus"
elif command -v ductus > /dev/null 2>&1; then
  ductus_bin="ductus"
fi

# An unreachable runtime is a finding, never a silent pass. A family that
# cannot run must not exit 0 wearing the costume of one that passed.
if [ -z "$ductus_bin" ]; then
  emit "(precondition)" \
    "ductus runtime not reachable — the flag/hint check could not run" \
    "run /ductus to acquire the runtime, or build it with cargo build --release in runtime/"
  exit "$drift"
fi

if ! payload="$("$ductus_bin" check-command-flags 2>/dev/null)"; then
  emit "(precondition)" \
    "ductus check-command-flags failed to execute" \
    "run $ductus_bin check-command-flags directly to see the error"
  exit "$drift"
fi

records="$(
  printf '%s' "$payload" | python3 -c '
import json, sys

data = json.load(sys.stdin)

# guidance is set when the run examined command files and found no Flags
# table at all: an extraction failure, not a clean corpus.
guidance = data.get("guidance", "")
if guidance:
    print("\t".join([data.get("commands-dir", "framework/commands"), guidance,
                     "check the Flags-section extraction in check-command-flags"]))

for skip in data.get("skipped", []):
    print("\t".join([skip["path"], skip["reason"],
                     "make the file readable and re-run"]))

for f in data.get("findings", []):
    flag = f.get("flag", "")
    fix = ("add %s to the argument-hint: frontmatter field" % flag) if flag \
        else "add an argument-hint: frontmatter field naming each flag in the table"
    print("\t".join([f["command"], f["reason"], fix]))

counts = (len(data.get("examined", [])), len(data.get("with-flags-table", [])))
sys.stderr.write(
    "command-flag-hint: examined %d command file(s); %d with a flags table\n" % counts)
'
)" || {
  emit "(precondition)" \
    "could not parse check-command-flags output" \
    "run $ductus_bin check-command-flags and inspect the JSON"
  exit "$drift"
}

while IFS="$(printf '\t')" read -r location message fix; do
  [ -n "$location" ] || continue
  emit "$location" "$message" "$fix"
done <<EOF
$records
EOF

exit "$drift"
