# nomad-driver-wasmtime

Nomad driver for wasmtime workloads

## Building

This repository contains Nomad as a submodule. After cloning this repo, run the
following command.

`git submodule init && git submodule update`

This will ensure you Nomad available on your environment so that the `build.rs`
file can reference Nomad's `.proto` files at a well known location.

Next, run `cargo build` from the root of this repository. To debug build errors, you
may wish to run `RUST_BACKTRACE=full cargo build &> build.log` instead, which
will write to `build.log` in the root of the repository, and allow you to more
easily examine build output.
