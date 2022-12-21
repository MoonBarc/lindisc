# lindisc
Linear + Discord

## Usage
Lindisc takes a Linear webhook and converts it into a Discord webhook.

It does this through the `/linput` route.

To start lindisc in development, run
```sh
cargo run
```
and then point your new Linear webhook to `http(s)://host:5252/linput`.

## Production
If you wish to run Lindisc in production, then you should add the `--release` flag to cargo.

For example:
```sh
cargo build --release # only required once
./target/release/lindisc # run the executable produced. add .exe if on windows
```