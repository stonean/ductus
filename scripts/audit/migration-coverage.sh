#!/usr/bin/env bash
# scripts/audit/migration-coverage.sh — Family 10 of /audit.
#
# Verifies framework/migrations.toml is internally consistent with the
# adjacent framework/migrations/*.md procedure files and with current
# framework/ state. Four checks:
#
#   10a no-orphan-procedure-files: every framework/migrations/*.md has a
#       matching TOML entry whose procedure_file points at it.
#   10b no-stale-target-paths: every active registry entry's framework/-
#       prefixed target_paths refers to a path that does NOT exist in
#       current framework/. (Non-framework/ target_paths are adopter-
#       relative and cannot be verified from this repo.)
#   10c no-broken-procedure-references: every TOML entry's procedure_file
#       points at an existing framework/migrations/{id}.md.
#   10d no-earlier-sorting-late-entry: every entry sorts after every entry
#       first present in an earlier commit of framework/migrations.toml.
#       Pre-run Migrations runs only entries after an adopter's
#       [migrations].last_applied, ordered by (introduced_in, then id), so an
#       entry added after another but sorting before it is silently never run
#       for an adopter whose last_applied names the later-sorting one (found
#       by 048's review, 2026-10-29). Entries first present in the same
#       commit are exempt from each other. This check reads history and needs
#       a full-depth checkout; a shallow one is a finding, never a pass.
#
# Requires `python3` (standard on developer macOS / CI Ubuntu) for TOML
# parsing — Python 3.11+ ships with `tomllib`.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family migration-coverage

REGISTRY="framework/migrations.toml"
PROCEDURES_DIR="framework/migrations"

if [ ! -f "$REGISTRY" ]; then
  emit "$REGISTRY" "registry file missing" "create $REGISTRY per spec 027"
  exit 1
fi
if ! command -v python3 >/dev/null 2>&1; then
  emit "$REGISTRY" "python3 not on PATH — cannot parse TOML" "install python3 (3.11+) and re-run"
  exit 1
fi
if ! python3 -c "import tomllib" 2>/dev/null; then
  emit "$REGISTRY" "python3 lacks tomllib (need Python 3.11+)" "upgrade python3 to 3.11+ and re-run"
  exit 1
fi
if ! python3 -c "import tomllib; tomllib.loads(open('$REGISTRY').read())" 2>/dev/null; then
  emit "$REGISTRY" "registry is not valid TOML" "fix the TOML syntax"
  exit 1
fi

# Extract per-entry data via python3 + tomllib.
# Output format, one row per entry: "{id}\t{procedure_file}\t{target_paths_pipe_separated}"
ENTRY_DATA=$(python3 <<'PY'
import tomllib
with open("framework/migrations.toml", "rb") as f:
    data = tomllib.load(f)
for m in data.get("migrations", []):
    id_ = m.get("id", "")
    pf = m.get("procedure_file", "")
    tps = "|".join(m.get("target_paths", []))
    print(f"{id_}\t{pf}\t{tps}")
PY
)

# 10a no-orphan-procedure-files: every framework/migrations/*.md must
# have a matching TOML entry whose procedure_file ends with that filename.
if [ -d "$PROCEDURES_DIR" ]; then
  for f in "$PROCEDURES_DIR"/*.md; do
    [ -e "$f" ] || continue
    if ! grep -qF "procedure_file = \"$f\"" "$REGISTRY"; then
      emit "$f" "orphan procedure file (no [[migrations]] entry references it)" \
        "add a [[migrations]] entry with procedure_file = \"$f\", or delete the file"
    fi
  done
fi

# 10c no-broken-procedure-references: every TOML procedure_file path
# must exist on disk.
while IFS=$'\t' read -r id pf tps; do
  [ -n "$id" ] || continue
  if [ ! -f "$pf" ]; then
    emit "$REGISTRY" "[[migrations]] entry '$id' references missing procedure_file: $pf" \
      "create $pf or correct the procedure_file value"
  fi
done <<< "$ENTRY_DATA"

# 10b no-stale-target-paths: every framework/-prefixed target_path in an
# active entry must NOT exist in current framework/.
# Glob patterns (containing *) are expanded; literal paths use test -e.
while IFS=$'\t' read -r id pf tps; do
  [ -n "$id" ] || continue
  IFS='|' read -r -a paths <<< "$tps"
  for p in "${paths[@]}"; do
    case "$p" in
      framework/*) ;;
      *) continue ;;  # adopter-relative; cannot verify from this repo
    esac
    if [[ "$p" == *"*"* ]]; then
      # Glob pattern — expand and check for any matches.
      # shellcheck disable=SC2206
      matches=( $p )
      for m in "${matches[@]}"; do
        if [ -e "$m" ]; then
          emit "$REGISTRY" "[[migrations]] entry '$id' claims '$p' was removed but '$m' still exists" \
            "delete $m or remove the target_path from the entry"
        fi
      done
    else
      if [ -e "$p" ]; then
        emit "$REGISTRY" "[[migrations]] entry '$id' claims '$p' was removed but it still exists" \
          "delete $p or remove the target_path from the entry"
      fi
    fi
  done
done <<< "$ENTRY_DATA"

# 10d no-earlier-sorting-late-entry: the registry's ordering key is
# (introduced_in, id); an entry that first appeared in a later commit than
# another but sorts before it is never run for an adopter whose last_applied
# names the other. The comparison is over the current registry's entries,
# each placed by the first commit of framework/migrations.toml that held its
# id; an entry not yet committed counts as newest.
#
# Decided historical exception: criterion-label-backfill (0.28.0, e10390d1)
# was added after ductus-rename and runtime-store-path and sorts before both.
# All three first shipped in ductus-v0.28.0, but before spec 061 adopters
# read main, so an adopter could have skipped it in between. The order cannot
# change now, since last_applied values already name these ids, so the two
# pairs are exempt rather than fixed.
if [ "$(git rev-parse --is-shallow-repository 2>/dev/null)" = "true" ]; then
  emit "$REGISTRY" "10d cannot run: the checkout is shallow, so the registry history is unreadable" \
    "fetch full history (git fetch --unshallow; in CI, actions/checkout fetch-depth: 0)"
else
  ORDER_OUT=$(REGISTRY="$REGISTRY" python3 <<'PY'
import os, re, subprocess, sys, tomllib

REGISTRY = os.environ["REGISTRY"]
EXEMPT = {
    ("criterion-label-backfill", "ductus-rename"),
    ("criterion-label-backfill", "runtime-store-path"),
}

def key(entry):
    m = re.search(r"(\d+)\.(\d+)\.(\d+)", entry.get("introduced_in", ""))
    return (tuple(int(x) for x in m.groups()) if m else (-1,), entry.get("id", ""))

def entries_of(text):
    return {m["id"]: m for m in tomllib.loads(text).get("migrations", []) if "id" in m}

def vcs(*args):
    return subprocess.run(["git", *args], capture_output=True, text=True)

log = vcs("log", "--reverse", "--format=%H", "--", REGISTRY)
if log.returncode != 0:
    print("FINDING\tcannot read the registry history: " + log.stderr.strip())
    sys.exit(0)
commits = log.stdout.split()
first_seen = {}
unreadable = 0
for idx, sha in enumerate(commits):
    shown = vcs("show", sha + ":" + REGISTRY)
    if shown.returncode != 0:
        continue  # the commit deleted the file
    try:
        snapshot = entries_of(shown.stdout)
    except tomllib.TOMLDecodeError:
        unreadable += 1
        continue
    for id_ in snapshot:
        first_seen.setdefault(id_, idx)

with open(REGISTRY, "rb") as f:
    current = entries_of(f.read().decode())
newest = len(commits)
placed = {i: first_seen.get(i, newest) for i in current}
compared = 0
for late, late_at in placed.items():
    for early, early_at in placed.items():
        if early_at >= late_at:
            continue
        compared += 1
        if key(current[late]) < key(current[early]) and (late, early) not in EXEMPT:
            print("FINDING\t[[migrations]] entry " + repr(late) + " (introduced_in " + current[late].get("introduced_in", "?") + ") was added after " + repr(early) + " (introduced_in " + current[early].get("introduced_in", "?") + ") but sorts before it, so an adopter whose last_applied names " + repr(early) + " never runs it")
print("COVERAGE\t" + str(len(current)) + " entries, " + str(compared) + " earlier-added pairs compared, " + str(len(commits)) + " commits read, " + str(unreadable) + " unreadable")
PY
)
  if [ -z "$ORDER_OUT" ]; then
    emit "$REGISTRY" "10d produced no output: the history comparison could not run" "re-run with python3 3.11+ and a full-depth checkout"
  fi
  while IFS=$'\t' read -r kind msg; do
    case "$kind" in
      FINDING) emit "$REGISTRY" "$msg" "re-id the entry so it sorts after the earlier one, or record a decided exception in 10d with its reason" ;;
      COVERAGE) echo "migration-coverage | 10d examined: $msg" >&2 ;;
    esac
  done <<< "$ORDER_OUT"
fi

# TODO: parse archived CHANGELOG.md entries for the same no-stale-target-paths check.
# Deferred until the first sunset commit so the archive format is established
# by example. Captured in spec 027 as a future Family 10 enhancement.

exit "$drift"
