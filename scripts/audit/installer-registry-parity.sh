#!/usr/bin/env bash
# scripts/audit/installer-registry-parity.sh — Family 14 of /audit.
#
# Verifies the one-line installer (install.sh) and the agent registry in
# framework/bootstrap/ductus.md agree on which agents exist and where each
# one's `ductus` bootstrap is placed.
#
# install.sh hard-codes one `case` arm per agent (`<key>) ... dest="..."`).
# The registry's §Derived values table derives each agent's `ductus` install
# path from its row by layout:
#
#   claude-style → {config_dir}/commands/ductus.md
#   antigravity  → {config_dir}/skills/ductus/SKILL.md
#   opencode     → {config_dir}/command/ductus.md
#
# The check enforces per-key parity in three directions, then one shared
# constant:
#
#   1. Every registry agent has a matching install.sh `case` arm whose
#      dest equals the registry-derived path. (Catches: an agent added to
#      the registry but not the installer — the gap the "single registry
#      row plus a permission file" claim would otherwise hide.)
#   2. Every install.sh `case` arm names a registry agent and installs to
#      that agent's derived path. (Catches: a stale or mis-mapped arm.)
#   3. Every settings file install.sh pre-seeds matches that agent's
#      registry settings_template, compared as JSON. (Catches: the seeded
#      permission copy silently drifting from the registry it duplicates.)
#   4. install.sh's REF_FLOOR equals ductus.md's {ref-floor}, the first
#      release whose bootstrap honors --ref (spec 061). (Catches: one entry
#      point accepting a tag the other refuses.)
#
# Directions 1-2 are pure text extraction — no jq, no associative arrays
# (macOS bash 3.2). Direction 3 uses python3 (already a ductus bootstrap
# dependency, and used by sibling audit scripts) because the three permission
# formats (one per layout, plus opencode's action map) make an
# order-insensitive JSON compare the only reliable check.
# This is the audit check spec 003's curl-sh-installer scenario calls for,
# resolving its installer<->registry parity open question per the
# "never depend on human diligence" design principle.

set -uo pipefail
# shellcheck source-path=SCRIPTDIR source=lib.sh
. "$(dirname "${BASH_SOURCE[0]}")/lib.sh" || exit 1
audit_family installer-registry-parity

DUCTUS="framework/bootstrap/ductus.md"
INSTALLER="install.sh"

if [ ! -f "$DUCTUS" ]; then
  emit "$DUCTUS" "agent registry source missing" "restore $DUCTUS"
  exit 1
fi
if [ ! -f "$INSTALLER" ]; then
  emit "$INSTALLER" "installer missing" "restore $INSTALLER or remove this audit family"
  exit 1
fi

# Derive an agent's `ductus` install path from its config_dir + layout,
# mirroring the §Derived values "ductus install path" row.
derive_path() {
  case "$2" in
    claude-style) printf '%s/commands/ductus.md\n' "$1" ;;
    antigravity)  printf '%s/skills/ductus/SKILL.md\n' "$1" ;;
    opencode)     printf '%s/command/ductus.md\n' "$1" ;;
    pi)           printf '%s/prompts/ductus.md\n' "$1" ;;
    *)            printf '\n' ;;  # unknown layout — signalled by empty result
  esac
}

# registry_map / installer_map: newline-delimited "key<TAB>path" records.
# Extract the agent-registry table rows: scoped to the `## Agent Registry`
# section, up to the next heading. Columns (split on `|`): 2=key,
# 4=config_dir, 5=layout. Backticks and spaces are stripped; the header row
# (key == "key") and the `---` separator are skipped. No registry cell
# contains a literal `|`, so the split is safe.
registry_map="$(
  awk '
    /^## Agent Registry/ { inseg = 1; next }
    inseg && /^#/        { inseg = 0 }
    inseg && /^\|/ {
      n = split($0, c, "|")
      key = c[2]; cd = c[4]; lay = c[5]
      gsub(/[`[:space:]]/, "", key)
      gsub(/[`[:space:]]/, "", cd)
      gsub(/[`[:space:]]/, "", lay)
      if (key == "" || key == "key" || key ~ /^-+$/) next
      print key "\t" cd "\t" lay
    }
  ' "$DUCTUS"
)"

# Extract install.sh's case-arm key -> dest mapping. Track the current
# arm's primary key (first token before `|`), and bind it to the first
# `dest="..."` that follows. The `*)` default arm has no dest and is skipped.
installer_map="$(
  awk '
    /case[[:space:]]+"\$agent"[[:space:]]+in/ { incase = 1; next }
    /^esac/ { incase = 0 }
    incase && /^[[:space:]]*[A-Za-z*][A-Za-z0-9_| ]*\)/ {
      line = $0
      sub(/\).*/, "", line)
      gsub(/[[:space:]]/, "", line)
      split(line, toks, "|")
      curkey = toks[1]
      next
    }
    incase && curkey != "" && curkey != "*" && /dest="/ {
      d = $0; sub(/.*dest="/, "", d); sub(/".*/, "", d)
      print curkey "\t" d
      curkey = ""
    }
  ' "$INSTALLER"
)"

lookup() { awk -F'\t' -v k="$2" '$1 == k { print $NF; exit }' <<EOF
$1
EOF
}

# Direction 1: every registry agent has a matching installer arm.
while IFS="$(printf '\t')" read -r key cd lay; do
  [ -n "$key" ] || continue
  want="$(derive_path "$cd" "$lay")"
  if [ -z "$want" ]; then
    emit "$DUCTUS (agent $key)" "unrecognized layout '$lay' — cannot derive install path" \
      "add a '$lay' branch to derive_path() in scripts/audit/installer-registry-parity.sh and a matching install.sh case arm"
    continue
  fi
  got="$(lookup "$installer_map" "$key")"
  if [ -z "$got" ]; then
    emit "$INSTALLER" "registry agent '$key' has no install.sh case arm" \
      "add a '$key)' arm to install.sh placing the bootstrap at $want"
  elif [ "$got" != "$want" ]; then
    emit "$INSTALLER (agent $key)" "installs to '$got' but the registry derives '$want'" \
      "fix the '$key)' dest in install.sh to match the registry-derived path"
  fi
done <<EOF
$registry_map
EOF

# Direction 2: every installer arm names a known registry agent.
while IFS="$(printf '\t')" read -r key dest; do
  [ -n "$key" ] || continue
  if [ -z "$(lookup "$registry_map" "$key")" ]; then
    emit "$INSTALLER (agent $key)" "installs to '$dest' but '$key' is not in the agent registry" \
      "add '$key' to the §Agent Registry table in $DUCTUS, or remove its arm from install.sh"
  fi
done <<EOF
$installer_map
EOF

# Direction 3: settings-template parity. install.sh pre-seeds each agent's
# permission file (so the first /ductus run does not prompt for its bootstrap
# shell commands) by hard-coding a copy of that agent's registry settings_template.
# That duplicate must not silently drift. The three permission formats (claude
# Bash()/Read(), auggie toolPermissions/regex, antigravity command()) make a text
# diff unreliable — and opencode adds a fourth, its `permission` action map — so
# this pass uses python3 for an order-insensitive JSON compare
# of each install.sh seed against its §Agent Registry settings_template.
seed_drift="$(
python3 - "$DUCTUS" "$INSTALLER" <<'PY'
import json, re, sys
ductus, installer = sys.argv[1], sys.argv[2]

# Registry: agent key -> settings_template JSON (column 5 of the §Agent Registry
# table). The cell is backtick-wrapped JSON containing no literal '|'.
rows = {}
in_reg = False
for line in open(ductus):
    if line.startswith("## Agent Registry"):
        in_reg = True
        continue
    if in_reg and line.startswith("#"):
        break
    if in_reg and line.lstrip().startswith("|"):
        c = line.split("|")
        if len(c) < 7:
            continue
        key = c[1].strip().strip("`").strip()
        if key in ("", "key") or set(key) == {"-"}:
            continue
        rows[key] = c[5].strip().strip("`").strip()

# Installer: settings-file path -> seeded JSON heredoc body.
text = open(installer).read()
PATH2KEY = {
    ".claude/settings.local.json": "claude",
    ".augment/settings.local.json": "auggie",
    ".agents/settings.json": "antigravity",
    "opencode.json": "opencode",
}
seeds = {}
for m in re.finditer(r"cat > (\S+) <<'JSON'\n(.*?)\nJSON", text, re.S):
    path, body = m.group(1), m.group(2)
    key = PATH2KEY.get(path)
    if key:
        seeds[key] = (path, body)

def norm(x):
    if isinstance(x, dict):
        return {k: norm(v) for k, v in x.items()}
    if isinstance(x, list):
        return sorted((norm(i) for i in x), key=lambda e: json.dumps(e, sort_keys=True))
    return x

# One tab-separated `location<TAB>message<TAB>fix` record per finding. The
# shell renders them through `emit` below, so the finding line shape stays
# defined once in lib.sh (same hand-off as migration-coverage.sh).
def finding(loc, msg, fix):
    print(f"{loc}\t{msg}\t{fix}")

for key, (path, body) in seeds.items():
    if key not in rows:
        finding(f"{installer} ({path})",
                f"seeds settings for '{key}' but '{key}' is not in the agent registry",
                f"add '{key}' to the §Agent Registry table in {ductus}, or remove its seed from {installer}")
        continue
    try:
        reg = norm(json.loads(rows[key]))
    except json.JSONDecodeError as e:
        finding(f"{ductus} (agent {key})", f"settings_template is not valid JSON: {e}",
                "repair the registry settings_template cell")
        continue
    try:
        seed = norm(json.loads(body))
    except json.JSONDecodeError as e:
        finding(f"{installer} ({path})", f"seeded settings JSON is malformed: {e}",
                "repair the install.sh heredoc")
        continue
    if reg != seed:
        finding(f"{installer} ({path})",
                f"settings seed for '{key}' drifts from the registry settings_template",
                f"re-sync the '{path}' heredoc in {installer} with the '{key}' row in {ductus} §Agent Registry")
PY
)"
if [ -n "$seed_drift" ]; then
  while IFS="$(printf '\t')" read -r loc msg fix; do
    [ -z "$loc" ] && continue
    emit "$loc" "$msg" "$fix"
  done <<< "$seed_drift"
fi

# Direction 3b: Pi's parity is the *absence* of a seed. Pi has no
# permission-gating settings (spec 064 §Verified Pi Layout), so the
# install.sh pi arm must write no settings file, and the registry row's
# settings_template must stay the empty object. Asserted directly rather
# than through the python seed-compare above — an empty `{}` would compare
# equal to every non-empty `{}`-drift seed, which is the failure mode this
# family exists to catch — and stated here so the check's bound is visible
# in the family, per the check-that-cannot-run rule.
pi_seed_findings=0
if grep -nE "^[[:space:]]*pi\)" "$INSTALLER" >/dev/null; then
  # The pi arm — from `pi)` to its `;;` — must not contain a `cat > … <<'JSON'`
  # settings write.
  if awk '
    /^[[:space:]]*pi\)/ { inpi = 1 }
    inpi && /cat > .*settings/ { print; exit }
    inpi && /;;/ { exit }
  ' "$INSTALLER" | grep -q .; then
    emit "$INSTALLER (pi arm)" "pi arm seeds a settings file, but Pi has no permission-gating settings to seed" \
      "remove the heredoc from the pi arm of $INSTALLER"
    pi_seed_findings=1
  fi
else
  emit "$INSTALLER" "no install.sh pi arm" "add a 'pi)' arm to install.sh"
  pi_seed_findings=1
fi
# The registry row's settings_template for pi must be the empty object.
if ! grep -qE '^\| `pi` .*\| `\{\}` \|' "$DUCTUS"; then
  emit "$DUCTUS (agent pi)" "pi settings_template is not the empty object — Pi has no permission-gating settings to seed" \
    "set the pi settings_template cell in §Agent Registry to {} (with backticks)"
  pi_seed_findings=1
fi
[ "$pi_seed_findings" -eq 0 ] || drift=1

# Direction 4: the release floor. Both entry points refuse a tag older than
# the first release whose bootstrap honors --ref (spec 061), and each carries
# that version: install.sh as REF_FLOOR="X.Y.Z", ductus.md as the `{ref-floor}`
# row of §Derived paths. Once that release exists the value never changes, but
# until it is cut the two copies can drift, and a mismatch lets one entry point
# accept a tag the other refuses. A copy this check cannot read is a finding,
# not a pass: a floor it could not find is a floor it did not compare.
installer_floor="$(sed -n 's/^REF_FLOOR="\([0-9][0-9.]*\)"$/\1/p' "$INSTALLER" | head -1)"
bootstrap_floor="$(sed -n 's/^| `{ref-floor}` | `\([0-9][0-9.]*\)`.*/\1/p' "$DUCTUS" | head -1)"
if [ -z "$installer_floor" ]; then
  emit "$INSTALLER" "no REF_FLOOR=\"X.Y.Z\" line — the release floor was not compared" \
    "restore install.sh's REF_FLOOR constant (spec 061)"
elif [ -z "$bootstrap_floor" ]; then
  emit "$DUCTUS" "no \`{ref-floor}\` row in §Derived paths — the release floor was not compared" \
    "restore the {ref-floor} row in $DUCTUS §Derived paths (spec 061)"
elif [ "$installer_floor" != "$bootstrap_floor" ]; then
  emit "$INSTALLER" "REF_FLOOR is $installer_floor but $DUCTUS §Derived paths {ref-floor} is $bootstrap_floor" \
    "set both to the version of the first release whose bootstrap honors --ref"
fi

exit "$drift"
