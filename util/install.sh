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
$DOAS cp -v target/x86_64-unknown-linux-musl/release/wg-man "$PREFIX/bin"
