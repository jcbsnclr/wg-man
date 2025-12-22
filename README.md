# `wg-man` -- WireGuard configuration manager
`wg-man` lets you spin up/down a random WireGuard client connection based on a given regex.

## usage
```sh
wg-man up                    # bring up random config matching default regex (`^`)
wg-man up ^se                # bring up random config matching given regex
wg-man down                  # bring down currently running config
```

### OpenRC
you can configure options for `wg-man` in [`/etc/conf.d/wg-man`](etc/conf.d/wg-man).

```sh
rc-service wg-man start      # start wg-man service
rc-service wg-man stop       # stop wg-man service
rc-update add wg-man         # add wg-man to default runlevel
rc-update add wg-man [LEVEL] # add wg-man to given runlevel
```

## building & installation
```sh
cargo build --release        # build project
./util/install.sh            # build & install to $PREFIX (/usr/local by default)
./util/install-openrc.sh     # copy OpenRC service & config files to /etc
```

## copying
see: [LICENSE](/LICENSE)
