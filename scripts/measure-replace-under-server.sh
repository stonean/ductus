#!/usr/bin/env bash
# Measures whether the runtime's store and pointer can be replaced while a
# `ductus mcp` server runs from them, replacing each the way
# framework/bootstrap/ductus.md does: the store by tempfile + rename (Runtime
# acquisition, step 6), the pointer by `ln -sf` and then the `cp` fallback
# (Pointer materialization, steps 3-5). Spec 048, tasks 29 and 31.
#
# It prints the environment, then one line per case: the command, its exit
# status and stderr, whether the server was still running afterwards and two
# seconds later, whether the destination is a symlink, and whether it now holds
# the new content. It also measures whether a pointer `ln -sf` made follows the
# store when the store is replaced, and on Windows the elevation, Developer
# Mode, and `ln -sf` with MSYS asking for native symlinks. Last it measures the
# bootstrap's own repair sequence (Pointer materialization, steps 2-5): the
# digest comparison spelled per platform (`shasum -a 256`, `sha256sum`,
# `certutil -hashfile`), a stale pointer re-made by `ln -sf` with a server
# running from it, and the move-aside sequence with and without a leftover
# `.old`.
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

# The digest the bootstrap's Pointer materialization spells for this platform,
# first digest only; empty when the tool cannot compute one.
bootstrap_digest() {
  case "$(uname -s)" in
    Darwin) shasum -a 256 "$1" 2> /dev/null | cut -d' ' -f1 ;;
    MINGW* | MSYS* | CYGWIN*)
      certutil -hashfile "$(cygpath -w "$1")" SHA256 2> /dev/null | sed -n 2p | tr -d ' \r' | tr 'A-F' 'a-f'
      ;;
    *) sha256sum "$1" 2> /dev/null | cut -d' ' -f1 ;;
  esac
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

if [ -z "$(bootstrap_digest "$old")" ]; then
  echo "harness: the bootstrap's digest tool computed nothing for $old" >&2
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
  local name="$1" dest="$2" cmd="$3" rc err alive="-" later="-" link=no held=other
  if [ -n "$srv" ] && ! kill -0 "$srv" 2> /dev/null; then
    echo "harness: $name: the server is not running before the replacement" >&2
    exit 1
  fi
  err="$(eval "$cmd" 2>&1 > /dev/null)"
  rc=$?
  if [ -n "$srv" ]; then
    if kill -0 "$srv" 2> /dev/null; then alive=yes; else alive=no; fi
    # Again after a pause: a process whose image changed under it may die
    # only when it next pages code in.
    sleep 2
    if kill -0 "$srv" 2> /dev/null; then later=yes; else later=no; fi
  fi
  if [ -L "$dest" ]; then link=yes; fi
  case "$(digest "$dest")" in
    "$d_new") held=new ;;
    "$d_old") held=old ;;
    "") held=absent ;;
  esac
  printf '%s | %s | exit %s | stderr: %s | server alive after: %s, 2s later: %s | symlink: %s | holds: %s\n' \
    "$name" "$cmd" "$rc" "${err:-<none>}" "$alive" "$later" "$link" "$held"
  last_rc=$rc
}

# compare_row <case>: what the bootstrap's comparison (Pointer materialization,
# step 2) concludes about the pointer and the store, spelled as the bootstrap
# spells it. A digest that cannot be computed reads as differing.
compare_row() {
  local name="$1" dp ds verdict held=other link=no
  dp="$(bootstrap_digest "$pointer")"
  ds="$(bootstrap_digest "$store")"
  if [ -z "$dp" ] || [ -z "$ds" ]; then
    verdict="differs (a digest could not be computed)"
  elif [ "$dp" = "$ds" ]; then
    verdict=equal
  else
    verdict=differs
  fi
  if [ -L "$pointer" ]; then link=yes; fi
  case "$(digest "$pointer")" in
    "$d_new") held=new ;;
    "$d_old") held=old ;;
    "") held=absent ;;
  esac
  printf '%s | compare pointer and store digests | %s | pointer holds: %s | symlink: %s\n' \
    "$name" "$verdict" "$held" "$link"
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
pointer_aside_cmd="rm -f $pointer.old && mv -f $pointer $pointer.old && $cp_cmd"

echo "--- store: tempfile + rename"
cp "$old" "$store"
measure store-control "$store" "$store_cmd"
require_ok store-control

cp "$old" "$store"
start_server "$store" store
measure store-running "$store" "$store_cmd"
echo "store directory: $(ls -A "$store_dir" | tr '\n' ' ')"
stop_server
rm -f "$store.tmp"

cp "$old" "$store"
start_server "$store" store-aside
measure store-running-moved-aside "$store" "$store_aside_cmd"
measure store-running-remove-moved-aside "$store.old" "rm -f ~/.ductus/bin/ductus$ext.old"
echo "store directory: $(ls -A "$store_dir" | tr '\n' ' ')"
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
compare_row pointer-after-moved-aside-compare
stop_server

rm -f "$pointer" "$pointer.old"
cp "$old" "$pointer"
printf 'left by an interrupted run' > "$pointer.old"
start_server "$pointer" pointer-aside-leftover
measure pointer-running-moved-aside-leftover-old "$pointer" "$pointer_aside_cmd"
measure pointer-running-remove-moved-aside-leftover-old "$pointer.old" "rm -f $pointer.old"
compare_row pointer-after-moved-aside-leftover-old-compare
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

# Whether the pointer follows the store: materialized by the bootstrap's
# `ln -sf`, then the store replaced as acquisition replaces it. A symlink holds
# the new binary afterwards; a regular file holding the store's bytes keeps the
# old one.
echo "--- pointer after the store is replaced"
rm -f "$pointer" "$pointer.old"
cp "$old" "$store"
measure pointer-materialized-ln "$pointer" "$ln_cmd"
measure pointer-after-store-replaced "$pointer" "$store_cmd"

if $native; then
  echo "--- Windows: elevation, Developer Mode, and ln -sf asked for native symlinks"
  if net session > /dev/null 2>&1; then echo "elevated: yes"; else echo "elevated: no"; fi
  echo "developer mode: $(MSYS2_ARG_CONV_EXCL='*' reg query 'HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\AppModelUnlock' /v AllowDevelopmentWithoutDevLicense 2>&1 | tr -s '\r\n ' ' ')"
  cp "$new" "$store"
  for mode in native nativestrict; do
    rm -f "$pointer"
    cp "$old" "$pointer"
    measure "pointer-control-ln-$mode" "$pointer" "MSYS=winsymlinks:$mode $ln_cmd"
    rm -f "$pointer"
    cp "$old" "$pointer"
    start_server "$pointer" "pointer-ln-$mode"
    measure "pointer-running-ln-$mode" "$pointer" "MSYS=winsymlinks:$mode $ln_cmd"
    stop_server
  done
  rm -f "$pointer"
  cp "$old" "$store"
  measure pointer-materialized-ln-nativestrict "$pointer" "MSYS=winsymlinks:nativestrict $ln_cmd"
  measure pointer-after-store-replaced-nativestrict "$pointer" "$store_cmd"
fi

# The bootstrap's own repair (Pointer materialization, steps 2-3): compare the
# pointer with the store, and re-make a pointer that differs with `ln -sf`
# while a server runs from it. The store holds the new binary throughout.
echo "--- the bootstrap's repair: compare, then re-make"
cp "$new" "$store"
rm -f "$pointer" "$pointer.old"
cp "$old" "$pointer"
compare_row pointer-stale-copy-compare
cp "$new" "$pointer"
compare_row pointer-current-copy-compare
rm -f "$pointer"
compare_row pointer-missing-compare
# A dangling link: made only where `ln -s` makes a link at all.
ln -s "$work/nowhere" "$pointer" 2> /dev/null
compare_row pointer-dangling-compare
rm -f "$pointer"
cp "$old" "$pointer"
start_server "$pointer" repair-ln
measure pointer-stale-running-repair-ln "$pointer" "$ln_cmd"
compare_row pointer-after-repair-ln-compare
stop_server

# `cp` onto a pointer that is a differing symlink writes into the link's
# target; the bootstrap removes the link first. The decoy stands for the store
# or a `[runtime] path` binary the link points at, and must stay unchanged.
echo "--- cp onto a symlink pointer: the target, and the bootstrap's rm -f first"
decoy="$work/decoy$ext"
for variant in cp-only rm-then-cp; do
  rm -f "$pointer"
  cp "$old" "$decoy"
  ln -s "$decoy" "$pointer" 2> /dev/null
  if [ "$variant" = cp-only ]; then
    measure "pointer-symlink-$variant" "$pointer" "$cp_cmd"
  else
    measure "pointer-symlink-$variant" "$pointer" "if [ -L $pointer ]; then rm -f $pointer; fi && $cp_cmd"
  fi
  case "$(digest "$decoy")" in
    "$d_new") echo "  decoy (the link's target) holds: new" ;;
    "$d_old") echo "  decoy (the link's target) holds: old" ;;
    *) echo "  decoy (the link's target) holds: other" ;;
  esac
done

echo "--- done"
