#!/bin/sh
# ductus installer — places the /ductus bootstrap command for your AI coding agent.
#
# Usage:
#   curl --proto '=https' --tlsv1.2 -sSfL https://github.com/stonean/ductus/releases/latest/download/install.sh | sh
#
# Pick an agent explicitly (default: claude):
#   ... | sh -s -- claude
#   ... | sh -s -- auggie
#   ... | sh -s -- antigravity   # 'agy' (the Antigravity CLI name) also works
#   ... | sh -s -- opencode
#   ... | sh -s -- pi
#
# Pick the source (default: the latest release), in any position beside the agent:
#   ... | sh -s -- claude --ref=main             # main, ahead of any release
#   ... | sh -s -- --ref=ductus-v0.55.0 auggie   # a named release
#   ... | sh -s -- --ref=latest                  # the default, spelled out
#
# The installer is attached to every release, and the one-liner above fetches
# the latest release's copy (-L follows GitHub's redirect to its asset host).
# By default it places the latest release's bootstrap, resolved from the
# releases/latest redirect, so the installer and the bootstrap come from the
# same release. --ref places another source's bootstrap instead. The installer
# writes no project configuration, so run /ductus with the same --ref to record
# the choice; a plain /ductus uses the latest release. Spec 061 is the contract.
#
# The script is idempotent — re-run it any time to refresh the bootstrap file.
#
# Source repository (spec 065): the raw bootstrap URL's owner/repo. When
# DUCTUS_REPO is unset or empty, the canonical stonean/ductus is fetched, byte
# for byte as before. Set it to another owner/repo to adopt or test ductus from
# a fork (e.g. DUCTUS_REPO=myfork/ductus); /ductus itself honors the same
# variable for its subsequent fetches (version pin, archive, runtime release,
# self-update), so the whole adoption stays on one origin. Any other origin is
# announced on stderr, so a variable left set in a shell profile cannot
# silently change where the runtime binary comes from.
set -eu

CANONICAL_REPO="stonean/ductus"
repo="${DUCTUS_REPO:-$CANONICAL_REPO}"
REPO_RAW="https://raw.githubusercontent.com/$repo"
LATEST_URL="https://github.com/$repo/releases/latest"
if [ "$repo" != "$CANONICAL_REPO" ]; then
  echo "ductus: source repository is $repo (from DUCTUS_REPO), not the canonical $CANONICAL_REPO" >&2
fi

# The first release whose bootstrap honors --ref. Every earlier release's
# bootstrap fetches from main whatever ref it is given, so a tag below this
# cannot be honored. framework/bootstrap/ductus.md carries the same value as
# {ref-floor}, and /audit Family 14 asserts the two agree.
REF_FLOOR="0.55.0"

# is_tag VALUE — true when VALUE is ductus-v<MAJOR>.<MINOR>.<PATCH>, digits only.
is_tag() {
  case "$1" in ductus-v*) ;; *) return 1 ;; esac
  v="${1#ductus-v}"
  major="${v%%.*}"; rest="${v#*.}"
  [ "$rest" != "$v" ] || return 1
  minor="${rest%%.*}"; patch="${rest#*.}"
  [ "$patch" != "$rest" ] || return 1
  for n in "$major" "$minor" "$patch"; do
    case "$n" in '' | *[!0-9]*) return 1 ;; esac
  done
}

# version_lt A B — true when the X.Y.Z version A is below B, field by field.
version_lt() {
  a="$1"; b="$2"
  for _ in 1 2 3; do
    x="${a%%.*}"; y="${b%%.*}"
    [ "$x" -lt "$y" ] && return 0
    [ "$x" -gt "$y" ] && return 1
    a="${a#*.}"; b="${b#*.}"
  done
  return 1
}

# Arguments: the first non-flag word is the agent; --ref=<value> may appear in
# any position. A repeated --ref, an unknown flag, or a second word halts.
agent=""
ref=""
ref_count=0
ref_values=""
for arg in "$@"; do
  case "$arg" in
    --ref=* | --ref)
      ref="${arg#--ref}"; ref="${ref#=}"
      ref_count=$((ref_count + 1))
      ref_values="${ref_values:+$ref_values, }\"$ref\""
      ;;
    --*)
      echo "ductus: unknown flag '$arg' (expected: --ref=<latest|main|ductus-vX.Y.Z>)" >&2
      exit 1
      ;;
    *)
      if [ -n "$agent" ]; then
        echo "ductus: unexpected argument '$arg' — give one agent, and the source as --ref=<value>" >&2
        exit 1
      fi
      agent="$arg"
      ;;
  esac
done
agent="${agent:-claude}"

if [ "$ref_count" -gt 1 ]; then
  echo "ductus: --ref was given $ref_count times ($ref_values) — give it once" >&2
  exit 1
fi
if [ "$ref_count" -eq 1 ] && [ "$ref" != "latest" ] && [ "$ref" != "main" ] && ! is_tag "$ref"; then
  echo "ductus: invalid --ref \"$ref\" — accepted forms are latest, main, or ductus-v<MAJOR>.<MINOR>.<PATCH>" >&2
  exit 1
fi

if ! command -v curl >/dev/null 2>&1; then
  echo "ductus: curl is required but was not found on PATH" >&2
  exit 1
fi

# Resolve the source. `latest` is GitHub's latest release — never a draft or a
# prerelease — read from the redirect's Location header without following it.
# There is no fallback to main: an unresolvable latest release halts.
if [ -z "$ref" ] || [ "$ref" = "latest" ]; then
  if ! headers="$(curl --proto '=https' --tlsv1.2 -sSI "$LATEST_URL" 2>&1)"; then
    echo "ductus: could not resolve the latest release from $LATEST_URL — curl failed: $headers" >&2
    echo "ductus: the installer does not fall back to main; pass --ref=main or --ref=<tag> to choose a source explicitly" >&2
    exit 1
  fi
  headers="$(printf '%s\n' "$headers" | tr -d '\r')"
  location="$(printf '%s\n' "$headers" | sed -n 's/^[Ll][Oo][Cc][Aa][Tt][Ii][Oo][Nn]:[[:space:]]*//p' | tail -n 1)"
  tag="${location##*/releases/tag/}"
  if [ -z "$location" ] || [ "$tag" = "$location" ] || ! is_tag "$tag"; then
    status="$(printf '%s\n' "$headers" | sed -n '1p')"
    echo "ductus: could not resolve the latest release from $LATEST_URL — got \"$status\" with Location \"${location:-no Location header}\"" >&2
    echo "ductus: the installer does not fall back to main; pass --ref=main or --ref=<tag> to choose a source explicitly" >&2
    exit 1
  fi
  raw_ref="$tag"
  source_label="latest release $tag"
elif [ "$ref" = "main" ]; then
  tag=""
  raw_ref="main"
  source_label="main"
else
  tag="$ref"
  raw_ref="$ref"
  source_label="$ref"
fi

if [ -n "$tag" ] && version_lt "${tag#ductus-v}" "$REF_FLOOR"; then
  if [ -z "$ref" ] || [ "$ref" = "latest" ]; then
    echo "ductus: the latest release, $tag, is older than ductus-v$REF_FLOOR, the first release that carries this installer's bootstrap. That release may still be publishing. Re-run once it exists, or pass --ref=main to proceed now." >&2
  else
    echo "ductus: $tag is older than ductus-v$REF_FLOOR, the first release whose bootstrap honors --ref. Every earlier release's bootstrap fetches from main, so $tag could not be honored. Name ductus-v$REF_FLOOR or later, or pass --ref=main." >&2
  fi
  exit 1
fi

RAW="$REPO_RAW/$raw_ref/framework/bootstrap/ductus.md"

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT
if ! code="$(curl --proto '=https' --tlsv1.2 -sSL -o "$tmp" -w '%{http_code}' "$RAW")"; then
  echo "ductus: could not fetch the bootstrap from $RAW — check network access" >&2
  exit 1
fi
if [ "$code" = "404" ] && [ -n "$ref" ] && [ "$ref" != "latest" ] && [ "$ref" != "main" ]; then
  echo "ductus: tag $ref does not exist — $RAW returned 404" >&2
  exit 1
fi
if [ "$code" != "200" ]; then
  echo "ductus: could not fetch the bootstrap from $RAW — HTTP $code" >&2
  exit 1
fi

# Confirm the payload is actually the bootstrap before installing it. `curl -f`
# rejects an HTTP error status, but a 200 carrying a captive-portal page, a
# proxy interstitial, or an empty body passes -f and would otherwise be written
# out and reported as a successful install. The antigravity arm below splits
# ductus.md on its frontmatter, so such a payload yields a skill with an empty
# body — a no-op install that says it worked. Checked once here, for every arm,
# so no arm reports an install it did not verify.
delims="$(grep -c '^---[[:space:]]*$' "$tmp" || true)"
if [ ! -s "$tmp" ] || [ "${delims:-0}" -lt 2 ]; then
  echo "ductus: the downloaded file is not the ductus bootstrap — got $(wc -c < "$tmp" | tr -d ' ') bytes with ${delims:-0} frontmatter delimiter(s), expected at least 2" >&2
  echo "ductus: check network access to $RAW — a proxy or captive portal may have answered with a placeholder page" >&2
  exit 1
fi

case "$agent" in
  claude)
    dest=".claude/commands/ductus.md"
    mkdir -p .claude/commands
    cp "$tmp" "$dest"
    # Pre-seed permissions so the first /ductus run does not prompt for its
    # bootstrap shell commands (see the antigravity arm for the rationale).
    # Written only when absent — /ductus owns additive merges. Keep in sync with
    # the claude settings_template in framework/bootstrap/ductus.md (§Agent Registry).
    if [ ! -f .claude/settings.local.json ]; then
      cat > .claude/settings.local.json <<'JSON'
{
  "permissions": {
    "allow": [
      "Bash(curl *)",
      "Bash(ls *)",
      "Bash(tar *)",
      "Bash(mktemp *)",
      "Bash(git status *)",
      "Bash(git config *)",
      "Bash(git rev-parse *)",
      "Bash(git diff *)",
      "Bash(git ls-files *)",
      "Bash(chmod *)",
      "Bash(awk *)",
      "Bash(command -v *)",
      "Bash(mkdir *)",
      "Bash(shasum *)",
      "Bash(sha256sum *)",
      "Bash(certutil *)",
      "Bash(ln *)",
      "Bash(cp *)",
      "Bash(~/.ductus/bin/ductus *)",
      "Bash(.ductus/bin/ductus *)",
      "Read(/private/var/folders/**/T/ductus-*/**)",
      "Read(//private/var/folders/**/T/ductus-*/**)",
      "Read(/var/folders/**/T/ductus-*/**)",
      "Read(//var/folders/**/T/ductus-*/**)",
      "Read(/tmp/ductus-*/**)",
      "Read(//tmp/ductus-*/**)"
    ],
    "deny": []
  }
}
JSON
    fi
    ;;
  auggie)
    dest=".augment/commands/ductus.md"
    mkdir -p .augment/commands
    cp "$tmp" "$dest"
    # Pre-seed permissions so the first /ductus run does not prompt for its
    # bootstrap shell commands (see the antigravity arm for the rationale).
    # Written only when absent — /ductus owns additive merges. Keep in sync with
    # the auggie settings_template in framework/bootstrap/ductus.md (§Agent Registry).
    if [ ! -f .augment/settings.local.json ]; then
      cat > .augment/settings.local.json <<'JSON'
{
  "toolPermissions": [
    {
      "toolName": "launch-process",
      "shellInputRegex": "^curl ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^ls ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^tar ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^mktemp ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^git status ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^git config ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^git rev-parse ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^git diff ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^git ls-files ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^chmod ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^awk ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^command -v ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^mkdir ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^shasum ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^sha256sum ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^certutil ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^ln ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^cp ",
      "permission": {
        "type": "allow"
      }
    },
    {
      "toolName": "launch-process",
      "shellInputRegex": "^[^ ]*\\.ductus/bin/ductus ",
      "permission": {
        "type": "allow"
      }
    }
  ]
}
JSON
    fi
    ;;
  antigravity | agy)
    agent="antigravity"  # 'agy' is the Antigravity CLI command name
    dest=".agents/skills/ductus/SKILL.md"
    mkdir -p .agents/skills/ductus
    # Antigravity discovers dir-form skills: wrap ductus.md's body in skill
    # frontmatter, dropping ductus.md's own frontmatter (everything up to and
    # including the second `---`).
    {
      printf -- '---\nname: ductus\n---\n'
      awk 'p{print} /^---[[:space:]]*$/{c++; if(c==2)p=1}' "$tmp"
    } > "$dest"
    # Pre-seed the permission file so the first /ductus run does not prompt for
    # its bootstrap shell commands. Antigravity loads permissions at session
    # start, so ductus.md's in-run Permission Setup seed lands too late for the
    # first run. Written only when absent — /ductus owns additive merges into an
    # existing settings.json. Keep this allow-list in sync with the antigravity
    # settings_template in framework/bootstrap/ductus.md (§Agent Registry).
    if [ ! -f .agents/settings.json ]; then
      cat > .agents/settings.json <<'JSON'
{
  "permissions": {
    "allow": [
      "command(curl)",
      "command(ls)",
      "command(tar)",
      "command(mktemp)",
      "command(git status)",
      "command(git config)",
      "command(git rev-parse)",
      "command(git diff)",
      "command(git ls-files)",
      "command(chmod)",
      "command(awk)",
      "command(which)",
      "command(mkdir)",
      "command(shasum)",
      "command(sha256sum)",
      "command(certutil)",
      "command(ln)",
      "command(cp)"
    ],
    "deny": [],
    "ask": []
  }
}
JSON
    fi
    ;;
  opencode)
    dest=".opencode/command/ductus.md"
    mkdir -p .opencode/command
    cp "$tmp" "$dest"
    # Pre-seed permissions so the first /ductus run does not prompt for its
    # bootstrap shell commands. OpenCode keeps both MCP wiring and permissions in
    # one committed opencode.json; this seeds only the permission block, and only
    # when neither opencode.json nor opencode.jsonc exists — /ductus owns additive
    # merges. Keep in sync with the opencode settings_template in
    # framework/bootstrap/ductus.md (§Agent Registry).
    if [ ! -f opencode.json ] && [ ! -f opencode.jsonc ]; then
      cat > opencode.json <<'JSON'
{
  "$schema": "https://opencode.ai/config.json",
  "permission": {
    "bash": {
      "curl *": "allow",
      "ls *": "allow",
      "tar *": "allow",
      "mktemp *": "allow",
      "git status *": "allow",
      "git config *": "allow",
      "git rev-parse *": "allow",
      "git diff *": "allow",
      "git ls-files *": "allow",
      "chmod *": "allow",
      "awk *": "allow",
      "command -v *": "allow",
      "mkdir *": "allow",
      "shasum *": "allow",
      "sha256sum *": "allow",
      "certutil *": "allow",
      "ln *": "allow",
      "cp *": "allow",
      "*/.ductus/bin/ductus *": "allow"
    }
  }
}
JSON
    fi
    ;;
  pi)
    dest=".pi/prompts/ductus.md"
    mkdir -p .pi/prompts
    cp "$tmp" "$dest"
    # No settings seed: Pi has no permission-gating settings (verified, spec 064
    # §Verified Pi Layout — the model runs tool calls without a host permission
    # prompt), so there is nothing to pre-authorize for the first /ductus run.
    # The one Pi prerequisite is project trust — prompt templates under .pi/
    # load only after the project is trusted, which pi itself prompts for on the
    # first interactive start (or --approve on a non-interactive run).
    ;;
  *)
    echo "ductus: unknown agent '$agent' (expected: claude, auggie, antigravity, agy, opencode, or pi)" >&2
    exit 1
    ;;
esac

echo "ductus: installed the $agent bootstrap from $source_label -> $dest"
if [ "$ref_count" -eq 1 ]; then
  echo "ductus: now run '/ductus --ref=$ref <project-name>' in your agent to scaffold the project; that run records the source (a plain '/ductus' uses the latest release)."
else
  echo "ductus: now run '/ductus <project-name>' in your agent to scaffold the project."
fi
# /ductus reads DUCTUS_REPO from the agent's own environment, which a variable
# set inline for this script alone never reaches — and the fork's bootstrap
# would then fetch everything after it from the canonical repository.
if [ "$repo" != "$CANONICAL_REPO" ]; then
  echo "ductus: start your agent with DUCTUS_REPO=$repo exported, or /ductus fetches the rest of the adoption from $CANONICAL_REPO"
fi
