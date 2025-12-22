#!/usr/bin/env sh

if [ -z "$PREFIX" ]; then
  PREFIX="/usr/local"
fi

DOAS="$(which sudo | which doas)"

if [ "$USER" != "root" ] && [ -z "$DOAS" ]; then
  echo "$(basename $0): sudo or doas not found; exiting" 1>&2
  exit 1
fi

cargo build --release
$DOAS install -Dm755 target/release/wg-man "$PREFIX/bin"
