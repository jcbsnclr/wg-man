#!/usr/bin/env sh

DOAS="$(which sudo | which doas)"

if [ "$USER" != "root" ] && [ -z "$DOAS" ]; then
  echo "$(basename $0): sudo or doas not found; exiting" 1>&2
  exit 1
fi

# $DOAS cp -rv etc/* /etc/
$DOAS install -Dm755 etc/init.d/wg-man /etc/init.d/wg-man
$DOAS install -Dm644 etc/conf.d/wg-man /etc/conf.d/wg-man
