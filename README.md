# `wg-man` -- WireGuard configuration manager
`wg-man` lets you spin up/down a random WireGuard client connection based on a given regex.

## usage
```sh
wg-man up                # bring up random config matching default regex (`^`)
wg-man up ^se            # bring up random config matching given regex
wg-man down              # bring down currently running config
```

## building & installation
```sh
cargo build --release    # build project
./util/install.sh        # build & install to $PREFIX (/usr/local by default)
./util/install-openrc.sh # copy OpenRC service & config files to /etc
```

## copying
see: [LICENSE](/LICENSE)
