#!/usr/bin/env bash
# Measures whether the runtime's store and pointer can be replaced while a
# `ductus mcp` server runs from them, replacing each the way
# framework/bootstrap/ductus.md does: the store by tempfile + rename (Runtime
# acquisition, step 6), the pointer by `ln -sf` and then the `cp` fallback
# (Pointer materialization, steps 2-3). Spec 048, task 29.
#
# It prints the environment, then one line per case: the command, its exit
# status and stderr, whether the server was still running afterwards, whether
# the destination is a symlink, and whether it now holds the new content.
# It fails only when it cannot establish what it measures (a server not running
# just before its replacement, a control replacement with no server that
# fails, an existing store it would overwrite), never on a measured outcome.
#
# Usage: measure-replace-under-server.sh <ductus binary>
# It writes $HOME/.ductus/bin, as the bootstrap does, and refuses to run when a
# store is already there: run it on a disposable runner or with HOME pointed at
# a scratch directory.
set -uo pipefail

bin="${1:?usage: $0 <ductus binary>}"
bin="$(cd "$(dirname "$bin")" && pwd)/$(basename "$bin")"
ext=""
native=false
case "$(uname -s)" in
  MINGW* | MSYS* | CYGWIN*)
    ext=".exe"
    native=true
    ;;
esac

store_dir="$HOME/.ductus/bin"
store="$store_dir/ductus$ext"
if [ -e "$store" ] || [ -L "$store" ]; then
  echo "harness: $store exists; refusing to overwrite it" >&2
  exit 1
fi

work="$(mktemp -d)"
proj="$work/proj"
pointer=".ductus/bin/ductus$ext"
mkdir -p "$store_dir" "$proj/.ductus/bin"
cd "$proj" || exit 1

digest() {
  if command -v sha256sum > /dev/null 2>&1; then
    sha256sum "$1" 2> /dev/null | cut -d' ' -f1
  else
    shasum -a 256 "$1" 2> /dev/null | cut -d' ' -f1
  fi
}

# A is what each server runs; B is what replaces it, A plus one byte, so the
# destination's digest says which one it holds.
old="$work/old$ext"
new="$work/new$ext"
cp "$bin" "$old"
cp "$bin" "$new"
printf '\0' >> "$new"
d_old="$(digest "$old")"
d_new="$(digest "$new")"
if [ -z "$d_old" ] || [ "$d_old" = "$d_new" ]; then
  echo "harness: could not make two distinct binaries" >&2
  exit 1
fi

echo "uname: $(uname -a)"
echo "bash: $BASH_VERSION"
echo "MSYS: ${MSYS-<unset>}"
echo "image: ${ImageOS-<unset>} ${ImageVersion-<unset>}"
echo "binary: $bin ($("$bin" --version 2>&1))"
echo "HOME: $HOME"
echo "project: $proj"

srv=""
feeder=""
# Starts `<exe> mcp` with its stdin held open by a sleep, since the server
# exits at end of input, and a server that has exited makes every replacement
# succeed.
start_server() {
  local exe="$1" label="$2"
  # shellcheck disable=SC2016 # $$ is the child sh's own pid, which exec keeps
  sh -c 'echo "$$" > "$1"; exec sleep 600' sh "$work/$label.feeder" < /dev/null 2> /dev/null |
    "$exe" mcp > "$work/$label.out" 2> "$work/$label.err" &
  srv=$!
  sleep 2
  feeder="$(cat "$work/$label.feeder")"
  if ! kill -0 "$srv" 2> /dev/null; then
    echo "harness: the $label server from $exe is not running after start" >&2
    cat "$work/$label.err" >&2
    exit 1
  fi
}

stop_server() {
  [ -n "$feeder" ] && kill "$feeder" 2> /dev/null
  sleep 1
  [ -n "$srv" ] && kill "$srv" 2> /dev/null
  [ -n "$srv" ] && wait "$srv" 2> /dev/null
  srv=""
  feeder=""
  return 0
}
trap 'stop_server; rm -f "$store" "$store.tmp" "$store.old"; rm -rf "$work"' EXIT

last_rc=0
# measure <case> <destination> <command>: runs the command in this shell, at
# the project root, and prints what it did. The server, when one runs, must be
# alive just before the command.
measure() {
  local name="$1" dest="$2" cmd="$3" rc err alive="-" link=no held=other
  if [ -n "$srv" ] && ! kill -0 "$srv" 2> /dev/null; then
    echo "harness: $name: the server is not running before the replacement" >&2
    exit 1
  fi
  err="$(eval "$cmd" 2>&1 > /dev/null)"
  rc=$?
  if [ -n "$srv" ]; then
    if kill -0 "$srv" 2> /dev/null; then alive=yes; else alive=no; fi
  fi
  if [ -L "$dest" ]; then link=yes; fi
  case "$(digest "$dest")" in
    "$d_new") held=new ;;
    "$d_old") held=old ;;
    "") held=absent ;;
  esac
  printf '%s | %s | exit %s | stderr: %s | server alive after: %s | symlink: %s | holds: %s\n' \
    "$name" "$cmd" "$rc" "${err:-<none>}" "$alive" "$link" "$held"
  last_rc=$rc
}

require_ok() {
  if [ "$last_rc" -ne 0 ]; then
    echo "harness: control $1 failed with no server running" >&2
    exit 1
  fi
}

# The bootstrap's spellings, run at the project root.
store_cmd="cp '$new' ~/.ductus/bin/ductus$ext.tmp && mv -f ~/.ductus/bin/ductus$ext.tmp ~/.ductus/bin/ductus$ext"
ln_cmd="ln -sf ~/.ductus/bin/ductus$ext $pointer"
cp_cmd="cp ~/.ductus/bin/ductus$ext $pointer"
store_aside_cmd="mv -f ~/.ductus/bin/ductus$ext ~/.ductus/bin/ductus$ext.old && $store_cmd"
pointer_aside_cmd="mv -f $pointer $pointer.old && $cp_cmd"

echo "--- store: tempfile + rename"
cp "$old" "$store"
measure store-control "$store" "$store_cmd"
require_ok store-control

cp "$old" "$store"
start_server "$store" store
measure store-running "$store" "$store_cmd"
stop_server
rm -f "$store.tmp"

cp "$old" "$store"
start_server "$store" store-aside
measure store-running-moved-aside "$store" "$store_aside_cmd"
measure store-running-remove-moved-aside "$store.old" "rm -f ~/.ductus/bin/ductus$ext.old"
stop_server
rm -f "$store.tmp" "$store.old"

if $native; then
  w_new="$(cygpath -w "$new")"
  w_store="$(cygpath -w "$store")"
  ps_store_cmd="powershell -NoProfile -Command \"\\\$ErrorActionPreference='Stop'; Copy-Item -LiteralPath '$w_new' -Destination '$w_store.tmp'; Move-Item -Force -LiteralPath '$w_store.tmp' -Destination '$w_store'\""
  cp "$old" "$store"
  measure store-control-powershell "$store" "$ps_store_cmd"
  require_ok store-control-powershell
  cp "$old" "$store"
  start_server "$store" store-ps
  measure store-running-powershell "$store" "$ps_store_cmd"
  stop_server
  rm -f "$store.tmp"
fi

echo "--- pointer: ln -sf, then the cp fallback (the store holds the new binary)"
cp "$new" "$store"

rm -f "$pointer"
cp "$old" "$pointer"
measure pointer-control-ln "$pointer" "$ln_cmd"

rm -f "$pointer"
cp "$old" "$pointer"
measure pointer-control-cp "$pointer" "$cp_cmd"
require_ok pointer-control-cp

rm -f "$pointer"
cp "$old" "$pointer"
start_server "$pointer" pointer-ln
measure pointer-running-ln "$pointer" "$ln_cmd"
stop_server

rm -f "$pointer"
cp "$old" "$pointer"
start_server "$pointer" pointer-cp
measure pointer-running-cp "$pointer" "$cp_cmd"
stop_server

rm -f "$pointer" "$pointer.old"
cp "$old" "$pointer"
start_server "$pointer" pointer-aside
measure pointer-running-moved-aside "$pointer" "$pointer_aside_cmd"
measure pointer-running-remove-moved-aside "$pointer.old" "rm -f $pointer.old"
stop_server

if $native; then
  w_pointer="$(cygpath -w "$proj/$pointer")"
  ps_cp_cmd="powershell -NoProfile -Command \"\\\$ErrorActionPreference='Stop'; Copy-Item -Force -LiteralPath '$w_store' -Destination '$w_pointer'\""
  rm -f "$pointer" "$pointer.old"
  cp "$old" "$pointer"
  start_server "$pointer" pointer-ps
  measure pointer-running-powershell "$pointer" "$ps_cp_cmd"
  stop_server
fi

echo "--- done"
