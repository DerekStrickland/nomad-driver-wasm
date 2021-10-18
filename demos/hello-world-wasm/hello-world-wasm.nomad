job "hello-world-wasm" {
  datacenters = ["dc1"]

  type = "service"

  group "wasm" {

    count = 1

    network {
      port "http" {}

      port "https" {
        static = 443
      }
    }

    service {
      port = "http"

      check {
        type     = "http"
        path     = "/health"
        interval = "10s"
        timeout  = "2s"
      }
    }

    task "hello-world-wasm" {
      driver = "nomad-driver-wasm"

      artifact {
        source      = "https://github.com/DerekStrickland/nomad-driver-wasm/tree/main/demos/hello-world-wasm/ipsum.txt"
        destination = "local/config.hcl"
      }

      config {
        ports = ["http", "https"]
        command = "my-app"
        args = [
          "-config", "local/config.hcl",
        ]
      }

      env {
        FOO = "BAR"
      }

      resources {
        cpu    = 50 # MHz
        memory = 24 # MB
      }
    }
  }
}