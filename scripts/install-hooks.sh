#!/usr/bin/env bash
# Install ductus repo's git hooks by setting core.hooksPath, and make sure the
# runtime binary those hooks call actually exists.
#
# Idempotent: safe to run repeatedly. The actual hook scripts live in
# .githooks/ and are part of the repo.

set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

# --- core.hooksPath ----------------------------------------------------------

current="$(git config --get core.hooksPath || echo "")"
if [ "$current" = ".githooks" ]; then
  echo "core.hooksPath already set to .githooks (no change)"
else
  if [ -n "$current" ]; then
    echo "Warning: core.hooksPath is currently set to '$current'." >&2
    echo "Overwriting with '.githooks'." >&2
  fi
  git config core.hooksPath .githooks
  echo "Set core.hooksPath = .githooks"
fi

# Make sure hook scripts are executable.
chmod +x "$ROOT/.githooks/pre-commit"

# --- the runtime the hook calls ---------------------------------------------
#
# `.githooks/pre-commit` derives the `dependencies:` and `references:`
# frontmatter through the runtime (spec 022, adopter-generator-promotion) and
# HALTS when the binary is unreachable, because those indexes are captured by
# the commit and skipping them would land stale values.
#
# This check runs on every invocation, including the already-wired path above:
# "hooks installed, binary absent" is reachable whenever the binary is missing,
# not only on the run that first sets core.hooksPath. Turning the hook on is
# also the moment that state starts to matter, which is what makes this the
# right place to catch it — the alternative is the contributor discovering it
# when their next commit is rejected.
#
# A cold `cargo build --release` is slow, so this builds only when the binary is
# absent; cargo is a cheap no-op once it exists. A missing toolchain or a failed
# build is a warning rather than a hard failure: the hooks are installed either
# way, the pre-commit halt names the same fix, and `--no-verify` remains the
# deliberate bypass. Refusing to finish the install would be the worse trade.

RUNTIME_BIN="runtime/target/release/ductus"

if [ -x "$RUNTIME_BIN" ]; then
  echo "Runtime binary present at $RUNTIME_BIN"
elif ! command -v cargo > /dev/null 2>&1; then
  echo "Warning: $RUNTIME_BIN is missing and cargo is not on PATH." >&2
  echo "  The pre-commit hook derives spec frontmatter through the runtime and" >&2
  echo "  will halt until the binary exists. Install Rust, then run:" >&2
  echo "    (cd runtime && cargo build --release)" >&2
else
  echo "Building the runtime the hook calls ($RUNTIME_BIN)..."
  if (cd runtime && cargo build --release --quiet); then
    echo "Runtime built."
  else
    echo "Warning: cargo build --release failed." >&2
    echo "  Hooks are installed, but the pre-commit hook will halt until the" >&2
    echo "  build succeeds. Re-run: (cd runtime && cargo build --release)" >&2
  fi
fi

echo "Hooks installed. The next commit will run all generators."
