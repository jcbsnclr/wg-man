#!/usr/bin/env sh

DOAS="$(which sudo | which doas)"

if [ "$USER" != "root" ] && [ -z "$DOAS" ]; then
  echo "$(basename $0): sudo or doas not found; exiting" 1>&2
  exit 1
fi

$DOAS cp -rv etc/* /etc/
