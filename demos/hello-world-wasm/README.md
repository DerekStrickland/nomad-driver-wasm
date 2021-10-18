# Hello World Wasm for the Nomad WASM Task Driver

A simple hello world example in Rust that will print:

- The environment variables available to the process
- Text to both stdout and stderr.
- Any args passed to the process

It is meant to be a simple demo for Nomad wasm Task Driver.

## Running the example

First, make sure you have Nomad running and configured with the task driver installed.
See the main [README](https://github.com/DerekStrickland/nomad-driver-wasm#readme)
for instructions.

## Building from Source

To compile and upload the demo, you'll need to do the following.

### Prerequisites

You'll need to have Rust installed with `wasm32-wasi` target installed:

```shell
$ rustup target add wasm32-wasi
```

The Nomad WASM Task Driver relies on you having pushed your WebAssembly module
to a container registry using. You must package your wasm module as an OCI image
using [`wasm-to-oci`](https://github.com/engineerd/wasm-to-oci#installation). See
that link for instruction on how to install `wasm-to-oci`.

These examples use a local registry that will be run in docker on your development
host, but this should work with any OCI spec compliant registry. 

For more details on how to run a local registry, [review these instructions](https://github.com/docker/docker.github.io/blob/master/registry/deploying.md#run-a-local-registry).

### Building

Run:

```shell
$ cargo build --target wasm32-wasi --release
```

### Running the local registry

If using a local registry, start it now if you haven't already.

```shell
$ docker run -d -p 5000:5000 --restart=always --name registry registry:2
```

### Pushing

Push to the local registry.

```shell
$ wasm-to-oci push target/wasm32-wasi/release/hello-world-wasm.wasm localhost:5000/hello-world-wasm:v1
```

### Register the job with Nomad

Now register the job with Nomad.

```shell
$ nomad run hello-world-wasm.nomad
```

## Attribution

This demo is adapted from the [Hello World Rust](https://github.com/krustlet/krustlet/tree/main/demos/wasi/hello-world-rust)
demo that is part of the [Krustlet project](https://krustlet.dev/). Thanks to
[DeisLabs](https://deislabs.io/) for making their work open source.
