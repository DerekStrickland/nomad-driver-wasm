plugin_dir = "/opt/nomad/plugins"

plugin "nomad-driver-wasm" {
  config {
    enabled = true
    wasm_runtime = "~/.wasmtime/bin/wasmtime"
    stats_interval = "5s"
    allow_privileged = true
    auth {
      username = ""
      password = ""
    }
  }
}
