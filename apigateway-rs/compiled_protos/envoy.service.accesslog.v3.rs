/// Empty response for the StreamAccessLogs API. Will never be sent. See below.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAccessLogsResponse {}
/// Stream message for the StreamAccessLogs API. Envoy will open a stream to the server and stream
/// access logs without ever expecting a response.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamAccessLogsMessage {
    /// Identifier data that will only be sent in the first message on the stream. This is effectively
    /// structured metadata and is a performance optimization.
    #[prost(message, optional, tag = "1")]
    pub identifier: ::core::option::Option<stream_access_logs_message::Identifier>,
    /// Batches of log entries of a single type. Generally speaking, a given stream should only
    /// ever include one type of log entry.
    #[prost(oneof = "stream_access_logs_message::LogEntries", tags = "2, 3")]
    pub log_entries: ::core::option::Option<stream_access_logs_message::LogEntries>,
}
/// Nested message and enum types in `StreamAccessLogsMessage`.
pub mod stream_access_logs_message {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Identifier {
        /// The node sending the access log messages over the stream.
        #[prost(message, optional, tag = "1")]
        pub node: ::core::option::Option<
            super::super::super::super::config::core::v3::Node,
        >,
        /// The friendly name of the log configured in :ref:`CommonGrpcAccessLogConfig
        /// <envoy_v3_api_msg_extensions.access_loggers.grpc.v3.CommonGrpcAccessLogConfig>`.
        #[prost(string, tag = "2")]
        pub log_name: ::prost::alloc::string::String,
    }
    /// Wrapper for batches of HTTP access log entries.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HttpAccessLogEntries {
        #[prost(message, repeated, tag = "1")]
        pub log_entry: ::prost::alloc::vec::Vec<
            super::super::super::super::data::accesslog::v3::HttpAccessLogEntry,
        >,
    }
    /// Wrapper for batches of TCP access log entries.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TcpAccessLogEntries {
        #[prost(message, repeated, tag = "1")]
        pub log_entry: ::prost::alloc::vec::Vec<
            super::super::super::super::data::accesslog::v3::TcpAccessLogEntry,
        >,
    }
    /// Batches of log entries of a single type. Generally speaking, a given stream should only
    /// ever include one type of log entry.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LogEntries {
        #[prost(message, tag = "2")]
        HttpLogs(HttpAccessLogEntries),
        #[prost(message, tag = "3")]
        TcpLogs(TcpAccessLogEntries),
    }
}
/// Generated client implementations.
pub mod access_log_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Service for streaming access logs from Envoy to an access log server.
    #[derive(Debug, Clone)]
    pub struct AccessLogServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AccessLogServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AccessLogServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AccessLogServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AccessLogServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Envoy will connect and send StreamAccessLogsMessage messages forever. It does not expect any
        /// response to be sent as nothing would be done in the case of failure. The server should
        /// disconnect if it expects Envoy to reconnect. In the future we may decide to add a different
        /// API for "critical" access logs in which Envoy will buffer access logs for some period of time
        /// until it gets an ACK so it could then retry. This API is designed for high throughput with the
        /// expectation that it might be lossy.
        pub async fn stream_access_logs(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::StreamAccessLogsMessage,
            >,
        ) -> std::result::Result<
            tonic::Response<super::StreamAccessLogsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.accesslog.v3.AccessLogService/StreamAccessLogs",
            );
            let mut req = request.into_streaming_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "envoy.service.accesslog.v3.AccessLogService",
                        "StreamAccessLogs",
                    ),
                );
            self.inner.client_streaming(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod access_log_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AccessLogServiceServer.
    #[async_trait]
    pub trait AccessLogService: Send + Sync + 'static {
        /// Envoy will connect and send StreamAccessLogsMessage messages forever. It does not expect any
        /// response to be sent as nothing would be done in the case of failure. The server should
        /// disconnect if it expects Envoy to reconnect. In the future we may decide to add a different
        /// API for "critical" access logs in which Envoy will buffer access logs for some period of time
        /// until it gets an ACK so it could then retry. This API is designed for high throughput with the
        /// expectation that it might be lossy.
        async fn stream_access_logs(
            &self,
            request: tonic::Request<tonic::Streaming<super::StreamAccessLogsMessage>>,
        ) -> std::result::Result<
            tonic::Response<super::StreamAccessLogsResponse>,
            tonic::Status,
        >;
    }
    /// Service for streaming access logs from Envoy to an access log server.
    #[derive(Debug)]
    pub struct AccessLogServiceServer<T: AccessLogService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AccessLogService> AccessLogServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AccessLogServiceServer<T>
    where
        T: AccessLogService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/envoy.service.accesslog.v3.AccessLogService/StreamAccessLogs" => {
                    #[allow(non_camel_case_types)]
                    struct StreamAccessLogsSvc<T: AccessLogService>(pub Arc<T>);
                    impl<
                        T: AccessLogService,
                    > tonic::server::ClientStreamingService<
                        super::StreamAccessLogsMessage,
                    > for StreamAccessLogsSvc<T> {
                        type Response = super::StreamAccessLogsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::StreamAccessLogsMessage>,
                            >,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).stream_access_logs(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = StreamAccessLogsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AccessLogService> Clone for AccessLogServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: AccessLogService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AccessLogService> tonic::server::NamedService for AccessLogServiceServer<T> {
        const NAME: &'static str = "envoy.service.accesslog.v3.AccessLogService";
    }
}
