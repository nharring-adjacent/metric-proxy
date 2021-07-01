#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DescribeMetricsRequest {
    #[prost(string, tag = "1")]
    pub filter: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricsDescription {
    #[prost(bool, tag = "1")]
    pub error: bool,
    #[prost(string, tag = "2")]
    pub error_info: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub page_info: ::core::option::Option<PageInfo>,
    #[prost(map = "string, message", tag = "4")]
    pub metrics: ::std::collections::HashMap<::prost::alloc::string::String, MetricType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MetricType {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(enumeration = "DataType", tag = "3")]
    pub r#type: i32,
    #[prost(string, repeated, tag = "4")]
    pub tags: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, message", tag = "5")]
    pub tag_values: ::std::collections::HashMap<::prost::alloc::string::String, TagValues>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TagValues {
    #[prost(string, repeated, tag = "1")]
    pub values: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(map = "string, int32", tag = "2")]
    pub frequencies: ::std::collections::HashMap<::prost::alloc::string::String, i32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PageInfo {
    #[prost(int32, tag = "1")]
    pub count: i32,
    #[prost(int32, tag = "2")]
    pub offset: i32,
    #[prost(enumeration = "SortOrder", tag = "3")]
    pub order: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataType {
    Bool = 0,
    Count = 1,
    Rate = 2,
    Gauge = 3,
    Histogram = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SortOrder {
    Undef = 0,
    Asc = 1,
    Dsc = 2,
}
#[doc = r" Generated client implementations."]
pub mod metric_proxy_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct MetricProxyClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl MetricProxyClient<tonic::transport::Channel> {
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
    impl<T> MetricProxyClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn describe_metrics(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeMetricsRequest>,
        ) -> Result<tonic::Response<super::MetricsDescription>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/metricproxy.MetricProxy/DescribeMetrics");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for MetricProxyClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for MetricProxyClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "MetricProxyClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod metric_proxy_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with MetricProxyServer."]
    #[async_trait]
    pub trait MetricProxy: Send + Sync + 'static {
        async fn describe_metrics(
            &self,
            request: tonic::Request<super::DescribeMetricsRequest>,
        ) -> Result<tonic::Response<super::MetricsDescription>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct MetricProxyServer<T: MetricProxy> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: MetricProxy> MetricProxyServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for MetricProxyServer<T>
    where
        T: MetricProxy,
        B: HttpBody + Send + Sync + 'static,
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
                "/metricproxy.MetricProxy/DescribeMetrics" => {
                    #[allow(non_camel_case_types)]
                    struct DescribeMetricsSvc<T: MetricProxy>(pub Arc<T>);
                    impl<T: MetricProxy> tonic::server::UnaryService<super::DescribeMetricsRequest>
                        for DescribeMetricsSvc<T>
                    {
                        type Response = super::MetricsDescription;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DescribeMetricsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).describe_metrics(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DescribeMetricsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
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
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: MetricProxy> Clone for MetricProxyServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: MetricProxy> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: MetricProxy> tonic::transport::NamedService for MetricProxyServer<T> {
        const NAME: &'static str = "metricproxy.MetricProxy";
    }
}
