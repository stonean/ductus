#!/usr/bin/env bash
# scripts/audit/migration-coverage.sh — Family 10 of /audit.
#
# Verifies framework/migrations.toml is internally consistent with the
# adjacent framework/migrations/*.md procedure files, with CHANGELOG.md's
# archived entries and with current framework/ state. Five checks:
#
#   10a no-orphan-procedure-files: every framework/migrations/*.md has a
#       matching TOML entry whose procedure_file points at it.
#   10b no-stale-target-paths: every framework/-prefixed target_path of an
#       active registry entry, and of an archived entry in CHANGELOG.md's
#       Archived migrations (its **Target paths:** line), refers to a path
#       that does NOT exist in current framework/. (Non-framework/ target_paths
#       are adopter-relative and cannot be verified from this repo.) An
#       archived entry with no **Target paths:** line is a finding, since 10b
#       could not check it.
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
#   10e no-overdue-sunset: no active entry's sunset_after is at or below the
#       release the repository's `version` file names. Pre-run Migrations
#       already skips such an entry, so it belongs archived: removed from the
#       registry, its procedure appended to CHANGELOG.md with its target
#       paths, and its procedure file deleted, in one commit (spec 027 AC13,
#       AC14). Nothing else prompts that archival, which five entries missed
#       for up to 49 minor releases.
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
if ! python3 -c "import sys, tomllib; tomllib.loads(open(sys.argv[1]).read())" "$REGISTRY" 2>/dev/null; then
  emit "$REGISTRY" "registry is not valid TOML" "fix the TOML syntax"
  exit 1
fi

# Extract per-entry data via python3 + tomllib.
# Output format, one row per entry: "{id}<US>{procedure_file}<US>{target_paths_pipe_separated}",
# <US> the unit separator (0x1f). A tab is IFS whitespace, so `read` collapses
# consecutive tabs: an entry with no procedure_file would read its target_paths
# as the procedure_file and 10b would never examine it. The separator is not
# whitespace, so an empty field stays a field.
ENTRY_DATA=$(python3 <<'PY'
import tomllib
with open("framework/migrations.toml", "rb") as f:
    data = tomllib.load(f)
for m in data.get("migrations", []):
    id_ = m.get("id", "")
    pf = m.get("procedure_file", "")
    tps = "|".join(m.get("target_paths", []))
    print(f"{id_}\x1f{pf}\x1f{tps}")
PY
)

# 10a no-orphan-procedure-files: every framework/migrations/*.md must
# have a matching TOML entry whose procedure_file ends with that filename.
if [ -d "$PROCEDURES_DIR" ]; then
  for f in "$PROCEDURES_DIR"/*.md; do
    # A glob match is a path this family did not write; one resolving outside
    # the repository is reported and not probed further (026 plan, BE-INPUT-004).
    contained "$ROOT" "$f"
    case $? in
      1)
        emit "$f" "procedure file resolves outside the repository, so it was not examined" \
          "replace the symbolic link with the file, or remove it"
        continue
        ;;
      2)
        containment_unchecked "$f"
        exit "$drift"
        ;;
    esac
    [ -e "$f" ] || continue
    if ! grep -qF "procedure_file = \"$f\"" "$REGISTRY"; then
      emit "$f" "orphan procedure file (no [[migrations]] entry references it)" \
        "add a [[migrations]] entry with procedure_file = \"$f\", or delete the file"
    fi
  done
fi

# 10c no-broken-procedure-references: every TOML procedure_file path
# must exist on disk.
while IFS=$'\037' read -r id pf tps; do
  [ -n "$id" ] || continue
  # The registry field is committed text, contained before it is probed.
  contained "$ROOT" "$pf"
  case $? in
    1)
      emit "$REGISTRY" "[[migrations]] entry '$id' procedure_file resolves outside the repository and was not probed: $pf" \
        "correct the procedure_file value to a file inside the repository"
      continue
      ;;
    2)
      containment_unchecked "$pf"
      exit "$drift"
      ;;
  esac
  if [ ! -f "$pf" ]; then
    emit "$REGISTRY" "[[migrations]] entry '$id' references missing procedure_file: $pf" \
      "create $pf or correct the procedure_file value"
  fi
done <<< "$ENTRY_DATA"

# 10b no-stale-target-paths: every framework/-prefixed target_path of an
# active or archived entry must NOT exist in current framework/.
# Glob patterns (containing *) are expanded; literal paths use test -e.
# check_target_paths LOCATION LABEL TPS — LOCATION is the file a finding
# names, LABEL how it names the entry, TPS the paths joined by `|`.
check_target_paths() {
  local location="$1" label="$2" tps="$3" p m
  local -a paths matches
  IFS='|' read -r -a paths <<< "$tps"
  for p in "${paths[@]+"${paths[@]}"}"; do
    case "$p" in
      framework/*) ;;
      *) continue ;;  # adopter-relative; cannot verify from this repo
    esac
    # The entry is committed text: the pattern is contained before it is
    # expanded, and each match after.
    contained "$ROOT" "$p"
    case $? in
      1)
        emit "$location" "$label target_path resolves outside the repository and was not probed: $p" \
          "correct the target_path to a path inside the repository"
        continue
        ;;
      2)
        containment_unchecked "$p"
        exit "$drift"
        ;;
    esac
    if [[ "$p" == *"*"* ]]; then
      # Glob pattern — expand and check for any matches.
      # shellcheck disable=SC2206
      matches=( $p )
      for m in "${matches[@]}"; do
        contained "$ROOT" "$m"
        case $? in
          1)
            emit "$location" "$label target_path '$p' matched '$m', which resolves outside the repository and was not probed" \
              "remove the symbolic link '$m', or narrow the target_path"
            continue
            ;;
          2)
            containment_unchecked "$m"
            exit "$drift"
            ;;
        esac
        if [ -e "$m" ]; then
          emit "$location" "$label claims '$p' was removed but '$m' still exists" \
            "delete $m or remove the target_path from the entry"
        fi
      done
    else
      if [ -e "$p" ]; then
        emit "$location" "$label claims '$p' was removed but it still exists" \
          "delete $p or remove the target_path from the entry"
      fi
    fi
  done
}

while IFS=$'\037' read -r id pf tps; do
  [ -n "$id" ] || continue
  check_target_paths "$REGISTRY" "[[migrations]] entry '$id'" "$tps"
done <<< "$ENTRY_DATA"

# 10b, archived half: each entry under CHANGELOG.md's `## Archived migrations`
# opens with a `### {id} (…)` heading and keeps its target_paths on a
# `**Target paths:**` line of backticked paths (spec 027 AC18). A repository
# with no CHANGELOG.md has archived nothing; one whose CHANGELOG.md holds no
# `## Archived migrations` section is a finding, since a renamed heading would
# otherwise leave every archived entry unread.
#
# archived_rows FILE — one row per archived entry, unit-separated: ROW, the id
# and its paths joined by `|`; MISSING and the id, for an entry with no
# **Target paths:** line or one naming no backticked path; NOSECTION when the
# section is absent; then COVERAGE and the count.
# A function, since bash 3.2 misparses a heredoc holding backticks inside $().
archived_rows() {
  ARCHIVE="$1" python3 <<'PY'
import os, re
text = open(os.environ["ARCHIVE"], encoding="utf-8").read()
section = re.search(r"^## Archived migrations[ \t]*\n(.*?)(?=^## |\Z)", text, re.S | re.M)
if section is None:
    print("NOSECTION\x1f\x1f")
count = 0
for block in re.split(r"(?m)^### ", section.group(1) if section else "")[1:]:
    head, _, rest = block.partition("\n")
    id_ = head.split(" ", 1)[0].strip()
    count += 1
    line = re.search(r"(?m)^\*\*Target paths:\*\*(.*)$", rest)
    paths = re.findall(r"`([^`]+)`", line.group(1)) if line else []
    if not paths:
        print("MISSING\x1f" + id_ + "\x1f")
    else:
        print("ROW\x1f" + id_ + "\x1f" + "|".join(paths))
print("COVERAGE\x1f" + str(count) + "\x1f")
PY
}

ARCHIVE="CHANGELOG.md"
if [ -f "$ARCHIVE" ]; then
  contained "$ROOT" "$ARCHIVE"
  case $? in
    1)
      emit "$ARCHIVE" "the archive resolves outside the repository, so its entries were not examined" \
        "replace the symbolic link with the file"
      ;;
    2)
      containment_unchecked "$ARCHIVE"
      exit "$drift"
      ;;
    *)
      ARCHIVED_DATA=$(archived_rows "$ARCHIVE")
      if [ -z "$ARCHIVED_DATA" ]; then
        emit "$ARCHIVE" "10b's archived half produced no output: the archive could not be read" "re-run with python3 3.11+"
      fi
      while IFS=$'\037' read -r kind id tps; do
        case "$kind" in
          ROW) check_target_paths "$ARCHIVE" "archived entry '$id'" "$tps" ;;
          MISSING) emit "$ARCHIVE" "archived entry '$id' has no **Target paths:** line naming a backticked path, so 10b could not check it" \
            "add a **Target paths:** line listing the entry's target_paths, each in backticks" ;;
          NOSECTION) emit "$ARCHIVE" "$ARCHIVE holds no '## Archived migrations' section, so 10b read no archived entry" \
            "restore the section heading the archived entries sit under" ;;
          COVERAGE) echo "migration-coverage | 10b examined: $id archived entries in $ARCHIVE" >&2 ;;
        esac
      done <<< "$ARCHIVED_DATA"
      ;;
  esac
fi

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

# 10e no-overdue-sunset: an active entry whose sunset_after is at or below the
# release `version` names is one Pre-run Migrations already skips ("the current
# ductus release version is less than sunset_after" keeps it), so it is owed
# its archival.
if [ ! -f version ]; then
  emit "version" "10e cannot run: no version file names the current release" \
    "restore the repository's version file"
else
  SUNSET_OUT=$(REGISTRY="$REGISTRY" python3 <<'PY'
import os, re, tomllib

def release(text):
    m = re.fullmatch(r"\s*(\d+)\.(\d+)\.(\d+)\s*", text or "")
    return tuple(int(x) for x in m.groups()) if m else None

raw = open("version", encoding="utf-8").read().strip()
current = release(raw)
if current is None:
    print("FINDING\tversion holds " + repr(raw) + ", not a MAJOR.MINOR.PATCH release, so 10e cannot compare")
else:
    with open(os.environ["REGISTRY"], "rb") as f:
        entries = tomllib.load(f).get("migrations", [])
    compared = 0
    for m in entries:
        if "sunset_after" not in m:
            continue
        compared += 1
        sunset = release(str(m["sunset_after"]))
        if sunset is None:
            print("FINDING\t[[migrations]] entry " + repr(m.get("id", "")) + " has sunset_after " + repr(m["sunset_after"]) + ", not a MAJOR.MINOR.PATCH release")
        elif current >= sunset:
            print("FINDING\t[[migrations]] entry " + repr(m.get("id", "")) + " is past its sunset_after " + str(m["sunset_after"]) + " (version " + raw + "), so Pre-run Migrations already skips it")
    print("COVERAGE\t" + str(compared) + " entries with a sunset_after compared with version " + raw)
PY
)
  if [ -z "$SUNSET_OUT" ]; then
    emit "$REGISTRY" "10e produced no output: the sunset comparison could not run" "re-run with python3 3.11+"
  fi
  while IFS=$'\t' read -r kind msg; do
    case "$kind" in
      FINDING) emit "$REGISTRY" "$msg" "archive it: remove the entry, append its procedure and **Target paths:** to CHANGELOG.md's Archived migrations, and delete its procedure file, in one commit (spec 027 AC13, AC14)" ;;
      COVERAGE) echo "migration-coverage | 10e examined: $msg" >&2 ;;
    esac
  done <<< "$SUNSET_OUT"
fi

exit "$drift"
