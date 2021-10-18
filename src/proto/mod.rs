pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
}
pub mod hashicorp {
    pub mod nomad {
        pub mod plugins {
            pub mod shared {
                pub mod structs {
                    include!("hashicorp.nomad.plugins.shared.structs.rs");
                }
                pub mod hclspec {
                    include!("hashicorp.nomad.plugins.shared.hclspec.rs");
                }
            }
            pub mod base {
                pub mod proto {
                    include!("hashicorp.nomad.plugins.base.proto.rs");
                }
            }
            pub mod drivers {
                pub mod proto {
                    include!("hashicorp.nomad.plugins.drivers.proto.rs");
                }
            }
        }
    }
}
