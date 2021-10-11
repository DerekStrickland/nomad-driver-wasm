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

## Running with Nomad.

To run with Nomad, first deploy the binary to your plugins folder. You can do this
by running the following from the root of this directory. By default, this script
writes output to `/opt/nomad/plugins`, which required elevated privileges to write
to.

```shell
$ sudo ./release.sh
```

You can customize the location of your release binary by setting the `NOMAD_PLUGIN_DIR`
environment variable to the directory of your choosing.  

Next, deploy the `nomad-driver-wasm.hcl` file to your nomad config directory, or 
manually merge its contents with whatever file you used to configure your Nomad
agent. If you have deployed the plugin to some location other than `/opt/nomad/plugins`,
you will need to make sure you update the `plugin_dir` setting in this example
file to match the location to which you have deployed the `nomad-driver-wasm`
binary.

Finally, start or restart you Nomad agent.  You should see a log entry similar
to the following indicating the task driver has successfully started.

```shell
[DEBUG] client.driver_mgr: plugin started: driver=nomad-driver-wasm path=/opt/nomad/plugins/nomad-driver-wasm pid=4679
```