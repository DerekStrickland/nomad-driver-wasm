#!/bin/bash

echo "Loading plugin dir..."

NOMAD_PLUGIN_DIR=${NOMAD_PLUGIN_DIR:-/opt/nomad/plugins}
echo "Deploying to : $NOMAD_PLUGIN_DIR"

if [ ! -w "$NOMAD_PLUGIN_DIR" ] ; then
  echo 'NOMAD_PLUGIN_DIR is not writable by your user; re-run script with sudo'
  exit 1
fi

RUST_LOG=debug cargo build --bin nomad-driver-wasm --release

cp target/release/nomad-driver-wasm "$NOMAD_PLUGIN_DIR"/nomad-driver-wasm

stat "$NOMAD_PLUGIN_DIR"/nomad-driver-wasm
