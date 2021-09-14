/// PluginInfoRequest is used to request the plugins basic information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginInfoRequest {}
/// PluginInfoResponse returns basic information about the plugin such
/// that Nomad can decide whether to load the plugin or not.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginInfoResponse {
    /// type indicates what type of plugin this is.
    #[prost(enumeration = "PluginType", tag = "1")]
    pub r#type: i32,
    /// plugin_api_versions indicates the versions of the Nomad Plugin API
    /// this plugin supports.
    #[prost(string, repeated, tag = "2")]
    pub plugin_api_versions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// plugin_version is the semver version of this individual plugin.
    /// This is divorce from Nomad’s development and versioning.
    #[prost(string, tag = "3")]
    pub plugin_version: ::prost::alloc::string::String,
    /// name is the name of the plugin
    #[prost(string, tag = "4")]
    pub name: ::prost::alloc::string::String,
}
/// ConfigSchemaRequest is used to request the configurations schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSchemaRequest {}
/// ConfigSchemaResponse returns the plugins configuration schema.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigSchemaResponse {
    /// spec is the plugins configuration schema
    #[prost(message, optional, tag = "1")]
    pub spec: ::core::option::Option<crate::proto::hclspec::Spec>,
}
/// SetConfigRequest is used to set the configuration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigRequest {
    /// msgpack_config is the configuration encoded as MessagePack.
    #[prost(bytes = "vec", tag = "1")]
    pub msgpack_config: ::prost::alloc::vec::Vec<u8>,
    /// nomad_config is the nomad client configuration sent to all plugins.
    #[prost(message, optional, tag = "2")]
    pub nomad_config: ::core::option::Option<NomadConfig>,
    /// plugin_api_version is the api version to use.
    #[prost(string, tag = "3")]
    pub plugin_api_version: ::prost::alloc::string::String,
}
/// NomadConfig is the client configuration sent to all plugins
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NomadConfig {
    /// driver specific configuration sent to all plugins
    #[prost(message, optional, tag = "1")]
    pub driver: ::core::option::Option<NomadDriverConfig>,
}
/// NomadDriverConfig is the driver specific client configuration sent to all
/// driver plugins
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NomadDriverConfig {
    /// ClientMaxPort is the upper range of the ports that the client uses for
    /// communicating with plugin subsystems over loopback
    /// buf:lint:ignore FIELD_LOWER_SNAKE_CASE
    #[prost(uint32, tag = "1")]
    pub client_max_port: u32,
    /// ClientMinPort is the lower range of the ports that the client uses for
    /// communicating with plugin subsystems over loopback
    /// buf:lint:ignore FIELD_LOWER_SNAKE_CASE
    #[prost(uint32, tag = "2")]
    pub client_min_port: u32,
}
/// SetConfigResponse is used to respond to setting the configuration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetConfigResponse {}
/// PluginType enumerates the type of plugins Nomad supports
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PluginType {
    Unknown = 0,
    Driver = 2,
    Device = 3,
}
#[doc = r" Generated client implementations."]
pub mod base_plugin_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " BasePlugin is the methods that all Nomad plugins must support."]
    #[derive(Debug, Clone)]
    pub struct BasePluginClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BasePluginClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BasePluginClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BasePluginClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            BasePluginClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " PluginInfo describes the type and version of a plugin."]
        pub async fn plugin_info(
            &mut self,
            request: impl tonic::IntoRequest<super::PluginInfoRequest>,
        ) -> Result<tonic::Response<super::PluginInfoResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.base.proto.BasePlugin/PluginInfo",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " ConfigSchema returns the schema for parsing the plugins configuration."]
        pub async fn config_schema(
            &mut self,
            request: impl tonic::IntoRequest<super::ConfigSchemaRequest>,
        ) -> Result<tonic::Response<super::ConfigSchemaResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.base.proto.BasePlugin/ConfigSchema",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " SetConfig is used to set the configuration."]
        pub async fn set_config(
            &mut self,
            request: impl tonic::IntoRequest<super::SetConfigRequest>,
        ) -> Result<tonic::Response<super::SetConfigResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/hashicorp.nomad.plugins.base.proto.BasePlugin/SetConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod base_plugin_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BasePluginServer."]
    #[async_trait]
    pub trait BasePlugin: Send + Sync + 'static {
        #[doc = " PluginInfo describes the type and version of a plugin."]
        async fn plugin_info(
            &self,
            request: tonic::Request<super::PluginInfoRequest>,
        ) -> Result<tonic::Response<super::PluginInfoResponse>, tonic::Status>;
        #[doc = " ConfigSchema returns the schema for parsing the plugins configuration."]
        async fn config_schema(
            &self,
            request: tonic::Request<super::ConfigSchemaRequest>,
        ) -> Result<tonic::Response<super::ConfigSchemaResponse>, tonic::Status>;
        #[doc = " SetConfig is used to set the configuration."]
        async fn set_config(
            &self,
            request: tonic::Request<super::SetConfigRequest>,
        ) -> Result<tonic::Response<super::SetConfigResponse>, tonic::Status>;
    }
    #[doc = " BasePlugin is the methods that all Nomad plugins must support."]
    #[derive(Debug)]
    pub struct BasePluginServer<T: BasePlugin> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BasePlugin> BasePluginServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BasePluginServer<T>
    where
        T: BasePlugin,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/hashicorp.nomad.plugins.base.proto.BasePlugin/PluginInfo" => {
                    #[allow(non_camel_case_types)]
                    struct PluginInfoSvc<T: BasePlugin>(pub Arc<T>);
                    impl<T: BasePlugin> tonic::server::UnaryService<super::PluginInfoRequest> for PluginInfoSvc<T> {
                        type Response = super::PluginInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PluginInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).plugin_info(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = PluginInfoSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.base.proto.BasePlugin/ConfigSchema" => {
                    #[allow(non_camel_case_types)]
                    struct ConfigSchemaSvc<T: BasePlugin>(pub Arc<T>);
                    impl<T: BasePlugin> tonic::server::UnaryService<super::ConfigSchemaRequest> for ConfigSchemaSvc<T> {
                        type Response = super::ConfigSchemaResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ConfigSchemaRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).config_schema(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ConfigSchemaSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/hashicorp.nomad.plugins.base.proto.BasePlugin/SetConfig" => {
                    #[allow(non_camel_case_types)]
                    struct SetConfigSvc<T: BasePlugin>(pub Arc<T>);
                    impl<T: BasePlugin> tonic::server::UnaryService<super::SetConfigRequest> for SetConfigSvc<T> {
                        type Response = super::SetConfigResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetConfigSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: BasePlugin> Clone for BasePluginServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BasePlugin> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BasePlugin> tonic::transport::NamedService for BasePluginServer<T> {
        const NAME: &'static str = "hashicorp.nomad.plugins.base.proto.BasePlugin";
    }
}
