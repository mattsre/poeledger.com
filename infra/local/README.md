# Local Setup

Make sure you have `docker`, `kind`, `kubectl`, `helm`, and `ctlptl` installed. Then you can simply run:
```sh
./setup.sh
```

## NATS

A single-node NATS Jetstream server is deployed to the `poeledger-local` cluster by default, as this is used for recieving messages from the river crawlers.
