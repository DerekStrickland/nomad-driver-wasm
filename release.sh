#!/bin/bash

read NOMAD_PLUGIN_DIR
NOMAD_PLUGIN_DIR=${NOMAD_PLUGIN_DIR:-/opt/nomad/data/plugins}
echo "Deploying to : $NOMAD_PLUGIN_DIR"

RUST_LOG=debug CARGO_TARGET_DIR=$NOMAD_PLUGIN_DIR cargo build --bin nomad-driver-wasm --release