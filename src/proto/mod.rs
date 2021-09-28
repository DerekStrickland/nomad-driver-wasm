
pub mod hashicorp {
    pub mod nomad {
        pub mod plugins {
            pub mod drivers {
                pub mod proto {
                    include!("hashicorp.nomad.plugins.drivers.proto.rs");
                }
            }
            pub mod base {
                pub mod proto {
                    include!("hashicorp.nomad.plugins.base.proto.rs");
                }
            }
            pub mod shared {
                pub mod hclspec {
                    include!("hashicorp.nomad.plugins.shared.hclspec.rs");
                }
                pub mod structs {
                    include!("hashicorp.nomad.plugins.shared.structs.rs");
                }
            }
        }
    }
}
pub mod google {
    pub mod protobuf {
        include!("google.protobuf.rs");
    }
}
